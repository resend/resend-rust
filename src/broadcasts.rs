use std::sync::Arc;

use reqwest::Method;
use types::{UpdateBroadcastOptions, UpdateBroadcastResponse};

use crate::types::{
    Broadcast, CreateBroadcastOptions, CreateBroadcastResponse, RemoveBroadcastResponse,
    SendBroadcastOptions, SendBroadcastResponse,
};
use crate::{Config, Result};

/// `Resend` APIs for `/broadcasts` endpoints.
#[derive(Clone, Debug)]
pub struct BroadcastsSvc(pub(crate) Arc<Config>);

impl BroadcastsSvc {
    /// Create a new broadcast to send to your audience.
    ///
    /// <https://resend.com/docs/api-reference/broadcasts/create-broadcast>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn create(
        &self,
        broadcast: CreateBroadcastOptions,
    ) -> Result<CreateBroadcastResponse> {
        let request = self.0.build(Method::POST, "/broadcasts");
        let response = self.0.send(request.json(&broadcast)).await?;
        let content = response.json::<CreateBroadcastResponse>().await?;

        Ok(content)
    }

    /// Start sending broadcasts to your audience through the Resend API.
    ///
    /// <https://resend.com/docs/api-reference/broadcasts/send-broadcast>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn send(&self, broadcast: SendBroadcastOptions) -> Result<SendBroadcastResponse> {
        let path = format!("/broadcasts/{}/send", broadcast.broadcast_id);

        let request = self.0.build(Method::POST, &path);
        let response = self.0.send(request.json(&broadcast)).await?;
        let content = response.json::<SendBroadcastResponse>().await?;

        Ok(content)
    }

    /// Retrieve a list of broadcasts.
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn list(&self) -> Result<Vec<Broadcast>> {
        let request = self.0.build(Method::GET, "/broadcasts");
        let response = self.0.send(request).await?;
        let content = response.json::<types::ListBroadcastResponse>().await?;

        Ok(content.data)
    }

    /// Retrieve a single broadcast.
    ///
    /// <https://resend.com/docs/api-reference/broadcasts/get-broadcast>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn get(&self, broadcast_id: &str) -> Result<Broadcast> {
        let path = format!("/broadcasts/{broadcast_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Broadcast>().await?;

        Ok(content)
    }

    /// Remove an existing broadcast.
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn delete(&self, broadcast_id: &str) -> Result<bool> {
        let path = format!("/broadcasts/{broadcast_id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<RemoveBroadcastResponse>().await?;

        Ok(content.deleted)
    }

    /// Update a broadcast to send to your audience.
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn update(
        &self,
        broadcast_id: &str,
        update: UpdateBroadcastOptions,
    ) -> Result<UpdateBroadcastResponse> {
        let path = format!("/broadcasts/{broadcast_id}");

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&update)).await?;
        let content = response.json::<UpdateBroadcastResponse>().await?;

        Ok(content)
    }
}

pub mod types {
    use std::{fmt, ops::Deref};

    use ecow::EcoString;
    use serde::{Deserialize, Serialize};

    use crate::types::AudienceId;

    /// Details of a new `Broadcast`.
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateBroadcastOptions {
        audience_id: String,
        from: String,
        subject: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_to: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        html: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    }

    impl CreateBroadcastOptions {
        /// Creates a new [`CreateBroadcastOptions`].
        ///
        /// - `audience_id`: The ID of the audience you want to send to.
        /// - `from`: To include a friendly name, use the format `"Your Name <sender@domain.com>"`.
        /// - `subject`: Email subject.
        pub fn new(audience_id: &str, from: &str, subject: &str) -> Self {
            Self {
                audience_id: audience_id.to_string(),
                from: from.to_string(),
                subject: subject.to_string(),
                reply_to: None,
                html: None,
                text: None,
                name: None,
            }
        }

        /// Appends `reply_to` address to the broadcast.
        pub fn with_reply(mut self, to: &str) -> Self {
            let reply_to = self.reply_to.get_or_insert_with(Vec::new);
            reply_to.push(to.to_owned());
            self
        }

        /// Appends multiple `reply_to` addresses to the broadcast.
        pub fn with_reply_multiple(mut self, to: &[String]) -> Self {
            let reply_to = self.reply_to.get_or_insert_with(Vec::new);
            reply_to.extend_from_slice(to);
            self
        }

        /// Adds or overwrites the HTML version of the message.
        #[inline]
        pub fn with_html(mut self, html: &str) -> Self {
            self.html = Some(html.to_owned());
            self
        }

