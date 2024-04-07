use crate::{ResendInner, Result};

/// TODO.
#[derive(Debug, Clone)]
pub struct Emails(pub(crate) ResendInner);

impl Emails {
    /// Start sending emails through the Resend Email API.
    ///
    /// <https://resend.com/docs/api-reference/emails/send-email>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn send(&self, email: types::EmailRequest) -> Result<types::EmailResponse> {
        let uri = "https://api.resend.com/emails";
        let key = self.0.api_key.as_str();

        let request = self.0.client.get(uri).bearer_auth(key).json(&email);
        let response = request.send().await?;
        let content = response.json::<types::EmailResponse>().await?;

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
    pub async fn send_batch<T>(&self, emails: T) -> Result<types::EmailBatchResponse>
    where
        T: IntoIterator<Item = types::EmailRequest> + Send,
    {
        let uri = "https://api.resend.com/emails/batch";
        let key = self.0.api_key.as_str();
        let emails: Vec<_> = emails.into_iter().collect();

        let request = self.0.client.get(uri).bearer_auth(key).json(&emails);
        let response = request.send().await?;
        let content = response.json::<types::EmailBatchResponse>().await?;

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
    pub fn send(&self, email: types::EmailRequest) -> Result<crate::emails::types::EmailResponse> {
        todo!()
    }

    /// Trigger up to 100 batch emails at once.
    ///
    /// Instead of sending one email per HTTP request, we provide a batching endpoint
    /// that permits you to send up to 100 emails in a single API call.
    ///
    /// <https://resend.com/docs/api-reference/emails/send-batch-emails>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn send_batch<T>(&self, emails: T) -> Result<types::EmailBatchResponse>
    where
        T: IntoIterator<Item = types::EmailRequest> + Send,
    {
        todo!()
    }

    /// Retrieve a single email.
    ///
    /// <https://resend.com/docs/api-reference/emails/retrieve-email>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn retrieve(&self, id: &str) -> Result<types::Email> {
        todo!()
    }
}

pub mod types {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum OneOrMore<T> {
        One(T),
        Many(Vec<T>),
    }

    #[derive(Debug, Clone, Default, Serialize)]
    pub struct EmailRequest {
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

        /// Bcc recipient email address. For multiple addresses, send as an array of strings.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bcc: Option<String>,
        /// Cc recipient email address. For multiple addresses, send as an array of strings.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cc: Option<String>,
        /// Reply-to email address. For multiple addresses, send as an array of strings.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reply_to: Option<String>,
        /// Custom headers to add to the email.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub headers: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attachments: Option<Vec<Attachment>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<Tag>>,
    }

    impl EmailRequest {
        /// Creates a new [`EmailRequest`].
        pub fn new(from: &str, to: Vec<String>, subject: &str) -> Self {
            Self {
                from: from.to_string(),
                to,
                subject: subject.to_string(),
                ..Self::default()
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
    pub struct EmailResponse {
        /// The ID of the sent email.
        pub id: String,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct EmailBatchResponse {
        /// The IDs of the sent emails.
        pub data: Vec<EmailResponse>,
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
    use crate::types::EmailRequest;
    use crate::Resend;

    fn new() -> Resend {
        let api_key = std::env::var("API_KEY");
        Resend::new(api_key.unwrap().as_str())
    }

    #[tokio::test]
    #[cfg(not(feature = "blocking"))]
    fn send() {
        let email = EmailRequest::default();
        let _ = new().emails.send(email).unwrap();
    }

    #[test]
    #[cfg(feature = "blocking")]
    fn send() {
        let email = EmailRequest::default();
        let _ = new().emails.send(email).unwrap();
    }

    #[tokio::test]
    #[cfg(not(feature = "blocking"))]
    fn send_batch() {}

    #[test]
    #[cfg(feature = "blocking")]
    fn send_batch() {}

    #[tokio::test]
    #[cfg(not(feature = "blocking"))]
    fn retrieve() {}

    #[test]
    #[cfg(feature = "blocking")]
    fn retrieve() {}
}
