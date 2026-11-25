#[cfg(not(feature = "blocking"))]
use governor::{
    Quota, RateLimiter,
    clock::MonotonicClock,
    middleware::NoOpMiddleware,
    state::{InMemoryState, NotKeyed},
};
#[cfg(feature = "blocking")]
use reqwest::blocking::{Client, RequestBuilder, Response};
#[cfg(not(feature = "blocking"))]
use reqwest::{Client, RequestBuilder, Response};
use reqwest::{Method, Url};
use reqwest::{StatusCode, header::USER_AGENT};
use std::num::NonZeroU32;
use std::{env, fmt};
#[cfg(not(feature = "blocking"))]
use std::{sync::Arc, time::Duration};

use crate::{
    Error, Result,
    error::types::{ErrorKind, ErrorResponse},
};

/// Client-side rate limit setting for [`ConfigBuilder`].
#[derive(Debug, Clone, Copy)]
enum RateLimitConfig {
    /// Read `RESEND_RATE_LIMIT`, falling back to the default of 9 requests per 1.1 seconds.
    EnvOrDefault,
    /// Do not rate limit requests.
    Disabled,
    /// Limit to this many requests per 1.1 seconds.
    #[cfg_attr(feature = "blocking", allow(dead_code))]
    Limit(NonZeroU32),
}

#[cfg(doc)]
use crate::Resend;

/// Convenience builder for [`Config`].
///
/// This requires from you to set the API key ([`ConfigBuilder::new`]), but also
/// makes it possible to set a `reqwest` http client with your custom configuration
/// (see also [`Resend::with_client`]), override Resend's base url, and configure or
/// disable the client-side rate limiter.
///
/// ```no_run
/// # use resend_rs::ConfigBuilder;
/// let http_client = reqwest::Client::builder()
///      .timeout(std::time::Duration::from_secs(10))
///      .build()
///      .unwrap();
///
/// // Make sure to not store secrets in code, instead consider using crates like `dotenvy`
/// // or `secrecy`.
/// let _config = ConfigBuilder::new("re_...")
///     // this can be your proxy's url (if any) or a test server url which
///     // is intercepting request and allows to inspect them later on
///     .base_url("http://wiremock:35353".parse().unwrap())
///     .client(http_client)
///     .build();
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ConfigBuilder {
    api_key: String,
    base_url: Option<Url>,
    client: Option<Client>,
    rate_limit: RateLimitConfig,
}

impl ConfigBuilder {
    /// Create new [`ConfigBuilder`] with `api_key` set.
    pub fn new<S>(api_key: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            api_key: api_key.into(),
            base_url: None,
            client: None,
            rate_limit: RateLimitConfig::EnvOrDefault,
        }
    }

    /// Set a custom Resend's base url.
    ///
    /// This can be your proxy's url (if any) or a test server url which
    /// intercepting request and allows to inspect them later on.
    ///
    /// If not provided here, the `RESEND_BASE_URL` environment variable will be
    /// used. If that is not not provided either - a [default] url will be used.
    ///
    /// [default]: https://resend.com/docs/api-reference/introduction#base-url
    #[must_use]
    pub fn base_url(mut self, url: Url) -> Self {
        self.base_url = Some(url);
        self
    }

    /// Set custom http client.
    #[must_use]
    pub fn client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    /// Set the client-side rate limit, in requests per 1.1 seconds.
    ///
    /// Pass `None` to disable the limiter. If this method is not called, the
    /// `RESEND_RATE_LIMIT` environment variable is used, falling back to 9 requests per 1.1
    /// seconds.
    ///
    /// This only applies to the async client; the blocking client has no rate limiter.
    ///
    /// # Panics
    ///
    /// Building the config panics if the rate is above 1,100,000,000, because governor cannot
    /// represent a period shorter than one nanosecond. Pass `None` to disable the limiter.
    #[must_use]
    pub fn rate_limit(mut self, rate_limit: Option<NonZeroU32>) -> Self {
        self.rate_limit = rate_limit.map_or(RateLimitConfig::Disabled, RateLimitConfig::Limit);
        self
    }

    /// Builder's terminal method producing [`Config`].
    pub fn build(self) -> Config {
        Config::with_rate_limit(
            self.api_key,
            self.client.unwrap_or_default(),
            self.base_url,
            self.rate_limit,
        )
    }
}

