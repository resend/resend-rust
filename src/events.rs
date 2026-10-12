//! Parsing for Resend's Events.
//!
//! For an example on how to add (Axum) middleware that verifies incoming event signatures,
//! check out [this example](https://github.com/resend/resend-rust/blob/main/examples/axum-verify-event-middleware.rs).

#![allow(dead_code)]

use std::{collections::HashMap, sync::Arc};

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    Config, Result,
    list_opts::{ListOptions, ListResponse},
    types::{
        BroadcastId, CreateEventOptions, CreateEventResponse, DeleteEventResponse, Domain, EmailId,
        GetEventResponse, InboundAttachment, Inbox, InboxDraftId, InboxDraftType,
        InboxEmailDirection, InboxFolder, InboxId, InboxThreadEmailId, InboxThreadId,
        InboxThreadLabel, InboxThreadSummary, SegmentId, SendEventOptions, SendEventResponse,
        SubscriptionType, TemplateId, TopicId, UpdateEventOptions, UpdateEventResponse,
    },
};

/// `Resend` APIs for `/events` endpoints.
#[derive(Clone, Debug)]
pub struct EventsSvc(pub(crate) Arc<Config>);

impl EventsSvc {
    /// Create a new event that can be used to trigger automations.
    ///
    /// <https://resend.com/docs/api-reference/events/create-event>
    #[maybe_async::maybe_async]
    pub async fn create(&self, event: CreateEventOptions) -> Result<CreateEventResponse> {
        let request = self.0.build(Method::POST, "/events");
        let response = self.0.send(request.json(&event)).await?;
        let content = response.json::<CreateEventResponse>().await?;

        Ok(content)
    }

    /// Send a named event to trigger matching automations.
    ///
    /// <https://resend.com/docs/api-reference/events/send-event>
    #[maybe_async::maybe_async]
    pub async fn send(&self, opts: SendEventOptions) -> Result<SendEventResponse> {
        let request = self.0.build(Method::POST, "/events/send");
        let response = self.0.send(request.json(&opts)).await?;
        let content = response.json::<SendEventResponse>().await?;

        Ok(content)
    }

    /// Retrieve a single event by ID or name.
    ///
    /// <https://resend.com/docs/api-reference/events/get-event>
    #[maybe_async::maybe_async]
    pub async fn get(&self, event_id: &str) -> Result<GetEventResponse> {
        let path = format!("/events/{event_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<GetEventResponse>().await?;

        Ok(content)
    }

    /// Retrieve a list of events.
    ///
    /// <https://resend.com/docs/api-reference/events/list-events>
    #[maybe_async::maybe_async]
    pub async fn list<T>(
        &self,
        list_opts: ListOptions<T>,
    ) -> Result<ListResponse<GetEventResponse>> {
        let request = self.0.build(Method::GET, "/events").query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<GetEventResponse>>().await?;

        Ok(content)
    }

    /// Update an existing event schema.
    ///
    /// <https://resend.com/docs/api-reference/events/update-event>
    #[maybe_async::maybe_async]
    pub async fn update(
        &self,
        event_id: &str,
        update: UpdateEventOptions,
    ) -> Result<UpdateEventResponse> {
        let path = format!("/events/{event_id}");

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&update)).await?;
        let content = response.json::<UpdateEventResponse>().await?;

        Ok(content)
    }

    /// Remove an existing event.
    ///
    /// <https://resend.com/docs/api-reference/events/delete-event>
    #[maybe_async::maybe_async]
    pub async fn delete(&self, event_id: &str) -> Result<DeleteEventResponse> {
        let path = format!("/events/{event_id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<DeleteEventResponse>().await?;

        Ok(content)
    }
}

