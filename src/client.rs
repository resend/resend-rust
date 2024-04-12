use std::sync::Arc;
use std::{env, fmt};

#[cfg(feature = "blocking")]
use reqwest::blocking::Client as ReqwestClient;
#[cfg(not(feature = "blocking"))]
use reqwest::Client as ReqwestClient;
use reqwest::Url;

use crate::config::Config;
use crate::services::{ApiKeys, Audiences, Contacts, Domains, Emails};

// TODO: Arc<ClientInner> + impl Deref?

/// A minimal [Resend](https://resend.com) client.
///
/// #### Example
///
/// ```rust,no_run
/// use resend_rs::{Client, Result};
/// use resend_rs::types::SendEmailRequest;
///
/// let from = "Acme <onboarding@resend.dev>".to_owned();
/// let to = vec!["delivered@resend.dev".to_owned()];
/// let sub = "Hello World".to_owned();
///
/// let email = SendEmailRequest::new(from, to, sub)
///     .with_text("Hello World!");
///
/// let resend = Client::default();
/// # tokio::runtime::Handle::block_on(async {
/// let resp = resend.emails.send(email).await?;
/// println!("id: {}", resp.id);
/// # });
/// ```
#[must_use]
#[derive(Clone)]
pub struct Client {
    /// `Resend` APIs for `/emails` endpoints.
    pub emails: Emails,
    /// `Resend` APIs for `/api-keys` endpoints.
    pub api_keys: ApiKeys,
    /// `Resend` APIs for `/audiences` endpoints.
    pub audiences: Audiences,
    /// `Resend` APIs for `/audiences/:id/contacts` endpoints.
    pub contacts: Contacts,
    /// `Resend` APIs for `/domains` endpoints.
    pub domains: Domains,
}

impl Client {
    /// Creates a new [`Resend`] client.
    ///
    /// ### Panics
    ///
    /// Panics if the environment variable `RESEND_BASE_URL` is present but is not a valid URL.
    ///
    /// [`Resend`]: https://resend.com
    pub fn new(api_key: &str) -> Self {
        Self::with_client(api_key, ReqwestClient::default())
    }

    /// Creates a new [`Resend`] client with a provided [`reqwest::Client`].
    ///
    /// ### Panics
    ///
    /// Panics if the environment variable `RESEND_BASE_URL` is present but is not a valid URL.
    ///
    /// [`Resend`]: https://resend.com
    /// [`reqwest::Client`]: ReqwestClient
    pub fn with_client(api_key: &str, client: ReqwestClient) -> Self {
        let env_base_url = env::var("RESEND_BASE_URL")
            .map_or_else(
                |_| Url::parse("https://api.resend.com"),
                |env_var| Url::parse(env_var.as_str()),
            )
            .expect("env variable `RESEND_BASE_URL` should be a valid URL");

        let env_user_agent = env::var("RESEND_USER_AGENT").unwrap_or_else(|_| {
            format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
        });

        let inner = Arc::new(Config {
            user_agent: env_user_agent,
            api_key: api_key.to_owned(),
            base_url: env_base_url,
            client,
        });

        Self {
            api_keys: ApiKeys(inner.clone()),
            audiences: Audiences(inner.clone()),
            contacts: Contacts(inner.clone()),
            domains: Domains(inner.clone()),
            emails: Emails(inner),
        }
    }

    /// Returns the used `User-Agent` header value.
    #[must_use]
    pub fn user_agent(&self) -> String {
        self.emails.0.user_agent.clone()
    }

    /// Returns the provided API key.
    #[must_use]
    pub fn api_key(&self) -> String {
        self.emails.0.api_key.clone()
    }

    /// Returns the underlying [`reqwest::Client`].
    ///
    /// [`reqwest::Client`]: ReqwestClient
    #[must_use]
    pub fn client(&self) -> ReqwestClient {
        self.emails.0.client.clone()
    }
}

impl Default for Client {
    /// Creates a new [`Client`] from the `RESEND_API_KEY` environment variable .
    ///
    /// ### Panics
    ///
    /// Panics if the environment variable `RESEND_API_KEY` is not set.
    fn default() -> Self {
        let api_key = env::var("RESEND_API_KEY")
            .expect("env variable `RESEND_API_KEY` should be a valid API key");

        Self::new(api_key.as_str())
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.emails, f)
    }
}
