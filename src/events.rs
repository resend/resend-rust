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
        GetEventResponse, InboundAttachment, SegmentId, SendEventOptions, SendEventResponse,
        TemplateId, UpdateEventOptions, UpdateEventResponse,
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
    #[allow(clippy::needless_pass_by_value)]
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
    #[allow(clippy::needless_pass_by_value)]
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
    #[allow(clippy::needless_pass_by_value)]
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
    #[allow(clippy::needless_pass_by_value)]
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
///       "created_at": "2024-02-22T23:41:11.894719+00:00",
///       "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
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
    DomainEvent(DomainEvent),
}

/// Represents any [Resend Event Type](https://resend.com/docs/dashboard/webhooks/event-types).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventType {
    EmailEventType(EmailEventType),
    ContactEventType(ContactEventType),
    DomainEventType(DomainEventType),
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

impl From<DomainEventType> for EventType {
    fn from(value: DomainEventType) -> Self {
        Self::DomainEventType(value)
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
pub struct DomainEvent {
    #[serde(rename = "type")]
    pub r#type: DomainEventType,
    pub created_at: String,
    pub data: Domain,
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
pub enum DomainEventType {
    #[serde(rename = "domain.created")]
    DomainCreated,
    #[serde(rename = "domain.updated")]
    DomainUpdated,
    #[serde(rename = "domain.deleted")]
    DomainDeleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailBody {
    pub broadcast_id: Option<BroadcastId>,
    pub created_at: String,
    pub email_id: EmailId,
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
    pub message_id: String,
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

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use crate::{
        events::{
            ContactEventType, DomainEventType, EmailEventType, Event, try_parse_event,
            try_parse_event_type,
        },
        list_opts::ListOptions,
        test::CLIENT,
        types::{ContactIdOrEmail, CreateContactOptions, CreateEventOptions},
    };
    use crate::{test::DebugResult, types::SendEventOptions};

    use serde_json::json;
    use strum::EnumCount;

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

    #[cfg(not(feature = "blocking"))]
    #[test]
    fn email_sent() {
        let data = r#"
    {
      "type": "email.sent",
      "created_at": "2024-02-22T23:41:12.126Z",
      "data": {
        "broadcast_id": "8b146471-e88e-4322-86af-016cd36fd216",
        "created_at": "2024-02-22T23:41:11.894719+00:00",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
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
        "created_at": "2024-02-22T23:41:11.894719+00:00",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
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
        "created_at": "2024-02-22T23:41:11.894719+00:00",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
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
        "created_at": "2024-02-22T23:41:11.894719+00:00",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
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
        "created_at": "2024-11-22T23:41:11.894719+00:00",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
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
        "created_at": "2024-02-22T23:41:11.894719+00:00",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
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
        "created_at": "2024-11-22T23:41:11.894719+00:00",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
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
        "created_at": "2024-11-22T23:41:11.894719+00:00",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
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
        "created_at": "2024-02-22T23:41:11.894719+00:00",
        "from": "Acme <onboarding@resend.dev>",
        "to": ["delivered@resend.dev"],
        "bcc": [],
        "cc": [],
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
            assert!(email_event.data.received.is_some());
            assert!(email_event.data.tags.is_empty());

            let received = email_event.data.received.unwrap();
            assert!(received.attachments.len() == 1);
            assert!(received.cc.is_empty());
            assert!(received.bcc.is_empty());
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
        "created_at": "2024-02-22T23:41:11.894719+00:00",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
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
        "created_at": "2024-11-22T23:41:11.894719+00:00",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
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
            assert!(contact_event.data.segment_ids.len() == 1);
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
        "created_at": "2024-10-10T15:11:94.110Z",
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
            assert!(contact_event.data.segment_ids.len() == 1);
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
        "created_at": "2024-11-10T15:11:94.110Z",
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
            assert!(contact_event.data.segment_ids.len() == 1);
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
    #[ignore = "JSON outdated"]
    fn domain_created() {
        let data = r#"
    {
      "type": "domain.created",
      "created_at": "2024-11-17T19:32:22.980Z",
      "data": {
        "id": "d91cd9bd-1176-453e-8fc1-35364d380206",
        "name": "example.com",
        "status": "not_started",
        "created_at": "2024-04-26T20:21:26.347412+00:00",
        "region": "us-east-1",
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
    #[ignore = "JSON outdated"]
    fn domain_updated() {
        let data = r#"
    {
      "type": "domain.updated",
      "created_at": "2024-11-17T19:32:22.980Z",
      "data": {
        "id": "d91cd9bd-1176-453e-8fc1-35364d380206",
        "name": "example.com",
        "status": "not_started",
        "created_at": "2024-04-26T20:21:26.347412+00:00",
        "region": "us-east-1",
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
    #[ignore = "JSON outdated"]
    fn domain_deleted() {
        let data = r#"
    {
      "type": "domain.deleted",
      "created_at": "2024-11-17T19:32:22.980Z",
      "data": {
        "id": "d91cd9bd-1176-453e-8fc1-35364d380206",
        "name": "example.com",
        "status": "not_started",
        "created_at": "2024-04-26T20:21:26.347412+00:00",
        "region": "us-east-1",
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

    /// Similar to the test in `error.rs`
    #[allow(clippy::unwrap_used)]
    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn events_up_to_date() -> DebugResult<()> {
        let response = reqwest::get("https://resend.com/docs/dashboard/webhooks/event-types")
            .await
            .unwrap();

        let html = response.text().await.unwrap();

        let fragment = scraper::Html::parse_document(&html);
        let selector =
            scraper::Selector::parse("#content > div > div > div > span > a > code").unwrap();

        let expected = EmailEventType::COUNT + ContactEventType::COUNT + DomainEventType::COUNT;
        let actual = fragment
            .select(&selector)
            .map(|el| el.inner_html())
            .collect::<Vec<_>>();

        for el in &actual {
            // Add quotes around it
            let parsed = try_parse_event_type(&format!("\"{el}\""));
            assert!(parsed.is_ok(), "Could not parse: {el}");
        }

        assert!(expected == actual.len());

        Ok(())
    }
}
