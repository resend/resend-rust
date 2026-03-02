use std::sync::Arc;

use mailparse::{DispositionType, MailHeaderMap};
use reqwest::Method;

use crate::{
    Config, Error, Result,
    emails::EmailsSvc,
    list_opts::{ListOptions, ListResponse},
    receiving::types::{ContentSpecified, ForwardReceivingEmail},
    types::{
        Attachment, CreateAttachment, CreateEmailBaseOptions, ForwardInboundEmailResponse,
        InboundEmail, InboundEmailId,
    },
};

/// `Resend` APIs for `/emails/receiving` endpoints.
#[derive(Clone, Debug)]
pub struct ReceivingSvc(pub(crate) Arc<Config>);

impl ReceivingSvc {
    /// Retrieve a single received email.
    ///
    /// <https://resend.com/docs/api-reference/emails/retrieve-received-email>
    #[maybe_async::maybe_async]
    pub async fn get(&self, email_id: &str) -> Result<InboundEmail> {
        let path = format!("/emails/receiving/{email_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<InboundEmail>().await?;

        Ok(content)
    }

    /// Retrieve a list of received emails for the authenticated user.
    ///
    /// <https://resend.com/docs/api-reference/emails/list-received-emails>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn list<T>(&self, list_opts: ListOptions<T>) -> Result<ListResponse<InboundEmail>> {
        let request = self
            .0
            .build(Method::GET, "/emails/receiving")
            .query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<InboundEmail>>().await?;

        Ok(content)
    }

    /// Retrieve a single attachment from a received email.
    ///
    /// <https://resend.com/docs/api-reference/emails/retrieve-received-email>
    #[maybe_async::maybe_async]
    pub async fn get_attachment(&self, attachment_id: &str, email_id: &str) -> Result<Attachment> {
        let path = format!("/emails/receiving/{email_id}/attachments/{attachment_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Attachment>().await?;

        Ok(content)
    }

    /// Retrieve a list of email attachments and their contents.
    ///
    /// <https://resend.com/docs/api-reference/attachments/list-received-email-attachments>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn list_attachments<T>(
        &self,
        email_id: &str,
        list_opts: ListOptions<T>,
    ) -> Result<ListResponse<Attachment>> {
        let path = format!("/emails/receiving/{email_id}/attachments");

        let request = self.0.build(Method::GET, &path).query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<Attachment>>().await?;

        Ok(content)
    }

    pub async fn forward(
        &self,
        opts: ForwardReceivingEmail<ContentSpecified>,
    ) -> Result<ForwardInboundEmailResponse> {
        let email_response = self.get(&opts.email_id).await?;

        let raw = email_response.raw.ok_or_else(|| {
            Error::Resend(crate::types::ErrorResponse {
                status_code: 400,
                message: "Raw email content is not available for this email".to_owned(),
                name: "validation_error".to_owned(),
            })
        })?;

        let raw_response_content = reqwest::get(raw.download_url).await?.bytes().await?;

        let email_svc = EmailsSvc(Arc::<Config>::clone(&self.0));

        if opts.passthrough {
            let parsed = mailparse::parse_mail(&raw_response_content)
                .map_err(|_e| Error::Parse("Failed to parse raw email".to_owned()))?;

            let attachments = parsed
                .subparts
                .iter()
                .filter(|el| {
                    el.get_content_disposition().disposition == DispositionType::Attachment
                })
                .map(|attachment| {
                    let disposition = attachment.get_content_disposition();

                    let filename = disposition
                        .params
                        .get("filename")
                        .ok_or_else(|| Error::Parse("Could not parse filename".to_string()))?
                        .to_owned();
                    let content = attachment
                        .get_body_raw()
                        .map_err(|_e| Error::Parse("Could not get attachment body".to_string()))?;
                    let content_type = attachment.ctype.mimetype.clone();

                    if let Some(content_id) = attachment.headers.get_first_header("Content-ID") {
                        let mut content_id = content_id.get_key();
                        if content_id.starts_with('<') {
                            content_id = content_id[1..content_id.len() - 1].to_string();
                        }

                        let attachment = CreateAttachment::from_content(content)
                            .with_content_id(&content_id)
                            .with_filename(&filename)
                            .with_content_type(&content_type);
                        Ok(attachment)
                    } else {
                        let attachment = CreateAttachment::from_content(content)
                            .with_filename(&filename)
                            .with_content_type(&content_type);
                        Ok(attachment)
                    }
                })
                .collect::<Result<Vec<_>>>()?;

            let mut email = CreateEmailBaseOptions::new(opts.from, opts.to, email_response.subject)
                .with_attachments(attachments);

            if let Some(text) = &opts.text {
                email = email.with_text(text);
            } else if let Some(html) = &opts.html {
                email = email.with_html(html);
            }

            let res = email_svc.send(email).await?;

            Ok(ForwardInboundEmailResponse {
                id: InboundEmailId::new(&res.id),
            })
        } else {
            let subject = if email_response.subject.starts_with("Fwd:") {
                email_response.subject
            } else {
                format!("Fwd: {}", email_response.subject)
            };

            let attachment = CreateAttachment::from_content(raw_response_content.to_vec())
                .with_filename("forwarded_message.eml")
                .with_content_type("message/rfc822");

            let mut email = CreateEmailBaseOptions::new(opts.from, opts.to, subject)
                .with_attachments(vec![attachment]);

            if let Some(text) = &opts.text {
                email = email.with_text(text);
            } else if let Some(html) = &opts.html {
                email = email.with_html(html);
            }

            let res = email_svc.send(email).await?;

            Ok(ForwardInboundEmailResponse {
                id: InboundEmailId::new(&res.id),
            })
        }
    }
}

