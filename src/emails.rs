use std::sync::Arc;

use reqwest::Method;
use serde::{Deserialize, Deserializer};

use crate::{
    Config, Result,
    list_opts::{ListOptions, ListResponse},
    types::Attachment,
};
use crate::{
    idempotent::Idempotent,
    types::{
        CancelScheduleResponse, CreateEmailBaseOptions, CreateEmailResponse, Email, EmailMetrics,
        GetEmailMetricsOptions, ShareEmailOptions, ShareEmailResponse, UpdateEmailOptions,
        UpdateEmailResponse,
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

    #[maybe_async::maybe_async]
    pub async fn share(
        &self,
        email_id: &str,
        options: ShareEmailOptions,
    ) -> Result<ShareEmailResponse> {
        let path = format!("/emails/{email_id}/share");

        let request = self.0.build(Method::POST, &path);
        let response = self.0.send(request.json(&options)).await?;
        let content = response.json::<ShareEmailResponse>().await?;

        Ok(content)
    }

    /// Retrieve a list of emails.
    ///
    /// - Default limit: 20
    ///
    /// <https://resend.com/docs/api-reference/emails/list-emails>
    #[maybe_async::maybe_async]
    pub async fn list<T>(&self, list_opts: ListOptions<T>) -> Result<ListResponse<Email>> {
        let request = self.0.build(Method::GET, "/emails").query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<Email>>().await?;

        Ok(content)
    }

    /// Retrieve a single attachment from a sent email.
    ///
    /// <https://resend.com/docs/api-reference/attachments/retrieve-sent-email-attachment>
    #[maybe_async::maybe_async]
    pub async fn get_attachment(&self, email_id: &str, attachment_id: &str) -> Result<Attachment> {
        let path = format!("/emails/{email_id}/attachments/{attachment_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Attachment>().await?;

        Ok(content)
    }

    /// Retrieve a list of email attachments and their contents.
    ///
    /// <https://resend.com/docs/api-reference/attachments/list-sent-email-attachments>
    #[maybe_async::maybe_async]
    pub async fn list_attachments<T>(
        &self,
        email_id: &str,
        list_opts: ListOptions<T>,
    ) -> Result<ListResponse<Attachment>> {
        let path = format!("/emails/{email_id}/attachments");

        let request = self.0.build(Method::GET, &path).query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<Attachment>>().await?;

        Ok(content)
    }

    /// Retrieve email delivery metrics aggregated over a date range.
    ///
    /// This is a beta endpoint and its shape may still change.
    ///
    /// <https://resend.com/docs/api-reference/emails/metrics>
    #[maybe_async::maybe_async]
    pub async fn metrics(&self, options: GetEmailMetricsOptions) -> Result<EmailMetrics> {
        let request = self.0.build(Method::GET, "/emails/metrics").query(&options);
        let response = self.0.send(request).await?;
        let content = response.json::<EmailMetrics>().await?;

        Ok(content)
    }
}

#[allow(unreachable_pub)]
pub mod types {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    use crate::{
        emails::{join_comma, parse_nullable_vec},
        idempotent::Idempotent,
        types::{BroadcastId, DomainId, TemplateId, TopicId},
    };

    crate::define_id_type!(EmailId);
    crate::define_id_type!(AttachmentId);

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
        attachments: Option<Vec<CreateAttachment>>,
        /// Email tags.
        #[serde(skip_serializing_if = "Option::is_none")]
        tags: Option<Vec<Tag>>,
        /// The template to use for the email.
        #[serde(skip_serializing_if = "Option::is_none")]
        template: Option<EmailTemplate>,
        /// The topic ID to receive the email.
        #[serde(skip_serializing_if = "Option::is_none")]
        topic_id: Option<TopicId>,

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
                template: None,
                topic_id: None,
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

        /// Adds an email header.
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
        pub fn with_attachment(mut self, file: impl Into<CreateAttachment>) -> Self {
            let attachments = self.attachments.get_or_insert_with(Vec::new);
            attachments.push(file.into());
            self
        }

        /// Adds multiple attachments.
        ///
        /// Limited to max 40mb per email.
        #[inline]
        pub fn with_attachments(
            mut self,
            new_attachments: impl IntoIterator<Item = impl Into<CreateAttachment>>,
        ) -> Self {
            let attachments = self.attachments.get_or_insert_with(Vec::new);
            attachments.extend(new_attachments.into_iter().map(Into::into));
            self
        }

        /// Adds additional email tag.
        #[inline]
        pub fn with_tag(mut self, tag: impl Into<Tag>) -> Self {
            let tags = self.tags.get_or_insert_with(Vec::new);
            tags.push(tag.into());
            self
        }

        /// Adds the template to use for the email.
        #[inline]
        pub fn with_template(mut self, template: impl Into<EmailTemplate>) -> Self {
            self.template = Some(template.into());
            self
        }

        /// Sets the topic ID to receive the email.
        #[inline]
        pub fn with_topic(mut self, topic_id: &str) -> Self {
            self.topic_id = Some(TopicId::new(topic_id));
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

    #[derive(Debug, Clone, Serialize, Deserialize)]
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

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateEmailResponse {
        /// Unique identifier for the updated contact.
        pub id: EmailId,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CancelScheduleResponse {
        /// The ID of the cancelled email.
        pub id: EmailId,
    }

    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct ShareEmailOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        expires_in: Option<String>,
    }

    impl ShareEmailOptions {
        #[inline]
        pub fn new() -> Self {
            Self::default()
        }

        #[inline]
        pub fn with_expires_in(mut self, expires_in: &str) -> Self {
            self.expires_in = Some(expires_in.to_owned());
            self
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ShareEmailResponse {
        /// The ID of the shared email.
        pub id: EmailId,
        /// The shareable link to the email.
        pub url: String,
    }

    /// Name and value of the attached [`Email`] tag.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub struct CreateAttachment {
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

    impl CreateAttachment {
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

    impl From<Vec<u8>> for CreateAttachment {
        #[inline]
        fn from(value: Vec<u8>) -> Self {
            Self::from_content(value)
        }
    }

    impl From<&[u8]> for CreateAttachment {
        #[inline]
        fn from(value: &[u8]) -> Self {
            value.to_vec().into()
        }
    }

    /// Received email.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Email {
        /// The ID of the email.
        pub id: EmailId,
        /// RFC Message-ID header value for the email.
        pub message_id: Option<String>,

        /// Sender email address.
        pub from: String,
        /// Recipient email address.
        pub to: Vec<String>,
        /// The subject line of the email.
        pub subject: String,

        /// The date and time the email was created.
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
        pub last_event: EmailEvent,

        /// The scheduled send time of the email.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub scheduled_at: Option<String>,
    }

    /// Strongly typed `last_event`.
    ///
    /// <https://resend.com/docs/dashboard/emails/introduction#understand-email-events>
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
    #[serde(rename_all = "snake_case")]
    pub enum EmailEvent {
        /// The recipient's mail server rejected the email.
        ///
        /// <https://resend.com/docs/dashboard/emails/email-bounces>
        Bounced,
        /// The scheduled email was canceled (by user).
        Canceled,
        /// The recipient clicked on a link in the email.
        Clicked,
        /// The email was successfully delivered to the recipient’s mail server, but the recipient marked it as spam.
        Complained,
        /// Resend successfully delivered the email to the recipient’s mail server.
        Delivered,
        /// The email couldn’t be delivered to the recipient’s mail server because a temporary issue occurred. Delivery delays can occur, for example, when the recipient’s inbox is full, or when the receiving email server experiences a transient issue.
        DeliveryDelayed,
        /// The email failed to be sent.
        Failed,
        /// The recipient opened the email.
        Opened,
        /// The email created from Broadcasts or Batches is queued for delivery.
        Queued,
        /// The email is scheduled for delivery.
        Scheduled,
        /// The email was sent successfully.
        Sent,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Attachment {
        pub id: AttachmentId,
        pub filename: Option<String>,
        pub size: u32,
        pub content_type: String,
        pub content_disposition: ContentDisposition,
        pub content_id: Option<String>,
        pub download_url: String,
        pub expires_at: String,
    }

    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum ContentDisposition {
        Inline,
        Attachment,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, PartialEq, Eq)]
    pub struct EmailTemplate {
        pub id: TemplateId,
        pub variables: Option<HashMap<String, serde_json::Value>>,
    }

    impl EmailTemplate {
        pub fn new(id: &str) -> Self {
            Self {
                id: TemplateId::new(id),
                variables: None,
            }
        }

        /// Adds a variable
        pub fn with_variable(mut self, key: &str, value: serde_json::Value) -> Self {
            let variables = self.variables.get_or_insert_with(HashMap::new);
            let _old = variables.insert(key.to_owned(), value);
            self
        }

        /// Adds variables
        pub fn with_variables(mut self, variables: HashMap<String, serde_json::Value>) -> Self {
            let self_variables = self.variables.get_or_insert_with(HashMap::new);
            self_variables.extend(variables);
            self
        }
    }

    /// Query parameters for [`crate::emails::EmailsSvc::metrics`].
    ///
    /// Note that [`GetEmailMetricsOptions::default()`] applies no filters, which means the
    /// server-side defaults are used (last 7 days, `UTC`, daily granularity, all metrics, no
    /// dimensions).
    ///
    /// <https://resend.com/docs/api-reference/emails/metrics>
    #[must_use]
    #[derive(Debug, Clone, Default, Serialize)]
    pub struct GetEmailMetricsOptions {
        /// Start of the date range (ISO 8601 date or datetime).
        #[serde(skip_serializing_if = "Option::is_none")]
        start_date: Option<String>,
        /// End of the date range (ISO 8601 date or datetime).
        #[serde(skip_serializing_if = "Option::is_none")]
        end_date: Option<String>,
        /// IANA timezone, e.g. `America/New_York`. Defaults to `UTC`.
        #[serde(skip_serializing_if = "Option::is_none")]
        timezone: Option<String>,
        /// Bucket size used when [`Dimension::Period`] is requested. Defaults to `daily`.
        #[serde(skip_serializing_if = "Option::is_none")]
        granularity: Option<MetricsGranularity>,
        /// Metrics to compute. Defaults to all metrics.
        #[serde(skip_serializing_if = "Vec::is_empty", serialize_with = "join_comma")]
        metrics: Vec<Metric>,
        /// How to break down the returned data. Defaults to none, in which case only `totals`
        /// is returned.
        #[serde(skip_serializing_if = "Vec::is_empty", serialize_with = "join_comma")]
        dimensions: Vec<Dimension>,
        /// Restrict results to these sending domain IDs. Max 100.
        #[serde(
            rename = "domain_id",
            skip_serializing_if = "Vec::is_empty",
            serialize_with = "join_comma"
        )]
        domain_ids: Vec<DomainId>,
        /// Restrict results to these email IDs. Max 100. Cannot be combined with
        /// [`Dimension::Broadcast`] or `broadcast_id`.
        #[serde(
            rename = "email_id",
            skip_serializing_if = "Vec::is_empty",
            serialize_with = "join_comma"
        )]
        email_ids: Vec<EmailId>,
        /// Restrict results to these broadcast IDs. Max 100. Cannot be combined with
        /// [`Dimension::Email`] or `email_id`.
        #[serde(
            rename = "broadcast_id",
            skip_serializing_if = "Vec::is_empty",
            serialize_with = "join_comma"
        )]
        broadcast_ids: Vec<BroadcastId>,
    }

    impl GetEmailMetricsOptions {
        /// Creates a new [`GetEmailMetricsOptions`] applying no filters.
        #[inline]
        pub fn new() -> Self {
            Self::default()
        }

        /// Sets the start of the date range (ISO 8601 date or datetime).
        #[inline]
        pub fn with_start_date(mut self, start_date: &str) -> Self {
            self.start_date = Some(start_date.to_owned());
            self
        }

        /// Sets the end of the date range (ISO 8601 date or datetime).
        #[inline]
        pub fn with_end_date(mut self, end_date: &str) -> Self {
            self.end_date = Some(end_date.to_owned());
            self
        }

        /// Sets the IANA timezone, e.g. `America/New_York`.
        #[inline]
        pub fn with_timezone(mut self, timezone: &str) -> Self {
            self.timezone = Some(timezone.to_owned());
            self
        }

        /// Sets the bucket size used when [`Dimension::Period`] is requested.
        #[inline]
        pub fn with_granularity(mut self, granularity: MetricsGranularity) -> Self {
            self.granularity = Some(granularity);
            self
        }

        /// Adds a metric to compute.
        #[inline]
        pub fn with_metric(mut self, metric: Metric) -> Self {
            self.metrics.push(metric);
            self
        }

        /// Adds multiple metrics to compute.
        #[inline]
        pub fn with_metrics(mut self, metrics: impl IntoIterator<Item = Metric>) -> Self {
            self.metrics.extend(metrics);
            self
        }

        /// Adds a dimension to break results down by.
        #[inline]
        pub fn with_dimension(mut self, dimension: Dimension) -> Self {
            self.dimensions.push(dimension);
            self
        }

        /// Adds multiple dimensions to break results down by.
        #[inline]
        pub fn with_dimensions(mut self, dimensions: impl IntoIterator<Item = Dimension>) -> Self {
            self.dimensions.extend(dimensions);
            self
        }

        /// Restricts results to an additional sending domain ID.
        #[inline]
        pub fn with_domain_id(mut self, domain_id: &str) -> Self {
            self.domain_ids.push(DomainId::new(domain_id));
            self
        }

        /// Restricts results to additional sending domain IDs.
        #[inline]
        pub fn with_domain_ids<T: AsRef<str>>(
            mut self,
            domain_ids: impl IntoIterator<Item = T>,
        ) -> Self {
            self.domain_ids
                .extend(domain_ids.into_iter().map(|id| DomainId::new(id.as_ref())));
            self
        }

        /// Restricts results to an additional email ID.
        #[inline]
        pub fn with_email_id(mut self, email_id: &str) -> Self {
            self.email_ids.push(EmailId::new(email_id));
            self
        }

        /// Restricts results to additional email IDs.
        #[inline]
        pub fn with_email_ids<T: AsRef<str>>(
            mut self,
            email_ids: impl IntoIterator<Item = T>,
        ) -> Self {
            self.email_ids
                .extend(email_ids.into_iter().map(|id| EmailId::new(id.as_ref())));
            self
        }

        /// Restricts results to an additional broadcast ID.
        #[inline]
        pub fn with_broadcast_id(mut self, broadcast_id: &str) -> Self {
            self.broadcast_ids.push(BroadcastId::new(broadcast_id));
            self
        }

        /// Restricts results to additional broadcast IDs.
        #[inline]
        pub fn with_broadcast_ids<T: AsRef<str>>(
            mut self,
            broadcast_ids: impl IntoIterator<Item = T>,
        ) -> Self {
            self.broadcast_ids.extend(
                broadcast_ids
                    .into_iter()
                    .map(|id| BroadcastId::new(id.as_ref())),
            );
            self
        }
    }

    /// A metric available on the `/emails/metrics` endpoint.
    ///
    /// <https://resend.com/docs/api-reference/emails/metrics>
    #[must_use]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum Metric {
        Received,
        Delivered,
        Complained,
        Suppressed,
        Bounced,
        BouncedTransient,
        BouncedPermanent,
        BouncedUndetermined,
        Opened,
        Clicked,
        Unsubscribed,
        DeliveryDelayed,
        Failed,
        Sent,
        UniqueOpened,
        UniqueClicked,
        DeliveryRate,
        OpenRate,
        ClickRate,
        BounceRate,
        ComplaintRate,
        UnsubscribeRate,
    }

    impl Metric {
        /// The wire representation of this [`Metric`], as used both in the `metrics` query
        /// parameter and in JSON responses.
        #[must_use]
        pub const fn as_str(self) -> &'static str {
            match self {
                Self::Received => "received",
                Self::Delivered => "delivered",
                Self::Complained => "complained",
                Self::Suppressed => "suppressed",
                Self::Bounced => "bounced",
                Self::BouncedTransient => "bounced_transient",
                Self::BouncedPermanent => "bounced_permanent",
                Self::BouncedUndetermined => "bounced_undetermined",
                Self::Opened => "opened",
                Self::Clicked => "clicked",
                Self::Unsubscribed => "unsubscribed",
                Self::DeliveryDelayed => "delivery_delayed",
                Self::Failed => "failed",
                Self::Sent => "sent",
                Self::UniqueOpened => "unique_opened",
                Self::UniqueClicked => "unique_clicked",
                Self::DeliveryRate => "delivery_rate",
                Self::OpenRate => "open_rate",
                Self::ClickRate => "click_rate",
                Self::BounceRate => "bounce_rate",
                Self::ComplaintRate => "complaint_rate",
                Self::UnsubscribeRate => "unsubscribe_rate",
            }
        }
    }

    impl AsRef<str> for Metric {
        #[inline]
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }

    impl Serialize for Metric {
        fn serialize<S>(&self, serializer: S) -> crate::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }

    /// A dimension to break `/emails/metrics` results down by.
    ///
    /// Note that `email` cannot be combined with `broadcast` (server-validated).
    ///
    /// <https://resend.com/docs/api-reference/emails/metrics>
    #[must_use]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum Dimension {
        Period,
        Domain,
        Email,
        Broadcast,
    }

    impl Dimension {
        /// The wire representation of this [`Dimension`], as used both in the `dimensions`
        /// query parameter and in JSON responses.
        #[must_use]
        pub const fn as_str(self) -> &'static str {
            match self {
                Self::Period => "period",
                Self::Domain => "domain",
                Self::Email => "email",
                Self::Broadcast => "broadcast",
            }
        }
    }

    impl AsRef<str> for Dimension {
        #[inline]
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }

    impl Serialize for Dimension {
        fn serialize<S>(&self, serializer: S) -> crate::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }

    /// Bucket size used when [`Dimension::Period`] is requested.
    #[must_use]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum MetricsGranularity {
        Hourly,
        Daily,
        Weekly,
        Monthly,
    }

    /// Response of [`crate::emails::EmailsSvc::metrics`].
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EmailMetrics {
        /// Start of the date range these metrics cover.
        pub start_date: String,
        /// End of the date range these metrics cover.
        pub end_date: String,
        /// The metrics that were computed.
        pub metrics: Vec<Metric>,
        /// The dimensions results are broken down by.
        pub dimensions: Vec<Dimension>,
        /// The bucket size used for the [`Dimension::Period`] dimension.
        pub granularity: MetricsGranularity,
        /// Totals across the whole date range, keyed by metric name.
        pub totals: HashMap<String, f64>,
        /// Per-dimension breakdown. Absent when no `dimensions` were requested.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<Vec<EmailMetricsDataPoint>>,
    }

    /// A single row of [`EmailMetrics::data`].
    ///
    /// Which dimension fields are populated depends on the requested [`Dimension`]s, and which
    /// metric fields are present (in [`EmailMetricsDataPoint::metrics`]) depends on the
    /// requested [`Metric`]s.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EmailMetricsDataPoint {
        /// Present when [`Dimension::Period`] was requested.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub period: Option<String>,
        /// Present when [`Dimension::Domain`] was requested.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub domain_id: Option<DomainId>,
        /// Present when [`Dimension::Domain`] was requested.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub domain_name: Option<String>,
        /// Present when [`Dimension::Email`] was requested.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email_id: Option<EmailId>,
        /// Present when [`Dimension::Broadcast`] was requested.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub broadcast_id: Option<BroadcastId>,
        /// Present when [`Dimension::Broadcast`] was requested.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub broadcast_name: Option<String>,
        /// The requested metric values for this row, keyed by metric name.
        #[serde(flatten)]
        pub metrics: HashMap<String, f64>,
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

