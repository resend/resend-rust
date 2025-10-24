#[allow(unreachable_pub)]
pub mod types {
    use serde::Deserialize;

    /// Error returned as a response.
    ///
    /// <https://resend.com/docs/api-reference/errors>
    #[derive(Debug, Clone, Deserialize, thiserror::Error)]
    #[error("{name}: {message}")]
    pub struct ErrorResponse {
        #[serde(rename = "statusCode")]
        pub status_code: u16,
        pub message: String,
        pub name: String,
    }

    impl ErrorResponse {
        /// Returns the [`ErrorKind`].
        #[must_use]
        pub fn kind(&self) -> ErrorKind {
            ErrorKind::from(self.name.as_str())
        }
    }

    /// Error type for operations of a [`Resend`] client.
    ///
    /// <https://resend.com/docs/api-reference/errors>
    ///
    /// [`Resend`]: crate::Resend
    #[non_exhaustive]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(test, derive(strum::EnumCount))]
    pub enum ErrorKind {
        /// Error name is not in the API spec.
        Unrecognized,

        /// 400 Bad Request.
        ///
        /// - `invalid_idempotency_key`
        ///
        /// The key must be between 1-256 chars.
        ///
        /// Retry with a valid idempotency key.
        InvalidIdempotencyKey,

        /// 400 Bad Request.
        ///
        /// - `validation_error`
        ///
        /// We found an error with one or more fields in the request.
        ///
        /// The message will contain more details about what field and error were found.
        ValidationError400,

        /// 401 Unauthorized.
        ///
        /// - `missing_api_key`
        ///
        /// Missing API key in the authorization header.
        ///
        /// Include the following header `Authorization: Bearer YOUR_API_KEY` in the request.
        MissingApiKey,

        /// 401 Unauthorized
        ///
        /// - `restricted_api_key`
        ///
        /// This API key is restricted to only send emails.
        ///
        /// Make sure the API key has `Full access` to perform actions other than sending emails.
        RestrictedApiKey,

        /// 403 Forbidden.
        ///
        /// - `invalid_api_key`
        ///
        /// API key is invalid.
        ///
        /// Make sure the API key is correct or generate a new [API key in the dashboard].
        ///
        /// [API key in the dashboard]: https://resend.com/api-keys
        InvalidApiKey,

        /// 403 Forbidden.
        ///
        /// - `validation_error`
        ///
        /// You can only send testing emails to your own email address (`youremail@domain.com`).
        ///
        /// In [Resend's Domain page], add and verify a domain for
        /// which you have DNS access. This allows you to send emails to addresses beyond your own.
        ///
        /// [Resend's Domain page]: https://resend.com/domains
        ValidationError403,

        /// 404 Not Found.
        ///
        /// - `not_found`
        ///
        /// The requested endpoint does not exist.
        ///
        /// Change your request URL to match a valid API endpoint.
        NotFound,

        /// 405 Method Not Allowed.
        ///
        /// - `method_not_allowed`
        ///
        /// Method is not allowed for the requested path.
        ///
        /// Change your API endpoint to use a valid method.
        MethodNotAllowed,

        /// 409 Conflict
        ///
        /// - `invalid_idempotent_request`
        ///
        /// Same idempotency key used with a different request payload.
        ///
        /// Change your idempotency key or payload.
        InvalidIdempotentRequest,

        /// 409 Conflict
        ///
        /// - `concurrent_idempotent_requests`
        ///
        /// Same idempotency key used while original request is still in progress.
        ///
        /// Try the request again later.
        ConcurrentIdempotentRequests,

        /// 422 Unprocessable Content.
        ///
        /// - `invalid_attachment`
        ///
        /// Attachment must have either a `content` or `path`.
        ///
        /// Attachments must either have a `content` (strings, Buffer, or Stream contents) or
        /// `path` to a remote resource (better for larger attachments).
        InvalidAttachment,

        /// 422 Unprocessable Content.
        ///
        /// - `invalid_from_address`
        ///
        /// Invalid from field.
        ///
        /// Make sure the from field is a valid. The email address needs to follow the
        /// `email@example.com` or `Name <email@example.com>` format.
        InvalidFromAddress,

        /// 422 Unprocessable Content
        ///
        /// - `invalid_access`
        ///
        /// Access must be `"full_access" | "sending_access"`.
        ///
        /// Make sure the API key has necessary permissions.
        InvalidAccess,

        /// 422 Unprocessable Content
        ///
        /// - `invalid_parameter`
        ///
        /// The parameter must be a valid UUID.
        ///
        /// Check the value and make sure it’s valid.
        InvalidParameter,

        /// 422 Unprocessable Content
        ///
        /// - `invalid_region`
        ///
        /// Region must be `"us-east-1" | "us-east-1" | "sa-east-1"`.
        ///
        /// Make sure the correct region is selected.
        InvalidRegion,

        /// 422 Unprocessable Content.
        ///
        /// - `missing_required_field`
        ///
        /// The request body is missing one or more required fields.
        ///
        /// Check the error message to see the list of missing fields.
        MissingRequiredField,

        /// 429 Too Many Requests.
        ///
        /// - `monthly_quota_exceeded`
        ///
        /// You have reached your monthly email sending quota.
        ///
        ///  Upgrade your plan to remove the increase the monthly sending limit.
        MonthlyQuotaExceeded,

        /// 429 Too Many Requests.
        ///
        /// - `daily_quota_exceeded`
        ///
        /// You have reached your daily email sending quota.
        ///
        /// Upgrade your plan to remove the daily quota limit or wait
        /// until 24 hours have passed to continue sending.
        DailyQuotaExceeded,

