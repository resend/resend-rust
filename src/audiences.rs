use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use crate::types::{Audience, Audiences, CreateAudienceResponse};
use crate::{Config, Result};

/// `Resend` APIs for `/audiences` endpoints.
#[derive(Clone)]
pub struct AudiencesService(pub(crate) Arc<Config>);

impl AudiencesService {
    /// Create a list of contacts.
    ///
    /// Returns an `id` of a created audience.
    ///
    /// <https://resend.com/docs/api-reference/audiences/create-audience>
    #[maybe_async::maybe_async]
    pub async fn create(&self, audience: &str) -> Result<CreateAudienceResponse> {
        let audience = types::CreateAudienceRequest::new(audience);

        // TODO: Returns only id?
        let request = self.0.build(Method::POST, "/audiences");
        let response = self.0.send(request.json(&audience)).await?;
        let content = response.json::<CreateAudienceResponse>().await?;

        Ok(content)
    }

    /// Retrieve a single audience.
    ///
    /// <https://resend.com/docs/api-reference/audiences/get-audience>
    #[maybe_async::maybe_async]
    pub async fn get(&self, id: &str) -> Result<Audience> {
        let path = format!("/audiences/{id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Audience>().await?;

        Ok(content)
    }

    /// Remove an existing audience.
    ///
    /// Returns `true` if the audience is deleted.
    ///
    /// <https://resend.com/docs/api-reference/audiences/delete-audience>
    #[maybe_async::maybe_async]
    pub async fn delete(&self, id: &str) -> Result<bool> {
        let path = format!("/audiences/{id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<types::RemoveAudienceResponse>().await?;

        Ok(content.deleted)
    }

    /// Retrieve a list of audiences.
    ///
    /// <https://resend.com/docs/api-reference/audiences/list-audiences>
    #[maybe_async::maybe_async]
    pub async fn list(&self) -> Result<Audiences> {
        let request = self.0.build(Method::GET, "/audiences");
        let response = self.0.send(request).await?;
        let content = response.json::<Audiences>().await?;

        Ok(content)
    }
}

impl fmt::Debug for AudiencesService {
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
        pub id: String,
        /// The name of the audience.
        pub name: String,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct Audience {
        /// The ID of the audience.
        pub id: String,
        // /// The object of the audience.
        // pub object: String,
        /// The name of the audience.
        pub name: String,
        /// The date that the object was created.
        pub created_at: String,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct RemoveAudienceResponse {
        /// The ID of the audience.
        pub id: String,
        /// The deleted attribute indicates that the corresponding audience has been deleted.
        pub deleted: bool,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct Audiences {
        /// Array containing audience information.
        #[serde(rename = "data")]
        pub audiences: Vec<Audience>,
    }
}

#[cfg(test)]
mod test {
    use crate::{Client, Result};

    #[tokio::test]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> Result<()> {
        let resend = Client::default();
        let audience = "test_";

        // Create.
        let status = resend.audiences.create(audience).await?;
        let id = status.id.as_str();

        // Get.
        let data = resend.audiences.get(id).await?;
        assert_eq!(data.name.as_str(), audience);

        // List.
        let list = resend.audiences.list().await?;
        assert!(list.audiences.len() > 1);

        // Delete.
        let deleted = resend.audiences.delete(id).await?;
        assert!(deleted);

        Ok(())
    }
}
