use std::sync::Arc;

use reqwest::Method;

use crate::{
    Config, Result,
    list_opts::{ListOptions, ListResponse},
    suppressions::types::SpecifiedMarker,
    types::{
        AddSuppressionOptions, AddSuppressionResponse, BatchAddSuppressionOptions,
        BatchAddSuppressionResponse, BatchRemoveSuppressionOptions,
        BatchRemoveSuppressionsResponse, RemoveSuppressionResponse, Suppression,
    },
};

/// `Resend` APIs for `/suppressions` endpoints.
#[derive(Clone, Debug)]
pub struct SuppressionsSvc(pub(crate) Arc<Config>);

impl SuppressionsSvc {
    /// Add an email address to the suppression list.
    ///
    /// <https://resend.com/docs/api-reference/suppressions/add-suppression>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn add(&self, opts: AddSuppressionOptions) -> Result<AddSuppressionResponse> {
        let request = self.0.build(Method::POST, "/suppressions");
        let response = self.0.send(request.json(&opts)).await?;
        let content = response.json::<AddSuppressionResponse>().await?;

        Ok(content)
    }

    /// Retrieve a single suppression by ID or email.
    ///
    /// <https://resend.com/docs/api-reference/suppressions/get-suppression>
    #[maybe_async::maybe_async]
    pub async fn get(&self, id_or_email: &str) -> Result<Suppression> {
        let id_or_email = urlencoding::encode(id_or_email);
        let path = format!("/suppressions/{id_or_email}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Suppression>().await?;

        Ok(content)
    }

    /// Show all suppressions.
    ///
    /// - Default limit: 20
    ///
    /// <https://resend.com/docs/api-reference/suppressions/list-suppressions>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn list<T>(&self, list_opts: ListOptions<T>) -> Result<ListResponse<Suppression>> {
        let request = self.0.build(Method::GET, "/suppressions").query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<Suppression>>().await?;

        Ok(content)
    }

    /// Remove a single suppression by ID or email.
    ///
    /// <https://resend.com/docs/api-reference/suppressions/remove-suppression>
    #[maybe_async::maybe_async]
    pub async fn remove(&self, id_or_email: &str) -> Result<RemoveSuppressionResponse> {
        let id_or_email = urlencoding::encode(id_or_email);
        let path = format!("/suppressions/{id_or_email}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<RemoveSuppressionResponse>().await?;

        Ok(content)
    }

    /// Add up to 100 email addresses to the suppression list at once.
    ///
    /// <https://resend.com/docs/api-reference/suppressions/add-suppressions>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn batch_add(
        &self,
        opts: BatchAddSuppressionOptions,
    ) -> Result<BatchAddSuppressionResponse> {
        let request = self.0.build(Method::POST, "/suppressions/batch/add");
        let response = self.0.send(request.json(&opts)).await?;
        let content = response.json::<BatchAddSuppressionResponse>().await?;

        Ok(content)
    }

    /// Remove up to 100 suppressions from the suppression list at once.
    ///
    /// This endpoint requires that you have specified either ids or emails but not both,
    /// hence you need [`BatchRemoveSuppressionOptions<EmailsSpecified>`] or
    /// [`BatchRemoveSuppressionOptions<IdsSpecified>`].
    ///
    /// You can create those by doing:
    ///
    /// ```rust
    /// # use resend_rs::types::BatchRemoveSuppressionOptions;
    /// let _tmp = BatchRemoveSuppressionOptions::new().add_emails(vec!["emails"]);
    /// let _tmp = BatchRemoveSuppressionOptions::new().add_ids(vec!["ids"]);
    /// ```
    ///
    /// <https://resend.com/docs/api-reference/suppressions/remove-suppressions>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value, private_bounds)]
    pub async fn batch_remove<M: SpecifiedMarker>(
        &self,
        ids_or_emails: BatchRemoveSuppressionOptions<M>,
    ) -> Result<BatchRemoveSuppressionsResponse> {
        let request = self.0.build(Method::POST, "/suppressions/batch/remove");
        let response = self.0.send(request.json(&ids_or_emails)).await?;
        let content = response.json::<BatchRemoveSuppressionsResponse>().await?;

        Ok(content)
    }
}

#[allow(unreachable_pub)]
pub mod types {
    use std::borrow::ToOwned;

    use serde::{Deserialize, Serialize};

    use crate::types::EmailId;

    crate::define_id_type!(SuppressionId);

    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct AddSuppressionOptions {
        email: String,
    }