        /// 429 Too Many Requests.
        ///
        /// - `rate_limit_exceeded`
        ///
        /// Too many requests. Please limit the number of requests per second.
        /// Or contact support to increase rate limit.
        ///
        /// You should read the response headers and reduce the rate at which you request the API.
        /// This can be done by introducing a queue mechanism or reducing the number of concurrent
        /// requests per second. If you have specific requirements, contact support to request a
        /// rate increase.
        ///
        /// ## Note
        ///
        /// This should *never* be returned anymore as it's been replaced by the more detailed
        /// [`Error::RateLimit`](crate::Error::RateLimit).
        RateLimitExceeded,

        /// 451 Unavailable For Legal Reasons
        ///
        /// - `security_error`
        ///
        /// We may have found a security issue with the request.
        ///
        /// The message will contain more details. Contact support for more information.
        SecurityError,

        /// 500 Internal Server Error
        ///
        /// - `application_error`
        ///
        /// An unexpected error occurred.
        ///
        /// Try the request again later. If the error does not resolve, check our status page
        /// for service updates.
        ApplicationError,

        /// 500 Internal Server Error.
        ///
        /// - `internal_server_error`
        ///
        /// An unexpected error occurred.
        ///
        /// Try the request again later. If the error does not resolve,
        /// check our [`status page`] for service updates.
        ///
        /// [`status page`]: https://resend-status.com/
        InternalServerError,
    }

    impl From<ErrorResponse> for ErrorKind {
        fn from(value: ErrorResponse) -> Self {
            // There exist 2 validation_error variants, differentiate via status code
            if value.name == "validation_error" {
                return match value.status_code {
                    400 => Self::ValidationError400,
                    403 => Self::ValidationError403,
                    _ => Self::Unrecognized,
                };
            }

            // For the rest use old From implementation.
            Self::from(value.name)
        }
    }

    impl<T: AsRef<str>> From<T> for ErrorKind {
        fn from(value: T) -> Self {
            match value.as_ref() {
                "invalid_idempotency_key" => Self::InvalidIdempotencyKey,
                "missing_api_key" => Self::MissingApiKey,
                "restricted_api_key" => Self::RestrictedApiKey,
                "invalid_api_key" => Self::InvalidApiKey,
                "not_found" => Self::NotFound,
                "method_not_allowed" => Self::MethodNotAllowed,
                "invalid_idempotent_request" => Self::InvalidIdempotentRequest,
                "concurrent_idempotent_requests" => Self::ConcurrentIdempotentRequests,
                "invalid_attachment" => Self::InvalidAttachment,
                "invalid_from_address" => Self::InvalidFromAddress,
                "invalid_access" => Self::InvalidAccess,
                "invalid_parameter" => Self::InvalidParameter,
                "invalid_region" => Self::InvalidRegion,
                "missing_required_field" => Self::MissingRequiredField,
                "monthly_quota_exceeded" => Self::MonthlyQuotaExceeded,
                "daily_quota_exceeded" => Self::DailyQuotaExceeded,
                "rate_limit_exceeded" => Self::RateLimitExceeded,
                "security_error" => Self::SecurityError,
                "application_error" => Self::ApplicationError,
                "internal_server_error" => Self::InternalServerError,
                _ => Self::Unrecognized,
            }
        }
    }
}

#[cfg(test)]
mod test {
    /// This test parses [all Resend errors] and makes sure [`crate::types::ErrorKind`] models
    /// them correctly, namely:
    ///
    /// - No error is parsed as [`crate::types::ErrorKind::Unrecognized`] (they are all recognized)
    /// - The amount of errors from the website + 1 (for the unrecognized variant) is equal to the
    ///   number of error variants in [`crate::types::ErrorKind`].
    ///
    /// There is a very real chance this will break in the future if anything changes in the
    /// structure of the errors page but for now it is useful to have to make sure all errors are
    /// modelled in the code.
    ///
    /// [all Resend errors]: https://resend.com/docs/api-reference/errors
    #[allow(clippy::unwrap_used)]
    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn errors_up_to_date() {
        use strum::EnumCount;

        use crate::types::{ErrorKind, ErrorResponse};

        let response = reqwest::get("https://resend.com/docs/api-reference/errors")
            .await
            .unwrap();

        let html = response.text().await.unwrap();

        let fragment = scraper::Html::parse_document(&html);
        let selector = scraper::Selector::parse("h3 > span").unwrap();

        let re = regex::Regex::new(r"<code>(\w+)</code>").unwrap();

        let actual = ErrorKind::COUNT;
        let expected = fragment
            .select(&selector)
            .map(|el| el.inner_html())
            .map(|inner| {
                let mut results = vec![];
                for (_, [error]) in re.captures_iter(&inner).map(|c| c.extract()) {
                    results.push(error.to_string());
                }
                results
            })
            .collect::<Vec<_>>();

        // Make sure no error is parsed as `ErrorKind::Unrecognized`
        for error_name in expected.iter().flatten() {
            let error_response = ErrorResponse {
                status_code: 400,
                message: String::new(),
                name: error_name.clone(),
            };

            let error_kind = ErrorKind::from(error_response);
            assert!(
                !matches!(error_kind, ErrorKind::Unrecognized),
                "Could not parse {error_name}"
            );
        }

        // Expected is actually one less than what we have because of the `Unrecognized` variant.
        let expected = expected.len() + 1;

        assert_eq!(actual, expected);
    }
}
