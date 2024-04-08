use std::fmt;

use crate::{Config, Result};
use crate::types::{Email, SendEmailBatchResponse, SendEmailRequest, SendEmailResponse};

/// TODO.
#[derive(Clone)]
pub struct Emails(pub(crate) Config);

impl Emails {
    /// Start sending emails through the Resend Email API.
    ///
    /// <https://resend.com/docs/api-reference/emails/send-email>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn send(&self, email: SendEmailRequest) -> Result<SendEmailResponse> {
        let uri = self.0.base_url.join("/emails")?;
        let key = self.0.api_key.as_str();

        let request = self.0.client.post(uri).bearer_auth(key).json(&email);
        let response = request.send().await?;
        let content = response.json::<SendEmailResponse>().await?;

        Ok(content)
    }

    /// Trigger up to 100 batch emails at once.
    ///
    /// Instead of sending one email per HTTP request, we provide a batching endpoint
    /// that permits you to send up to 100 emails in a single API call.
    ///
    /// <https://resend.com/docs/api-reference/emails/send-batch-emails>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn send_batch<T>(&self, emails: T) -> Result<SendEmailBatchResponse>
        where
            T: IntoIterator<Item=SendEmailRequest> + Send,
    {
        let uri = self.0.base_url.join("/emails/batch")?;
        let key = self.0.api_key.as_str();
        let emails: Vec<_> = emails.into_iter().collect();

        let request = self.0.client.post(uri).bearer_auth(key).json(&emails);
        let response = request.send().await?;
        let content = response.json::<SendEmailBatchResponse>().await?;

        Ok(content)
    }

    /// Retrieve a single email.
    ///
    /// <https://resend.com/docs/api-reference/emails/retrieve-email>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn retrieve(&self, id: &str) -> Result<Email> {
        let path = format!("/emails/{id}");
        let uri = self.0.base_url.join(path.as_str())?;
        let key = self.0.api_key.as_str();

        let request = self.0.client.get(uri).bearer_auth(key);
        let response = request.send().await?;
        let content = response.json::<Email>().await?;

        Ok(content)
    }

    /// Start sending emails through the Resend Email API.
    ///
    /// <https://resend.com/docs/api-reference/emails/send-email>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn send(&self, email: SendEmailRequest) -> Result<SendEmailResponse> {
        let uri = self.0.base_url.join("/emails")?;
        let key = self.0.api_key.as_str();

        let request = self.0.client.post(uri).bearer_auth(key).json(&email);
        let response = request.send()?;
        let content = response.json::<SendEmailResponse>()?;

        Ok(content)
    }

    /// Trigger up to 100 batch emails at once.
    ///
    /// Instead of sending one email per HTTP request, we provide a batching endpoint
    /// that permits you to send up to 100 emails in a single API call.
    ///
    /// <https://resend.com/docs/api-reference/emails/send-batch-emails>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn send_batch<T>(&self, emails: T) -> Result<SendEmailBatchResponse>
        where
            T: IntoIterator<Item=SendEmailRequest> + Send,
    {
        let uri = self.0.base_url.join("/emails/batch")?;
        let key = self.0.api_key.as_str();
        let emails: Vec<_> = emails.into_iter().collect();

        let request = self.0.client.post(uri).bearer_auth(key).json(&emails);
        let response = request.send()?;
        let content = response.json::<SendEmailBatchResponse>()?;

        Ok(content)
    }

    /// Retrieve a single email.
    ///
    /// <https://resend.com/docs/api-reference/emails/retrieve-email>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn retrieve(&self, id: &str) -> Result<Email> {
        let path = format!("/emails/{id}");
        let uri = self.0.base_url.join(path.as_str())?;
        let key = self.0.api_key.as_str();

        let request = self.0.client.get(uri).bearer_auth(key);
        let response = request.send()?;
        let content = response.json::<Email>()?;

        Ok(content)
    }
}

impl fmt::Debug for Emails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

pub mod types {
    use serde::{Deserialize, Serialize};

    /// TODO.
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct SendEmailRequest {
        /// Sender email address.
        /// To include a friendly name, use the format "Your Name <sender@domain.com>".
        pub from: String,
        /// Recipient email address. Max 50.
        pub to: Vec<String>,
        /// Email subject.
        pub subject: String,

        /// The HTML version of the message.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub html: Option<String>,
        /// The plain text version of the message.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub text: Option<String>,

        /// Bcc recipient email address.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bcc: Option<Vec<String>>,
        /// Cc recipient email address.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cc: Option<Vec<String>>,
        /// Reply-to email address.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reply_to: Option<Vec<String>>,
        /// Custom headers to add to the email.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub headers: Option<serde_json::Value>,
        /// Filename and content of attachments (max 40mb per email).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attachments: Option<Vec<Attachment>>,
        /// Email tags.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<Tag>>,
    }

