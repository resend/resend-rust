use std::fmt;
use std::sync::Arc;

use reqwest::Method;
use types::DeleteDomainResponse;

use crate::{Config, Result, domains::types::VerifyDomainResponse};
use crate::{
    list_opts::{ListOptions, ListResponse},
    types::{CreateDomainOptions, Domain, DomainChanges},
};

use self::types::UpdateDomainResponse;

/// `Resend` APIs for `/domains` endpoints.
#[derive(Clone)]
pub struct DomainsSvc(pub(crate) Arc<Config>);

impl DomainsSvc {
    /// Creates a domain through the Resend Email API.
    ///
    /// <https://resend.com/docs/api-reference/domains/create-domain>
    #[maybe_async::maybe_async]
    // Reasoning for allow: https://github.com/resend/resend-rust/pull/1#issuecomment-2081646115
    #[allow(clippy::needless_pass_by_value)]
    pub async fn create(&self, domain: CreateDomainOptions) -> Result<Domain> {
        let request = self.0.build(Method::POST, "/domains");
        let response = self.0.send(request.json(&domain)).await?;
        let content = response.json::<Domain>().await?;

        Ok(content)
    }

    /// Retrieves a single domain for the authenticated user.
    ///
    /// <https://resend.com/docs/api-reference/domains/get-domain>
    #[maybe_async::maybe_async]
    pub async fn get(&self, domain_id: &str) -> Result<Domain> {
        let path = format!("/domains/{domain_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Domain>().await?;

        Ok(content)
    }

    /// Verifies an existing domain.
    ///
    /// <https://resend.com/docs/api-reference/domains/verify-domain>
    #[maybe_async::maybe_async]
    pub async fn verify(&self, domain_id: &str) -> Result<VerifyDomainResponse> {
        let path = format!("/domains/{domain_id}/verify");

        let request = self.0.build(Method::POST, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<VerifyDomainResponse>().await?;

        Ok(content)
    }

    /// Updates an existing domain.
    ///
    /// <https://resend.com/docs/api-reference/domains/update-domain>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn update(
        &self,
        domain_id: &str,
        update: DomainChanges,
    ) -> Result<UpdateDomainResponse> {
        let path = format!("/domains/{domain_id}");

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&update)).await?;
        let content = response.json::<UpdateDomainResponse>().await?;

        Ok(content)
    }

    /// Retrieves a list of domains for the authenticated user.
    ///
    /// - Default limit: no limit (return everything)
    ///
    /// <https://resend.com/docs/api-reference/domains/list-domains>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn list<T>(&self, list_opts: ListOptions<T>) -> Result<ListResponse<Domain>> {
        let request = self.0.build(Method::GET, "/domains").query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<Domain>>().await?;

        Ok(content)
    }

    /// Removes an existing domain.
    ///
    /// Returns whether the domain was deleted successfully.
    ///
    /// <https://resend.com/docs/api-reference/domains/delete-domain>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn delete(&self, domain_id: &str) -> Result<DeleteDomainResponse> {
        let path = format!("/domains/{domain_id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<DeleteDomainResponse>().await?;

        Ok(content)
    }
}

impl fmt::Debug for DomainsSvc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

#[allow(unreachable_pub)]
pub mod types {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    #[derive(Debug, Copy, Clone, Serialize)]
    #[serde(rename_all = "lowercase")]
    pub enum Tls {
        /// Enforced TLS on the other hand, requires that the email communication must use TLS no
        /// matter what. If the receiving server does not support TLS, the email will not be sent.
        Enforced,
        /// Opportunistic TLS means that it always attempts to make a secure connection to the
        /// receiving mail server. If it can’t establish a secure connection, it sends the message
        /// unencrypted.
        Opportunistic,
    }

    crate::define_id_type!(DomainId);