    impl AddSuppressionOptions {
        #[inline]
        pub fn new() -> Self {
            Self::default()
        }

        #[inline]
        pub fn with_email(mut self, email: &str) -> Self {
            email.clone_into(&mut self.email);
            self
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct AddSuppressionResponse {
        pub id: SuppressionId,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct Suppression {
        pub id: SuppressionId,
        pub email: String,
        pub created_at: String,
        pub origin: SuppressionOrigin,
        pub source_id: Option<EmailId>,
    }

    #[must_use]
    #[derive(Debug, Clone, Copy, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum SuppressionOrigin {
        Bounce,
        Complaint,
        Manual,
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct RemoveSuppressionResponse {
        pub id: SuppressionId,
        pub deleted: bool,
    }

    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct BatchAddSuppressionOptions {
        emails: Vec<String>,
    }

    impl BatchAddSuppressionOptions {
        #[inline]
        pub fn new() -> Self {
            Self::default()
        }

        #[inline]
        pub fn add_email(mut self, email: &str) -> Self {
            self.emails.push(email.to_owned());
            self
        }

        #[inline]
        pub fn add_emails(mut self, emails: impl IntoIterator<Item = impl Into<String>>) -> Self {
            self.emails.extend(emails.into_iter().map(Into::into));
            self
        }
    }

    impl From<Vec<String>> for BatchAddSuppressionOptions {
        fn from(value: Vec<String>) -> Self {
            Self::new().add_emails(value)
        }
    }
    impl From<Vec<&str>> for BatchAddSuppressionOptions {
        fn from(value: Vec<&str>) -> Self {
            Self::new().add_emails(value.into_iter().map(ToOwned::to_owned))
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct BatchAddSuppressionResponse {
        pub data: Vec<AddSuppressionResponse>,
    }

    #[allow(clippy::redundant_pub_crate)]
    pub(crate) trait SpecifiedMarker {}
    impl SpecifiedMarker for EmailsSpecified {}
    impl SpecifiedMarker for IdsSpecified {}

    #[derive(Debug, Clone, Copy, Default)]
    pub struct NotSpecified {}

    #[derive(Debug, Clone, Copy)]
    pub struct EmailsSpecified {}

    #[derive(Debug, Clone, Copy)]
    pub struct IdsSpecified {}

    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct BatchRemoveSuppressionOptions<Marker = NotSpecified> {
        #[serde(skip)]
        marker: std::marker::PhantomData<Marker>,

        #[serde(skip_serializing_if = "Vec::is_empty")]
        emails: Vec<String>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        ids: Vec<String>,
    }

    impl BatchRemoveSuppressionOptions {
        #[inline]
        pub fn new() -> Self {
            Self {
                marker: std::marker::PhantomData::<NotSpecified>,
                emails: vec![],
                ids: vec![],
            }
        }
    }

    impl BatchRemoveSuppressionOptions<NotSpecified> {
        #[inline]
        pub fn add_emails(
            self,
            emails: impl IntoIterator<Item = impl Into<String>>,
        ) -> BatchRemoveSuppressionOptions<EmailsSpecified> {
            BatchRemoveSuppressionOptions::<EmailsSpecified> {
                marker: std::marker::PhantomData,
                emails: emails.into_iter().map(Into::into).collect(),
                ids: self.ids,
            }
        }

        #[inline]
        pub fn add_ids(
            self,
            ids: impl IntoIterator<Item = impl Into<String>>,
        ) -> BatchRemoveSuppressionOptions<IdsSpecified> {
            BatchRemoveSuppressionOptions::<IdsSpecified> {
                marker: std::marker::PhantomData,
                emails: self.emails,
                ids: ids.into_iter().map(Into::into).collect(),
            }
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct BatchRemoveSuppressionsResponse {
        pub data: Vec<RemoveSuppressionResponse>,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test {
    use crate::{
        list_opts::ListResponse,
        test::{CLIENT, DebugResult},
        types::{
            AddSuppressionResponse, BatchAddSuppressionResponse, BatchRemoveSuppressionsResponse,
            RemoveSuppressionResponse, Suppression,
        },
    };

    #[test]
    fn deserialize() {
        let add_suppression = r#"{
        "object": "suppression",
        "id": "e169aa45-1ecf-4183-9955-b1499d5701d3"
      }"#;
        let get_suppression = r#"{
        "object": "suppression",
        "id": "e169aa45-1ecf-4183-9955-b1499d5701d3",
        "email": "steve.wozniak@example.com",
        "origin": "bounce",
        "source_id": "4ef9a417-02e9-4d39-ad75-9611e0fcc33c",
        "created_at": "2026-10-06T23:47:56.678Z"
      }"#;
        let list_suppressions = r#"{
        "object": "list",
        "has_more": false,
        "data": [
          {
            "object": "suppression",
            "id": "e169aa45-1ecf-4183-9955-b1499d5701d3",
            "email": "steve.wozniak@example.com",
            "origin": "manual",
            "source_id": null,
            "created_at": "2026-10-06T23:47:56.678Z"
          },
          {
            "object": "suppression",
            "id": "520784e2-887d-4c25-b53c-4ad46ad38100",
            "email": "susan.kare@example.com",
            "origin": "bounce",
            "source_id": "4ef9a417-02e9-4d39-ad75-9611e0fcc33c",
            "created_at": "2026-10-07T08:12:03.412Z"
          }
        ]
      }"#;
        let remove_suppression = r#"{
        "object": "suppression",
        "id": "e169aa45-1ecf-4183-9955-b1499d5701d3",
        "deleted": true
      }"#;
        let add_suppressions = r#"{
        "data": [
          {
            "object": "suppression",
            "id": "e169aa45-1ecf-4183-9955-b1499d5701d3"
          },
          {
            "object": "suppression",
            "id": "520784e2-887d-4c25-b53c-4ad46ad38100"
          }
        ]
      }"#;
        let remove_suppressions = r#"{
        "data": [
          {
            "object": "suppression",
            "id": "e169aa45-1ecf-4183-9955-b1499d5701d3",
            "deleted": true
          }
        ]
      }"#;

        let res = serde_json::from_str::<AddSuppressionResponse>(add_suppression);
        assert!(res.is_ok());
        let res = serde_json::from_str::<Suppression>(get_suppression);
        assert!(res.is_ok());
        let res = serde_json::from_str::<ListResponse<Suppression>>(list_suppressions);
        assert!(res.is_ok());
        let res = serde_json::from_str::<RemoveSuppressionResponse>(remove_suppression);
        assert!(res.is_ok());
        let res = serde_json::from_str::<BatchAddSuppressionResponse>(add_suppressions);
        assert!(res.is_ok());
        let res = serde_json::from_str::<BatchRemoveSuppressionsResponse>(remove_suppressions);
        assert!(res.is_ok());
    }

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> DebugResult<()> {
        use crate::{
            list_opts::ListOptions,
            types::{
                AddSuppressionOptions, BatchAddSuppressionOptions, BatchRemoveSuppressionOptions,
            },
        };

        let resend = &*CLIENT;
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Add
        let opts = AddSuppressionOptions::new().with_email("steve.wozniak@example.com");
        let suppression = resend.suppressions.add(opts).await?;
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Get
        let suppression = resend.suppressions.get(&suppression.id).await?;

        // List
        let suppressions = resend.suppressions.list(ListOptions::default()).await?;
        assert_eq!(suppressions.len(), 1);

        // Remove
        let removed = resend.suppressions.remove(&suppression.id).await?;
        assert!(removed.deleted);

        std::thread::sleep(std::time::Duration::from_secs(2));

        let suppressions = resend.suppressions.list(ListOptions::default()).await?;
        assert_eq!(suppressions.len(), 0);

        // Batch add
        let opts = vec!["steve.wozniak@example.com", "susan.kare@example.com"];
        let batch_add = resend
            .suppressions
            .batch_add(BatchAddSuppressionOptions::from(opts))
            .await?;
        assert_eq!(batch_add.data.len(), 2);

        std::thread::sleep(std::time::Duration::from_secs(2));

        let suppressions = resend.suppressions.list(ListOptions::default()).await?;
        assert_eq!(suppressions.len(), 2);

        // Batch remove
        let opts = batch_add
            .data
            .into_iter()
            .map(|el| el.id.to_string())
            .collect::<Vec<_>>();
        let batch_remove = resend
            .suppressions
            .batch_remove(BatchRemoveSuppressionOptions::new().add_ids(opts))
            .await?;
        assert_eq!(batch_remove.data.len(), 2);

        std::thread::sleep(std::time::Duration::from_secs(2));

        let suppressions = resend.suppressions.list(ListOptions::default()).await?;
        assert_eq!(suppressions.len(), 0);

        Ok(())
    }
}
