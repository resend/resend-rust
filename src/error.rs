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
    pub enum ErrorKind {
        /// Error name is not in the API spec.
        Unrecognized,

        /// 401 Unauthorized.
        ///
        /// - `missing_api_key`
        ///
        /// Missing API key in the authorization header.
        ///
        /// Include the following header `Authorization: Bearer YOUR_API_KEY` in the request.
        MissingApiKey,

        /// 403 Forbidden.
        ///
        /// - `invalid_api_key`
        ///
        /// The API key is not valid.
        ///
        /// Generate a new API key in the dashboard.
        InvalidApiKey,

        /// 403 Forbidden.
        ///
        /// - `invalid_from_address`
        ///
        /// The from address is not valid.
        ///
        /// Review your existing domains in the dashboard.
        InvalidFromAddress,

        /// 403 Forbidden.
        ///
        /// - `invalid_to_address`
        ///
        /// You can only send testing emails to your own email address.
        ///
        /// In order to send emails to any external address, you need to add a domain and
        /// use that as the `from` address instead of `onboarding@resend.dev`.
        InvalidToAddress,

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
        /// This endpoint does not support the specified HTTP method.
        ///
        /// Change the HTTP method to follow the documentation for the endpoint.
        MethodNotAllowed,

        /// 422 Unprocessable Content.
        ///
        /// - `missing_required_field`
        ///
        /// The request body is missing one or more required fields.
        /// Check the error message to see the list of missing fields.
        MissingRequiredField,

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
        /// - `invalid_scope`
        ///
        /// This endpoint does not support the specified scope.
        /// Change the scope to follow the documentation for the endpoint.
        InvalidScope,

        /// 429 Too Many Requests.
        ///
        /// - `rate_limit_exceeded`
        ///
        ///  Too many requests. Please limit the number of requests per second.
        /// Or contact support to increase rate limit.
        ///
        /// You should read the response headers and reduce the rate at which you request the API.
        /// This can be done by introducing a queue mechanism or reducing the number of
        /// concurrent requests per second.
        ///
        /// If you have specific requirements, contact support to request a rate increase.
        ///
        /// ## Note
        ///
        /// This should *never* be returned anymore as it's been replaced by the more detailed
        /// [`Error::RateLimit`](crate::Error::RateLimit).
        RateLimitExceeded,

        /// 429 Too Many Requests.
        ///
        /// - `daily_quota_exceeded`
        ///
        /// You have reached your daily email sending quota.
        ///
        /// Upgrade your plan to remove daily quota limit or
        /// wait until 24 hours have passed to continue sending.
        DailyQuotaExceeded,

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

    impl<T: AsRef<str>> From<T> for ErrorKind {
        fn from(value: T) -> Self {
            match value.as_ref() {
                "missing_api_key" => Self::MissingApiKey,
                "invalid_api_key" => Self::InvalidApiKey,
                "invalid_from_address" => Self::InvalidFromAddress,
                "invalid_to_address" => Self::InvalidToAddress,
                "not_found" => Self::NotFound,
                "method_not_allowed" => Self::MethodNotAllowed,
                "missing_required_field" => Self::MissingRequiredField,
                "invalid_attachment" => Self::InvalidAttachment,
                "invalid_scope" => Self::InvalidScope,
                "rate_limit_exceeded" => Self::RateLimitExceeded,
                "daily_quota_exceeded" => Self::DailyQuotaExceeded,
                "internal_server_error" => Self::InternalServerError,
                _ => Self::Unrecognized,
            }
        }
    }
}
