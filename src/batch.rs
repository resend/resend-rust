use std::sync::Arc;

use reqwest::Method;
use types::SendEmailBatchResponse;

use crate::{Config, Result, emails::types::CreateEmailBaseOptions, types::CreateEmailResponse};

/// `Resend` APIs for `/emails` endpoints.
#[derive(Clone, Debug)]
pub struct BatchSvc(pub(crate) Arc<Config>);

impl BatchSvc {
    /// Trigger up to 100 batch emails at once.
    ///
    /// Instead of sending one email per HTTP request, we provide a batching endpoint
    /// that permits you to send up to 100 emails in a single API call.
    ///
    /// <https://resend.com/docs/api-reference/emails/send-batch-emails>
    #[maybe_async::maybe_async]
    pub async fn send<T>(&self, emails: T) -> Result<Vec<CreateEmailResponse>>
    where
        T: IntoIterator<Item = CreateEmailBaseOptions> + Send,
    {
        let emails: Vec<_> = emails.into_iter().collect();

        let request = self.0.build(Method::POST, "/emails/batch");
        let response = self.0.send(request.json(&emails)).await?;
        let content = response.json::<SendEmailBatchResponse>().await?;

        Ok(content.data)
    }
}

#[allow(unreachable_pub)]
pub mod types {
    use crate::types::CreateEmailResponse;

    #[derive(Debug, Clone, serde::Deserialize)]
    pub struct SendEmailBatchResponse {
        /// The IDs of the sent emails.
        pub data: Vec<CreateEmailResponse>,
    }
}