/// Serializes a slice as a single comma-joined string, as expected by query parameters like
/// `metrics` or `dimensions` on the `/emails/metrics` endpoint.
fn join_comma<S, T>(items: &[T], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: AsRef<str>,
{
    let joined = items
        .iter()
        .map(AsRef::as_ref)
        .collect::<Vec<_>>()
        .join(",");
    serializer.serialize_str(&joined)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::needless_return)]
mod test {
    #[cfg(not(feature = "blocking"))]
    use crate::{
        list_opts::ListOptions,
        types::{
            CreateAttachment, CreateTemplateOptions, EmailTemplate, ShareEmailOptions,
            UpdateEmailOptions, Variable, VariableType,
        },
    };
    use crate::{
        test::{CLIENT, DebugResult},
        types::{CreateEmailBaseOptions, Email, ShareEmailResponse, Tag},
    };
    #[cfg(not(feature = "blocking"))]
    use jiff::{Span, Timestamp, Zoned};

    use std::collections::HashMap;

    use crate::{
        Config,
        types::{Dimension, EmailMetrics, GetEmailMetricsOptions, Metric, MetricsGranularity},
    };

    /// Builds the request `metrics()` would send and returns its decoded query parameters,
    /// without making any network calls.
    fn built_query(opts: &GetEmailMetricsOptions) -> HashMap<String, String> {
        let config = Config::builder("re_test_key").build();
        let request = config
            .build(reqwest::Method::GET, "/emails/metrics")
            .query(opts)
            .build()
            .unwrap();

        request
            .url()
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect()
    }

