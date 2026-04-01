use std::sync::Arc;

use reqwest::Method;

use crate::{
    Config, Result,
    list_opts::{ListOptions, ListResponse},
    types::Log,
};

/// `Resend` APIs for `/logs` endpoints.
#[derive(Clone, Debug)]
pub struct LogsSvc(pub(crate) Arc<Config>);

impl LogsSvc {
    /// Retrieve a single API request log.
    ///
    /// <https://resend.com/docs/api-reference/logs/retrieve-log>
    #[maybe_async::maybe_async]
    pub async fn get(&self, log_id: &str) -> Result<Log> {
        let path = format!("/logs/{log_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Log>().await?;

        Ok(content)
    }

    /// Retrieve a list of API request logs.
    ///
    /// - Default limit: 20
    ///
    /// <https://resend.com/docs/api-reference/logs/list-logs>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn list<T>(&self, list_opts: ListOptions<T>) -> Result<ListResponse<Log>> {
        let request = self.0.build(Method::GET, "/logs").query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<Log>>().await?;

        Ok(content)
    }
}

#[allow(unreachable_pub)]
pub mod types {
    use std::num::NonZeroU16;

    use serde::{Deserialize, Serialize};

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Log {
        pub id: String,
        pub created_at: String,
        pub endpoint: String,
        pub method: String,
        pub response_status: NonZeroU16,
        pub user_agent: Option<String>,
        #[serde(default)]
        pub request_body: serde_json::Value,
        #[serde(default)]
        pub response_body: serde_json::Value,
    }
}

#[cfg(test)]
#[allow(clippy::needless_return, clippy::unwrap_used)]
mod test {
    use crate::list_opts::ListOptions;
    use crate::test::{CLIENT, DebugResult};

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> DebugResult<()> {
        let resend = &*CLIENT;

        // List
        let logs = resend.logs.list(ListOptions::default()).await?;
        assert!(!logs.data.is_empty());

        // Get
        let head = logs.data.first().unwrap();

        let _log = resend.logs.get(&head.id).await?;

        Ok(())
    }
}
