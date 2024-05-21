use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use crate::types::{CreateDomainOptions, Domain, DomainChanges, DomainId};
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
    // Reasoning for allow: https://github.com/AntoniosBarotsis/resend-rs/pull/1#issuecomment-2081646115
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
    pub async fn get(&self, domain_id: &DomainId) -> Result<Domain> {
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
    pub async fn verify(&self, domain_id: &DomainId) -> Result<()> {
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
        domain_id: &DomainId,
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
    pub async fn delete(&self, domain_id: &DomainId) -> Result<bool> {
        let path = format!("/domains/{domain_id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<types::DeleteDomainResponse>().await?;

        Ok(content.deleted)
    }
}

impl fmt::Debug for DomainsSvc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

pub mod types {
    use std::fmt;

    use ecow::EcoString;
    use serde::{Deserialize, Serialize};

    // TODO: Again, is this needed? Should it just be a normal String?
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

    impl AsRef<str> for DomainId {
        #[inline]
        fn as_ref(&self) -> &str {
            self.0.as_str()
        }
    }

    impl fmt::Display for DomainId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(self.as_ref(), f)
        }
    }

    /// Details of a new [`Domain`].
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateDomainOptions {
        /// The name of the domain you want to create.
        #[serde(rename = "name")]
        pub name: String,
        /// The region where the email will be sent from.
        ///
        /// Possible values are 'us-east-1' | 'eu-west-1' | 'sa-east-1'.
        #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
        pub region: Option<Region>,
    }

    impl CreateDomainOptions {
        /// Creates a new [`CreateDomainOptions`].
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_owned(),
                region: None,
            }
        }

        /// Specifies the region for sending emails from the domain.
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

    /// Individual [`Domain`] record.
    #[derive(Debug, Clone, Deserialize)]
    pub struct DomainRecord {
        /// The type of record.
        pub record: String,
        /// The name of the record.
        pub name: String,

        /// The type of record.
        #[serde(rename = "type")]
        pub d_type: Option<String>,
        /// The time to live for the record.
        pub ttl: Option<String>,
        /// The status of the record.
        pub status: Option<String>,
        /// The value of the record.
        pub value: Option<String>,
        /// The priority of the record.
        pub priority: Option<i32>,
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
        pub records: Vec<DomainRecord>,

        /// The service that runs DNS server.
        #[serde(rename = "dnsProvider")]
        pub dns_provider: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct VerifyDomainResponse {
        /// The ID of the domain.
        pub id: DomainId,
    }

    /// List of changes to apply to a [`Domain`].
    #[must_use]
    #[derive(Debug, Default, Copy, Clone, Serialize)]
    pub struct DomainChanges {
        /// Enable or disable click tracking for the domain.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub click_tracking: Option<bool>,
        /// Enable or disable open tracking for the domain.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub open_tracking: Option<bool>,
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
mod test {
    use crate::{Client, Result};

    #[tokio::test]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> Result<()> {
        let resend = Client::default();

        // TODO: Domain test.

        // List.
        let _ = resend.domains.list().await?;

        Ok(())
    }
}
