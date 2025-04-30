# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Updated `ErrorKind` to match <https://resend.com/docs/api-reference/errors>

### Removed

- `ErrorKind::InvalidToAddress`
- `ErrorKind::InvalidScope`

## [0.14.1] - 2025-04-29

### Added

- `CreateDomainOptions::with_custom_return_path`

## [0.14.0] - 2025-04-22

### Added

- `contacts::get_by_id` and `contacts::get_by_email`

### Changed

- Deprecated `contacts::get` in favor of `contacts::get_by_id` and `contacts::get_by_email`

### Removed

- Removed deprecated `contacts::update` function (use `contacts::update_by_id`/`contacts::update_by_email`)

## [0.13.0] - 2025-04-20

### Added

- `CreateEmailBaseOptions::with_idempotency_key`

### Changed

- Moved `SendEmailBatchResponse` from `email` to `batch` module
- Made `LocatedError`, `DebugResult` and `CLIENT` (all used for testing internals) private (they
  were accidentally public before, very unlikely they were used by anyone)

## [0.12.1] - 2025-03-26

### Added

- `broadcasts::update` and `UpdateBroadcastOptions`, `UpdateBroadcastResponse`

## [0.12.0] - 2025-02-25

### Added

- WASM support

## [0.11.2] - 2025-01-18

### Added

