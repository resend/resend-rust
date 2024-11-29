//! Parsing for Resend's Events.
//!
//! For an example on how to add (Axum) middleware that verifies incoming event signatures,
//! check out [this example](https://github.com/resend/resend-rust/blob/main/examples/axum-verify-event-middleware.rs).

#![allow(dead_code)]

use serde::Deserialize;

use crate::{types::Domain, Result};

/// Parses a JSON event into an [`Event`].
/// ## Example
///
/// ```
/// use resend_rs::events::{Event, try_parse_event};
///
/// let data = r#"
///   {
///     "type": "email.sent",
///     "created_at": "2024-11-23T15:53:07.839Z",
///     "data": {
///         "created_at": "2024-11-23 15:53:07.743225+00",
///         "email_id": "9a148e6d-d79f-43cb-8022-22320546e1db",
///         "from": "Acme <onboarding@resend.dev>",
///         "subject": "hello world",
///         "to": ["delivered@resend.dev"]
///     }
///   }"#;
///
///  let parsed: Result<Event, resend_rs::Error> = try_parse_event(data);
/// ```
pub fn try_parse_event(data: &str) -> Result<Event> {
    serde_json::from_str::<Event>(data).map_err(|e| crate::Error::Parse(e.to_string()))
}

/// Represents any [Resend Event Type](https://resend.com/docs/dashboard/webhooks/event-types).
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Event {
    EmailEvent(EmailEvent),
    ContactEvent(ContactEvent),
    DomainEvent(DomainEvent),
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailEvent {
    #[serde(rename = "type")]
    #[allow(clippy::used_underscore_binding)]
    _type: EmailEventType,
    created_at: String,

    #[serde(rename = "data")]
    body: EmailBody,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContactEvent {
    #[serde(rename = "type")]
    #[allow(clippy::used_underscore_binding)]
    _type: ContactEventType,
    created_at: String,

    #[serde(rename = "data")]
    body: ContactBody,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DomainEvent {
    #[serde(rename = "type")]
    #[allow(clippy::used_underscore_binding)]
    _type: DomainEventType,
    created_at: String,

    #[serde(rename = "data")]
    body: Domain,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum EmailEventType {
    #[serde(rename = "email.sent")]
    EmailSent,
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
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum ContactEventType {
    #[serde(rename = "contact.created")]
    ContactCreated,
    #[serde(rename = "contact.updated")]
    ContactUpdated,
    #[serde(rename = "contact.deleted")]
    ContactDeleted,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum DomainEventType {
    #[serde(rename = "domain.created")]
    DomainCreated,
    #[serde(rename = "domain.updated")]
    DomainUpdated,
    #[serde(rename = "domain.deleted")]
    DomainDeleted,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailBody {
    created_at: String,
    email_id: String,
    from: String,
    to: Vec<String>,
    click: Option<Click>,
    subject: String,
}

/// Extra data only populated in [`EmailEventType::EmailClicked`] events.
#[derive(Debug, Clone, Deserialize)]
pub struct Click {
    #[serde(rename = "ipAddress")]
    ip_address: String,
    link: String,
    timestamp: String,
    #[serde(rename = "userAgent")]
    user_agent: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContactBody {
    id: String,
    audience_id: String,
    created_at: String,
    updated_at: String,
    email: String,
    first_name: String,
    last_name: String,
    unsubscribed: bool,
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use crate::events::{
        try_parse_event, ContactEventType, DomainEventType, EmailEventType, Event,
    };

    #[test]
    fn email_sent() {
        let data = r#"
    {
      "type": "email.sent",
      "created_at": "2024-11-23T15:53:07.839Z",
      "data": {
          "created_at": "2024-11-23 15:53:07.743225+00",
          "email_id": "9a148e6d-d79f-43cb-8022-22320546e1db",
          "from": "Acme <onboarding@resend.dev>",
          "subject": "hello world",
          "to": ["delivered@resend.dev"]
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::EmailEvent(email_event) = parsed {
            assert!(matches!(email_event._type, EmailEventType::EmailSent));
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
        "created_at": "2024-02-22T23:41:11.894719+00:00",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
        "from": "Acme <onboarding@resend.dev>",
        "to": ["delivered@resend.dev"],
        "subject": "Sending this example"
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::EmailEvent(email_event) = parsed {
            assert!(matches!(email_event._type, EmailEventType::EmailDelivered));
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
        "created_at": "2024-02-22T23:41:11.894719+00:00",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
        "from": "Acme <onboarding@resend.dev>",
        "to": ["delivered@resend.dev"],
        "subject": "Sending this example"
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::EmailEvent(email_event) = parsed {
            assert!(matches!(
                email_event._type,
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
        "created_at": "2024-02-22T23:41:11.894719+00:00",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
        "from": "Acme <onboarding@resend.dev>",
        "to": ["delivered@resend.dev"],
        "subject": "Sending this example"
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::EmailEvent(email_event) = parsed {
            assert!(matches!(email_event._type, EmailEventType::EmailComplained));
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
        "created_at": "2024-11-22T23:41:11.894719+00:00",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
        "from": "Acme <onboarding@resend.dev>",
        "to": ["delivered@resend.dev"],
        "subject": "Sending this example"
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::EmailEvent(email_event) = parsed {
            assert!(matches!(email_event._type, EmailEventType::EmailBounced));
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
        "created_at": "2024-02-22T23:41:11.894719+00:00",
        "email_id": "56761188-7520-42d8-8898-ff6fc54ce618",
        "from": "Acme <onboarding@resend.dev>",
        "to": ["delivered@resend.dev"],
        "subject": "Sending this example"
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::EmailEvent(email_event) = parsed {
            assert!(matches!(email_event._type, EmailEventType::EmailOpened));
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
        "subject": "Sending this example"
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::EmailEvent(email_event) = parsed {
            assert!(matches!(email_event._type, EmailEventType::EmailClicked));
            assert!(email_event.body.click.is_some());
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
                contact_event._type,
                ContactEventType::ContactCreated
            ));
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
                contact_event._type,
                ContactEventType::ContactUpdated
            ));
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
                contact_event._type,
                ContactEventType::ContactDeleted
            ));
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
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
            assert!(matches!(domain_event._type, DomainEventType::DomainCreated));
            assert!(domain_event.body.records.is_some_and(|r| r.len() == 3));
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
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
          }
        ]
      }
    }"#;

        let parsed = try_parse_event(data);
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        if let Event::DomainEvent(domain_event) = parsed {
            assert!(matches!(domain_event._type, DomainEventType::DomainUpdated));
            assert!(domain_event.body.records.is_some_and(|r| r.len() == 3));
        } else {
            panic!("Wrong parsing");
        }
    }

    #[test]
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
            assert!(matches!(domain_event._type, DomainEventType::DomainDeleted));
            assert!(domain_event.body.records.is_some_and(|r| r.len() == 3));
        } else {
            panic!("Wrong parsing");
        }
    }
}
