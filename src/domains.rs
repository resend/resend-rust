use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use crate::types::{CreateDomainRequest, CreateDomainResponse};
use crate::types::{DeleteDomainResponse, Domain, ListDomainsResponse, VerifyDomainResponse};
use crate::types::{UpdateDomainRequest, UpdateDomainResponse};
use crate::{Config, Result};

/// `Resend` APIs for `METHOD /domains` endpoints.
#[derive(Clone)]
pub struct Domains(pub(crate) Arc<Config>);

impl Domains {
    /// Create a domain through the Resend Email API.
    ///
    /// <https://resend.com/docs/api-reference/domains/create-domain>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn add(&self, domain: CreateDomainRequest) -> Result<CreateDomainResponse> {
        let request = self.0.build(Method::POST, "/domains");
        let response = request.json(&domain).send().await?;
        let content = response.json::<CreateDomainResponse>().await?;

        Ok(content)
    }

    /// Retrieve a single domain for the authenticated user.
    ///
    /// <https://resend.com/docs/api-reference/domains/get-domain>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn retrieve(&self, domain_id: &str) -> Result<Domain> {
        let path = format!("/domains/{domain_id}");

        let request = self.0.build(Method::GET, &path);
        let response = request.send().await?;
        let content = response.json::<Domain>().await?;

        Ok(content)
    }

    /// Verify an existing domain.
    ///
    /// <https://resend.com/docs/api-reference/domains/verify-domain>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn verify(&self, domain_id: &str) -> Result<VerifyDomainResponse> {
        let path = format!("/domains/{domain_id}/verify");

        let request = self.0.build(Method::POST, &path);
        let response = request.send().await?;
        let content = response.json::<VerifyDomainResponse>().await?;

        Ok(content)
    }

    /// Update an existing domain.
    ///
    /// <https://resend.com/docs/api-reference/domains/update-domain>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn update(
        &self,
        domain_id: &str,
        update: UpdateDomainRequest,
    ) -> Result<UpdateDomainResponse> {
        let path = format!("/domains/{domain_id}");

        let request = self.0.build(Method::PATCH, &path);
        let response = request.json(&update).send().await?;
        let content = response.json::<UpdateDomainResponse>().await?;

        Ok(content)
    }

    /// Retrieve a list of domains for the authenticated user.
    ///
    /// <https://resend.com/docs/api-reference/domains/list-domains>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn list(&self) -> Result<ListDomainsResponse> {
        let request = self.0.build(Method::GET, "/domains");
        let response = request.send().await?;
        let content = response.json::<ListDomainsResponse>().await?;

        Ok(content)
    }

    /// Remove an existing domain.
    ///
    /// <https://resend.com/docs/api-reference/domains/delete-domain>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn delete(&self, domain_id: &str) -> Result<DeleteDomainResponse> {
        let path = format!("/domains/{domain_id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = request.send().await?;
        let content = response.json::<DeleteDomainResponse>().await?;

        Ok(content)
    }

    /// Create a domain through the Resend Email API.
    ///
    /// <https://resend.com/docs/api-reference/domains/create-domain>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn add(&self, domain: CreateDomainRequest) -> Result<CreateDomainResponse> {
        let request = self.0.build(Method::POST, "/domains");
        let response = request.json(&domain).send()?;
        let content = response.json::<CreateDomainResponse>()?;

        Ok(content)
    }

    /// Retrieve a single domain for the authenticated user.
    ///
    /// <https://resend.com/docs/api-reference/domains/get-domain>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn retrieve(&self, domain_id: &str) -> Result<Domain> {
        let path = format!("/domains/{domain_id}");

        let request = self.0.build(Method::GET, &path);
        let response = request.send()?;
        let content = response.json::<Domain>()?;

        Ok(content)
    }

    /// Verify an existing domain.
    ///
    /// <https://resend.com/docs/api-reference/domains/verify-domain>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn verify(&self, domain_id: &str) -> Result<VerifyDomainResponse> {
        let path = format!("/domains/{domain_id}/verify");

        let request = self.0.build(Method::POST, &path);
        let response = request.send()?;
        let content = response.json::<VerifyDomainResponse>()?;

        Ok(content)
    }

    /// Update an existing domain.
    ///
    /// <https://resend.com/docs/api-reference/domains/update-domain>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn update(
        &self,
        domain_id: &str,
        update: UpdateDomainRequest,
    ) -> Result<UpdateDomainResponse> {
        let path = format!("/domains/{domain_id}");

        let request = self.0.build(Method::PATCH, &path);
        let response = request.json(&update).send()?;
        let content = response.json::<UpdateDomainResponse>()?;

        Ok(content)
    }

    /// Retrieve a list of domains for the authenticated user.
    ///
    /// <https://resend.com/docs/api-reference/domains/list-domains>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn list(&self) -> Result<ListDomainsResponse> {
        let request = self.0.build(Method::GET, "/domains");
        let response = request.send()?;
        let content = response.json::<ListDomainsResponse>()?;

        Ok(content)
    }

    /// Remove an existing domain.
    ///
    /// <https://resend.com/docs/api-reference/domains/delete-domain>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn delete(&self, domain_id: &str) -> Result<DeleteDomainResponse> {
        let path = format!("/domains/{domain_id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = request.send()?;
        let content = response.json::<DeleteDomainResponse>()?;

        Ok(content)
    }
}

