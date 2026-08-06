# Contributing

## Running the Tests

The tests make calls to the Resend API which means that the `RESEND_API_KEY` must be set in order to
run them. 

You will notice that some of the tests are slow, there's 2 reasons for this:

1. The tests all use a shared client that implements rate-limiting to avoid getting errors from
   the server
2. Creating a resource is not instant, so tests have to wait for the server to finish processing
   it before asserting on the result

For the second point, prefer polling over sleeping for a fixed amount of time. The API is
eventually consistent: it reports an intermediate state (a freshly created email is `queued`, for
example) until processing finishes, so sleeping and then asserting once is a race that slower
machines lose. Two helpers exist for this:

- `crate::test::retry` re-runs a fallible operation until it succeeds or the attempts run out
- `crate::test::wait_for_email_event` builds on it to poll until an email reaches a given
  `last_event`

There are still a number of `std::thread::sleep` calls left over from before those helpers
existed. Be aware that they block a worker thread of the shared runtime rather than yielding to
it, which also stalls whatever else is running there, so reach for `tokio::time::sleep` when a
plain delay really is what you want.

Note that because rate limiting only works for the non`blocking` feature, if more non-async tests
are added, it might be necessary to add thread sleeps.

## Missing/New Features

Before implementing or even suggesting a new feature, make sure to check the unreleased section of
[the Changelog](./CHANGELOG.md). The reason for that is that sometimes I might implement something
but not publish it immediately to avoid flooding crates.io with new releases every time a small
change is made. Open an issue about it and I'll be sure to cut a release!
