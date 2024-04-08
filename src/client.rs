use std::sync::Arc;
use std::{env, fmt};

#[cfg(feature = "blocking")]
use reqwest::blocking::Client;
#[cfg(not(feature = "blocking"))]
use reqwest::Client;
use reqwest::Url;

use crate::services::{ApiKeys, Emails};

// TODO: audiences
// TODO: contacts
// TODO: domains

/// A minimal [Resend](https://resend.com) client.
#[derive(Clone)]
pub struct Resend {
    pub emails: Emails,
    pub api_keys: ApiKeys,
}

#[derive(Clone)]
pub(crate) struct Config {
    pub(crate) api_key: Arc<String>,
    pub(crate) base_url: Arc<Url>,

    #[cfg(feature = "blocking")]
    pub(crate) client: Client,
    #[cfg(not(feature = "blocking"))]
    pub(crate) client: Client,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Don't output API key.
        f.debug_struct("Resend")
            .field("base_url", &self.base_url)
            .field("client", &self.client)
            .finish()
    }
}

impl Resend {
    /// Creates a new [`Resend`] client.
    ///
    /// ### Panics
    ///
    /// Panics if the environment variable `RESEND_BASE_URL` is not a valid URL.
    ///
    /// [`Resend`]: https://resend.com
    pub fn new(api_key: &str) -> Self {
        let client = Client::default();
        Self::with_client(api_key, client)
    }

    /// Creates a new [`Resend`] client with a provided [`Client`].
    ///
    /// ### Panics
    ///
    /// Panics if the environment variable `RESEND_BASE_URL` is not a valid URL.
    ///
    /// [`Resend`]: https://resend.com
    pub fn with_client(api_key: &str, client: Client) -> Self {
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

    /// Returns the provided `Resend` API key.
    pub fn api_key(&self) -> String {
        self.emails.0.api_key.to_string()
    }

    /// Returns the underlying [`Client`].
    pub fn client(&self) -> Client {
        self.emails.0.client.clone()
    }
}

impl fmt::Debug for Resend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.emails.0, f)
    }
}

#[cfg(test)]
mod test {
    use crate::Resend;

    #[test]
    fn new() {
        let api_key = std::env::var("RESEND_API_KEY").unwrap();
        let _ = Resend::new(api_key.as_str());
    }
}
