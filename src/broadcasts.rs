use std::sync::Arc;

use reqwest::Method;
use types::{ListRecipientsOptions, UpdateBroadcastOptions, UpdateBroadcastResponse};

use crate::{Config, Result, list_opts::ListResponse};
use crate::{
    list_opts::ListOptions,
    types::{
        Broadcast, BroadcastRecipient, CancelBroadcastResponse, CreateBroadcastOptions,
        CreateBroadcastResponse, RemoveBroadcastResponse, SendBroadcastOptions,
        SendBroadcastResponse,
    },
};

/// `Resend` APIs for `/broadcasts` endpoints.
#[derive(Clone, Debug)]
pub struct BroadcastsSvc(pub(crate) Arc<Config>);

impl BroadcastsSvc {
    /// Create a new broadcast to send to your audience.
    ///
    /// <https://resend.com/docs/api-reference/broadcasts/create-broadcast>
    #[maybe_async::maybe_async]
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
    pub async fn send(&self, broadcast: SendBroadcastOptions) -> Result<SendBroadcastResponse> {
        let path = format!("/broadcasts/{}/send", broadcast.broadcast_id);

        let request = self.0.build(Method::POST, &path);
        let response = self.0.send(request.json(&broadcast)).await?;
        let content = response.json::<SendBroadcastResponse>().await?;

        Ok(content)
    }

    /// Retrieve a list of broadcasts.
    ///
    /// - Default limit: no limit (return everything)
    ///
    /// <https://resend.com/docs/api-reference/broadcasts/list-broadcasts>
    #[maybe_async::maybe_async]
    pub async fn list<T>(&self, list_opts: ListOptions<T>) -> Result<ListResponse<Broadcast>> {
        let request = self.0.build(Method::GET, "/broadcasts").query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<Broadcast>>().await?;

        Ok(content)
    }

