# resend-rs

[![Crates.io](https://img.shields.io/crates/v/resend-rs)](https://crates.io/crates/resend-rs)
[![docs.rs](https://img.shields.io/docsrs/resend-rs)](https://docs.rs/resend-rs)

A _very_ minimal [Resend](https://resend.com) client for sending emails.

Emails are sent via the [`resend_client::ResendClient`] which provides both a
synchronous and asynchronous send method. The two are mutually exclusive and
accessible via the `blocking` feature. The crate uses
[`reqwest`](https://github.com/seanmonstar/reqwest) internally.

Currently, this only supports the `html` Resend parameter as I built this for my
own use and that's all I need. If anyone else is looking into this, however, I
would not mind expanding it.
