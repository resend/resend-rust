#![forbid(unsafe_code)]
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
//!     let resp = resend.emails.send(email).await?;
//!     println!("id: {}", resp.id);
//!     Ok(())
//! }
//! ```

// FIXME: Tests can fail due to rate limit constraints (max 10 req/s). Running the tests on one
//        thread seems to work for now but this is just a workaround. For now, an alias is provided
//        for `cargo t` which automatically passes `-- --test-threads=1` to the tests.

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

    pub use super::api_keys::ApiKeysService;
    pub use super::audiences::AudiencesService;
    pub use super::contacts::ContactsService;
    pub use super::domains::DomainsService;
    pub use super::emails::EmailsService;
}

pub mod types {
    //! Request and response types.

    pub use super::api_keys::types::{ApiKey, ApiKeys, CreateApiKey, NewApiKey, Permission};
    pub use super::audiences::types::{Audience, Audiences, CreateAudienceResponse};
    pub use super::config::types::{ErrorKind, ErrorResponse};
    pub use super::contacts::types::{Contact, Contacts, CreateContact, UpdateContact};
    pub use super::domains::types::{
        CreateDomainRequest, CreateDomainResponse, DeleteDomainResponse, Domain, DomainChanges,
        DomainRecord, Domains, Region, UpdateDomainResponse, VerifyDomain,
    };
    pub use super::emails::types::{
        Attachment, ContentOrPath, Email, SendEmailBatchResponse, SendEmailRequest,
        SendEmailResponse, Tag,
    };
}

/// Error type for operations of a [`Client`].
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
