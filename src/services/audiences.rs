use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use crate::{Config, Result};
use crate::types::{CreateAudienceRequest, CreateAudienceResponse};
use crate::types::{GetAudienceResponse, ListAudiencesResponse, RemoveAudienceResponse};

/// `Resend` APIs for `METHOD /audiences` endpoints.
#[derive(Clone)]
pub struct Audiences(pub(crate) Arc<Config>);

impl Audiences {
    /// Create a list of contacts.
    ///
    /// <https://resend.com/docs/api-reference/audiences/create-audience>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn create(&self, audience: CreateAudienceRequest) -> Result<CreateAudienceResponse> {
        let request = self.0.build(Method::POST, "/audiences");
        let response = request.json(&audience).send().await?;
        let content = response.json::<CreateAudienceResponse>().await?;

        Ok(content)
    }

    /// Retrieve a single audience.
    ///
    /// <https://resend.com/docs/api-reference/audiences/get-audience>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn retrieve(&self, id: &str) -> Result<GetAudienceResponse> {
        let path = format!("/audiences/{id}");
        let request = self.0.build(Method::GET, &path);
        let response = request.send().await?;
        let content = response.json::<GetAudienceResponse>().await?;

        Ok(content)
    }

    /// Remove an existing audience.
    ///
    /// <https://resend.com/docs/api-reference/audiences/delete-audience>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn delete(&self, id: &str) -> Result<RemoveAudienceResponse> {
        let path = format!("/audiences/{id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = request.send().await?;
        let content = response.json::<RemoveAudienceResponse>().await?;

        Ok(content)
    }

    /// Retrieve a list of audiences.
    ///
    /// <https://resend.com/docs/api-reference/audiences/list-audiences>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn list(&self) -> Result<ListAudiencesResponse> {
        let request = self.0.build(Method::GET, "/audiences");
        let response = request.send().await?;
        let content = response.json::<ListAudiencesResponse>().await?;

        Ok(content)
    }

    /// Create a list of contacts.
    ///
    /// <https://resend.com/docs/api-reference/audiences/create-audience>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn create(&self, audience: CreateAudienceRequest) -> Result<CreateAudienceResponse> {
        let request = self.0.build(Method::POST, "/audiences");
        let response = request.json(&audience).send()?;
        let content = response.json::<CreateAudienceResponse>()?;

        Ok(content)
    }

    /// Retrieve a single audience.
    ///
    /// <https://resend.com/docs/api-reference/audiences/get-audience>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn retrieve(&self, id: &str) -> Result<GetAudienceResponse> {
        let path = format!("/audiences/{id}");

        let request = self.0.build(Method::GET, &path);
        let response = request.send()?;
        let content = response.json::<GetAudienceResponse>()?;

        Ok(content)
    }

    /// Remove an existing audience.
    ///
    /// <https://resend.com/docs/api-reference/audiences/delete-audience>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn delete(&self, id: &str) -> Result<RemoveAudienceResponse> {
        let path = format!("/audiences/{id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = request.send()?;
        let content = response.json::<RemoveAudienceResponse>()?;

        Ok(content)
    }

    /// Retrieve a list of audiences.
    ///
    /// <https://resend.com/docs/api-reference/audiences/list-audiences>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn list(&self) -> Result<ListAudiencesResponse> {
        let request = self.0.build(Method::GET, "/audiences");
        let response = request.send()?;
        let content = response.json::<ListAudiencesResponse>()?;

        Ok(content)
    }
}

impl fmt::Debug for Audiences {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}
