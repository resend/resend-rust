//! A *very* minimal [Resend](https://resend.com) client for sending emails.
//!
//! Emails are sent via the [`resend_client::ResendClient`] which provides both a synchronous and asynchronous
//! send method. The two are mutually exclusive and accessible via the `async` feature. The crate
//! uses [`reqwest`](https://github.com/seanmonstar/reqwest) internally.
//!
//! Currently this only supports the `html` Resend parameter as I built this for my own use and
//! that's all I need. If anyone else is looking into this however, I would not mind expanding it.

pub mod error;
pub mod mail;
pub mod resend_client;
