use std::sync::Arc;

use reqwest::Method;

use crate::{
    Config, Result,
    batch::types::BatchValidation,
    emails::types::CreateEmailBaseOptions,
    idempotent::Idempotent,
    types::{CreateEmailResponse, SendEmailBatchPermissiveResponse},
};

/// `Resend` APIs for `/emails` endpoints.
#[derive(Clone, Debug)]
pub struct BatchSvc(pub(crate) Arc<Config>);

impl BatchSvc {
    /// Trigger up to 100 batch emails at once.
    ///
    /// Instead of sending one email per HTTP request, we provide a batching endpoint
    /// that permits you to send up to 100 emails in a single API call.
    ///
    /// Each [`CreateEmailBaseOptions`] in the batch supports `scheduled_at`, `tags`,
    /// and `attachments`, in addition to the other send-email body parameters.
    ///
    /// <https://resend.com/docs/api-reference/emails/send-batch-emails>
    #[maybe_async::maybe_async]
    pub async fn send<T>(
        &self,
        emails: impl Into<Idempotent<T>>,
    ) -> Result<Vec<CreateEmailResponse>>
    where
        T: IntoIterator<Item = CreateEmailBaseOptions> + Send,
    {
        Ok(self
            .send_with_batch_validation(emails, BatchValidation::default())
            .await?
            .data)
    }

    /// The same as [`BatchSvc::send`] but allows you to specify a [`BatchValidation`] mode.
    #[maybe_async::maybe_async]
    pub async fn send_with_batch_validation<T>(
        &self,
        emails: impl Into<Idempotent<T>>,
        batch_validation: BatchValidation,
    ) -> Result<SendEmailBatchPermissiveResponse>
    where
        T: IntoIterator<Item = CreateEmailBaseOptions> + Send,
    {
        let emails: Idempotent<T> = emails.into();

        let mut request = self.0.build(Method::POST, "/emails/batch");

        request = request.header("x-batch-validation", batch_validation.to_string());

        if let Some(ref idempotency_key) = emails.idempotency_key {
            request = request.header("Idempotency-Key", idempotency_key);
        }

        let emails: Vec<_> = emails.data.into_iter().collect();

        let response = self.0.send(request.json(&emails)).await?;
        let content = response.json::<SendEmailBatchPermissiveResponse>().await?;

        Ok(content)
    }
}

#[allow(unreachable_pub)]
pub mod types {
    use serde::{Deserialize, Serialize};

    use crate::types::CreateEmailResponse;

    /// Batch validation modes control how emails are validated in batch sending.
    #[must_use]
    #[derive(Default, Debug, Copy, Clone)]
    pub enum BatchValidation {
        /// Strict mode (default)
        ///
        /// Strict mode only sends the batch if all emails in the batch request are valid.
        /// - Atomic behavior: if any email in the batch fails validation, the entire batch is rejected
        /// - Error details: only the validation error causing the failure is returned
        #[default]
        Strict,
        // Permissive mode processes all emails, allowing for partial success.
        Permissive,
    }

