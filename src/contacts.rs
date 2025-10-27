use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use crate::{
    Config, Result,
    list_opts::ListOptions,
    types::{
        AddContactSegmentResponse, ContactTopic, RemoveContactSegmentResponse, Segment,
        UpdateContactTopicOptions,
    },
};
use crate::{
    list_opts::ListResponse,
    types::{Contact, ContactChanges, ContactId, CreateContactOptions},
};

use self::types::UpdateContactResponse;

/// `Resend` APIs for `/audiences/:id/contacts` endpoints.
#[derive(Clone)]
pub struct ContactsSvc(pub(crate) Arc<Config>);

impl ContactsSvc {
    /// Create a contact.
    ///
    /// <https://resend.com/docs/api-reference/contacts/create-contact>
    #[maybe_async::maybe_async]
    // Reasoning for allow: https://github.com/resend/resend-rust/pull/1#issuecomment-2081646115
    #[allow(clippy::needless_pass_by_value)]
    pub async fn create(&self, contact: CreateContactOptions) -> Result<ContactId> {
        let path = contact.audience_id.as_ref().map_or_else(
            || "/contacts".to_string(),
            |audience_id| format!("/audiences/{audience_id}/contacts"),
        );

        let request = self.0.build(Method::POST, &path);
        let response = self.0.send(request.json(&contact)).await?;
        let content = response.json::<types::CreateContactResponse>().await?;

        Ok(content.id)
    }

    /// Retrieve a single contact.
    ///
    /// <https://resend.com/docs/api-reference/contacts/get-contact>
    #[maybe_async::maybe_async]
    pub async fn get(&self, contact_id_or_email: &str) -> Result<Contact> {
        let path = format!("/contacts/{contact_id_or_email}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Contact>().await?;

        Ok(content)
    }

    /// Update an existing contact.
    ///
    /// <https://resend.com/docs/api-reference/contacts/update-contact>
    #[maybe_async::maybe_async]
    // Reasoning for allow: https://github.com/resend/resend-rust/pull/1#issuecomment-2081646115
    #[allow(clippy::needless_pass_by_value)]
    pub async fn update(
        &self,
        contact_id_or_email: &str,
        update: ContactChanges,
    ) -> Result<UpdateContactResponse> {
        let path = format!("/contacts/{contact_id_or_email}");

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&update)).await?;
        let content = response.json::<UpdateContactResponse>().await?;

        Ok(content)
    }

    /// Remove an existing contact.
    ///
    /// <https://resend.com/docs/api-reference/contacts/delete-contact>
    #[maybe_async::maybe_async]
    pub async fn delete(&self, contact_id_or_email: &str) -> Result<bool> {
        let path = format!("/contacts/{contact_id_or_email}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<types::DeleteContactResponse>().await?;

        Ok(content.deleted)
    }

    /// Retrieves a list contacts from an audience.
    ///
    /// - Default limit: no limit (return everything)
    ///
    /// <https://resend.com/docs/api-reference/contacts/list-contacts>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn list<T>(
        &self,
        audience: &str,
        list_opts: ListOptions<T>,
    ) -> Result<ListResponse<Contact>> {
        let path = format!("/audiences/{audience}/contacts");

        let request = self.0.build(Method::GET, &path).query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<Contact>>().await?;

        Ok(content)
    }

    /// Retrieve a list of topics subscriptions for a contact.
    ///
    /// <https://resend.com/docs/api-reference/contacts/get-contact-topic>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn get_contact_topics<T>(
        &self,
        contact_id_or_email: &str,
        list_opts: ListOptions<T>,
    ) -> Result<ListResponse<ContactTopic>> {
        let path = format!("/contacts/{contact_id_or_email}/topics");

        let request = self.0.build(Method::GET, &path).query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<ContactTopic>>().await?;

        Ok(content)
    }

    /// Update an existing topic subscription for a contact.
    ///
    /// <https://resend.com/docs/api-reference/contacts/update-contact-topic>
    #[maybe_async::maybe_async]
    pub async fn update_contact_topics(
        &self,
        contact_id_or_email: &str,
        topics: impl Into<Vec<UpdateContactTopicOptions>>,
    ) -> Result<UpdateContactResponse> {
        let path = format!("/contacts/{contact_id_or_email}/topics");

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&topics.into())).await?;
        let content = response.json::<UpdateContactResponse>().await?;

        Ok(content)
    }

    /// Add an existing contact to a segment.
    ///
    /// <https://resend.com/docs/api-reference/contacts/add-contact-to-segment>
    #[maybe_async::maybe_async]
    pub async fn add_contact_segment(
        &self,
        contact_id_or_email: &str,
        segment_id: &str,
    ) -> Result<AddContactSegmentResponse> {
        let path = format!("/contacts/{contact_id_or_email}/segments/{segment_id}");

        let request = self.0.build(Method::POST, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<AddContactSegmentResponse>().await?;

        Ok(content)
    }

    /// Remove an existing contact from a segment.
    ///
    /// <https://resend.com/docs/api-reference/contacts/delete-contact-segment>
    #[maybe_async::maybe_async]
    pub async fn delete_contact_segment(
        &self,
        contact_id_or_email: &str,
        segment_id: &str,
    ) -> Result<RemoveContactSegmentResponse> {
        let path = format!("/contacts/{contact_id_or_email}/segments/{segment_id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<RemoveContactSegmentResponse>().await?;

        Ok(content)
    }

    /// Retrieve a list of segments that a contact is part of.
    ///
    /// <https://resend.com/docs/api-reference/contacts/list-contact-segments>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn list_contact_segment<T>(
        &self,
        contact_id_or_email: &str,
        list_opts: ListOptions<T>,
    ) -> Result<ListResponse<Segment>> {
        let path = format!("/contacts/{contact_id_or_email}/segments/");

        let request = self.0.build(Method::GET, &path).query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<Segment>>().await?;

        Ok(content)
    }
}

