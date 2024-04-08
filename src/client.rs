use std::{env, fmt};
use std::sync::Arc;

#[cfg(feature = "blocking")]
use reqwest::blocking::Client as ReqwestClient;
#[cfg(not(feature = "blocking"))]
use reqwest::Client as ReqwestClient;
use reqwest::Url;

use crate::services::{ApiKeys, Emails};

// TODO: audiences
// TODO: contacts
// TODO: domains

/// A minimal [Resend](https://resend.com) client.
#[must_use]
#[derive(Clone)]
pub struct Client {
    /// TODO.
    pub emails: Emails,
    /// TODO.
    pub api_keys: ApiKeys,
}

#[derive(Clone)]
pub struct Config {
    pub(crate) api_key: Arc<String>,
    pub(crate) base_url: Arc<Url>,
    pub(crate) client: ReqwestClient,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Don't output API key.
        f.debug_struct("Client")
            .field("api_key", &"re_*********")
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
        let base_url = env::var("RESEND_BASE_URL")
            .map_or_else(
                |_| Url::parse("https://api.resend.com"),
                |env_base| Url::parse(env_base.as_str()),
            )
            .expect("env variable `RESEND_BASE_URL` should be a valid URL");

        let inner = Config {
            api_key: Arc::new(api_key.to_string()),
            base_url: Arc::new(base_url),
            client,
        };

        Self {
            emails: Emails(inner.clone()),
            api_keys: ApiKeys(inner),
        }
    }

    /// Returns the provided API key.
    #[must_use]
    pub fn api_key(&self) -> String {
        self.emails.0.api_key.to_string()
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
    /// Creates a new [`Client`] client from the `RESEND_API_KEY` environment variable .
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
    use std::env;

    use crate::Client;

    #[test]
    fn new() {
        let api_key = env::var("RESEND_API_KEY").unwrap_or_default();
        let resend = Client::new(api_key.as_str());
        let _ = dbg!(resend);
    }
}
