use std::sync::Arc;

use reqwest::Method;

use crate::{
    Config, Result,
    list_opts::{ListOptions, ListResponse},
    types::{
        Automation, AutomationRun, CreateAutomationOptions, CreateAutomationResponse,
        DeleteAutomationResponse, StopAutomationResponse, UpdateAutomationOptions,
        UpdateAutomationResponse,
    },
};

/// `Resend` APIs for `/automations` endpoints.
#[derive(Clone, Debug)]
pub struct AutomationsSvc(pub(crate) Arc<Config>);

impl AutomationsSvc {
    /// Create a new automation to automate email sequences.
    ///
    /// <https://resend.com/docs/api-reference/automations/create-automation>
    #[maybe_async::maybe_async]
    pub async fn create(
        &self,
        automation: CreateAutomationOptions,
    ) -> Result<CreateAutomationResponse> {
        let request = self.0.build(Method::POST, "/automations");
        let response = self.0.send(request.json(&automation)).await?;
        let content = response.json::<CreateAutomationResponse>().await?;

        Ok(content)
    }

    /// Update an existing automation.
    ///
    /// <https://resend.com/docs/api-reference/automations/update-automation>
    #[maybe_async::maybe_async]
    pub async fn update(
        &self,
        automation_id: &str,
        update: UpdateAutomationOptions,
    ) -> Result<UpdateAutomationResponse> {
        let path = format!("/automations/{automation_id}");

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&update)).await?;
        let content = response.json::<UpdateAutomationResponse>().await?;

        Ok(content)
    }

    /// Retrieve a single automation.
    ///
    /// <https://resend.com/docs/api-reference/automations/get-automation>
    #[maybe_async::maybe_async]
    pub async fn get(&self, automation_id: &str) -> Result<Automation> {
        let path = format!("/automations/{automation_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Automation>().await?;

        Ok(content)
    }

    /// Retrieve a list of automations.
    ///
    /// <https://resend.com/docs/api-reference/automations/list-automations>
    #[maybe_async::maybe_async]
    pub async fn list<T>(&self, list_opts: ListOptions<T>) -> Result<ListResponse<Automation>> {
        let request = self.0.build(Method::GET, "/automations").query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<Automation>>().await?;

        Ok(content)
    }

    /// Stop a running automation.
    ///
    /// <https://resend.com/docs/api-reference/automations/stop-automation>
    #[maybe_async::maybe_async]
    pub async fn stop(&self, automation_id: &str) -> Result<StopAutomationResponse> {
        let path = format!("/automations/{automation_id}/stop");

        let request = self.0.build(Method::POST, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<StopAutomationResponse>().await?;

        Ok(content)
    }

    /// Remove an existing automation.
    ///
    /// <https://resend.com/docs/api-reference/automations/delete-automation>
    #[maybe_async::maybe_async]
    pub async fn delete(&self, automation_id: &str) -> Result<DeleteAutomationResponse> {
        let path = format!("/automations/{automation_id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<DeleteAutomationResponse>().await?;

        Ok(content)
    }

    /// Retrieve a list of automation runs.
    ///
    /// <https://resend.com/docs/api-reference/automations/list-automation-runs>
    #[maybe_async::maybe_async]
    pub async fn list_runs<T>(
        &self,
        automation_id: &str,
        status_filter: Option<String>,
        list_opts: ListOptions<T>,
    ) -> Result<ListResponse<AutomationRun>> {
        let path = format!("/automations/{automation_id}/runs");

        let request = self
            .0
            .build(Method::GET, &path)
            .query(&list_opts)
            .query(&status_filter);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<AutomationRun>>().await?;

        Ok(content)
    }

    /// Retrieve a single automation run.
    ///
    /// <https://resend.com/docs/api-reference/automations/get-automation-run>
    #[maybe_async::maybe_async]
    pub async fn get_run(&self, automation_id: &str, run_id: &str) -> Result<AutomationRun> {
        let path = format!("/automations/{automation_id}/runs/{run_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<AutomationRun>().await?;

        Ok(content)
    }
}

#[allow(unreachable_pub)]
pub mod types {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    crate::define_id_type!(AutomationId);
    crate::define_id_type!(AutomationRunId);

    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateAutomationOptions {
        pub name: String,
        pub status: AutomationStatus,
        pub steps: Vec<Step>,
        pub edges: Vec<Edge>,
    }

    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
    #[serde(rename_all = "snake_case")]
    pub enum AutomationStatus {
        Enabled,
        #[default]
        Disabled,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "type", rename_all = "snake_case")]
    pub enum Step {
        Trigger {
            r#ref: String,
            config: TriggerStepConfig,
        },
        Delay {
            r#ref: String,
            config: DelayStepConfig,
        },
        SendEmail {
            r#ref: String,
            config: SendEmailStepConfig,
        },
        WaitForEvent {
            r#ref: String,
            config: WaitForEventStepConfig,
        },
        Condition {
            r#ref: String,
            config: Value,
        },
        ContactUpdate {
            r#ref: String,
            config: Value,
        },
        ContactDelete {
            r#ref: String,
            config: Value,
        },
        AddToSegment {
            r#ref: String,
            config: Value,
        },
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TriggerStepConfig {
        pub event_name: String,
    }

    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct DelayStepConfig {
        pub seconds: u32,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SendEmailStepConfig {
        pub template_id: String,
        pub subject: Option<String>,
        pub from: Option<String>,
        pub reply_to: Option<String>,
        // I give up
        pub variables: HashMap<String, Value>,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WaitForEventStepConfig {
        pub event_name: String,
        pub timeout_seconds: Option<u32>,
        pub filter_rule: Option<Value>,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Edge {
        pub from: String,
        pub to: String,
        pub edge_type: Option<EdgeType>,
    }

    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
    #[serde(rename_all = "snake_case")]
    pub enum EdgeType {
        #[default]
        Default,
        ConditionMet,
        ConditionNotMet,
        Timeout,
        EventReceived,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreateAutomationResponse {
        pub id: AutomationId,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Automation {
        pub id: AutomationId,
        pub name: String,
        pub status: AutomationStatus,
        pub created_at: String,
        pub updated_at: Option<String>,
        pub steps: Vec<Step>,
        pub edges: Vec<Edge>,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Default)]
    pub struct UpdateAutomationOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        status: Option<AutomationStatus>,
        #[serde(skip_serializing_if = "Option::is_none")]
        steps: Option<Vec<Step>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        edges: Option<Vec<Edge>>,
    }

    impl UpdateAutomationOptions {
        #[inline]
        pub fn new() -> Self {
            Self::default()
        }

        #[inline]
        pub fn with_name(mut self, name: &str) -> Self {
            self.name = Some(name.to_owned());
            self
        }

        #[inline]
        pub fn with_status(mut self, status: AutomationStatus) -> Self {
            self.status = Some(status);
            self
        }

        #[inline]
        pub fn with_steps(mut self, steps: Vec<Step>) -> Self {
            self.steps = Some(steps);
            self
        }

        #[inline]
        pub fn with_edges(mut self, edges: Vec<Edge>) -> Self {
            self.edges = Some(edges);
            self
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateAutomationResponse {
        pub id: AutomationId,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct StopAutomationResponse {
        pub id: AutomationId,
        pub status: AutomationStatus,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DeleteAutomationResponse {
        pub id: AutomationId,
        pub deleted: bool,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AutomationRun {
        id: AutomationRunId,
        #[serde(skip_serializing_if = "Option::is_none")]
        started_at: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        completed_at: Option<String>,
        created_at: String,
        status: AutomationRunStatus,
        trigger: Option<AutomationRunTrigger>,
    }

    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum AutomationRunStatus {
        Running,
        Completed,
        Failed,
        Cancelled,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AutomationRunTrigger {
        event_name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        payload: Option<HashMap<String, Value>>,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::needless_return)]
mod test {
    use crate::{
        automations::types::TriggerStepConfig,
        test::{CLIENT, DebugResult},
        types::{
            Automation, AutomationStatus, CreateAutomationOptions, Edge, EdgeType, Step,
            WaitForEventStepConfig,
        },
    };

    #[test]
    fn serialize_create() {
        let tmp = CreateAutomationOptions {
            name: "automation".to_owned(),
            status: AutomationStatus::Enabled,
            steps: vec![Step::WaitForEvent {
                r#ref: "test".to_owned(),
                config: WaitForEventStepConfig {
                    event_name: "test".to_owned(),
                    timeout_seconds: Some(3),
                    filter_rule: Some(serde_json::json!({
                      "type": "rule",
                      "field": "idk",
                      "operator": "eq",
                      "value": null,
                    })),
                },
            }],
            edges: vec![Edge {
                from: "from".to_owned(),
                to: "to".to_owned(),
                edge_type: Some(EdgeType::Default),
            }],
        };

        println!("{}", serde_json::to_string(&tmp).unwrap());
    }

    #[test]
    fn deserialize_get() {
        let tmp = r#"
          {
            "object": "automation",
            "id": "c9b16d4f-ba6c-4e2e-b044-6bf4404e57fd",
            "name": "Welcome series",
            "status": "enabled",
            "created_at": "2025-10-01 12:00:00.000000+00",
            "updated_at": "2025-10-01 12:00:00.000000+00",
            "steps": [
              {
                "type": "trigger",
                "config": { "event_name": "user.created" }
              },
              {
                "type": "send_email",
                "config": {
                  "template_id": "tpl_xxxxxxxxx",
                  "subject": "Welcome!",
                  "from": "Acme <hello@example.com>"
                }
              },
              {
                "type": "delay",
                "config": { "seconds": 172800 }
              },
              {
                "type": "send_email",
                "config": {
                  "template_id": "f6e86e54-0ab4-404d-8edc-d52ea8cf602e",
                  "subject": "Getting started",
                  "from": "Acme <hello@example.com>"
                }
              }
            ],
            "edges": [
              {
                "from": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
                "to": "b2c3d4e5-f6a7-8901-bcde-f12345678901",
                "edge_type": "default"
              },
              {
                "from": "b2c3d4e5-f6a7-8901-bcde-f12345678901",
                "to": "c3d4e5f6-a7b8-9012-cdef-123456789012",
                "edge_type": "default"
              },
              {
                "from": "c3d4e5f6-a7b8-9012-cdef-123456789012",
                "to": "d4e5f6a7-b8c9-0123-def1-234567890123",
                "edge_type": "default"
              }
            ]
          }"#;

        let _res = serde_json::from_str::<Automation>(tmp).unwrap();
    }

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> DebugResult<()> {
        use crate::{list_opts::ListOptions, types::UpdateAutomationOptions};

        let resend = &*CLIENT;

        // Create
        let opts = CreateAutomationOptions {
            name: "Welcome series".to_owned(),
            status: AutomationStatus::Enabled,
            steps: vec![Step::Trigger {
                r#ref: "trigger".to_owned(),
                config: TriggerStepConfig {
                    event_name: "user.created".to_owned(),
                },
            }],
            edges: vec![],
        };
        let automation = resend.automations.create(opts).await?;
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Update
        let opts = UpdateAutomationOptions::new().with_status(AutomationStatus::Enabled);
        let automation = resend.automations.update(&automation.id, opts).await?;

        // Get
        // TODO: Missing ref error
        // let automation = resend.automations.get(&automation.id).await?;

        // List
        // TODO: Missing ref error
        // let automations = resend.automations.list(ListOptions::default()).await?;

        // List Runs
        let runs = resend
            .automations
            .list_runs(&automation.id, None, ListOptions::default())
            .await?;
        assert!(runs.data.is_empty());

        // Stop
        let automation = resend.automations.stop(&automation.id).await?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        // Delete
        let automation = resend.automations.delete(&automation.id).await?;
        assert!(automation.deleted);

        Ok(())
    }
}
