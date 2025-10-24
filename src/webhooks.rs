use std::sync::Arc;

use reqwest::Method;

use crate::{
    Config, Result,
    list_opts::{ListOptions, ListResponse},
    types::{
        CreateWebhookOptions, CreateWebhookResponse, DeleteWebhookResponse, UpdateWebhookOptions,
        UpdateWebhookResponse, Webhook,
    },
};

/// `Resend` APIs for `/webhooks` endpoints.
#[derive(Clone, Debug)]
pub struct WebhookSvc(pub(crate) Arc<Config>);

impl WebhookSvc {
    /// Create a webhook to receive real-time notifications about email events.
    ///
    /// <https://resend.com/docs/api-reference/webhooks/create-webhook>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn create(&self, webhook: CreateWebhookOptions) -> Result<CreateWebhookResponse> {
        let request = self.0.build(Method::POST, "/webhooks");
        let response = self.0.send(request.json(&webhook)).await?;
        let content = response.json::<CreateWebhookResponse>().await?;

        Ok(content)
    }

    /// Retrieve a single webhook for the authenticated user.
    ///
    /// <https://resend.com/docs/api-reference/webhooks/get-webhook>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn get(&self, webhook_id: &str) -> Result<Webhook> {
        let path = format!("/webhooks/{webhook_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Webhook>().await?;

        Ok(content)
    }

    /// Update an existing webhook configuration.
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn update(
        &self,
        webhook_id: &str,
        update: UpdateWebhookOptions,
    ) -> Result<UpdateWebhookResponse> {
        let path = format!("/webhooks/{webhook_id}");

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&update)).await?;
        let content = response.json::<UpdateWebhookResponse>().await?;

        Ok(content)
    }

    /// Retrieve a list of webhooks for the authenticated user.
    ///
    /// <https://resend.com/docs/api-reference/webhooks/list-webhooks>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn list<T>(&self, list_opts: ListOptions<T>) -> Result<ListResponse<Webhook>> {
        let request = self.0.build(Method::GET, "/webhooks").query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<Webhook>>().await?;

        Ok(content)
    }

    /// Remove an existing webhook.
    ///
    /// <https://resend.com/docs/api-reference/webhooks/delete-webhook>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn delete(&self, webhook_id: &str) -> Result<bool> {
        let path = format!("/webhooks/{webhook_id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<DeleteWebhookResponse>().await?;

        Ok(content.deleted)
    }
}

#[allow(unreachable_pub)]
pub mod types {
    use serde::{Deserialize, Serialize};

    use crate::events::EventType;

    crate::define_id_type!(WebhookId);

    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateWebhookOptions {
        endpoint: String,
        events: Vec<EventType>,
    }

    impl CreateWebhookOptions {
        pub fn new(
            endpoint: impl Into<String>,
            events: impl IntoIterator<Item = impl Into<EventType>>,
        ) -> Self {
            Self {
                endpoint: endpoint.into(),
                events: events.into_iter().map(Into::into).collect::<Vec<_>>(),
            }
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct CreateWebhookResponse {
        pub id: WebhookId,
        pub signing_secret: String,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct Webhook {
        pub id: WebhookId,
        pub created_at: String,
        pub status: String,
        pub endpoint: String,
        #[serde(default)]
        pub events: Vec<EventType>,
    }

    #[derive(Debug, Clone, Copy, Serialize)]
    #[serde(rename_all = "lowercase")]
    pub enum WebhookStatus {
        Enabled,
        Disabled,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Default)]
    pub struct UpdateWebhookOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        endpoint: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        events: Option<Vec<EventType>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        status: Option<WebhookStatus>,
    }

    impl UpdateWebhookOptions {
        #[inline]
        pub fn with_endpoint(mut self, endpoint: &str) -> Self {
            self.endpoint = Some(endpoint.to_owned());
            self
        }

        #[inline]
        pub fn with_events(
            mut self,
            events: impl IntoIterator<Item = impl Into<EventType>>,
        ) -> Self {
            self.events = Some(events.into_iter().map(Into::into).collect());
            self
        }

        #[inline]
        #[allow(clippy::missing_const_for_fn)]
        pub fn with_status(mut self, status: WebhookStatus) -> Self {
            self.status = Some(status);
            self
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct UpdateWebhookResponse {
        /// Unique identifier for the updated webhook.
        pub id: WebhookId,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct DeleteWebhookResponse {
        /// The ID of the webhook.
        #[allow(dead_code)]
        pub id: WebhookId,
        /// Indicates whether the webhook was deleted successfully.
        pub deleted: bool,
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{
        CreateWebhookOptions, CreateWebhookResponse, UpdateWebhookOptions, WebhookStatus,
    };
    use crate::{
        events::EmailEventType,
        list_opts::ListOptions,
        test::{CLIENT, DebugResult},
        types::Webhook,
    };

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> DebugResult<()> {
        let resend = &*CLIENT;

        let events = [
            EmailEventType::EmailSent,
            EmailEventType::EmailDelivered,
            EmailEventType::EmailBounced,
        ];
        let webhook = CreateWebhookOptions::new("https://webhook.example.com/handler", events);
        let id = resend.webhooks.create(webhook).await?.id;

        std::thread::sleep(std::time::Duration::from_secs(1));

        let webhook = resend.webhooks.get(&id).await?;
        assert!(webhook.events.len() == 3);
        let webhooks = resend.webhooks.list(ListOptions::default()).await?;
        assert!(!webhooks.is_empty());

        let update = UpdateWebhookOptions::default()
            .with_endpoint("https://new-webhook.example.com/handler")
            .with_events([EmailEventType::EmailSent, EmailEventType::EmailDelivered])
            .with_status(WebhookStatus::Enabled);
        let _webhook = resend.webhooks.update(&id, update).await?;
        std::thread::sleep(std::time::Duration::from_secs(1));
        let webhook = resend.webhooks.get(&id).await?;
        assert!(webhook.events.len() == 2);

        let deleted = resend.webhooks.delete(&id).await?;
        assert!(deleted);
        std::thread::sleep(std::time::Duration::from_secs(1));

        let webhooks = resend.webhooks.list(ListOptions::default()).await?;
        assert!(webhooks.is_empty());

        Ok(())
    }

    #[test]
    fn serialize_test() {
        let events = [EmailEventType::EmailSent];

        let webhook =
            CreateWebhookOptions::new("https://webhook.example.com/handler".to_owned(), events);

        let res = serde_json::to_string(&webhook);
        assert!(res.is_ok());
    }

    #[test]
    fn deserialize_test() {
        let create_webhook_res = r#"{
  "object": "webhook",
  "id": "4dd369bc-aa82-4ff3-97de-514ae3000ee0",
  "signing_secret": "whsec_xxxxxxxxxx"
}"#;

        let res = serde_json::from_str::<CreateWebhookResponse>(create_webhook_res);
        assert!(res.is_ok());

        let webhook = r#"{
  "object": "webhook",
  "id": "4dd369bc-aa82-4ff3-97de-514ae3000ee0",
  "created_at": "2023-08-22T15:28:00.000Z",
  "status": "enabled",
  "endpoint": "https://webhook.example.com/handler",
  "events": ["email.sent", "email.received"],
  "signing_secret": "whsec_xxxxxxxxxx"
}"#;

        let res = serde_json::from_str::<Webhook>(webhook);
        assert!(res.is_ok());
    }
}