- `ErrorKind` now implements `PartialEq, Eq` (https://github.com/resend/resend-rust/pull/23)

## [0.11.1] - 2025-01-16

### Added

- `Contacts::update_by_id`, `Contacts::update_by_email`

### Changed

- `Contacts::update` has been deprecated in favor of the above methods. The ability to use emails
  instead of ids was just added to the Resend backend.

## [0.11.0] - 2025-01-13

### Added

- `BroadcastId::new`

### Changed

- Made the following methods require `&str` instead of their specific ID types. This was changed to
  make their usage a bit simpler from the user's perspective, after all, the ID types were meant to
  be more like hints that a string is of an id type, rather than having specific properties in the
  program itself. The ID types are still used in the return types for this reason and they are
  also all constructable in case you want to use them. All the ID types dereference to a string
  so `&IdType` is equivalent to a `&str` (you might need to make this change if you were
  using any ID types directly before).
  - `ApiKeys::delete`
  - `Audiences::delete`
  - `Broadcasts::get,delete`
  - `Domains::delete`
- `SendBroadcastOptions::new` is no longer `const`

## [0.10.0] - 2024-12-13

### Added

- `broadcasts` module
- `CreateEmailBaseOptions::with_reply_multiple`

### Changed

- Made `CreateEmailBaseOptions.reply_to` field private
- The following changes were made to ensure IDs are consumed upon deletion to avoid accidetally
  reusing them. Of course it is still possible to clone them if necessary.
  - `api_keys::delete` now takes `ApiKeyId` instead of `&str`
  - `audiences::delete` now takes `AudienceId` instead of `&str`
  - `domains::delete` now takes `DomainId` instead of `&str`

## [0.9.2] - 2024-11-29

### Added

- Added webhook support with the `events` module

## [0.9.1] - 2024-08-13

### Changed

- renamed `CreateEmailBaseOptions::with_scheduled` to `with_scheduled_at`
- renamed `emails::cancel_schedule` to `cancel`

## [0.9.0] - 2024-08-12

Yanked due to some method naming, use `0.9.1` instead.

### Added

- `email.scheduled_at` field (and `emails::with_scheduled` method for setting it)
- `emails::update` method
- `emails::cancel_schedule` method

### Changed

- The following structs had all their fields made private in order to prevent future breaking
  changes when new fields are added. Simply use the relevant builder methods instead.
  [GitHub issue](https://github.com/resend/resend-rust/issues/15).

  - `CreateEmailBaseOptions`
  - `ContactData`
  - `ContactChanges`
  - `CreateDomainOptions`
  - `DomainChanges`
  - `UpdateEmailOptions`

## [0.8.1] - 2024-07-11

### Changed

- `Email.text` is now optional

### Fixed

- The `cc`, `bcc` and `text` fields of the `Email` struct are nullable which broke deserialization.
  It now works as expected.

## [0.8.0] - 2024-07-05

### Added

- `rate_limit` module
- `Error::RateLimit`

## [0.7.0] - 2024-07-01

### Added

- `Attachment::with_content_type` method and `content_type` field
- `Error::Parse` variant

### Changed

- HTML server responses instead of proper JSON errors (which should mostly happen in outages)
  now have better error messages


## [0.6.0] - 2024-06-16

### Changed

- `RATE_LIMIT` to `RESEND_RATE_LIMIT` to avoid potential collisions with other libs

### Removed

- Outdated doc comment from `Client::user_agent`

## [0.5.2] - 2024-06-10

### Added

- `DomainChanges::with_tls` method
- docs for `Domain.Tls` enum options

## [0.5.1] - 2024-06-08

### Changed

- Moved Github URLs from `resend-rs` to `resend-rust` (no code changes)

## [0.5.0] - 2024-06-08

### Added

- `DomainChanges.tls`
- Rate limiting (default 9/s), configurable with `RATE_LIMIT` env variable (async only)

### Changed

- renamed `SendEmail` to `CreateEmailBaseOptions`
- renamed `email.retrieve` to `email.get`
- renamed `SendEmailResponse` to `CreateEmailResponse`
- renamed `ApiKeyData` to `CreateApiKeyOptions`
- renamed `domains.DomainData` to `CreateDomainOptions`
- `email.send` now returns a `CreateEmailResponse` instead of an `EmailId`
- `batch.send` now returns a `Vec<CreateEmailResponse>` instead of a `Vec<EmailId>`
- `audiences.create` now returns `CreateAudienceResponse` instead of `AudienceId`
- `contacts.update` now returns `UpdateContactResponse`
- `domains.update` now returns `UpdateDomainResponse`
- moved batch related stuff to a new module
- `email.send_batch` is now `batch.send`
- moved error stuff from `config` to their own `error` module
- removed `[...]Id` types from function arguments
- implemented `Deref<str>` for all `[...]Id` types
- `DomainRecord` has been converted to an enum for better/more detailed type handling
- `Domain.records` is now optional
- `Email.to` is now a vec
- `contacts.delete_by_email` and `contacts.delete_by_contact_id` now return the `deleted` boolean
- Renamed `Client` to `Resend`
- `Domain` no longer has the `dns_provider` field.
- `Domain.delete` now returns a `DeleteDomainResponse` 
- `Email.Tag.value` is no longer an optional
- `Email.html` and `Email.reply_to` are now both optional

### Deleted

- removed ability to configure user agent via `RESEND_USER_AGENT` (this is no longer configuable)
- `email.with_value`, use the `new` constructor instead
- `impl<T: AsRef<str>> From<T> for Tag` is removed since Tag now also needs a value
- `ContactChanges.email` and `ContactChanges.with_email`

## [0.4.0] - 2024-05-01

`@martsokha` basically rewrote the entire repository 0_0
([pr](https://github.com/resend/resend-rust/pull/1))

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

[0.14.1]: https://crates.io/crates/resend-rs/0.14.1
[0.14.0]: https://crates.io/crates/resend-rs/0.14.0
[0.13.0]: https://crates.io/crates/resend-rs/0.13.0
[0.12.1]: https://crates.io/crates/resend-rs/0.12.1
[0.12.0]: https://crates.io/crates/resend-rs/0.12.0
[0.11.2]: https://crates.io/crates/resend-rs/0.11.2
[0.11.1]: https://crates.io/crates/resend-rs/0.11.1
[0.11.0]: https://crates.io/crates/resend-rs/0.11.0
[0.10.0]: https://crates.io/crates/resend-rs/0.10.0
[0.9.2]: https://crates.io/crates/resend-rs/0.9.2
[0.9.1]: https://crates.io/crates/resend-rs/0.9.1
[0.9.0]: https://crates.io/crates/resend-rs/0.9.0
[0.8.1]: https://crates.io/crates/resend-rs/0.8.1
[0.8.0]: https://crates.io/crates/resend-rs/0.8.0
[0.7.0]: https://crates.io/crates/resend-rs/0.7.0
[0.6.0]: https://crates.io/crates/resend-rs/0.6.0
[0.5.2]: https://crates.io/crates/resend-rs/0.5.2
[0.5.1]: https://crates.io/crates/resend-rs/0.5.1
[0.5.0]: https://crates.io/crates/resend-rs/0.5.0
[0.4.0]: https://crates.io/crates/resend-rs/0.4.0
[0.3.0]: https://crates.io/crates/resend-rs/0.3.0
[0.2.0]: https://crates.io/crates/resend-rs/0.2.0
[0.1.0]: https://crates.io/crates/resend-rs/0.1.0