#[allow(unreachable_pub)]
pub mod types {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    crate::define_id_type!(InboundEmailId);
    crate::define_id_type!(InboundAttachmentId);

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Raw {
        pub download_url: String,
        pub expires_at: String,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InboundEmail {
        pub id: InboundEmailId,
        pub to: Vec<String>,
        pub from: String,
        pub created_at: String,
        pub subject: String,
        #[serde(default)]
        pub bcc: Vec<String>,
        #[serde(default)]
        pub cc: Vec<String>,
        #[serde(default)]
        pub reply_to: Vec<String>,
        pub html: Option<String>,
        pub text: Option<String>,
        #[serde(default)]
        pub headers: HashMap<String, String>,
        pub message_id: String,
        pub raw: Option<Raw>,
        #[serde(default)]
        pub attachments: Vec<InboundAttachment>,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InboundAttachment {
        pub id: InboundAttachmentId,
        pub filename: Option<String>,
        pub size: Option<u32>,
        pub content_type: String,
        pub content_id: Option<String>,
        pub content_disposition: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct ContentNotSpecified {}

    #[derive(Debug, Clone, Copy)]
    pub struct ContentSpecified {}

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ForwardReceivingEmail<Contents = ContentNotSpecified> {
        #[serde(skip)]
        pub(crate) contents: std::marker::PhantomData<Contents>,

        pub(crate) passthrough: bool,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) html: Option<String>,

        pub(crate) email_id: InboundEmailId,
        pub(crate) to: Vec<String>,
        pub(crate) from: String,
    }

    impl ForwardReceivingEmail {
        pub fn new(
            email_id: InboundEmailId,
            from: impl Into<String>,
            to: impl IntoIterator<Item = impl Into<String>>,
        ) -> Self {
            Self {
                contents: std::marker::PhantomData,
                passthrough: true,
                text: None,
                html: None,
                email_id,
                to: to.into_iter().map(Into::into).collect(),
                from: from.into(),
            }
        }
    }

    impl<T> ForwardReceivingEmail<T> {
        #[inline]
        pub fn with_passthrough(mut self, passthrough: bool) -> Self {
            self.passthrough = passthrough;
            self
        }
    }

    impl ForwardReceivingEmail<ContentNotSpecified> {
        #[inline]
        pub fn with_text(self, text: &str) -> ForwardReceivingEmail<ContentSpecified> {
            ForwardReceivingEmail::<ContentSpecified> {
                contents: std::marker::PhantomData,

                passthrough: self.passthrough,

                text: Some(text.to_owned()),
                html: None,

                email_id: self.email_id,
                to: self.to,
                from: self.from,
            }
        }

        #[inline]
        pub fn with_html(self, html: &str) -> ForwardReceivingEmail<ContentSpecified> {
            ForwardReceivingEmail::<ContentSpecified> {
                contents: std::marker::PhantomData,

                passthrough: self.passthrough,

                text: None,
                html: Some(html.to_owned()),

                email_id: self.email_id,
                to: self.to,
                from: self.from,
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ForwardInboundEmailResponse {
        pub id: InboundEmailId,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::needless_return)]
mod test {
    use crate::{
        list_opts::{ListOptions, ListResponse},
        test::{CLIENT, DebugResult},
        types::{ForwardReceivingEmail, InboundEmail},
    };

    #[ignore = "At the moment, we can't programmatically send inbound emails and since said inbound emails are only retained for 2 weeks, this cannot be automatically tested."]
    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> DebugResult<()> {
        let resend = &*CLIENT;

        // std::thread::sleep(std::time::Duration::from_secs(1));

        let emails = resend.receiving.list(ListOptions::default()).await?;

        let email_id = &emails.data.first().unwrap().id;

        let _email = resend.receiving.get(email_id).await?;

        let fwd_opts = ForwardReceivingEmail::new(
            email_id.clone(),
            "test@resend.dev",
            vec!["delivered@resend.dev"],
        )
        .with_text("text")
        .with_passthrough(true);
        let _fwd_res = resend.receiving.forward(fwd_opts).await?;

        let attachments = resend
            .receiving
            .list_attachments(email_id, ListOptions::default())
            .await?;

        let attachment_id = &attachments.data.first().unwrap().id;

        let _attachment = resend
            .receiving
            .get_attachment(attachment_id, email_id)
            .await?;

        Ok(())
    }

    #[test]
    fn deserialize_test() {
        let emails = r#"{
  "object": "list",
  "has_more": true,
  "data": [
    {
      "id": "a39999a6-88e3-48b1-888b-beaabcde1b33",
      "to": ["recipient@example.com"],
      "from": "sender@example.com",
      "created_at": "2025-10-09 14:37:40.951732+00",
      "subject": "Hello World",
      "bcc": [],
      "cc": [],
      "reply_to": [],
      "message_id": "<111-222-333@email.provider.example.com>",
      "attachments": [
        {
          "filename": "example.txt",
          "content_type": "text/plain",
          "content_id": null,
          "content_disposition": "attachment",
          "id": "47e999c7-c89c-4999-bf32-aaaaa1c3ff21",
          "size": 13
        }
      ]
    }
  ]
}"#;

        let res = serde_json::from_str::<ListResponse<InboundEmail>>(emails);
        assert!(res.is_ok());
    }

    #[test]
    fn deserialize_test2() {
        let emails = r#"{
  "object": "list",
  "has_more": true,
  "data": [
    {
      "id": "a39999a6-88e3-48b1-888b-beaabcde1b33",
      "to": ["recipient@example.com"],
      "from": "sender@example.com",
      "created_at": "2025-10-09 14:37:40.951732+00",
      "subject": "Hello World",
      "bcc": [],
      "cc": [],
      "reply_to": [],
      "message_id": "<111-222-333@email.provider.example.com>",
      "raw": {
        "download_url": "https://example.com/emails/raw/abc123?signature=xyz789",
        "expires_at": "2023-04-08T00:13:52.669661+00:00"
      },
      "attachments": [
        {
          "filename": "example.txt",
          "content_type": "text/plain",
          "content_id": null,
          "content_disposition": "attachment",
          "id": "47e999c7-c89c-4999-bf32-aaaaa1c3ff21",
          "size": 13
        }
      ]
    }
  ]
}"#;

        let res = serde_json::from_str::<ListResponse<InboundEmail>>(emails);
        assert!(res.is_ok());
        assert!(res.unwrap().data.first().unwrap().raw.is_some());
    }
}
