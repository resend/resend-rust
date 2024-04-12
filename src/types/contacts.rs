use serde::{Deserialize, Serialize};

#[must_use]
#[derive(Debug, Clone, Serialize)]
pub struct CreateContactRequest {
    /// Email address of the contact.
    pub email: String,
    /// First name of the contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name of the contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Indicates if the contact is unsubscribed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribed: Option<bool>,
    /// Unique identifier of the audience to which the contact belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience_id: Option<String>,
}

impl CreateContactRequest {
    /// Creates a new [`CreateContactRequest`].
    pub fn new(email: &str) -> Self {
        Self {
            email: email.to_owned(),
            first_name: None,
            last_name: None,
            unsubscribed: None,
            audience_id: None,
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

    /// Adds a contact to the audience.
    #[inline]
    pub fn with_audience(mut self, id: &str) -> Self {
        self.audience_id = Some(id.to_owned());
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateContactResponse {
    /// Type of the response object.
    pub object: Option<String>,
    /// Unique identifier for the created contact.
    pub id: Option<String>,
}

#[must_use]
#[derive(Debug, Clone, Deserialize)]
pub struct ListContactsResponse {
    /// Type of the response object.
    pub object: Option<String>,
    /// Array containing contact information.
    pub data: Option<Vec<ListContactsItem>>,
}

#[must_use]
#[derive(Debug, Clone, Deserialize)]
pub struct GetContactResponse {
    /// Type of the response object.
    pub object: Option<String>,
    /// Unique identifier for the contact.
    pub id: Option<String>,
    /// Email address of the contact.
    pub email: Option<String>,
    /// First name of the contact.
    pub first_name: Option<String>,
    /// Last name of the contact.
    pub last_name: Option<String>,
    /// Timestamp indicating when the contact was created.
    pub created_at: Option<String>,
    /// Indicates if the contact is unsubscribed.
    pub unsubscribed: Option<bool>,
}

#[must_use]
#[derive(Debug, Clone, Serialize)]
pub struct UpdateContactRequest {
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

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateContactResponse {
    /// Type of the response object.
    pub object: Option<String>,
    /// Unique identifier for the updated contact.
    pub id: Option<String>,
}

#[must_use]
#[derive(Debug, Clone, Deserialize)]
pub struct ListContactsItem {
    /// Unique identifier for the contact.
    pub id: Option<String>,
    /// Email address of the contact.
    pub email: Option<String>,
    /// First name of the contact.
    pub first_name: Option<String>,
    /// Last name of the contact.
    pub last_name: Option<String>,
    /// Timestamp indicating when the contact was created.
    pub created_at: Option<String>,
    /// Indicates if the contact is unsubscribed.
    pub unsubscribed: Option<bool>,
}
