use std::sync::Arc;

use ::reqwest::Method;
use mailparse::{DispositionType, MailHeaderMap};

#[cfg(feature = "blocking")]
use reqwest::blocking as reqwest;

#[cfg(not(feature = "blocking"))]
use ::reqwest::RequestBuilder;
#[cfg(feature = "blocking")]
use ::reqwest::blocking::RequestBuilder;

use crate::{
    Config, Error, Result,
    emails::EmailsSvc,
    list_opts::{ListOptions, ListResponse},
    receiving::types::ForwardReceivingEmail,
    types::{
        Attachment, CreateAttachment, CreateEmailBaseOptions, ForwardInboundEmailResponse,
        GetInboundEmailOptions, InboundEmail, InboundEmailId, ListInboundEmailsOptions,
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
        self.list_with_filters(ListInboundEmailsOptions::default(), list_opts)
            .await
    }

    /// Retrieve a list of received emails, filtered by recipient and date.
    ///
    /// <https://resend.com/docs/api-reference/emails/list-received-emails>
    #[maybe_async::maybe_async]
    pub async fn list_with_filters<T>(
        &self,
        filters: ListInboundEmailsOptions,
        list_opts: ListOptions<T>,
    ) -> Result<ListResponse<InboundEmail>> {
        let request = self.list_request(&filters, &list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<InboundEmail>>().await?;

        Ok(content)
    }

    fn list_request<T>(
        &self,
        filters: &ListInboundEmailsOptions,
        list_opts: &ListOptions<T>,
    ) -> RequestBuilder {
        self.0
            .build(Method::GET, "/emails/receiving")
            .query(list_opts)
            .query(filters)
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

        let raw = email_response.raw.ok_or_else(|| Error::Resend {
            status: 400,
            error: crate::types::ErrorResponse {
                status_code: 400,
                message: "Raw email content is not available for this email".to_owned(),
                name: "validation_error".to_owned(),
            },
        })?;

        let raw_response_content = download_raw(raw.download_url).await?;

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

            let attachment = CreateAttachment::from_content(raw_response_content)
                .with_filename("forwarded_message.eml")
                .with_content_type("message/rfc822");

            let email = non_passthrough_email(opts, subject, attachment);

            let res = email_svc.send(email).await?;

            Ok(ForwardInboundEmailResponse {
                id: InboundEmailId::new(&res.id),
            })
        }
    }
}

/// Downloads the raw message, failing on a non success status so that an error page is never
/// forwarded as if it were the original email.
#[maybe_async::maybe_async]
async fn download_raw(url: String) -> Result<Vec<u8>> {
    Ok(reqwest::get(url)
        .await?
        .error_for_status()?
        .bytes()
        .await?
        .to_vec())
}

