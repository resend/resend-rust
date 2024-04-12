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