    /// Retrieve a single broadcast.
    ///
    /// <https://resend.com/docs/api-reference/broadcasts/get-broadcast>
    #[maybe_async::maybe_async]
    pub async fn get(&self, broadcast_id: &str) -> Result<Broadcast> {
        let path = format!("/broadcasts/{broadcast_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Broadcast>().await?;

        Ok(content)
    }

    #[maybe_async::maybe_async]
    pub async fn cancel(&self, broadcast_id: &str) -> Result<CancelBroadcastResponse> {
        let path = format!("/broadcasts/{broadcast_id}/cancel");

        let request = self.0.build(Method::POST, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<CancelBroadcastResponse>().await?;

        Ok(content)
    }

    /// Remove an existing broadcast.
    ///
    /// <https://resend.com/docs/api-reference/broadcasts/delete-broadcast>
    #[maybe_async::maybe_async]
    pub async fn delete(&self, broadcast_id: &str) -> Result<bool> {
        let path = format!("/broadcasts/{broadcast_id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<RemoveBroadcastResponse>().await?;

        Ok(content.deleted)
    }

    /// Update a broadcast to send to your audience.
    #[maybe_async::maybe_async]
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

    /// Retrieve the recipients of a broadcast for a given event type, such as who opened,
    /// clicked, or bounced.
    ///
    /// - Default limit: 20
    ///
    /// <https://resend.com/docs/api-reference/broadcasts/list-broadcast-recipients>
    #[maybe_async::maybe_async]
    pub async fn recipients<T>(
        &self,
        broadcast_id: &str,
        list_opts: ListRecipientsOptions<T>,
    ) -> Result<ListResponse<BroadcastRecipient>> {
        let path = format!("/broadcasts/{broadcast_id}/recipients");

        let request = self.0.build(Method::GET, &path).query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<BroadcastRecipient>>().await?;

        Ok(content)
    }
}

#[allow(unreachable_pub)]
pub mod types {
    use ecow::EcoString;
    use serde::{Deserialize, Serialize};

    use crate::{
        list_opts::{ListAfter, ListBefore, ListOptions, TimeNotSpecified},
        types::{ContactId, SegmentId},
    };

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
        #[serde(skip_serializing_if = "Option::is_none")]
        send: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        scheduled_at: Option<String>,
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
                send: None,
                scheduled_at: None,
            }
        }

        /// Appends `reply_to` address to the broadcast.
        #[inline]
        pub fn with_reply(mut self, to: &str) -> Self {
            let reply_to = self.reply_to.get_or_insert_with(Vec::new);
            reply_to.push(to.to_owned());
            self
        }

        /// Appends multiple `reply_to` addresses to the broadcast.
        #[inline]
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

        /// When set to `true`, the broadcast will be sent or scheduled (if `scheduled_at` is
        /// provided) without requiring a separate call to the
        /// [`crate::broadcasts::BroadcastsSvc::send`] endpoint.
        #[inline]
        pub fn with_send(mut self, send: bool) -> Self {
            self.send = Some(send);
            self
        }

        /// Schedule email to be sent later. The date should be in language natural (e.g.: in 1 min)
        /// or ISO 8601 format (e.g: 2024-08-05T11:52:01.858Z).
        #[inline]
        pub fn with_scheduled_at(mut self, scheduled_at: &str) -> Self {
            self.scheduled_at = Some(scheduled_at.to_owned());
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
        #[inline]
        pub fn with_from(mut self, from: &str) -> Self {
            self.from = Some(from.to_owned());
            self
        }

        /// Adds or overwrites the subject.
        #[inline]
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
        #[inline]
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

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateBroadcastResponse {
        /// Unique identifier for the updated broadcast.
        pub id: BroadcastId,
    }

    crate::define_id_type!(BroadcastId);

    #[derive(Debug, Clone, Serialize, Deserialize)]
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
        #[inline]
        pub fn with_scheduled_at(mut self, scheduled_at: &str) -> Self {
            self.scheduled_at = Some(scheduled_at.to_owned());
            self
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SendBroadcastResponse {
        /// The ID of the sent broadcast.
        pub id: BroadcastId,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Broadcast {
        pub id: BroadcastId,
        pub name: String,
        pub audience_id: SegmentId,
        pub status: String,
        pub created_at: String,
        pub scheduled_at: Option<String>,
        pub sent_at: Option<String>,
        pub from: Option<String>,
        pub subject: Option<String>,
        pub reply_to: Option<Vec<String>>,
        pub preview_text: Option<String>,
        pub text: Option<String>,
        pub html: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CancelBroadcastResponse {
        pub id: BroadcastId,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RemoveBroadcastResponse {
        /// The ID of the broadcast.
        #[allow(dead_code)]
        pub id: BroadcastId,
        /// The deleted attribute indicates that the corresponding broadcast has been deleted.
        pub deleted: bool,
    }

    /// The recipient event type to filter by when listing [`BroadcastRecipient`]s.
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
    #[must_use]
    #[serde(rename_all = "snake_case")]
    pub enum BroadcastRecipientEventType {
        Sent,
        Delivered,
        Opened,
        Clicked,
        Bounced,
        Complained,
        Unsubscribed,
        Suppressed,
    }

    /// The classification of a bounce for a [`BroadcastRecipient`].
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
    #[must_use]
    #[serde(rename_all = "snake_case")]
    pub enum BroadcastRecipientBounceType {
        Permanent,
        Transient,
        Undetermined,
    }

    /// Query parameters for [`crate::broadcasts::BroadcastsSvc::recipients`].
    ///
    /// `before` and `after` are mutually exclusive; use [`ListRecipientsOptions::list_before`] or
    /// [`ListRecipientsOptions::list_after`] to pick one, same as [`ListOptions`].
    ///
    /// <https://resend.com/docs/api-reference/broadcasts/list-broadcast-recipients>
    ///
    /// ## Example
    ///
    /// ```
    /// # use resend_rs::types::{BroadcastRecipientEventType, ListRecipientsOptions};
    /// let list_opts =
    ///   ListRecipientsOptions::new(BroadcastRecipientEventType::Clicked).with_limit(10);
    /// ```
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    // The `List` parameter only ever reaches the wire through `pagination`'s own (bound-free)
    // `Serialize` impl, so it never needs to implement `Serialize` itself; the phantom marker
    // types (e.g. `TimeNotSpecified`) intentionally don't.
    #[serde(bound(serialize = ""))]
    pub struct ListRecipientsOptions<List = TimeNotSpecified> {
        #[serde(rename = "type")]
        r#type: BroadcastRecipientEventType,

        #[serde(skip_serializing_if = "Option::is_none")]
        email: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        bounce_type: Option<BroadcastRecipientBounceType>,

        #[serde(flatten)]
        pagination: ListOptions<List>,
    }

    impl ListRecipientsOptions<TimeNotSpecified> {
        /// Creates a new [`ListRecipientsOptions`], filtering recipients by the given event
        /// `type`.
        pub fn new(event_type: BroadcastRecipientEventType) -> Self {
            Self {
                r#type: event_type,
                email: None,
                bounce_type: None,
                pagination: ListOptions::default(),
            }
        }

        /// The id before which we'll retrieve the items. This id will *not* be included in the
        /// list.
        #[inline]
        pub fn list_before(self, id: &str) -> ListRecipientsOptions<ListBefore> {
            ListRecipientsOptions {
                r#type: self.r#type,
                email: self.email,
                bounce_type: self.bounce_type,
                pagination: self.pagination.list_before(id),
            }
        }

        /// The id after which we'll retrieve the items. This id will *not* be included in the
        /// list.
        #[inline]
        pub fn list_after(self, id: &str) -> ListRecipientsOptions<ListAfter> {
            ListRecipientsOptions {
                r#type: self.r#type,
                email: self.email,
                bounce_type: self.bounce_type,
                pagination: self.pagination.list_after(id),
            }
        }
    }

    impl<T> ListRecipientsOptions<T> {
        /// Number of recipients to retrieve.
        ///
        /// - min: 1
        /// - max: 100
        /// - default: 20
        #[inline]
        pub fn with_limit(mut self, limit: u8) -> Self {
            self.pagination = self.pagination.with_limit(limit);
            self
        }

        /// Filters recipients whose email contains this value.
        #[inline]
        pub fn with_email(mut self, email: &str) -> Self {
            self.email = Some(email.to_owned());
            self
        }

        /// Filters bounced recipients by bounce type.
        ///
        /// Only meaningful when `type` is [`BroadcastRecipientEventType::Bounced`].
        #[inline]
        pub fn with_bounce_type(mut self, bounce_type: BroadcastRecipientBounceType) -> Self {
            self.bounce_type = Some(bounce_type);
            self
        }
    }

    /// A link clicked by a [`BroadcastRecipient`]. Only present when `type` is `clicked`.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BroadcastRecipientClickedLink {
        /// The clicked URL.
        pub url: String,
        /// The number of times this recipient clicked this URL.
        pub clicks: u32,
    }

    /// A single recipient of a broadcast, matching the requested event `type`.
    ///
    /// <https://resend.com/docs/api-reference/broadcasts/list-broadcast-recipients>
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BroadcastRecipient {
        /// Opaque cursor identifying this row, used only for pagination. This does not identify
        /// any entity in Resend; use [`BroadcastRecipient::contact_id`] to reference the contact.
        pub id: String,
        /// The ID of the contact associated with this recipient. `None` if the recipient's email
        /// no longer maps to a contact.
        pub contact_id: Option<ContactId>,
        /// The recipient's email address.
        pub email: String,
        /// The number of times this recipient triggered the event. Only present when the
        /// requested `type` is `opened` or `clicked`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub count: Option<u32>,
        /// The type of bounce. Only present when the requested `type` is `bounced`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub bounce_type: Option<BroadcastRecipientBounceType>,
        /// The links this recipient clicked. Only present when the requested `type` is `clicked`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub clicked_links: Option<Vec<BroadcastRecipientClickedLink>>,
    }
}

#[cfg(test)]
#[allow(clippy::needless_return, clippy::indexing_slicing)]
mod test {
    #[cfg(not(feature = "blocking"))]
    use crate::{
        list_opts::ListOptions,
        test::{CLIENT, DebugResult},
        types::{
            CreateBroadcastOptions, CreateContactOptions, SendBroadcastOptions,
            UpdateBroadcastOptions,
        },
    };

    use super::types::{
        Broadcast, BroadcastRecipient, BroadcastRecipientEventType, CancelBroadcastResponse,
        ListRecipientsOptions,
    };

    #[tokio_shared_rt::test(shared = true)]
    #[serial_test::serial]
    #[cfg(not(feature = "blocking"))]
    #[ignore = "Can no longer send broadcasts from the resend.dev domain"]
    async fn create_send_broadcast() -> DebugResult<()> {
        let resend = &*CLIENT;
        std::thread::sleep(std::time::Duration::from_secs(1));

        let audience_id = resend.segments.create("audience").await?.id;

        let contact = CreateContactOptions::new("steve.wozniak@gmail.com")
            .with_first_name("Steve")
            .with_last_name("Wozniak")
            .with_unsubscribed(false)
            .with_audience_id(&audience_id);

        let _contact_id = resend.contacts.create(contact).await?;

        let from = "Acme <onboarding@resend.dev>";
        let subject = "hello world";
        let html =
            "Hi {{{FIRST_NAME|there}}}, you can unsubscribe here: {{{RESEND_UNSUBSCRIBE_URL}}}";

        std::thread::sleep(std::time::Duration::from_secs(2));

        // Create
        let broadcast = CreateBroadcastOptions::new(&audience_id, from, subject).with_html(html);
        let res = resend.broadcasts.create(broadcast).await?;

        std::thread::sleep(std::time::Duration::from_secs(4));

        // Send
        let opts = SendBroadcastOptions::new(&res.id);
        let _res = resend.broadcasts.send(opts).await?;

        // Cleanup
        std::thread::sleep(std::time::Duration::from_secs(2));

        let deleted = resend.segments.delete(&audience_id).await?;
        std::thread::sleep(std::time::Duration::from_secs(1));

        assert!(deleted);

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[serial_test::serial]
    #[cfg(not(feature = "blocking"))]
    #[ignore = "Can no longer send broadcasts from the resend.dev domain"]
    async fn list_get_broadcast() -> DebugResult<()> {
        let resend = &*CLIENT;
        std::thread::sleep(std::time::Duration::from_secs(1));

        let broadcasts = resend.broadcasts.list(ListOptions::default()).await?;
        assert!(!broadcasts.data.is_empty(), "No broadcasts found");
        let broadcast = broadcasts[0].clone();

        let _res = resend.broadcasts.get(&broadcast.id.clone()).await?;
        let _deleted = resend.broadcasts.delete(&broadcast.id).await;
        // TODO: This does not seem to be the case anymore?
        // Already used broadcasts cant be deleted
        // assert!(deleted.is_err());

        // Create fresh broadcast and delete that instead
        let audience_id = resend.segments.create("audience").await?.id;
        let from = "Acme <onboarding@resend.dev>";
        let subject = "hello world";
        let text = "text";

        let broadcast = CreateBroadcastOptions::new(&audience_id, from, subject).with_text(text);
        let res = resend.broadcasts.create(broadcast).await?;
        std::thread::sleep(std::time::Duration::from_secs(2));
        let deleted_broadcast = resend.broadcasts.delete(&res.id).await;
        let deleted_audience = resend.segments.delete(&audience_id).await;
        std::thread::sleep(std::time::Duration::from_secs(1));

        assert!(deleted_broadcast.is_ok());
        assert!(deleted_audience.is_ok());

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[serial_test::serial]
    #[cfg(not(feature = "blocking"))]
    #[ignore = "Can no longer send broadcasts from the resend.dev domain"]
    async fn update_broadcast() -> DebugResult<()> {
        let resend = &*CLIENT;
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Create audience & broadcast
        let audience_id = resend.segments.create("audience").await?.id;
        let from = "Acme <onboarding@resend.dev>";
        let subject = "hello world";

        let create_broadcast =
            CreateBroadcastOptions::new(&audience_id, from, subject).with_text("text");
        let broadcast_id = resend.broadcasts.create(create_broadcast).await?.id;
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Assert subject == initial subject
        let broadcast = resend.broadcasts.get(&broadcast_id).await?;
        assert_eq!(Some(subject.to_string()), broadcast.subject);

        std::thread::sleep(std::time::Duration::from_secs(2));

        // Update subject
        let subject = "updated";
        let opts = UpdateBroadcastOptions::new().with_subject(subject);
        let _unused = resend.broadcasts.update(&broadcast_id, opts).await?;

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

    #[test]
    fn parse_cancel_broadcast_response_test() {
        let data = r#"{
    "object": "broadcast",
    "id": "498ee8e4-7aa2-4eb5-9f04-4194848049d1"
}"#;

        let _parsed =
            serde_json::from_str::<CancelBroadcastResponse>(data).expect("Parsing failed");
    }

    #[test]
    fn parse_recipients_response_sent_test() {
        let data = r#"{
    "object": "list",
    "has_more": false,
    "data": [
        {
            "id": "b2Zmc2V0OjA",
            "contact_id": "e169aa45-1ecf-4183-9955-b1499d5701d3",
            "email": "steve.wozniak@gmail.com"
        },
        {
            "id": "b2Zmc2V0OjE",
            "contact_id": null,
            "email": "dana@example.com"
        }
    ]
}"#;

        let parsed =
            serde_json::from_str::<crate::list_opts::ListResponse<BroadcastRecipient>>(data)
                .expect("Parsing failed");

        assert!(!parsed.has_more);
        assert_eq!(parsed.len(), 2);
        assert_eq!(
            parsed[0].contact_id.as_deref(),
            Some("e169aa45-1ecf-4183-9955-b1499d5701d3")
        );
        assert!(parsed[1].contact_id.is_none());
        assert!(parsed[0].count.is_none());
        assert!(parsed[0].bounce_type.is_none());
        assert!(parsed[0].clicked_links.is_none());
    }

    #[test]
    fn parse_recipients_response_opened_test() {
        let data = r#"{
    "id": "b2Zmc2V0OjA",
    "contact_id": "e169aa45-1ecf-4183-9955-b1499d5701d3",
    "email": "steve.wozniak@gmail.com",
    "count": 3
}"#;

        let parsed = serde_json::from_str::<BroadcastRecipient>(data).expect("Parsing failed");

        assert_eq!(parsed.count, Some(3));
        assert!(parsed.bounce_type.is_none());
        assert!(parsed.clicked_links.is_none());
    }

    #[test]
    fn parse_recipients_response_clicked_test() {
        let data = r#"{
    "id": "b2Zmc2V0OjA",
    "contact_id": "e169aa45-1ecf-4183-9955-b1499d5701d3",
    "email": "carter@example.com",
    "count": 3,
    "clicked_links": [
        { "url": "https://resend.com/pricing", "clicks": 2 },
        { "url": "https://resend.com/docs", "clicks": 1 }
    ]
}"#;

        let parsed = serde_json::from_str::<BroadcastRecipient>(data).expect("Parsing failed");

        assert_eq!(parsed.count, Some(3));
        let clicked_links = parsed.clicked_links.expect("clicked_links should be set");
        assert_eq!(clicked_links.len(), 2);
        assert_eq!(clicked_links[0].url, "https://resend.com/pricing");
        assert_eq!(clicked_links[0].clicks, 2);
    }

    #[test]
    fn parse_recipients_response_bounced_test() {
        let data = r#"{
    "id": "b2Zmc2V0OjA",
    "contact_id": null,
    "email": "bounced@example.com",
    "bounce_type": "permanent"
}"#;

        let parsed = serde_json::from_str::<BroadcastRecipient>(data).expect("Parsing failed");

        assert!(parsed.contact_id.is_none());
        assert!(parsed.count.is_none());
        assert!(parsed.clicked_links.is_none());
        assert_eq!(
            parsed.bounce_type,
            Some(super::types::BroadcastRecipientBounceType::Permanent)
        );
    }

    #[test]
    fn serialize_list_recipients_options_test() {
        use super::types::BroadcastRecipientBounceType;

        let opts = ListRecipientsOptions::new(BroadcastRecipientEventType::Bounced)
            .with_email("steve")
            .with_bounce_type(BroadcastRecipientBounceType::Permanent)
            .with_limit(10)
            .list_after("cursor-123");

        let json = serde_json::to_value(&opts).expect("Failed to serialize");

        assert_eq!(json["type"], "bounced");
        assert_eq!(json["email"], "steve");
        assert_eq!(json["bounce_type"], "permanent");
        assert_eq!(json["limit"], 10);
        assert_eq!(json["after"], "cursor-123");
        assert!(json.get("before").is_none() || json["before"].is_null());
    }

    #[tokio_shared_rt::test(shared = true)]
    #[serial_test::serial]
    #[cfg(not(feature = "blocking"))]
    #[ignore = "requires RESEND_API_KEY and network access"]
    async fn recipients_not_found() -> DebugResult<()> {
        let resend = &*CLIENT;
        std::thread::sleep(std::time::Duration::from_secs(1));

        let list_opts = ListRecipientsOptions::new(BroadcastRecipientEventType::Sent);
        let result = resend
            .broadcasts
            .recipients("00000000-0000-0000-0000-000000000000", list_opts)
            .await;

        assert!(result.is_err());

        Ok(())
    }
}
