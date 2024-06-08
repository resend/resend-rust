#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
//! #### Examples
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
mod client;
mod config;
mod contacts;
mod domains;
mod emails;
mod error;

pub mod services {
    //! `Resend` API services.

    pub use super::api_keys::ApiKeysSvc;
    pub use super::audiences::AudiencesSvc;
    pub use super::batch::BatchSvc;
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
    pub use super::batch::BatchSvc;
    pub use super::contacts::types::{Contact, ContactChanges, ContactData, ContactId};
    pub use super::domains::types::{
        CreateDomainOptions, DkimRecordType, Domain, DomainChanges, DomainDkimRecord, DomainId,
        DomainRecord, DomainSpfRecord, DomainStatus, ProxyStatus, Region, SpfRecordType, Tls,
        UpdateDomainResponse,
    };
    pub use super::emails::types::{
        Attachment, ContentOrPath, CreateEmailBaseOptions, CreateEmailResponse, Email, EmailId, Tag,
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
}

/// Specialized [`Result`] type for an [`Error`].
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
pub(crate) mod tests {
    use std::sync::OnceLock;

    use crate::Resend;

    /// Use this client in all tests to ensure rate limits are respected.
    ///
    /// Instantiate with:
    /// ```
    /// let resend = CLIENT.get_or_init(Resend::default);
    /// ```
    pub static CLIENT: OnceLock<Resend> = OnceLock::new();
}
