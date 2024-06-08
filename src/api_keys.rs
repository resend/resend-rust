use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use crate::types::{ApiKey, ApiKeyToken, CreateApiKeyOptions};
use crate::{Config, Result};

/// `Resend` APIs for `/api-keys` endpoints.
#[derive(Clone)]
pub struct ApiKeysSvc(pub(crate) Arc<Config>);

impl ApiKeysSvc {
    /// Add a new API key to authenticate communications with Resend.
    ///
    /// <https://resend.com/docs/api-reference/api-keys/create-api-key>
    #[maybe_async::maybe_async]
    // Reasoning for allow: https://github.com/resend/resend-rs/pull/1#issuecomment-2081646115
    #[allow(clippy::needless_pass_by_value)]
    pub async fn create(&self, api_key: CreateApiKeyOptions) -> Result<ApiKeyToken> {
        let request = self.0.build(Method::POST, "/api-keys");
        let response = self.0.send(request.json(&api_key)).await?;
        let content = response.json::<ApiKeyToken>().await?;

        Ok(content)
    }

    /// Retrieve a list of API keys for the authenticated user.
    ///
    /// <https://resend.com/docs/api-reference/api-keys/list-api-keys>
    #[maybe_async::maybe_async]
    pub async fn list(&self) -> Result<Vec<ApiKey>> {
        let request = self.0.build(Method::GET, "/api-keys");
        let response = self.0.send(request).await?;
        let content = response.json::<types::ListApiKeyResponse>().await?;

        Ok(content.data)
    }

    /// Remove an existing API key.
    ///
    /// <https://resend.com/docs/api-reference/api-keys/delete-api-key>
    #[maybe_async::maybe_async]
    pub async fn delete(&self, api_key_id: &str) -> Result<()> {
        let path = format!("/api-keys/{api_key_id}");

        let request = self.0.build(Method::DELETE, &path);
        let _response = self.0.send(request).await?;

        Ok(())
    }
}

impl fmt::Debug for ApiKeysSvc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

pub mod types {
    use std::{fmt, ops::Deref};

    use ecow::EcoString;
    use serde::{Deserialize, Serialize};

    use crate::types::DomainId;

    /// Unique [`ApiKey`] identifier.
    #[derive(Debug, Clone, Deserialize)]
    pub struct ApiKeyId(EcoString);

    impl ApiKeyId {
        /// Creates a new [`ApiKeyId`].
        #[inline]
        #[must_use]
        pub fn new(id: &str) -> Self {
            Self(EcoString::from(id))
        }
    }

    impl Deref for ApiKeyId {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            self.as_ref()
        }
    }

    impl AsRef<str> for ApiKeyId {
        #[inline]
        fn as_ref(&self) -> &str {
            self.0.as_str()
        }
    }

    impl fmt::Display for ApiKeyId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(&self, f)
        }
    }

    /// Name and permissions of the new [`ApiKey`].
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateApiKeyOptions {
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
        pub domain_id: Option<DomainId>,
    }

    impl CreateApiKeyOptions {
        /// Creates a new [`CreateApiKeyOptions`].
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
        pub const fn with_full_access(mut self) -> Self {
            self.permission = Some(Permission::FullAccess);
            self
        }

        /// Restricts an API key to only sending emails
        #[inline]
        pub const fn with_sending_access(mut self) -> Self {
            self.permission = Some(Permission::SendingAccess);
            self
        }

        /// Restricts an API key to send emails only from a specific domain.
        #[inline]
        pub fn with_domain_access(mut self, domain_id: &DomainId) -> Self {
            self.permission = Some(Permission::SendingAccess);
            self.domain_id = Some(domain_id.clone());
            self
        }
    }

    /// Full or restricted access of the [`ApiKey`].
    ///
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

    /// Token and ID of the newly created [`ApiKey`].
    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct ApiKeyToken {
        /// The ID of the API key.
        pub id: ApiKeyId,
        /// The token of the API key.
        pub token: String,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct ListApiKeyResponse {
        /// Array containing api key information.
        pub data: Vec<ApiKey>,
    }

    /// Name and ID of an existing API key.
    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct ApiKey {
        /// The ID of the API key.
        pub id: ApiKeyId,
        /// The name of the API key.
        pub name: String,
        /// The date and time the API key was created in ISO8601 format.
        pub created_at: String,
    }
}

#[cfg(test)]
mod test {
    use crate::tests::CLIENT;
    use crate::types::CreateApiKeyOptions;
    use crate::{Resend, Result};

    #[tokio::test]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> Result<()> {
        let resend = CLIENT.get_or_init(Resend::default);

        let api_key = "test_";

        // Create.
        let request = CreateApiKeyOptions::new(api_key).with_full_access();
        let response = resend.api_keys.create(request).await?;
        let id = response.id;

        // List.
        let api_keys = resend.api_keys.list().await?;
        let api_keys_amt = api_keys.len();

        // Delete.
        resend.api_keys.delete(&id).await?;

        // List.
        let api_keys = resend.api_keys.list().await?;
        assert!(api_keys_amt == api_keys.len() + 1);

        Ok(())
    }
}
