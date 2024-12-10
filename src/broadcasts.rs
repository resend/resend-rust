use std::sync::Arc;

use reqwest::Method;

use crate::types::{
    Broadcast, BroadcastId, CreateBroadcastOptions, CreateBroadcastResponse,
    RemoveBroadcastResponse, SendBroadcastOptions, SendBroadcastResponse,
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

    // TODO: docs
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn list(&self) -> Result<Vec<Broadcast>> {
        let request = self.0.build(Method::GET, "/broadcasts");
        let response = self.0.send(request).await?;
        let content = response.json::<types::ListBroadcastResponse>().await?;

        Ok(content.data)
    }

    // TODO: docs
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn get(&self, broadcast_id: BroadcastId) -> Result<Broadcast> {
        let path = format!("/broadcasts/{broadcast_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Broadcast>().await?;

        Ok(content)
    }

    // TODO: docs
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn delete(&self, broadcast_id: BroadcastId) -> Result<bool> {
        let path = format!("/broadcasts/{broadcast_id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<RemoveBroadcastResponse>().await?;

        Ok(content.deleted)
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

    /// Unique [`Email`] identifier.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BroadcastId(EcoString);

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
        pub const fn new(broadcast_id: BroadcastId) -> Self {
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
mod test {
    use crate::{
        tests::CLIENT,
        types::{CreateBroadcastOptions, SendBroadcastOptions},
        Result,
    };

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn create_send_broadcast() -> Result<()> {
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
        let opts = SendBroadcastOptions::new(res.id);
        let _res = resend.broadcasts.send(opts).await?;

        // Cleanup
        std::thread::sleep(std::time::Duration::from_secs(2));

        let deleted = resend.audiences.delete(audience_id).await?;
        std::thread::sleep(std::time::Duration::from_secs(1));

        assert!(deleted);

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn list_get_broadcast() -> Result<()> {
        let resend = &*CLIENT;
        std::thread::sleep(std::time::Duration::from_secs(1));

        let broadcasts = resend.broadcasts.list().await?;
        assert!(!broadcasts.is_empty(), "No broadcasts found");
        let broadcast = broadcasts[0].clone();

        let _res = resend.broadcasts.get(broadcast.id.clone()).await?;
        let deleted = resend.broadcasts.delete(broadcast.id).await;
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
        let deleted = resend.broadcasts.delete(res.id).await;
        std::thread::sleep(std::time::Duration::from_secs(1));

        assert!(deleted.is_ok());

        Ok(())
    }
}
