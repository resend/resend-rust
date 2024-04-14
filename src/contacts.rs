use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use crate::{Config, Result};
use crate::types::{Contact, Contacts};
use crate::types::{ContactChanges, UpdateContactResponse};
use crate::types::{CreateContactResponse, NewContact};

/// `Resend` APIs for `/audiences/:id/contacts` endpoints.
#[derive(Clone)]
pub struct ContactsService(pub(crate) Arc<Config>);

impl ContactsService {
    /// Create a contact inside an audience.
    ///
    /// <https://resend.com/docs/api-reference/contacts/create-contact>
    #[maybe_async::maybe_async]
    pub async fn create(&self, contact: NewContact) -> Result<CreateContactResponse> {
        let path = format!("/audiences/{}/contacts", &contact.audience_id);

        let request = self.0.build(Method::POST, &path);
        let response = self.0.send(request.json(&contact)).await?;
        let content = response.json::<CreateContactResponse>().await?;

        Ok(content)
    }

    /// Retrieve a single contact from an audience.
    ///
    /// <https://resend.com/docs/api-reference/contacts/get-contact>
    #[maybe_async::maybe_async]
    pub async fn retrieve(&self, contact_id: &str, audience_id: &str) -> Result<Contact> {
        let path = format!("/audiences/{audience_id}/contacts/{contact_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Contact>().await?;

        Ok(content)
    }

    /// Update an existing contact.
    ///
    /// <https://resend.com/docs/api-reference/contacts/update-contact>
    #[maybe_async::maybe_async]
    pub async fn update(
        &self,
        contact_id: &str,
        audience_id: &str,
        contact: ContactChanges,
    ) -> Result<()> {
        let path = format!("/audiences/{audience_id}/contacts/{contact_id}");

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&contact)).await?;
        let content = response.json::<UpdateContactResponse>().await?;
        println!("contact_id: {}", contact_id);
        println!("content_id: {}", content.id.as_str());

        Ok(())
    }

    /// Remove an existing contact from an audience by their email or ID.
    ///
    /// <https://resend.com/docs/api-reference/contacts/delete-contact>
    #[maybe_async::maybe_async]
    pub async fn delete(&self, audience_id: &str, email_or_id: &str) -> Result<()> {
        let path = format!("/audiences/{audience_id}/contacts/{email_or_id}");

        let request = self.0.build(Method::DELETE, &path);
        let _response = self.0.send(request).await?;

        Ok(())
    }

    /// Show all contacts from an audience.
    ///
    /// <https://resend.com/docs/api-reference/contacts/list-contacts>
    #[maybe_async::maybe_async]
    pub async fn list(&self, audience_id: &str) -> Result<Contacts> {
        let path = format!("/audiences/{audience_id}/contacts");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Contacts>().await?;

        Ok(content)
    }
}

impl fmt::Debug for ContactsService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

pub mod types {
    use serde::{Deserialize, Serialize};

    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct NewContact {
        /// Email address of the contact.
        pub email: String,
        /// Unique identifier of the audience to which the contact belongs.
        // TODO: Should it really be here?
        pub audience_id: String,

        /// First name of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub first_name: Option<String>,
        /// Last name of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub last_name: Option<String>,
        /// Indicates if the contact is unsubscribed.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unsubscribed: Option<bool>,
    }

    impl NewContact {
        /// Creates a new [`NewContact`].
        pub fn new(email: &str, audience_id: &str) -> Self {
            Self {
                email: email.to_owned(),
                audience_id: audience_id.to_owned(),
                first_name: None,
                last_name: None,
                unsubscribed: None,
            }
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
        pub fn with_unsubscribed(mut self, unsubscribed: bool) -> Self {
            self.unsubscribed = Some(unsubscribed);
            self
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct CreateContactResponse {
        /// Unique identifier for the created contact.
        pub id: String,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct Contacts {
        /// Array containing contact information.
        #[serde(rename = "data")]
        pub contacts: Vec<Contact>,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct Contact {
        /// Unique identifier for the contact.
        pub id: String,
        /// Email address of the contact.
        pub email: String,
        /// First name of the contact.
        pub first_name: String,
        /// Last name of the contact.
        pub last_name: String,
        /// Timestamp indicating when the contact was created.
        pub created_at: String,
        /// Indicates if the contact is unsubscribed.
        pub unsubscribed: bool,
    }

    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct ContactChanges {
        /// Email address of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        /// First name of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub first_name: Option<String>,
        /// Last name of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub last_name: Option<String>,
        /// Indicates the subscription status of the contact.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unsubscribed: Option<bool>,
    }

    impl ContactChanges {
        /// Creates a new [`ContactChanges`].
        #[inline]
        pub fn new() -> Self {
            Self::default()
        }

        /// Updates the emails of the contact.
        #[inline]
        pub fn with_email(mut self, email: &str) -> Self {
            self.email = Some(email.to_owned());
            self
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
        pub fn with_unsubscribed(mut self, unsubscribed: bool) -> Self {
            self.unsubscribed = Some(unsubscribed);
            self
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct UpdateContactResponse {
        /// Unique identifier for the updated contact.
        pub id: String,
    }
}

#[cfg(test)]
mod test {
    use crate::{Client, Result};
    use crate::types::{ContactChanges, NewContact};

    #[tokio::test]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> Result<()> {
        let resend = Client::default();

        // Create audience.
        let audience = "test_";
        let status = resend.audiences.create(audience).await?;
        let audience_id = status.id.as_str();

        // Create.
        let contact = NewContact::new("antonios.barotsis@pm.me", audience_id)
            .with_first_name("Antonios")
            .with_last_name("Barotsis")
            .with_unsubscribed(false);
        let response = resend.contacts.create(contact).await?;
        let id = response.id.as_str();

        // Update.
        let changes = ContactChanges::new().with_unsubscribed(true);
        resend.contacts.update(id, audience_id, changes).await?;

        // Retrieve.
        let contact = resend.contacts.retrieve(id, audience_id).await?;
        assert!(contact.unsubscribed);

        // List.
        let response = resend.contacts.list(audience_id).await?;
        assert_eq!(response.contacts.len(), 1);

        // Delete.
        resend.contacts.delete(audience_id, id).await?;

        // Delete audience.
        let _ = resend.audiences.delete(audience_id).await?;

        Ok(())
    }
}
