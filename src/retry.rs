// TODO: Some tests
// TODO: Docs, examples

use crate::{Error, Result};
use rand::Rng;
use std::{future::Future, ops::Range, time::Duration};

#[derive(Debug)]
pub struct RetryOptions {
    duration_ms: u64,
    jitter_range_ms: Range<u64>,
}

impl Default for RetryOptions {
    fn default() -> Self {
        Self {
            duration_ms: 1000,
            jitter_range_ms: 0..30,
        }
    }
}

#[allow(dead_code)]
pub async fn send_with_retry_opts<A: Future<Output = Result<B>> + Send, B: Send>(
    f: impl Fn() -> A + Send,
    opts: &RetryOptions,
) -> Result<B> {
    let res = f().await;

    if let Err(Error::RateLimit {
        ratelimit_limit: _,
        ratelimit_remaining: _,
        ratelimit_reset,
    }) = res
    {
        let sleep_millis = ratelimit_reset.map_or(opts.duration_ms, |r| r.saturating_mul(1000));
        let jitter = rand::thread_rng().gen_range(opts.jitter_range_ms.clone());
        std::thread::sleep(Duration::from_millis(sleep_millis + jitter));

        Box::pin(send_with_retry(f)).await
    } else {
        res
    }
}

#[allow(dead_code)]
pub async fn send_with_retry<A: Future<Output = Result<B>> + Send, B: Send>(
    f: impl Fn() -> A + Send,
) -> Result<B> {
    send_with_retry_opts(f, &RetryOptions::default()).await
}
