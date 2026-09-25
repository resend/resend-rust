use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use crate::{Config, Result, types::Usage};

/// `Resend` APIs for `/usage` endpoints.
#[derive(Clone)]
pub struct UsageSvc(pub(crate) Arc<Config>);

impl UsageSvc {
    /// Retrieves the account-level usage and quota data for the authenticated user.
    ///
    /// <https://resend.com/docs/api-reference/usage/retrieve-usage>
    #[maybe_async::maybe_async]
    pub async fn get(&self) -> Result<Usage> {
        let request = self.0.build(Method::GET, "/usage");
        let response = self.0.send(request).await?;
        let content = response.json::<Usage>().await?;

        Ok(content)
    }
}

impl fmt::Debug for UsageSvc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

#[allow(unreachable_pub)]
pub mod types {
    use serde::{Deserialize, Serialize};

    /// Account-level usage and quota data.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Usage {
        /// Email sending/receiving usage.
        pub emails: EmailsUsage,
        /// Contacts usage.
        pub contacts: ContactsUsage,
        /// Segments usage.
        pub segments: SegmentsUsage,
        /// Broadcasts usage.
        pub broadcasts: BroadcastsUsage,
        /// AI credits usage.
        pub ai_credits: AiCreditsUsage,
        /// Automation runs usage.
        pub automation_runs: AutomationRunsUsage,
        /// Domains usage.
        pub domains: DomainsUsage,
        /// The current rate limit applied to this account.
        pub rate_limit: UsageRateLimit,
    }

    /// Email usage, broken down by billing period.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EmailsUsage {
        /// Email usage for the current day.
        pub daily: EmailUsagePeriod,
        /// Email usage for the current month.
        pub monthly: EmailUsagePeriod,
    }

    /// Email usage for a given billing period.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EmailUsagePeriod {
        /// The total number of emails used (sent + received) in this period.
        pub used: u32,
        /// The maximum number of emails allowed in this period. `None` means unlimited.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<u32>,
        /// The number of emails sent in this period.
        pub sent: u32,
        /// The number of emails received in this period.
        pub received: u32,
        /// When this period's usage resets.
        pub resets_at: String,
    }

    /// Contacts usage.
    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct ContactsUsage {
        /// The number of contacts currently stored.
        pub used: u32,
        /// The maximum number of contacts allowed.
        pub limit: u32,
    }

    /// Segments usage.
    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct SegmentsUsage {
        /// The number of segments currently created.
        pub used: u32,
        /// The maximum number of segments allowed. `None` means unlimited.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<u32>,
    }

    /// Broadcasts usage.
    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct BroadcastsUsage {
        /// The number of broadcasts sent.
        pub used: u32,
        /// The maximum number of broadcasts allowed. Always `None` (unlimited).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<u32>,
    }

    /// AI credits usage.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AiCreditsUsage {
        /// The number of AI credits used.
        pub used: u32,
        /// The maximum number of AI credits allowed. `None` means unlimited.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<u32>,
        /// When the AI credits allowance will next increase. `None` if not scheduled.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub next_increase_at: Option<String>,
    }

    /// Automation runs usage.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AutomationRunsUsage {
        /// The number of automation runs used.
        pub used: u32,
        /// The maximum number of automation runs allowed.
        pub limit: u32,
        /// When this period's usage resets.
        pub resets_at: String,
    }

    /// Domains usage.
    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct DomainsUsage {
        /// The number of domains currently created.
        pub used: u32,
        /// The maximum number of domains allowed. `None` means unlimited.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<u32>,
    }

    /// The current rate limit applied to this account.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UsageRateLimit {
        /// The maximum number of requests allowed per `duration`.
        pub limit: u32,
        /// The duration of the rate limit window (e.g. `"1000ms"`).
        pub duration: String,
    }
}

#[cfg(test)]
#[allow(clippy::needless_return)]
mod test {
    #[cfg(not(feature = "blocking"))]
    use crate::test::{CLIENT, DebugResult};

    #[tokio_shared_rt::test(shared = true)]
    #[serial_test::serial]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> DebugResult<()> {
        let resend = &*CLIENT;

        let usage = resend.usage.get().await?;
        assert!(usage.emails.monthly.limit.is_some());

        Ok(())
    }

    #[test]
    fn deserialize_usage() {
        use crate::usage::types::Usage;

        let json = r#"{
            "object": "usage",
            "emails": {
                "daily": { "used": 258, "limit": null, "sent": 57, "received": 201, "resets_at": "2026-07-17T00:00:00.000Z" },
                "monthly": { "used": 5442, "limit": 10000, "sent": 1000, "received": 4442, "resets_at": "2026-08-01T00:00:00.000Z" }
            },
            "contacts": { "used": 85000, "limit": 150000 },
            "segments": { "used": 2, "limit": 3 },
            "broadcasts": { "used": 100, "limit": null },
            "ai_credits": { "used": 0, "limit": 500, "next_increase_at": "2026-07-18T09:00:00.000Z" },
            "automation_runs": { "used": 0, "limit": 1000, "resets_at": "2026-08-01T00:00:00.000Z" },
            "domains": { "used": 1, "limit": 1000 },
            "rate_limit": { "limit": 10, "duration": "1000ms" }
        }"#;

        let usage: Usage = serde_json::from_str(json).expect("usage deserializes");

        assert_eq!(usage.emails.daily.used, 258);
        assert_eq!(usage.emails.daily.limit, None);
        assert_eq!(usage.emails.monthly.limit, Some(10_000));
        assert_eq!(usage.contacts.limit, 150_000);
        assert_eq!(usage.broadcasts.limit, None);
        assert_eq!(
            usage.ai_credits.next_increase_at.as_deref(),
            Some("2026-07-18T09:00:00.000Z")
        );
        assert_eq!(usage.rate_limit.duration, "1000ms");
    }

    #[test]
    fn deserialize_usage_with_no_ai_credits_increase() {
        use crate::usage::types::Usage;

        let json = r#"{
            "object": "usage",
            "emails": {
                "daily": { "used": 0, "limit": 100, "sent": 0, "received": 0, "resets_at": "2026-07-17T00:00:00.000Z" },
                "monthly": { "used": 0, "limit": 10000, "sent": 0, "received": 0, "resets_at": "2026-08-01T00:00:00.000Z" }
            },
            "contacts": { "used": 0, "limit": 150000 },
            "segments": { "used": 0, "limit": null },
            "broadcasts": { "used": 0, "limit": null },
            "ai_credits": { "used": 0, "limit": null, "next_increase_at": null },
            "automation_runs": { "used": 0, "limit": 1000, "resets_at": "2026-08-01T00:00:00.000Z" },
            "domains": { "used": 0, "limit": null },
            "rate_limit": { "limit": 10, "duration": "1000ms" }
        }"#;

        let usage: Usage = serde_json::from_str(json).expect("usage deserializes");

        assert_eq!(usage.segments.limit, None);
        assert_eq!(usage.ai_credits.limit, None);
        assert_eq!(usage.ai_credits.next_increase_at, None);
        assert_eq!(usage.domains.limit, None);
    }
}
