# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- ## [Unreleased] -->

## [Unreleased]

### Changed

- renamed `SendEmail` to `CreateEmailBaseOptions`
- renamed `email.retrieve` to `email.get`
- renamed `SendEmailResponse` to `CreateEmailResponse`
- moved batch related stuff to a new module
- `email.send` now returns a `CreateEmailResponse` instead of an `EmailId`
- `batch.send` now returns a `Vec<CreateEmailResponse>` instead of a `Vec<EmailId>`
- `email.send_batch` is now `batch.send`
- renamed `ApiKeyData` to `CreateApiKeyOptions`
- `audiences.create` now returns `CreateAudienceResponse` instead of `AudienceId`
- `contacts.update` now returns `UpdateContactResponse`
- renamed `domains.DomainData` to `CreateDomainOptions`
- `domains.update` now returns `UpdateDomainResponse`

### Deleted

- removed ability to configure user agent via `RESEND_USER_AGENT` (this is no longer configuable)

## [0.4.0] - 2024-05-01

`@martsokha` basically rewrote the entire repository 0_0
([pr](https://github.com/AntoniosBarotsis/resend-rs/pull/1))

The crate now supports the entire Resend API surface, check the
[docs](https://docs.rs/resend-rs/latest/resend_rs/) for examples.

## [0.3.0] - 2024-02-06

### Changed

- `Mail::new` now accepts a list of `to` addresses and thus supports sending an
  email to multiple recipients.

  ```rs
  // Before:
  let mail = Mail::new("from1", "to1", "subject1", "html1");

  // Now
  let mail = Mail::new("from1", &["to1"], "subject1", "html1");
  ```

## [0.2.0] - 2023-07-10

Disabled `reqwest`'s default features and enabled `rustls-tls`.

## [0.1.0] - 2023-07-10

Initial release.

[0.4.0]: https://crates.io/crates/resend-rs/0.4.0
[0.3.0]: https://crates.io/crates/resend-rs/0.3.0
[0.2.0]: https://crates.io/crates/resend-rs/0.2.0
[0.1.0]: https://crates.io/crates/resend-rs/0.1.0
