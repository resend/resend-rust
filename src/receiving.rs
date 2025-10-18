use std::sync::Arc;

use reqwest::Method;

use crate::{
    Config, Result,
    list_opts::{ListOptions, ListResponse},
    types::{InboundAttachment, InboundEmail},
};

/// `Resend` APIs for `/emails/receiving` endpoints.
#[derive(Clone, Debug)]
pub struct ReceivingSvc(pub(crate) Arc<Config>);

impl ReceivingSvc {
    /// Retrieve a single received email.
    ///
    /// <https://resend.com/docs/api-reference/emails/retrieve-received-email>
    #[maybe_async::maybe_async]
    pub async fn get(&self, email_id: &str) -> Result<InboundEmail> {
        let path = format!("/emails/receiving/{email_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<InboundEmail>().await?;

        Ok(content)
    }

    /// Retrieve a list of received emails for the authenticated user.
    ///
    /// <https://resend.com/docs/api-reference/emails/list-received-emails>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn list<T>(&self, list_opts: ListOptions<T>) -> Result<ListResponse<InboundEmail>> {
        let request = self
            .0
            .build(Method::GET, "/emails/receiving")
            .query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<InboundEmail>>().await?;

        Ok(content)
    }

    /// Retrieve a single attachment from a received email.
    ///
    /// <https://resend.com/docs/api-reference/emails/retrieve-received-email>
    #[maybe_async::maybe_async]
    pub async fn get_attachment(
        &self,
        attachment_id: &str,
        email_id: &str,
    ) -> Result<InboundAttachment> {
        let path = format!("/emails/receiving/{email_id}/attachments/{attachment_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<InboundAttachment>().await?;

        Ok(content)
    }

    /// Retrieve a list of email attachments and their contents.
    ///
    /// <https://resend.com/docs/api-reference/attachments/list-received-email-attachments>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn list_attachments<T>(
        &self,
        email_id: &str,
        list_opts: ListOptions<T>,
    ) -> Result<ListResponse<InboundAttachment>> {
        let path = format!("/emails/receiving/{email_id}/attachments");

        let request = self.0.build(Method::GET, &path).query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<InboundAttachment>>().await?;

        Ok(content)
    }
}

#[allow(unreachable_pub)]
pub mod types {
    use std::collections::HashMap;

    use serde::Deserialize;

    crate::define_id_type!(InboundEmailId);
    crate::define_id_type!(InboundAttatchmentId);

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct InboundEmail {
        pub id: InboundEmailId,
        pub to: Vec<String>,
        pub from: String,
        pub created_at: String,
        pub subject: String,
        #[serde(default)]
        pub bcc: Vec<String>,
        #[serde(default)]
        pub cc: Vec<String>,
        #[serde(default)]
        pub reply_to: Vec<String>,
        pub html: Option<String>,
        pub text: Option<String>,
        #[serde(default)]
        pub headers: HashMap<String, String>,
        #[serde(default)]
        pub attachments: Vec<InboundAttachment>,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct InboundAttachment {
        pub id: InboundAttatchmentId,
        pub filename: String,
        pub content_type: String,
        pub content_id: Option<String>,
        pub content_disposition: String,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::needless_return)]
mod test {
    use crate::test::{CLIENT, DebugResult};
    use crate::{list_opts::ListOptions, types::InboundEmail};

    #[ignore = "At the moment, we can't programmatically send inbound emails and since said inbound emails are only retained for 2 weeks, this cannot be automatically tested."]
    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> DebugResult<()> {
        let resend = &*CLIENT;

        // std::thread::sleep(std::time::Duration::from_secs(1));

        let emails = resend.receiving.list(ListOptions::default()).await?;

        let email_id = &emails.data.first().unwrap().id;

        let _email = resend.receiving.get(email_id).await?;

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
    fn deserialize_test() {
        let email = r#"{
  "object": "email",
  "id": "4ef9a417-02e9-4d39-ad75-9611e0fcc33c",
  "to": ["delivered@resend.dev"],
  "from": "Acme <onboarding@resend.dev>",
  "created_at": "2023-04-03T22:13:42.674981+00:00",
  "subject": "Hello World",
  "html": "Congrats on sending your <strong>first email</strong>!",
  "text": null,
  "bcc": [],
  "cc": [],
  "reply_to": [],
  "message_id": "<example+123>",
  "attachments": [
    {
      "id": "2a0c9ce0-3112-4728-976e-47ddcd16a318",
      "filename": "avatar.png",
      "content_type": "image/png",
      "content_disposition": "inline",
      "content_id": "img001"
    }
  ]
}"#;

        let res = serde_json::from_str::<InboundEmail>(email);
        assert!(res.is_ok());
    }
}
