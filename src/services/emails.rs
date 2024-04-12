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
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
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
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
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
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn retrieve(&self, id: &str) -> Result<Email> {
        let path = format!("/emails/{id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Email>().await?;

        Ok(content)
    }

    /// Start sending emails through the Resend Email API.
    ///
    /// <https://resend.com/docs/api-reference/emails/send-email>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn send(&self, email: SendEmailRequest) -> Result<SendEmailResponse> {
        let request = self.0.build(Method::POST, "/emails");
        let response = self.0.send(request.json(&email))?;
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
        T: IntoIterator<Item = SendEmailRequest> + Send,
    {
        let emails: Vec<_> = emails.into_iter().collect();

        let request = self.0.build(Method::POST, "/emails/batch");
        let response = self.0.send(request.json(&emails))?;
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

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request)?;
        let content = response.json::<Email>()?;

        Ok(content)
    }
}

impl fmt::Debug for Emails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod test {
    use crate::types::SendEmailRequest;
    use crate::{Client, Result};

    #[tokio::test]
    #[cfg(not(feature = "blocking"))]
    async fn send() -> Result<()> {
        let resend = Client::default();

        let from = "Acme <onboarding@resend.dev>".to_owned();
        let to = vec!["delivered@resend.dev".to_owned()];
        let subject = "Hello World".to_owned();

        let email = SendEmailRequest::new(from, to, subject)
            .with_text("Hello World!")
            .with_tag("Welcome");

        let _ = resend.emails.send(email).await?;
        Ok(())
    }
}