impl fmt::Debug for ContactsSvc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

#[allow(unreachable_pub)]
pub mod types {
    use serde::{Deserialize, Serialize};

    use crate::{
        topics::types::TopicId,
        types::{SegmentId, SubscriptionType},
    };

    crate::define_id_type!(ContactId);

    /// Details of a new [`Contact`].
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateContactOptions {
        /// Email address of the contact.
        email: String,

        /// The Audience ID.
        pub(crate) audience_id: Option<String>,

        /// First name of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        first_name: Option<String>,
        /// Last name of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        last_name: Option<String>,
        /// Indicates if the contact is unsubscribed.
        #[serde(skip_serializing_if = "Option::is_none")]
        unsubscribed: Option<bool>,
    }

    impl CreateContactOptions {
        /// Creates a new [`ContactData`].
        pub fn new(email: &str) -> Self {
            Self {
                email: email.to_owned(),
                audience_id: None,
                first_name: None,
                last_name: None,
                unsubscribed: None,
            }
        }

        /// Adds the audience id to the contact.
        #[inline]
        pub fn with_audience_id(mut self, audience_id: &str) -> Self {
            self.audience_id = Some(audience_id.to_owned());
            self
        }

        /// Adds the first name to the contact.
        #[inline]
        pub fn with_first_name(mut self, name: &str) -> Self {
            self.first_name = Some(name.to_owned());
            self
        }

        /// Adds the last name to the contact.
        #[inline]
        pub fn with_last_name(mut self, name: &str) -> Self {
            self.last_name = Some(name.to_owned());
            self
        }