    impl SendEmailRequest {
        /// Creates a new [`SendEmailRequest`].
        #[inline]
        pub fn new(from: String, to: Vec<String>, subject: String) -> Self {
            Self {
                from,
                to,
                subject,

                html: None,
                text: None,

                bcc: None,
                cc: None,
                reply_to: None,

                headers: None,
                attachments: None,
                tags: None,
            }
        }

        #[inline]
        pub fn with_html(mut self, html: &str) -> Self {
            self.html = Some(html.to_string());
            self
        }

        #[inline]
        pub fn with_text(mut self, text: &str) -> Self {
            self.text = Some(text.to_string());
            self
        }

        /// Adds another attachment (max 40mb per email).
        #[inline]
        pub fn with_attachment(mut self, file: Attachment) -> Self {
            self.attachments.get_or_insert_with(Vec::new).push(file);
            self
        }

        /// Adds additional email tag.
        #[inline]
        pub fn with_tag(mut self, tag: Tag) -> Self {
            self.tags.get_or_insert_with(Vec::new).push(tag);
            self
        }
    }

    /// TODO.
    #[derive(Debug, Clone, Deserialize)]
    pub struct SendEmailResponse {
        /// The ID of the sent email.
        pub id: String,
    }

    /// TODO.
    #[derive(Debug, Clone, Deserialize)]
    pub struct SendEmailBatchResponse {
        /// The IDs of the sent emails.
        pub data: Vec<SendEmailResponse>,
    }

    /// Email tag.
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct Tag {
        /// The name of the email tag. It can only contain ASCII letters (a–z, A–Z), numbers (0–9),
        /// underscores (_), or dashes (-). It can contain no more than 256 characters.
        pub name: String,
        /// The value of the email tag. It can only contain ASCII letters (a–z, A–Z), numbers (0–9),
        /// underscores (_), or dashes (-). It can contain no more than 256 characters.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    impl Tag {
        /// Creates the new email [`Tag`] with a provided `name`.
        ///
        /// It can only contain ASCII letters (a–z, A–Z), numbers (0–9), underscores (_),
        /// or dashes (-). It can contain no more than 256 characters.
        #[inline]
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_owned(),
                value: None,
            }
        }

        /// Adds a value to the email tag.
        ///
        /// It can only contain ASCII letters (a–z, A–Z), numbers (0–9), underscores (_),
        /// or dashes (-). It can contain no more than 256 characters.
        #[inline]
        pub fn with_value(mut self, value: &str) -> Self {
            self.value = Some(value.to_owned());
            self
        }
    }

    /// Filename and content of attachments (max 40mb per email).
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct Attachment {
        /// Content or path of an attached file.
        #[serde(flatten)]
        pub content_or_path: ContentOrPath,
        /// Name of attached file.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub filename: Option<String>,
    }

    /// Content or path of an attached file.
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
        pub fn from_content(content: Vec<u8>) -> Self {
            Self {
                content_or_path: ContentOrPath::Content(content),
                filename: None,
            }
        }

        /// Creates a new [`Attachment`] from the path where the attachment file is hosted.
        #[inline]
        pub fn from_path(path: &str) -> Self {
            Self {
                content_or_path: ContentOrPath::Path(path.to_owned()),
                filename: None,
            }
        }

        /// Adds a filename to the attached file.
        #[inline]
        pub fn with_filename(mut self, filename: &str) -> Self {
            self.filename = Some(filename.to_owned());
            self
        }
    }

    /// TODO.
    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct Email {
        /// The type of object.
        pub object: Option<String>,
        /// The ID of the email.
        pub id: Option<String>,

        /// Sender email address.
        pub from: Option<String>,
        /// Recipient email address.
        pub to: Option<Vec<String>>,
        /// The subject line of the email.
        pub subject: Option<String>,
        /// The date and time the email was created.
        pub created_at: Option<String>,

        /// The HTML body of the email.
        pub html: Option<String>,
        /// The plain text body of the email.
        pub text: Option<String>,

        /// The email addresses of the blind carbon copy recipients.
        pub bcc: Option<Vec<String>>,
        /// The email addresses of the carbon copy recipients.
        pub cc: Option<Vec<String>>,
        /// The email addresses to which replies should be sent.
        pub reply_to: Option<Vec<String>>,
        /// The status of the email.
        pub last_event: Option<String>,
    }
}
