use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use crate::{Config, Result};
use crate::types::{Audience, AudienceId};

/// `Resend` APIs for `/audiences` endpoints.
#[derive(Clone)]
pub struct AudiencesService(pub(crate) Arc<Config>);

impl AudiencesService {
    /// Creates a new list of contacts.
    ///
    /// Returns an `id` of a created audience.
    ///
    /// <https://resend.com/docs/api-reference/audiences/create-audience>
    #[maybe_async::maybe_async]
    pub async fn create(&self, name: &str) -> Result<AudienceId> {
        let audience = types::CreateAudienceRequest {
            name: name.to_owned(),
        };

        let request = self.0.build(Method::POST, "/audiences");
        let response = self.0.send(request.json(&audience)).await?;
        let content = response.json::<types::CreateAudienceResponse>().await?;

        Ok(content.id)
    }

    /// Retrieves a single audience.
    ///
    /// <https://resend.com/docs/api-reference/audiences/get-audience>
    #[maybe_async::maybe_async]
    pub async fn get(&self, id: &AudienceId) -> Result<Audience> {
        let path = format!("/audiences/{id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Audience>().await?;

        Ok(content)
    }

    /// Removes an existing audience.
    ///
    /// Returns `true` if the audience is deleted.
    ///
    /// <https://resend.com/docs/api-reference/audiences/delete-audience>
    #[maybe_async::maybe_async]
    pub async fn delete(&self, id: &AudienceId) -> Result<bool> {
        let path = format!("/audiences/{id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<types::RemoveAudienceResponse>().await?;

        Ok(content.deleted)
    }

    /// Retrieves a list of audiences.
    ///
    /// <https://resend.com/docs/api-reference/audiences/list-audiences>
    #[maybe_async::maybe_async]
    pub async fn list(&self) -> Result<Vec<Audience>> {
        let request = self.0.build(Method::GET, "/audiences");
        let response = self.0.send(request).await?;
        let content = response.json::<types::ListAudienceResponse>().await?;

        Ok(content.data)
    }
}

impl fmt::Debug for AudiencesService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

pub mod types {
    use std::fmt;

    use ecow::EcoString;
    use serde::{Deserialize, Serialize};

    /// Unique [`Audience`] identifier.
    #[derive(Debug, Clone, Deserialize)]
    pub struct AudienceId(EcoString);

    impl AudienceId {
        /// Creates a new [`AudienceId`].
        pub fn new(id: &str) -> Self {
            Self(EcoString::from(id))
        }
    }

    impl AsRef<str> for AudienceId {
        #[inline]
        fn as_ref(&self) -> &str {
            self.0.as_str()
        }
    }

    impl fmt::Display for AudienceId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(self.as_ref(), f)
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateAudienceRequest {
        /// The name of the audience you want to create.
        pub name: String,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct CreateAudienceResponse {
        /// The ID of the audience.
        pub id: AudienceId,
        /// The name of the audience.
        pub name: String,
    }

    /// Name and ID of an existing contact list.
    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct Audience {
        /// The ID of the audience.
        pub id: AudienceId,
        // /// The object of the audience.
        // pub object: String,
        /// The name of the audience.
        pub name: String,
        /// The date that the object was created in ISO8601 format.
        pub created_at: String,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct RemoveAudienceResponse {
        /// The ID of the audience.
        pub id: AudienceId,
        /// The deleted attribute indicates that the corresponding audience has been deleted.
        pub deleted: bool,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct ListAudienceResponse {
        /// Array containing audience information.
        pub data: Vec<Audience>,
    }
}

#[cfg(test)]
mod test {
    use crate::{Client, Result};

    #[tokio::test]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> Result<()> {
        let resend = Client::default();
        let audience = "test_audiences";

        // Create.
        let id = resend.audiences.create(audience).await?;

        // Get.
        let data = resend.audiences.get(&id).await?;
        assert_eq!(data.name.as_str(), audience);

        // List.
        let audiences = resend.audiences.list().await?;
        assert!(audiences.len() > 1);
        println!("{:?}", &audiences);

        // Delete.
        let deleted = resend.audiences.delete(&id).await?;
        assert!(deleted);

        Ok(())
    }
}