    impl std::fmt::Display for BatchValidation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Strict => write!(f, "strict"),
                Self::Permissive => write!(f, "permissive"),
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SendEmailBatchResponse {
        /// The IDs of the sent emails.
        pub data: Vec<CreateEmailResponse>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SendEmailBatchPermissiveResponse {
        /// The IDs of the sent emails.
        pub data: Vec<CreateEmailResponse>,
        /// Array of objects for emails which could not be created due to validation errors.
        #[serde(default)]
        pub errors: Vec<PermissiveBatchErrors>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PermissiveBatchErrors {
        /// Index of the email in the batch request
        pub index: i32,
        /// Error message identifying the validation error
        pub message: String,
    }
}

#[cfg(test)]
mod test {
    use crate::test::{CLIENT, DebugResult};
    use crate::types::{
        BatchValidation, CreateAttachment, CreateEmailBaseOptions, CreateTemplateOptions,
        EmailEvent, EmailTemplate, Tag, Variable, VariableType,
    };

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    #[allow(clippy::unwrap_used, clippy::indexing_slicing)]
    async fn strict_error() -> DebugResult<()> {
        let resend = &*CLIENT;
        std::thread::sleep(std::time::Duration::from_secs(1));

        let emails = vec![
            CreateEmailBaseOptions::new(
                "Acme <onboarding@resend.dev>",
                vec!["delivered@resend.dev"],
                "hello world",
            )
            .with_html("<h1>it works!</h1>"),
            CreateEmailBaseOptions::new(
                "Acme <onboarding@resend.dev>",
                vec!["NOTantosnis.barotsis@gmail.com"],
                "world hello",
            )
            .with_html("<p>it works!</p>"),
        ];

        let emails = resend
            .batch
            .send_with_batch_validation(emails, BatchValidation::Strict)
            .await;

        // This should be a "global" error because we are in strict mode
        assert!(emails.is_err());

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    #[allow(clippy::unwrap_used, clippy::indexing_slicing)]
    async fn permissive_error() -> DebugResult<()> {
        let resend = &*CLIENT;
        std::thread::sleep(std::time::Duration::from_secs(1));

        let emails = vec![
            CreateEmailBaseOptions::new(
                "Acme <onboarding@resend.dev>",
                vec!["delivered@resend.dev"],
                "hello world",
            )
            .with_html("<h1>it works!</h1>"),
            CreateEmailBaseOptions::new(
                "Acme <onboarding@resend.dev>",
                vec!["someotheremail@gmail.com"],
                "world hello",
            )
            .with_html("<p>it works!</p>"),
        ];

        let emails = resend
            .batch
            .send_with_batch_validation(emails, BatchValidation::Permissive)
            .await;

        // This should not be a "global" error because we are in permissive mode
        assert!(emails.is_ok());
        let emails = emails.unwrap();

        // There should be one error but apparently the errors array is empty
        // check with a get instead
        std::thread::sleep(std::time::Duration::from_secs(4));
        let failed_id = &emails.data[1].id;
        let status = resend.emails.get(failed_id).await?;
        assert_eq!(status.last_event, EmailEvent::Failed);

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    #[allow(clippy::unwrap_used, clippy::indexing_slicing)]
    async fn permissive_ok() -> DebugResult<()> {
        let resend = &*CLIENT;
        std::thread::sleep(std::time::Duration::from_secs(1));

        let emails = vec![
            CreateEmailBaseOptions::new(
                "Acme <onboarding@resend.dev>",
                vec!["delivered@resend.dev"],
                "hello world",
            )
            .with_html("<h1>it works!</h1>"),
            CreateEmailBaseOptions::new(
                "Acme <onboarding@resend.dev>",
                vec!["delivered@resend.dev"],
                "world hello",
            )
            .with_html("<p>it works!</p>"),
        ];

        let emails = resend
            .batch
            .send_with_batch_validation(emails, BatchValidation::Permissive)
            .await;

        // This should be all ok
        assert!(emails.is_ok());
        let emails = emails.unwrap();

        // There should be no errors
        assert!(emails.errors.is_empty());

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    #[allow(clippy::unwrap_used, clippy::indexing_slicing)]
    async fn strict_ok() -> DebugResult<()> {
        let resend = &*CLIENT;
        std::thread::sleep(std::time::Duration::from_secs(1));

        let emails = vec![
            CreateEmailBaseOptions::new(
                "Acme <onboarding@resend.dev>",
                vec!["delivered@resend.dev"],
                "hello world",
            )
            .with_html("<h1>it works!</h1>"),
            CreateEmailBaseOptions::new(
                "Acme <onboarding@resend.dev>",
                vec!["delivered@resend.dev"],
                "world hello",
            )
            .with_html("<p>it works!</p>"),
        ];

        let emails = resend.batch.send(emails).await;

        // This should be all ok
        assert!(emails.is_ok());
        let _emails = emails.unwrap();

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
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

        let mut variables1 = HashMap::<String, serde_json::Value>::new();
        let _added = variables1.insert("NAME".to_string(), serde_json::json!("Tony"));
        let _added = variables1.insert("AGE".to_string(), serde_json::json!(25));

        let template1 = EmailTemplate::new(&template.id).with_variables(variables1);
        let template_id = &template1.id.clone();

        let mut variables2 = HashMap::<String, serde_json::Value>::new();
        let _added = variables2.insert("NAME".to_string(), serde_json::json!("Not Tony"));
        let _added = variables2.insert("AGE".to_string(), serde_json::json!(42));

        let template2 = EmailTemplate::new(&template.id).with_variables(variables2);
        let _ = &template2.id.clone();

        // Create email
        let from = "Acme <onboarding@resend.dev>";
        let to = ["delivered@resend.dev"];
        let subject = "hello world";

        let emails = vec![
            CreateEmailBaseOptions::new(from, to, subject).with_template(template1),
            CreateEmailBaseOptions::new(from, to, subject).with_template(template2),
        ];

        let _email = resend.batch.send(emails).await?;
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Delete template
        let deleted = resend.templates.delete(template_id).await?;
        assert!(deleted.deleted);

        Ok(())
    }

    #[test]
    fn serialize_schedule_tags_attachments() {
        use serde_json::json;

        let email = CreateEmailBaseOptions::new(
            "Acme <onboarding@resend.dev>",
            ["delivered@resend.dev"],
            "hello world",
        )
        .with_html("<h1>it works!</h1>")
        .with_tag(Tag::new("category", "confirm_email"))
        .with_scheduled_at("2025-09-25T11:52:01.858Z")
        .with_attachment(
            CreateAttachment::from_content(b"hello".to_vec()).with_filename("test.txt"),
        );

        let emails = vec![email];
        let value = serde_json::to_value(&emails).unwrap();

        let email = &value[0];
        assert_eq!(
            email["tags"],
            json!([{"name": "category", "value": "confirm_email"}])
        );
        assert_eq!(email["scheduled_at"], json!("2025-09-25T11:52:01.858Z"));
        assert!(email["attachments"].is_array());
        assert_eq!(email["attachments"][0]["filename"], "test.txt");
    }

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn tags() -> DebugResult<()> {
        let resend = &*CLIENT;
        std::thread::sleep(std::time::Duration::from_secs(1));

        let emails = vec![
            CreateEmailBaseOptions::new(
                "Acme <onboarding@resend.dev>",
                ["delivered@resend.dev"],
                "hello world",
            )
            .with_html("<h1>it works!</h1>")
            .with_tag(Tag::new("category", "confirm_email")),
            CreateEmailBaseOptions::new(
                "Acme <onboarding@resend.dev>",
                ["delivered@resend.dev"],
                "world hello",
            )
            .with_html("<p>it works!</p>")
            .with_tag(Tag::new("category", "confirm_email")),
        ];

        let emails = resend.batch.send(emails).await?;
        assert_eq!(emails.len(), 2);

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn schedule() -> DebugResult<()> {
        use jiff::{Span, Timestamp, Zoned};

        let now_plus_1h = Zoned::now()
            .checked_add(Span::new().hours(1))
            .expect("Valid date")
            .timestamp()
            .to_string();

        let resend = &*CLIENT;
        std::thread::sleep(std::time::Duration::from_secs(1));

        let emails = vec![
            CreateEmailBaseOptions::new(
                "Acme <onboarding@resend.dev>",
                ["delivered@resend.dev"],
                "hello world",
            )
            .with_html("<h1>it works!</h1>")
            .with_scheduled_at(&now_plus_1h),
            CreateEmailBaseOptions::new(
                "Acme <onboarding@resend.dev>",
                ["delivered@resend.dev"],
                "world hello",
            )
            .with_html("<p>it works!</p>")
            .with_scheduled_at(&now_plus_1h),
        ];

        let emails = resend.batch.send(emails).await?;
        assert_eq!(emails.len(), 2);
        std::thread::sleep(std::time::Duration::from_secs(4));

        for email in emails {
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

            let _cancelled = resend.emails.cancel(&email.id).await?;
        }

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn attachments() -> DebugResult<()> {
        use crate::list_opts::ListOptions;

        let resend = &*CLIENT;
        std::thread::sleep(std::time::Duration::from_secs(1));

        let attachment = CreateAttachment::from_content(include_bytes!("../README.md").to_vec())
            .with_filename("README.md");

        let emails = vec![
            CreateEmailBaseOptions::new(
                "Acme <onboarding@resend.dev>",
                ["delivered@resend.dev"],
                "hello world",
            )
            .with_html("<h1>it works!</h1>")
            .with_attachment(attachment.clone()),
            CreateEmailBaseOptions::new(
                "Acme <onboarding@resend.dev>",
                ["delivered@resend.dev"],
                "world hello",
            )
            .with_html("<p>it works!</p>")
            .with_attachment(attachment),
        ];

        let emails = resend.batch.send(emails).await?;
        assert_eq!(emails.len(), 2);
        std::thread::sleep(std::time::Duration::from_secs(1));

        for email in emails {
            let attachments = resend
                .emails
                .list_attachments(&email.id, ListOptions::default())
                .await?;
            assert_eq!(attachments.data.len(), 1);
        }

        Ok(())
    }
}