    /// Details of a new [`Domain`].
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateDomainOptions {
        /// The name of the domain you want to create.
        #[serde(rename = "name")]
        name: String,
        /// The region where the email will be sent from.
        ///
        /// Possible values are `'us-east-1' | 'eu-west-1' | 'sa-east-1' | 'ap-northeast-1'`.
        #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
        region: Option<Region>,
        /// For advanced use cases, choose a subdomain for the Return-Path address.
        /// The custom return path is used for SPF authentication, DMARC alignment, and handling
        /// bounced emails. Defaults to `send` (i.e., `send.yourdomain.tld`). Avoid setting values
        /// that could undermine credibility (e.g. `testing`), as they may be exposed to recipients.
        #[serde(skip_serializing_if = "Option::is_none")]
        custom_return_path: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        open_tracking: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        click_tracking: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tracking_subdomain: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tls: Option<Tls>,
        #[serde(skip_serializing_if = "Option::is_none")]
        capabilities: Option<Value>,
    }

    impl CreateDomainOptions {
        /// Creates a new [`CreateDomainOptions`].
        ///
        /// - `name`: The name of the domain you want to create.
        #[inline]
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_owned(),
                region: None,
                custom_return_path: None,

                open_tracking: None,
                click_tracking: None,
                tracking_subdomain: None,
                tls: None,
                capabilities: None,
            }
        }

        /// The region where the email will be sent from.
        #[inline]
        pub fn with_region(mut self, region: impl Into<Region>) -> Self {
            self.region = Some(region.into());
            self
        }

        /// For advanced use cases, choose a subdomain for the Return-Path address.
        /// The custom return path is used for SPF authentication, DMARC alignment, and handling
        /// bounced emails. Defaults to `send` (i.e., `send.yourdomain.tld`). Avoid setting values
        /// that could undermine credibility (e.g. `testing`), as they may be exposed to recipients.
        #[inline]
        pub fn with_custom_return_path(mut self, custom_return_path: impl Into<String>) -> Self {
            self.custom_return_path = Some(custom_return_path.into());
            self
        }

        #[inline]
        pub fn with_open_tracking(mut self, open_tracking: bool) -> Self {
            self.open_tracking = Some(open_tracking);
            self
        }

        #[inline]
        pub fn with_click_tracking(mut self, click_tracking: bool) -> Self {
            self.click_tracking = Some(click_tracking);
            self
        }

        #[inline]
        pub fn with_tracking_subdomain(mut self, tracking_subdomain: impl Into<String>) -> Self {
            self.tracking_subdomain = Some(tracking_subdomain.into());
            self
        }

        #[inline]
        pub fn with_tls(mut self, tls: Tls) -> Self {
            self.tls = Some(tls);
            self
        }

        #[inline]
        pub fn with_capabilities(mut self, capabilities: Value) -> Self {
            self.capabilities = Some(capabilities);
            self
        }
    }

    /// Region where [`CreateEmailBaseOptions`]s will be sent from.
    ///
    /// Possible values are 'us-east-1' | 'eu-west-1' | 'sa-east-1' | 'ap-northeast-1'.
    ///
    /// [`CreateEmailBaseOptions`]: crate::types::CreateEmailBaseOptions
    #[non_exhaustive]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Region {
        /// 'us-east-1'
        #[serde(rename = "us-east-1")]
        UsEast1,
        /// 'eu-west-1'
        #[serde(rename = "eu-west-1")]
        EuWest1,
        /// 'sa-east-1'
        #[serde(rename = "sa-east-1")]
        SaEast1,
        /// 'ap-northeast-1'
        #[serde(rename = "ap-northeast-1")]
        ApNorthEast1,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DomainSpfRecord {
        /// The name of the record.
        pub name: String,
        /// The value of the record.
        pub value: String,
        /// The type of record.
        #[serde(rename = "type")]
        pub r#type: SpfRecordType,
        /// The time to live for the record.
        pub ttl: String,
        /// The status of the record.
        pub status: DomainStatus,

        pub routing_policy: Option<String>,
        pub priority: Option<i32>,
        pub proxy_status: Option<ProxyStatus>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DomainDkimRecord {
        /// The name of the record.
        pub name: String,
        /// The value of the record.
        pub value: String,
        /// The type of record.
        #[serde(rename = "type")]
        pub r#type: DkimRecordType,
        /// The time to live for the record.
        pub ttl: String,
        /// The status of the record.
        pub status: DomainStatus,

        pub routing_policy: Option<String>,
        pub priority: Option<i32>,
        pub proxy_status: Option<ProxyStatus>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ReceivingRecord {
        /// The name of the record.
        pub name: String,
        /// The value of the record.
        pub value: String,
        /// The type of record.
        #[serde(rename = "type")]
        pub r#type: ReceivingRecordType,
        /// The time to live for the record.
        pub ttl: String,
        /// The status of the record.
        pub status: DomainStatus,

        pub priority: i32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TrackingRecord {
        /// The name of the record.
        pub name: String,
        /// The value of the record.
        pub value: String,
        /// The type of record.
        #[serde(rename = "type")]
        pub r#type: TrackingRecordType,
        /// The time to live for the record.
        pub ttl: String,
        /// The status of the record.
        pub status: DomainStatus,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TrackingCaaRecord {
        /// The name of the record.
        pub name: String,
        /// The value of the record (e.g. `0 issue "amazon.com"`).
        pub value: String,
        /// The type of record.
        #[serde(rename = "type")]
        pub r#type: TrackingCaaRecordType,
        /// The time to live for the record.
        pub ttl: String,
        /// The status of the record.
        pub status: DomainStatus,
    }

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub enum ReceivingRecordType {
        #[allow(clippy::upper_case_acronyms)]
        MX,
    }

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub enum TrackingRecordType {
        #[allow(clippy::upper_case_acronyms)]
        CNAME,
    }

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub enum TrackingCaaRecordType {
        #[allow(clippy::upper_case_acronyms)]
        CAA,
    }

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub enum ProxyStatus {
        Enable,
        Disable,
    }

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum DomainStatus {
        Pending,
        Verified,
        Failed,
        TemporaryFailure,
        NotStarted,
    }

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub enum SpfRecordType {
        MX,
        #[allow(clippy::upper_case_acronyms)]
        TXT,
    }

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    pub enum DkimRecordType {
        #[allow(clippy::upper_case_acronyms)]
        CNAME,
        #[allow(clippy::upper_case_acronyms)]
        TXT,
    }

    /// Individual [`Domain`] record.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "record")]
    pub enum DomainRecord {
        #[serde(rename = "SPF")]
        DomainSpfRecord(DomainSpfRecord),
        #[serde(rename = "DKIM")]
        DomainDkimRecord(DomainDkimRecord),
        #[serde(rename = "Receiving MX")]
        ReceivingRecord(ReceivingRecord),
        #[serde(rename = "Tracking")]
        TrackingRecord(TrackingRecord),
        #[serde(rename = "TrackingCAA")]
        TrackingCaaRecord(TrackingCaaRecord),
    }

    /// Details of an existing domain.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Domain {
        /// The ID of the domain.
        pub id: DomainId,
        /// The name of the domain.
        pub name: String,
        /// The status of the domain.
        pub status: DomainStatus,

        /// The date and time the domain was created in ISO8601 format.
        pub created_at: String,
        /// The region where the domain is hosted.
        pub region: Region,
        /// The records of the domain.
        pub records: Option<Vec<DomainRecord>>,

        pub capabilities: DomainCapabilities,
    }

    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct DomainCapabilities {
        pub sending: DomainCapabilityStatus,
        pub receiving: DomainCapabilityStatus,
    }

    #[non_exhaustive]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum DomainCapabilityStatus {
        Enabled,
        Disabled,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VerifyDomainResponse {
        /// The ID of the domain.
        #[allow(dead_code)]
        pub id: DomainId,
    }

    /// List of changes to apply to a [`Domain`].
    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct DomainChanges {
        /// Enable or disable click tracking for the domain.
        #[serde(skip_serializing_if = "Option::is_none")]
        click_tracking: Option<bool>,
        /// Enable or disable open tracking for the domain.
        #[serde(skip_serializing_if = "Option::is_none")]
        open_tracking: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tls: Option<Tls>,
        #[serde(skip_serializing_if = "Option::is_none")]
        capabilities: Option<DomainCapabilities>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tracking_subdomain: Option<String>,
    }

    impl DomainChanges {
        /// Creates a new [`DomainChanges`].
        #[inline]
        pub fn new() -> Self {
            Self::default()
        }

        /// Toggles the click tracking to `enable`.
        #[inline]
        pub const fn with_click_tracking(mut self, enable: bool) -> Self {
            self.click_tracking = Some(enable);
            self
        }

        /// Toggles the open tracing to `enable`.
        #[inline]
        pub const fn with_open_tracking(mut self, enable: bool) -> Self {
            self.open_tracking = Some(enable);
            self
        }

        /// Changes the TLS configuration.
        #[inline]
        pub const fn with_tls(mut self, tls: Tls) -> Self {
            self.tls = Some(tls);
            self
        }

        #[inline]
        pub fn with_tracking_subdomain(mut self, tracking_subdomain: impl Into<String>) -> Self {
            self.tracking_subdomain = Some(tracking_subdomain.into());
            self
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateDomainResponse {
        /// The ID of the updated domain.
        pub id: DomainId,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DeleteDomainResponse {
        /// The ID of the domain.
        pub id: DomainId,
        /// Indicates whether the domain was deleted successfully.
        pub deleted: bool,
    }
}

#[cfg(test)]
#[allow(clippy::needless_return)]
mod test {
    use crate::domains::types::DeleteDomainResponse;
    use crate::list_opts::ListOptions;
    use crate::{
        domains::types::{CreateDomainOptions, DomainChanges, Tls},
        test::{CLIENT, DebugResult, retry},
    };

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    #[ignore = "Flaky backend"]
    async fn all() -> DebugResult<()> {
        let resend = &*CLIENT;

        // Create
        let domain = resend
            .domains
            .create(CreateDomainOptions::new("example.com"))
            .await?;

        std::thread::sleep(std::time::Duration::from_secs(4));

        // List.
        let list = resend.domains.list(ListOptions::default()).await?;
        assert!(list.len() == 1);

        // Get
        let domain = resend.domains.get(&domain.id).await?;

        // Update
        let updates = DomainChanges::new()
            .with_open_tracking(false)
            .with_click_tracking(true)
            .with_tls(Tls::Enforced);

        std::thread::sleep(std::time::Duration::from_secs(4));
        let f = async || resend.domains.update(&domain.id, updates.clone()).await;
        let domain = retry(f, 5, std::time::Duration::from_secs(2)).await?;
        std::thread::sleep(std::time::Duration::from_secs(4));

        // Delete
        let f = async || resend.domains.delete(&domain.id).await;
        let resp: DeleteDomainResponse = retry(f, 5, std::time::Duration::from_secs(2)).await?;

        assert!(resp.deleted);

        // List.
        let list = resend.domains.list(ListOptions::default()).await?;
        assert!(list.is_empty());

        Ok(())
    }

    #[test]
    fn deserialize_domain_with_tracking_caa_record() {
        use crate::domains::types::{Domain, DomainRecord};

        let json = r#"{
            "object": "domain",
            "id": "7c2a439f-d5fc-4dc1-8bab-ced17f14c972",
            "name": "namingishard.dev",
            "created_at": "2026-04-14 11:16:24.808219+00",
            "status": "verified",
            "capabilities": { "sending": "enabled", "receiving": "disabled" },
            "records": [
                {
                    "record": "Tracking",
                    "type": "CNAME",
                    "name": "links",
                    "value": "links1.resend-dns-staging.com",
                    "ttl": "Auto",
                    "status": "verified"
                },
                {
                    "record": "TrackingCAA",
                    "name": "",
                    "type": "CAA",
                    "ttl": "Auto",
                    "value": "0 issue \"amazon.com\"",
                    "status": "verified"
                }
            ],
            "region": "eu-west-1"
        }"#;

        let domain: Domain = serde_json::from_str(json).expect("domain deserializes");
        let records = domain.records.expect("records present");
        assert_eq!(records.len(), 2);
        assert!(matches!(records[0], DomainRecord::TrackingRecord(_)));
        assert!(matches!(records[1], DomainRecord::TrackingCaaRecord(_)));
    }
}
