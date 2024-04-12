#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
//! #### Examples
//!
//! ```rust,no_run
//! use resend_rs::{Client, Result};
//! use resend_rs::types::SendEmailRequest;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let resend = Client::default();
//!
//!     let from = "Acme <onboarding@resend.dev>".to_owned();
//!     let to = vec!["delivered@resend.dev".to_owned()];
//!     let subject = "Hello World".to_owned();
//!
//!     let email = SendEmailRequest::new(from, to, subject)
//!         .with_text("Hello World!")
//!         .with_tag("Welcome");
//!
//!     let _ = resend.emails.send(email).await?;
//!     Ok(())
//! }
//! ```

pub use client::Client;
pub(crate) use config::Config;

mod api_keys;
mod audiences;
mod client;
mod config;
mod contacts;
mod domains;
mod emails;

// TODO: urlencode path params?

pub mod services {
    //! TODO.

    pub use super::api_keys::ApiKeys;
    pub use super::audiences::Audiences;
    pub use super::contacts::Contacts;
    pub use super::domains::Domains;
    pub use super::emails::Emails;
}

pub mod types {
    //! TODO.

    use serde::Deserialize;

    pub use super::api_keys::types::*;
    pub use super::audiences::types::*;
    pub use super::contacts::types::*;
    pub use super::domains::types::*;
    pub use super::emails::types::*;

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

/// Error type for operations of a [`Client`].
///
/// <https://resend.com/docs/api-reference/errors>
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// TODO.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// TODO.
    #[error("resend error: {0}")]
    Resend(#[from] types::ErrorResponse),
}

/// Specialized [`Result`] type for an [`Error`].
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
