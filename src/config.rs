use std::time::Duration;
use std::{env, fmt};

#[cfg(feature = "blocking")]
use reqwest::blocking::{Client, RequestBuilder, Response};
use reqwest::header::USER_AGENT;
#[cfg(not(feature = "blocking"))]
use reqwest::{Client, RequestBuilder, Response};
use reqwest::{Method, Url};

use crate::types::ErrorResponse;
use crate::{Error, Result};

pub struct Config {
    pub(crate) user_agent: String,
    pub(crate) api_key: String,
    pub(crate) base_url: Url,
    #[cfg(not(feature = "blocking"))]
    pub(crate) client: tower::limit::RateLimit<Client>,
    #[cfg(feature = "blocking")]
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

        let env_user_agent = env::var("RESEND_USER_AGENT").unwrap_or_else(|_| {
            format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
        });

        #[cfg(not(feature = "blocking"))]
        let client = tower::ServiceBuilder::new()
            .rate_limit(10, Duration::from_secs(1))
            .service(client);

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

        self.client()
            .request(method, path)
            .bearer_auth(self.api_key.as_str())
            .header(USER_AGENT, self.user_agent.as_str())
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

    pub fn client(&self) -> Client {
        #[cfg(not(feature = "blocking"))]
        {
            self.client.get_ref().clone()
        }
        #[cfg(feature = "blocking")]
        {
            self.client.clone()
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

pub mod types {
    use serde::Deserialize;

    /// Error returned as a response.
    ///
    /// <https://resend.com/docs/api-reference/errors>
    #[derive(Debug, Clone, Deserialize, thiserror::Error)]
    #[error("resend error: {message}")]
    pub struct ErrorResponse {
        #[serde(rename = "statusCode")]
        pub status_code: u16,
        pub message: String,
        pub name: String,
    }

    impl ErrorResponse {
        /// Returns the [`ErrorKind`].
        #[must_use]
        pub fn kind(&self) -> ErrorKind {
            ErrorKind::from(self.name.as_str())
        }
    }

    /// Error type for operations of a [`Client`].
    ///
    /// <https://resend.com/docs/api-reference/errors>
    ///
    /// [`Client`]: crate::Client
    #[non_exhaustive]
    #[derive(Debug, Copy, Clone)]
    pub enum ErrorKind {
        /// Error name is not in the API spec.
        Unrecognized,

        /// 401 Unauthorized.
        ///
        /// - `missing_api_key`
        ///
        /// Missing API key in the authorization header.
        ///
        /// Include the following header `Authorization: Bearer YOUR_API_KEY` in the request.
        MissingApiKey,

        /// 403 Forbidden.
        ///
        /// - `invalid_api_key`
        ///
        /// The API key is not valid.
        ///
        /// Generate a new API key in the dashboard.
        InvalidApiKey,

        /// 403 Forbidden.
        ///
        /// - `invalid_from_address`
        ///
        /// The from address is not valid.
        ///
        /// Review your existing domains in the dashboard.
        InvalidFromAddress,

        /// 403 Forbidden.
        ///
        /// - `invalid_to_address`
        ///
        /// You can only send testing emails to your own email address.
        ///
        /// In order to send emails to any external address, you need to add a domain and
        /// use that as the `from` address instead of `onboarding@resend.dev`.
        InvalidToAddress,

        /// 404 Not Found.
        ///
        /// - `not_found`
        ///
        /// The requested endpoint does not exist.
        ///
        /// Change your request URL to match a valid API endpoint.
        NotFound,

        /// 405 Method Not Allowed.
        ///
        /// - `method_not_allowed`
        ///
        /// This endpoint does not support the specified HTTP method.
        ///
        /// Change the HTTP method to follow the documentation for the endpoint.
        MethodNotAllowed,

        /// 422 Unprocessable Content.
        ///
        /// - `missing_required_field`
        ///
        /// The request body is missing one or more required fields.
        /// Check the error message to see the list of missing fields.
        MissingRequiredField,

        /// 422 Unprocessable Content.
        ///
        /// - `invalid_attachment`
        ///
        /// Attachment must have either a `content` or `path`.
        ///
        /// Attachments must either have a `content` (strings, Buffer, or Stream contents) or
        /// `path` to a remote resource (better for larger attachments).
        InvalidAttachment,

        /// 422 Unprocessable Content.
        ///
        /// - `invalid_scope`
        ///
        /// This endpoint does not support the specified scope.
        /// Change the scope to follow the documentation for the endpoint.
        InvalidScope,

        /// 429 Too Many Requests.
        ///
        /// - `rate_limit_exceeded`
        ///
        ///  Too many requests. Please limit the number of requests per second.
        /// Or contact support to increase rate limit.
        ///
        /// You should read the response headers and reduce the rate at which you request the API.
        /// This can be done by introducing a queue mechanism or reducing the number of
        /// concurrent requests per second.
        ///
        /// If you have specific requirements, contact support to request a rate increase.
        RateLimitExceeded,

        /// 429 Too Many Requests.
        ///
        /// - `daily_quota_exceeded`
        ///
        /// You have reached your daily email sending quota.
        ///
        /// Upgrade your plan to remove daily quota limit or
        /// wait until 24 hours have passed to continue sending.
        DailyQuotaExceeded,

        /// 500 Internal Server Error.
        ///
        /// - `internal_server_error`
        ///
        /// An unexpected error occurred.
        ///
        /// Try the request again later. If the error does not resolve,
        /// check our [`status page`] for service updates.
        ///
        /// [`status page`]: https://resend-status.com/
        InternalServerError,
    }

    impl<T: AsRef<str>> From<T> for ErrorKind {
        fn from(value: T) -> Self {
            match value.as_ref() {
                "missing_api_key" => Self::MissingApiKey,
                "invalid_api_key" => Self::InvalidApiKey,
                "invalid_from_address" => Self::InvalidFromAddress,
                "invalid_to_address" => Self::InvalidToAddress,
                "not_found" => Self::NotFound,
                "method_not_allowed" => Self::MethodNotAllowed,
                "missing_required_field" => Self::MissingRequiredField,
                "invalid_attachment" => Self::InvalidAttachment,
                "invalid_scope" => Self::InvalidScope,
                "rate_limit_exceeded" => Self::RateLimitExceeded,
                "daily_quota_exceeded" => Self::DailyQuotaExceeded,
                "internal_server_error" => Self::InternalServerError,
                _ => Self::Unrecognized,
            }
        }
    }
}
