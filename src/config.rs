use std::{env, fmt};

#[cfg(feature = "blocking")]
use reqwest::blocking::{Client, RequestBuilder, Response};
use reqwest::header::USER_AGENT;
#[cfg(not(feature = "blocking"))]
use reqwest::{Client, RequestBuilder, Response};
use reqwest::{Method, Url};

use crate::{error::types::ErrorResponse, Error, Result};

pub struct Config {
    pub(crate) user_agent: String,
    pub(crate) api_key: String,
    pub(crate) base_url: Url,
    pub(crate) client: Client,
}

impl Config {
    /// Creates a new [`Config`].
    pub fn new(api_key: &str, client: Client) -> Self {
        let env_base_url = env::var("RESEND_BASE_URL")
            .map_or_else(
                |_| Url::parse("https://api.resend.com"),
                |env_var| Url::parse(env_var.as_str()),
            )
            .expect("env variable `RESEND_BASE_URL` should be a valid URL");

        let env_user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

        Self {
            user_agent: env_user_agent,
            api_key: api_key.to_owned(),
            base_url: env_base_url,
            client,
        }
    }

    /// Constructs a new [`RequestBuilder`].
    pub fn build(&self, method: Method, path: &str) -> RequestBuilder {
        let path = self
            .base_url
            .join(path)
            .expect("should be a valid API endpoint");

        self.client
            .request(method, path)
            .bearer_auth(self.api_key.as_str())
            .header(USER_AGENT, self.user_agent.as_str())
    }

    #[maybe_async::maybe_async]
    pub async fn send(&self, request: RequestBuilder) -> Result<Response> {
        let request = request.build()?;
        let response = self.client.execute(request).await?;

        match response.status() {
            x if x.is_client_error() || x.is_server_error() => {
                let error = response.json::<ErrorResponse>().await?;
                Err(Error::Resend(error))
            }
            _ => Ok(response),
        }
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Don't output API key.
        f.debug_struct("Client")
            .field("api_key", &"re_*********")
            .field("user_agent", &self.user_agent.as_str())
            .field("base_url", &self.base_url.as_str())
            .finish_non_exhaustive()
    }
}
