use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use crate::types::{Email, EmailId, SendEmail};
use crate::{Config, Result};

/// `Resend` APIs for `/emails` endpoints.
#[derive(Clone)]
pub struct EmailsSvc(pub(crate) Arc<Config>);

impl EmailsSvc {
    /// Start sending emails through the `Resend` Email API.
    ///
    /// <https://resend.com/docs/api-reference/emails/send-email>
    #[maybe_async::maybe_async]
    // Reasoning for allow: https://github.com/AntoniosBarotsis/resend-rs/pull/1#issuecomment-2081646115
    #[allow(clippy::needless_pass_by_value)]
    pub async fn send(&self, email: SendEmail) -> Result<EmailId> {
        let request = self.0.build(Method::POST, "/emails");
        let response = self.0.send(request.json(&email)).await?;
        let content = response.json::<types::SendEmailResponse>().await?;

        Ok(content.id)
    }

    /// Trigger up to 100 batch emails at once.
    ///
    /// Instead of sending one email per HTTP request, we provide a batching endpoint
    /// that permits you to send up to 100 emails in a single API call.
    ///
    /// <https://resend.com/docs/api-reference/emails/send-batch-emails>
    #[maybe_async::maybe_async]
    pub async fn send_batch<T>(&self, emails: T) -> Result<Vec<EmailId>>
    where
        T: IntoIterator<Item = SendEmail> + Send,
    {
        let emails: Vec<_> = emails.into_iter().collect();

        let request = self.0.build(Method::POST, "/emails/batch");
        let response = self.0.send(request.json(&emails)).await?;
        let content = response.json::<types::SendEmailBatchResponse>().await?;

        Ok(content.data.into_iter().map(|x| x.id).collect())
    }

    /// Retrieve a single email.
    ///
    /// <https://resend.com/docs/api-reference/emails/retrieve-email>
    #[maybe_async::maybe_async]
    pub async fn retrieve(&self, id: &EmailId) -> Result<Email> {
        let path = format!("/emails/{id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Email>().await?;

        Ok(content)
    }
}

impl fmt::Debug for EmailsSvc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

pub mod types {
    use std::collections::HashMap;
    use std::fmt;

    use ecow::EcoString;
    use serde::{Deserialize, Serialize};

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

    impl AsRef<str> for EmailId {
        #[inline]
        fn as_ref(&self) -> &str {
            self.0.as_str()
        }
    }

    impl fmt::Display for EmailId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Display::fmt(self.as_ref(), f)
        }
    }

    /// All requisite components and associated data to send an email.
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct SendEmail {
        /// Sender email address.
        ///
        /// To include a friendly name, use the format:
        ///
        /// `Your Name <sender@domain.com>`
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
        pub headers: Option<HashMap<String, String>>,
        /// Filename and content of attachments (max 40mb per email).
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attachments: Option<Vec<Attachment>>,
        /// Email tags.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<Tag>>,
    }

    impl SendEmail {
        /// Creates a new [`SendEmail`].
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
        pub fn with_bcc(mut self, address: &str) -> Self {
            let bcc = self.bcc.get_or_insert_with(Vec::new);
            bcc.push(address.to_owned());
            self
        }

        /// Attaches `cc` recipient email address.
        pub fn with_cc(mut self, address: &str) -> Self {
            let cc = self.cc.get_or_insert_with(Vec::new);
            cc.push(address.to_owned());
            self
        }

        /// Adds another `reply_to` address to the email.
        pub fn with_reply(mut self, to: &str) -> Self {
            let reply_to = self.reply_to.get_or_insert_with(Vec::new);
            reply_to.push(to.to_owned());
            self
        }

        /// Adds or overwrites an email header.
        pub fn with_header(mut self, name: &str, value: &str) -> Self {
            let headers = self.headers.get_or_insert_with(HashMap::new);
            let _ = headers.insert(name.to_owned(), value.to_owned());

            self
        }

        /// Adds another attachment.
        ///
        /// Limited to max 40mb per email.
        pub fn with_attachment(mut self, file: impl Into<Attachment>) -> Self {
            let attachments = self.attachments.get_or_insert_with(Vec::new);
            attachments.push(file.into());
            self
        }

        /// Adds additional email tag.
        pub fn with_tag(mut self, tag: impl Into<Tag>) -> Self {
            let tags = self.tags.get_or_insert_with(Vec::new);
            tags.push(tag.into());
            self
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct SendEmailResponse {
        /// The ID of the sent email.
        pub id: EmailId,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct SendEmailBatchResponse {
        /// The IDs of the sent emails.
        pub data: Vec<SendEmailResponse>,
    }

    /// Name and value of the attached [`Email`] tag.
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

    /// Filename and content of the [`SendEmail`] attachment.
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
        pub to: String,
        /// The subject line of the email.
        pub subject: String,

        /// The date and time the email was created in ISO8601 format.
        pub created_at: String,
        /// The HTML body of the email.
        pub html: String,
        /// The plain text body of the email.
        pub text: String,

        /// The email addresses of the blind carbon copy recipients.
        pub bcc: Vec<String>,
        /// The email addresses of the carbon copy recipients.
        pub cc: Vec<String>,
        /// The email addresses to which replies should be sent.
        pub reply_to: Vec<String>,
        /// The status of the email.
        pub last_event: String,
    }
}

#[cfg(test)]
mod test {
    use crate::types::SendEmail;
    use crate::{Client, Result};

    #[tokio::test]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> Result<()> {
        let from = "Acme <onboarding@resend.dev>";
        let to = ["delivered@resend.dev"];
        let subject = "Hello World!";

        let resend = Client::default();
        let email = SendEmail::new(from, to, subject)
            .with_text("Hello World!")
            .with_attachment("Hello World as file.".as_bytes())
            .with_tag("Welcome");

        let _ = resend.emails.send(email).await?;
        Ok(())
    }

    #[test]
    #[cfg(feature = "blocking")]
    fn all_blocking() -> Result<()> {
        let from = "Acme <onboarding@resend.dev>";
        let to = ["delivered@resend.dev"];
        let subject = "Hello World!";

        let resend = Client::default();
        let email = SendEmail::new(from, to, subject)
            .with_text("Hello World!")
            .with_tag("Welcome");

        let _ = resend.emails.send(email)?;
        Ok(())
    }
}
