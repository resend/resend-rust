## resend-rs

[![Build Status][action-badge]][action-url]
[![Crate Docs][docs-badge]][docs-url]
[![Crate Version][crates-badge]][crates-url]

A minimal [Resend](https://resend.com) client.

Add with:

```sh
cargo add resend-rs
cargo add tokio -F macros,rt-multi-thread
```

[action-badge]: https://img.shields.io/github/actions/workflow/status/resend/resend-rs/ci.yml
[action-url]: https://github.com/resend/resend-rs/actions/workflows/ci.yml
[crates-badge]: https://img.shields.io/crates/v/resend-rs
[crates-url]: https://crates.io/crates/resend-rs
[docs-badge]: https://img.shields.io/docsrs/resend-rs
[docs-url]: https://docs.rs/resend-rs

Emails are sent via the `Resend` client which provides both a synchronous and
asynchronous send method. The two are mutually exclusive and accessible via the
`blocking` feature. The crate uses [reqwest][reqwest] and [serde][serde]
internally.

[reqwest]: https://github.com/seanmonstar/reqwest
[serde]: https://github.com/serde-rs/serde

#### Features

- `blocking` to enable the blocking client.
- `native-tls` to use system-native TLS. **Enabled by default**.
- `rustls-tls` to use TLS backed by `rustls`.

#### Variables

- `RESEND_API_KEY` to enable `impl Default` for a `Resend` client (Required).
- `RESEND_BASE_URL` to override the default base address:
  `https://api.resend.com` (Optional).
- `RATE_LIMIT` to set the maximum amount of requests you can send per second. By default, this is
  10 as that is what Resend defaults to. In reality, the time window is set to 1.1s to avoid
  failures. This is thread-safe (as long as you use the same `Resend` client across threads!)

> <div class="warning">WARNING: Rate limiting only works when using the async version (default) of the crate</div>
