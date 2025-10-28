use std::sync::Arc;

use reqwest::Method;

use crate::types::{
    CreateTemplateOptions, CreateTemplateResponse, DeleteTemplateResponse,
    DuplicateTemplateResponse, PublishTemplateResponse, Template, UpdateTemplateOptions,
    UpdateTemplateResponse,
};
use crate::{
    Config, Result,
    list_opts::{ListOptions, ListResponse},
};

/// `Resend` APIs for `/templates` endpoints.
#[derive(Clone, Debug)]
pub struct TemplateSvc(pub(crate) Arc<Config>);

impl TemplateSvc {
    /// Create a new template.
    ///
    /// <https://resend.com/docs/api-reference/templates/create-template>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn create(&self, template: CreateTemplateOptions) -> Result<CreateTemplateResponse> {
        let request = self.0.build(Method::POST, "/templates");
        let response = self.0.send(request.json(&template)).await?;
        let content = response.json::<CreateTemplateResponse>().await?;

        Ok(content)
    }

    /// Get a template by ID
    ///
    /// <https://resend.com/docs/api-reference/templates/get-template>
    #[maybe_async::maybe_async]
    pub async fn get(&self, id_or_alias: &str) -> Result<Template> {
        let path = format!("/templates/{id_or_alias}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Template>().await?;

        Ok(content)
    }

    /// Update a template.
    ///
    /// <https://resend.com/docs/api-reference/templates/update-template>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn update(
        &self,
        id_or_alias: &str,
        update: UpdateTemplateOptions,
    ) -> Result<UpdateTemplateResponse> {
        let path = format!("/templates/{id_or_alias}");

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&update)).await?;
        let content = response.json::<UpdateTemplateResponse>().await?;

        Ok(content)
    }

    /// Publish a template.
    ///
    /// <https://resend.com/docs/api-reference/templates/publish-template>
    #[maybe_async::maybe_async]
    pub async fn publish(&self, id_or_alias: &str) -> Result<PublishTemplateResponse> {
        let path = format!("/templates/{id_or_alias}/publish");

        let request = self.0.build(Method::POST, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<PublishTemplateResponse>().await?;

        Ok(content)
    }

    /// Duplicate a template.
    ///
    /// <https://resend.com/docs/api-reference/templates/duplicate-template>
    #[maybe_async::maybe_async]
    pub async fn duplicate(&self, id_or_alias: &str) -> Result<DuplicateTemplateResponse> {
        let path = format!("/templates/{id_or_alias}/duplicate");

        let request = self.0.build(Method::POST, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<DuplicateTemplateResponse>().await?;

        Ok(content)
    }

    /// Delete a template.
    ///
    /// <https://resend.com/docs/api-reference/templates/delete-template>
    #[maybe_async::maybe_async]
    pub async fn delete(&self, id_or_alias: &str) -> Result<DeleteTemplateResponse> {
        let path = format!("/templates/{id_or_alias}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<DeleteTemplateResponse>().await?;

        Ok(content)
    }

    /// Retrieve a list of templates.
    ///
    /// - Default limit: 20
    ///
    /// <https://resend.com/docs/api-reference/templates/list-templates>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn list<T>(&self, list_opts: ListOptions<T>) -> Result<ListResponse<Template>> {
        let request = self.0.build(Method::GET, "/templates").query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<Template>>().await?;

        Ok(content)
    }
}

#[allow(unreachable_pub)]
pub mod types {
    use serde::{Deserialize, Deserializer, Serialize};
    crate::define_id_type!(TemplateId);

    /// See [relevant docs].
    ///
    /// [relevant docs]: <https://resend.com/docs/api-reference/templates/create-template#body-parameters>
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateTemplateOptions {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        alias: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        from: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        subject: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_to: Option<Vec<String>>,
        html: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        variables: Option<Vec<Variable>>,
    }

    /// See [relevant docs].
    ///
    /// [relevant docs]: <https://resend.com/docs/api-reference/templates/create-template#param-variables>
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub struct Variable {
        key: String,
        #[serde(rename = "type")]
        ttype: VariableType,
        fallback_value: Option<serde_json::Value>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
    #[must_use]
    #[serde(rename_all = "snake_case")]
    pub enum VariableType {
        String,
        Number,
    }

    impl CreateTemplateOptions {
        /// Creates a new [`CreateTemplateOptions`].
        ///
        /// - `name`: The name of the template.
        /// - `html`: The HTML version of the template.
        pub fn new(name: impl Into<String>, html: impl Into<String>) -> Self {
            Self {
                name: name.into(),
                alias: None,
                from: None,
                subject: None,
                reply_to: None,
                html: html.into(),
                text: None,
                variables: None,
            }
        }

        /// Adds or overwrites the alias version of the template.
        #[inline]
        pub fn with_alias(mut self, alias: &str) -> Self {
            self.alias = Some(alias.to_owned());
            self
        }

        /// Adds or overwrites the sender email address of the template.
        ///
        /// To include a friendly name, use the format `"Your Name <sender@domain.com>"`.
        ///
        /// If provided, this value can be overridden when sending an email using the template.
        #[inline]
        pub fn with_from(mut self, from: &str) -> Self {
            self.from = Some(from.to_owned());
            self
        }

        /// Adds or overwrites the sender email subject of the template.
        ///
        /// If provided, this value can be overridden when sending an email using the template.
        #[inline]
        pub fn with_subject(mut self, subject: &str) -> Self {
            self.subject = Some(subject.to_owned());
            self
        }

        /// Attaches reply-to email address.
        ///
        /// If provided, this value can be overridden when sending an email using the template.
        #[inline]
        pub fn with_reply_to(mut self, reply_to: &str) -> Self {
            let reply_to_vec = self.reply_to.get_or_insert_with(Vec::new);
            reply_to_vec.push(reply_to.to_owned());
            self
        }

        /// Attaches reply-to email addresses.
        ///
        /// If provided, this value can be overridden when sending an email using the template.
        #[inline]
        pub fn with_reply_tos(mut self, reply_tos: &[String]) -> Self {
            let reply_to_vec = self.reply_to.get_or_insert_with(Vec::new);
            reply_to_vec.extend_from_slice(reply_tos);
            self
        }

        /// Adds or overwrites the The plain text version of the message.
        ///
        /// If not provided, the HTML will be used to generate a plain text version. You can opt
        /// out of this behavior by setting value to an empty string.
        #[inline]
        pub fn with_text(mut self, text: &str) -> Self {
            self.text = Some(text.to_owned());
            self
        }

        /// Attaches a variable.
        ///
        /// Each template may contain up to 20 variables.
        #[inline]
        #[allow(clippy::needless_pass_by_value)]
        pub fn with_variable(mut self, variable: Variable) -> Self {
            let variables = self.variables.get_or_insert_with(Vec::new);
            variables.push(variable);
            self
        }

        /// Attaches variables.
        ///
        /// Each template may contain up to 20 variables.
        #[inline]
        #[allow(clippy::needless_pass_by_value)]
        pub fn with_variables(mut self, variables: &[Variable]) -> Self {
            let variables_vec = self.variables.get_or_insert_with(Vec::new);
            variables_vec.extend_from_slice(variables);
            self
        }
    }

    impl Variable {
        /// Creates a new [`Variable`].
        ///
        /// - `key`: The key of the variable. We recommend capitalizing the key (e.g. `FIRST_NAME`).
        /// - `ttype`: The type of the variable.
        ///   Can be `string`, `number`, `boolean`, `object`, or `list`.
        pub fn new(key: impl Into<String>, ttype: VariableType) -> Self {
            Self {
                key: key.into(),
                ttype,
                fallback_value: None,
            }
        }

        /// Adds or overwrites the fallback value.
        ///
        /// The fallback value of the variable. The value must match the type of the variable.
        ///
        /// If no fallback value is provided, you must provide a value for the variable when
        /// sending an email using the template.
        ///
        /// If `object` type is provided, you must include a fallback.
        #[inline]
        #[allow(clippy::needless_pass_by_value)]
        pub fn with_fallback(mut self, fallback: impl Into<serde_json::Value>) -> Self {
            self.fallback_value = Some(fallback.into());
            self
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct CreateTemplateResponse {
        /// The ID of the created template.
        pub id: TemplateId,
    }

    /// Received Template.
    #[must_use]
    #[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
    pub struct Template {
        pub id: TemplateId,
        pub alias: Option<String>,
        pub name: String,
        pub created_at: String,
        pub updated_at: String,
        pub status: TemplateEvent,
        pub published_at: Option<String>,
        pub from: Option<String>,
        pub subject: Option<String>,
        pub reply_to: Option<Vec<String>>,
        pub html: Option<String>,
        pub text: Option<String>,
        #[serde(deserialize_with = "parse_nullable_vec")]
        #[serde(default)]
        pub variables: Vec<Variable>,
    }

    /// Turns:
    /// - `null` -> `[]`
    /// - `["text"]` -> `["text"]`
    fn parse_nullable_vec<'de, D>(deserializer: D) -> Result<Vec<Variable>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::deserialize(deserializer)?;
        Ok(opt.unwrap_or_else(Vec::new))
    }

    /// Strongly typed `status`.
    #[derive(Debug, Clone, Copy, Deserialize, Eq, PartialEq)]
    #[serde(rename_all = "snake_case")]
    pub enum TemplateEvent {
        Draft,
        Published,
    }

    /// List of changes to apply to a [`Template`].
    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct UpdateTemplateOptions {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        alias: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        from: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        subject: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_to: Option<Vec<String>>,
        html: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        variables: Option<Vec<Variable>>,
    }

    impl UpdateTemplateOptions {
        pub fn new(name: impl Into<String>, html: impl Into<String>) -> Self {
            Self {
                name: name.into(),
                alias: None,
                from: None,
                subject: None,
                reply_to: None,
                html: html.into(),
                text: None,
                variables: None,
            }
        }

        #[inline]
        pub fn with_alias(mut self, alias: &str) -> Self {
            self.alias = Some(alias.to_owned());
            self
        }

        #[inline]
        pub fn with_from(mut self, from: &str) -> Self {
            self.from = Some(from.to_owned());
            self
        }

        #[inline]
        pub fn with_subject(mut self, subject: &str) -> Self {
            self.subject = Some(subject.to_owned());
            self
        }

        #[inline]
        pub fn with_reply_to(mut self, reply_to: &str) -> Self {
            let reply_tos = self.reply_to.get_or_insert_with(Vec::new);
            reply_tos.push(reply_to.to_owned());
            self
        }

        #[inline]
        pub fn with_reply_tos(mut self, reply_tos: &[String]) -> Self {
            let reply_tos_vec = self.reply_to.get_or_insert_with(Vec::new);
            reply_tos_vec.extend_from_slice(reply_tos);
            self
        }

        #[inline]
        pub fn with_text(mut self, text: &str) -> Self {
            self.text = Some(text.to_owned());
            self
        }

        #[inline]
        #[allow(clippy::needless_pass_by_value)]
        pub fn with_variable(mut self, variable: Variable) -> Self {
            let variables_vec = self.variables.get_or_insert_with(Vec::new);
            variables_vec.push(variable);
            self
        }

        #[inline]
        #[allow(clippy::needless_pass_by_value)]
        pub fn with_variables(mut self, variables: &[Variable]) -> Self {
            let variables_vec = self.variables.get_or_insert_with(Vec::new);
            variables_vec.extend_from_slice(variables);
            self
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct UpdateTemplateResponse {
        /// Unique identifier for the updated template.
        pub id: TemplateId,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct PublishTemplateResponse {
        /// Unique identifier for the published template.
        pub id: TemplateId,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct DuplicateTemplateResponse {
        /// Unique identifier for the duplicated template.
        pub id: TemplateId,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct DeleteTemplateResponse {
        /// Unique identifier for the template.
        pub id: TemplateId,
        /// Indicates whether the template was deleted successfully.
        pub deleted: bool,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::needless_return)]
mod test {
    use crate::{
        templates::Template,
        test::{CLIENT, DebugResult},
        types::CreateTemplateOptions,
    };

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> DebugResult<()> {
        use crate::{list_opts::ListOptions, types::UpdateTemplateOptions};

        let resend = &*CLIENT;

        let name = "my template";
        let html = "<p>hello</p>";
        let alias = "alias";

        // Create
        let template = CreateTemplateOptions::new(name, html).with_alias(alias);

        let template = resend.templates.create(template).await?;
        let id = template.id;

        std::thread::sleep(std::time::Duration::from_secs(1));

        let get_alias = resend.templates.get(alias).await?;
        let get_id = resend.templates.get(&id).await?;
        assert_eq!(get_alias, get_id);

        // Update
        let alias = "alias updated";
        let template = resend.templates.get(alias).await;
        assert!(template.is_err());

        let update = UpdateTemplateOptions::new(name, html).with_alias(alias);
        let _update = resend.templates.update("alias", update).await?;
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Get
        let template = resend.templates.get(alias).await;
        assert!(template.is_ok());

        // Publish
        let template = resend.templates.get(alias).await?;
        assert!(template.published_at.is_none());

        let template = resend.templates.publish(alias).await?;
        std::thread::sleep(std::time::Duration::from_secs(1));

        let template = resend.templates.get(&template.id).await?;
        assert!(template.published_at.is_some());

        // List
        let templates = resend.templates.list(ListOptions::default()).await?;
        assert!(templates.len() == 1);

        // Duplicate
        let duplicate = resend.templates.duplicate(alias).await?;
        assert!(duplicate.id != template.id);
        std::thread::sleep(std::time::Duration::from_secs(1));
        let templates = resend.templates.list(ListOptions::default()).await?;
        assert!(templates.len() == 2);

        // Delete
        let deleted = resend.templates.delete(alias).await?;
        assert!(deleted.deleted);
        let deleted = resend.templates.delete(&duplicate.id).await;
        assert!(deleted.is_ok());
        std::thread::sleep(std::time::Duration::from_secs(1));

        let deleted = resend.templates.delete(&duplicate.id).await;
        assert!(deleted.is_err());

        Ok(())
    }

    #[test]
    fn deserialize_test() {
        let template = r#"{
  "object": "template",
  "id": "34a080c9-b17d-4187-ad80-5af20266e535",
  "alias": "reset-password",
  "name": "reset-password",
  "created_at": "2023-10-06T23:47:56.678Z",
  "updated_at": "2023-10-06T23:47:56.678Z",
  "status": "published",
  "published_at": "2023-10-06T23:47:56.678Z",
  "from": "John Doe <john.doe@example.com>",
  "subject": "Hello, world!",
  "reply_to": null,
  "html": "<h1>Hello, world!</h1>",
  "text": "Hello, world!",
  "variables": [
    {
      "id": "e169aa45-1ecf-4183-9955-b1499d5701d3",
      "key": "user_name",
      "type": "string",
      "fallback_value": "John Doe",
      "created_at": "2023-10-06T23:47:56.678Z",
      "updated_at": "2023-10-06T23:47:56.678Z"
    }
  ]
}"#;

        let res = serde_json::from_str::<Template>(template);
        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(!res.variables.is_empty());

        let template = r#"{
  "object": "template",
  "id": "34a080c9-b17d-4187-ad80-5af20266e535",
  "alias": "reset-password",
  "name": "reset-password",
  "created_at": "2023-10-06T23:47:56.678Z",
  "updated_at": "2023-10-06T23:47:56.678Z",
  "status": "published",
  "published_at": "2023-10-06T23:47:56.678Z",
  "from": "John Doe <john.doe@example.com>",
  "subject": "Hello, world!",
  "reply_to": null,
  "html": "<h1>Hello, world!</h1>",
  "text": "Hello, world!"
}"#;

        let res = serde_json::from_str::<Template>(template);
        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(res.variables.is_empty());
    }
}
