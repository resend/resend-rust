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
        pub message_id: String,
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
        pub size: u32,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::needless_return)]
mod test {
    use crate::{list_opts::ListOptions, types::InboundEmail};
    use crate::{
        list_opts::ListResponse,
        test::{CLIENT, DebugResult},
    };

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
        let emails = r#"{
  "object": "list",
  "has_more": true,
  "data": [
    {
      "id": "a39999a6-88e3-48b1-888b-beaabcde1b33",
      "to": ["recipient@example.com"],
      "from": "sender@example.com",
      "created_at": "2025-10-09 14:37:40.951732+00",
      "subject": "Hello World",
      "bcc": [],
      "cc": [],
      "reply_to": [],
      "message_id": "<111-222-333@email.provider.example.com>",
      "attachments": [
        {
          "filename": "example.txt",
          "content_type": "text/plain",
          "content_id": null,
          "content_disposition": "attachment",
          "id": "47e999c7-c89c-4999-bf32-aaaaa1c3ff21",
          "size": 13
        }
      ]
    }
  ]
}"#;

        let res = serde_json::from_str::<ListResponse<InboundEmail>>(emails);
        assert!(res.is_ok());
    }
}
