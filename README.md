# resend-rs

[![Crates.io](https://img.shields.io/crates/v/resend-rs)](https://crates.io/crates/resend-rs)
[![docs.rs](https://img.shields.io/docsrs/resend-rs)](https://docs.rs/resend-rs)

A minimal [Resend](https://resend.com) client.

Emails are sent via the [`Client`] which provides both a synchronous and
asynchronous send method. The two are mutually exclusive and accessible via the
`blocking` feature. The crate uses
[`reqwest`](https://github.com/seanmonstar/reqwest) internally.

Currently, this only supports the `html` Resend parameter as I built this for my
own use and that's all I need. If anyone else is looking into this, however, I
would not mind expanding it.

- `RESEND_BASE_URL`
- `RESEND_API_KEY`

#### Features

- `blocking` to enable the blocking client.
- `native-tls` to use system-native TLS. **Enabled by default**.
- `rustls-tls` to use TLS backed by rustls .

#### Examples

```rust
use resend_rs::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
//     let resend = Resend::default();
//     let _ = resend.emails.send().await?;

    Ok(())
}
```
