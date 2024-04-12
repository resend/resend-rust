use serde::{Deserialize, Serialize};

#[must_use]
#[derive(Debug, Clone, Serialize)]
pub struct CreateDomainRequest {
    /// The name of the domain you want to create.
    #[serde(rename = "name")]
    pub name: String,
    /// The region where emails will be sent from.
    /// Possible values are us-east-1' | 'eu-west-1' | 'sa-east-1
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
