use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use crate::types::{CreateAudienceRequest, CreateAudienceResponse};
use crate::types::{GetAudienceResponse, ListAudiencesResponse, RemoveAudienceResponse};
use crate::{Config, Result};

/// `Resend` APIs for `METHOD /audiences` endpoints.
#[derive(Clone)]
pub struct Audiences(pub(crate) Arc<Config>);

impl Audiences {
    /// Create a list of contacts.
    ///
    /// <https://resend.com/docs/api-reference/audiences/create-audience>
    #[maybe_async::maybe_async]
    pub async fn create(&self, audience: CreateAudienceRequest) -> Result<CreateAudienceResponse> {
        let request = self.0.build(Method::POST, "/audiences");
        let response = self.0.send(request.json(&audience)).await?;
        let content = response.json::<CreateAudienceResponse>().await?;

        Ok(content)
    }

    /// Retrieve a single audience.
    ///
    /// <https://resend.com/docs/api-reference/audiences/get-audience>
    #[maybe_async::maybe_async]
    pub async fn retrieve(&self, id: &str) -> Result<GetAudienceResponse> {
        let path = format!("/audiences/{id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<GetAudienceResponse>().await?;

        Ok(content)
    }

    /// Remove an existing audience.
    ///
    /// <https://resend.com/docs/api-reference/audiences/delete-audience>
    #[maybe_async::maybe_async]
    pub async fn delete(&self, id: &str) -> Result<RemoveAudienceResponse> {
        let path = format!("/audiences/{id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<RemoveAudienceResponse>().await?;

        Ok(content)
    }

    /// Retrieve a list of audiences.
    ///
    /// <https://resend.com/docs/api-reference/audiences/list-audiences>
    #[maybe_async::maybe_async]
    pub async fn list(&self) -> Result<ListAudiencesResponse> {
        let request = self.0.build(Method::GET, "/audiences");
        let response = self.0.send(request).await?;
        let content = response.json::<ListAudiencesResponse>().await?;

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

    // TODO: From str?
    impl CreateAudienceRequest {
        /// Creates a new [`CreateAudienceRequest`].
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_owned(),
            }
        }
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
        pub data: Option<Vec<ListAudiencesItem>>,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct ListAudiencesItem {
        /// Unique identifier for the audience.
        pub id: Option<String>,
        /// Name of the audience.
        pub name: Option<String>,
        /// Timestamp indicating when the audience was created.
        pub created_at: Option<String>,
    }
}