        /// Toggles the unsubscribe status to `unsubscribe`.
        #[inline]
        pub const fn with_unsubscribed(mut self, unsubscribed: bool) -> Self {
            self.unsubscribed = Some(unsubscribed);
            self
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct CreateContactResponse {
        /// Unique identifier for the created contact.
        pub id: ContactId,
    }

    /// Details of an existing contact.
    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct Contact {
        /// Unique identifier for the contact.
        pub id: ContactId,
        /// Email address of the contact.
        pub email: String,
        /// First name of the contact.
        pub first_name: String,
        /// Last name of the contact.
        pub last_name: String,
        /// Indicates if the contact is unsubscribed.
        pub unsubscribed: bool,
        /// Timestamp indicating when the contact was created in ISO8601 format.
        pub created_at: String,
    }

    /// List of changes to apply to a [`Contact`].
    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct ContactChanges {
        /// First name of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        first_name: Option<String>,
        /// Last name of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        last_name: Option<String>,
        /// Indicates the subscription status of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        unsubscribed: Option<bool>,
    }

    impl ContactChanges {
        /// Creates a new [`ContactChanges`].
        #[inline]
        pub fn new() -> Self {
            Self::default()
        }

        /// Updates the first name of the contact.
        #[inline]
        pub fn with_first_name(mut self, name: &str) -> Self {
            self.first_name = Some(name.to_owned());
            self
        }

        /// Updates the last name of the contact.
        #[inline]
        pub fn with_last_name(mut self, name: &str) -> Self {
            self.last_name = Some(name.to_owned());
            self
        }

        /// Updates the unsubscribe status of the contact.
        #[inline]
        pub const fn with_unsubscribed(mut self, unsubscribed: bool) -> Self {
            self.unsubscribed = Some(unsubscribed);
            self
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct UpdateContactResponse {
        /// Unique identifier for the updated contact.
        pub id: ContactId,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct DeleteContactResponse {
        /// The ID of the domain.
        #[allow(dead_code)]
        pub contact: ContactId,
        /// Indicates whether the domain was deleted successfully.
        pub deleted: bool,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct ContactTopic {
        pub id: TopicId,
        pub name: String,
        pub description: Option<String>,
        pub subscription: SubscriptionType,
        pub created_at: String,
    }

    /// See [relevant docs].
    ///
    /// [relevant docs]: <https://resend.com/docs/api-reference/contacts/update-contact-topic#body-parameters>
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct UpdateContactTopicOptions {
        id: String,
        subscription: SubscriptionType,
    }

    impl UpdateContactTopicOptions {
        /// `id`: The Topic ID.
        /// `subscription`: The subscription action.
        pub fn new(id: impl Into<String>, subscription: SubscriptionType) -> Self {
            Self {
                id: id.into(),
                subscription,
            }
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct AddContactSegmentResponse {
        pub id: SegmentId,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct RemoveContactSegmentResponse {
        pub id: SegmentId,
        pub deleted: bool,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::needless_return)]
mod test {
    use crate::list_opts::ListOptions;
    use crate::test::{CLIENT, DebugResult};
    use crate::types::{
        ContactChanges, CreateContactOptions, CreateTopicOptions, SubscriptionType,
        UpdateContactTopicOptions,
    };

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn no_audience() -> DebugResult<()> {
        let resend = &*CLIENT;

        let contact = CreateContactOptions::new("steve.wozniak@gmail.com")
            .with_first_name("Steve")
            .with_last_name("Wozniak")
            .with_unsubscribed(false);
        let id = resend.contacts.create(contact).await?;
        std::thread::sleep(std::time::Duration::from_secs(4));

        let deleted = resend.contacts.delete(&id).await?;
        assert!(deleted);
        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> DebugResult<()> {
        let resend = &*CLIENT;
        let audience = "test_contacts";

        // Create audience.
        let audience = resend.segments.create(audience).await?;
        let audience_id = audience.id;
        std::thread::sleep(std::time::Duration::from_secs(4));

        // Create.
        let contact = CreateContactOptions::new("antonios.barotsis@pm.me")
            .with_first_name("Antonios")
            .with_last_name("Barotsis")
            .with_unsubscribed(false)
            .with_audience_id(&audience_id);
        let id = resend.contacts.create(contact).await?;
        std::thread::sleep(std::time::Duration::from_secs(4));

        // Get topic
        let topics = resend
            .contacts
            .get_contact_topics(&id, ListOptions::default())
            .await?;
        assert!(topics.data.is_empty());

        // Update topic
        let topic = resend
            .topics
            .create(CreateTopicOptions::new(
                "Weekly Newsletter",
                SubscriptionType::OptIn,
            ))
            .await?;
        let topics = [UpdateContactTopicOptions::new(
            topic.id.to_string(),
            SubscriptionType::OptIn,
        )];
        let _topics = resend.contacts.update_contact_topics(&id, topics).await?;
        std::thread::sleep(std::time::Duration::from_secs(4));

        // Get topic
        let topics = resend
            .contacts
            .get_contact_topics(&id, ListOptions::default())
            .await?;
        assert!(!topics.data.is_empty());

        // Delete topic
        let deleted = resend.topics.delete(&topic.id).await?;
        assert!(deleted.deleted);

        // Update.
        let changes = ContactChanges::new().with_unsubscribed(true);
        let _res = resend.contacts.update(&id, changes).await?;
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Retrieve.
        let contact = resend.contacts.get(&id).await?;
        assert!(contact.unsubscribed);

        // Retrieve by email.
        let contact = resend.contacts.get("antonios.barotsis@pm.me").await?;
        assert!(contact.unsubscribed);

        // List.
        let contacts = resend
            .contacts
            .list(&audience_id, ListOptions::default())
            .await?;
        assert_eq!(contacts.len(), 1);

        // Delete.
        let deleted = resend.contacts.delete(&id).await?;
        assert!(deleted);

        // Delete audience.
        let deleted = resend.segments.delete(&audience_id.clone()).await?;
        assert!(deleted);
        std::thread::sleep(std::time::Duration::from_secs(1));

        // List.
        let contacts = resend
            .contacts
            .list(&audience_id, ListOptions::default())
            .await?;
        assert!(contacts.is_empty());

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn segments() -> DebugResult<()> {
        let resend = &*CLIENT;

        // Create segment
        let segment = resend.segments.create("registered users").await?;
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Create contact
        let contact = CreateContactOptions::new("antonios.barotsis@pm.me")
            .with_first_name("Antonios")
            .with_last_name("Barotsis");
        let contact_id = resend.contacts.create(contact).await?;
        std::thread::sleep(std::time::Duration::from_secs(2));

        let _added = resend
            .contacts
            .add_contact_segment(&contact_id, &segment.id)
            .await?;
        std::thread::sleep(std::time::Duration::from_secs(4));

        let list = resend
            .contacts
            .list_contact_segment(&contact_id, ListOptions::default())
            .await?;
        assert!(!list.data.is_empty());

        let deleted = resend
            .contacts
            .delete_contact_segment(&contact_id, &segment.id)
            .await?;
        assert!(deleted.deleted);
        std::thread::sleep(std::time::Duration::from_secs(4));

        let list = resend
            .contacts
            .list_contact_segment(&contact_id, ListOptions::default())
            .await?;
        assert!(list.data.is_empty());

        // Delete
        let deleted = resend.segments.delete(&segment.id).await?;
        assert!(deleted);
        let deleted = resend.contacts.delete(&contact_id).await?;
        assert!(deleted);
        Ok(())
    }
}
