use crate::{Config, Result};

/// TODO.
#[derive(Debug, Clone)]
pub struct ApiKeys(pub(crate) Config);

impl ApiKeys {
    /// Add a new API key to authenticate communications with Resend.
    ///
    /// <https://resend.com/docs/api-reference/api-keys/create-api-key>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn create(
        &self,
        request: types::CreateApiKeyRequest,
    ) -> Result<types::CreateApiKeyResponse> {
        let _ = self.0;
        todo!()
    }

    /// Retrieve a list of API keys for the authenticated user.
    ///
    /// <https://resend.com/docs/api-reference/api-keys/list-api-keys>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn list(&self) -> Result<types::ListApiKeysResponse> {
        todo!()
    }

    /// Remove an existing API key.
    ///
    /// <https://resend.com/docs/api-reference/api-keys/delete-api-key>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn delete(&self, api_keys: &str) -> Result<()> {
        todo!()
    }
}

pub mod types {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize)]
    pub struct CreateApiKeyRequest {
        /// The API key name.
        pub name: String,
        /// The API key can have full access to Resend’s API or be only restricted to send emails.
        /// * full_access - Can create, delete, get, and update any resource.
        /// * sending_access - Can only send emails.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub permission: Option<Permission>,
        /// Restrict an API key to send emails only from a specific domain.
        /// Only used when the permission is `sending_access`.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub domain_id: Option<String>,
    }

    /// The API key can have full access to Resend’s API or be only restricted to send emails.
    /// * full_access - Can create, delete, get, and update any resource.
    /// * sending_access - Can only send emails.
    #[derive(Debug, Copy, Clone, Serialize)]
    pub enum Permission {
        #[serde(rename = "full_access")]
        FullAccess,
        #[serde(rename = "sending_access")]
        SendingAccess,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct CreateApiKeyResponse {
        /// The ID of the API key.
        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        /// The token of the API key.
        #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct ApiKey {
        /// The ID of the API key.
        pub id: Option<String>,
        /// The name of the API key.
        pub name: Option<String>,
        /// The date and time the API key was created.
        pub created_at: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct ListApiKeysResponse {
        pub data: Option<Vec<ApiKey>>,
    }
}

#[cfg(test)]
mod test {}
