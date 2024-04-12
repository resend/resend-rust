use serde::{Deserialize, Serialize};

#[must_use]
#[derive(Debug, Clone, Serialize)]
pub struct CreateApiKeyRequest {
    /// The API key name.
    pub name: String,

    /// The API key can have full access to Resend’s API or be only restricted to send emails.
    /// * `full_access` - Can create, delete, get, and update any resource.
    /// * `sending_access` - Can only send emails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<Permission>,
    /// Restrict an API key to send emails only from a specific domain.
    /// Only used when the permission is `sending_access`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
}

impl CreateApiKeyRequest {
    /// Creates a new [`CreateApiKeyRequest`].
    #[inline]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            permission: None,
            domain_id: None,
        }
    }

    /// Allows an API key to create, delete, get, and update any resource.
    #[inline]
    pub fn with_full_access(mut self) -> Self {
        self.permission = Some(Permission::FullAccess);
        self
    }

    /// Restricts an API key to only sending emails
    #[inline]
    pub fn with_sending_access(mut self) -> Self {
        self.permission = Some(Permission::SendingAccess);
        self
    }

    /// Restricts an API key to send emails only from a specific domain.
    #[inline]
    pub fn with_domain_access(mut self, domain_id: &str) -> Self {
        self.permission = Some(Permission::SendingAccess);
        self.domain_id = Some(domain_id.to_owned());
        self
    }
}

/// Full access to Resend’s API or restricted to only send emails.
/// * `full_access` - Can create, delete, get, and update any resource.
/// * `sending_access` - Can only send emails.
#[must_use]
#[derive(Debug, Copy, Clone, Serialize)]
pub enum Permission {
    #[serde(rename = "full_access")]
    FullAccess,
    #[serde(rename = "sending_access")]
    SendingAccess,
}

#[must_use]
#[derive(Debug, Clone, Deserialize)]
pub struct CreateApiKeyResponse {
    /// The ID of the API key.
    pub id: String,
    /// The token of the API key.
    pub token: String,
}

#[must_use]
#[derive(Debug, Clone, Deserialize)]
pub struct ListApiKeysResponse {
    pub data: Vec<ApiKey>,
}

#[must_use]
#[derive(Debug, Clone, Deserialize)]
pub struct ApiKey {
    /// The ID of the API key.
    pub id: String,
    /// The name of the API key.
    pub name: String,
    /// The date and time the API key was created.
    pub created_at: String,
}
