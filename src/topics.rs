use std::sync::Arc;

use reqwest::Method;

use crate::{
    Config, Result,
    list_opts::{ListOptions, ListResponse},
    types::{
        CreateTopicOptions, CreateTopicResponse, DeleteTopicResponse, Topic, UpdateTopicOptions,
        UpdateTopicResponse,
    },
};

/// `Resend` APIs for `/topics` endpoints.
#[derive(Clone, Debug)]
pub struct TopicsSvc(pub(crate) Arc<Config>);

impl TopicsSvc {
    /// Create and email topics to segment your audience.
    ///
    /// <https://resend.com/docs/api-reference/topics/create-topic>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn create(&self, topic: CreateTopicOptions) -> Result<CreateTopicResponse> {
        let request = self.0.build(Method::POST, "/topics");
        let response = self.0.send(request.json(&topic)).await?;
        let content = response.json::<CreateTopicResponse>().await?;

        Ok(content)
    }

    /// Retrieve a topic by its ID.
    ///
    /// <https://resend.com/docs/api-reference/topics/get-topic>
    #[maybe_async::maybe_async]
    pub async fn get(&self, topic_id: &str) -> Result<Topic> {
        let path = format!("/topics/{topic_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Topic>().await?;

        Ok(content)
    }

    /// Update an existing topic.
    ///
    /// <https://resend.com/docs/api-reference/topics/update-topic>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn update(
        &self,
        topic_id: &str,
        update: UpdateTopicOptions,
    ) -> Result<UpdateTopicResponse> {
        let path = format!("/topics/{topic_id}");

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&update)).await?;
        let content = response.json::<UpdateTopicResponse>().await?;

        Ok(content)
    }

    /// Remove an existing topic.
    ///
    /// <https://resend.com/docs/api-reference/topics/delete-topic>
    #[maybe_async::maybe_async]
    pub async fn delete(&self, topic_id: &str) -> Result<DeleteTopicResponse> {
        let path = format!("/topics/{topic_id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<DeleteTopicResponse>().await?;

        Ok(content)
    }

    /// Retrieve a list of topics for the authenticated user.
    ///
    /// - Default limit: 20
    ///
    /// <https://resend.com/docs/api-reference/topics/list-topics>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn list<T>(&self, list_opts: ListOptions<T>) -> Result<ListResponse<Topic>> {
        let request = self.0.build(Method::GET, "/topics").query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<Topic>>().await?;

        Ok(content)
    }
}

#[allow(unreachable_pub)]
pub mod types {
    use serde::{Deserialize, Serialize};

    crate::define_id_type!(TopicId);

    /// See [relevant docs].
    ///
    /// [relevant docs]: <https://resend.com/docs/api-reference/topics/create-topic#body-parameters>
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateTopicOptions {
        name: String,
        default_subscription: SubscriptionType,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        visibility: Option<TopicVisibility>,
    }

    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum SubscriptionType {
        OptIn,
        OptOut,
    }

    impl CreateTopicOptions {
        /// Creates a new [`CreateTopicOptions`].
        ///
        /// - `name`: The topic name. Max length is `50` characters.
        /// - `default_subscription`: The default subscription preference for new contacts.
        pub fn new(name: impl Into<String>, default_subscription: SubscriptionType) -> Self {
            Self {
                name: name.into(),
                default_subscription,
                description: None,
                visibility: None,
            }
        }

        /// The topic description. Max length is `200` characters.
        #[inline]
        pub fn with_description(mut self, description: String) -> Self {
            self.description = Some(description);
            self
        }

        /// The visibility of the topic on the unsubscribe page.
        #[inline]
        pub const fn with_visibility(mut self, visibility: TopicVisibility) -> Self {
            self.visibility = Some(visibility);
            self
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct CreateTopicResponse {
        /// Unique identifier for the published topic.
        pub id: TopicId,
    }

    /// Received Topic.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub struct Topic {
        pub id: TopicId,
        pub name: String,
        pub description: Option<String>,
        pub default_subscription: SubscriptionType,
        pub visibility: TopicVisibility,
        pub created_at: String,
    }

    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "kebab-case")]
    pub enum TopicVisibility {
        /// Only contacts who are opted in to the topic can see it on the unsubscribe page.
        Public,
        /// All contacts can see the topic on the unsubscribe page.
        Private,
    }

    /// List of changes to apply to a [`Topic`].
    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct UpdateTopicOptions {
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        visibility: Option<TopicVisibility>,
    }

    impl UpdateTopicOptions {
        pub const fn new() -> Self {
            Self {
                name: None,
                description: None,
                visibility: None,
            }
        }

        /// The topic name. Max length is `50` characters.
        #[inline]
        pub fn with_name(mut self, name: impl Into<String>) -> Self {
            self.name = Some(name.into());
            self
        }

        /// The topic description. Max length is `200` character
        #[inline]
        pub fn with_description(mut self, description: impl Into<String>) -> Self {
            self.description = Some(description.into());
            self
        }

        /// The visibility of the topic on the unsubscribe page.
        #[inline]
        pub const fn with_visibility(mut self, visibility: TopicVisibility) -> Self {
            self.visibility = Some(visibility);
            self
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct UpdateTopicResponse {
        /// Unique identifier for the updated topic.
        pub id: TopicId,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct DeleteTopicResponse {
        /// Unique identifier for the topic.
        pub id: TopicId,
        /// Indicates whether the topic was deleted successfully.
        pub deleted: bool,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::needless_return)]
mod test {
    use crate::list_opts::ListOptions;
    use crate::test::{CLIENT, DebugResult};
    use crate::types::{
        CreateTopicOptions, SubscriptionType, Topic, TopicVisibility, UpdateTopicOptions,
    };

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    #[ignore = "Flaky backend"]
    async fn all() -> DebugResult<()> {
        let resend = &*CLIENT;

        // Create
        let topic = CreateTopicOptions::new("Weekly Newsletter", SubscriptionType::OptIn)
            .with_visibility(TopicVisibility::Public);
        let topic = resend.topics.create(topic).await?;
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Get
        let topic = resend.topics.get(&topic.id).await?;
        assert_eq!(topic.visibility, TopicVisibility::Public);

        // Update
        let update = UpdateTopicOptions::new()
            .with_name("Weekly Newsletter")
            .with_description("Weekly newsletter for our subscribers")
            .with_visibility(TopicVisibility::Private);
        let topic = resend.topics.update(&topic.id, update).await?;
        std::thread::sleep(std::time::Duration::from_secs(4));

        // List
        let topics = resend.topics.list(ListOptions::default()).await?;
        assert!(topics.len() == 1, "{}", format!("Was {}", topics.len()));
        assert_eq!(
            topics.data.first().unwrap().visibility,
            TopicVisibility::Private
        );

        // Delete
        let deleted = resend.topics.delete(&topic.id).await?;
        assert!(deleted.deleted);

        std::thread::sleep(std::time::Duration::from_secs(4));

        let topics = resend.topics.list(ListOptions::default()).await?;
        assert!(topics.is_empty());

        Ok(())
    }

    #[test]
    fn deserialize_test() {
        let topic = r#"{
  "id": "b6d24b8e-af0b-4c3c-be0c-359bbd97381e",
  "name": "Weekly Newsletter",
  "description": "Weekly newsletter for our subscribers",
  "default_subscription": "opt_in",
  "visibility": "public",
  "created_at": "2023-04-08T00:11:13.110779+00:00"
}"#;

        let res = serde_json::from_str::<Topic>(topic);
        assert!(res.is_ok());
    }
}
