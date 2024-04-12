use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use crate::types::{CreateContactRequest, CreateContactResponse};
use crate::types::{GetContactResponse, ListContactsResponse};
use crate::types::{UpdateContactRequest, UpdateContactResponse};
use crate::{Config, Result};

/// `Resend` APIs for `METHOD /audiences/:id/contacts` endpoints.
#[derive(Clone)]
pub struct Contacts(pub(crate) Arc<Config>);

impl Contacts {
    /// Create a contact inside an audience.
    ///
    /// <https://resend.com/docs/api-reference/contacts/create-contact>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn add(
        &self,
        audience_id: &str,
        contact: CreateContactRequest,
    ) -> Result<CreateContactResponse> {
        let path = format!("/audiences/{audience_id}/contacts");

        let request = self.0.build(Method::POST, &path);
        let response = request.json(&contact).send().await?;
        let content = response.json::<CreateContactResponse>().await?;

        Ok(content)
    }

    /// Retrieve a single contact from an audience.
    ///
    /// <https://resend.com/docs/api-reference/contacts/get-contact>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn retrieve(
        &self,
        contact_id: &str,
        audience_id: &str,
    ) -> Result<GetContactResponse> {
        let path = format!("/audiences/{audience_id}/contacts/{contact_id}");

        let request = self.0.build(Method::GET, &path);
        let response = request.send().await?;
        let content = response.json::<GetContactResponse>().await?;

        Ok(content)
    }

    /// Update an existing contact.
    ///
    /// <https://resend.com/docs/api-reference/contacts/update-contact>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn update(
        &self,
        contact_id: &str,
        audience_id: &str,
        contact: UpdateContactRequest,
    ) -> Result<UpdateContactResponse> {
        let path = format!("/audiences/{audience_id}/contacts/{contact_id}");

        let request = self.0.build(Method::PATCH, &path);
        let response = request.json(&contact).send().await?;
        let content = response.json::<UpdateContactResponse>().await?;

        Ok(content)
    }

    /// Remove an existing contact from an audience by their email or ID.
    ///
    /// <https://resend.com/docs/api-reference/contacts/delete-contact>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn delete(&self, audience_id: &str, email_or_id: &str) -> Result<()> {
        let path = format!("/audiences/{audience_id}/contacts/{email_or_id}");

        let request = self.0.build(Method::DELETE, &path);
        let _response = request.send().await?;

        Ok(())
    }

    /// Show all contacts from an audience.
    ///
    /// <https://resend.com/docs/api-reference/contacts/list-contacts>
    #[cfg(not(feature = "blocking"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "blocking"))))]
    pub async fn list(&self, audience_id: &str) -> Result<ListContactsResponse> {
        let path = format!("/audiences/{audience_id}/contacts");

        let request = self.0.build(Method::GET, &path);
        let response = request.send().await?;
        let content = response.json::<ListContactsResponse>().await?;

        Ok(content)
    }

    /// Create a contact inside an audience.
    ///
    /// <https://resend.com/docs/api-reference/contacts/create-contact>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn add(
        &self,
        audience_id: &str,
        contact: CreateContactRequest,
    ) -> Result<CreateContactResponse> {
        let path = format!("/audiences/{audience_id}/contacts");

        let request = self.0.build(Method::POST, &path);
        let response = request.json(&contact).send()?;
        let content = response.json::<CreateContactResponse>()?;

        Ok(content)
    }

    /// Retrieve a single contact from an audience.
    ///
    /// <https://resend.com/docs/api-reference/contacts/get-contact>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn retrieve(&self, contact_id: &str, audience_id: &str) -> Result<GetContactResponse> {
        let path = format!("/audiences/{audience_id}/contacts/{contact_id}");

        let request = self.0.build(Method::GET, &path);
        let response = request.send()?;
        let content = response.json::<GetContactResponse>()?;

        Ok(content)
    }

    /// Update an existing contact.
    ///
    /// <https://resend.com/docs/api-reference/contacts/update-contact>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn update(
        &self,
        contact_id: &str,
        audience_id: &str,
        contact: UpdateContactRequest,
    ) -> Result<UpdateContactResponse> {
        let path = format!("/audiences/{audience_id}/contacts/{contact_id}");

        let request = self.0.build(Method::PATCH, &path);
        let response = request.json(&contact).send()?;
        let content = response.json::<UpdateContactResponse>()?;

        Ok(content)
    }

    /// Remove an existing contact from an audience by their email or ID.
    ///
    /// <https://resend.com/docs/api-reference/contacts/delete-contact>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn delete(&self, audience_id: &str, email_or_id: &str) -> Result<()> {
        let path = format!("/audiences/{audience_id}/contacts/{email_or_id}");

        let request = self.0.build(Method::DELETE, &path);
        let _response = request.send()?;

        Ok(())
    }

    /// Show all contacts from an audience.
    ///
    /// <https://resend.com/docs/api-reference/contacts/list-contacts>
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn list(&self, audience_id: &str) -> Result<ListContactsResponse> {
        let path = format!("/audiences/{audience_id}/contacts");

        let request = self.0.build(Method::GET, &path);
        let response = request.send()?;
        let content = response.json::<ListContactsResponse>()?;

        Ok(content)
    }
}

impl fmt::Debug for Contacts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}
