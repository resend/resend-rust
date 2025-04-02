use std::sync::Arc;
use std::{env, fmt};

#[cfg(feature = "blocking")]
use reqwest::blocking::Client as ReqwestClient;
#[cfg(not(feature = "blocking"))]
use reqwest::Client as ReqwestClient;

use crate::services::{
    ApiKeysSvc, AudiencesSvc, BroadcastsSvc, ContactsSvc, DomainsSvc, EmailsSvc,
};
use crate::{batch::BatchSvc, config::Config};

/// The [Resend](https://resend.com) client.
#[must_use]
#[derive(Clone)]
pub struct Resend {
    /// `Resend` APIs for `/emails` endpoints.
    pub emails: EmailsSvc,
    /// `Resend` APIs for the batch `/emails` endpoints.
    pub batch: BatchSvc,
    /// `Resend` APIs for `/api-keys` endpoints.
    pub api_keys: ApiKeysSvc,
    /// `Resend` APIs for `/audiences` endpoints.
    pub audiences: AudiencesSvc,
    /// `Resend` APIs for `/audiences/:id/contacts` endpoints.
    pub contacts: ContactsSvc,
    /// `Resend` APIs for `/domains` endpoints.
    pub domains: DomainsSvc,
    /// `Resend` APIs for `/broadcasts` endpoints.
    pub broadcasts: BroadcastsSvc,
}

impl Resend {
    /// Creates a new [`Resend`] client.
    ///
    /// ### Panics
    ///
    /// - Panics if the environment variable `RESEND_BASE_URL` is set but is not a valid `URL`.
    ///
    /// [`Resend`]: https://resend.com
    pub fn new(api_key: &str) -> Self {
        Self::with_client(api_key, ReqwestClient::default())
    }

    /// Creates a new [`Resend`] client with a provided [`reqwest::Client`].
    ///
    /// ### Panics
    ///
    /// - Panics if the environment variable `RESEND_BASE_URL` is set but is not a valid `URL`.
    ///
    /// [`Resend`]: https://resend.com
    /// [`reqwest::Client`]: ReqwestClient
    pub fn with_client(api_key: &str, client: ReqwestClient) -> Self {
        let inner = Arc::new(Config::new(api_key, client));

        Self {
            api_keys: ApiKeysSvc(Arc::clone(&inner)),
            audiences: AudiencesSvc(Arc::clone(&inner)),
            contacts: ContactsSvc(Arc::clone(&inner)),
            domains: DomainsSvc(Arc::clone(&inner)),
            emails: EmailsSvc(Arc::clone(&inner)),
            batch: BatchSvc(Arc::clone(&inner)),
            broadcasts: BroadcastsSvc(inner),
        }
    }

    /// Returns the reference to the used `User-Agent` header value.
    #[inline]
    #[must_use]
    pub fn user_agent(&self) -> &str {
        self.config().user_agent.as_str()
    }

    /// Returns the reference to the provided `API key`.
    #[inline]
    #[must_use]
    pub fn api_key(&self) -> &str {
        self.config().api_key.as_ref()
    }

    /// Returns the reference to the used `base URL`.
    ///
    /// ### Notes
    ///
    /// Use the `RESEND_BASE_URL` environment variable to override.
    #[inline]
    #[must_use]
    pub fn base_url(&self) -> &str {
        self.config().base_url.as_str()
    }

    /// Returns the underlying [`reqwest::Client`].
    ///
    /// [`reqwest::Client`]: ReqwestClient
    #[inline]
    #[must_use]
    pub fn client(&self) -> ReqwestClient {
        self.config().client.clone()
    }

    /// Returns the reference to the inner [`Config`].
    #[inline]
    fn config(&self) -> &Config {
        &self.emails.0
    }
}

impl Default for Resend {
    /// Creates a new [`Resend`] client from the `RESEND_API_KEY` environment variable .
    ///
    /// ### Panics
    ///
    /// - Panics if the environment variable `RESEND_API_KEY` is not set.
    /// - Panics if the environment variable `RESEND_BASE_URL` is set but is not a valid `URL`.
    fn default() -> Self {
        let api_key = env::var("RESEND_API_KEY")
            .expect("env variable `RESEND_API_KEY` should be a valid API key");

        Self::new(api_key.as_str())
    }
}

impl fmt::Debug for Resend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.emails, f)
    }
}
