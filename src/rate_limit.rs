//! Helper methods for retrying requests in case of a rate limit error.
//!
//! The [`retry!`](crate::retry!) and [`retry_opts!`](crate::retry_opts) macros are also implemented
//! as slightly-less-verbose alternatives.

use crate::{Error, Result};
use rand::Rng;
use std::{future::Future, ops::Range, time::Duration};

/// Configuration options for retrying requests.
#[derive(Debug, Clone)]
pub struct RetryOptions {
    /// The amount of milliseconds to wait between requests.
    pub duration_ms: u64,
    /// The range of random jitter to be added on top of `duration_ms`.
    pub jitter_range_ms: Range<u64>,
    /// Maximum amount of retries before returning an error.
    pub max_retries: u32,
}

impl Default for RetryOptions {
    fn default() -> Self {
        Self {
            duration_ms: 1000,
            jitter_range_ms: 0..30,
            max_retries: 3,
        }
    }
}

#[allow(clippy::too_long_first_doc_paragraph)] // It really is not that long though
/// Helper method that executes the passed function. If the function returns [`Ok`],
/// or a non-rate limit related [`Err`] the result is returned immediately. If the function
/// errors due to rate limits, the function will be retried with the specified [`RetryOptions`].
///
/// ## Example
///
/// Listing your API keys:
///
/// ```rust
/// use resend_rs::{
///  rate_limit::{send_with_retry, send_with_retry_opts, RetryOptions},
///  types::CreateEmailBaseOptions,
///};
///use resend_rs::{Resend, Result};
///
///#[tokio::main]
///async fn main() -> Result<()> {
///  let resend = Resend::default();
///
///  let retry_opts = RetryOptions::default();
///  let response = send_with_retry_opts(|| resend.api_keys.list(), &retry_opts).await;
///  assert!(response.is_ok());
///
///  Ok(())
///}
/// ```
///
/// Sending an email:
///
/// ```rust,no_run
/// use resend_rs::{
///  rate_limit::{send_with_retry, send_with_retry_opts, RetryOptions},
///  types::CreateEmailBaseOptions,
///};
///use resend_rs::{Resend, Result};
///
///#[tokio::main]
///async fn main() -> Result<()> {
///  let resend = Resend::default();
///
///  // Create email
///  let from = "Acme <onboarding@resend.dev>";
///  let to = ["delivered@resend.dev"];
///  let subject = "Hello World";
///
///  let email =
///    CreateEmailBaseOptions::new(from, to, subject).with_html("<strong>It works!</strong>");
///
///  // Try to send it using default options
///  let retry_opts = RetryOptions::default();
///  let response = send_with_retry_opts(|| resend.emails.send(email.clone()), &retry_opts).await;
///  assert!(response.is_ok());
///
///  Ok(())
///}
/// ```
pub async fn send_with_retry_opts<A: Future<Output = Result<B>> + Send, B: Send>(
    f: impl Fn() -> A + Send,
    opts: &RetryOptions,
    // This is used to test the recursion depth
    #[cfg(test)] retry_count: &mut u32,
) -> Result<B> {
    let res = f().await;

    if let Err(Error::RateLimit {
        ratelimit_limit: _,
        ratelimit_remaining: _,
        ratelimit_reset,
    }) = res
    {
        // Base case
        if opts.max_retries == 0 {
            return res;
        }

        #[cfg(test)]
        dbg!("Failed send, trying again...");

        // Decrement retries and try again
        let opts = RetryOptions {
            duration_ms: opts.duration_ms,
            jitter_range_ms: opts.jitter_range_ms.clone(),
            max_retries: opts.max_retries.saturating_sub(1),
        };

        let sleep_millis = ratelimit_reset.map_or(opts.duration_ms, |r| r.saturating_mul(1000));
        let jitter = rand::rng().random_range(opts.jitter_range_ms.clone());
        std::thread::sleep(Duration::from_millis(sleep_millis + jitter));

        #[cfg(test)]
        {
            *retry_count += 1;
        }

        Box::pin(send_with_retry_opts(
            f,
            &opts,
            #[cfg(test)]
            retry_count,
        ))
        .await
    } else {
        res
    }
}

/// Same as [`send_with_retry_opts`] but uses [`RetryOptions::default`].
#[allow(dead_code)]
pub async fn send_with_retry<A: Future<Output = Result<B>> + Send, B: Send>(
    f: impl Fn() -> A + Send,
) -> Result<B> {
    send_with_retry_opts(
        f,
        &RetryOptions::default(),
        #[cfg(test)]
        &mut 0,
    )
    .await
}

/// Equivalent to [`send_with_retry`].
///
/// ## Example
///
/// ```rust
/// use resend_rs::{
///   rate_limit::{send_with_retry_opts, RetryOptions},
///   retry,
/// };
/// use resend_rs::{Resend, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///   let resend = Resend::default();
///
///   let response = retry!(resend.api_keys.list());
///   assert!(response.is_ok());
///
///   Ok(())
/// }
/// ```
#[macro_export]
macro_rules! retry {
    ( $f:expr ) => {{
        let retry_opts = RetryOptions::default();
        send_with_retry_opts(|| $f, &retry_opts).await
    }};
}

/// Equivalent to [`send_with_retry_opts`].
///
/// ## Example
///
/// ```rust
/// use resend_rs::{
///   rate_limit::{send_with_retry_opts, RetryOptions},
///   retry_opts,
/// };
/// use resend_rs::{Resend, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///   let resend = Resend::default();
///
///   let retry_opts = RetryOptions::default();
///   let response = retry_opts!(resend.api_keys.list(), retry_opts);
///   assert!(response.is_ok());
///
///   Ok(())
/// }
/// ```
#[macro_export]
macro_rules! retry_opts {
    ( $f:expr, $opts:expr ) => {{ send_with_retry_opts(|| $f, &$opts).await }};
}

#[cfg(test)]
#[allow(clippy::needless_return)]
mod tests {
    use super::{RetryOptions, send_with_retry_opts};
    use crate::Error;

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn test_retry_count_err() {
        let mut run_count = 0u32;

        let f = || async {
            let err = Error::RateLimit {
                ratelimit_limit: Some(10),
                ratelimit_remaining: Some(10),
                ratelimit_reset: Some(1),
            };
            Result::<(), Error>::Err(err)
        };
        let mut opts = RetryOptions::default();

        let res = send_with_retry_opts(f, &opts, &mut run_count).await;

        assert!(res.is_err());
        assert!(run_count == 3);

        run_count = 0;
        opts.max_retries = 2;
        let res = send_with_retry_opts(f, &opts, &mut run_count).await;
        assert!(res.is_err());
        assert!(run_count == 2);

        run_count = 0;
        opts.max_retries = 0;
        let res = send_with_retry_opts(f, &opts, &mut run_count).await;
        assert!(res.is_err());
        assert!(run_count == 0);
    }

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn test_retry_count_ok() {
        let mut retry_count = 0u32;

        let f = || async { Result::<(), Error>::Ok(()) };
        let opts = RetryOptions::default();

        let res = send_with_retry_opts(f, &opts, &mut retry_count).await;

        assert!(res.is_ok());
        assert!(retry_count == 0);
    }
}
