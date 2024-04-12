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
