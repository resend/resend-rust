//! Run this example with:
//! 
//! ```sh
//! RESEND_API_KEY=re_your_api_key cargo run --example custom-config
//! ```

use anyhow::Context;
use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::{Config, Resend};
use std::time::Duration;

const EMAIL_SEND_TIMEOUT: Duration = Duration::from_secs(10);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = std::env::var("RESEND_API_KEY").context(
        "RESEND_API_KEY is expected in the environment for this example to run properly",
    )?;

    let resend = Resend::with_config(
        Config::builder(api_key)
            .base_url(
                // this is Resend's default base url, but you can provide
                // your override here, which is especially helpful when running
                // numerous parallel tests and intercepting email requests
                // in each of them
                "https://api.resend.com"
                    .parse()
                    .context("failed to parse URL")?,
            )
            .client(
                reqwest::Client::builder()
                    .timeout(EMAIL_SEND_TIMEOUT)
                    .build()
                    .context("failed to instrantiate an http client")?,
            )
            .build(),
    );

    let resp = resend
        .emails
        .send(
            CreateEmailBaseOptions::new("onboarding@resend.dev", ["delivered@resend.dev"], "Demo")
                .with_html("<strong>It works!</strong>")
                .with_text("It works!"),
        )
        .await?;

    Ok(())
}
