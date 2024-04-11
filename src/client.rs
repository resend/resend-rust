use std::{env, fmt};
use std::sync::Arc;

#[cfg(not(feature = "blocking"))]
use reqwest::{Client as ReqwestClient, RequestBuilder};
use reqwest::{Method, Url};
#[cfg(feature = "blocking")]
use reqwest::blocking::{Client as ReqwestClient, RequestBuilder};
use reqwest::header::USER_AGENT;

use crate::services::{ApiKeys, Audiences, Contacts, Domains, Emails};

/// A minimal [Resend](https://resend.com) client.
// TODO: Arc<ClientInner> + impl Deref?
#[must_use]
#[derive(Clone)]
pub struct Client {
    /// `Resend` APIs for `METHOD /emails` endpoints.
    pub emails: Emails,
    /// `Resend` APIs for `METHOD /api-keys` endpoints.
    pub api_keys: ApiKeys,
    /// `Resend` APIs for `METHOD /audiences` endpoints.
    pub audiences: Audiences,
    /// `Resend` APIs for `METHOD /audiences/:id/contacts` endpoints.
    pub contacts: Contacts,
    /// `Resend` APIs for `METHOD /domains` endpoints.
    pub domains: Domains,
}

#[derive(Clone)]
pub struct Config {
    user_agent: String,
    api_key: String,
    pub(crate) base_url: Url,
    pub(crate) client: ReqwestClient,
}

impl Config {
    /// Constructs a new [`RequestBuilder`] with parameters.
    pub fn build(&self, method: Method, path: &str) -> RequestBuilder {
        let path = self
            .base_url
            .join(path)
            .expect("should a valid API endpoint");

        self.client
            .request(method, path)
            .bearer_auth(self.api_key.as_str())
            .header(USER_AGENT, self.user_agent.as_str())
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Don't output API key.
        f.debug_struct("Client")
            .field("api_key", &"re_*********")
            .field("user_agent", &self.user_agent.as_str())
            .field("base_url", &self.base_url.as_str())
            // .field("client", &self.client)
            .finish()
    }
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
    /// Panics if the environment variable `RESEND_API_KEY` is not a valid API key.
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

#[cfg(test)]
mod test {
    use crate::Client;

    #[test]
    fn create() {
        let _ = Client::default();
    }
}