impl fmt::Debug for Domains {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

pub mod types {
    use serde::{Deserialize, Serialize};

    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateDomainRequest {
        /// The name of the domain you want to create.
        #[serde(rename = "name")]
        pub name: String,
        /// The region where emails will be sent from. Possible values are us-east-1' | 'eu-west-1' | 'sa-east-1
        #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
        pub region: Option<Region>,
    }

    /// The region where emails will be sent from.
    ///
    /// Possible values are 'us-east-1' | 'eu-west-1' | 'sa-east-1' | 'ap-northeast-1'.
    // TODO: Custom region.
    #[derive(Debug, Copy, Clone, Serialize)]
    pub enum Region {
        #[serde(rename = "us-east-1")]
        UsEast1,
        #[serde(rename = "eu-west-1")]
        EuWest1,
        #[serde(rename = "sa-east-1")]
        SaEast1,
        #[serde(rename = "ap-northeast-1")]
        ApNorthEast1,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct CreateDomainResponse {
        /// The ID of the domain.
        pub id: Option<String>,
        /// The name of the domain.
        pub name: Option<String>,
        /// The date and time the domain was created.
        pub created_at: Option<String>,
        /// The status of the domain.
        pub status: Option<String>,
        /// The records of the domain.
        pub records: Option<Vec<DomainRecord>>,
        /// The region where the domain is hosted.
        pub region: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct DomainRecord {
        /// The type of record.
        pub record: Option<String>,
        /// The name of the record.
        pub name: Option<String>,
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

    #[derive(Debug, Clone, Deserialize)]
    pub struct Domain {
        /// The type of object.
        pub object: Option<String>,
        /// The ID of the domain.
        pub id: Option<String>,
        /// The name of the domain.
        pub name: Option<String>,
        /// The status of the domain.
        pub status: Option<String>,
        /// The date and time the domain was created.
        pub created_at: Option<String>,
        /// The region where the domain is hosted.
        pub region: Option<String>,
        /// The records of the domain.
        pub records: Option<Vec<DomainRecord>>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct VerifyDomainResponse {
        /// The type of object.
        pub object: Option<String>,
        /// The ID of the domain.
        pub id: Option<String>,
    }

    #[must_use]
    #[derive(Debug, Default, Copy, Clone, Serialize)]
    pub struct UpdateDomainRequest {
        /// Enable or disable click tracking for the domain.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub click_tracking: Option<bool>,
        /// Enable or disable open tracking for the domain.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub open_tracking: Option<bool>,
    }

    impl UpdateDomainRequest {
        /// Creates a new [`UpdateDomainRequest`].
        pub fn new() -> Self {
            Self::default()
        }

        /// Toggles the click tracking to `enable`.
        #[inline]
        pub fn click_tracking(mut self, enable: bool) -> Self {
            self.click_tracking = Some(enable);
            self
        }

        /// Toggles the open tracing to `enable`.
        #[inline]
        pub fn open_tracking(mut self, enable: bool) -> Self {
            self.open_tracking = Some(enable);
            self
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct UpdateDomainResponse {
        /// The ID of the updated domain.
        pub id: Option<String>,
        /// The object type representing the updated domain.
        pub object: Option<String>,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct ListDomainsResponse {
        #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
        pub data: Option<Vec<ListDomainsItem>>,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct ListDomainsItem {
        /// The ID of the domain.
        pub id: Option<String>,
        /// The name of the domain.
        pub name: Option<String>,
        /// The status of the domain.
        pub status: Option<String>,
        /// The date and time the domain was created.
        pub created_at: Option<String>,
        /// The region where the domain is hosted.
        pub region: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct DeleteDomainResponse {
        /// The type of object.
        pub object: Option<String>,
        /// The ID of the domain.
        pub id: Option<String>,
        /// Indicates whether the domain was deleted successfully.
        pub deleted: Option<bool>,
    }
}
