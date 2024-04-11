use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use crate::types::{CreateApiKeyRequest, CreateApiKeyResponse, ListApiKeysResponse};
use crate::{Config, Result};

/// `Resend` APIs for `METHOD /api-keys` endpoints.
#[derive(Clone)]
pub struct ApiKeys(pub(crate) Arc<Config>);

impl ApiKeys {
    /// Add a new API key to authenticate communications with Resend.
    ///
    /// <https://resend.com/docs/api-reference/api-keys/create-api-key>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn create(&self, api_key: CreateApiKeyRequest) -> Result<CreateApiKeyResponse> {
        let request = self.0.build(Method::POST, "/api-keys");
        let response = request.json(&api_key).send().await?;
        let content = response.json::<CreateApiKeyResponse>().await?;

        Ok(content)
    }

    /// Retrieve a list of API keys for the authenticated user.
    ///
    /// <https://resend.com/docs/api-reference/api-keys/list-api-keys>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn list(&self) -> Result<ListApiKeysResponse> {
        let request = self.0.build(Method::GET, "/api-keys");
        let response = request.send().await?;
        let content = response.json::<ListApiKeysResponse>().await?;

        Ok(content)
    }

    /// Remove an existing API key.
    ///
    /// <https://resend.com/docs/api-reference/api-keys/delete-api-key>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn delete(&self, api_key_id: &str) -> Result<()> {
        let path = format!("/api-keys/{api_key_id}");

        let request = self.0.build(Method::DELETE, &path);
        let _response = request.send().await?;

        Ok(())
    }

    /// Add a new API key to authenticate communications with Resend.
    ///
    /// <https://resend.com/docs/api-reference/api-keys/create-api-key>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn create(&self, api_key: CreateApiKeyRequest) -> Result<CreateApiKeyResponse> {
        let request = self.0.build(Method::POST, "/api-keys");
        let response = request.json(&api_key).send()?;
        let content = response.json::<CreateApiKeyResponse>()?;

        Ok(content)
    }

    /// Retrieve a list of API keys for the authenticated user.
    ///
    /// <https://resend.com/docs/api-reference/api-keys/list-api-keys>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn list(&self) -> Result<ListApiKeysResponse> {
        let request = self.0.build(Method::GET, "/api-keys");
        let response = request.send()?;
        let content = response.json::<ListApiKeysResponse>()?;

        Ok(content)
    }

    /// Remove an existing API key.
    ///
    /// <https://resend.com/docs/api-reference/api-keys/delete-api-key>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn delete(&self, api_key_id: &str) -> Result<()> {
        let path = format!("/api-keys/{api_key_id}");

        let request = self.0.build(Method::DELETE, &path);
        let _response = request.send()?;

        Ok(())
    }
}

impl fmt::Debug for ApiKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

pub mod types {
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
}
