use crate::{Config, Result};

/// TODO.
#[derive(Debug, Clone)]
pub struct Emails(pub(crate) Config);

impl Emails {
    /// Start sending emails through the Resend Email API.
    ///
    /// <https://resend.com/docs/api-reference/emails/send-email>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn send(&self, email: types::SendEmailRequest) -> Result<types::SendEmailResponse> {
        let uri = "https://api.resend.com/emails";
        let key = self.0.api_key.as_str();

        let request = self.0.client.post(uri).bearer_auth(key).json(&email);
        let response = request.send().await?;
        let content = response.json::<types::SendEmailResponse>().await?;

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
    pub async fn send_batch<T>(&self, emails: T) -> Result<types::SendEmailBatchResponse>
    where
        T: IntoIterator<Item = types::SendEmailRequest> + Send,
    {
        let uri = "https://api.resend.com/emails/batch";
        let key = self.0.api_key.as_str();
        let emails: Vec<_> = emails.into_iter().collect();

        let request = self.0.client.post(uri).bearer_auth(key).json(&emails);
        let response = request.send().await?;
        let content = response.json::<types::SendEmailBatchResponse>().await?;

        Ok(content)
    }

    /// Retrieve a single email.
    ///
    /// <https://resend.com/docs/api-reference/emails/retrieve-email>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn retrieve(&self, id: &str) -> Result<types::Email> {
        let uri = format!("https://api.resend.com/emails/{id}");
        let key = self.0.api_key.as_str();

        let request = self.0.client.get(uri).bearer_auth(key);
        let response = request.send().await?;
        let content = response.json::<types::Email>().await?;

        Ok(content)
    }

    /// Start sending emails through the Resend Email API.
    ///
    /// <https://resend.com/docs/api-reference/emails/send-email>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn send(&self, email: types::SendEmailRequest) -> Result<types::SendEmailResponse> {
        let uri = "https://api.resend.com/emails";
        let key = self.0.api_key.as_str();

        let request = self.0.client.post(uri).bearer_auth(key).json(&email);
        let response = request.send()?;
        let content = response.json::<types::SendEmailResponse>()?;

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
    pub fn send_batch<T>(&self, emails: T) -> Result<types::SendEmailBatchResponse>
    where
        T: IntoIterator<Item = types::SendEmailRequest> + Send,
    {
        let uri = "https://api.resend.com/emails/batch";
        let key = self.0.api_key.as_str();
        let emails: Vec<_> = emails.into_iter().collect();

        let request = self.0.client.post(uri).bearer_auth(key).json(&emails);
        let response = request.send()?;
        let content = response.json::<types::SendEmailBatchResponse>()?;

        Ok(content)
    }

    /// Retrieve a single email.
    ///
    /// <https://resend.com/docs/api-reference/emails/retrieve-email>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn retrieve(&self, id: &str) -> Result<types::Email> {
        let uri = format!("https://api.resend.com/emails/{id}");
        let key = self.0.api_key.as_str();

        let request = self.0.client.get(uri).bearer_auth(key);
        let response = request.send()?;
        let content = response.json::<types::Email>()?;

        Ok(content)
    }
}

pub mod types {
    use serde::{Deserialize, Serialize};

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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attachments: Option<Vec<Attachment>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<Tag>>,
    }

    impl SendEmailRequest {
        /// Creates a new [`SendEmailRequest`].
        pub fn new(from: &str, to: Vec<String>, subject: &str) -> Self {
            Self {
                from: from.to_string(),
                to,
                subject: subject.to_string(),

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

        pub fn with_html(mut self, html: &str) -> Self {
            self.html = Some(html.to_string());
            self
        }

        pub fn with_text(mut self, text: &str) -> Self {
            self.text = Some(text.to_string());
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
        /// Creates a new [`Tag`].
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                value: None,
            }
        }

        pub fn with_value(mut self, value: &str) -> Self {
            self.value = Some(value.to_string());
            self
        }
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct Attachment {
        /// Content or path of an attached file.
        #[serde(flatten)]
        pub content_or_path: ContentOrPath,
        /// Name of attached file.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub filename: Option<String>,
    }

    #[derive(Debug, Clone, Serialize)]
    pub enum ContentOrPath {
        /// Content of an attached file.
        #[serde(rename = "content")]
        Content(String),
        #[serde(rename = "path")]
        Path(String),
    }

    impl Attachment {
        pub fn from_content(content: &str) -> Self {
            Self {
                content_or_path: ContentOrPath::Content(content.to_string()),
                filename: None,
            }
        }

        pub fn from_path(path: &str) -> Self {
            Self {
                content_or_path: ContentOrPath::Path(path.to_owned()),
                filename: None,
            }
        }

        pub fn with_filename(mut self, filename: &str) -> Self {
            self.filename = Some(filename.to_owned());
            self
        }
    }

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
