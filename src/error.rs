#[allow(unreachable_pub)]
pub mod types {
    use serde::{Deserialize, Serialize};

    /// Error returned as a response.
    ///
    /// <https://resend.com/docs/api-reference/errors>
    #[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
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
    #[cfg_attr(
        test,
        derive(strum::EnumCount, strum::VariantNames),
        strum(serialize_all = "snake_case")
    )]
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

        ValidationError,

        /// 401 Unauthorized.
        ///
        /// - `missing_api_key`
        ///
        /// Missing API key in the authorization header.
        ///
        /// Include the following header `Authorization: Bearer YOUR_API_KEY` in the request.
        MissingApiKey,

        RestrictedApiKey,

        /// 403 Forbidden.
        ///
        /// - `email_above_quota`
        ///
        /// You can’t retrieve this email’s content because it was above quota when received.
        ///
        /// [Upgrade your plan] to increase your quota.
        ///
        /// [Upgrade your plan]: https://resend.com/settings/billing
        EmailAboveQuota,

        /// 403 Forbidden.
        ///
        /// - `invalid_permission`
        ///
        /// Access token is missing required scopes.
        ///
        /// Request an access token that includes the scopes required by this endpoint.
        InvalidPermission,

        /// 403 Forbidden.
        ///
        /// - `suspended_api_key`
        ///
        /// This API key is suspended
        ///
        /// [Contact support] if you believe this is a mistake.
        ///
        /// [Contact support]: https://resend.com/contact
        SuspendedApiKey,

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
        /// - `concurrent_idempotent_requests`
        ///
        /// Same idempotency key used while original request is still in progress.
        ///
        /// Try the request again later.
        ConcurrentIdempotentRequests,

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
        /// - `resource_locked`
        ///
        /// Another request is already updating this resource.
        ///
        /// Retry the request after a short delay.
        ResourceLocked,

        /// 422 Unprocessable Content.
        ///
        /// - `invalid_attachment`
        ///
        /// Attachment must have either a `content` or `path`.
        ///
        /// Attachments must either have a `content` (strings, Buffer, or Stream contents) or
        /// `path` to a remote resource (better for larger attachments).
        InvalidAttachment,

        /// 422 Unprocessable Content
        ///
        /// - `invalid_parameter`
        ///
        /// The parameter must be a valid UUID.
        ///
        /// Check the value and make sure it’s valid.
        InvalidParameter,

        /// 422 Unprocessable Content.
        ///
        /// - `missing_required_field`
        ///
        /// The request body is missing one or more required fields.
        ///
        /// Check the error message to see the list of missing fields.
        MissingRequiredField,

        /// 422 Unprocessable Content.
        ///
        /// - `missing_required_parameter`
        ///
        /// The request is missing one or more required parameters.
        ///
        /// Check the error message to see the list of missing parameters.
        MissingRequiredParameter,

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
        /// - `monthly_quota_exceeded`
        ///
        /// You have reached your monthly email sending quota.
        ///
        ///  Upgrade your plan to remove the increase the monthly sending limit.
        MonthlyQuotaExceeded,

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

        /// 500 Internal Server Error
        ///
        /// - `application_error`
        ///
        /// An unexpected error occurred.
        ///
        /// Try the request again later. If the error does not resolve, check our status page
        /// for service updates.
        ApplicationError,

        /// 500 Service Unavailable
        ///
        /// - `service_unavailable`
        ///
        /// API is temporarily unavailable
        ///
        /// Try the request again later. Check our [status page] for service updates.
        ///
        /// [status page]: https://resend-status.com/
        ServiceUnavailable,
    }

    impl From<ErrorResponse> for ErrorKind {
        fn from(value: ErrorResponse) -> Self {
            Self::from(value.name)
        }
    }

    impl<T: AsRef<str>> From<T> for ErrorKind {
        fn from(value: T) -> Self {
            match value.as_ref() {
                "invalid_idempotency_key" => Self::InvalidIdempotencyKey,
                "validation_error" => Self::ValidationError,
                "missing_api_key" => Self::MissingApiKey,
                "restricted_api_key" => Self::RestrictedApiKey,
                "email_above_quota" => Self::EmailAboveQuota,
                "invalid_permission" => Self::InvalidPermission,
                "suspended_api_key" => Self::SuspendedApiKey,
                "not_found" => Self::NotFound,
                "method_not_allowed" => Self::MethodNotAllowed,
                "concurrent_idempotent_requests" => Self::ConcurrentIdempotentRequests,
                "invalid_idempotent_request" => Self::InvalidIdempotentRequest,
                "resource_locked" => Self::ResourceLocked,
                "invalid_attachment" => Self::InvalidAttachment,
                "invalid_parameter" => Self::InvalidParameter,
                "missing_required_field" => Self::MissingRequiredField,
                "missing_required_parameter" => Self::MissingRequiredParameter,
                "daily_quota_exceeded" => Self::DailyQuotaExceeded,
                "monthly_quota_exceeded" => Self::MonthlyQuotaExceeded,
                "rate_limit_exceeded" => Self::RateLimitExceeded,
                "application_error" => Self::ApplicationError,
                "service_unavailable" => Self::ServiceUnavailable,
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
    #[serial_test::serial]
    #[cfg(not(feature = "blocking"))]
    async fn errors_up_to_date() {
        use std::collections::HashSet;

        use strum::VariantNames;

        use crate::types::{ErrorKind, ErrorResponse};

        let response = reqwest::get("https://resend.com/docs/api-reference/errors")
            .await
            .unwrap();

        let html = response.text().await.unwrap();

        let fragment = scraper::Html::parse_document(&html);
        let selector = scraper::Selector::parse("h3 > span").unwrap();

        let re = regex::Regex::new(r"<code>(\w+)</code>").unwrap();

        let expected = fragment
            .select(&selector)
            .map(|el| el.inner_html())
            .filter(|el| el.starts_with("<code>"))
            .flat_map(|inner| {
                let mut results = vec![];
                for (_, [error]) in re.captures_iter(&inner).map(|c| c.extract()) {
                    results.push(error.to_string());
                }
                results
            })
            .collect::<HashSet<_>>();

        let enum_names = ErrorKind::VARIANTS
            .iter()
            .filter(|&&name| name != "unrecognized") // IGNORE unrecognized
            .map(|&name| name.to_string())
            .collect::<HashSet<_>>();

        // Make sure no error is parsed as `ErrorKind::Unrecognized`
        for error_name in &expected {
            let error_response = ErrorResponse {
                status_code: 400,
                message: String::new(),
                name: error_name.clone(),
            };

            let error_kind = ErrorKind::from(error_response);
            assert!(
                !matches!(error_kind, ErrorKind::Unrecognized),
                "Unrecognized: {error_name}"
            );
        }

        // Print inconsistencies
        let missing_from_enum = expected.difference(&enum_names).collect::<Vec<_>>();
        let extra_in_enum = enum_names.difference(&expected).collect::<Vec<_>>();

        if !missing_from_enum.is_empty() {
            println!("On the page but missing from ErrorKind:");
            for name in missing_from_enum {
                println!("  - {name}");
            }
        }
        if !extra_in_enum.is_empty() {
            println!("In ErrorKind but not on the page:");
            for name in extra_in_enum {
                println!("  - {name}");
            }
        }

        assert_eq!(expected.len(), enum_names.len());
    }
}
