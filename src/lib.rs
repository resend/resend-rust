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
pub(crate) use client::Config;

mod api_keys;
mod audiences;
mod client;
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

    pub use super::api_keys::types::*;
    pub use super::audiences::types::*;
    pub use super::contacts::types::*;
    pub use super::domains::types::*;
    pub use super::emails::types::*;
}

/// Error type for operations of a [`Client`].
///
/// <https://resend.com/docs/api-reference/errors>
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("http client error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

/// Specialized [`Result`] type for an [`Error`].
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
