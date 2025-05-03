//! A few helpers for adding idempotency keys to requests that support them.
//!
//! ### Example
//!
//! ```rust,no_run
//! use resend_rs::{idempotent::IdempotentTrait, types::CreateEmailBaseOptions};
//! use resend_rs::{Resend, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!   let resend = Resend::new("re_123456789");
//!
//!   let emails = vec![
//!     CreateEmailBaseOptions::new(
//!       "Acme <onboarding@resend.dev>",
//!       vec!["foo@gmail.com"],
//!      "hello world",
//!    )
//!    .with_html("<h1>it works!</h1>"),
//!    CreateEmailBaseOptions::new(
//!      "Acme <onboarding@resend.dev>",
//!      vec!["bar@outlook.com"],
//!      "world hello",
//!    )
//!    .with_html("<p>it works!</p>"),
//!  ].with_idempotency_key("welcome-user/123456789");
//!
//!  let _emails = resend.batch.send(emails).await?;
//!
//!  Ok(())
//!}
//! ```
use crate::types::CreateEmailBaseOptions;

/// Wrapper struct for adding an `idempotency_key` header to data `T`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Idempotent<T> {
    #[serde(skip)]
    pub(crate) idempotency_key: Option<String>,
    #[serde(flatten)]
    pub(crate) data: T,
}

/// Implements `From<inner>` only works for concrete types.
macro_rules! idempotent_from {
    ($inner:ty) => {
        impl From<$inner> for Idempotent<$inner> {
            fn from(value: $inner) -> Self {
                Self {
                    idempotency_key: None,
                    data: value,
                }
            }
        }
    };
}

idempotent_from!(CreateEmailBaseOptions);

/// Used to add easy conversion of trait impls to [`Idempotent`].
pub trait IdempotentTrait<T> {
    /// Adds an `Idempotency-Key` header to the request.
    fn with_idempotency_key(self, idempotency_key: &str) -> Idempotent<T>;
}

impl<T> IdempotentTrait<Self> for T
where
    T: IntoIterator<Item = CreateEmailBaseOptions> + Send,
{
    fn with_idempotency_key(self, idempotency_key: &str) -> Idempotent<Self> {
        Idempotent {
            idempotency_key: Some(idempotency_key.to_owned()),
            data: self,
        }
    }
}
