#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
//! #### Examples
//!
//! ```rust,no_run
//! use resend_rs::types::CreateEmailBaseOptions;
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
//!         .with_tag("Welcome");
//!
//!     let id = resend.emails.send(email).await?.id;
//!     println!("id: {id}");
//!     Ok(())
//! }
//!
//! ```

// FIXME: Tests can fail due to rate limit constraints (max 10 req/s). Running the tests on one
//        thread seems to work for now but this is just a workaround. For now, an alias is provided
//        for `cargo t` which automatically passes `-- --test-threads=1` to the tests.
//  Edit: Somewhat unsurprisingly, this sometimes fails in CI because the Linux image just runs
//        faster so additional thread sleeps were added, these need to be removed when (if?) this is
//        solved.
// TODO: Remember to fix email/batch GETs whenever the server is updated

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
        DomainRecord, DomainSpfRecord, DomainStatus, ProxyStatus, Region, SpfRecordType,
        UpdateDomainResponse, Tls
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
