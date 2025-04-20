#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
//! ### Rate Limits
//!
//! Resend implements rate limiting on their API which can sometimes get in the way of whatever
//! you are trying to do. This crate handles that in 2 ways:
//!
//! - Firstly *all* requests made by the [`Resend`] client are automatically rate limited to
//!   9 req/1.1s to avoid collisions with the 10 req/s limit that Resend imposes at the time of
//!   writing this.
//!
//!   Note that the client can be safely cloned as well as used in async/parallel contexts and the
//!   rate limit will work as intended. The only exception to this is creating 2 clients via the
//!   [`Resend::new`] or [`Resend::with_client`] methods which should be avoided, use `.clone()`
//!   instead.
//!
//! - Secondly, a couple of helper methods as well as macros are implemented in the [`rate_limit`]
//!   module that allow catching rate limit errors and retrying the request instead of failing.
//!   
//!   These were implemented to handle cases where this crate is used in a horizontally scaled
//!   environment and thus needs to work on different machines at the same time in which case the
//!   internal rate limits alone cannot guarantee that there will be no rate limit errors.
//!
//!   As long as only one program is interacting with the Resend servers on your behalf, this
//!   module does not need to be used.
//!
//! ### Examples
//!
//! ```rust,no_run
//! use resend_rs::types::{CreateEmailBaseOptions, Tag};
//! use resend_rs::{Resend, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let resend = Resend::default();
//!
//!     let from = "Acme <onboarding@a.dev>";
//!     let to = ["delivered@resend.dev"];
//!     let subject = "Hello World!";
//!
//!     let email = CreateEmailBaseOptions::new(from, to, subject)
//!         .with_text("Hello World!")
//!         .with_tag(Tag::new("hello", "world"));
//!
//!     let id = resend.emails.send(email).await?.id;
//!     println!("id: {id}");
//!     Ok(())
//! }
//!
//! ```

pub use client::Resend;
pub(crate) use config::Config;

mod api_keys;
mod audiences;
mod batch;
mod broadcasts;
mod client;
mod config;
mod contacts;
mod domains;
mod emails;
mod error;
pub mod events;
pub mod rate_limit;

pub mod services {
    //! `Resend` API services.

    pub use super::api_keys::ApiKeysSvc;
    pub use super::audiences::AudiencesSvc;
    pub use super::batch::BatchSvc;
    pub use super::broadcasts::BroadcastsSvc;
    pub use super::contacts::ContactsSvc;
    pub use super::domains::DomainsSvc;
    pub use super::emails::EmailsSvc;
}

pub mod types {
    //! Request and response types.

    pub use super::api_keys::types::{
        ApiKey, ApiKeyId, ApiKeyToken, CreateApiKeyOptions, Permission,
    };
    pub use super::audiences::types::{Audience, AudienceId, CreateAudienceResponse};
    pub use super::batch::types::SendEmailBatchResponse;
    pub use super::broadcasts::types::{
        Broadcast, BroadcastId, CreateBroadcastOptions, CreateBroadcastResponse,
        RemoveBroadcastResponse, SendBroadcastOptions, SendBroadcastResponse,
        UpdateBroadcastOptions, UpdateBroadcastResponse,
    };
    pub use super::contacts::types::{Contact, ContactChanges, ContactData, ContactId};
    pub use super::domains::types::{
        CreateDomainOptions, DkimRecordType, Domain, DomainChanges, DomainDkimRecord, DomainId,
        DomainRecord, DomainSpfRecord, DomainStatus, ProxyStatus, Region, SpfRecordType, Tls,
        UpdateDomainResponse,
    };
    pub use super::emails::types::{
        Attachment, CancelScheduleResponse, ContentOrPath, CreateEmailBaseOptions,
        CreateEmailResponse, Email, EmailId, Tag, UpdateEmailOptions, UpdateEmailResponse,
    };
    pub use super::error::types::{ErrorKind, ErrorResponse};
}

/// Error type for operations of a [`Resend`] client.
///
/// <https://resend.com/docs/api-reference/errors>
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Errors that may occur during the processing an HTTP request.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// Errors that may occur during the processing of the API request.
    #[error("resend error: {0}")]
    Resend(#[from] types::ErrorResponse),

    /// Errors that may occur during the parsing of an API response.
    #[error("Failed to parse Resend API response. Received: \n{0}")]
    Parse(String),

    /// Detailed rate limit error. For the old error variant see
    /// [`types::ErrorKind::RateLimitExceeded`].
    #[error("Too many requests. Limit is {ratelimit_limit:?} per {ratelimit_reset:?} seconds.")]
    RateLimit {
        ratelimit_limit: Option<u64>,
        ratelimit_remaining: Option<u64>,
        ratelimit_reset: Option<u64>,
    },
}

#[cfg(test)]
mod test {
    use crate::Error;

    #[allow(dead_code, clippy::redundant_pub_crate)]
    pub(crate) struct LocatedError<E: std::error::Error + 'static> {
        inner: E,
        location: &'static std::panic::Location<'static>,
    }

    impl From<Error> for LocatedError<Error> {
        #[track_caller]
        fn from(value: Error) -> Self {
            Self {
                inner: value,
                location: std::panic::Location::caller(),
            }
        }
    }

    impl<T: std::error::Error + 'static> std::fmt::Debug for LocatedError<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}:{}:{}\n{:?}",
                self.location.file(),
                self.location.line(),
                self.location.column(),
                self.inner
            )
        }
    }

    #[allow(clippy::redundant_pub_crate)]
    pub(crate) type DebugResult<T, E = LocatedError<Error>> = Result<T, E>;
}

/// Specialized [`Result`] type for an [`Error`].
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
pub(crate) mod tests {
    use std::sync::LazyLock;

    use crate::Resend;

    #[allow(clippy::redundant_pub_crate)]
    /// Use this client in all tests to ensure rate limits are respected.
    ///
    /// Instantiate with:
    /// ```
    /// let resend = &*CLIENT;
    /// ```
    pub(crate) static CLIENT: LazyLock<Resend> = LazyLock::new(Resend::default);
}
