use std::fmt;
use std::sync::Arc;

use reqwest::Method;

use crate::{Config, Result, list_opts::ListOptions};
use crate::{list_opts::ListResponse, types::Segment};

use self::types::CreateSegmentResponse;

/// `Resend` APIs for `/segments` endpoints.
#[derive(Clone)]
pub struct SegmentsSvc(pub(crate) Arc<Config>);

impl SegmentsSvc {
    /// Create a new segment for contacts to be added to.
    ///
    /// Returns an `id` of a created segment.
    ///
    /// <https://resend.com/docs/api-reference/segments/create-segment>
    #[maybe_async::maybe_async]
    pub async fn create(&self, name: &str) -> Result<CreateSegmentResponse> {
        let segment = types::CreateSegmentRequest {
            name: name.to_owned(),
        };

        let request = self.0.build(Method::POST, "/segments");
        let response = self.0.send(request.json(&segment)).await?;
        let content = response.json::<CreateSegmentResponse>().await?;

        Ok(content)
    }

    /// Retrieve a single segment.
    ///
    /// <https://resend.com/docs/api-reference/segments/get-segment>
    #[maybe_async::maybe_async]
    pub async fn get(&self, id: &str) -> Result<Segment> {
        let path = format!("/segments/{id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Segment>().await?;

        Ok(content)
    }

    /// Remove an existing segment.
    ///
    /// <https://resend.com/docs/api-reference/segments/delete-segment>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn delete(&self, id: &str) -> Result<bool> {
        let path = format!("/segments/{id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<types::RemoveSegmentResponse>().await?;

        Ok(content.deleted)
    }

    /// Retrieve a list of segments.
    ///
    /// - Default limit: no limit (return everything)
    ///
    /// <https://resend.com/docs/api-reference/segments/list-segments>
    #[maybe_async::maybe_async]
    #[allow(clippy::needless_pass_by_value)]
    pub async fn list<T>(&self, list_opts: ListOptions<T>) -> Result<ListResponse<Segment>> {
        let request = self.0.build(Method::GET, "/segments").query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<Segment>>().await?;

        Ok(content)
    }
}

impl fmt::Debug for SegmentsSvc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

#[allow(unreachable_pub)]
pub mod types {
    use serde::{Deserialize, Serialize};

    crate::define_id_type!(SegmentId);

    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateSegmentRequest {
        /// The name of the segment you want to create.
        pub name: String,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct CreateSegmentResponse {
        /// The ID of the segment.
        pub id: SegmentId,
        /// The name of the segment.
        pub name: String,
    }

    /// Name and ID of an existing contact list.
    #[must_use]
    #[derive(Debug, Clone, Deserialize)]
    pub struct Segment {
        /// The ID of the segment.
        pub id: SegmentId,
        // /// The object of the segment.
        // pub object: String,
        /// The name of the segment.
        pub name: String,
        /// The date that the object was created in ISO8601 format.
        pub created_at: String,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct RemoveSegmentResponse {
        /// The ID of the segment.
        #[allow(dead_code)]
        pub id: SegmentId,
        /// The deleted attribute indicates that the corresponding segment has been deleted.
        pub deleted: bool,
    }
}

#[cfg(test)]
#[allow(clippy::needless_return)]
mod test {
    use crate::list_opts::ListOptions;
    use crate::test::{CLIENT, DebugResult};

    #[tokio_shared_rt::test(shared = true)]
    #[cfg(not(feature = "blocking"))]
    async fn all() -> DebugResult<()> {
        let resend = &*CLIENT;
        let segment = "test_segments";

        // Create.
        let created = resend.segments.create(segment).await?;
        let id = created.id;
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Get.
        let data = resend.segments.get(&id).await?;
        assert_eq!(data.name.as_str(), segment);

        // List.
        let segments = resend.segments.list(ListOptions::default()).await?;
        let segments_before = segments.len();
        assert!(segments_before > 1);

        // Delete.
        let deleted = resend.segments.delete(&id).await?;
        assert!(deleted);

        Ok(())
    }
}