    #[tokio_shared_rt::test(shared = true)]
    #[serial_test::serial]
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
            "message_id": "<111-222-333@email.example.com>",
            "to": [
                "email@gmail.com"
            ],
            "from": "email@gmail.com>",
            "created_at": "2024-07-11 07:49:53.682607+00",
            "subject": "Subject",
            "bcc": null,
            "cc": null,
            "reply_to": null,
            "last_event": "delivery_delayed",
            "html": "<div></div>",
            "text": null,
            "scheduled_at": null
        }"#;

        let res = serde_json::from_str::<Email>(email);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.message_id.unwrap(), "<111-222-333@email.example.com>");
        assert!(res.cc.is_empty());
        assert!(res.bcc.is_empty());
        assert!(res.text.is_none());

        let email = r#"{
            "object": "email",
            "id": "6757a66c-3a5b-49ee-98cc-fca7a5f423c0",
            "message_id": "<111-222-333@email.example.com>",
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
    fn parse_share_email_response_test() {
        let data = r#"{
            "object": "email",
            "id": "6757a66c-3a5b-49ee-98cc-fca7a5f423c0",
            "url": "https://resend.com/share/6757a66c-3a5b-49ee-98cc-fca7a5f423c0"
        }"#;

        let _parsed = serde_json::from_str::<ShareEmailResponse>(data).expect("Parsing failed");
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

        let _email = resend.emails.send(email)?;

        std::thread::sleep(std::time::Duration::from_millis(1100));

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[serial_test::serial]
    #[cfg(not(feature = "blocking"))]
    async fn schedule_email() -> DebugResult<()> {
        use crate::emails::types::EmailEvent;

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
        std::thread::sleep(std::time::Duration::from_secs(5));

        // Get
        let email = resend.emails.get(&email.id).await?;
        assert_eq!(email.last_event, EmailEvent::Scheduled);
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
        assert_eq!(email.last_event, EmailEvent::Scheduled);
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
        assert_eq!(email.last_event, EmailEvent::Canceled);

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[serial_test::serial]
    #[cfg(not(feature = "blocking"))]
    async fn share_email() -> DebugResult<()> {
        let from = "Acme <onboarding@resend.dev>";
        let to = ["delivered@resend.dev"];
        let subject = "Hello World!";

        let resend = &*CLIENT;

        let email = CreateEmailBaseOptions::new(from, to, subject).with_text("Hello World!");
        let email = resend.emails.send(email).await?;
        std::thread::sleep(std::time::Duration::from_secs(1));

        let shared = resend
            .emails
            .share(&email.id, ShareEmailOptions::new())
            .await?;
        assert_eq!(shared.id, email.id);
        assert!(!shared.url.is_empty());
        std::thread::sleep(std::time::Duration::from_secs(1));

        let shared = resend
            .emails
            .share(&email.id, ShareEmailOptions::new().with_expires_in("10m"))
            .await?;
        assert_eq!(shared.id, email.id);
        assert!(!shared.url.is_empty());
        std::thread::sleep(std::time::Duration::from_secs(1));

        let shared = resend
            .emails
            .share(&email.id, ShareEmailOptions::new().with_expires_in("72h"))
            .await;
        assert!(shared.is_err());
        std::thread::sleep(std::time::Duration::from_secs(1));

        let shared = resend
            .emails
            .share(
                &email.id,
                ShareEmailOptions::new().with_expires_in("not-a-duration"),
            )
            .await;
        assert!(matches!(shared, Err(crate::Error::Resend(_))));
        std::thread::sleep(std::time::Duration::from_secs(1));

        let shared = resend
            .emails
            .share(
                "00000000-0000-0000-0000-000000000000",
                ShareEmailOptions::new(),
            )
            .await;
        assert!(matches!(shared, Err(crate::Error::Resend(_))));

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[serial_test::serial]
    #[cfg(not(feature = "blocking"))]
    async fn list_emails() -> DebugResult<()> {
        let resend = &*CLIENT;
        std::thread::sleep(std::time::Duration::from_secs(1));

        let list_opts = ListOptions::default()
            .with_limit(3)
            .list_before("71f170f3-826e-47e3-9128-a5958e3b375e");

        let list = resend.emails.list(list_opts).await?;

        // There now should be more
        assert!(list.has_more);
        // There should be 3 emails due to the limit we set
        assert_eq!(list.data.len(), 3);

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[serial_test::serial]
    #[cfg(not(feature = "blocking"))]
    async fn attachments() -> DebugResult<()> {
        let resend = &*CLIENT;
        std::thread::sleep(std::time::Duration::from_secs(1));

        let attachment = CreateAttachment::from_content(include_bytes!("../README.md").to_vec())
            .with_filename("README.md");

        let from = "Acme <onboarding@resend.dev>";
        let to = ["delivered@resend.dev"];
        let subject = "Hello World!";

        let email = CreateEmailBaseOptions::new(from, to, subject)
            .with_attachment(attachment)
            .with_text("Hello World!");

        let email = resend.emails.send(email).await?;
        let email_id = &email.id;
        std::thread::sleep(std::time::Duration::from_secs(1));

        let attachments = resend
            .emails
            .list_attachments(email_id, ListOptions::default())
            .await?;
        assert_eq!(attachments.data.len(), 1);
        let attachment_id = &attachments.data.first().unwrap().id;

        let _attachment = resend
            .emails
            .get_attachment(email_id, attachment_id)
            .await?;

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[serial_test::serial]
    #[cfg(not(feature = "blocking"))]
    async fn template() -> DebugResult<()> {
        use std::collections::HashMap;

        let resend = &*CLIENT;
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Create template
        let name = "welcome-email";
        let html = "<strong>Hey, {{{NAME}}}, you are {{{AGE}}} years old.</strong>";
        let variables = [
            Variable::new("NAME", VariableType::String).with_fallback("user"),
            Variable::new("AGE", VariableType::Number).with_fallback(25),
            Variable::new("OPTIONAL_VARIABLE", VariableType::String).with_fallback(None::<String>),
        ];
        let opts = CreateTemplateOptions::new(name, html).with_variables(&variables);
        let template = resend.templates.create(opts).await?;
        std::thread::sleep(std::time::Duration::from_secs(2));
        let template = resend.templates.publish(&template.id).await?;
        std::thread::sleep(std::time::Duration::from_secs(2));
        let mut variables = HashMap::<String, serde_json::Value>::new();
        let _added = variables.insert("NAME".to_string(), serde_json::json!("Tony"));
        let _added = variables.insert("AGE".to_string(), serde_json::json!(25));

        let template = EmailTemplate::new(&template.id).with_variables(variables);
        let template_id = &template.id.clone();

        // Create email
        let from = "Acme <onboarding@resend.dev>";
        let to = ["delivered@resend.dev"];
        let subject = "hello world";

        let email = CreateEmailBaseOptions::new(from, to, subject).with_template(template);

        let _email = resend.emails.send(email).await?;
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Delete template
        let deleted = resend.templates.delete(template_id).await?;
        assert!(deleted.deleted);

        Ok(())
    }

    #[test]
    fn metrics_query_no_options() {
        let query = built_query(&GetEmailMetricsOptions::default());
        assert!(query.is_empty());
    }

    #[test]
    fn metrics_query_each_dimension() {
        for (dimension, expected) in [
            (Dimension::Period, "period"),
            (Dimension::Domain, "domain"),
            (Dimension::Email, "email"),
            (Dimension::Broadcast, "broadcast"),
        ] {
            let opts = GetEmailMetricsOptions::new().with_dimension(dimension);
            let query = built_query(&opts);
            assert_eq!(query.get("dimensions").map(String::as_str), Some(expected));
        }
    }

    #[test]
    fn metrics_query_multiple_dimensions() {
        // `email` + `broadcast` is server-validated, not client-validated - the SDK allows it.
        let opts = GetEmailMetricsOptions::new()
            .with_dimensions([Dimension::Period, Dimension::Broadcast]);
        let query = built_query(&opts);
        assert_eq!(
            query.get("dimensions").map(String::as_str),
            Some("period,broadcast")
        );
    }

    #[test]
    fn metrics_query_domain_id_filter() {
        let single = GetEmailMetricsOptions::new().with_domain_id("d1");
        let query = built_query(&single);
        assert_eq!(query.get("domain_id").map(String::as_str), Some("d1"));

        let multiple = GetEmailMetricsOptions::new().with_domain_ids(["d1", "d2", "d3"]);
        let query = built_query(&multiple);
        assert_eq!(query.get("domain_id").map(String::as_str), Some("d1,d2,d3"));
    }

    #[test]
    fn metrics_query_email_id_filter() {
        let single = GetEmailMetricsOptions::new().with_email_id("e1");
        let query = built_query(&single);
        assert_eq!(query.get("email_id").map(String::as_str), Some("e1"));

        let multiple = GetEmailMetricsOptions::new().with_email_ids(["e1", "e2"]);
        let query = built_query(&multiple);
        assert_eq!(query.get("email_id").map(String::as_str), Some("e1,e2"));
    }

    #[test]
    fn metrics_query_broadcast_id_filter() {
        let single = GetEmailMetricsOptions::new().with_broadcast_id("b1");
        let query = built_query(&single);
        assert_eq!(query.get("broadcast_id").map(String::as_str), Some("b1"));

        let multiple = GetEmailMetricsOptions::new().with_broadcast_ids(["b1", "b2"]);
        let query = built_query(&multiple);
        assert_eq!(query.get("broadcast_id").map(String::as_str), Some("b1,b2"));
    }

    #[test]
    fn metrics_query_metrics_passed_through() {
        let single = GetEmailMetricsOptions::new().with_metric(Metric::Delivered);
        let query = built_query(&single);
        assert_eq!(query.get("metrics").map(String::as_str), Some("delivered"));

        let multiple =
            GetEmailMetricsOptions::new().with_metrics([Metric::Delivered, Metric::Opened]);
        let query = built_query(&multiple);
        assert_eq!(
            query.get("metrics").map(String::as_str),
            Some("delivered,opened")
        );
    }

    #[test]
    fn metrics_query_granularity_and_timezone_passed_through() {
        let opts = GetEmailMetricsOptions::new()
            .with_start_date("2026-07-01")
            .with_end_date("2026-07-08")
            .with_timezone("America/New_York")
            .with_granularity(MetricsGranularity::Weekly);

        let query = built_query(&opts);
        assert_eq!(
            query.get("start_date").map(String::as_str),
            Some("2026-07-01")
        );
        assert_eq!(
            query.get("end_date").map(String::as_str),
            Some("2026-07-08")
        );
        assert_eq!(
            query.get("timezone").map(String::as_str),
            Some("America/New_York")
        );
        assert_eq!(query.get("granularity").map(String::as_str), Some("weekly"));
    }

    #[test]
    fn deserialize_metrics_response_with_data() {
        let json = r#"{
            "object": "metrics",
            "start_date": "2026-07-01T00:00:00.000Z",
            "end_date": "2026-07-08T00:00:00.000Z",
            "metrics": ["delivered", "opened"],
            "dimensions": ["period", "broadcast"],
            "granularity": "daily",
            "totals": { "delivered": 100, "opened": 40 },
            "data": [
                {
                    "period": "2026-07-01",
                    "broadcast_id": "5c9c5f21-3b3a-4f0a-8f6b-3f2d1e6f6c9a",
                    "broadcast_name": "July Newsletter",
                    "delivered": 10,
                    "opened": 4
                }
            ]
        }"#;

        let metrics: EmailMetrics = serde_json::from_str(json).unwrap();

        assert_eq!(metrics.start_date, "2026-07-01T00:00:00.000Z");
        assert_eq!(metrics.end_date, "2026-07-08T00:00:00.000Z");
        assert_eq!(metrics.metrics, vec![Metric::Delivered, Metric::Opened]);
        assert_eq!(
            metrics.dimensions,
            vec![Dimension::Period, Dimension::Broadcast]
        );
        assert_eq!(metrics.granularity, MetricsGranularity::Daily);
        assert_eq!(metrics.totals.get("delivered"), Some(&100.0));
        assert_eq!(metrics.totals.get("opened"), Some(&40.0));

        let data = metrics
            .data
            .expect("data present when dimensions requested");
        assert_eq!(data.len(), 1);

        let row = data.first().expect("one data row");
        assert_eq!(row.period.as_deref(), Some("2026-07-01"));
        assert!(row.domain_id.is_none());
        assert!(row.email_id.is_none());
        assert_eq!(
            row.broadcast_id.as_deref(),
            Some("5c9c5f21-3b3a-4f0a-8f6b-3f2d1e6f6c9a")
        );
        assert_eq!(row.broadcast_name.as_deref(), Some("July Newsletter"));
        assert_eq!(row.metrics.get("delivered"), Some(&10.0));
        assert_eq!(row.metrics.get("opened"), Some(&4.0));
    }

    #[test]
    fn deserialize_metrics_response_without_dimensions() {
        let json = r#"{
            "object": "metrics",
            "start_date": "2026-07-01T00:00:00.000Z",
            "end_date": "2026-07-08T00:00:00.000Z",
            "metrics": ["delivered"],
            "dimensions": [],
            "granularity": "daily",
            "totals": { "delivered": 100 }
        }"#;

        let metrics: EmailMetrics = serde_json::from_str(json).unwrap();

        assert!(metrics.dimensions.is_empty());
        assert!(metrics.data.is_none());
    }
}
