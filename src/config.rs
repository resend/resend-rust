use std::fmt;

#[cfg(not(feature = "blocking"))]
use reqwest::{Client, Request, RequestBuilder, Response};
use reqwest::{Method, Url};
#[cfg(feature = "blocking")]
use reqwest::blocking::{Client, Request, RequestBuilder, Response};
use reqwest::header::USER_AGENT;
use serde::Serialize;

use crate::{Error, Result};
use crate::types::ErrorResponse;

#[derive(Clone)]
pub struct Config {
    pub(crate) user_agent: String,
    pub(crate) api_key: String,
    pub(crate) base_url: Url,
    pub(crate) client: Client,
}

impl Config {
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

    pub fn plain(&self, method: Method, path: &str) -> Result<Request> {
        self.build(method, path).build().map_err(Into::into)
    }

    pub fn json<T: Serialize>(&self, method: Method, path: &str, data: T) -> Result<Request> {
        self.build(method, path)
            .json(&data)
            .build()
            .map_err(Into::into)
    }

    #[cfg(not(feature = "blocking"))]
    pub async fn send(&self, request: RequestBuilder) -> Result<Response> {
        let response = request.send().await?;

        match response.status() {
            x if x.is_client_error() || x.is_server_error() => {
                let error = response.json::<ErrorResponse>().await?;
                Err(Error::Resend(error))
            }
            _ => Ok(response),
        }
    }

    #[cfg(feature = "blocking")]
    pub fn send(&self, request: RequestBuilder) -> Result<Response> {
        let response = request.send()?;

        match response.status() {
            x if x.is_client_error() || x.is_server_error() => {
                let error = response.json::<ErrorResponse>()?;
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
            // .field("client", &self.client)
            .finish()
    }
}
