use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use crate::types::ListApiKeysResponse;
use crate::types::{CreateApiKeyRequest, CreateApiKeyResponse};
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
