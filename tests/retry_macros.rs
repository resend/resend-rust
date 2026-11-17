//! The exported macros must compile in a crate that has not imported
//! `RetryOptions` or `send_with_retry_opts` into scope.

#![allow(dead_code)]

#[cfg(not(feature = "blocking"))]
async fn uses_retry() -> resend_rs::Result<()> {
    resend_rs::retry!(async { Ok(()) })
}

#[cfg(not(feature = "blocking"))]
async fn uses_retry_opts() -> resend_rs::Result<()> {
    let opts = resend_rs::rate_limit::RetryOptions::default();
    resend_rs::retry_opts!(async { Ok(()) }, opts)
}

#[cfg(not(feature = "blocking"))]
#[tokio::test]
async fn macros_resolve_without_imports() {
    uses_retry().await.expect("Retry succeeded");
    uses_retry_opts().await.expect("Retry succeeded");
}