/// Builds the email sent by a forward without `passthrough`: the raw message is attached and
/// both the caller supplied text and HTML bodies are kept.
fn non_passthrough_email(
    opts: ForwardReceivingEmail,
    subject: String,
    attachment: CreateAttachment,
) -> CreateEmailBaseOptions {
    let mut email =
        CreateEmailBaseOptions::new(opts.from, opts.to, subject).with_attachments(vec![attachment]);

    if let Some(text) = &opts.text {
        email = email.with_text(text);
    }

    if let Some(html) = &opts.html {
        email = email.with_html(html);
    }

    email
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

    /// Filters for [`ReceivingSvc::list_with_filters`](crate::services::ReceivingSvc::list_with_filters).
    ///
    /// [`ListInboundEmailsOptions::default()`] applies no filters.
    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct ListInboundEmailsOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        to: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        start_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        end_date: Option<String>,
    }

    impl ListInboundEmailsOptions {
        pub fn new() -> Self {
            Self::default()
        }

        /// Recipients. Returns emails delivered to any of these addresses. Each value must match
        /// the envelope recipient exactly, including case. Up to 20 values.
        #[inline]
        pub fn with_to<S: AsRef<str>>(mut self, to: impl IntoIterator<Item = S>) -> Self {
            self.to = crate::inboxes::types::join(to);
            self
        }

        /// Matches emails received on or after this date. Accepts a date like `2026-09-01` or an
        /// ISO 8601 timestamp, in UTC unless the timestamp has an offset.
        #[inline]
        pub fn with_start_date(mut self, start_date: impl Into<String>) -> Self {
            self.start_date = Some(start_date.into());
            self
        }

        /// Matches emails received on or before this date. Same formats as `start_date`. A date
        /// covers that whole day. Must not be before `start_date`.
        #[inline]
        pub fn with_end_date(mut self, end_date: impl Into<String>) -> Self {
            self.end_date = Some(end_date.into());
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
        #[serde(deserialize_with = "nullable_string")]
        pub subject: String,
        #[serde(default, deserialize_with = "nullable_vec")]
        pub bcc: Vec<String>,
        #[serde(default, deserialize_with = "nullable_vec")]
        pub cc: Vec<String>,
        #[serde(default, deserialize_with = "nullable_vec")]
        pub reply_to: Vec<String>,
        #[serde(default)]
        pub received_for: Vec<String>,
        pub html: Option<String>,
        pub text: Option<String>,
        #[serde(default, deserialize_with = "nullable_headers")]
        pub headers: HashMap<String, Vec<String>>,
        pub message_id: String,
        pub raw: Option<Raw>,
        #[serde(default)]
        pub attachments: Vec<InboundAttachment>,
    }

    impl InboundEmail {
        /// Returns the first value for the header with the given name.
        ///
        /// Header names are matched case-insensitively.
        #[must_use]
        pub fn header(&self, name: &str) -> Option<&str> {
            self.headers
                .iter()
                .find(|(header, _)| header.eq_ignore_ascii_case(name))
                .and_then(|(_, values)| values.first())
                .map(String::as_str)
        }
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

    /// Deserializes an `array | null` address field, treating `null` as an empty list.
    fn nullable_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Option::<Vec<String>>::deserialize(deserializer)?.unwrap_or_default())
    }

    /// Deserializes a `string | null` field, treating `null` as an empty string.
    fn nullable_string<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Option::<String>::deserialize(deserializer)?.unwrap_or_default())
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum HeaderValue {
        One(String),
        Many(Vec<String>),
    }

    /// Deserializes the `headers` object, treating `null` as an empty map.
    ///
    /// Each value is kept as a list so repeated headers keep their boundaries.
    fn nullable_headers<'de, D>(deserializer: D) -> Result<HashMap<String, Vec<String>>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let Some(headers) = Option::<HashMap<String, HeaderValue>>::deserialize(deserializer)?
        else {
            return Ok(HashMap::new());
        };

        Ok(headers
            .into_iter()
            .map(|(name, value)| {
                let values = match value {
                    HeaderValue::One(value) => vec![value],
                    HeaderValue::Many(values) => values,
                };
                (name, values)
            })
            .collect())
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
        status: None,
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
    fn non_passthrough_forward_keeps_text_and_html() {
        use crate::types::{CreateAttachment, ForwardReceivingEmail, InboundEmailId};

        let opts = ForwardReceivingEmail::new(
            InboundEmailId::new("id"),
            "from@example.test",
            vec!["to@example.test"],
        )
        .with_text("plain")
        .with_html("<p>rich</p>");
        let attachment = CreateAttachment::from_content(b"raw".to_vec());

        let email = super::non_passthrough_email(opts, "Fwd: hi".to_owned(), attachment);
        let json = serde_json::to_value(email).unwrap();
        assert_eq!(
            json.get("text").and_then(serde_json::Value::as_str),
            Some("plain")
        );
        assert_eq!(
            json.get("html").and_then(serde_json::Value::as_str),
            Some("<p>rich</p>")
        );
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

    #[test]
    fn deserialize_test3() {
        let email = r#"{
          "object": "email",
          "id": "4ef9a417-02e9-4d39-ad75-9611e0fcc33c",
          "to": ["delivered@resend.dev"],
          "from": "onboarding@resend.dev",
          "created_at": "2026-04-03T22:13:42.674Z",
          "subject": "Hello World",
          "html": "Congrats on sending your <strong>first email</strong>!",
          "html_format": "data_uri",
          "text": null,
          "headers": {
            "from": "Acme <onboarding@resend.dev>",
            "return-path": "lucas.costa@resend.com",
            "mime-version": "1.0"
          },
          "bcc": [],
          "cc": [],
          "reply_to": [],
          "received_for": ["forwarded@example.com"],
          "authentication": {
            "spf": "pass",
            "dkim": "pass",
            "dmarc": "pass"
          },
          "message_id": "<111-222-333@email.example.com>",
          "raw": {
            "download_url": "https://example.resend.com/receiving/raw/054da427-439a-4e91-b785-e4fb1966285f?Signature=...",
            "expires_at": "2026-04-03T23:13:42.674Z"
          },
          "attachments": [
            {
              "id": "2a0c9ce0-3112-4728-976e-47ddcd16a318",
              "filename": "avatar.png",
              "content_type": "image/png",
              "content_disposition": "inline",
              "content_id": "img001",
              "size": 4096
            },
            {
              "id": "3b1d0df1-4223-5839-087f-54eedd27b419",
              "filename": "document.pdf",
              "content_type": "application/pdf",
              "content_disposition": null,
              "content_id": null,
              "size": 13264
            }
          ]
        }"#;

        let res = serde_json::from_str::<InboundEmail>(email);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().headers.keys().len(), 3);
    }

    #[test]
    fn rejects_headers_as_name_value_list() {
        let email = r#"    {
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
          "headers": [
            { "name": "References", "value": ["<id1@example.com>", "<id2@example.com>"] }
          ],
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
        }"#;

        // The spec defines `headers` as an object or null, not a list of name/value entries.
        let res = serde_json::from_str::<InboundEmail>(email);
        assert!(res.is_err());
    }

    #[test]
    fn deserialize_nullable_inbound_addresses_and_headers() {
        let email = r#"{
            "object": "email",
            "id": "00000000-0000-0000-0000-000000000000",
            "to": ["a@example.com"],
            "from": "b@example.com",
            "subject": null,
            "html": "",
            "text": "",
            "headers": null,
            "bcc": null,
            "cc": null,
            "reply_to": null,
            "received_for": ["forwarded@example.com"],
            "attachments": [],
            "created_at": "2026-11-18 00:00:00.000000+00",
            "message_id": "<x@example.com>"
        }"#;

        let email: InboundEmail = serde_json::from_str(email).expect("Parsing failed");
        assert_eq!(email.to, vec!["a@example.com"]);
        assert_eq!(email.subject, "");
        assert_eq!(email.bcc, Vec::<String>::new());
        assert_eq!(email.cc, Vec::<String>::new());
        assert_eq!(email.reply_to, Vec::<String>::new());
        assert_eq!(email.received_for, vec!["forwarded@example.com"]);
        assert_eq!(email.headers, std::collections::HashMap::new());
    }

    /// Parses an `InboundEmail` with the given extra JSON fields appended.
    fn inbound_email_with(fields: &str) -> Result<InboundEmail, serde_json::Error> {
        let json = format!(
            r#"{{
                "id": "00000000-0000-0000-0000-000000000000",
                "from": "b@example.com",
                "created_at": "2026-11-18 00:00:00.000000+00",
                "subject": "Hi",
                "message_id": "<x@example.com>"{fields}
            }}"#
        );
        serde_json::from_str(&json)
    }

    #[test]
    fn rejects_non_spec_shapes() {
        assert!(inbound_email_with(r#", "to": "a@example.com""#).is_err());
        assert!(inbound_email_with(r#", "to": null"#).is_err());
        assert!(inbound_email_with(r#", "to": ["a@example.com"], "cc": "c@example.com""#).is_err());
        assert!(
            inbound_email_with(
                r#", "to": ["a@example.com"], "headers": [{ "name": "X-Trace", "value": "abc" }]"#
            )
            .is_err()
        );
    }

    fn assert_header(email: &InboundEmail, name: &str, expected: &[&str]) {
        let expected: Vec<String> = expected.iter().map(|value| (*value).to_owned()).collect();
        assert_eq!(
            email.headers.get(name).map(Vec::as_slice),
            Some(expected.as_slice())
        );
    }

    #[test]
    fn deserialize_map_headers_with_list_values() {
        let email = r#"{
            "id": "00000000-0000-0000-0000-000000000000",
            "to": ["a@example.com"],
            "from": "b@example.com",
            "created_at": "2026-11-18 00:00:00.000000+00",
            "subject": "Hi",
            "headers": {
                "References": ["<one@example.com>", "<two@example.com>"],
                "In-Reply-To": ["<a@example.com>", "<b@example.com>"],
                "X-Trace": ["abc", "def"],
                "Subject": "Hi"
            },
            "message_id": "<x@example.com>"
        }"#;

        let email: InboundEmail = serde_json::from_str(email).expect("Parsing failed");
        assert_header(
            &email,
            "References",
            &["<one@example.com>", "<two@example.com>"],
        );
        assert_header(
            &email,
            "In-Reply-To",
            &["<a@example.com>", "<b@example.com>"],
        );
        assert_header(&email, "X-Trace", &["abc", "def"]);
        assert_header(&email, "Subject", &["Hi"]);

        assert_eq!(email.header("Subject"), Some("Hi"));
        assert_eq!(email.header("subject"), Some("Hi"));
        assert_eq!(email.header("missing"), None);
    }

    /// Serves one canned HTTP response on a local port and returns the URL.
    #[cfg(not(feature = "blocking"))]
    fn serve_once(response: &'static str) -> String {
        use std::io::{Read, Write};

        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/raw", listener.local_addr().unwrap());
        let _t = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0u8; 1024];
            let _res = stream.read(&mut buf);
            stream.write_all(response.as_bytes()).unwrap();
        });
        url
    }

    #[test]
    fn list_filters_query() {
        use crate::{Resend, list_opts::ListOptions, types::ListInboundEmailsOptions};

        fn query<T>(filters: &ListInboundEmailsOptions, list_opts: &ListOptions<T>) -> String {
            let request = Resend::new("re_123")
                .receiving
                .list_request(filters, list_opts);
            request
                .build()
                .unwrap()
                .url()
                .query()
                .unwrap_or_default()
                .to_owned()
        }

        let filters = ListInboundEmailsOptions::new()
            .with_to(["support@acme.com", "billing@acme.com"])
            .with_start_date("2026-11-01")
            .with_end_date("2026-11-15T12:00:00Z");
        let list_opts = ListOptions::default()
            .with_limit(25)
            .list_after("cursor123");
        assert_eq!(
            query(&filters, &list_opts),
            "limit=25&after=cursor123&to=support%40acme.com%2Cbilling%40acme.com&start_date=2026-11-01&end_date=2026-11-15T12%3A00%3A00Z"
        );

        let filters = ListInboundEmailsOptions::new().with_to(["support@acme.com"]);
        assert_eq!(
            query(&filters, &ListOptions::default()),
            "to=support%40acme.com"
        );

        assert_eq!(
            query(
                &ListInboundEmailsOptions::default(),
                &ListOptions::default()
            ),
            ""
        );
    }

    #[tokio::test]
    #[cfg(not(feature = "blocking"))]
    async fn download_raw_rejects_error_status() {
        let url = serve_once(
            "HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\nConnection: close\r\n\r\nnot found",
        );

        let res = super::download_raw(url).await;
        assert!(matches!(res, Err(crate::Error::Http(_))), "{res:?}");
    }

    #[tokio::test]
    #[cfg(not(feature = "blocking"))]
    async fn download_raw_returns_body_on_success() {
        let url = serve_once("HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok");

        let res = super::download_raw(url).await.unwrap();
        assert_eq!(res, b"ok");
    }
}
