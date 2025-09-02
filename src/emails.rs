use std::sync::Arc;

use reqwest::Method;
use serde::{Deserialize, Deserializer};

use crate::{
    Config, Result,
    types::{ListEmailOptions, ListEmailResponse},
};
use crate::{
    idempotent::Idempotent,
    types::{
        CancelScheduleResponse, CreateEmailBaseOptions, CreateEmailResponse, Email,
        UpdateEmailOptions, UpdateEmailResponse,
    },
};

/// `Resend` APIs for `/emails` endpoints.
#[derive(Clone, Debug)]
pub struct EmailsSvc(pub(crate) Arc<Config>);

impl EmailsSvc {
    /// Start sending emails through the `Resend` Email API.
    ///
    /// <https://resend.com/docs/api-reference/emails/send-email>
    #[maybe_async::maybe_async]
    // Reasoning for allow: https://github.com/resend/resend-rust/pull/1#issuecomment-2081646115
    #[allow(clippy::needless_pass_by_value)]
    pub async fn send(
        &self,
        email: impl Into<Idempotent<CreateEmailBaseOptions>>,
    ) -> Result<CreateEmailResponse> {
        let email: Idempotent<CreateEmailBaseOptions> = email.into();

        let mut request = self.0.build(Method::POST, "/emails");

        if let Some(ref idempotency_key) = email.idempotency_key {
            request = request.header("Idempotency-Key", idempotency_key);
        }

        let response = self.0.send(request.json(&email)).await?;
        let content = response.json::<CreateEmailResponse>().await?;

        Ok(content)
    }

    /// Retrieve a single email.
    ///
    /// <https://resend.com/docs/api-reference/emails/retrieve-email>
    #[maybe_async::maybe_async]
    pub async fn get(&self, email_id: &str) -> Result<Email> {
        let path = format!("/emails/{email_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Email>().await?;

        Ok(content)
    }

    /// Update a scheduled email.
    ///
    /// <https://resend.com/docs/api-reference/emails/update-email>
    #[maybe_async::maybe_async]
    // Reasoning for allow: https://github.com/resend/resend-rust/pull/1#issuecomment-2081646115
    #[allow(clippy::needless_pass_by_value)]
    pub async fn update(
        &self,
        email_id: &str,
        update: UpdateEmailOptions,
    ) -> Result<UpdateEmailResponse> {
        let path = format!("/emails/{email_id}");

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&update)).await?;
        let content = response.json::<UpdateEmailResponse>().await?;

        Ok(content)
    }

    /// Cancel a scheduled email.
    ///
    /// <https://resend.com/docs/api-reference/emails/cancel-email>
    #[maybe_async::maybe_async]
    pub async fn cancel(&self, email_id: &str) -> Result<CancelScheduleResponse> {
        let path = format!("/emails/{email_id}/cancel");

        let request = self.0.build(Method::POST, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<CancelScheduleResponse>().await?;

        Ok(content)
    }

    /// Retrieve a list of emails.
    ///
    /// <https://resend.com/docs/api-reference/emails/list-emails>
    #[maybe_async::maybe_async]
    pub async fn list<T>(&self, list_opts: ListEmailOptions<T>) -> Result<ListEmailResponse> {
        let request = self.0.build(Method::GET, "/emails").query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListEmailResponse>().await?;

        Ok(content)
    }
}

#[allow(unreachable_pub)]
pub mod types {
    use std::fmt;
    use std::{collections::HashMap, ops::Deref};

    use ecow::EcoString;
    use serde::{Deserialize, Serialize};

    use crate::{emails::parse_nullable_vec, idempotent::Idempotent};

    /// Unique [`Email`] identifier.
    #[derive(Debug, Clone, Deserialize)]
    pub struct EmailId(EcoString);

    impl EmailId {
        /// Creates a new [`EmailId`].
        #[inline]
        #[must_use]
        pub fn new(id: &str) -> Self {
            Self(EcoString::from(id))
        }
    }

    impl Deref for EmailId {
        type Target = str;

        #[inline]
        fn deref(&self) -> &Self::Target {
            self.as_ref()
        }
    }

    impl AsRef<str> for EmailId {
        #[inline]
        fn as_ref(&self) -> &str {
            self.0.as_str()
        }
    }

