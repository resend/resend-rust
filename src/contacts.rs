use std::io::Read;
use std::sync::Arc;
use std::{fmt, fs::File};

#[cfg(feature = "blocking")]
use reqwest::blocking::multipart::{Form, Part};
#[cfg(not(feature = "blocking"))]
use reqwest::multipart::{Form, Part};

use reqwest::Method;

use self::types::UpdateContactResponse;
use crate::{
    Config, Error, Result,
    contacts::types::ContactPropertyChanges,
    list_opts::ListOptions,
    types::{
        AddContactSegmentResponse, ContactImport, ContactProperty, ContactTopic,
        CreateContactImportOptions, CreateContactImportResponse, CreateContactPropertyOptions,
        CreateContactPropertyResponse, DeleteContactPropertyResponse, RemoveContactSegmentResponse,
        Segment, UpdateContactPropertyResponse, UpdateContactTopicOptions,
    },
};
use crate::{
    list_opts::ListResponse,
    types::{Contact, ContactChanges, ContactId, CreateContactOptions},
};

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

    /// Create a custom property for your contacts.
    ///
    /// <https://resend.com/docs/api-reference/contact-properties/create-contact-property>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn create_property(
        &self,
        contact_property: CreateContactPropertyOptions,
    ) -> Result<CreateContactPropertyResponse> {
        let path = "/contact-properties";

        let request = self.0.build(Method::POST, path);
        let response = self.0.send(request.json(&contact_property)).await?;
        let content = response.json::<CreateContactPropertyResponse>().await?;

        Ok(content)
    }

    /// Retrieve a contact property by its ID.
    ///
    /// <https://resend.com/docs/api-reference/contact-properties/get-contact-property>
    #[maybe_async::maybe_async]
    pub async fn get_property(&self, contact_property_id: &str) -> Result<ContactProperty> {
        let path = format!("/contact-properties/{contact_property_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<ContactProperty>().await?;

        Ok(content)
    }

    /// Update an existing contact property.
    ///
    /// <https://resend.com/docs/api-reference/contact-properties/update-contact-property>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn update_property(
        &self,
        contact_property_id: &str,
        update: ContactPropertyChanges,
    ) -> Result<UpdateContactPropertyResponse> {
        let path = format!("/contact-properties/{contact_property_id}");

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&update)).await?;
        let content = response.json::<UpdateContactPropertyResponse>().await?;

        Ok(content)
    }

    /// Remove an existing contact property.
    ///
    /// <https://resend.com/docs/api-reference/contact-properties/delete-contact-property>
    #[maybe_async::maybe_async]
    pub async fn delete_property(
        &self,
        contact_property_id: &str,
    ) -> Result<DeleteContactPropertyResponse> {
        let path = format!("/contact-properties/{contact_property_id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<DeleteContactPropertyResponse>().await?;

        Ok(content)
    }

    /// Retrieve a list of contact properties.
    ///
    /// - Default limit: 20
    ///
    /// <https://resend.com/docs/api-reference/contact-properties/list-contact-properties>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn list_properties<T>(
        &self,
        list_opts: ListOptions<T>,
    ) -> Result<ListResponse<ContactProperty>> {
        let path = "/contact-properties";

        let request = self.0.build(Method::GET, path).query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<ContactProperty>>().await?;

        Ok(content)
    }

    /// Create a contact import.
    ///
    /// # Important
    ///
    /// Make sure the file handle has read permissions. You can ensure this by either opening it via [`File::open`] or
    /// using [`std::fs::OpenOptions`] and setting `read` to `true`.
    ///
    /// <https://resend.com/docs/api-reference/contacts/create-contact-import>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn create_import(
        &self,
        mut file: File,
        contact_import: CreateContactImportOptions,
    ) -> Result<CreateContactImportResponse> {
        let path = "/contacts/imports";

        let mut contents = String::new();
        #[allow(clippy::verbose_file_reads)]
        let _size = file
            .read_to_string(&mut contents)
            .map_err(|err| Error::Parse {
                message: "Error while reading file".to_owned(),
                source: Some(Box::new(err)),
            })?;

        let mut form = Form::new().part(
            "file",
            Part::text(contents)
                .mime_str("text/csv")?
                .file_name("foo.csv"),
        );
        form = Self::add_non_json_part(form, "on_conflict", contact_import.on_conflict.as_ref())?;
        form = Self::add_non_json_part(form, "segments", contact_import.segments.as_ref())?;
        form = Self::add_non_json_part(form, "topics", contact_import.topics.as_ref())?;

        if let Some(column_map) = contact_import.column_map {
            let json = serde_json::to_string(&column_map).map_err(|e| Error::Parse {
                message: "Could not convert field to JSON".to_owned(),
                source: Some(Box::new(e)),
            })?;
            form = form.part("column_map".to_owned(), Part::text(json));
        }

        let request = self.0.build(Method::POST, path).multipart(form);
        let response = self.0.send(request).await?;
        let content = response.json::<CreateContactImportResponse>().await?;

        Ok(content)
    }

    fn add_non_json_part<T: serde::Serialize>(
        form: Form,
        field_name: &str,
        value: Option<&T>,
    ) -> Result<Form, Error> {
        match value {
            Some(v) => {
                let val = serde_json::to_value(v).map_err(|e| Error::Parse {
                    message: "Could not convert field to JSON".to_owned(),
                    source: Some(Box::new(e)),
                })?;

                let json = match val {
                    serde_json::Value::String(s) => s,
                    other => other.to_string(),
                };

                Ok(form.part(field_name.to_owned(), Part::text(json)))
            }
            None => Ok(form),
        }
    }

    /// Retrieve a single contact import.
    ///
    /// <https://resend.com/docs/api-reference/contacts/get-contact-import>
    #[maybe_async::maybe_async]
    pub async fn get_import(&self, contact_import_id: &str) -> Result<ContactImport> {
        let path = format!("/contacts/imports/{contact_import_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<ContactImport>().await?;

        Ok(content)
    }

    /// Retrieve a list of contact imports.
    ///
    /// - Default limit: 10
    ///
    /// <https://resend.com/docs/api-reference/contacts/list-contact-imports>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn list_imports<T>(
        &self,
        list_opts: ListOptions<T>,
    ) -> Result<ListResponse<ContactImport>> {
        let path = "/contacts/imports".to_string();

        let request = self.0.build(Method::GET, &path).query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<ContactImport>>().await?;

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
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    use crate::{
        topics::types::TopicId,
        types::{SegmentId, SubscriptionType},
    };

    crate::define_id_type!(ContactId);
    crate::define_id_type!(ContactPropertyId);
    crate::define_id_type!(ContactImportId);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SegmentObject {
        pub id: SegmentId,
    }

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
        /// Custom properties for the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        properties: Option<HashMap<String, String>>,
        /// Segment IDs to add the contact to.
        #[serde(skip_serializing_if = "Option::is_none")]
        segments: Option<Vec<SegmentObject>>,
        /// Topic subscriptions for the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        topics: Option<Vec<UpdateContactTopicOptions>>,
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
                properties: None,
                segments: None,
                topics: None,
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

        /// Adds a custom property to the contact.
        #[inline]
        pub fn with_property(mut self, key: &str, value: &str) -> Self {
            let properties = self.properties.get_or_insert_with(HashMap::new);
            let _old = properties.insert(key.to_owned(), value.to_owned());
            self
        }

        /// Adds custom properties to the contact.
        #[inline]
        pub fn with_properties(mut self, properties: HashMap<String, String>) -> Self {
            let self_properties = self.properties.get_or_insert_with(HashMap::new);
            self_properties.extend(properties);
            self
        }

        /// Adds a segment ID to add the contact to.
        #[inline]
        pub fn with_segment(mut self, id: &str) -> Self {
            let id = SegmentObject {
                id: SegmentId::new(id),
            };
            let segments = self.segments.get_or_insert_with(Vec::new);
            segments.push(id);
            self
        }

        /// Adds multiple segment IDs to add the contact to.
        #[inline]
        pub fn with_segments(mut self, ids: &[String]) -> Self {
            let segments = self.segments.get_or_insert_with(Vec::new);
            for id in ids {
                let id = SegmentObject {
                    id: SegmentId::new(id),
                };
                segments.push(id);
            }
            self
        }

        /// Adds a topic subscription for the contact.
        #[inline]
        #[allow(clippy::needless_pass_by_value)]
        pub fn with_topic(mut self, topic: UpdateContactTopicOptions) -> Self {
            let topics = self.topics.get_or_insert_with(Vec::new);
            topics.push(topic);
            self
        }

        /// Adds multiple topic subscriptions for the contact.
        #[inline]
        #[allow(clippy::needless_pass_by_value)]
        pub fn with_topics(mut self, topics: &[UpdateContactTopicOptions]) -> Self {
            let topics_vec = self.topics.get_or_insert_with(Vec::new);
            topics_vec.extend_from_slice(topics);
            self
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreateContactResponse {
        /// Unique identifier for the created contact.
        pub id: ContactId,
    }

    /// Details of an existing contact.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
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
        /// Custom properties for the contact.
        #[serde(default)]
        properties: Option<HashMap<String, ContactPropertyResponse>>,
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

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateContactResponse {
        /// Unique identifier for the updated contact.
        pub id: ContactId,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DeleteContactResponse {
        /// The ID of the contact.
        #[allow(dead_code)]
        pub contact: ContactId,
        /// Indicates whether the contact was deleted successfully.
        pub deleted: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
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
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AddContactSegmentResponse {
        pub id: SegmentId,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RemoveContactSegmentResponse {
        pub id: SegmentId,
        pub deleted: bool,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
    #[must_use]
    #[serde(rename_all = "snake_case")]
    pub enum PropertyType {
        String,
        Number,
    }

    /// Details of a new [`ContactProperty`].
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateContactPropertyOptions {
        key: String,
        #[serde(rename = "type")]
        r#type: PropertyType,
        fallback_value: Option<serde_json::Value>,
    }

    impl CreateContactPropertyOptions {
        /// - `key`: The property key. Max length is `50` characters. Only alphanumeric characters
        ///   and underscores are allowed.
        /// - `r#type`: The property type.
        pub fn new(key: impl Into<String>, r#type: PropertyType) -> Self {
            Self {
                key: key.into(),
                r#type,
                fallback_value: None,
            }
        }

        /// The default value to use when the property is not set for a contact. Must match the
        /// type specified in the `r#type` field.
        pub fn with_fallback(mut self, fallback: impl Into<serde_json::Value>) -> Self {
            self.fallback_value = Some(fallback.into());
            self
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreateContactPropertyResponse {
        pub id: ContactPropertyId,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ContactProperty {
        pub id: ContactPropertyId,
        pub created_at: String,
        pub key: String,
        #[serde(rename = "type")]
        pub r#type: PropertyType,
        pub fallback_value: Option<serde_json::Value>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ContactPropertyResponse {
        pub value: String,
        #[serde(rename = "type")]
        pub r#type: PropertyType,
    }

    /// List of changes to apply to a [`ContactProperty`].
    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct ContactPropertyChanges {
        #[serde(skip_serializing_if = "Option::is_none")]
        fallback_value: Option<serde_json::Value>,
    }

    impl ContactPropertyChanges {
        /// The default value to use when the property is not set for a contact. Must match the
        /// type of the property.
        pub fn with_fallback(mut self, fallback: impl Into<serde_json::Value>) -> Self {
            self.fallback_value = Some(fallback.into());
            self
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateContactPropertyResponse {
        /// Unique identifier for the updated contact property.
        pub id: ContactPropertyId,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DeleteContactPropertyResponse {
        #[allow(dead_code)]
        pub id: ContactPropertyId,
        pub deleted: bool,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Default)]
    pub struct CreateContactImportOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) column_map: Option<ContactImportColumnMap>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) on_conflict: Option<ContactImportOnConflict>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) segments: Option<Vec<SegmentObject>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) topics: Option<Vec<ContactImportTopic>>,
    }

    impl CreateContactImportOptions {
        /// Creates a new [`CreateContactImportOptions`].
        pub fn new() -> Self {
            Self {
                column_map: None,
                on_conflict: None,
                segments: None,
                topics: None,
            }
        }

        #[inline]
        pub fn with_column_map(mut self, column_map: ContactImportColumnMap) -> Self {
            self.column_map = Some(column_map);
            self
        }

        #[inline]
        pub fn with_on_conflict(mut self, on_conflict: ContactImportOnConflict) -> Self {
            self.on_conflict = Some(on_conflict);
            self
        }

        /// Adds a segment ID to add the contact import to.
        #[inline]
        pub fn with_segment(mut self, id: &str) -> Self {
            let id = SegmentObject {
                id: SegmentId::new(id),
            };
            let segments = self.segments.get_or_insert_with(Vec::new);
            segments.push(id);
            self
        }

        /// Adds multiple segment IDs to add the contact import to.
        #[inline]
        pub fn with_segments(mut self, ids: &[String]) -> Self {
            let segments = self.segments.get_or_insert_with(Vec::new);
            for id in ids {
                let id = SegmentObject {
                    id: SegmentId::new(id),
                };
                segments.push(id);
            }
            self
        }

        /// Adds a topic subscription for the contact import.
        #[inline]
        #[allow(clippy::needless_pass_by_value)]
        pub fn with_topic(mut self, topic: ContactImportTopic) -> Self {
            let topics = self.topics.get_or_insert_with(Vec::new);
            topics.push(topic);
            self
        }

        /// Adds multiple topic subscriptions for the contact import.
        #[inline]
        #[allow(clippy::needless_pass_by_value)]
        pub fn with_topics(mut self, topics: &[ContactImportTopic]) -> Self {
            let topics_vec = self.topics.get_or_insert_with(Vec::new);
            topics_vec.extend_from_slice(topics);
            self
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Default)]
    pub struct ContactImportColumnMap {
        #[serde(skip_serializing_if = "Option::is_none")]
        email: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        first_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        last_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        unsubscribed: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        properties: Option<HashMap<String, ContactImportPropertyMapping>>,
    }

    impl ContactImportColumnMap {
        pub fn new() -> Self {
            Self {
                email: None,
                first_name: None,
                last_name: None,
                unsubscribed: None,
                properties: None,
            }
        }

        pub fn with_email(mut self, email: &str) -> Self {
            self.email = Some(email.to_owned());
            self
        }

        pub fn with_first_name(mut self, first_name: &str) -> Self {
            self.first_name = Some(first_name.to_owned());
            self
        }

        pub fn with_last_name(mut self, last_name: &str) -> Self {
            self.last_name = Some(last_name.to_owned());
            self
        }

        pub fn with_unsubscribed(mut self, unsubscribed: &str) -> Self {
            self.unsubscribed = Some(unsubscribed.to_owned());
            self
        }

        /// Adds a custom property.
        #[inline]
        #[allow(clippy::needless_pass_by_value)]
        pub fn with_property(mut self, key: &str, value: ContactImportPropertyMapping) -> Self {
            let properties = self.properties.get_or_insert_with(HashMap::new);
            let _old = properties.insert(key.to_owned(), value);
            self
        }

        /// Adds custom properties.
        #[inline]
        pub fn with_properties(
            mut self,
            properties: HashMap<String, ContactImportPropertyMapping>,
        ) -> Self {
            let self_properties = self.properties.get_or_insert_with(HashMap::new);
            self_properties.extend(properties);
            self
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct ContactImportPropertyMapping {
        pub column: String,
        #[serde(rename = "type")]
        pub r#type: ContactImportPropertyType,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
    #[must_use]
    #[serde(rename_all = "snake_case")]
    pub enum ContactImportPropertyType {
        String,
        Number,
        Boolean,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
    #[must_use]
    #[serde(rename_all = "snake_case")]
    pub enum ContactImportOnConflict {
        Upsert,
        Skip,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreateContactImportResponse {
        /// Unique identifier for the created contact import.
        pub id: ContactImportId,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ContactImportTopic {
        pub id: String,
        pub subscription: ContactImportTopicSubscription,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
    #[must_use]
    #[serde(rename_all = "snake_case")]
    pub enum ContactImportTopicSubscription {
        OptIn,
        OptOut,
    }

    // counts: ContactImportCounts;
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ContactImport {
        pub id: ContactImportId,
        pub status: ContactImportStatus,
        pub created_at: String,
        pub completed_at: Option<String>,
        pub counts: ContactImportCounts,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
    #[must_use]
    #[serde(rename_all = "snake_case")]
    pub enum ContactImportStatus {
        Queued,
        InProgress,
        Completed,
        Failed,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Copy)]
    pub struct ContactImportCounts {
        total: u32,
        created: u32,
        updated: u32,
        skipped: u32,
        failed: u32,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::needless_return)]
mod test {
    use std::collections::HashMap;

    use crate::test::{CLIENT, DebugResult};
    use crate::types::{
        ContactChanges, ContactProperty, CreateContactOptions, CreateTopicOptions,
        SubscriptionType, UpdateContactTopicOptions,
    };
    use crate::{list_opts::ListOptions, types::Contact};

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

        std::thread::sleep(std::time::Duration::from_secs(4));

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    #[ignore = "Flaky backend"]
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

        // Update.
        let changes = ContactChanges::new().with_unsubscribed(true);
        let _contact = resend.contacts.update(&id, changes).await?;
        std::thread::sleep(std::time::Duration::from_secs(4));

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

        // Delete topic
        let deleted = resend.topics.delete(&topic.id).await?;
        assert!(deleted.deleted);

        // List.
        let contacts = resend
            .contacts
            .list(&audience_id, ListOptions::default())
            .await?;
        assert!(contacts.is_empty());

        std::thread::sleep(std::time::Duration::from_secs(4));

        Ok(())
    }

    #[ignore = "Flaky backend"]
    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn contact_properties() -> DebugResult<()> {
        use crate::types::CreateContactPropertyOptions;

        let resend = &*CLIENT;

        let contact_property =
            CreateContactPropertyOptions::new("key", crate::types::PropertyType::String);
        let contact_property = resend.contacts.create_property(contact_property).await?;

        let contact = CreateContactOptions::new("steve.wozniak@gmail.com")
            .with_first_name("Steve")
            .with_last_name("Wozniak")
            .with_unsubscribed(false)
            .with_property("key", "value");

        let contact = resend.contacts.create(contact).await?;

        let contact = resend.contacts.get(&contact).await?;

        let deleted = resend.contacts.delete(&contact.id).await?;
        assert!(deleted);
        let deleted = resend
            .contacts
            .delete_property(&contact_property.id)
            .await?;
        assert!(deleted.deleted);

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    #[ignore = "Flaky backend"]
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
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Delete
        let deleted = resend.contacts.delete(&contact_id).await?;
        assert!(deleted);
        let deleted = resend.segments.delete(&segment.id).await?;
        assert!(deleted);

        std::thread::sleep(std::time::Duration::from_secs(4));

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    #[ignore = "Flaky backend"]
    async fn properties() -> DebugResult<()> {
        use crate::{
            contacts::types::ContactPropertyChanges,
            types::{CreateContactPropertyOptions, PropertyType},
        };

        let resend = &*CLIENT;

        // Create
        let contact_property =
            CreateContactPropertyOptions::new("company_name", PropertyType::String)
                .with_fallback("Acme Corp");
        let contact_property = resend.contacts.create_property(contact_property).await?;
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Get
        let contact_property = resend.contacts.get_property(&contact_property.id).await?;

        // Update
        let update = ContactPropertyChanges::default().with_fallback("Example Company");
        let contact_property = resend
            .contacts
            .update_property(&contact_property.id, update)
            .await?;

        // List
        let contact_properties = resend
            .contacts
            .list_properties(ListOptions::default())
            .await?;
        assert!(!contact_properties.is_empty());

        // Delete
        let deleted = resend
            .contacts
            .delete_property(&contact_property.id)
            .await?;
        assert!(deleted.deleted);

        std::thread::sleep(std::time::Duration::from_secs(4));

        Ok(())
    }

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    #[ignore = "Flaky backend"]
    async fn contact_import() -> DebugResult<()> {
        use crate::{
            contacts::types::{
                ContactImportColumnMap, ContactImportOnConflict::Upsert,
                ContactImportPropertyMapping, ContactImportPropertyType::String,
            },
            types::CreateContactImportOptions,
        };
        use std::fs::File;
        use std::io::prelude::*;

        let resend = &*CLIENT;

        // Create
        let mut file = File::create("foo.csv").unwrap();
        file.write_all(
            b"Email,First Name,Last Name,Plan\nonboarding@resend.dev,John,Onboarding,El Plan",
        )
        .unwrap();

        let import = CreateContactImportOptions::new()
            .with_column_map(
                ContactImportColumnMap::new()
                    .with_email("Email")
                    .with_first_name("First Name")
                    .with_last_name("Last Name")
                    .with_property(
                        "plan",
                        ContactImportPropertyMapping {
                            column: "Plan".to_owned(),
                            r#type: String,
                        },
                    ),
            )
            .with_on_conflict(Upsert);

        let import = resend
            .contacts
            .create_import(File::open("foo.csv").unwrap(), import)
            .await?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        // Get
        let _import = resend.contacts.get_import(&import.id).await?;

        // List
        let imports = resend.contacts.list_imports(ListOptions::default()).await?;
        assert!(!imports.is_empty());

        // Delete
        let deleted = resend.contacts.delete("onboarding@resend.dev").await?;
        assert!(deleted);
        let properties = resend
            .contacts
            .list_properties(ListOptions::default())
            .await?;
        let deleted = resend.contacts.delete_property(&properties[0].id).await?;
        assert!(deleted.deleted);

        std::fs::remove_file("foo.csv").unwrap();

        Ok(())
    }

    #[test]
    fn deserialize_test() {
        let contact_property = r#"{
  "object": "contact_property",
  "id": "b6d24b8e-af0b-4c3c-be0c-359bbd97381e",
  "key": "company_name",
  "type": "string",
  "fallback_value": "Acme Corp",
  "created_at": "2023-04-08T00:11:13.110779+00:00"
}"#;

        let res = serde_json::from_str::<ContactProperty>(contact_property);
        assert!(res.is_ok());
    }

    #[test]
    fn deserialize_test2() {
        let contact_property = r#"{
          "object": "contact",
          "id": "257d6d3b-e796-464d-ba7d-8ccabc58d16d",
          "email": "steve.wozniak@gmail.com",
          "first_name": "Steve",
          "last_name": "Wozniak",
          "created_at": "2025-07-21 23:58:57.096708+00",
          "unsubscribed": false,
          "properties": {
            "key": {
              "value": "value",
              "type": "string"
            }
          }
        }"#;

        let res = serde_json::from_str::<Contact>(contact_property);
        assert!(res.is_ok());
    }

    #[test]
    #[allow(clippy::indexing_slicing)]
    fn serialize_create_contact_with_extras() {
        use crate::types::{SubscriptionType, UpdateContactTopicOptions};

        let topic = UpdateContactTopicOptions::new("topic_123", SubscriptionType::OptIn);
        let mut properties = HashMap::new();
        let _old = properties.insert("company".to_owned(), "Acme Corp".to_owned());

        let contact = CreateContactOptions::new("test@example.com")
            .with_first_name("John")
            .with_last_name("Doe")
            .with_property("department", "Sales")
            .with_properties(properties)
            .with_segment("segment_123")
            .with_segments(&["segment_456".to_string()])
            .with_topic(topic)
            .with_topics(&[UpdateContactTopicOptions::new(
                "topic_789",
                SubscriptionType::OptOut,
            )]);

        let json = serde_json::to_value(&contact).expect("Failed to serialize");

        // Verify structure
        assert_eq!(json["email"], "test@example.com");
        assert_eq!(json["first_name"], "John");
        assert_eq!(json["last_name"], "Doe");

        // Verify properties
        assert!(json["properties"].is_object());
        let properties = json["properties"]
            .as_object()
            .expect("properties should be a map");
        assert_eq!(properties.len(), 2);
        assert!(properties.contains_key("department"));
        assert_eq!(
            properties.get("department"),
            Some(serde_json::Value::String("Sales".to_owned())).as_ref()
        );
        assert_eq!(
            properties.get("company"),
            Some(serde_json::Value::String("Acme Corp".to_owned())).as_ref()
        );

        // Verify segments
        assert!(json["segments"].is_array());
        let segments = json["segments"]
            .as_array()
            .expect("segments should be an array");
        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0], serde_json::json!({"id": "segment_123"}));
        assert_eq!(segments[1], serde_json::json!({"id": "segment_456"}));

        // Verify topics
        assert!(json["topics"].is_array());
        let topics = json["topics"]
            .as_array()
            .expect("topics should be an array");
        assert_eq!(topics.len(), 2);
        assert_eq!(topics[0]["id"], "topic_123");
        assert_eq!(topics[0]["subscription"], "opt_in");
        assert_eq!(topics[1]["id"], "topic_789");
        assert_eq!(topics[1]["subscription"], "opt_out");
    }
}
