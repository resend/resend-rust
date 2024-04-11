use std::fmt;

use crate::types::{CreateAudienceRequest, CreateAudienceResponse};
use crate::types::{GetAudienceResponse, ListAudiencesResponse, RemoveAudienceResponse};
use crate::{Config, Result};

/// `Resend` APIs for `METHOD /audiences` endpoints.
#[derive(Clone)]
pub struct Audiences(pub(crate) Config);

impl Audiences {
    /// Create a list of contacts.
    ///
    /// <https://resend.com/docs/api-reference/audiences/create-audience>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn create(&self, audience: CreateAudienceRequest) -> Result<CreateAudienceResponse> {
        let uri = self.0.base_url.join("/audiences")?;
        let key = self.0.api_key.as_str();

        let request = self.0.client.post(uri).bearer_auth(key).json(&audience);
        let response = request.send().await?;
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
        let uri = self.0.base_url.join(path.as_str())?;
        let key = self.0.api_key.as_str();

        let request = self.0.client.get(uri).bearer_auth(key);
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
        let uri = self.0.base_url.join(path.as_str())?;
        let key = self.0.api_key.as_str();

        let request = self.0.client.delete(uri).bearer_auth(key);
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
        let uri = self.0.base_url.join("/audiences")?;
        let key = self.0.api_key.as_str();

        let request = self.0.client.get(uri).bearer_auth(key);
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
        let uri = self.0.base_url.join("/audiences")?;
        let key = self.0.api_key.as_str();

        let request = self.0.client.post(uri).bearer_auth(key).json(&audience);
        let response = request.send()?;
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
        let uri = self.0.base_url.join(path.as_str())?;
        let key = self.0.api_key.as_str();

        let request = self.0.client.get(uri).bearer_auth(key);
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
        let uri = self.0.base_url.join(path.as_str())?;
        let key = self.0.api_key.as_str();

        let request = self.0.client.delete(uri).bearer_auth(key);
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
        let uri = self.0.base_url.join("/audiences")?;
        let key = self.0.api_key.as_str();

        let request = self.0.client.get(uri).bearer_auth(key);
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

pub mod types {
    use serde::{Deserialize, Serialize};

    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateAudienceRequest {
        /// The name of the audience you want to create.
        pub name: String,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct CreateAudienceResponse {
        /// The ID of the audience.
        pub id: Option<String>,
        /// The object of the audience.
        pub object: Option<String>,
        /// The name of the audience.
        pub name: Option<String>,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct GetAudienceResponse {
        /// The ID of the audience.
        pub id: Option<String>,
        /// The object of the audience.
        pub object: Option<String>,
        /// The name of the audience.
        pub name: Option<String>,
        /// The date that the object was created.
        pub created_at: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct RemoveAudienceResponse {
        /// The ID of the audience.
        pub id: Option<String>,
        /// The object of the audience.
        pub object: Option<String>,
        /// The deleted attribute indicates that the corresponding audience has been deleted.
        pub deleted: Option<bool>,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct ListAudiencesResponse {
        /// Type of the response object.
        pub object: Option<String>,
        /// Array containing audience information.
        pub data: Option<Vec<ListAudiencesDataResponse>>,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct ListAudiencesDataResponse {
        /// Unique identifier for the audience.
        pub id: Option<String>,
        /// Name of the audience.
        pub name: Option<String>,
        /// Timestamp indicating when the audience was created.
        pub created_at: Option<String>,
    }
}
