use std::sync::Arc;

use reqwest::Method;

use crate::{
    Config, Result,
    list_opts::{ListOptions, ListResponse},
    types::{
        Automation, AutomationMinimal, AutomationRun, CreateAutomationOptions,
        CreateAutomationResponse, DeleteAutomationResponse, StopAutomationResponse,
        UpdateAutomationOptions, UpdateAutomationResponse,
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
    #[allow(clippy::needless_pass_by_value)]
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
    #[allow(clippy::needless_pass_by_value)]
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
    #[allow(clippy::needless_pass_by_value)]
    pub async fn list<T>(
        &self,
        list_opts: ListOptions<T>,
    ) -> Result<ListResponse<AutomationMinimal>> {
        let request = self.0.build(Method::GET, "/automations").query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<AutomationMinimal>>().await?;

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
    #[allow(clippy::needless_pass_by_value)]
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
        pub connections: Vec<Connection>,
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
            key: String,
            config: TriggerStepConfig,
        },
        SendEmail {
            key: String,
            config: SendEmailStepConfig,
        },
        Delay {
            key: String,
            config: DelayStepConfig,
        },
        WaitForEvent {
            key: String,
            config: WaitForEventStepConfig,
        },
        Condition {
            key: String,
            config: Value,
        },
        ContactUpdate {
            key: String,
            config: Value,
        },
        ContactDelete {
            key: String,
            config: Value,
        },
        AddToSegment {
            key: String,
            config: AddToSegmentStepConfig,
        },
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TriggerStepConfig {
        pub event_name: String,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SendEmailStepConfig {
        pub template: AutomationTemplate,
        pub subject: Option<String>,
        pub from: Option<String>,
        pub reply_to: Option<String>,
        pub variables: Option<Value>,
    }

    impl SendEmailStepConfig {
        #[inline]
        pub fn new(template: AutomationTemplate) -> Self {
            Self {
                template,
                subject: None,
                from: None,
                reply_to: None,
                variables: None,
            }
        }

        #[inline]
        pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
            self.subject = Some(subject.into());
            self
        }

        #[inline]
        pub fn with_from(mut self, from: impl Into<String>) -> Self {
            self.from = Some(from.into());
            self
        }

        #[inline]
        pub fn with_reply_to(mut self, reply_to: impl Into<String>) -> Self {
            self.reply_to = Some(reply_to.into());
            self
        }

        #[inline]
        pub fn with_variables(mut self, variables: Value) -> Self {
            self.variables = Some(variables);
            self
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub struct AutomationTemplate {
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub variables: Option<Value>,
    }

    impl AutomationTemplate {
        #[inline]
        pub fn new(id: impl Into<String>) -> Self {
            Self {
                id: id.into(),
                variables: None,
            }
        }

        #[inline]
        pub fn with_variables(mut self, variables: Value) -> Self {
            self.variables = Some(variables);
            self
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DelayStepConfig {
        pub duration: String,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WaitForEventStepConfig {
        pub event_name: String,
        pub timeout: Option<String>,
        pub filter_rule: Option<Value>,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AddToSegmentStepConfig {
        pub segment_id: String,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Connection {
        pub from: String,
        pub to: String,
        pub r#type: Option<ConnectionType>,
    }

    impl Connection {
        #[inline]
        pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
            Self {
                from: from.into(),
                to: to.into(),
                r#type: None,
            }
        }

        #[inline]
        pub fn with_type(mut self, r#type: ConnectionType) -> Self {
            self.r#type = Some(r#type);
            self
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
    #[serde(rename_all = "snake_case")]
    pub enum ConnectionType {
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
        pub connections: Vec<Connection>,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AutomationMinimal {
        pub id: AutomationId,
        pub name: String,
        pub status: AutomationStatus,
        pub created_at: String,
        pub updated_at: Option<String>,
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
        connections: Option<Vec<Connection>>,
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
        pub fn with_connections(mut self, connections: Vec<Connection>) -> Self {
            self.connections = Some(connections);
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
        automations::types::{SendEmailStepConfig, TriggerStepConfig},
        test::{CLIENT, DebugResult},
        types::{Automation, AutomationStatus, Connection, CreateAutomationOptions, Step},
    };

    #[test]
    fn serialize_create() {
        let tmp = CreateAutomationOptions {
            name: "Welcome series".to_owned(),
            status: AutomationStatus::Enabled,
            steps: vec![
                Step::Trigger {
                    key: "start".to_owned(),
                    config: TriggerStepConfig {
                        event_name: "user.created".to_owned(),
                    },
                },
                Step::SendEmail {
                    key: "welcome".to_owned(),
                    config: SendEmailStepConfig {
                        subject: None,
                        from: None,
                        reply_to: None,
                        variables: None,
                        template: crate::automations::types::AutomationTemplate {
                            id: "34a080c9-b17d-4187-ad80-5af20266e535".to_owned(),
                            variables: None,
                        },
                    },
                },
            ],
            connections: vec![Connection {
                from: "start".to_owned(),
                to: "welcome".to_owned(),
                r#type: None,
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
            "status": "disabled",
            "created_at": "2026-10-01 12:00:00.000000+00",
            "updated_at": "2026-10-01 12:00:00.000000+00",
            "steps": [
              {
                "key": "start",
                "type": "trigger",
                "config": { "event_name": "user.created" }
              },
              {
                "key": "welcome",
                "type": "send_email",
                "config": {
                  "template": { "id": "34a080c9-b17d-4187-ad80-5af20266e535" }
                }
              }
            ],
            "connections": [
              {
                "from": "start",
                "to": "welcome",
                "type": "default"
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
                key: "trigger".to_owned(),
                config: TriggerStepConfig {
                    event_name: "user.created".to_owned(),
                },
            }],
            connections: vec![],
        };
        let automation = resend.automations.create(opts).await?;
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Update
        let opts = UpdateAutomationOptions::new().with_status(AutomationStatus::Enabled);
        let automation = resend.automations.update(&automation.id, opts).await?;

        // Get
        let automation = resend.automations.get(&automation.id).await?;

        // List
        let automations = resend.automations.list(ListOptions::default()).await?;
        assert!(!automations.data.is_empty());

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