    impl fmt::Display for EmailId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(&self.0, f)
        }
    }

    /// All requisite components and associated data to send an email.
    ///
    /// See [`docs`].
    ///
    /// [`docs`]: https://resend.com/docs/api-reference/emails/send-email#body-parameters
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateEmailBaseOptions {
        /// Sender email address.
        ///
        /// To include a friendly name, use the format:
        ///
        /// `Your Name <sender@domain.com>`
        from: String,
        /// Recipient email address. Max 50.
        to: Vec<String>,
        /// Email subject.
        subject: String,

        /// The HTML version of the message.
        #[serde(skip_serializing_if = "Option::is_none")]
        html: Option<String>,
        /// The plain text version of the message.
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,

        /// Bcc recipient email address.
        #[serde(skip_serializing_if = "Option::is_none")]
        bcc: Option<Vec<String>>,
        /// Cc recipient email address.
        #[serde(skip_serializing_if = "Option::is_none")]
        cc: Option<Vec<String>>,
        /// Reply-to email address.
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_to: Option<Vec<String>>,
        /// Custom headers to add to the email.
        #[serde(skip_serializing_if = "Option::is_none")]
        headers: Option<HashMap<String, String>>,
        /// Filename and content of attachments (max 40mb per email).
        #[serde(skip_serializing_if = "Option::is_none")]
        attachments: Option<Vec<Attachment>>,
        /// Email tags.
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<Tag>>,

        /// Schedule email to be sent later. The date should be in ISO 8601 format
        /// (e.g: `2024-08-05T11:52:01.858Z`).
        #[serde(skip_serializing_if = "Option::is_none")]
        scheduled_at: Option<String>,
    }

    impl CreateEmailBaseOptions {
        /// Creates a new [`CreateEmailBaseOptions`].
        ///
        /// - `from`: Sender email address.
        ///   To include a friendly name, use the format: `Your Name <sender@domain.com>`.
        /// - `to`: Recipient email addresses. Max 50.
        /// - `subject`: Email subject.
        pub fn new<T, A>(from: impl Into<String>, to: T, subject: impl Into<String>) -> Self
        where
            T: IntoIterator<Item = A>,
            A: Into<String>,
        {
            Self {
                from: from.into(),
                to: to.into_iter().map(Into::into).collect(),
                subject: subject.into(),

                html: None,
                text: None,

                bcc: None,
                cc: None,
                reply_to: None,

                headers: None,
                attachments: None,
                tags: None,
                scheduled_at: None,
            }
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

        /// Attaches `bcc` recipient email address.
        #[inline]
        pub fn with_bcc(mut self, address: &str) -> Self {
            let bcc = self.bcc.get_or_insert_with(Vec::new);
            bcc.push(address.to_owned());
            self
        }

        /// Attaches `cc` recipient email address.
        #[inline]
        pub fn with_cc(mut self, address: &str) -> Self {
            let cc = self.cc.get_or_insert_with(Vec::new);
            cc.push(address.to_owned());
            self
        }

        /// Adds another `reply_to` address to the email.
        #[inline]
        pub fn with_reply(mut self, to: &str) -> Self {
            let reply_to = self.reply_to.get_or_insert_with(Vec::new);
            reply_to.push(to.to_owned());
            self
        }

        /// Append multiple `reply_to` addresses to the email.
        #[inline]
        pub fn with_reply_multiple(mut self, to: &[String]) -> Self {
            let reply_to = self.reply_to.get_or_insert_with(Vec::new);
            reply_to.extend_from_slice(to);
            self
        }

        /// Adds or overwrites an email header.
        #[inline]
        pub fn with_header(mut self, name: &str, value: &str) -> Self {
            let headers = self.headers.get_or_insert_with(HashMap::new);
            let _unused = headers.insert(name.to_owned(), value.to_owned());

            self
        }

        /// Adds another attachment.
        ///
        /// Limited to max 40mb per email.
        #[inline]
        pub fn with_attachment(mut self, file: impl Into<Attachment>) -> Self {
            let attachments = self.attachments.get_or_insert_with(Vec::new);
            attachments.push(file.into());
            self
        }

        /// Adds additional email tag.
        #[inline]
        pub fn with_tag(mut self, tag: impl Into<Tag>) -> Self {
            let tags = self.tags.get_or_insert_with(Vec::new);
            tags.push(tag.into());
            self
        }

        /// Schedule email to be sent later. The date should be in ISO 8601 format
        /// (e.g: `2024-08-05T11:52:01.858Z`).
        #[inline]
        pub fn with_scheduled_at(mut self, scheduled_at: &str) -> Self {
            self.scheduled_at = Some(scheduled_at.to_owned());
            self
        }

        // Adds an `Idempotency-Key` header to the request.
        #[inline]
        pub fn with_idempotency_key(self, idempotency_key: &str) -> Idempotent<Self> {
            Idempotent {
                idempotency_key: Some(idempotency_key.to_owned()),
                data: self,
            }
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct CreateEmailResponse {
        /// The ID of the sent email.
        pub id: EmailId,
    }

    /// List of changes to apply to an [`Email`].
    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct UpdateEmailOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        scheduled_at: Option<String>,
    }

    impl UpdateEmailOptions {
        #[inline]
        pub fn new() -> Self {
            Self::default()
        }

        #[inline]
        pub fn with_scheduled_at(mut self, scheduled_at: &str) -> Self {
            self.scheduled_at = Some(scheduled_at.to_owned());
            self
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct UpdateEmailResponse {
        /// Unique identifier for the updated contact.
        pub id: EmailId,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct CancelScheduleResponse {
        /// The ID of the cancelled email.
        pub id: EmailId,
    }

    /// Name and value of the attached [`Email`] tag.
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct Tag {
        /// The name of the email tag. It can only contain ASCII letters (a–z, A–Z), numbers (0–9),
        /// underscores (_), or dashes (-). It can contain no more than 256 characters.
        name: String,
        /// The value of the email tag. It can only contain ASCII letters (a–z, A–Z), numbers (0–9),
        /// underscores (_), or dashes (-). It can contain no more than 256 characters.
        value: String,
    }

    impl Tag {
        /// Creates the new email [`Tag`] with a provided `name`.
        ///
        /// It can only contain ASCII letters (a–z, A–Z), numbers (0–9), underscores (_),
        /// or dashes (-). It can contain no more than 256 characters.
        #[inline]
        pub fn new(name: &str, value: &str) -> Self {
            Self {
                name: name.to_owned(),
                value: value.to_owned(),
            }
        }
    }

    /// Filename and content of the [`CreateEmailBaseOptions`] attachment.
    ///
    /// Limited to max 40mb per email.
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct Attachment {
        /// Content or path of an attached file.
        #[serde(flatten)]
        content_or_path: ContentOrPath,
        /// Name of attached file.
        #[serde(skip_serializing_if = "Option::is_none")]
        filename: Option<String>,
        /// Optional content type for the attachment, if not set will be derived from the filename
        /// property.
        #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
        content_type: Option<String>,
        /// Optional content ID for the attachment, to be used as a reference in the HTML content.
        /// If set, this attachment will be sent as an inline attachment and you can reference it
        /// in the HTML content using the `cid:` prefix.
        #[serde(skip_serializing_if = "Option::is_none")]
        content_id: Option<String>,
    }

    /// Content or path of the [`Attachment`].
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub enum ContentOrPath {
        /// Content of an attached file.
        #[serde(rename = "content")]
        Content(Vec<u8>),
        /// Path where the attachment file is hosted.
        #[serde(rename = "path")]
        Path(String),
    }

    impl Attachment {
        /// Creates a new [`Attachment`] from the content of an attached file.
        #[inline]
        pub const fn from_content(content: Vec<u8>) -> Self {
            Self {
                content_or_path: ContentOrPath::Content(content),
                filename: None,
                content_type: None,
                content_id: None,
            }
        }

        /// Creates a new [`Attachment`] from the path where the attachment file is hosted.
        #[inline]
        pub fn from_path(path: &str) -> Self {
            Self {
                content_or_path: ContentOrPath::Path(path.to_owned()),
                filename: None,
                content_type: None,
                content_id: None,
            }
        }

        /// Adds a filename to the attached file.
        #[inline]
        pub fn with_filename(mut self, filename: &str) -> Self {
            self.filename = Some(filename.to_owned());
            self
        }

        /// Adds a contenent type to the attached file.
        #[inline]
        pub fn with_content_type(mut self, content_type: &str) -> Self {
            self.content_type = Some(content_type.to_owned());
            self
        }

        /// Adds an inline content id to the attached file.
        #[deprecated(
            since = "0.16.1",
            note = "Parameter got internally renamed to just `content_id`. Use `with_content_id` instead."
        )]
        #[inline]
        pub fn with_inline_content_id(mut self, inline_content_id: &str) -> Self {
            self.content_id = Some(inline_content_id.to_owned());
            self
        }

        #[inline]
        pub fn with_content_id(mut self, content_id: &str) -> Self {
            self.content_id = Some(content_id.to_owned());
            self
        }
    }

    impl From<Vec<u8>> for Attachment {
        #[inline]
        fn from(value: Vec<u8>) -> Self {
            Self::from_content(value)
        }
    }

    impl From<&[u8]> for Attachment {
        #[inline]
        fn from(value: &[u8]) -> Self {
            value.to_vec().into()
        }
    }

    /// Received email.
    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct Email {
        /// The ID of the email.
        pub id: EmailId,

        /// Sender email address.
        pub from: String,
        /// Recipient email address.
        pub to: Vec<String>,
        /// The subject line of the email.
        pub subject: String,

        /// The date and time the email was created in ISO8601 format.
        pub created_at: String,
        /// The HTML body of the email.
        pub html: Option<String>,
        /// The plain text body of the email.
        pub text: Option<String>,

        /// The email addresses of the blind carbon copy recipients.
        #[serde(deserialize_with = "parse_nullable_vec")]
        pub bcc: Vec<String>,
        /// The email addresses of the carbon copy recipients.
        #[serde(deserialize_with = "parse_nullable_vec")]
        pub cc: Vec<String>,
        /// The email addresses to which replies should be sent.
        pub reply_to: Option<Vec<String>>,
        /// The status of the email.
        pub last_event: String,

        /// Schedule email to be sent later. The date should be in ISO 8601 format
        /// (e.g: `2024-08-05T11:52:01.858Z`).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub scheduled_at: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct ListBefore {}

    #[derive(Debug, Clone, Copy)]
    pub struct ListAfter {}

    #[derive(Debug, Clone, Copy)]
    pub struct TimeNotSpecified {}

    /// Query parameters for retrieving a list of [`Email`]s.
    ///
    /// ## Example
    ///
    /// ```
    /// # use resend_rs::types::ListEmailOptions;
    /// let list_opts = ListEmailOptions::default()
    ///   .with_limit(3)
    ///   .list_before("71f170f3-826e-47e3-9128-a5958e3b375e");
    /// ```
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct ListEmailOptions<List = TimeNotSpecified> {
        #[serde(skip)]
        list: std::marker::PhantomData<List>,
        /// Number of emails to retrieve. Default is 20, maximum is 100.
        #[serde(skip_serializing_if = "Option::is_none")]
        limit: Option<u8>,
        #[serde(rename = "before")]
        before_id: Option<String>,
        #[serde(rename = "after")]
        after_id: Option<String>,
    }

    impl Default for ListEmailOptions {
        fn default() -> Self {
            Self {
                list: std::marker::PhantomData::<TimeNotSpecified>,
                limit: None,
                before_id: None,
                after_id: None,
            }
        }
    }

    impl<T> ListEmailOptions<T> {
        #[inline]
        pub const fn with_limit(mut self, limit: u8) -> Self {
            self.limit = Some(limit);
            self
        }
    }

    impl ListEmailOptions<TimeNotSpecified> {
        #[inline]
        pub fn list_before(self, email_id: &str) -> ListEmailOptions<ListBefore> {
            ListEmailOptions::<ListBefore> {
                list: std::marker::PhantomData,
                limit: self.limit,
                before_id: Some(email_id.to_string()),
                after_id: None,
            }
        }

        #[inline]
        pub fn list_after(self, email_id: &str) -> ListEmailOptions<ListAfter> {
            ListEmailOptions::<ListAfter> {
                list: std::marker::PhantomData,
                limit: self.limit,
                before_id: None,
                after_id: Some(email_id.to_string()),
            }
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct ListEmailResponse {
        pub has_more: bool,
        pub data: Vec<Email>,
    }
}

/// Turns:
/// - `null` -> `[]`
/// - `["text"]` -> `["text"]`
fn parse_nullable_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_else(Vec::new))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::needless_return)]
mod test {
    use crate::types::{CreateEmailBaseOptions, Email, ListEmailOptions, Tag, UpdateEmailOptions};
    use crate::{test::DebugResult, tests::CLIENT};
    use jiff::{Span, Timestamp, Zoned};

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> DebugResult<()> {
        let from = "Acme <onboarding@resend.dev>";
        let to = ["delivered@resend.dev"];
        let subject = "Hello World!";

        let resend = &*CLIENT;

        // Create
        #[allow(clippy::string_lit_as_bytes)] // False warning
        let email = CreateEmailBaseOptions::new(from, to, subject)
            .with_text("Hello World!")
            .with_attachment("Hello World as file.".as_bytes())
            .with_tag(Tag::new("category", "confirm_email"));

        let email = resend.emails.send(email).await?;

        std::thread::sleep(std::time::Duration::from_secs(1));

        // Get
        let _email = resend.emails.get(&email.id).await?;

        Ok(())
    }

    #[test]
    fn deserialize_test() {
        let email = r#"{
            "object": "email",
            "id": "6757a66c-3a5b-49ee-98cc-fca7a5f423c0",
            "to": [
                "email@gmail.com"
            ],
            "from": "email@gmail.com>",
            "created_at": "2024-07-11 07:49:53.682607+00",
            "subject": "Subject",
            "bcc": null,
            "cc": null,
            "reply_to": null,
            "last_event": "delivered",
            "html": "<div></div>",
            "text": null,
            "scheduled_at": null
        }"#;

        let res = serde_json::from_str::<Email>(email);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(res.cc.is_empty());
        assert!(res.bcc.is_empty());
        assert!(res.text.is_none());

        let email = r#"{
            "object": "email",
            "id": "6757a66c-3a5b-49ee-98cc-fca7a5f423c0",
            "to": [
                "email@gmail.com"
            ],
            "from": "email@gmail.com>",
            "created_at": "2024-07-11 07:49:53.682607+00",
            "subject": "Subject",
            "bcc": ["hello", "world"],
            "cc": ["!"],
            "reply_to": null,
            "last_event": "delivered",
            "html": "<div></div>",
            "text": "Not null",
            "scheduled_at": "2024-08-07 15:15:37+00"
        }"#;

        let res = serde_json::from_str::<Email>(email);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(!res.cc.is_empty());
        assert!(!res.bcc.is_empty());
        assert!(res.text.is_some());
    }

    #[test]
    #[cfg(feature = "blocking")]
    fn all_blocking() -> DebugResult<()> {
        let from = "Acme <onboarding@resend.dev>";
        let to = ["delivered@resend.dev"];
        let subject = "Hello World!";

        let resend = &*CLIENT;
        let email = CreateEmailBaseOptions::new(from, to, subject)
            .with_text("Hello World!")
            .with_tag(Tag::new("category", "confirm_email"));

        let _ = resend.emails.send(email)?;

        std::thread::sleep(std::time::Duration::from_millis(1100));

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn schedule_email() -> DebugResult<()> {
        let now_plus_1h = Zoned::now()
            .checked_add(Span::new().hours(1))
            .expect("Valid date")
            .timestamp()
            .to_string();

        let now_plus_2h = Zoned::now()
            .checked_add(Span::new().hours(2))
            .expect("Valid date")
            .timestamp()
            .to_string();

        let from = "Acme <onboarding@resend.dev>";
        let to = ["delivered@resend.dev"];
        let subject = "Hello World!";

        let resend = &*CLIENT;

        // Create
        let email = CreateEmailBaseOptions::new(from, to, subject)
            .with_text("Hello World!")
            .with_scheduled_at(&now_plus_1h);
        let email = resend.emails.send(email).await?;
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Get
        let email = resend.emails.get(&email.id).await?;
        assert_eq!(email.last_event, "scheduled".to_string());
        assert!(email.scheduled_at.is_some());
        let time = email
            .scheduled_at
            .unwrap()
            .parse::<Timestamp>()
            .expect("Valid timestamp");
        let time_delta = (time - Timestamp::now()).round(jiff::Unit::Hour).unwrap();
        assert_eq!(
            time_delta.compare(Span::new().hours(1)).unwrap(),
            std::cmp::Ordering::Equal
        );

        // Update
        let changes = UpdateEmailOptions::new().with_scheduled_at(&now_plus_2h);
        let email = resend.emails.update(&email.id, changes).await?;
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Get
        let email = resend.emails.get(&email.id).await?;
        assert_eq!(email.last_event, "scheduled".to_string());
        assert!(email.scheduled_at.is_some());
        let time = email
            .scheduled_at
            .unwrap()
            .parse::<Timestamp>()
            .expect("Valid timestamp");
        let time_delta = (time - Timestamp::now()).round(jiff::Unit::Hour).unwrap();
        assert_eq!(
            time_delta.compare(Span::new().hours(2)).unwrap(),
            std::cmp::Ordering::Equal
        );

        // Cancel
        let _cancelled = resend.emails.cancel(&email.id).await?;
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Get again, make sure it was cancelled
        let email = resend.emails.get(&email.id).await?;
        assert_eq!(email.last_event, "canceled".to_string());

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn list_emails() -> DebugResult<()> {
        let resend = &*CLIENT;
        std::thread::sleep(std::time::Duration::from_secs(1));

        let list_opts = ListEmailOptions::default()
            .with_limit(3)
            .list_before("71f170f3-826e-47e3-9128-a5958e3b375e");

        let list = resend.emails.list(list_opts).await?;

        // There now should be more
        assert!(list.has_more);
        // There should be 3 emails due to the limit we set
        assert!(list.data.len() == 3);

        Ok(())
    }
}
