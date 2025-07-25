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
use std::{env, fmt};
#[cfg(not(feature = "blocking"))]
use std::{num::NonZeroU32, sync::Arc, time::Duration};

use crate::{Error, Result, error::types::ErrorResponse};

#[cfg(doc)]
use crate::Resend;

/// Convenience builder for [`Config`].
///
/// This requires from you to set the API key ([`ConfigBuilder::new`]), but also
/// makes it possible to set a `reqwest` http client with your custom configuration
/// (see also [`Resend::with_client`]) as well as an override for the Resend's
/// base url to send requests to.
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

    /// Builder's terminal method producing [`Config`].
    pub fn build(self) -> Config {
        Config::new(self.api_key, self.client.unwrap_or_default(), self.base_url)
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
    limiter: Arc<
        RateLimiter<
            NotKeyed,
            InMemoryState,
            MonotonicClock,
            NoOpMiddleware<<MonotonicClock as governor::clock::Clock>::Instant>,
        >,
    >,
}

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
        #[cfg(not(feature = "blocking"))]
        let rate_limit_per_sec = env::var("RESEND_RATE_LIMIT")
            .unwrap_or_else(|_| "9".to_owned())
            .parse::<u32>()
            .expect("env variable `RESEND_RATE_LIMIT` should be a valid u32");

        #[cfg(not(feature = "blocking"))]
        let quota = Quota::with_period(Duration::from_millis(1100))
            .expect("Valid quota")
            .allow_burst(
                NonZeroU32::new(rate_limit_per_sec).expect("Rate limit is a valid non zero u32"),
            );

        #[cfg(not(feature = "blocking"))]
        let limiter = Arc::new(RateLimiter::direct_with_clock(quota, MonotonicClock));
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

    /// Constructs a new [`RequestBuilder`].
    pub(crate) fn build(&self, method: Method, path: &str) -> RequestBuilder {
        let path = self
            .base_url
            .join(path)
            .expect("should be a valid API endpoint");

        self.client
            .request(method, path)
            .bearer_auth(self.api_key.as_str())
            .header(USER_AGENT, self.user_agent.as_str())
    }

    #[allow(unreachable_pub)]
    #[maybe_async::maybe_async]
    pub async fn send(&self, request: RequestBuilder) -> Result<Response> {
        #[cfg(not(feature = "blocking"))]
        {
            let jitter =
                governor::Jitter::new(Duration::from_millis(10), Duration::from_millis(50));
            self.limiter.until_ready_with_jitter(jitter).await;
        }

        let request = request.build()?;

        let response = self.client.execute(request).await?;

        match response.status() {
            StatusCode::TOO_MANY_REQUESTS => {
                let headers = response.headers();

                let ratelimit_limit = headers
                    .get("ratelimit-limit")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok());
                let ratelimit_remaining = headers
                    .get("ratelimit-remaining")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok());
                let ratelimit_reset = headers
                    .get("ratelimit-reset")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok());

                Err(Error::RateLimit {
                    ratelimit_limit,
                    ratelimit_remaining,
                    ratelimit_reset,
                })
            }
            x if x.is_client_error() || x.is_server_error() => {
                // TODO: Make this more testable
                let content_type_is_html = response
                    .headers()
                    .get("content-type")
                    .and_then(|el| el.to_str().ok())
                    .is_some_and(|content_type| content_type.contains("html"));

                if content_type_is_html {
                    return Err(Error::Parse(response.text().await?));
                }

                let error = response.json::<ErrorResponse>().await?;
                Err(Error::Resend(error))
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
