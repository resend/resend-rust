#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

pub(crate) use client::Config;
pub use client::Resend;

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

/// Error type for operations of a [`Resend`] client.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

/// Specialized [`Result`] type for an [`Error`].
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
