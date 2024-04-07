use crate::{ResendClientInner, Result};

/// TODO.
#[derive(Debug, Clone)]
pub struct Emails {
    inner: ResendClientInner,
}

impl Emails {
    /// Creates a new [`Emails`].
    pub(crate) fn new(inner: ResendClientInner) -> Self {
        Self { inner }
    }

    /// `<https://resend.com/docs/api-reference/emails/send-email>`
    #[cfg(not(feature = "blocking"))]
    pub async fn send(&self, email: types::SendEmailRequest) -> Result<types::SendEmailResponse> {
        let uri = "https://api.resend.com/emails";
        let key = self.inner.api_key.as_str();

        let request = self.inner.client.get(uri).bearer_auth(key).json(&email);
        let response = request.send().await?;
        let content = response.json::<types::SendEmailResponse>().await?;

        Ok(content)
    }

    /// `<https://resend.com/docs/api-reference/emails/send-batch-emails>`
    #[cfg(not(feature = "blocking"))]
    pub async fn send_batch<T>(&self, emails: T) -> Result<types::SendEmailBatchResponse>
        where
            T: IntoIterator<Item=types::SendEmailRequest> + Send,
    {
        let uri = "https://api.resend.com/emails/batch";
        let key = self.inner.api_key.as_str();
        let emails: Vec<_> = emails.into_iter().collect();

        let request = self.inner.client.get(uri).bearer_auth(key).json(&emails);
        let response = request.send().await?;
        let content = response.json::<types::SendEmailBatchResponse>().await?;

        Ok(content)
    }

    /// `<https://resend.com/docs/api-reference/emails/retrieve-email>`
    #[cfg(not(feature = "blocking"))]
    pub async fn receive(&self, id: &str) -> Result<types::Email> {
        let uri = format!("https://api.resend.com/emails/{id}");
        let key = self.inner.api_key.as_str();

        let request = self.inner.client.get(uri).bearer_auth(key);
        let response = request.send().await?;
        let content = response.json::<types::Email>().await?;

        Ok(content)
    }
}

pub mod types {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Default, Serialize)]
    pub struct SendEmailRequest {
        /// Sender email address. To include a friendly name, use the format "Your Name <sender@domain.com>".
        #[serde(rename = "from")]
        pub from: String,
        #[serde(rename = "to")]
        pub to: Vec<String>,
        /// Email subject.
        #[serde(rename = "subject")]
        pub subject: String,
        /// Bcc recipient email address. For multiple addresses, send as an array of strings.
        #[serde(rename = "bcc", skip_serializing_if = "Option::is_none")]
        pub bcc: Option<String>,
        /// Cc recipient email address. For multiple addresses, send as an array of strings.
        #[serde(rename = "cc", skip_serializing_if = "Option::is_none")]
        pub cc: Option<String>,
        /// Reply-to email address. For multiple addresses, send as an array of strings.
        #[serde(rename = "reply_to", skip_serializing_if = "Option::is_none")]
        pub reply_to: Option<String>,
        /// The HTML version of the message.
        #[serde(rename = "html", skip_serializing_if = "Option::is_none")]
        pub html: Option<String>,
        /// The plain text version of the message.
        #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
        pub text: Option<String>,
        /// Custom headers to add to the email.
        #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
        pub headers: Option<serde_json::Value>,
        #[serde(rename = "attachments", skip_serializing_if = "Option::is_none")]
        pub attachments: Option<Vec<Attachment>>,
        #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<Tag>>,
    }

    impl SendEmailRequest {
        pub fn new(from: &str, to: Vec<String>, subject: &str) -> Self {
            Self {
                from: from.to_string(),
                to,
                subject: subject.to_string(),
                ..Self::default()
            }
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct SendEmailResponse {
        /// The ID of the sent email.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: String,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct SendEmailBatchResponse {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub data: Option<Vec<SendEmailResponse>>,
    }

    #[derive(Debug, Default, Clone, Serialize)]
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
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                value: None,
            }
        }
    }

    #[derive(Debug, Clone, Default, Serialize)]
    pub struct Attachment {
        /// Content of an attached file.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub content: Option<std::path::PathBuf>,
        /// Name of attached file.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub filename: Option<String>,
        /// Path where the attachment file is hosted
        #[serde(skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
    }

    impl Attachment {
        pub fn new() -> Self {
            Self::default()
        }
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    pub struct Email {
        /// The type of object.
        #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
        pub object: Option<String>,
        /// The ID of the email.
        #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
        pub to: Option<Vec<String>>,
        /// The email address of the sender.
        #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
        pub from: Option<String>,
        /// The date and time the email was created.
        #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
        pub created_at: Option<String>,
        /// The subject line of the email.
        #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
        pub subject: Option<String>,
        /// The HTML body of the email.
        #[serde(rename = "html", skip_serializing_if = "Option::is_none")]
        pub html: Option<String>,
        /// The plain text body of the email.
        #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
        pub text: Option<String>,
        /// The email addresses of the blind carbon copy recipients.
        #[serde(rename = "bcc", skip_serializing_if = "Option::is_none")]
        pub bcc: Option<Vec<String>>,
        /// The email addresses of the carbon copy recipients.
        #[serde(rename = "cc", skip_serializing_if = "Option::is_none")]
        pub cc: Option<Vec<String>>,
        /// The email addresses to which replies should be sent.
        #[serde(rename = "reply_to", skip_serializing_if = "Option::is_none")]
        pub reply_to: Option<Vec<String>>,
        /// The status of the email.
        #[serde(rename = "last_event", skip_serializing_if = "Option::is_none")]
        pub last_event: Option<String>,
    }

    impl Email {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