        /// Adds or overwrites the plain text version of the message.
        #[inline]
        pub fn with_text(mut self, text: &str) -> Self {
            self.text = Some(text.to_owned());
            self
        }

        /// Sets the broadast name.
        #[inline]
        pub fn with_name(mut self, name: &str) -> Self {
            self.name = Some(name.to_owned());
            self
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Default)]
    pub struct UpdateBroadcastOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        from: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        subject: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_to: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        html: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    }

    impl UpdateBroadcastOptions {
        /// Creates a new [`UpdateBroadcastOptions`].
        pub fn new() -> Self {
            Self::default()
        }

        /// Adds or overwrites the sender email address.
        pub fn with_from(mut self, from: &str) -> Self {
            self.from = Some(from.to_owned());
            self
        }

        pub fn with_subject(mut self, subject: &str) -> Self {
            self.subject = Some(subject.to_owned());
            self
        }

        /// Appends `reply_to` address to the broadcast.
        pub fn with_reply(mut self, to: &str) -> Self {
            let reply_to = self.reply_to.get_or_insert_with(Vec::new);
            reply_to.push(to.to_owned());
            self
        }

        /// Appends multiple `reply_to` addresses to the broadcast.
        pub fn with_reply_multiple(mut self, to: &[String]) -> Self {
            let reply_to = self.reply_to.get_or_insert_with(Vec::new);
            reply_to.extend_from_slice(to);
            self
        }

        /// Adds or overwrites the HTML version of the message.
        #[inline]
        pub fn with_html(mut self, html: &str) -> Self {
            self.html = Some(html.to_owned());
            self
        }

        /// Adds or overwrites the plain text version of the message.
        #[inline]
        pub fn with_text(mut self, text: &str) -> Self {
            self.text = Some(text.to_owned());
            self
        }

        /// Sets the broadast name.
        #[inline]
        pub fn with_name(mut self, name: &str) -> Self {
            self.name = Some(name.to_owned());
            self
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct UpdateBroadcastResponse {
        /// Unique identifier for the updated broadcast.
        pub id: BroadcastId,
    }

    /// Unique [`Broadcast`] identifier.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BroadcastId(EcoString);

    impl BroadcastId {
        /// Creates a new [`BroadcastId`].
        #[inline]
        #[must_use]
        pub fn new(id: &str) -> Self {
            Self(EcoString::from(id))
        }
    }

    impl Deref for BroadcastId {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            self.as_ref()
        }
    }

    impl AsRef<str> for BroadcastId {
        #[inline]
        fn as_ref(&self) -> &str {
            self.0.as_str()
        }
    }

    impl fmt::Display for BroadcastId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(&self.0, f)
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct CreateBroadcastResponse {
        /// The ID of the created broadcast.
        pub id: BroadcastId,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct SendBroadcastOptions {
        pub(crate) broadcast_id: BroadcastId,

        #[serde(skip_serializing_if = "Option::is_none")]
        scheduled_at: Option<String>,
    }

    impl SendBroadcastOptions {
        pub fn new(broadcast_id: &str) -> Self {
            let broadcast_id = BroadcastId(EcoString::from(broadcast_id.to_owned()));

            Self {
                broadcast_id,
                scheduled_at: None,
            }
        }

        /// Schedule email to be sent later. The date should be in language natural (e.g.: in 1 min)
        /// or ISO 8601 format (e.g: 2024-08-05T11:52:01.858Z).
        pub fn with_scheduled_at(mut self, scheduled_at: &str) -> Self {
            self.scheduled_at = Some(scheduled_at.to_owned());
            self
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct SendBroadcastResponse {
        /// The ID of the sent broadcast.
        pub id: BroadcastId,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct Broadcast {
        pub id: BroadcastId,
        pub name: String,
        pub audience_id: AudienceId,
        pub status: String,
        pub created_at: String,
        pub scheduled_at: Option<String>,
        pub sent_at: Option<String>,
        pub from: Option<String>,
        pub subject: Option<String>,
        pub reply_to: Option<Vec<String>>,
        pub preview_text: Option<String>,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct ListBroadcastResponse {
        pub data: Vec<Broadcast>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct RemoveBroadcastResponse {
        /// The ID of the broadcast.
        #[allow(dead_code)]
        pub id: BroadcastId,
        /// The deleted attribute indicates that the corresponding broadcast has been deleted.
        pub deleted: bool,
    }
}

#[cfg(test)]
#[allow(clippy::needless_return)]
mod test {
    use crate::{
        test::DebugResult,
        tests::CLIENT,
        types::{CreateBroadcastOptions, SendBroadcastOptions, UpdateBroadcastOptions},
    };

    use super::types::Broadcast;

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn create_send_broadcast() -> DebugResult<()> {
        let resend = &*CLIENT;
        std::thread::sleep(std::time::Duration::from_secs(1));

        let audience_id = resend.audiences.create("audience").await?.id;
        let from = "Acme <onboarding@resend.dev>";
        let subject = "hello world";
        let html =
            "Hi {{{FIRST_NAME|there}}}, you can unsubscribe here: {{{RESEND_UNSUBSCRIBE_URL}}}";

        std::thread::sleep(std::time::Duration::from_secs(2));

        // Create
        let broadcast = CreateBroadcastOptions::new(&audience_id, from, subject).with_html(html);
        let res = resend.broadcasts.create(broadcast).await?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        // Send
        let opts = SendBroadcastOptions::new(&res.id);
        let _res = resend.broadcasts.send(opts).await?;

        // Cleanup
        std::thread::sleep(std::time::Duration::from_secs(2));

        let deleted = resend.audiences.delete(&audience_id).await?;
        std::thread::sleep(std::time::Duration::from_secs(1));

        assert!(deleted);

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn list_get_broadcast() -> DebugResult<()> {
        let resend = &*CLIENT;
        std::thread::sleep(std::time::Duration::from_secs(1));

        let broadcasts = resend.broadcasts.list().await?;
        assert!(!broadcasts.is_empty(), "No broadcasts found");
        let broadcast = broadcasts[0].clone();

        let _res = resend.broadcasts.get(&broadcast.id.clone()).await?;
        let deleted = resend.broadcasts.delete(&broadcast.id).await;
        // Already used broadcasts cant be deleted
        assert!(deleted.is_err());

        // Create fresh broadcast and delete that instead
        let audience_id = resend.audiences.create("audience").await?.id;
        let from = "Acme <onboarding@resend.dev>";
        let subject = "hello world";
        let text = "text";

        let broadcast = CreateBroadcastOptions::new(&audience_id, from, subject).with_text(text);
        let res = resend.broadcasts.create(broadcast).await?;
        std::thread::sleep(std::time::Duration::from_secs(2));
        let deleted_broadcast = resend.broadcasts.delete(&res.id).await;
        let deleted_audience = resend.audiences.delete(&audience_id).await;
        std::thread::sleep(std::time::Duration::from_secs(1));

        assert!(deleted_broadcast.is_ok());
        assert!(deleted_audience.is_ok());

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    #[track_caller]
    async fn update_broadcast() -> DebugResult<()> {
        let resend = &*CLIENT;
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Create audience & broadcast
        let audience_id = resend.audiences.create("audience").await?.id;
        let from = "Acme <onboarding@resend.dev>";
        let subject = "hello world";

        let create_broadcast =
            CreateBroadcastOptions::new(&audience_id, from, subject).with_text("text");
        let broadcast_id = resend.broadcasts.create(create_broadcast).await?.id;

        // Assert subject == initial subject
        let broadcast = resend.broadcasts.get(&broadcast_id).await?;
        assert_eq!(Some(subject.to_string()), broadcast.subject);

        std::thread::sleep(std::time::Duration::from_secs(1));

        // Update subject
        let subject = "updated";
        let opts = UpdateBroadcastOptions::new().with_subject(subject);
        let _ = resend.broadcasts.update(&broadcast_id, opts).await?;

        // Assert subject == updated subject
        let broadcast = resend.broadcasts.get(&broadcast_id).await?;
        assert_eq!(Some(subject.to_string()), broadcast.subject);

        // Delete
        let deleted = resend.broadcasts.delete(&broadcast_id).await?;
        assert!(deleted);

        Ok(())
    }

    #[test]
    fn parse_broadcast_test() {
        let data = r#"{
    "object": "broadcast",
    "id": "498ee8e4-7aa2-4eb5-9f04-4194848049d1",
    "name": "Untitled",
    "audience_id": "fd644f07-a05a-467e-9bae-23bb7c35766a",
    "from": "Acme <onboarding@resend.dev>",
    "subject": "Hello!",
    "reply_to": [],
    "preview_text": null,
    "status": "scheduled",
    "created_at": "2024-12-18 18:05:09.905933+00",
    "scheduled_at": null,
    "sent_at": null
}"#;

        let _parsed = serde_json::from_str::<Broadcast>(data).expect("Parsing failed");
    }
}
