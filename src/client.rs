use std::fmt;
use std::sync::Arc;

#[cfg(feature = "blocking")]
use reqwest::blocking::Client;
#[cfg(not(feature = "blocking"))]
use reqwest::Client;

use crate::services::Emails;

// TODO: api_keys
// TODO: audiences
// TODO: contacts
// TODO: domains

/// A minimal [Resend](https://resend.com) client.
#[derive(Clone)]
pub struct ResendClient {
    inner: ResendClientInner,
    pub emails: Emails,
}

#[derive(Clone)]
pub struct ResendClientInner {
    pub(crate) api_key: Arc<String>,
    #[cfg(feature = "blocking")]
    pub(crate) client: Client,
    #[cfg(not(feature = "blocking"))]
    pub(crate) client: Client,
}

impl fmt::Debug for ResendClientInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Don't output API key.
        fmt::Debug::fmt(&self.client, f)
    }
}

impl ResendClient {
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
        let inner = ResendClientInner { api_key, client };

        Self {
            emails: Emails::new(inner.clone()),

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

impl fmt::Debug for ResendClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}
