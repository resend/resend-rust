use std::fmt;
use std::sync::Arc;

use reqwest::Method;
use types::DeleteDomainResponse;

use crate::types::{CreateDomainOptions, Domain, DomainChanges};
use crate::{Config, Result};

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
    pub async fn add(&self, domain: CreateDomainOptions) -> Result<Domain> {
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
    pub async fn verify(&self, domain_id: &str) -> Result<()> {
        let path = format!("/domains/{domain_id}/verify");

        let request = self.0.build(Method::POST, &path);
        let response = self.0.send(request).await?;
        let _content = response.json::<types::VerifyDomainResponse>().await?;

        Ok(())
    }

    /// Updates an existing domain.
    ///
    /// <https://resend.com/docs/api-reference/domains/update-domain>
    #[maybe_async::maybe_async]
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
    /// <https://resend.com/docs/api-reference/domains/list-domains>
    #[maybe_async::maybe_async]
    pub async fn list(&self) -> Result<Vec<Domain>> {
        let request = self.0.build(Method::GET, "/domains");
        let response = self.0.send(request).await?;
        let content = response.json::<types::ListDomainResponse>().await?;

        Ok(content.data)
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

pub mod types {
    use std::{fmt, ops::Deref};

    use ecow::EcoString;
    use serde::{Deserialize, Serialize};

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

    /// Unique [`Domain`] identifier.
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct DomainId(EcoString);

    impl DomainId {
        /// Creates a new [`DomainId`].
        #[inline]
        #[must_use]
        pub fn new(id: &str) -> Self {
            Self(EcoString::from(id))
        }
    }

    impl Deref for DomainId {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            self.as_ref()
        }
    }

    impl AsRef<str> for DomainId {
        #[inline]
        fn as_ref(&self) -> &str {
            self.0.as_str()
        }
    }

    impl fmt::Display for DomainId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(&self.0, f)
        }
    }

    /// Details of a new [`Domain`].
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateDomainOptions {
        /// The name of the domain you want to create.
        #[serde(rename = "name")]
        name: String,
        /// The region where the email will be sent from.
        ///
        /// Possible values are 'us-east-1' | 'eu-west-1' | 'sa-east-1'.
        #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
        region: Option<Region>,
    }

    impl CreateDomainOptions {
        /// Creates a new [`CreateDomainOptions`].
        ///
        /// - `name`: The name of the domain you want to create.
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_owned(),
                region: None,
            }
        }

        /// The region where the email will be sent from.
        pub fn with_region(mut self, region: impl Into<Region>) -> Self {
            self.region = Some(region.into());
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

    #[derive(Debug, Clone, Deserialize)]
    pub struct DomainSpfRecord {
        /// The name of the record.
        pub name: String,
        /// The value of the record.
        pub value: String,
        /// The type of record.
        #[serde(rename = "type")]
        pub d_type: SpfRecordType,
        /// The time to live for the record.
        pub ttl: String,
        /// The status of the record.
        pub status: DomainStatus,

        pub routing_policy: Option<String>,
        pub priority: Option<i32>,
        pub proxy_status: Option<ProxyStatus>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct DomainDkimRecord {
        /// The name of the record.
        pub name: String,
        /// The value of the record.
        pub value: String,
        /// The type of record.
        #[serde(rename = "type")]
        pub d_type: DkimRecordType,
        /// The time to live for the record.
        pub ttl: String,
        /// The status of the record.
        pub status: DomainStatus,

        pub routing_policy: Option<String>,
        pub priority: Option<i32>,
        pub proxy_status: Option<ProxyStatus>,
    }

    #[derive(Debug, Copy, Clone, Deserialize)]
    pub enum ProxyStatus {
        Enable,
        Disable,
    }

    #[derive(Debug, Copy, Clone, Deserialize)]
    pub enum DomainStatus {
        Pending,
        Verified,
        Failed,
        #[serde(rename = "temporary_failure")]
        TemporaryFailure,
        #[serde(rename = "not_started")]
        NotStarted,
    }

    #[derive(Debug, Copy, Clone, Deserialize)]
    pub enum SpfRecordType {
        MX,
        #[allow(clippy::upper_case_acronyms)]
        TXT,
    }

    #[derive(Debug, Copy, Clone, Deserialize)]
    pub enum DkimRecordType {
        #[allow(clippy::upper_case_acronyms)]
        CNAME,
        #[allow(clippy::upper_case_acronyms)]
        TXT,
    }

    /// Individual [`Domain`] record.
    #[derive(Debug, Clone, Deserialize)]
    #[serde(tag = "record")]
    pub enum DomainRecord {
        #[serde(rename = "SPF")]
        DomainSpfRecord(DomainSpfRecord),
        #[serde(rename = "DKIM")]
        DomainDkimRecord(DomainDkimRecord),
    }

    /// Details of an existing domain.
    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct Domain {
        /// The ID of the domain.
        pub id: DomainId,
        /// The name of the domain.
        pub name: String,
        // TODO: Technically both this and the domainrecord could be an enum https://resend.com/docs/api-reference/domains/get-domain#path-parameters
        /// The status of the domain.
        pub status: String,

        /// The date and time the domain was created in ISO8601 format.
        pub created_at: String,
        /// The region where the domain is hosted.
        pub region: Region,
        /// The records of the domain.
        pub records: Option<Vec<DomainRecord>>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct VerifyDomainResponse {
        /// The ID of the domain.
        #[allow(dead_code)]
        pub id: DomainId,
    }

    /// List of changes to apply to a [`Domain`].
    #[must_use]
    #[derive(Debug, Default, Copy, Clone, Serialize)]
    pub struct DomainChanges {
        /// Enable or disable click tracking for the domain.
        #[serde(skip_serializing_if = "Option::is_none")]
        click_tracking: Option<bool>,
        /// Enable or disable open tracking for the domain.
        #[serde(skip_serializing_if = "Option::is_none")]
        open_tracking: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tls: Option<Tls>,
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
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct UpdateDomainResponse {
        /// The ID of the updated domain.
        pub id: DomainId,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct ListDomainResponse {
        pub data: Vec<Domain>,
    }

    #[derive(Debug, Clone, Deserialize)]
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
    use crate::{
        domains::types::{CreateDomainOptions, DomainChanges, Tls},
        tests::CLIENT,
        Result,
    };

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> Result<()> {
        let resend = &*CLIENT;

        // Create
        let domain = resend
            .domains
            .add(CreateDomainOptions::new("example.com"))
            .await?;

        std::thread::sleep(std::time::Duration::from_secs(4));

        // List.
        let list = resend.domains.list().await?;
        assert!(list.len() == 1);

        // Get
        let domain = resend.domains.get(&domain.id).await?;

        // Update
        let updates = DomainChanges::new()
            .with_open_tracking(false)
            .with_click_tracking(true)
            .with_tls(Tls::Enforced);

        let domain = resend.domains.update(&domain.id, updates).await?;
        std::thread::sleep(std::time::Duration::from_secs(4));

        // Delete
        let resp = resend.domains.delete(&domain.id).await?;
        assert!(resp.deleted);

        // List.
        let list = resend.domains.list().await?;
        assert!(list.is_empty());

        Ok(())
    }
}
