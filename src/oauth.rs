use std::sync::Arc;

use reqwest::Method;

use crate::{
    Config, Result,
    list_opts::{ListOptions, ListResponse},
    types::{OAuthGrant, RevokeOAuthGrantResponse},
};

/// `Resend` APIs for `/oauth` endpoints.
#[derive(Clone, Debug)]
pub struct OAuthSvc(pub(crate) Arc<Config>);

impl OAuthSvc {
    /// Retrieve a list of OAuth grants for the authenticated team.
    ///
    /// - Default limit: *infinite*
    ///
    /// <https://resend.com/docs/api-reference/oauth/list-grants>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn list<T>(&self, list_opts: ListOptions<T>) -> Result<ListResponse<OAuthGrant>> {
        let request = self.0.build(Method::GET, "/oauth/grants").query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<OAuthGrant>>().await?;

        Ok(content)
    }

    /// Revoke an OAuth grant for the authenticated team.
    ///
    /// <https://resend.com/docs/api-reference/oauth/revoke-grant>
    #[maybe_async::maybe_async]
    pub async fn revoke(&self, oauth_grant_id: &str) -> Result<RevokeOAuthGrantResponse> {
        let path = format!("/oauth/grants/{oauth_grant_id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<RevokeOAuthGrantResponse>().await?;

        Ok(content)
    }
}

#[allow(unreachable_pub)]
pub mod types {
    use serde::{Deserialize, Serialize};

    crate::define_id_type!(OAuthGrantId);
    crate::define_id_type!(ClientId);

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct OAuthGrant {
        pub id: OAuthGrantId,
        pub client_id: ClientId,
        pub scopes: Vec<String>,
        pub created_at: String,
        pub revoked_at: Option<String>,
        pub revoked_reason: Option<String>,
        pub client: OAuthGrantClient,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct OAuthGrantClient {
        pub name: String,
        pub logo_uri: Option<String>,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RevokeOAuthGrantResponse {
        pub id: OAuthGrantId,
        pub revoked_at: String,
        pub revoked_reason: String,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test {
    use crate::{
        list_opts::{ListOptions, ListResponse},
        types::OAuthGrant,
    };
    use crate::{
        test::{CLIENT, DebugResult},
        types::RevokeOAuthGrantResponse,
    };

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> DebugResult<()> {
        let resend = &*CLIENT;

        let logs = resend.oauth.list(ListOptions::default()).await?;
        assert!(logs.data.is_empty());

        Ok(())
    }

    #[test]
    fn deserialize_grant() {
        let grant = r#"{
          "id": "650e8400-e29b-41d4-a716-446655440002",
          "client_id": "430eed87-632a-4ea6-90db-0aace67ec228",
          "scopes": ["emails:send", "domains:read"],
          "created_at": "2026-04-07 00:11:13.110779+00",
          "revoked_at": "2026-04-09 00:11:13.110779+00",
          "revoked_reason": "revoked_from_api",
          "client": {
            "name": "Resend CLI",
            "logo_uri": "https://example.com/logo.png"
          }
        }"#;

        let res = serde_json::from_str::<OAuthGrant>(grant);
        assert!(res.is_ok());
    }

    #[test]
    fn deserialize_list() {
        let grants = r#"{
          "object": "list",
          "has_more": false,
          "data": [
            {
              "id": "650e8400-e29b-41d4-a716-446655440001",
              "client_id": "430eed87-632a-4ea6-90db-0aace67ec228",
              "scopes": ["emails:send"],
              "created_at": "2026-04-08 00:11:13.110779+00",
              "revoked_at": null,
              "revoked_reason": null,
              "client": {
                "name": "Resend CLI",
                "logo_uri": "https://example.com/logo.png"
              }
            },
            {
              "id": "650e8400-e29b-41d4-a716-446655440002",
              "client_id": "430eed87-632a-4ea6-90db-0aace67ec228",
              "scopes": ["emails:send", "domains:read"],
              "created_at": "2026-04-07 00:11:13.110779+00",
              "revoked_at": "2026-04-09 00:11:13.110779+00",
              "revoked_reason": "revoked_from_api",
              "client": {
                "name": "Resend CLI",
                "logo_uri": "https://example.com/logo.png"
              }
            }
          ]
        }"#;

        let res = serde_json::from_str::<ListResponse<OAuthGrant>>(grants);
        assert!(res.is_ok(), "{:?}", res.err());
    }

    #[test]
    fn deserialize_revoke() {
        let revoke = r#"{
          "object": "oauth_grant",
          "id": "650e8400-e29b-41d4-a716-446655440001",
          "revoked_at": "2026-04-08T00:11:13.110Z",
          "revoked_reason": "revoked_from_api"
        }"#;

        let res = serde_json::from_str::<RevokeOAuthGrantResponse>(revoke);
        assert!(res.is_ok());
    }
}