#[allow(unreachable_pub)]
pub mod types {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    crate::define_id_type!(EventId);

    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateEventOptions {
        pub name: String,
        pub schema: Value,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreateEventResponse {
        pub id: EventId,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum ContactIdOrEmail {
        ContactId(String),
        Email(String),
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct SendEventOptions {
        pub event: String,
        #[serde(flatten)]
        pub contact_id_or_email: ContactIdOrEmail,
        pub payload: Value,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct UpdateEventOptions {
        pub schema: Value,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SendEventResponse {
        pub event: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GetEventResponse {
        pub id: EventId,
        pub name: String,
        pub schema: Option<Value>,
        pub created_at: String,
        pub updated_at: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DeleteEventResponse {
        pub id: EventId,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateEventResponse {
        pub id: EventId,
    }
}

/// Parses a JSON event into an [`Event`].
/// ## Example
///
/// ```
/// use resend_rs::events::{Event, try_parse_event};
///
/// let data = r#"
///   {
///     "type": "email.sent",
///     "created_at": "2024-02-22T23:41:12.126Z",
///     "data": {
///       "broadcast_id": "8b146471-e88e-4322-86af-016cd36fd216",
///       "created_at": "2024-02-22T23:41:11.894Z",
///       "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
///       "message_id": "<111-222-333@email.example.com>",
///       "from": "Acme <onboarding@resend.dev>",
///       "to": ["delivered@resend.dev"],
///       "subject": "Sending this example",
///       "template_id": "43f68331-0622-4e15-8202-246a0388854b",
///       "tags": {
///         "category": "confirm_email"
///       }
///     }
/// }"#;
///
///  let parsed: Result<Event, resend_rs::Error> = try_parse_event(data);
/// ```
pub fn try_parse_event(data: &str) -> Result<Event> {
    serde_json::from_str::<Event>(data).map_err(|e| crate::Error::Parse {
        message: "Could not parse event".to_owned(),
        source: Some(Box::new(e)),
    })
}

/// Parses an event type string into an [`EventType`].
/// ## Example
///
/// ```
/// use resend_rs::events::{EventType, try_parse_event_type};
///
/// let data = "\"email.sent\"";
///
///  let parsed: Result<EventType, resend_rs::Error> = try_parse_event_type(data);
/// ```
pub fn try_parse_event_type(data: &str) -> Result<EventType> {
    serde_json::from_str::<EventType>(data).map_err(|e| crate::Error::Parse {
        message: "Could not parse event type".to_owned(),
        source: Some(Box::new(e)),
    })
}

/// Represents any [Resend Event](https://resend.com/docs/dashboard/webhooks/event-types).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Event {
    EmailEvent(EmailEvent),
    ContactEvent(ContactEvent),
    ContactTopicsEvent(ContactTopicsEvent),
    DomainEvent(DomainEvent),
    SuppressionEvent(SuppressionEvent),
    TopicEvent(TopicEvent),
    InboxEvent(InboxEvent),
}

/// Represents any [Resend Event Type](https://resend.com/docs/dashboard/webhooks/event-types).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventType {
    EmailEventType(EmailEventType),
    ContactEventType(ContactEventType),
    ContactTopicsEventType(ContactTopicsEventType),
    DomainEventType(DomainEventType),
    SuppressionEventType(SuppressionEventType),
    TopicEventType(TopicEventType),
    InboxEventType(InboxEventType),
}

impl From<EmailEventType> for EventType {
    fn from(value: EmailEventType) -> Self {
        Self::EmailEventType(value)
    }
}

impl From<ContactEventType> for EventType {
    fn from(value: ContactEventType) -> Self {
        Self::ContactEventType(value)
    }
}

impl From<ContactTopicsEventType> for EventType {
    fn from(value: ContactTopicsEventType) -> Self {
        Self::ContactTopicsEventType(value)
    }
}

impl From<DomainEventType> for EventType {
    fn from(value: DomainEventType) -> Self {
        Self::DomainEventType(value)
    }
}

impl From<SuppressionEventType> for EventType {
    fn from(value: SuppressionEventType) -> Self {
        Self::SuppressionEventType(value)
    }
}

impl From<TopicEventType> for EventType {
    fn from(value: TopicEventType) -> Self {
        Self::TopicEventType(value)
    }
}

impl From<InboxEventType> for EventType {
    fn from(value: InboxEventType) -> Self {
        Self::InboxEventType(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailEvent {
    #[serde(rename = "type")]
    pub r#type: EmailEventType,
    pub created_at: String,
    pub data: EmailBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactEvent {
    #[serde(rename = "type")]
    pub r#type: ContactEventType,
    pub created_at: String,
    pub data: ContactBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactTopicsEvent {
    #[serde(rename = "type")]
    pub r#type: ContactTopicsEventType,
    pub created_at: String,
    pub data: ContactTopicsBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEvent {
    #[serde(rename = "type")]
    pub r#type: DomainEventType,
    pub created_at: String,
    pub data: Domain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuppressionEvent {
    #[serde(rename = "type")]
    pub r#type: SuppressionEventType,
    pub created_at: String,
    pub data: SuppressionBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicEvent {
    #[serde(rename = "type")]
    pub r#type: TopicEventType,
    pub created_at: String,
    pub data: TopicBody,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(strum::EnumCount))]
pub enum EmailEventType {
    #[serde(rename = "email.sent")]
    EmailSent,
    #[serde(rename = "email.suppressed")]
    EmailSuppressed,
    #[serde(rename = "email.delivered")]
    EmailDelivered,
    #[serde(rename = "email.delivery_delayed")]
    EmailDeliveryDelayed,
    #[serde(rename = "email.complained")]
    EmailComplained,
    #[serde(rename = "email.bounced")]
    EmailBounced,
    #[serde(rename = "email.opened")]
    EmailOpened,
    #[serde(rename = "email.clicked")]
    EmailClicked,
    #[serde(rename = "email.received")]
    EmailReceived,
    #[serde(rename = "email.scheduled")]
    EmailScheduled,
    #[serde(rename = "email.failed")]
    EmailFailed,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(strum::EnumCount))]
pub enum ContactEventType {
    #[serde(rename = "contact.created")]
    ContactCreated,
    #[serde(rename = "contact.updated")]
    ContactUpdated,
    #[serde(rename = "contact.deleted")]
    ContactDeleted,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(strum::EnumCount))]
pub enum ContactTopicsEventType {
    #[serde(rename = "contact.topics.updated")]
    ContactTopicsUpdated,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(strum::EnumCount))]
pub enum DomainEventType {
    #[serde(rename = "domain.created")]
    DomainCreated,
    #[serde(rename = "domain.updated")]
    DomainUpdated,
    #[serde(rename = "domain.deleted")]
    DomainDeleted,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(strum::EnumCount))]
pub enum SuppressionEventType {
    #[serde(rename = "suppression.added")]
    SuppressionAdded,
    #[serde(rename = "suppression.removed")]
    SuppressionRemoved,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(strum::EnumCount))]
pub enum TopicEventType {
    #[serde(rename = "topic.created")]
    TopicCreated,
    #[serde(rename = "topic.updated")]
    TopicUpdated,
    #[serde(rename = "topic.deleted")]
    TopicDeleted,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(strum::EnumCount))]
pub enum InboxEventType {
    #[serde(rename = "inbox.created")]
    InboxCreated,
    #[serde(rename = "inbox.updated")]
    InboxUpdated,
    #[serde(rename = "inbox.deleted")]
    InboxDeleted,
    #[serde(rename = "inbox.thread.created")]
    InboxThreadCreated,
    #[serde(rename = "inbox.thread.folder.updated")]
    InboxThreadFolderUpdated,
    #[serde(rename = "inbox.thread.assigned")]
    InboxThreadAssigned,
    #[serde(rename = "inbox.thread.unassigned")]
    InboxThreadUnassigned,
    #[serde(rename = "inbox.thread.labels.updated")]
    InboxThreadLabelsUpdated,
    #[serde(rename = "inbox.email.received")]
    InboxEmailReceived,
    #[serde(rename = "inbox.email.sent")]
    InboxEmailSent,
    #[serde(rename = "inbox.draft.created")]
    InboxDraftCreated,
    #[serde(rename = "inbox.draft.updated")]
    InboxDraftUpdated,
    #[serde(rename = "inbox.draft.sent")]
    InboxDraftSent,
    #[serde(rename = "inbox.draft.deleted")]
    InboxDraftDeleted,
}

/// An `inbox.*` event. The shape of `data` depends on the event type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(clippy::large_enum_variant)]
pub enum InboxEvent {
    #[serde(rename = "inbox.created")]
    InboxCreated { created_at: String, data: InboxBody },
    #[serde(rename = "inbox.updated")]
    InboxUpdated { created_at: String, data: InboxBody },
    #[serde(rename = "inbox.deleted")]
    InboxDeleted {
        created_at: String,
        data: InboxDeletedBody,
    },
    #[serde(rename = "inbox.thread.created")]
    InboxThreadCreated {
        created_at: String,
        data: InboxThreadEmailBody,
    },
    #[serde(rename = "inbox.thread.folder.updated")]
    InboxThreadFolderUpdated {
        created_at: String,
        data: InboxThreadFolderUpdatedBody,
    },
    #[serde(rename = "inbox.thread.assigned")]
    InboxThreadAssigned {
        created_at: String,
        data: InboxThreadAssignedBody,
    },
    #[serde(rename = "inbox.thread.unassigned")]
    InboxThreadUnassigned {
        created_at: String,
        data: InboxThreadUnassignedBody,
    },
    #[serde(rename = "inbox.thread.labels.updated")]
    InboxThreadLabelsUpdated {
        created_at: String,
        data: InboxThreadLabelsUpdatedBody,
    },
    #[serde(rename = "inbox.email.received")]
    InboxEmailReceived {
        created_at: String,
        data: InboxThreadEmailBody,
    },
    #[serde(rename = "inbox.email.sent")]
    InboxEmailSent {
        created_at: String,
        data: InboxThreadEmailBody,
    },
    #[serde(rename = "inbox.draft.created")]
    InboxDraftCreated {
        created_at: String,
        data: InboxDraftBody,
    },
    #[serde(rename = "inbox.draft.updated")]
    InboxDraftUpdated {
        created_at: String,
        data: InboxDraftBody,
    },
    #[serde(rename = "inbox.draft.sent")]
    InboxDraftSent {
        created_at: String,
        data: InboxDraftSentBody,
    },
    #[serde(rename = "inbox.draft.deleted")]
    InboxDraftDeleted {
        created_at: String,
        data: InboxDraftBody,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailBody {
    pub broadcast_id: Option<BroadcastId>,
    pub created_at: String,
    pub email_id: EmailId,
    /// RFC Message-ID header value for the email.
    pub message_id: String,
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    pub template_id: Option<TemplateId>,

    #[serde(flatten)]
    pub received: Option<Received>,
    pub click: Option<Click>,
    pub bounce: Option<Bounce>,
    pub failed: Option<Failed>,
    pub suppressed: Option<Suppressed>,

    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Extra data only populated in [`EmailEventType::EmailSuppressed`] events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suppressed {
    pub message: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Extra data only populated in [`EmailEventType::EmailReceived`] events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Received {
    pub bcc: Vec<String>,
    pub cc: Vec<String>,
    #[serde(default)]
    pub received_for: Vec<String>,
    pub attachments: Vec<InboundAttachment>,
}

/// Extra data only populated in [`EmailEventType::EmailFailed`] events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Failed {
    pub reason: String,
}

/// Extra data only populated in [`EmailEventType::EmailBounced`] events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bounce {
    pub message: String,
    #[serde(rename = "subType")]
    pub sub_type: BounceType,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BounceType {
    Suppressed,
    MessageRejected,
}

/// Extra data only populated in [`EmailEventType::EmailClicked`] events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Click {
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    pub link: String,
    pub timestamp: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactBody {
    pub id: String,
    pub audience_id: String,
    pub segment_ids: Vec<SegmentId>,
    pub created_at: String,
    pub updated_at: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub unsubscribed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactTopicsBody {
    pub email: String,
    pub topics: Vec<ContactTopicSubscription>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactTopicSubscription {
    pub id: TopicId,
    pub subscription: SubscriptionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuppressionBody {
    pub id: String,
    pub email: String,
    pub origin: SuppressionOriginType,
    pub source_id: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuppressionOriginType {
    Bounce,
    Complaint,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicBody {
    pub id: TopicId,
    pub name: String,
    pub description: Option<String>,
    pub default_subscription: SubscriptionType,
    pub deleted: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// What made the change in an `inbox.*` event.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InboxEventSource {
    Api,
    Dashboard,
    Agent,
    System,
}

/// An attachment on an email in an `inbox.*` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxEventEmailAttachment {
    /// The ID of the attachment. Not always a UUID. Outbound attachments are identified by their
    /// position in the sent email.
    pub id: String,
    pub filename: Option<String>,
    /// The size of the attachment in bytes.
    pub size: Option<u64>,
}

/// The email carried by an `inbox.*` event. It has no `html` or `text`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxEventEmail {
    pub id: InboxThreadEmailId,
    pub direction: InboxEmailDirection,
    /// The sender, as an address or as `Name <address>`.
    pub from: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub reply_to: Vec<String>,
    pub subject: Option<String>,
    /// The Message-ID header of the email.
    pub message_id: Option<String>,
    pub attachments: Vec<InboxEventEmailAttachment>,
    pub read: bool,
    /// Timestamp when the email arrived or was sent.
    pub received_at: String,
}

/// The draft carried by an `inbox.*` event. It has no `html` or `text`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxEventDraft {
    pub object: String,
    pub id: InboxDraftId,
    #[serde(rename = "type")]
    pub r#type: InboxDraftType,
    pub to: Option<Vec<String>>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: Option<String>,
    /// The ID of the thread when this draft is a reply. `None` otherwise.
    pub thread_id: Option<InboxThreadId>,
    /// The ID of the email this draft replies to. `None` otherwise.
    pub reply_to_email_id: Option<InboxThreadEmailId>,
    /// The ID of the queued outbound email after send. `None` until then.
    pub email_id: Option<EmailId>,
    pub created_at: String,
    pub updated_at: String,
}

/// Data of [`InboxEvent::InboxCreated`] and [`InboxEvent::InboxUpdated`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxBody {
    pub source: InboxEventSource,
    pub inbox_id: InboxId,
    /// The inbox as it is when the webhook is sent. Absent when the inbox no longer exists.
    pub inbox: Option<Inbox>,
}

/// Data of [`InboxEvent::InboxDeleted`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxDeletedBody {
    pub source: InboxEventSource,
    pub inbox_id: InboxId,
}

/// Data of [`InboxEvent::InboxThreadCreated`], [`InboxEvent::InboxEmailReceived`] and
/// [`InboxEvent::InboxEmailSent`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxThreadEmailBody {
    pub source: InboxEventSource,
    pub inbox_id: InboxId,
    pub thread_id: InboxThreadId,
    pub email_id: InboxThreadEmailId,
    /// The thread as it is when the webhook is sent. Absent when the thread no longer exists.
    pub thread: Option<InboxThreadSummary>,
    /// The email as it is when the webhook is sent, without `html` or `text`. Absent when the
    /// email no longer exists.
    pub email: Option<InboxEventEmail>,
}

/// Data of [`InboxEvent::InboxThreadFolderUpdated`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxThreadFolderUpdatedBody {
    /// The folder the thread was in.
    pub from: InboxFolder,
    /// The folder the thread moved to.
    pub to: InboxFolder,
    pub source: InboxEventSource,
    pub inbox_id: InboxId,
    pub thread_id: InboxThreadId,
    /// The thread as it is when the webhook is sent. Absent when the thread no longer exists.
    pub thread: Option<InboxThreadSummary>,
}

/// Data of [`InboxEvent::InboxThreadAssigned`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxThreadAssignedBody {
    /// The email of the member the thread is now assigned to.
    pub assignee_email: Option<String>,
    /// The email of the member who made the assignment. `None` when no member made it.
    pub assigned_by_email: Option<String>,
    /// The email of the member the thread was assigned to before. `None` when it was unassigned.
    pub previous_assignee_email: Option<String>,
    pub source: InboxEventSource,
    pub inbox_id: InboxId,
    pub thread_id: InboxThreadId,
    /// The thread as it is when the webhook is sent. Absent when the thread no longer exists.
    pub thread: Option<InboxThreadSummary>,
}

/// Data of [`InboxEvent::InboxThreadUnassigned`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxThreadUnassignedBody {
    /// The email of the member the thread was assigned to.
    pub previous_assignee_email: Option<String>,
    pub source: InboxEventSource,
    pub inbox_id: InboxId,
    pub thread_id: InboxThreadId,
    /// The thread as it is when the webhook is sent. Absent when the thread no longer exists.
    pub thread: Option<InboxThreadSummary>,
}

/// Data of [`InboxEvent::InboxThreadLabelsUpdated`]. One event carries one label, in either
/// `added` or `removed`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxThreadLabelsUpdatedBody {
    /// The labels applied to the thread.
    pub added: Vec<InboxThreadLabel>,
    /// The labels removed from the thread.
    pub removed: Vec<InboxThreadLabel>,
    pub source: InboxEventSource,
    pub inbox_id: InboxId,
    pub thread_id: InboxThreadId,
    /// The thread as it is when the webhook is sent. Absent when the thread no longer exists.
    pub thread: Option<InboxThreadSummary>,
}

/// Data of [`InboxEvent::InboxDraftCreated`], [`InboxEvent::InboxDraftUpdated`] and
/// [`InboxEvent::InboxDraftDeleted`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxDraftBody {
    pub source: InboxEventSource,
    pub inbox_id: InboxId,
    /// The ID of the thread. Only on reply drafts.
    pub thread_id: Option<InboxThreadId>,
    pub draft_id: InboxDraftId,
    /// The thread as it is when the webhook is sent. Absent when the thread no longer exists.
    /// Only on reply drafts.
    pub thread: Option<InboxThreadSummary>,
    /// The draft as it is when the webhook is sent, without `html` or `text`. Absent when the
    /// draft no longer exists.
    pub draft: Option<InboxEventDraft>,
}

/// Data of [`InboxEvent::InboxDraftSent`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxDraftSentBody {
    pub source: InboxEventSource,
    pub inbox_id: InboxId,
    pub thread_id: InboxThreadId,
    /// The ID of the sent email.
    pub email_id: EmailId,
    pub draft_id: InboxDraftId,
    /// The thread as it is when the webhook is sent. Absent when the thread no longer exists.
    pub thread: Option<InboxThreadSummary>,
    /// The email as it is when the webhook is sent, without `html` or `text`. Only when the email
    /// is already on the thread when the webhook is sent.
    pub email: Option<InboxEventEmail>,
    /// The draft as it is when the webhook is sent, without `html` or `text`. Absent when the
    /// draft no longer exists.
    pub draft: Option<InboxEventDraft>,
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use crate::types::SendEventOptions;
    #[cfg(not(feature = "blocking"))]
    use crate::{
        events::{InboxEventType, try_parse_event_type},
        list_opts::ListOptions,
        test::{CLIENT, DebugResult},
        types::{CreateContactOptions, CreateEventOptions},
    };
    #[cfg(not(feature = "blocking"))]
    use strum::EnumCount;

    use crate::{
        events::{
            ContactEventType, ContactTopicsEventType, DomainEventType, EmailEventType, Event,
            EventType, InboxEvent, InboxEventSource, SuppressionEventType, TopicEventType,
            try_parse_event,
        },
        types::{
            ContactIdOrEmail, InboxDraftType, InboxEmailDirection, InboxFolder, SubscriptionType,
        },
    };

    use serde_json::json;

    #[test]
    fn serialize_send() {
        let opts = SendEventOptions {
            event: "user.created".to_owned(),
            contact_id_or_email: ContactIdOrEmail::ContactId("contact".to_string()),
            payload: json!({
              "plan": "pro"
            }),
        };
        let res = serde_json::to_string(&opts).unwrap();
        println!("{res}");
    }

    #[tokio_shared_rt::test(shared = true)]
    #[serial_test::serial]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> DebugResult<()> {
        use crate::types::UpdateEventOptions;

        let resend = &*CLIENT;

        // Create
        let opts = CreateEventOptions {
            name: "user.created".to_owned(),
            schema: json!({
              "plan": "string"
            }),
        };
        let _event = resend.events.create(opts).await?;

        // Send
        let opts = CreateContactOptions::new("steve.wozniak@gmail.com");
        let contact = resend.contacts.create(opts).await?;
        std::thread::sleep(std::time::Duration::from_secs(2));

        let opts = SendEventOptions {
            event: "user.created".to_owned(),
            contact_id_or_email: ContactIdOrEmail::ContactId(contact.to_string()),
            payload: json!({
              "plan": "pro"
            }),
        };
        let event = resend.events.send(opts).await?;
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Get
        let event = resend.events.get(&event.event).await?;

        // List
        let events = resend.events.list(ListOptions::default()).await?;
        assert!(!events.is_empty());

        // Update
        let opts = UpdateEventOptions {
            schema: json!({
              "plan": "string",
              "trial": "boolean"
            }),
        };
        let event = resend.events.update(&event.id, opts).await?;

        // Delete
        let _deleted = resend.events.delete(&event.id).await?;
        let _deleted = resend.contacts.delete("steve.wozniak@gmail.com").await?;

        Ok(())
    }

    #[test]
    fn email_sent() {
        let data = r#"
    {
      "type": "email.sent",
      "created_at": "2024-02-22T23:41:12.126Z",
      "data": {
        "broadcast_id": "8b146471-e88e-4322-86af-016cd36fd216",
        "created_at": "2024-02-22T23:41:11.894Z",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
        "message_id": "<111-222-333@email.example.com>",
        "from": "Acme <onboarding@resend.dev>",
        "to": ["delivered@resend.dev"],
        "subject": "Sending this example",
        "template_id": "43f68331-0622-4e15-8202-246a0388854b",
        "tags": {
          "category": "confirm_email"
        }
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::EmailEvent(email_event) = parsed {
            assert!(matches!(email_event.r#type, EmailEventType::EmailSent));
            assert_eq!(
                email_event.data.message_id,
                "<111-222-333@email.example.com>"
            );
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn email_delivered() {
        let data = r#"
    {
      "type": "email.delivered",
      "created_at": "2024-02-22T23:41:12.126Z",
      "data": {
        "broadcast_id": "8b146471-e88e-4322-86af-016cd36fd216",
        "created_at": "2024-02-22T23:41:11.894Z",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
        "message_id": "<111-222-333@email.example.com>",
        "from": "Acme <onboarding@resend.dev>",
        "to": ["delivered@resend.dev"],
        "subject": "Sending this example",
        "template_id": "43f68331-0622-4e15-8202-246a0388854b",
        "tags": {
          "category": "confirm_email"
        }
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::EmailEvent(email_event) = parsed {
            assert!(matches!(email_event.r#type, EmailEventType::EmailDelivered));
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn email_delivery_delayed() {
        let data = r#"
    {
      "type": "email.delivery_delayed",
      "created_at": "2024-02-22T23:41:12.126Z",
      "data": {
        "broadcast_id": "8b146471-e88e-4322-86af-016cd36fd216",
        "created_at": "2024-02-22T23:41:11.894Z",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
        "message_id": "<111-222-333@email.example.com>",
        "from": "Acme <onboarding@resend.dev>",
        "to": ["delivered@resend.dev"],
        "subject": "Sending this example",
        "template_id": "43f68331-0622-4e15-8202-246a0388854b",
        "tags": {
          "category": "confirm_email"
        }
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::EmailEvent(email_event) = parsed {
            assert!(matches!(
                email_event.r#type,
                EmailEventType::EmailDeliveryDelayed
            ));
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn email_complained() {
        let data = r#"
    {
      "type": "email.complained",
      "created_at": "2024-02-22T23:41:12.126Z",
      "data": {
        "broadcast_id": "8b146471-e88e-4322-86af-016cd36fd216",
        "created_at": "2024-02-22T23:41:11.894Z",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
        "message_id": "<111-222-333@email.example.com>",
        "from": "Acme <onboarding@resend.dev>",
        "to": ["delivered@resend.dev"],
        "subject": "Sending this example",
        "template_id": "43f68331-0622-4e15-8202-246a0388854b",
        "tags": {
          "category": "confirm_email"
        }
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::EmailEvent(email_event) = parsed {
            assert!(matches!(
                email_event.r#type,
                EmailEventType::EmailComplained
            ));
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn email_bounced() {
        let data = r#"
    {
      "type": "email.bounced",
      "created_at": "2024-11-22T23:41:12.126Z",
      "data": {
        "broadcast_id": "8b146471-e88e-4322-86af-016cd36fd216",
        "created_at": "2024-11-22T23:41:11.894Z",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
        "message_id": "<111-222-333@email.example.com>",
        "from": "Acme <onboarding@resend.dev>",
        "to": ["delivered@resend.dev"],
        "subject": "Sending this example",
        "template_id": "43f68331-0622-4e15-8202-246a0388854b",
        "bounce": {
          "message": "The recipient's email address is on the suppression list because it has a recent history of producing hard bounces.",
          "subType": "Suppressed",
          "type": "Permanent"
        },
        "tags": {
          "category": "confirm_email"
        }
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::EmailEvent(email_event) = parsed {
            assert!(matches!(email_event.r#type, EmailEventType::EmailBounced));
            assert!(email_event.data.bounce.is_some());
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn email_opened() {
        let data = r#"
    {
      "type": "email.opened",
      "created_at": "2024-02-22T23:41:12.126Z",
      "data": {
        "broadcast_id": "8b146471-e88e-4322-86af-016cd36fd216",
        "created_at": "2024-02-22T23:41:11.894Z",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
        "message_id": "<111-222-333@email.example.com>",
        "from": "Acme <onboarding@resend.dev>",
        "to": ["delivered@resend.dev"],
        "subject": "Sending this example",
        "template_id": "43f68331-0622-4e15-8202-246a0388854b",
        "tags": {
          "category": "confirm_email"
        }
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::EmailEvent(email_event) = parsed {
            assert!(matches!(email_event.r#type, EmailEventType::EmailOpened));
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn email_clicked() {
        let data = r#"
    {
      "type": "email.clicked",
      "created_at": "2024-11-22T23:41:12.126Z",
      "data": {
        "broadcast_id": "8b146471-e88e-4322-86af-016cd36fd216",
        "created_at": "2024-11-22T23:41:11.894Z",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
        "message_id": "<111-222-333@email.example.com>",
        "from": "Acme <onboarding@resend.dev>",
        "to": ["delivered@resend.dev"],
        "click": {
          "ipAddress": "122.115.53.11",
          "link": "https://resend.com",
          "timestamp": "2024-11-24T05:00:57.163Z",
          "userAgent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.1 Safari/605.1.15"
        },
        "subject": "Sending this example",
        "template_id": "43f68331-0622-4e15-8202-246a0388854b",
        "tags": {
          "category": "confirm_email"
        }
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::EmailEvent(email_event) = parsed {
            assert!(matches!(email_event.r#type, EmailEventType::EmailClicked));
            assert!(email_event.data.click.is_some());
            assert!(!email_event.data.tags.is_empty());
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn email_failed() {
        let data = r#"
    {
      "type": "email.failed",
      "created_at": "2024-11-22T23:41:12.126Z",
      "data": {
        "broadcast_id": "8b146471-e88e-4322-86af-016cd36fd216",
        "created_at": "2024-11-22T23:41:11.894Z",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
        "message_id": "<111-222-333@email.example.com>",
        "from": "Acme <onboarding@resend.dev>",
        "to": ["delivered@resend.dev"],
        "subject": "Sending this example",
        "template_id": "43f68331-0622-4e15-8202-246a0388854b",
        "failed": {
          "reason": "reached_daily_quota"
        },
        "tags": {
          "category": "confirm_email"
        }
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::EmailEvent(email_event) = parsed {
            assert!(matches!(email_event.r#type, EmailEventType::EmailFailed));
            assert!(email_event.data.failed.is_some());
            assert!(!email_event.data.tags.is_empty());
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn email_received() {
        let data = r#"
    {
      "type": "email.received",
      "created_at": "2024-02-22T23:41:12.126Z",
      "data": {
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
        "created_at": "2024-02-22T23:41:11.894Z",
        "from": "Acme <onboarding@resend.dev>",
        "to": ["delivered@resend.dev"],
        "bcc": [],
        "cc": [],
        "received_for": ["forwarded@example.com"],
        "message_id": "<example+123>",
        "subject": "Sending this example",
        "attachments": [
          {
            "id": "2a0c9ce0-3112-4728-976e-47ddcd16a318",
            "filename": "avatar.png",
            "content_type": "image/png",
            "content_disposition": "inline",
            "content_id": "img001"
          }
        ]
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::EmailEvent(email_event) = parsed {
            assert!(matches!(email_event.r#type, EmailEventType::EmailReceived));
            assert_eq!(email_event.data.message_id, "<example+123>");
            assert!(email_event.data.received.is_some());
            assert!(email_event.data.tags.is_empty());

            let received = email_event.data.received.unwrap();
            assert_eq!(received.attachments.len(), 1);
            assert_eq!(received.cc, Vec::<String>::new());
            assert_eq!(received.bcc, Vec::<String>::new());
            assert_eq!(received.received_for, vec!["forwarded@example.com"]);
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn email_scheduled() {
        let data = r#"
    {
      "type": "email.scheduled",
      "created_at": "2024-02-22T23:41:12.126Z",
      "data": {
        "broadcast_id": "8b146471-e88e-4322-86af-016cd36fd216",
        "created_at": "2024-02-22T23:41:11.894Z",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
        "message_id": "<111-222-333@email.example.com>",
        "from": "Acme <onboarding@resend.dev>",
        "to": ["delivered@resend.dev"],
        "subject": "Sending this example",
        "template_id": "43f68331-0622-4e15-8202-246a0388854b",
        "tags": {
          "category": "confirm_email"
        }
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::EmailEvent(email_event) = parsed {
            assert!(matches!(email_event.r#type, EmailEventType::EmailScheduled));
            assert!(email_event.data.received.is_none());
            assert!(!email_event.data.tags.is_empty());
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn email_suppressed() {
        let data = r#"
    {
      "type": "email.suppressed",
      "created_at": "2024-11-22T23:41:12.126Z",
      "data": {
        "broadcast_id": "8b146471-e88e-4322-86af-016cd36fd216",
        "created_at": "2024-11-22T23:41:11.894Z",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
        "message_id": "<111-222-333@email.example.com>",
        "from": "Acme <onboarding@resend.dev>",
        "to": ["delivered@resend.dev"],
        "subject": "Sending this example",
        "template_id": "43f68331-0622-4e15-8202-246a0388854b",
        "suppressed": {
          "message": "Resend has suppressed sending to this address because it is on the account-level suppression list. This does not count toward your bounce rate metric",
          "type": "OnAccountSuppressionList"
        },
        "tags": {
          "category": "confirm_email"
        }
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::EmailEvent(email_event) = parsed {
            assert!(matches!(
                email_event.r#type,
                EmailEventType::EmailSuppressed
            ));
            assert!(email_event.data.received.is_none());
            assert!(!email_event.data.tags.is_empty());
            assert!(email_event.data.suppressed.is_some());
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn contact_created() {
        let data = r#"
    {
      "type": "contact.created",
      "created_at": "2024-11-17T19:32:22.980Z",
      "data": {
        "id": "e169aa45-1ecf-4183-9955-b1499d5701d3",
        "audience_id": "78261eea-8f8b-4381-83c6-79fa7120f1cf",
        "segment_ids": ["78261eea-8f8b-4381-83c6-79fa7120f1cf"],
        "created_at": "2024-11-17T19:32:22.980Z",
        "updated_at": "2024-11-17T19:32:22.980Z",
        "email": "steve.wozniak@gmail.com",
        "first_name": "Steve",
        "last_name": "Wozniak",
        "unsubscribed": false
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::ContactEvent(contact_event) = parsed {
            assert!(matches!(
                contact_event.r#type,
                ContactEventType::ContactCreated
            ));
            assert_eq!(contact_event.data.segment_ids.len(), 1);
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn contact_updated() {
        let data = r#"
    {
      "type": "contact.updated",
      "created_at": "2024-10-11T23:47:56.678Z",
      "data": {
        "id": "e169aa45-1ecf-4183-9955-b1499d5701d3",
        "audience_id": "78261eea-8f8b-4381-83c6-79fa7120f1cf",
        "segment_ids": ["78261eea-8f8b-4381-83c6-79fa7120f1cf"],
        "created_at": "2024-10-10T15:11:54.110Z",
        "updated_at": "2024-10-11T23:47:56.678Z",
        "email": "steve.wozniak@gmail.com",
        "first_name": "Steve",
        "last_name": "Wozniak",
        "unsubscribed": false
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::ContactEvent(contact_event) = parsed {
            assert!(matches!(
                contact_event.r#type,
                ContactEventType::ContactUpdated
            ));
            assert_eq!(contact_event.data.segment_ids.len(), 1);
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn contact_deleted() {
        let data = r#"
    {
      "type": "contact.deleted",
      "created_at": "2024-11-17T19:32:22.980Z",
      "data": {
        "id": "e169aa45-1ecf-4183-9955-b1499d5701d3",
        "audience_id": "78261eea-8f8b-4381-83c6-79fa7120f1cf",
        "segment_ids": ["78261eea-8f8b-4381-83c6-79fa7120f1cf"],
        "created_at": "2024-11-10T15:11:54.110Z",
        "updated_at": "2024-11-17T19:32:22.980Z",
        "email": "steve.wozniak@gmail.com",
        "first_name": "Steve",
        "last_name": "Wozniak",
        "unsubscribed": false
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::ContactEvent(contact_event) = parsed {
            assert!(matches!(
                contact_event.r#type,
                ContactEventType::ContactDeleted
            ));
            assert_eq!(contact_event.data.segment_ids.len(), 1);
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn contact_topics_updated() {
        let data = r#"{
          "type": "contact.topics.updated",
          "created_at": "2026-02-12T10:00:00.000Z",
          "data": {
            "email": "steve.wozniak@gmail.com",
            "topics": [
              {
                "id": "b6d24b8e-af0b-4c3c-be0c-359bbd97381e",
                "subscription": "opt_in"
              },
              {
                "id": "07d84122-7224-4881-9c31-1c048e204602",
                "subscription": "opt_out"
              }
            ]
          }
        }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::ContactTopicsEvent(contact_topics_event) = parsed {
            assert!(matches!(
                contact_topics_event.r#type,
                ContactTopicsEventType::ContactTopicsUpdated
            ));
            assert_eq!(contact_topics_event.data.email, "steve.wozniak@gmail.com");
            let subscriptions = contact_topics_event
                .data
                .topics
                .iter()
                .map(|topic| topic.subscription)
                .collect::<Vec<_>>();
            assert_eq!(
                subscriptions,
                vec![SubscriptionType::OptIn, SubscriptionType::OptOut]
            );
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn domain_created() {
        let data = r#"{
          "type": "domain.created",
          "created_at": "2026-11-17T19:32:22.980Z",
          "data": {
            "id": "d91cd9bd-1176-453e-8fc1-35364d380206",
            "name": "example.com",
            "status": "not_started",
            "created_at": "2026-04-26T20:21:26.347Z",
            "region": "us-east-1",
            "capabilities": {
              "sending": "enabled",
              "receiving": "disabled"
            },
            "records": [
              {
                "record": "SPF",
                "name": "send",
                "type": "MX",
                "ttl": "Auto",
                "status": "not_started",
                "value": "feedback-smtp.us-east-1.amazonses.com",
                "priority": 10
              },
              {
                "record": "SPF",
                "name": "send",
                "value": "\"v=spf1 include:amazonses.com ~all\"",
                "type": "TXT",
                "ttl": "Auto",
                "status": "not_started"
              },
              {
                "record": "DKIM",
                "name": "resend._domainkey",
                "value": "p=MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDsc4Lh8xilsngyKEgN2S84+21gn+x6SEXtjWvPiAAmnmggr5FWG42WnqczpzQ/mNblqHz4CDwUum6LtY6SdoOlDmrhvp5khA3cd661W9FlK3yp7+jVACQElS7d9O6jv8VsBbVg4COess3gyLE5RyxqF1vYsrEXqyM8TBz1n5AGkQIDAQA2",
                "type": "TXT",
                "status": "not_started",
                "ttl": "Auto"
              }
            ]
          }
        }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::DomainEvent(domain_event) = parsed {
            assert!(matches!(
                domain_event.r#type,
                DomainEventType::DomainCreated
            ));
            assert!(domain_event.data.records.is_some_and(|r| r.len() == 3));
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn domain_updated() {
        let data = r#"{
          "type": "domain.updated",
          "created_at": "2026-11-17T19:32:22.980Z",
          "data": {
            "id": "d91cd9bd-1176-453e-8fc1-35364d380206",
            "name": "example.com",
            "status": "not_started",
            "created_at": "2026-04-26T20:21:26.347Z",
            "region": "us-east-1",
            "capabilities": {
              "sending": "enabled",
              "receiving": "enabled"
            },
            "records": [
              {
                "record": "SPF",
                "name": "send",
                "type": "MX",
                "ttl": "Auto",
                "status": "not_started",
                "value": "feedback-smtp.us-east-1.amazonses.com",
                "priority": 10
              },
              {
                "record": "SPF",
                "name": "send",
                "value": "\"v=spf1 include:amazonses.com ~all\"",
                "type": "TXT",
                "ttl": "Auto",
                "status": "not_started"
              },
              {
                "record": "DKIM",
                "name": "resend._domainkey",
                "value": "p=MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDsc4Lh8xilsngyKEgN2S84+21gn+x6SEXtjWvPiAAmnmggr5FWG42WnqczpzQ/mNblqHz4CDwUum6LtY6SdoOlDmrhvp5khA3cd661W9FlK3yp7+jVACQElS7d9O6jv8VsBbVg4COess3gyLE5RyxqF1vYsrEXqyM8TBz1n5AGkQIDAQA2",
                "type": "TXT",
                "status": "not_started",
                "ttl": "Auto"
              },
              {
                "name": "inbound.yourdomain.tld",
                "priority": 10,
                "record": "Receiving MX",
                "status": "pending",
                "ttl": "Auto",
                "type": "MX",
                "value": "inbound-smtp.us-east-1.amazonaws.com"
              }
            ]
          }
        }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::DomainEvent(domain_event) = parsed {
            assert!(matches!(
                domain_event.r#type,
                DomainEventType::DomainUpdated
            ));
            assert!(domain_event.data.records.is_some_and(|r| r.len() == 4));
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn domain_deleted() {
        let data = r#"{
          "type": "domain.deleted",
          "created_at": "2026-11-17T19:32:22.980Z",
          "data": {
            "id": "d91cd9bd-1176-453e-8fc1-35364d380206",
            "name": "example.com",
            "status": "not_started",
            "created_at": "2026-04-26T20:21:26.347Z",
            "region": "us-east-1",
            "capabilities": {
              "sending": "enabled",
              "receiving": "disabled"
            },
            "records": [
              {
                "record": "SPF",
                "name": "send",
                "type": "MX",
                "ttl": "Auto",
                "status": "not_started",
                "value": "feedback-smtp.us-east-1.amazonses.com",
                "priority": 10
              },
              {
                "record": "SPF",
                "name": "send",
                "value": "\"v=spf1 include:amazonses.com ~all\"",
                "type": "TXT",
                "ttl": "Auto",
                "status": "not_started"
              },
              {
                "record": "DKIM",
                "name": "resend._domainkey",
                "value": "p=MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDsc4Lh8xilsngyKEgN2S84+21gn+x6SEXtjWvPiAAmnmggr5FWG42WnqczpzQ/mNblqHz4CDwUum6LtY6SdoOlDmrhvp5khA3cd661W9FlK3yp7+jVACQElS7d9O6jv8VsBbVg4COess3gyLE5RyxqF1vYsrEXqyM8TBz1n5AGkQIDAQA2",
                "type": "TXT",
                "status": "not_started",
                "ttl": "Auto"
              }
            ]
          }
        }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::DomainEvent(domain_event) = parsed {
            assert!(matches!(
                domain_event.r#type,
                DomainEventType::DomainDeleted
            ));
            assert!(domain_event.data.records.is_some_and(|r| r.len() == 3));
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn suppression_added() {
        let data = r#"{
          "type": "suppression.added",
          "created_at": "2026-11-17T19:32:22.980Z",
          "data": {
            "id": "e169aa45-1ecf-4183-9955-b1499d5701d3",
            "email": "steve.wozniak@gmail.com",
            "origin": "bounce",
            "source_id": "4ef9a417-02e9-4d39-ad75-9611e0fcc33c",
            "created_at": "2026-11-17T19:32:22.980Z"
          }
        }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::SuppressionEvent(contact_event) = parsed {
            assert!(matches!(
                contact_event.r#type,
                SuppressionEventType::SuppressionAdded
            ));
            assert!(contact_event.data.source_id.is_some());
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn suppression_removed() {
        let data = r#"{
          "type": "suppression.removed",
          "created_at": "2026-11-17T19:32:22.980Z",
          "data": {
            "id": "e169aa45-1ecf-4183-9955-b1499d5701d3",
            "email": "steve.wozniak@gmail.com",
            "origin": "manual",
            "source_id": null,
            "created_at": "2026-11-15T08:12:45.120Z"
          }
        }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::SuppressionEvent(contact_event) = parsed {
            assert!(matches!(
                contact_event.r#type,
                SuppressionEventType::SuppressionRemoved
            ));
            assert!(contact_event.data.source_id.is_none());
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn topic_created() {
        let data = r#"{
          "type": "topic.created",
          "created_at": "2026-02-12T10:00:00.000Z",
          "data": {
            "id": "b6d24b8e-af0b-4c3c-be0c-359bbd97381e",
            "name": "Product Updates",
            "description": "New features and improvements",
            "default_subscription": "opt_in",
            "deleted": false,
            "created_at": "2026-02-12T10:00:00.000Z",
            "updated_at": "2026-02-12T10:00:00.000Z"
          }
        }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::TopicEvent(topic_event) = parsed {
            assert!(matches!(topic_event.r#type, TopicEventType::TopicCreated));
            assert_eq!(
                topic_event.data.default_subscription,
                SubscriptionType::OptIn
            );
            assert!(topic_event.data.description.is_some());
            assert!(!topic_event.data.deleted);
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn topic_updated() {
        let data = r#"{
          "type": "topic.updated",
          "created_at": "2026-02-12T10:00:00.000Z",
          "data": {
            "id": "b6d24b8e-af0b-4c3c-be0c-359bbd97381e",
            "name": "Product Updates",
            "description": null,
            "default_subscription": "opt_out",
            "deleted": false,
            "created_at": "2026-02-12T10:00:00.000Z",
            "updated_at": "2026-02-12T10:00:00.000Z"
          }
        }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::TopicEvent(topic_event) = parsed {
            assert!(matches!(topic_event.r#type, TopicEventType::TopicUpdated));
            assert_eq!(
                topic_event.data.default_subscription,
                SubscriptionType::OptOut
            );
            assert!(topic_event.data.description.is_none());
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn topic_deleted() {
        let data = r#"{
          "type": "topic.deleted",
          "created_at": "2026-02-12T10:00:00.000Z",
          "data": {
            "id": "b6d24b8e-af0b-4c3c-be0c-359bbd97381e",
            "name": "Product Updates",
            "description": "New features and improvements",
            "default_subscription": "opt_in",
            "deleted": true,
            "created_at": "2026-02-12T10:00:00.000Z",
            "updated_at": "2026-02-12T10:00:00.000Z"
          }
        }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::TopicEvent(topic_event) = parsed {
            assert!(matches!(topic_event.r#type, TopicEventType::TopicDeleted));
            assert!(topic_event.data.deleted);
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    fn inbox_event_types() {
        for event_type in [
            "inbox.created",
            "inbox.updated",
            "inbox.deleted",
            "inbox.thread.created",
            "inbox.thread.folder.updated",
            "inbox.thread.assigned",
            "inbox.thread.unassigned",
            "inbox.thread.labels.updated",
            "inbox.email.received",
            "inbox.email.sent",
            "inbox.draft.created",
            "inbox.draft.updated",
            "inbox.draft.sent",
            "inbox.draft.deleted",
        ] {
            let parsed = serde_json::from_value::<EventType>(json!(event_type)).unwrap();
            assert!(matches!(parsed, EventType::InboxEventType(_)));
            assert_eq!(serde_json::to_value(parsed).unwrap(), json!(event_type));
        }
    }

    fn parse_inbox_event(data: &str) -> InboxEvent {
        match try_parse_event(data).unwrap() {
            Event::InboxEvent(event) => event,
            other => panic!("Wrong parsing: {other:?}"),
        }
    }

    #[test]
    fn inbox_created() {
        let data = r#"{
          "type": "inbox.created",
          "created_at": "2026-09-29T12:00:00.000Z",
          "data": {
            "source": "api",
            "inbox_id": "b3e2b2b6-3f0e-4c8e-9ad3-2f43a1e2c7f1",
            "inbox": {
              "object": "inbox",
              "id": "b3e2b2b6-3f0e-4c8e-9ad3-2f43a1e2c7f1",
              "name": "Customer Support",
              "email_address": "support@example.com",
              "domain_id": "d91cd9bd-1176-453e-8fc1-35364d380206",
              "receiving_address": null,
              "from_name": "Ada from Support",
              "unread": 0,
              "drafts": 0,
              "last_received": null,
              "created_at": "2026-09-29T12:00:00.000Z"
            }
          }
        }"#;

        match parse_inbox_event(data) {
            InboxEvent::InboxCreated { data, .. } => {
                assert_eq!(data.source, InboxEventSource::Api);
                assert!(data.inbox.is_some());
            }
            other => panic!("Wrong parsing: {other:?}"),
        }
    }

    #[test]
    fn inbox_updated() {
        let data = r#"{
          "type": "inbox.updated",
          "created_at": "2026-09-29T12:00:00.000Z",
          "data": {
            "source": "dashboard",
            "inbox_id": "b3e2b2b6-3f0e-4c8e-9ad3-2f43a1e2c7f1",
            "inbox": {
              "object": "inbox",
              "id": "b3e2b2b6-3f0e-4c8e-9ad3-2f43a1e2c7f1",
              "name": "Customer Support",
              "email_address": "support@example.com",
              "domain_id": "d91cd9bd-1176-453e-8fc1-35364d380206",
              "receiving_address": null,
              "from_name": "Ada from Support",
              "unread": 3,
              "drafts": 2,
              "last_received": "2026-09-29T11:42:08.000Z",
              "created_at": "2026-09-01T09:30:00.000Z"
            }
          }
        }"#;

        match parse_inbox_event(data) {
            InboxEvent::InboxUpdated { data, .. } => assert!(data.inbox.is_some()),
            other => panic!("Wrong parsing: {other:?}"),
        }
    }

    #[test]
    fn inbox_deleted() {
        let data = r#"{
          "type": "inbox.deleted",
          "created_at": "2026-09-29T12:00:00.000Z",
          "data": {
            "source": "api",
            "inbox_id": "b3e2b2b6-3f0e-4c8e-9ad3-2f43a1e2c7f1"
          }
        }"#;

        match parse_inbox_event(data) {
            InboxEvent::InboxDeleted { data, .. } => {
                assert_eq!(
                    data.inbox_id.as_ref(),
                    "b3e2b2b6-3f0e-4c8e-9ad3-2f43a1e2c7f1"
                );
            }
            other => panic!("Wrong parsing: {other:?}"),
        }
    }

    #[test]
    fn inbox_thread_created() {
        let data = r#"{
          "type": "inbox.thread.created",
          "created_at": "2026-09-29T12:00:00.000Z",
          "data": {
            "source": "system",
            "inbox_id": "b3e2b2b6-3f0e-4c8e-9ad3-2f43a1e2c7f1",
            "thread_id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
            "email_id": "4ef9a417-02e9-4d39-ad75-9611e0fcc33c",
            "thread": {
              "object": "inbox_thread",
              "id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
              "subject": "Question about my invoice",
              "folder": "inbox",
              "labels": [],
              "read": false
            },
            "email": {
              "id": "4ef9a417-02e9-4d39-ad75-9611e0fcc33c",
              "direction": "inbound",
              "from": "Steve Wozniak <steve.wozniak@gmail.com>",
              "to": ["support@example.com"],
              "cc": [],
              "bcc": [],
              "reply_to": [],
              "subject": "Question about my invoice",
              "message_id": "<CAF7c1f0a2e5d3b@mail.gmail.com>",
              "attachments": [
                {
                  "id": "2a0c9ce0-3112-4728-976e-47ddcd16a318",
                  "filename": "invoice.pdf",
                  "size": 48213
                }
              ],
              "read": false,
              "received_at": "2026-09-29T12:00:00.000Z"
            }
          }
        }"#;

        match parse_inbox_event(data) {
            InboxEvent::InboxThreadCreated { data, .. } => {
                assert_eq!(data.email.unwrap().attachments.len(), 1);
            }
            other => panic!("Wrong parsing: {other:?}"),
        }
    }

    #[test]
    fn inbox_thread_folder_updated() {
        let data = r#"{
          "type": "inbox.thread.folder.updated",
          "created_at": "2026-09-29T12:00:00.000Z",
          "data": {
            "from": "inbox",
            "to": "archive",
            "source": "dashboard",
            "inbox_id": "b3e2b2b6-3f0e-4c8e-9ad3-2f43a1e2c7f1",
            "thread_id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
            "thread": {
              "object": "inbox_thread",
              "id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
              "subject": "Question about my invoice",
              "folder": "archive",
              "labels": [],
              "read": true
            }
          }
        }"#;

        match parse_inbox_event(data) {
            InboxEvent::InboxThreadFolderUpdated { data, .. } => {
                assert_eq!(data.from, InboxFolder::Inbox);
                assert_eq!(data.to, InboxFolder::Archive);
            }
            other => panic!("Wrong parsing: {other:?}"),
        }
    }

    #[test]
    fn inbox_thread_assigned() {
        let data = r#"{
          "type": "inbox.thread.assigned",
          "created_at": "2026-09-29T12:00:00.000Z",
          "data": {
            "assignee_email": "ada@example.com",
            "assigned_by_email": "grace@example.com",
            "previous_assignee_email": null,
            "source": "dashboard",
            "inbox_id": "b3e2b2b6-3f0e-4c8e-9ad3-2f43a1e2c7f1",
            "thread_id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
            "thread": {
              "object": "inbox_thread",
              "id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
              "subject": "Question about my invoice",
              "folder": "inbox",
              "labels": [],
              "read": false
            }
          }
        }"#;

        match parse_inbox_event(data) {
            InboxEvent::InboxThreadAssigned { data, .. } => {
                assert!(data.previous_assignee_email.is_none());
            }
            other => panic!("Wrong parsing: {other:?}"),
        }
    }

    #[test]
    fn inbox_thread_unassigned() {
        let data = r#"{
          "type": "inbox.thread.unassigned",
          "created_at": "2026-09-29T12:00:00.000Z",
          "data": {
            "previous_assignee_email": "ada@example.com",
            "source": "dashboard",
            "inbox_id": "b3e2b2b6-3f0e-4c8e-9ad3-2f43a1e2c7f1",
            "thread_id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
            "thread": {
              "object": "inbox_thread",
              "id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
              "subject": "Question about my invoice",
              "folder": "inbox",
              "labels": [],
              "read": false
            }
          }
        }"#;

        match parse_inbox_event(data) {
            InboxEvent::InboxThreadUnassigned { data, .. } => {
                assert!(data.previous_assignee_email.is_some());
            }
            other => panic!("Wrong parsing: {other:?}"),
        }
    }

    #[test]
    fn inbox_thread_labels_updated() {
        let data = r##"{
          "type": "inbox.thread.labels.updated",
          "created_at": "2026-09-29T12:00:00.000Z",
          "data": {
            "added": [
              {
                "id": "c0a8012e-3b4f-4d7a-9e21-5f6a7b8c9d0e",
                "name": "Billing",
                "color": "#F76B15"
              }
            ],
            "removed": [],
            "source": "dashboard",
            "inbox_id": "b3e2b2b6-3f0e-4c8e-9ad3-2f43a1e2c7f1",
            "thread_id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
            "thread": {
              "object": "inbox_thread",
              "id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
              "subject": "Question about my invoice",
              "folder": "inbox",
              "labels": [
                {
                  "id": "c0a8012e-3b4f-4d7a-9e21-5f6a7b8c9d0e",
                  "name": "Billing",
                  "color": "#F76B15"
                }
              ],
              "read": false
            }
          }
        }"##;

        match parse_inbox_event(data) {
            InboxEvent::InboxThreadLabelsUpdated { data, .. } => {
                assert_eq!(data.added.len(), 1);
                assert!(data.removed.is_empty());
            }
            other => panic!("Wrong parsing: {other:?}"),
        }
    }

    #[test]
    fn inbox_email_received() {
        let data = r#"{
          "type": "inbox.email.received",
          "created_at": "2026-09-29T12:00:00.000Z",
          "data": {
            "source": "system",
            "inbox_id": "b3e2b2b6-3f0e-4c8e-9ad3-2f43a1e2c7f1",
            "thread_id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
            "email_id": "4ef9a417-02e9-4d39-ad75-9611e0fcc33c",
            "thread": {
              "object": "inbox_thread",
              "id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
              "subject": "Question about my invoice",
              "folder": "inbox",
              "labels": [],
              "read": false
            },
            "email": {
              "id": "4ef9a417-02e9-4d39-ad75-9611e0fcc33c",
              "direction": "inbound",
              "from": "Steve Wozniak <steve.wozniak@gmail.com>",
              "to": ["support@example.com"],
              "cc": [],
              "bcc": [],
              "reply_to": [],
              "subject": "Question about my invoice",
              "message_id": "<CAF7c1f0a2e5d3b@mail.gmail.com>",
              "attachments": [
                {
                  "id": "2a0c9ce0-3112-4728-976e-47ddcd16a318",
                  "filename": "invoice.pdf",
                  "size": 48213
                }
              ],
              "read": false,
              "received_at": "2026-09-29T12:00:00.000Z"
            }
          }
        }"#;

        match parse_inbox_event(data) {
            InboxEvent::InboxEmailReceived { data, .. } => {
                assert_eq!(data.email.unwrap().direction, InboxEmailDirection::Inbound);
            }
            other => panic!("Wrong parsing: {other:?}"),
        }
    }

    #[test]
    fn inbox_email_sent() {
        let data = r#"{
          "type": "inbox.email.sent",
          "created_at": "2026-09-29T12:00:00.000Z",
          "data": {
            "source": "system",
            "inbox_id": "b3e2b2b6-3f0e-4c8e-9ad3-2f43a1e2c7f1",
            "thread_id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
            "email_id": "4ef9a417-02e9-4d39-ad75-9611e0fcc33c",
            "thread": {
              "object": "inbox_thread",
              "id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
              "subject": "Question about my invoice",
              "folder": "inbox",
              "labels": [],
              "read": true
            },
            "email": {
              "id": "4ef9a417-02e9-4d39-ad75-9611e0fcc33c",
              "direction": "outbound",
              "from": "Ada from Support <support@example.com>",
              "to": ["steve.wozniak@gmail.com"],
              "cc": [],
              "bcc": [],
              "reply_to": [],
              "subject": "Re: Question about my invoice",
              "message_id": "<0100019a4ef9a417@email.amazonses.com>",
              "attachments": [],
              "read": true,
              "received_at": "2026-09-29T12:00:00.000Z"
            }
          }
        }"#;

        match parse_inbox_event(data) {
            InboxEvent::InboxEmailSent { data, .. } => {
                assert_eq!(data.email.unwrap().direction, InboxEmailDirection::Outbound);
            }
            other => panic!("Wrong parsing: {other:?}"),
        }
    }

    #[test]
    fn inbox_draft_created() {
        let data = r#"{
          "type": "inbox.draft.created",
          "created_at": "2026-09-29T12:00:00.000Z",
          "data": {
            "source": "api",
            "inbox_id": "b3e2b2b6-3f0e-4c8e-9ad3-2f43a1e2c7f1",
            "thread_id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
            "draft_id": "9a2b7c4d-1e3f-4a5b-8c6d-0e1f2a3b4c5d",
            "thread": {
              "object": "inbox_thread",
              "id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
              "subject": "Question about my invoice",
              "folder": "inbox",
              "labels": [],
              "read": false
            },
            "draft": {
              "object": "inbox_draft",
              "id": "9a2b7c4d-1e3f-4a5b-8c6d-0e1f2a3b4c5d",
              "type": "reply",
              "to": ["steve.wozniak@gmail.com"],
              "cc": [],
              "bcc": [],
              "subject": "Re: Question about my invoice",
              "thread_id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
              "reply_to_email_id": "4ef9a417-02e9-4d39-ad75-9611e0fcc33c",
              "email_id": null,
              "created_at": "2026-09-29T11:58:00.000Z",
              "updated_at": "2026-09-29T12:00:00.000Z"
            }
          }
        }"#;

        match parse_inbox_event(data) {
            InboxEvent::InboxDraftCreated { data, .. } => {
                assert_eq!(data.draft.unwrap().r#type, InboxDraftType::Reply);
            }
            other => panic!("Wrong parsing: {other:?}"),
        }
    }

    #[test]
    fn inbox_draft_updated() {
        let data = r#"{
          "type": "inbox.draft.updated",
          "created_at": "2026-09-29T12:00:00.000Z",
          "data": {
            "source": "agent",
            "inbox_id": "b3e2b2b6-3f0e-4c8e-9ad3-2f43a1e2c7f1",
            "thread_id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
            "draft_id": "9a2b7c4d-1e3f-4a5b-8c6d-0e1f2a3b4c5d",
            "thread": {
              "object": "inbox_thread",
              "id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
              "subject": "Question about my invoice",
              "folder": "inbox",
              "labels": [],
              "read": false
            },
            "draft": {
              "object": "inbox_draft",
              "id": "9a2b7c4d-1e3f-4a5b-8c6d-0e1f2a3b4c5d",
              "type": "reply",
              "to": ["steve.wozniak@gmail.com"],
              "cc": [],
              "bcc": [],
              "subject": "Re: Question about my invoice",
              "thread_id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
              "reply_to_email_id": "4ef9a417-02e9-4d39-ad75-9611e0fcc33c",
              "email_id": null,
              "created_at": "2026-09-29T11:58:00.000Z",
              "updated_at": "2026-09-29T12:00:00.000Z"
            }
          }
        }"#;

        match parse_inbox_event(data) {
            InboxEvent::InboxDraftUpdated { data, .. } => {
                assert_eq!(data.source, InboxEventSource::Agent);
            }
            other => panic!("Wrong parsing: {other:?}"),
        }
    }

    #[test]
    fn inbox_draft_sent() {
        let data = r#"{
          "type": "inbox.draft.sent",
          "created_at": "2026-09-29T12:00:00.000Z",
          "data": {
            "source": "api",
            "inbox_id": "b3e2b2b6-3f0e-4c8e-9ad3-2f43a1e2c7f1",
            "thread_id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
            "email_id": "81ce9b8d-1b0a-4d02-a4d9-2a91f5ab6397",
            "draft_id": "9a2b7c4d-1e3f-4a5b-8c6d-0e1f2a3b4c5d",
            "thread": {
              "object": "inbox_thread",
              "id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
              "subject": "Question about my invoice",
              "folder": "inbox",
              "labels": [],
              "read": true
            },
            "draft": {
              "object": "inbox_draft",
              "id": "9a2b7c4d-1e3f-4a5b-8c6d-0e1f2a3b4c5d",
              "type": "reply",
              "to": ["steve.wozniak@gmail.com"],
              "cc": [],
              "bcc": [],
              "subject": "Re: Question about my invoice",
              "thread_id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
              "reply_to_email_id": "4ef9a417-02e9-4d39-ad75-9611e0fcc33c",
              "email_id": "81ce9b8d-1b0a-4d02-a4d9-2a91f5ab6397",
              "created_at": "2026-09-29T11:58:00.000Z",
              "updated_at": "2026-09-29T12:00:00.000Z"
            }
          }
        }"#;

        match parse_inbox_event(data) {
            InboxEvent::InboxDraftSent { data, .. } => {
                assert!(data.email.is_none());
                assert!(data.draft.unwrap().email_id.is_some());
            }
            other => panic!("Wrong parsing: {other:?}"),
        }
    }

    #[test]
    fn inbox_draft_deleted() {
        let data = r#"{
          "type": "inbox.draft.deleted",
          "created_at": "2026-09-29T12:00:00.000Z",
          "data": {
            "source": "dashboard",
            "inbox_id": "b3e2b2b6-3f0e-4c8e-9ad3-2f43a1e2c7f1",
            "thread_id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
            "draft_id": "9a2b7c4d-1e3f-4a5b-8c6d-0e1f2a3b4c5d",
            "thread": {
              "object": "inbox_thread",
              "id": "7c1f0a2e-5d3b-4e8a-9f61-2b8d4c6e1a90",
              "subject": "Question about my invoice",
              "folder": "inbox",
              "labels": [],
              "read": false
            }
          }
        }"#;

        match parse_inbox_event(data) {
            InboxEvent::InboxDraftDeleted { data, .. } => assert!(data.draft.is_none()),
            other => panic!("Wrong parsing: {other:?}"),
        }
    }

    /// Similar to the test in `error.rs`
    #[allow(clippy::unwrap_used)]
    #[tokio_shared_rt::test(shared = true)]
    #[serial_test::serial]
    #[cfg(not(feature = "blocking"))]
    async fn events_up_to_date() -> DebugResult<()> {
        let response = reqwest::get("https://resend.com/docs/dashboard/webhooks/event-types")
            .await
            .unwrap();

        let html = response.text().await.unwrap();

        let fragment = scraper::Html::parse_document(&html);
        let selector =
            scraper::Selector::parse("#content > div > div > div > span > a > code").unwrap();

        let expected = EmailEventType::COUNT
            + ContactEventType::COUNT
            + ContactTopicsEventType::COUNT
            + DomainEventType::COUNT
            + SuppressionEventType::COUNT
            + TopicEventType::COUNT
            + InboxEventType::COUNT;
        let actual = fragment
            .select(&selector)
            .map(|el| el.inner_html())
            .collect::<Vec<_>>();

        for el in &actual {
            // Add quotes around it
            let parsed = try_parse_event_type(&format!("\"{el}\""));
            assert!(parsed.is_ok(), "Could not parse: {el}");
        }

        assert_eq!(expected, actual.len());

        Ok(())
    }
}
