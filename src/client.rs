use std::fmt;
use std::sync::Arc;

#[cfg(feature = "blocking")]
use reqwest::blocking::Client;
#[cfg(not(feature = "blocking"))]
use reqwest::Client;

use crate::api_keys::ApiKeys;
use crate::services::Emails;

// TODO: api_keys
// TODO: audiences
// TODO: contacts
// TODO: domains

/// A minimal [Resend](https://resend.com) client.
#[derive(Clone)]
pub struct Resend {
    inner: ResendInner,
    pub emails: Emails,
    pub api_keys: ApiKeys,
}

#[derive(Clone)]
pub(crate) struct ResendInner {
    pub(crate) api_key: Arc<String>,
    #[cfg(feature = "blocking")]
    pub(crate) client: Client,
    #[cfg(not(feature = "blocking"))]
    pub(crate) client: Client,
}

impl fmt::Debug for ResendInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Don't output API key.
        fmt::Debug::fmt(&self.client, f)
    }
}

impl Resend {
    /// Creates a new [`Resend`] client.
    ///
    /// [`Resend`]: https://resend.com
    pub fn new(api_key: &str) -> Self {
        let client = Client::default();
        Self::with_client(api_key, client)
    }

    /// Creates a new [`Resend`] client with a provided [`Client`].
    ///
    /// [`Resend`]: https://resend.com
    pub fn with_client(api_key: &str, client: Client) -> Self {
        let api_key = Arc::new(api_key.to_string());
        let inner = ResendInner { api_key, client };

        Self {
            emails: Emails(inner.clone()),
            api_keys: ApiKeys(inner.clone()),

            inner,
        }
    }

    /// Returns the provided `Resend` API key.
    pub fn api_key(&self) -> String {
        self.inner.api_key.to_string()
    }

    /// Returns the underlying [`Client`].
    pub fn client(&self) -> Client {
        self.inner.client.clone()
    }
}

impl fmt::Debug for Resend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

#[cfg(test)]
mod test {
    use crate::Resend;

    #[test]
    fn new() {
        let api_key = std::env::var("API_KEY").unwrap();
        let _ = Resend::new(api_key.as_str());
    }
}
