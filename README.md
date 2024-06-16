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

Emails are sent via the `Resend` client which provides both a synchronous and
asynchronous send method. The two are mutually exclusive and accessible via the
`blocking` feature. The crate uses [reqwest][reqwest] and [serde][serde]
internally.

### Documentation

Crate documentation is available in [docsrs][docs-url]. Example usage is available in the
[get started guide][get-started] on the Resend website, you can also find examples in the
[API reference][resend-api-ref].

### Features

- `blocking` to enable the blocking client.
- `native-tls` to use system-native TLS. **Enabled by default**.
- `rustls-tls` to use TLS backed by `rustls`.

### Variables

- `RESEND_API_KEY` to enable `impl Default` for a `Resend` client (Required).
- `RESEND_BASE_URL` to override the default base address:
  `https://api.resend.com` (Optional).
- `RESEND_RATE_LIMIT` to set the maximum amount of requests you can send per second. By default, this is
  9 (Resend defaults to 10). In reality, the time window is set to 1.1s to avoid
  failures. This is thread-safe (as long as you use the same `Resend` client across threads!)

> <div class="warning">WARNING: Rate limiting only works when using the async version (default) of the crate</div>

[action-badge]: https://img.shields.io/github/actions/workflow/status/resend/resend-rust/ci.yml
[action-url]: https://github.com/resend/resend-rust/actions/workflows/ci.yml
[crates-badge]: https://img.shields.io/crates/v/resend-rs
[crates-url]: https://crates.io/crates/resend-rs
[docs-badge]: https://img.shields.io/docsrs/resend-rs
[docs-url]: https://docs.rs/resend-rs
[reqwest]: https://github.com/seanmonstar/reqwest
[serde]: https://github.com/serde-rs/serde
[get-started]: https://resend.com/docs/send-with-rust
[resend-api-ref]: https://resend.com/api-reference