/// Configuration for `Resend` client.
///
/// Use [`Config::builder`] to start constructing your custom configuration.
#[non_exhaustive]
#[derive(Clone)]
pub struct Config {
    pub(crate) user_agent: String,
    pub(crate) api_key: String,
    pub(crate) base_url: Url,
    pub(crate) client: Client,
    #[cfg(not(feature = "blocking"))]
    limiter: Option<Arc<Limiter>>,
}

#[cfg(not(feature = "blocking"))]
type Limiter = RateLimiter<
    NotKeyed,
    InMemoryState,
    MonotonicClock,
    NoOpMiddleware<<MonotonicClock as governor::clock::Clock>::Instant>,
>;

impl Config {
    /// Create new [`ConfigBuilder`] with `api_key` set.
    ///
    /// A convenience method, that, internally, will call [`ConfigBuilder::new`].
    pub fn builder<S>(api_key: S) -> ConfigBuilder
    where
        S: Into<String>,
    {
        ConfigBuilder::new(api_key.into())
    }

    /// Creates a new [`Config`].
    ///
    /// Note: the `base_url` parameter takes presedence over the `RESEND_BASE_URL` environment
    /// variable.
    #[must_use]
    pub(crate) fn new(api_key: String, client: Client, base_url: Option<Url>) -> Self {
        Self::with_rate_limit(api_key, client, base_url, RateLimitConfig::EnvOrDefault)
    }

    fn with_rate_limit(
        api_key: String,
        client: Client,
        base_url: Option<Url>,
        rate_limit: RateLimitConfig,
    ) -> Self {
        let env_base_url = base_url.unwrap_or_else(|| {
            env::var("RESEND_BASE_URL")
                .map_or_else(
                    |_| Url::parse("https://api.resend.com"),
                    |env_var| Url::parse(env_var.as_str()),
                )
                .expect("env variable `RESEND_BASE_URL` should be a valid URL")
        });

        let env_user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

        // ==== Rate limiting is a non-blocking thing only ====
        #[cfg(feature = "blocking")]
        let _ = rate_limit;

        #[cfg(not(feature = "blocking"))]
        let limiter = match rate_limit {
            RateLimitConfig::Disabled => None,
            RateLimitConfig::Limit(rate) => Some(Self::build_limiter(rate)),
            RateLimitConfig::EnvOrDefault => {
                let rate = env::var("RESEND_RATE_LIMIT")
                    .unwrap_or_else(|_| "9".to_owned())
                    .parse::<u32>()
                    .expect("env variable `RESEND_RATE_LIMIT` should be a valid u32");
                Some(Self::build_limiter(
                    NonZeroU32::new(rate).expect("Rate limit is a valid non zero u32"),
                ))
            }
        };
        // ====================================================

        Self {
            user_agent: env_user_agent,
            api_key,
            base_url: env_base_url,
            client,
            #[cfg(not(feature = "blocking"))]
            limiter,
        }
    }

    #[cfg(not(feature = "blocking"))]
    fn build_limiter(rate: NonZeroU32) -> Arc<Limiter> {
        Arc::new(RateLimiter::direct_with_clock(
            Self::rate_limit_quota(rate.get()),
            MonotonicClock,
        ))
    }

    #[cfg(not(feature = "blocking"))]
    fn rate_limit_quota(rate_limit: u32) -> Quota {
        let rate = NonZeroU32::new(rate_limit).expect("Rate limit is a valid non zero u32");
        // Governor replenishes one cell per period; burst size only sets capacity.
        Quota::with_period(Duration::from_millis(1100) / rate.get())
            .expect("Valid quota")
            .allow_burst(rate)
    }

    /// Constructs a new [`RequestBuilder`].
    pub(crate) fn build(&self, method: Method, path: &str) -> RequestBuilder {
        let mut url = self.base_url.clone();
        let base_path = url.path().trim_end_matches('/').to_owned();
        url.set_path(&format!("{}/{}", base_path, path.trim_start_matches('/')));

        self.client
            .request(method, url)
            .bearer_auth(self.api_key.as_str())
            .header(USER_AGENT, self.user_agent.as_str())
    }

