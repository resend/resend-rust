use std::ops::Index;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct ListBefore {}

#[derive(Debug, Clone, Copy)]
pub struct ListAfter {}

#[derive(Debug, Clone, Copy)]
pub struct TimeNotSpecified {}

/// Query parameters for retrieving a list of things.
///
/// Note that [`ListOptions::default()`] applies no filters.
///
/// See <https://resend.com/docs/pagination> for more information.
///
/// ## Example
///
/// ```
/// # use resend_rs::list_opts::ListOptions;
/// let list_opts = ListOptions::default()
///   .with_limit(3)
///   .list_before("71f170f3-826e-47e3-9128-a5958e3b375e");
/// ```
#[must_use]
#[derive(Debug, Clone, Serialize)]
pub struct ListOptions<List = TimeNotSpecified> {
    #[serde(skip)]
    list: std::marker::PhantomData<List>,

    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u8>,

    #[serde(rename = "before")]
    before_id: Option<String>,

    #[serde(rename = "after")]
    after_id: Option<String>,
}

impl Default for ListOptions {
    /// Applies no filters (old default behavior).
    fn default() -> Self {
        Self {
            list: std::marker::PhantomData::<TimeNotSpecified>,
            limit: None,
            before_id: None,
            after_id: None,
        }
    }
}

impl<T> ListOptions<T> {
    /// Number of things to retrieve. If no limit is provided then the default limit will be used
    /// which varies from endpoint to endpoint, consult the specific method's documentation.
    ///
    /// - min: 1
    /// - max: 100
    #[inline]
    pub const fn with_limit(mut self, limit: u8) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl ListOptions<TimeNotSpecified> {
    /// The id before which we'll retrieve the items. This id will *not* be included in the list.
    #[inline]
    pub fn list_before(self, email_id: &str) -> ListOptions<ListBefore> {
        ListOptions::<ListBefore> {
            list: std::marker::PhantomData,
            limit: self.limit,
            before_id: Some(email_id.to_string()),
            after_id: None,
        }
    }

    /// The id after which we'll retrieve the items. This id will *not* be included in the list.
    #[inline]
    pub fn list_after(self, email_id: &str) -> ListOptions<ListAfter> {
        ListOptions::<ListAfter> {
            list: std::marker::PhantomData,
            limit: self.limit,
            before_id: None,
            after_id: Some(email_id.to_string()),
        }
    }
}

/// Paginated response.
///
/// See <https://resend.com/docs/pagination> for more information.
#[must_use]
#[derive(Debug, Clone, Deserialize)]
pub struct ListResponse<T> {
    pub has_more: bool,
    pub data: Vec<T>,
}

impl<T> Index<usize> for ListResponse<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        #[allow(clippy::indexing_slicing)]
        &self.data[index]
    }
}

impl<T> ListResponse<T> {
    /// Equivalent to `self.data.is_empty()`.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Equivalent to `self.data.len()`.
    #[inline]
    pub const fn len(&self) -> usize {
        self.data.len()
    }
}
