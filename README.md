## resend-rs

[![Build Status][action-badge]][action-url]
[![Crate Docs][docs-badge]][docs-url]
[![Crate Version][crates-badge]][crates-url]

A minimal [Resend](https://resend.com) client.

[action-badge]: https://img.shields.io/github/actions/workflow/status/AntoniosBarotsis/resend-rs/ci.yml
[action-url]: https://github.com/spire-rs/AntoniosBarotsis/resend-rs/workflows/build.yaml
[crates-badge]: https://img.shields.io/crates/v/resend-rs
[crates-url]: https://crates.io/crates/resend-rs
[docs-badge]: https://img.shields.io/docsrs/resend-rs
[docs-url]: https://docs.rs/resend-rs

Emails are sent via the `Client` which provides both a synchronous and
asynchronous send method. The two are mutually exclusive and accessible via the
`blocking` feature. The crate uses [reqwest][reqwest] and [serde][serde]
internally.

[reqwest]: https://github.com/seanmonstar/reqwest
[serde]: https://github.com/serde-rs/serde

If anyone else is looking into this, however, I would not mind expanding it.

#### Features

- `blocking` to enable the blocking client.
- `native-tls` to use system-native TLS. **Enabled by default**.
- `rustls-tls` to use TLS backed by rustls .

#### Variables

- (Required) `RESEND_API_KEY` to enable `impl Default` for a `Client`.
- (Optional) `RESEND_BASE_URL` to override the default base address:
  `https://api.resend.com`.
- (Optional) `RESEND_USER_AGENT` to override the default user-agent:
  `resend-rs/0.1.0`.