    /// Builds the error for a 429 response.
    ///
    /// Quota errors (`daily_quota_exceeded`, `monthly_quota_exceeded`) are returned as
    /// [`Error::Resend`] since retrying won't help; anything else is a [`Error::RateLimit`].
    fn too_many_requests_error(headers: &reqwest::header::HeaderMap, body: String) -> Error {
        if let Ok(error) = serde_json::from_str::<ErrorResponse>(&body)
            && matches!(
                error.kind(),
                ErrorKind::DailyQuotaExceeded | ErrorKind::MonthlyQuotaExceeded
            )
        {
            return Error::Resend {
                status: StatusCode::TOO_MANY_REQUESTS.as_u16(),
                error,
            };
        }

        let header_u64 = |name: &str| {
            headers
                .get(name)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
        };

        Error::RateLimit {
            body,
            ratelimit_limit: header_u64("ratelimit-limit"),
            ratelimit_remaining: header_u64("ratelimit-remaining"),
            ratelimit_reset: header_u64("ratelimit-reset"),
        }
    }

    /// Builds the error for a non-2xx response that is not a 429.
    fn response_error(status: StatusCode, content_type_is_html: bool, body: String) -> Error {
        if content_type_is_html {
            return Error::Parse {
                status: Some(status.as_u16()),
                message: body,
                source: None,
            };
        }

        match serde_json::from_str::<ErrorResponse>(&body) {
            Ok(error) => Error::Resend {
                status: status.as_u16(),
                error,
            },
            Err(source) => Error::Parse {
                status: Some(status.as_u16()),
                message: body,
                source: Some(Box::new(source)),
            },
        }
    }

