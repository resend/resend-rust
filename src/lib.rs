#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub use client::ResendClient;
pub(crate) use client::ResendClientInner;

mod client;
mod emails;

pub mod services {
    //! TODO.

    pub use super::emails::Emails;
}

pub mod types {
    //! TODO.

    pub use super::emails::types::*;
}

/// Failure during sending a request with a [`ResendClient`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

/// Specialized [`Result`] type for a [`ResendClient`].
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
