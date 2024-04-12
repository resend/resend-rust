use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use crate::types::{Email, SendEmailBatchResponse};
use crate::types::{SendEmailRequest, SendEmailResponse};
use crate::{Config, Result};

/// `Resend` APIs for `METHOD /emails` endpoints.
#[derive(Clone)]
pub struct Emails(pub(crate) Arc<Config>);

impl Emails {
    /// Start sending emails through the Resend Email API.
    ///
    /// <https://resend.com/docs/api-reference/emails/send-email>
    #[maybe_async::maybe_async]
    pub async fn send(&self, email: SendEmailRequest) -> Result<SendEmailResponse> {
        let request = self.0.build(Method::POST, "/emails");
        let response = self.0.send(request.json(&email)).await?;
        let content = response.json::<SendEmailResponse>().await?;

        Ok(content)
    }

    /// Trigger up to 100 batch emails at once.
    ///
    /// Instead of sending one email per HTTP request, we provide a batching endpoint
    /// that permits you to send up to 100 emails in a single API call.
    ///
    /// <https://resend.com/docs/api-reference/emails/send-batch-emails>
    #[maybe_async::maybe_async]
    pub async fn send_batch<T>(&self, emails: T) -> Result<SendEmailBatchResponse>
    where
        T: IntoIterator<Item = SendEmailRequest> + Send,
    {
        let emails: Vec<_> = emails.into_iter().collect();

        let request = self.0.build(Method::POST, "/emails/batch");
        let response = self.0.send(request.json(&emails)).await?;
        let content = response.json::<SendEmailBatchResponse>().await?;

        Ok(content)
    }

    /// Retrieve a single email.
    ///
    /// <https://resend.com/docs/api-reference/emails/retrieve-email>
    #[maybe_async::maybe_async]
    pub async fn retrieve(&self, id: &str) -> Result<Email> {
        let path = format!("/emails/{id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Email>().await?;

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

        /// Adds the HTML version of the message.
        #[inline]
        pub fn with_html(mut self, html: &str) -> Self {
            self.html = Some(html.to_string());
            self
        }

        /// Adds the plain text version of the message.
        #[inline]
        pub fn with_text(mut self, text: &str) -> Self {
            self.text = Some(text.to_string());
            self
        }

        /// Attaches additional `reply_to` address to the email.
        #[inline]
        pub fn with_reply(mut self, to: &str) -> Self {
            let to = to.to_owned();
            self.reply_to.get_or_insert_with(Vec::new).push(to);
            self
        }

        /// Attaches additional header to the email.
        #[inline]
        pub fn with_header(mut self) -> Self {
            todo!()
        }

        /// Adds another attachment (max 40mb per email).
        #[inline]
        pub fn with_attachment(mut self, file: Attachment) -> Self {
            self.attachments.get_or_insert_with(Vec::new).push(file);
            self
        }

        /// Adds additional email tag.
        #[inline]
        pub fn with_tag(mut self, tag: impl Into<Tag>) -> Self {
            self.tags.get_or_insert_with(Vec::new).push(tag.into());
            self
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct SendEmailResponse {
        /// The ID of the sent email.
        pub id: String,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct SendEmailBatchResponse {
        /// The IDs of the sent emails.
        pub data: Vec<SendEmailResponse>,
    }

    /// Name and value of the attached email tag.
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

    impl<T: AsRef<str>> From<T> for Tag {
        fn from(value: T) -> Self {
            Self::new(value.as_ref())
        }
    }

    /// Filename and content of attachment.
    ///
    /// Limited to max 40mb per email.
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

#[cfg(test)]
mod test {
    use crate::types::SendEmailRequest;
    use crate::{Client, Result};

    #[tokio_macros::test]
    #[cfg(not(feature = "blocking"))]
    async fn send() -> Result<()> {
        let from = "Acme <onboarding@resend.dev>".to_owned();
        let to = vec!["delivered@resend.dev".to_owned()];
        let subject = "Hello World".to_owned();

        let resend = Client::default();
        let email = SendEmailRequest::new(from, to, subject)
            .with_text("Hello World!")
            .with_tag("Welcome");

        let _ = resend.emails.send(email).await?;
        Ok(())
    }

    #[test]
    #[cfg(feature = "blocking")]
    fn send_blocking() -> Result<()> {
        let from = "Acme <onboarding@resend.dev>".to_owned();
        let to = vec!["delivered@resend.dev".to_owned()];
        let subject = "Hello World".to_owned();

        let resend = Client::default();
        let email = SendEmailRequest::new(from, to, subject)
            .with_text("Hello World!")
            .with_tag("Welcome");

        let _ = resend.emails.send(email)?;
        Ok(())
    }
}
