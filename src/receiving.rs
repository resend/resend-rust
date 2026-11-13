use std::sync::Arc;

use ::reqwest::Method;
use mailparse::{DispositionType, MailHeaderMap};

#[cfg(feature = "blocking")]
use reqwest::blocking as reqwest;

use crate::{
    Config, Error, Result,
    emails::EmailsSvc,
    list_opts::{ListOptions, ListResponse},
    receiving::types::ForwardReceivingEmail,
    types::{
        Attachment, CreateAttachment, CreateEmailBaseOptions, ForwardInboundEmailResponse,
        GetInboundEmailOptions, InboundEmail, InboundEmailId,
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
    pub async fn get(&self, email_id: &str, opts: GetInboundEmailOptions) -> Result<InboundEmail> {
        let path = format!("/emails/receiving/{email_id}");

        let request = self.0.build(Method::GET, &path).query(&opts);
        let response = self.0.send(request).await?;
        let content = response.json::<InboundEmail>().await?;

        Ok(content)
    }

    /// Retrieve a list of received emails for the authenticated user.
    ///
    /// <https://resend.com/docs/api-reference/emails/list-received-emails>
    #[maybe_async::maybe_async]
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

    #[maybe_async::maybe_async]
    pub async fn forward(
        &self,
        opts: ForwardReceivingEmail,
    ) -> Result<ForwardInboundEmailResponse> {
        let email_response = self
            .get(&opts.email_id, GetInboundEmailOptions::default())
            .await?;

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
            let content = passthrough_content(&raw_response_content)?;

            let mut email = CreateEmailBaseOptions::new(opts.from, opts.to, email_response.subject)
                .with_attachments(content.attachments);

            // A caller supplied body wins, otherwise the original one is re-sent so
            // that `cid:` references in the HTML keep resolving to the attachments.
            let (text, html) = if opts.text.is_some() || opts.html.is_some() {
                (opts.text, opts.html)
            } else {
                (content.text, content.html)
            };

            if let Some(text) = text {
                email = email.with_text(&text);
            }

            if let Some(html) = html {
                email = email.with_html(&html);
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
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct GetInboundEmailOptions {
        html_format: InboundEmailHtmlFormat,
        #[serde(skip_serializing_if = "Option::is_none")]
        raw: Option<GetInboundEmailRaw>,
    }

    impl GetInboundEmailOptions {
        #[inline]
        pub fn with_html_format(mut self, html_format: InboundEmailHtmlFormat) -> Self {
            self.html_format = html_format;
            self
        }

        #[inline]
        pub fn with_raw(mut self, raw: GetInboundEmailRaw) -> Self {
            self.raw = Some(raw);
            self
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Default)]
    #[must_use]
    #[serde(rename_all = "snake_case")]
    pub enum InboundEmailHtmlFormat {
        #[default]
        DataUri,
        Cid,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GetInboundEmailRaw {
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
        #[serde(default)]
        pub received_for: Vec<String>,
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

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ForwardReceivingEmail {
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
                passthrough: true,
                text: None,
                html: None,
                email_id,
                to: to.into_iter().map(Into::into).collect(),
                from: from.into(),
            }
        }
    }

    impl ForwardReceivingEmail {
        #[inline]
        pub fn with_passthrough(mut self, passthrough: bool) -> Self {
            self.passthrough = passthrough;
            self
        }

        #[inline]
        pub fn with_text(mut self, text: &str) -> Self {
            self.text = Some(text.to_owned());
            self
        }

        #[inline]
        pub fn with_html(mut self, html: &str) -> Self {
            self.html = Some(html.to_owned());
            self
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ForwardInboundEmailResponse {
        pub id: InboundEmailId,
    }
}

/// The parts of a raw MIME message that `ReceivingSvc::forward` with `passthrough`
/// re-sends as-is.
struct PassthroughContent {
    attachments: Vec<CreateAttachment>,
    text: Option<String>,
    html: Option<String>,
}

/// Extracts the body and the attachments of a raw MIME message so they can be
/// re-sent as-is.
///
/// Parts are collected from the whole MIME tree, not only from the top level, so
/// attachments nested inside e.g. a `multipart/related` are kept. Inline parts
/// such as `cid:` images are kept as well, see [`is_passthrough_attachment`].
fn passthrough_content(raw: &[u8]) -> Result<PassthroughContent> {
    let parsed = mailparse::parse_mail(raw).map_err(|e| Error::Parse {
        message: "Failed to parse raw email".to_owned(),
        source: Some(Box::new(e)),
    })?;

    let attachments = parsed
        .parts()
        .filter(|part| is_passthrough_attachment(part))
        .map(create_passthrough_attachment)
        .collect::<Result<Vec<_>>>()?;

    Ok(PassthroughContent {
        attachments,
        text: collect_body(&parsed, "text/plain"),
        html: collect_body(&parsed, "text/html"),
    })
}

/// Builds the [`CreateAttachment`] that re-sends a single MIME part.
fn create_passthrough_attachment(part: &mailparse::ParsedMail<'_>) -> Result<CreateAttachment> {
    let content = part
        .get_body_raw()
        .map_err(|_e| Error::Other("Could not get attachment body".to_string()))?;
    let content_type = part.ctype.mimetype.clone();

    // The filename is usually on `Content-Disposition`, but inline parts
    // sometimes only carry it as the `name` parameter of `Content-Type`.
    let filename = part
        .get_content_disposition()
        .params
        .get("filename")
        .or_else(|| part.ctype.params.get("name"))
        .map(ToOwned::to_owned);

    // The angle brackets around a `Content-ID` are not part of the value.
    let content_id = part
        .headers
        .get_first_header("Content-ID")
        .map(|header| {
            header
                .get_value()
                .trim()
                .trim_start_matches('<')
                .trim_end_matches('>')
                .to_owned()
        })
        .filter(|content_id| !content_id.is_empty());

    let mut attachment = CreateAttachment::from_content(content).with_content_type(&content_type);

    if let Some(filename) = filename {
        attachment = attachment.with_filename(&filename);
    }

    if let Some(content_id) = content_id {
        attachment = attachment.with_content_id(&content_id);
    }

    Ok(attachment)
}

/// Collects the decoded bodies of all non-attachment parts of the given MIME type.
fn collect_body(parsed: &mailparse::ParsedMail<'_>, mimetype: &str) -> Option<String> {
    let bodies = parsed
        .parts()
        .filter(|part| {
            part.ctype.mimetype == mimetype
                && part.get_content_disposition().disposition != DispositionType::Attachment
        })
        .filter_map(decoded_body)
        .collect::<Vec<_>>();

    if bodies.is_empty() {
        None
    } else {
        Some(bodies.join("\n"))
    }
}

/// Returns the decoded body of a MIME part, falling back to a lossy decoding so a
/// single oddly encoded part cannot fail the whole forward.
fn decoded_body(part: &mailparse::ParsedMail<'_>) -> Option<String> {
    part.get_body().ok().or_else(|| {
        part.get_body_raw()
            .ok()
            .map(|raw| String::from_utf8_lossy(&raw).into_owned())
    })
}

/// Returns whether a MIME part of a raw email should be re-sent as an attachment.
///
/// This mirrors the reference implementation: every part that is neither a
/// `multipart/*` container nor one of the message bodies (`text/plain` or
/// `text/html` without an `attachment` disposition) is treated as an attachment.
/// That covers regular `Content-Disposition: attachment` parts as well as inline
/// parts, which is where `cid:` referenced images usually live.
fn is_passthrough_attachment(part: &mailparse::ParsedMail<'_>) -> bool {
    if part.ctype.mimetype.starts_with("multipart/") {
        return false;
    }

    part.get_content_disposition().disposition == DispositionType::Attachment
        || !matches!(part.ctype.mimetype.as_str(), "text/plain" | "text/html")
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::needless_return)]
mod test {
    #[cfg(not(feature = "blocking"))]
    use crate::{
        list_opts::ListOptions,
        test::{CLIENT, DebugResult},
        types::ForwardReceivingEmail,
    };
    use crate::{list_opts::ListResponse, types::InboundEmail};

    #[ignore = "At the moment, we can't programmatically send inbound emails and since said inbound emails are only retained for 2 weeks, this cannot be automatically tested."]
    #[tokio_shared_rt::test(shared = true)]
    #[serial_test::serial]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> DebugResult<()> {
        use crate::types::GetInboundEmailOptions;

        let resend = &*CLIENT;

        // std::thread::sleep(std::time::Duration::from_secs(1));

        let emails = resend.receiving.list(ListOptions::default()).await?;

        let email_id = &emails.data.first().unwrap().id;

        let _email = resend
            .receiving
            .get(email_id, GetInboundEmailOptions::default())
            .await?;

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
    fn passthrough_keeps_attachment_content_id() {
        let raw = b"Content-Type: multipart/mixed; boundary=x\r\n\r\n--x\r\nContent-Type: image/png\r\nContent-Disposition: attachment; filename=logo.png\r\nContent-ID: <logo@example.test>\r\n\r\nPNG\r\n--x--\r\n";
        let attachments = super::passthrough_content(raw).unwrap().attachments;
        let json = serde_json::to_value(attachments.first().unwrap()).unwrap();
        assert_eq!(json.get("content_id").unwrap(), "logo@example.test");
    }

    #[test]
    fn passthrough_finds_nested_attachments() {
        let raw = b"Content-Type: multipart/mixed; boundary=out\r\n\r\n--out\r\nContent-Type: multipart/related; boundary=in\r\n\r\n--in\r\nContent-Type: text/html\r\n\r\n<img src=\"cid:logo@example.test\">\r\n--in\r\nContent-Type: image/png\r\nContent-Disposition: attachment; filename=logo.png\r\nContent-ID: <logo@example.test>\r\n\r\nPNG\r\n--in--\r\n--out--\r\n";
        let attachments = super::passthrough_content(raw).unwrap().attachments;
        assert_eq!(attachments.len(), 1);

        let json = serde_json::to_value(attachments.first().unwrap()).unwrap();
        assert_eq!(
            json.get("content_id").and_then(serde_json::Value::as_str),
            Some("logo@example.test")
        );
        assert_eq!(
            json.get("filename").and_then(serde_json::Value::as_str),
            Some("logo.png")
        );
    }

    #[test]
    fn passthrough_keeps_inline_cid_attachments() {
        let raw = b"Content-Type: multipart/related; boundary=x\r\n\r\n--x\r\nContent-Type: text/html\r\n\r\n<img src=\"cid:logo@example.test\">\r\n--x\r\nContent-Type: image/png\r\nContent-Disposition: inline; filename=logo.png\r\nContent-ID: <logo@example.test>\r\n\r\nPNG\r\n--x--\r\n";
        let attachments = super::passthrough_content(raw).unwrap().attachments;
        assert_eq!(attachments.len(), 1);

        let json = serde_json::to_value(attachments.first().unwrap()).unwrap();
        assert_eq!(
            json.get("content_id").and_then(serde_json::Value::as_str),
            Some("logo@example.test")
        );
        assert_eq!(
            json.get("filename").and_then(serde_json::Value::as_str),
            Some("logo.png")
        );
    }

    #[test]
    fn passthrough_falls_back_to_content_type_name_for_filename() {
        let raw = b"Content-Type: multipart/related; boundary=x\r\n\r\n--x\r\nContent-Type: text/html\r\n\r\n<img src=\"cid:photo@example.test\">\r\n--x\r\nContent-Type: image/jpeg; name=\"photo.jpg\"\r\nContent-ID: <photo@example.test>\r\n\r\nJPEG\r\n--x--\r\n";
        let attachments = super::passthrough_content(raw).unwrap().attachments;
        assert_eq!(attachments.len(), 1);

        let json = serde_json::to_value(attachments.first().unwrap()).unwrap();
        assert_eq!(
            json.get("filename").and_then(serde_json::Value::as_str),
            Some("photo.jpg")
        );
        assert_eq!(
            json.get("content_id").and_then(serde_json::Value::as_str),
            Some("photo@example.test")
        );
    }

    #[test]
    fn passthrough_allows_attachments_without_filename() {
        let raw = b"Content-Type: multipart/mixed; boundary=x\r\n\r\n--x\r\nContent-Type: text/plain\r\n\r\nbody\r\n--x\r\nContent-Type: application/pdf\r\nContent-Disposition: attachment\r\nContent-ID: <doc@example.test>\r\n\r\nPDF\r\n--x--\r\n";
        let attachments = super::passthrough_content(raw).unwrap().attachments;
        assert_eq!(attachments.len(), 1);

        let json = serde_json::to_value(attachments.first().unwrap()).unwrap();
        assert!(json.get("filename").is_none());
        assert_eq!(
            json.get("content_id").and_then(serde_json::Value::as_str),
            Some("doc@example.test")
        );
    }

    #[test]
    fn passthrough_resends_the_original_body() {
        let raw = b"Content-Type: multipart/alternative; boundary=x\r\n\r\n--x\r\nContent-Type: text/plain; charset=utf-8\r\n\r\nHello\r\n--x\r\nContent-Type: text/html; charset=utf-8\r\nContent-Transfer-Encoding: base64\r\n\r\nPGI+SGVsbG8gPGltZyBzcmM9ImNpZDpsb2dvQGV4YW1wbGUudGVzdCI+PC9iPg==\r\n--x--\r\n";
        let content = super::passthrough_content(raw).unwrap();

        assert_eq!(content.text.as_deref(), Some("Hello"));
        assert_eq!(
            content.html.as_deref(),
            Some("<b>Hello <img src=\"cid:logo@example.test\"></b>")
        );
        assert!(content.attachments.is_empty());
    }

    #[test]
    fn passthrough_resends_body_and_attachments_together() {
        let raw = b"Content-Type: multipart/related; boundary=x\r\n\r\n--x\r\nContent-Type: multipart/alternative; boundary=y\r\n\r\n--y\r\nContent-Type: text/plain\r\n\r\nHello\r\n--y\r\nContent-Type: text/html\r\n\r\n<p>Hello <img src=\"cid:logo@example.test\"></p>\r\n--y--\r\n--x\r\nContent-Type: image/png\r\nContent-Disposition: inline; filename=logo.png\r\nContent-ID: <logo@example.test>\r\n\r\nPNG\r\n--x--\r\n";
        let content = super::passthrough_content(raw).unwrap();

        assert_eq!(content.text.as_deref(), Some("Hello"));
        assert_eq!(
            content.html.as_deref(),
            Some("<p>Hello <img src=\"cid:logo@example.test\"></p>")
        );
        assert_eq!(content.attachments.len(), 1);
        assert_eq!(
            serde_json::to_value(content.attachments.first().unwrap())
                .unwrap()
                .get("content_id")
                .and_then(serde_json::Value::as_str),
            Some("logo@example.test")
        );
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
      "received_for": ["forwarded@example.com"],
      "message_id": "<111-222-333@email.provider.example.com>",
      "raw": {
        "download_url": "https://example.com/emails/raw/abc123?signature=xyz789",
        "expires_at": "2023-04-08 00:13:52.669661+00"
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
        let data = res.unwrap();
        let email = data.data.first().unwrap();
        assert!(email.raw.is_some());
        assert_eq!(email.received_for, vec!["forwarded@example.com"]);
    }
}
