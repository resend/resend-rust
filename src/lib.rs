#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub use client::Client;
pub(crate) use client::Config;

mod api_keys;
mod client;
mod emails;

pub mod services {
    //! TODO.

    pub use super::api_keys::ApiKeys;
    pub use super::emails::Emails;
}

pub mod types {
    //! TODO.

    pub use super::api_keys::types::*;
    pub use super::emails::types::*;
}

/// Error type for operations of a [`Client`] client.
///
/// <https://resend.com/docs/api-reference/errors>
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("http client error: {0}")]
    Reqwest(#[from] reqwest::Error),
    // TODO: Remove Error::ParseUrl.
    #[error("url parsing error: {0}")]
    ParseUrl(#[from] url::ParseError),
}

/// Specialized [`Result`] type for an [`Error`].
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