    #[allow(unreachable_pub)]
    #[maybe_async::maybe_async]
    pub async fn send(&self, request: RequestBuilder) -> Result<Response> {
        #[cfg(not(feature = "blocking"))]
        if let Some(limiter) = &self.limiter {
            let jitter =
                governor::Jitter::new(Duration::from_millis(10), Duration::from_millis(50));
            limiter.until_ready_with_jitter(jitter).await;
        }

        let request = request.build()?;

        let response = self.client.execute(request).await?;

        match response.status() {
            StatusCode::TOO_MANY_REQUESTS => {
                let headers = response.headers().clone();
                let body = response.text().await.unwrap_or_default();

                Err(Self::too_many_requests_error(&headers, body))
            }
            x if x.is_client_error() || x.is_server_error() => {
                let content_type_is_html = response
                    .headers()
                    .get("content-type")
                    .and_then(|el| el.to_str().ok())
                    .is_some_and(|content_type| content_type.contains("html"));

                let body = response.text().await.map_err(|source| Error::Parse {
                    status: Some(x.as_u16()),
                    message: String::new(),
                    source: Some(Box::new(source)),
                })?;

                Err(Self::response_error(x, content_type_is_html, body))
            }
            _ => Ok(response),
        }
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Don't output API key.
        f.debug_struct("Client")
            .field("api_key", &"re_*********")
            .field("user_agent", &self.user_agent.as_str())
            .field("base_url", &self.base_url.as_str())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test {
    use super::*;

    #[cfg(not(feature = "blocking"))]
    #[test]
    fn rate_limit_quota_replenishes_configured_rate() {
        use governor::clock::FakeRelativeClock;

        for rate in [1, 9, 20] {
            let clock = FakeRelativeClock::default();
            let limiter =
                RateLimiter::direct_with_clock(Config::rate_limit_quota(rate), clock.clone());

            for _ in 0..rate {
                assert!(limiter.check().is_ok());
            }
            assert!(limiter.check().is_err());

            clock.advance(Duration::from_millis(1100));
            let replenished = (0..rate).filter(|_| limiter.check().is_ok()).count();
            assert_eq!(replenished, rate as usize, "rate {rate}");
            assert!(limiter.check().is_err());
        }
    }

    #[cfg(not(feature = "blocking"))]
    #[test]
    fn builder_can_disable_rate_limit() {
        let config = Config::builder("re_test").rate_limit(None).build();
        assert!(config.limiter.is_none());
    }

    #[cfg(not(feature = "blocking"))]
    #[test]
    fn builder_can_set_rate_limit() {
        let config = Config::builder("re_test")
            .rate_limit(NonZeroU32::new(1))
            .build();
        let limiter = config.limiter.expect("limiter should be set");
        assert!(limiter.check().is_ok());
        assert!(limiter.check().is_err());
    }

    #[cfg(not(feature = "blocking"))]
    #[test]
    fn builder_enables_rate_limit_by_default() {
        let config = Config::builder("re_test").build();
        assert!(config.limiter.is_some());
    }

    fn url_for(base_url: &str, path: &str) -> String {
        let config = Config::builder("re_test")
            .base_url(base_url.parse().unwrap())
            .build();

        config
            .build(Method::GET, path)
            .build()
            .unwrap()
            .url()
            .to_string()
    }

    #[test]
    fn base_url_without_path() {
        assert_eq!(
            url_for("https://api.resend.com", "/emails"),
            "https://api.resend.com/emails"
        );
    }

    #[test]
    fn base_url_path_prefix_is_kept() {
        assert_eq!(
            url_for("https://proxy.example.com/resend", "/emails/abc"),
            "https://proxy.example.com/resend/emails/abc"
        );
        assert_eq!(
            url_for("https://proxy.example.com/resend/", "/emails/abc"),
            "https://proxy.example.com/resend/emails/abc"
        );
    }

    #[test]
    fn too_many_requests_quota_errors_are_resend_errors() {
        let headers = reqwest::header::HeaderMap::new();
        for (name, kind) in [
            ("daily_quota_exceeded", ErrorKind::DailyQuotaExceeded),
            ("monthly_quota_exceeded", ErrorKind::MonthlyQuotaExceeded),
        ] {
            let body = format!(r#"{{"statusCode":429,"name":"{name}","message":"quota"}}"#);
            let error = Config::too_many_requests_error(&headers, body);
            assert_eq!(error.status(), Some(429));
            match error {
                Error::Resend { error, .. } => assert_eq!(error.kind(), kind),
                other => panic!("expected Error::Resend, got {other:?}"),
            }
        }
    }

    #[test]
    fn too_many_requests_rate_limit_keeps_headers_and_body() {
        let mut headers = reqwest::header::HeaderMap::new();
        drop(headers.insert("ratelimit-limit", "2".parse().unwrap()));
        drop(headers.insert("ratelimit-remaining", "0".parse().unwrap()));
        drop(headers.insert("ratelimit-reset", "1".parse().unwrap()));
        let body = r#"{"statusCode":429,"name":"rate_limit_exceeded","message":"slow down"}"#;

        let error = Config::too_many_requests_error(&headers, body.to_owned());
        assert_eq!(error.status(), Some(429));
        assert_eq!(error.raw_body(), Some(body));
        assert!(matches!(
            error,
            Error::RateLimit {
                ratelimit_limit: Some(2),
                ratelimit_remaining: Some(0),
                ratelimit_reset: Some(1),
                ..
            }
        ));

        let error = Config::too_many_requests_error(&headers, "not json".to_owned());
        assert_eq!(error.status(), Some(429));
        assert_eq!(error.raw_body(), Some("not json"));
        assert!(matches!(error, Error::RateLimit { .. }));
    }

    #[test]
    fn standard_api_errors_are_resend_errors() {
        let body = r#"{"statusCode":404,"name":"not_found","message":"Email not found"}"#;

        let error = Config::response_error(StatusCode::NOT_FOUND, false, body.to_owned());

        assert!(matches!(error, Error::Resend { .. }));
        assert_eq!(error.status(), Some(404));
        assert_eq!(error.raw_body(), None);
    }

    #[test]
    fn parsed_errors_report_the_transport_status() {
        // The body deliberately disagrees with the transport status.
        let body = r#"{"statusCode":400,"name":"not_found","message":"Email not found"}"#;

        let error = Config::response_error(StatusCode::NOT_FOUND, false, body.to_owned());

        assert_eq!(error.status(), Some(404));
        assert!(matches!(error, Error::Resend { status: 404, .. }));
    }

    #[test]
    fn non_standard_bodies_keep_status_and_body() {
        let json = r#"{"name":"not_found","message":"Email not found"}"#;
        let error = Config::response_error(StatusCode::NOT_FOUND, false, json.to_owned());
        assert_eq!(error.status(), Some(404));
        assert_eq!(error.raw_body(), Some(json));
        assert!(matches!(
            error,
            Error::Parse {
                status: Some(404),
                source: Some(_),
                ..
            }
        ));

        let html = "<html><body>Bad gateway</body></html>";
        let error = Config::response_error(StatusCode::BAD_GATEWAY, true, html.to_owned());
        assert_eq!(error.status(), Some(502));
        assert_eq!(error.raw_body(), Some(html));
        assert!(matches!(
            error,
            Error::Parse {
                status: Some(502),
                source: None,
                ..
            }
        ));
    }
}
