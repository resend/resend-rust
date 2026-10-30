use std::sync::Arc;

use reqwest::Method;

use crate::{
    Config, Result,
    idempotent::Idempotent,
    list_opts::{ListOptions, ListResponse},
    types::{
        CreateInboxDraftOptions, CreateInboxDraftResponse, CreateInboxLabelOptions,
        CreateInboxLabelResponse, CreateInboxOptions, CreateInboxResponse,
        DeleteInboxDraftResponse, DeleteInboxLabelResponse, DeleteInboxResponse,
        DeleteInboxThreadResponse, ForwardInboxThreadEmailOptions, ForwardInboxThreadEmailResponse,
        Inbox, InboxAgent, InboxDraft, InboxDraftListItem, InboxListItem, InboxThread,
        InboxThreadEmail, InboxThreadSearchResult, InboxThreadSummary, ListInboxLabelsResponse,
        ListInboxThreadLabelsResponse, ListInboxThreadsOptions, ReplyInboxThreadEmailOptions,
        ReplyInboxThreadEmailResponse, SearchInboxThreadsOptions, SendInboxDraftOptions,
        SendInboxDraftResponse, UpdateInboxAgentOptions, UpdateInboxAgentResponse,
        UpdateInboxDraftOptions, UpdateInboxDraftResponse, UpdateInboxLabelOptions,
        UpdateInboxLabelResponse, UpdateInboxOptions, UpdateInboxResponse,
        UpdateInboxThreadOptions,
    },
};

/// `Resend` APIs for `/inboxes` endpoints.
#[derive(Clone, Debug)]
pub struct InboxesSvc(pub(crate) Arc<Config>);

impl InboxesSvc {
    /// Creates an inbox that sends and receives email at an address on one of your domains.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/create-inbox>
    #[maybe_async::maybe_async]
    pub async fn create(&self, inbox: CreateInboxOptions) -> Result<CreateInboxResponse> {
        let request = self.0.build(Method::POST, "/inboxes");
        let response = self.0.send(request.json(&inbox)).await?;
        let content = response.json::<CreateInboxResponse>().await?;

        Ok(content)
    }

    /// Returns the inboxes of the team, newest first.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/list-inboxes>
    #[maybe_async::maybe_async]
    pub async fn list<T>(&self, list_opts: ListOptions<T>) -> Result<ListResponse<InboxListItem>> {
        let request = self.0.build(Method::GET, "/inboxes").query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<InboxListItem>>().await?;

        Ok(content)
    }

    /// Retrieve a single inbox by ID or email address.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/get-inbox>
    #[maybe_async::maybe_async]
    pub async fn get(&self, inbox_id_or_email: &str) -> Result<Inbox> {
        let path = format!("/inboxes/{}", urlencoding::encode(inbox_id_or_email));

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Inbox>().await?;

        Ok(content)
    }

    /// Updates the internal name of the inbox or the name recipients see when mail is sent from
    /// it.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/update-inbox>
    #[maybe_async::maybe_async]
    pub async fn update(
        &self,
        inbox_id_or_email: &str,
        update: UpdateInboxOptions,
    ) -> Result<UpdateInboxResponse> {
        let path = format!("/inboxes/{}", urlencoding::encode(inbox_id_or_email));

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&update)).await?;
        let content = response.json::<UpdateInboxResponse>().await?;

        Ok(content)
    }

    /// Removes the inbox.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/delete-inbox>
    #[maybe_async::maybe_async]
    pub async fn delete(&self, inbox_id_or_email: &str) -> Result<DeleteInboxResponse> {
        let path = format!("/inboxes/{}", urlencoding::encode(inbox_id_or_email));

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<DeleteInboxResponse>().await?;

        Ok(content)
    }

    /// Returns the agent settings of the inbox.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/get-agent>
    #[maybe_async::maybe_async]
    pub async fn get_agent(&self, inbox_id_or_email: &str) -> Result<InboxAgent> {
        let path = format!("/inboxes/{}/agent", urlencoding::encode(inbox_id_or_email));

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<InboxAgent>().await?;

        Ok(content)
    }

    /// Updates the agent settings of the inbox.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/update-agent>
    #[maybe_async::maybe_async]
    pub async fn update_agent(
        &self,
        inbox_id_or_email: &str,
        update: UpdateInboxAgentOptions,
    ) -> Result<UpdateInboxAgentResponse> {
        let path = format!("/inboxes/{}/agent", urlencoding::encode(inbox_id_or_email));

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&update)).await?;
        let content = response.json::<UpdateInboxAgentResponse>().await?;

        Ok(content)
    }

    /// Returns every label on the inbox, oldest first.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/list-labels>
    #[maybe_async::maybe_async]
    pub async fn list_labels(&self, inbox_id_or_email: &str) -> Result<ListInboxLabelsResponse> {
        let path = format!("/inboxes/{}/labels", urlencoding::encode(inbox_id_or_email));

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<ListInboxLabelsResponse>().await?;

        Ok(content)
    }

    /// Create a label on an inbox.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/create-label>
    #[maybe_async::maybe_async]
    pub async fn create_label(
        &self,
        inbox_id_or_email: &str,
        label: CreateInboxLabelOptions,
    ) -> Result<CreateInboxLabelResponse> {
        let path = format!("/inboxes/{}/labels", urlencoding::encode(inbox_id_or_email));

        let request = self.0.build(Method::POST, &path);
        let response = self.0.send(request.json(&label)).await?;
        let content = response.json::<CreateInboxLabelResponse>().await?;

        Ok(content)
    }

    /// Renames or recolors a label.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/update-label>
    #[maybe_async::maybe_async]
    pub async fn update_label(
        &self,
        inbox_id_or_email: &str,
        label_id: &str,
        update: UpdateInboxLabelOptions,
    ) -> Result<UpdateInboxLabelResponse> {
        let path = format!(
            "/inboxes/{}/labels/{label_id}",
            urlencoding::encode(inbox_id_or_email)
        );

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&update)).await?;
        let content = response.json::<UpdateInboxLabelResponse>().await?;

        Ok(content)
    }

    /// Deletes the label and removes it from every thread that carries it.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/delete-label>
    #[maybe_async::maybe_async]
    pub async fn delete_label(
        &self,
        inbox_id_or_email: &str,
        label_id: &str,
    ) -> Result<DeleteInboxLabelResponse> {
        let path = format!(
            "/inboxes/{}/labels/{label_id}",
            urlencoding::encode(inbox_id_or_email)
        );

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<DeleteInboxLabelResponse>().await?;

        Ok(content)
    }

    /// Returns the threads in an inbox, most recently active first.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/list-threads>
    #[maybe_async::maybe_async]
    pub async fn list_threads<T>(
        &self,
        inbox_id_or_email: &str,
        filters: ListInboxThreadsOptions,
        list_opts: ListOptions<T>,
    ) -> Result<ListResponse<InboxThread>> {
        let path = format!(
            "/inboxes/{}/threads",
            urlencoding::encode(inbox_id_or_email)
        );

        let request = self
            .0
            .build(Method::GET, &path)
            .query(&list_opts)
            .query(&filters);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<InboxThread>>().await?;

        Ok(content)
    }

    /// Finds threads by the text of their emails, who sent or received them, whether they have
    /// attachments, and their dates.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/search-threads>
    #[maybe_async::maybe_async]
    pub async fn search_threads<T>(
        &self,
        inbox_id_or_email: &str,
        search: SearchInboxThreadsOptions,
        list_opts: ListOptions<T>,
    ) -> Result<ListResponse<InboxThreadSearchResult>> {
        let path = format!(
            "/inboxes/{}/threads/search",
            urlencoding::encode(inbox_id_or_email)
        );

        let request = self
            .0
            .build(Method::GET, &path)
            .query(&list_opts)
            .query(&search);
        let response = self.0.send(request).await?;
        let content = response
            .json::<ListResponse<InboxThreadSearchResult>>()
            .await?;

        Ok(content)
    }

    /// Returns the thread summary.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/get-thread>
    #[maybe_async::maybe_async]
    pub async fn get_thread(
        &self,
        inbox_id_or_email: &str,
        thread_id: &str,
    ) -> Result<InboxThreadSummary> {
        let path = format!(
            "/inboxes/{}/threads/{thread_id}",
            urlencoding::encode(inbox_id_or_email)
        );

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<InboxThreadSummary>().await?;

        Ok(content)
    }

    /// Marks a thread read or unread, or moves it to a folder.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/update-thread>
    #[maybe_async::maybe_async]
    pub async fn update_thread(
        &self,
        inbox_id_or_email: &str,
        thread_id: &str,
        update: UpdateInboxThreadOptions,
    ) -> Result<InboxThreadSummary> {
        let path = format!(
            "/inboxes/{}/threads/{thread_id}",
            urlencoding::encode(inbox_id_or_email)
        );

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&update)).await?;
        let content = response.json::<InboxThreadSummary>().await?;

        Ok(content)
    }

    /// Moves the thread to `trash` and schedules it for deletion in 30 days.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/delete-thread>
    #[maybe_async::maybe_async]
    pub async fn delete_thread(
        &self,
        inbox_id_or_email: &str,
        thread_id: &str,
    ) -> Result<DeleteInboxThreadResponse> {
        let path = format!(
            "/inboxes/{}/threads/{thread_id}",
            urlencoding::encode(inbox_id_or_email)
        );

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<DeleteInboxThreadResponse>().await?;

        Ok(content)
    }

    /// Applies one or more labels to the thread. Labels already on the thread stay on it.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/add-thread-labels>
    #[maybe_async::maybe_async]
    pub async fn add_thread_labels(
        &self,
        inbox_id_or_email: &str,
        thread_id: &str,
        labels: &[&str],
    ) -> Result<ListInboxThreadLabelsResponse> {
        let path = format!(
            "/inboxes/{}/threads/{thread_id}/labels",
            urlencoding::encode(inbox_id_or_email)
        );

        let request = self.0.build(Method::POST, &path);
        let response = self
            .0
            .send(request.json(&InboxThreadLabelsBody { labels }))
            .await?;
        let content = response.json::<ListInboxThreadLabelsResponse>().await?;

        Ok(content)
    }

    /// Replaces every label on the thread with the given labels, in one atomic change.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/set-thread-labels>
    #[maybe_async::maybe_async]
    pub async fn set_thread_labels(
        &self,
        inbox_id_or_email: &str,
        thread_id: &str,
        labels: &[&str],
    ) -> Result<ListInboxThreadLabelsResponse> {
        let path = format!(
            "/inboxes/{}/threads/{thread_id}/labels",
            urlencoding::encode(inbox_id_or_email)
        );

        let request = self.0.build(Method::PUT, &path);
        let response = self
            .0
            .send(request.json(&InboxThreadLabelsBody { labels }))
            .await?;
        let content = response.json::<ListInboxThreadLabelsResponse>().await?;

        Ok(content)
    }

    /// Takes one label off the thread.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/remove-thread-label>
    #[maybe_async::maybe_async]
    pub async fn remove_thread_label(
        &self,
        inbox_id_or_email: &str,
        thread_id: &str,
        label_id: &str,
    ) -> Result<ListInboxThreadLabelsResponse> {
        let path = format!(
            "/inboxes/{}/threads/{thread_id}/labels/{label_id}",
            urlencoding::encode(inbox_id_or_email)
        );

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<ListInboxThreadLabelsResponse>().await?;

        Ok(content)
    }

    /// Returns the messages of a thread, oldest first.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/list-thread-emails>
    #[maybe_async::maybe_async]
    pub async fn list_thread_emails<T>(
        &self,
        inbox_id_or_email: &str,
        thread_id: &str,
        list_opts: ListOptions<T>,
    ) -> Result<ListResponse<InboxThreadEmail>> {
        let path = format!(
            "/inboxes/{}/threads/{thread_id}/emails",
            urlencoding::encode(inbox_id_or_email)
        );

        let request = self.0.build(Method::GET, &path).query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<InboxThreadEmail>>().await?;

        Ok(content)
    }

    /// Retrieve a single thread email.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/get-thread-email>
    #[maybe_async::maybe_async]
    pub async fn get_thread_email(
        &self,
        inbox_id_or_email: &str,
        thread_id: &str,
        email_id: &str,
    ) -> Result<InboxThreadEmail> {
        let path = format!(
            "/inboxes/{}/threads/{thread_id}/emails/{email_id}",
            urlencoding::encode(inbox_id_or_email)
        );

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<InboxThreadEmail>().await?;

        Ok(content)
    }

    /// Sends a reply from the inbox address right away, with no stored draft.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/reply-thread-email>
    #[maybe_async::maybe_async]
    pub async fn reply_thread_email(
        &self,
        inbox_id_or_email: &str,
        thread_id: &str,
        email_id: &str,
        reply: impl Into<Idempotent<ReplyInboxThreadEmailOptions>>,
    ) -> Result<ReplyInboxThreadEmailResponse> {
        let path = format!(
            "/inboxes/{}/threads/{thread_id}/emails/{email_id}/reply",
            urlencoding::encode(inbox_id_or_email)
        );
        let reply: Idempotent<ReplyInboxThreadEmailOptions> = reply.into();

        let mut request = self.0.build(Method::POST, &path);

        if let Some(ref idempotency_key) = reply.idempotency_key {
            request = request.header("Idempotency-Key", idempotency_key);
        }

        let response = self.0.send(request.json(&reply)).await?;
        let content = response.json::<ReplyInboxThreadEmailResponse>().await?;

        Ok(content)
    }

    /// Forwards a message from the inbox address right away, with no stored draft.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/forward-thread-email>
    #[maybe_async::maybe_async]
    pub async fn forward_thread_email(
        &self,
        inbox_id_or_email: &str,
        thread_id: &str,
        email_id: &str,
        forward: impl Into<Idempotent<ForwardInboxThreadEmailOptions>>,
    ) -> Result<ForwardInboxThreadEmailResponse> {
        let path = format!(
            "/inboxes/{}/threads/{thread_id}/emails/{email_id}/forward",
            urlencoding::encode(inbox_id_or_email)
        );
        let forward: Idempotent<ForwardInboxThreadEmailOptions> = forward.into();

        let mut request = self.0.build(Method::POST, &path);

        if let Some(ref idempotency_key) = forward.idempotency_key {
            request = request.header("Idempotency-Key", idempotency_key);
        }

        let response = self.0.send(request.json(&forward)).await?;
        let content = response.json::<ForwardInboxThreadEmailResponse>().await?;

        Ok(content)
    }

    /// Returns the unsent drafts on the inbox, most recently updated first.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/list-drafts>
    #[maybe_async::maybe_async]
    pub async fn list_drafts<T>(
        &self,
        inbox_id_or_email: &str,
        list_opts: ListOptions<T>,
    ) -> Result<ListResponse<InboxDraftListItem>> {
        let path = format!("/inboxes/{}/drafts", urlencoding::encode(inbox_id_or_email));

        let request = self.0.build(Method::GET, &path).query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<InboxDraftListItem>>().await?;

        Ok(content)
    }

    /// Creates an unsent draft on the inbox.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/create-draft>
    #[maybe_async::maybe_async]
    pub async fn create_draft(
        &self,
        inbox_id_or_email: &str,
        draft: impl Into<Idempotent<CreateInboxDraftOptions>>,
    ) -> Result<CreateInboxDraftResponse> {
        let path = format!("/inboxes/{}/drafts", urlencoding::encode(inbox_id_or_email));
        let draft: Idempotent<CreateInboxDraftOptions> = draft.into();

        let mut request = self.0.build(Method::POST, &path);

        if let Some(ref idempotency_key) = draft.idempotency_key {
            request = request.header("Idempotency-Key", idempotency_key);
        }

        let response = self.0.send(request.json(&draft)).await?;
        let content = response.json::<CreateInboxDraftResponse>().await?;

        Ok(content)
    }

    /// Retrieve a single draft.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/get-draft>
    #[maybe_async::maybe_async]
    pub async fn get_draft(&self, inbox_id_or_email: &str, draft_id: &str) -> Result<InboxDraft> {
        let path = format!(
            "/inboxes/{}/drafts/{draft_id}",
            urlencoding::encode(inbox_id_or_email)
        );

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<InboxDraft>().await?;

        Ok(content)
    }

    /// Updates the content of an unsent draft. Omitted fields are left unchanged.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/update-draft>
    #[maybe_async::maybe_async]
    pub async fn update_draft(
        &self,
        inbox_id_or_email: &str,
        draft_id: &str,
        update: UpdateInboxDraftOptions,
    ) -> Result<UpdateInboxDraftResponse> {
        let path = format!(
            "/inboxes/{}/drafts/{draft_id}",
            urlencoding::encode(inbox_id_or_email)
        );

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&update)).await?;
        let content = response.json::<UpdateInboxDraftResponse>().await?;

        Ok(content)
    }

    /// Discards an unsent draft.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/delete-draft>
    #[maybe_async::maybe_async]
    pub async fn delete_draft(
        &self,
        inbox_id_or_email: &str,
        draft_id: &str,
    ) -> Result<DeleteInboxDraftResponse> {
        let path = format!(
            "/inboxes/{}/drafts/{draft_id}",
            urlencoding::encode(inbox_id_or_email)
        );

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<DeleteInboxDraftResponse>().await?;

        Ok(content)
    }

    /// Sends the draft from the inbox address.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/send-draft>
    #[maybe_async::maybe_async]
    pub async fn send_draft(
        &self,
        inbox_id_or_email: &str,
        draft_id: &str,
        opts: impl Into<Idempotent<SendInboxDraftOptions>>,
    ) -> Result<SendInboxDraftResponse> {
        let path = format!(
            "/inboxes/{}/drafts/{draft_id}/send",
            urlencoding::encode(inbox_id_or_email)
        );
        let opts: Idempotent<SendInboxDraftOptions> = opts.into();

        let mut request = self.0.build(Method::POST, &path);

        if let Some(ref idempotency_key) = opts.idempotency_key {
            request = request.header("Idempotency-Key", idempotency_key);
        }

        if opts.data.headers.is_some() {
            request = request.json(&opts);
        }

        let response = self.0.send(request).await?;
        let content = response.json::<SendInboxDraftResponse>().await?;

        Ok(content)
    }
}

#[derive(serde::Serialize)]
struct InboxThreadLabelsBody<'a> {
    labels: &'a [&'a str],
}

#[allow(unreachable_pub)]
pub mod types {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    use crate::idempotent::Idempotent;
    use crate::types::{ContentOrPath, CreateAttachment, DomainId, EmailId};

    crate::define_id_type!(InboxId);
    crate::define_id_type!(InboxAgentId);
    crate::define_id_type!(InboxLabelId);
    crate::define_id_type!(InboxThreadId);
    crate::define_id_type!(InboxThreadEmailId);
    crate::define_id_type!(InboxDraftId);

    /// A field that is either set to a value or cleared with `null`.
    #[derive(Debug, Clone, Serialize)]
    #[serde(untagged)]
    enum Nullable<T> {
        Value(T),
        Null,
    }

    fn join<I, S>(values: I) -> Option<String>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = values
            .into_iter()
            .map(|value| value.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");

        (!joined.is_empty()).then_some(joined)
    }

    /// A single inbox.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Inbox {
        /// The ID of the inbox.
        pub id: InboxId,
        /// Internal name for the inbox. Recipients do not see it. Falls back to the inbox email
        /// address.
        pub name: String,
        /// The address of the inbox.
        pub email_address: String,
        /// The ID of the domain that owns the address.
        pub domain_id: DomainId,
        /// The address to forward mail to when forwarding is enabled. `None` otherwise.
        pub receiving_address: Option<String>,
        /// The name recipients see when mail is sent from this inbox. `None` when not set.
        pub from_name: Option<String>,
        /// The number of unread threads in the Inbox folder.
        pub unread: u64,
        /// The number of unsent drafts.
        pub drafts: u64,
        /// ISO 8601 timestamp of the newest received email in the Inbox folder. `None` when the
        /// folder has no received email.
        pub last_received: Option<String>,
        /// ISO 8601 timestamp when the inbox was created.
        pub created_at: String,
        /// ISO 8601 timestamp when the inbox was last updated.
        pub updated_at: String,
    }

    /// An inbox as it appears in [`InboxesSvc::list`](crate::services::InboxesSvc::list).
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InboxListItem {
        /// The ID of the inbox.
        pub id: InboxId,
        /// Internal name for the inbox. Recipients do not see it.
        pub name: Option<String>,
        /// The address of the inbox.
        pub email_address: String,
        /// The name recipients see when mail is sent from this inbox. `None` when not set.
        pub from_name: Option<String>,
        /// The number of unread threads in the Inbox folder.
        pub unread: u64,
        /// ISO 8601 timestamp of the newest received email in the Inbox folder. `None` when the
        /// folder has no received email.
        pub last_received: Option<String>,
        /// ISO 8601 timestamp when the inbox was created.
        pub created_at: String,
        /// ISO 8601 timestamp when the inbox was last updated.
        pub updated_at: String,
    }

    /// See [relevant docs].
    ///
    /// [relevant docs]: <https://resend.com/docs/api-reference/inboxes/create-inbox#body-parameters>
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateInboxOptions {
        email_address: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        from_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        forwarding: Option<bool>,
    }

    impl CreateInboxOptions {
        /// Creates a new [`CreateInboxOptions`].
        pub fn new(email_address: impl Into<String>) -> Self {
            Self {
                email_address: email_address.into(),
                name: None,
                from_name: None,
                forwarding: None,
            }
        }

        /// Internal name for the inbox. Recipients do not see it. Max 64 characters. Defaults to
        /// `email_address`.
        #[inline]
        pub fn with_name(mut self, name: impl Into<String>) -> Self {
            self.name = Some(name.into());
            self
        }

        /// The name recipients see when mail is sent from this inbox. A plain name, not a
        /// `Name <email>` address. Max 78 characters.
        #[inline]
        pub fn with_from_name(mut self, from_name: impl Into<String>) -> Self {
            self.from_name = Some(from_name.into());
            self
        }

        /// When `true`, Resend provisions a receiving address so you can receive mail without
        /// adding an MX record. The address is returned in `receiving_address`. Defaults to
        /// `false`.
        #[inline]
        pub const fn with_forwarding(mut self, forwarding: bool) -> Self {
            self.forwarding = Some(forwarding);
            self
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreateInboxResponse {
        /// The ID of the inbox.
        pub id: InboxId,
    }

    /// List of changes to apply to an [`Inbox`].
    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct UpdateInboxOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        from_name: Option<String>,
    }

    impl UpdateInboxOptions {
        pub fn new() -> Self {
            Self::default()
        }

        /// New internal name for the inbox. Recipients do not see it. Between 1 and 64
        /// characters after trimming.
        #[inline]
        pub fn with_name(mut self, name: impl Into<String>) -> Self {
            self.name = Some(name.into());
            self
        }

        /// The name recipients see when mail is sent from this inbox. A plain name, not a
        /// `Name <email>` address. Max 78 characters.
        #[inline]
        pub fn with_from_name(mut self, from_name: impl Into<String>) -> Self {
            self.from_name = Some(from_name.into());
            self
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateInboxResponse {
        /// The ID of the inbox.
        pub id: InboxId,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DeleteInboxResponse {
        /// The ID of the inbox.
        pub id: InboxId,
        /// Indicates whether the inbox was successfully deleted.
        pub deleted: bool,
    }

    /// An action the inbox agent can take.
    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum InboxAgentAction {
        DraftReply,
        ForwardThread,
        AddLabels,
        AssignThread,
        MarkAsSpam,
        Archive,
        MoveToTrash,
    }

    /// A tone preset the inbox agent writes in.
    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum InboxAgentTone {
        ConciseProfessional,
        ConciseCasual,
        ExpandedProfessional,
        ExpandedCasual,
    }

    /// The agent settings of an inbox.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InboxAgent {
        /// Instructions the agent follows when it handles the threads of the inbox.
        pub instructions: Option<String>,
        /// The tone the agent writes in.
        pub tone: Option<InboxAgentTone>,
        /// The actions the agent can take.
        pub actions: Vec<InboxAgentAction>,
    }

    /// List of changes to apply to an [`InboxAgent`].
    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct UpdateInboxAgentOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        instructions: Option<Nullable<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tone: Option<Nullable<InboxAgentTone>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        actions: Option<Vec<InboxAgentAction>>,
    }

    impl UpdateInboxAgentOptions {
        pub fn new() -> Self {
            Self::default()
        }

        /// Instructions the agent follows when it handles the threads of the inbox. Max 4000
        /// characters. Leading and trailing whitespace is trimmed, and the trimmed value must not
        /// be empty.
        #[inline]
        pub fn with_instructions(mut self, instructions: impl Into<String>) -> Self {
            self.instructions = Some(Nullable::Value(instructions.into()));
            self
        }

        /// Sends `null` to clear the instructions.
        #[inline]
        pub fn clear_instructions(mut self) -> Self {
            self.instructions = Some(Nullable::Null);
            self
        }

        /// The tone the agent writes in.
        #[inline]
        pub fn with_tone(mut self, tone: InboxAgentTone) -> Self {
            self.tone = Some(Nullable::Value(tone));
            self
        }

        /// Sends `null` to clear the tone.
        #[inline]
        pub fn clear_tone(mut self) -> Self {
            self.tone = Some(Nullable::Null);
            self
        }

        /// The actions the agent can take. Replaces the whole set. Duplicates are removed. Send an
        /// empty list to disable all actions.
        #[inline]
        pub fn with_actions(mut self, actions: impl IntoIterator<Item = InboxAgentAction>) -> Self {
            self.actions = Some(actions.into_iter().collect());
            self
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateInboxAgentResponse {
        /// The ID of the inbox agent.
        pub id: InboxAgentId,
    }

    /// A named, colored tag that belongs to one inbox.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InboxLabel {
        /// The ID of the label.
        pub id: InboxLabelId,
        /// The name of the label.
        pub name: String,
        /// The color of the label, as a `#RRGGBB` hex color in uppercase.
        pub color: String,
        /// Timestamp when the label was created.
        pub created_at: String,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ListInboxLabelsResponse {
        /// The labels on the inbox, oldest first.
        pub data: Vec<InboxLabel>,
    }

    /// See [relevant docs].
    ///
    /// [relevant docs]: <https://resend.com/docs/api-reference/inboxes/create-label#body-parameters>
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateInboxLabelOptions {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        color: Option<String>,
    }

    impl CreateInboxLabelOptions {
        /// Creates a new [`CreateInboxLabelOptions`].
        ///
        /// - `name`: The name of the label. Max 64 characters. Leading and trailing whitespace is
        ///   trimmed. Must not match another label on the inbox, compared without case.
        pub fn new(name: impl Into<String>) -> Self {
            Self {
                name: name.into(),
                color: None,
            }
        }

        /// The color of the label, as a `#RRGGBB` hex color. It is stored in uppercase. When
        /// omitted, a random color from the dashboard palette is used.
        #[inline]
        pub fn with_color(mut self, color: impl Into<String>) -> Self {
            self.color = Some(color.into());
            self
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreateInboxLabelResponse {
        /// The ID of the label.
        pub id: InboxLabelId,
    }

    /// List of changes to apply to an [`InboxLabel`]. Omitted fields keep their current value.
    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct UpdateInboxLabelOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        color: Option<String>,
    }

    impl UpdateInboxLabelOptions {
        pub fn new() -> Self {
            Self::default()
        }

        /// A new name for the label. Max 64 characters. Leading and trailing whitespace is
        /// trimmed. Must not match another label on the inbox, compared without case.
        #[inline]
        pub fn with_name(mut self, name: impl Into<String>) -> Self {
            self.name = Some(name.into());
            self
        }

        /// A new color for the label, as a `#RRGGBB` hex color. It is stored in uppercase.
        #[inline]
        pub fn with_color(mut self, color: impl Into<String>) -> Self {
            self.color = Some(color.into());
            self
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateInboxLabelResponse {
        pub id: InboxLabelId,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DeleteInboxLabelResponse {
        pub id: InboxLabelId,
        pub deleted: bool,
    }

    /// The folder a thread is in. Every thread is in exactly one folder.
    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum InboxFolder {
        Inbox,
        Archive,
        Spam,
        Sent,
        Trash,
    }

    impl InboxFolder {
        /// The folder as the API names it.
        #[must_use]
        pub const fn as_str(self) -> &'static str {
            match self {
                Self::Inbox => "inbox",
                Self::Archive => "archive",
                Self::Spam => "spam",
                Self::Sent => "sent",
                Self::Trash => "trash",
            }
        }
    }

    impl AsRef<str> for InboxFolder {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }

    /// A label as it appears on a thread.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InboxThreadLabel {
        pub id: InboxLabelId,
        pub name: String,
        pub color: String,
    }

    /// Every label on a thread after a thread label change.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ListInboxThreadLabelsResponse {
        pub data: Vec<InboxThreadLabel>,
    }

    /// A thread as it appears in the list of an inbox's threads.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InboxThread {
        /// The ID of the thread.
        pub id: InboxThreadId,
        /// The subject of the thread.
        pub subject: Option<String>,
        /// The sender email address. Can include the display name.
        pub from: Option<String>,
        /// The recipients of the most recent email. For a received email, these are the
        /// addresses it was delivered to, not the full `To` header.
        pub to: Vec<String>,
        /// The CC recipients of the most recent email.
        pub cc: Vec<String>,
        /// The BCC recipients of the most recent email. Always empty for a received email.
        pub bcc: Vec<String>,
        /// The labels attached to the thread.
        pub labels: Vec<InboxThreadLabel>,
        /// The number of emails in the thread.
        pub email_count: u64,
        /// Whether the most recent email in the thread has an attachment.
        pub has_attachment: bool,
        /// Whether the thread has an unsent draft.
        pub has_draft: bool,
        /// True only when every email in the thread is read.
        pub read: bool,
        /// Timestamp when the thread was last active.
        pub received_at: String,
        /// The folder the thread is in.
        pub folder: InboxFolder,
    }

    /// A thread returned by [`InboxesSvc::search_threads`](crate::services::InboxesSvc::search_threads).
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InboxThreadSearchResult {
        #[serde(flatten)]
        pub thread: InboxThread,
        /// The ID of the email in the thread that matched the search.
        pub matched_email_id: Option<InboxThreadEmailId>,
    }

    /// The summary of a single thread.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InboxThreadSummary {
        /// The ID of the thread.
        pub id: InboxThreadId,
        /// The subject of the thread.
        pub subject: Option<String>,
        /// The folder the thread is in.
        pub folder: InboxFolder,
        /// The labels attached to the thread.
        pub labels: Vec<InboxThreadLabel>,
        /// True only when every email in the thread is read.
        pub read: bool,
    }

    /// Filters for [`InboxesSvc::list_threads`](crate::services::InboxesSvc::list_threads).
    ///
    /// [`ListInboxThreadsOptions::default()`] applies no filters.
    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct ListInboxThreadsOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        folders: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        labels: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        read: Option<bool>,
    }

    impl ListInboxThreadsOptions {
        pub fn new() -> Self {
            Self::default()
        }

        /// The folders to list.
        #[inline]
        pub fn with_folders(mut self, folders: impl IntoIterator<Item = InboxFolder>) -> Self {
            self.folders = join(folders);
            self
        }

        /// Label IDs.
        #[inline]
        pub fn with_labels<S: AsRef<str>>(mut self, labels: impl IntoIterator<Item = S>) -> Self {
            self.labels = join(labels);
            self
        }

        /// `true` returns threads where every email is read. `false` returns threads with at
        /// least one unread email.
        #[inline]
        pub const fn with_read(mut self, read: bool) -> Self {
            self.read = Some(read);
            self
        }
    }

    /// Parameters for [`InboxesSvc::search_threads`](crate::services::InboxesSvc::search_threads).
    ///
    /// [`SearchInboxThreadsOptions::default()`] applies no filters.
    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct SearchInboxThreadsOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        query: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        from: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        to: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        cc: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        bcc: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        has_attachment: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        start_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        end_date: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        folders: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        labels: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        read: Option<bool>,
    }

    impl SearchInboxThreadsOptions {
        pub fn new() -> Self {
            Self::default()
        }

        /// Free text to search for in the subject, body, sender, recipients, and attachment
        /// names. Every word must match. Wrap words in double quotes to match an exact phrase.
        /// Start a word or quoted phrase with `-` to exclude it. Max 256 characters and 32 words.
        #[inline]
        pub fn with_query(mut self, query: impl Into<String>) -> Self {
            self.query = Some(query.into());
            self
        }

        /// Senders. Each value matches part of the sender's address or name, without case.
        /// Returns threads that match any of them. Up to 20 values.
        #[inline]
        pub fn with_from<S: AsRef<str>>(mut self, from: impl IntoIterator<Item = S>) -> Self {
            self.from = join(from);
            self
        }

        /// Recipients, matched the same way as `from`. Up to 20 values.
        #[inline]
        pub fn with_to<S: AsRef<str>>(mut self, to: impl IntoIterator<Item = S>) -> Self {
            self.to = join(to);
            self
        }

        /// CC recipients, matched the same way as `from`. Up to 20 values.
        #[inline]
        pub fn with_cc<S: AsRef<str>>(mut self, cc: impl IntoIterator<Item = S>) -> Self {
            self.cc = join(cc);
            self
        }

        /// BCC recipients, matched the same way as `from`. Up to 20 values.
        #[inline]
        pub fn with_bcc<S: AsRef<str>>(mut self, bcc: impl IntoIterator<Item = S>) -> Self {
            self.bcc = join(bcc);
            self
        }

        /// `true` matches emails with an attachment. `false` matches emails without one.
        #[inline]
        pub const fn with_has_attachment(mut self, has_attachment: bool) -> Self {
            self.has_attachment = Some(has_attachment);
            self
        }

        /// Matches emails sent on or after this date. Accepts a date like `2026-09-01` or an
        /// ISO 8601 timestamp, in UTC unless the timestamp has an offset.
        #[inline]
        pub fn with_start_date(mut self, start_date: impl Into<String>) -> Self {
            self.start_date = Some(start_date.into());
            self
        }

        /// Matches emails sent on or before this date. Same formats as `start_date`. A date
        /// covers that whole day.
        #[inline]
        pub fn with_end_date(mut self, end_date: impl Into<String>) -> Self {
            self.end_date = Some(end_date.into());
            self
        }

        /// The folders to search. A thread is returned when any of its emails is in one of the
        /// folders. Defaults to `inbox`, or to `inbox,archive,sent` when `labels` is set. `spam`
        /// and `trash` are only included when you name them.
        #[inline]
        pub fn with_folders(mut self, folders: impl IntoIterator<Item = InboxFolder>) -> Self {
            self.folders = join(folders);
            self
        }

        /// Label IDs. Returns threads with any of these labels. Up to 50 IDs. An ID that does not
        /// exist in the inbox returns a 404.
        #[inline]
        pub fn with_labels<S: AsRef<str>>(mut self, labels: impl IntoIterator<Item = S>) -> Self {
            self.labels = join(labels);
            self
        }

        /// `true` returns threads where every email is read. `false` returns threads with at
        /// least one unread email.
        #[inline]
        pub const fn with_read(mut self, read: bool) -> Self {
            self.read = Some(read);
            self
        }
    }

    /// The folder to move a thread to. `sent` is assigned automatically and cannot be set.
    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum UpdateInboxThreadFolder {
        Inbox,
        Archive,
        Spam,
        /// Moving a thread to `trash` schedules it for deletion in 30 days.
        Trash,
    }

    /// List of changes to apply to a thread.
    #[must_use]
    #[derive(Debug, Default, Clone, Copy, Serialize)]
    pub struct UpdateInboxThreadOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        read: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        folder: Option<UpdateInboxThreadFolder>,
    }

    impl UpdateInboxThreadOptions {
        pub fn new() -> Self {
            Self::default()
        }

        /// Marks every email in the thread read or unread.
        #[inline]
        pub const fn with_read(mut self, read: bool) -> Self {
            self.read = Some(read);
            self
        }

        /// Moves the thread to this folder. `sent` is assigned automatically and cannot be set.
        #[inline]
        pub const fn with_folder(mut self, folder: UpdateInboxThreadFolder) -> Self {
            self.folder = Some(folder);
            self
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DeleteInboxThreadResponse {
        /// The ID of the thread.
        pub id: InboxThreadId,
        /// True when this call moved the thread to `trash`. False when the thread was already
        /// there.
        pub deleted: bool,
    }

    /// Whether the inbox received a message or sent it.
    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum InboxEmailDirection {
        Inbound,
        Outbound,
    }

    /// An attachment of a thread email.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InboxThreadEmailAttachment {
        pub id: String,
        pub filename: Option<String>,
        pub size: Option<u64>,
    }

    /// A message of a thread.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InboxThreadEmail {
        /// The ID of the message.
        pub id: InboxThreadEmailId,
        /// Whether the inbox received the message or sent it.
        pub direction: InboxEmailDirection,
        /// Sender email address. Includes the display name when the sender has one, in the
        /// format `Name <sender@domain.com>`.
        pub from: String,
        /// The recipients of the message. For a received message, these are the addresses it was
        /// delivered to, not the full `To` header.
        pub to: Vec<String>,
        /// The CC recipients of the message.
        pub cc: Vec<String>,
        /// The BCC recipients of the message. Always empty for a received message.
        pub bcc: Vec<String>,
        /// The Reply-To addresses of the message.
        pub reply_to: Vec<String>,
        /// The subject of the message.
        pub subject: Option<String>,
        /// The Message-ID header of the message.
        pub message_id: Option<String>,
        /// The HTML body.
        pub html: Option<String>,
        /// The plain-text body.
        pub text: Option<String>,
        /// The attachments of the message.
        pub attachments: Vec<InboxThreadEmailAttachment>,
        /// Whether the message has been read.
        pub read: bool,
        /// Timestamp when the inbox received the message, or when the inbox sent it.
        pub received_at: String,
    }

    /// See [relevant docs].
    ///
    /// [relevant docs]: <https://resend.com/docs/api-reference/inboxes/reply-thread-email#body-parameters>
    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct ReplyInboxThreadEmailOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        cc: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        bcc: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        html: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        subject: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_all: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        attachments: Option<Vec<CreateAttachment>>,
        /// Custom headers to add to the email.
        #[serde(skip_serializing_if = "Option::is_none")]
        headers: Option<HashMap<String, String>>,
    }

    impl ReplyInboxThreadEmailOptions {
        pub fn new() -> Self {
            Self::default()
        }

        /// CC recipient email addresses. Without `reply_all`, the reply copies only these
        /// addresses. With `reply_all`, these addresses are merged with the copied recipients,
        /// without duplicates. `cc` and `bcc` combined must be fewer than 50 addresses. After the
        /// `to` recipients are added, the reply cannot exceed 50 recipients.
        #[inline]
        pub fn with_cc<S: Into<String>>(mut self, cc: impl IntoIterator<Item = S>) -> Self {
            self.cc = Some(cc.into_iter().map(Into::into).collect());
            self
        }

        /// BCC recipient email addresses. BCC recipients of the original message are never
        /// copied.
        #[inline]
        pub fn with_bcc<S: Into<String>>(mut self, bcc: impl IntoIterator<Item = S>) -> Self {
            self.bcc = Some(bcc.into_iter().map(Into::into).collect());
            self
        }

        /// The HTML body of the reply.
        #[inline]
        pub fn with_html(mut self, html: impl Into<String>) -> Self {
            self.html = Some(html.into());
            self
        }

        /// The plain-text body of the reply.
        #[inline]
        pub fn with_text(mut self, text: impl Into<String>) -> Self {
            self.text = Some(text.into());
            self
        }

        /// The subject of the reply. Cannot contain a line break. When omitted, the thread
        /// subject is used, prefixed with `Re:` if it does not already start with it.
        #[inline]
        pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
            self.subject = Some(subject.into());
            self
        }

        /// Reply to everyone on the original message. For a message the inbox received, the
        /// reply goes to its `Reply-To` addresses, or to its sender when it has none, and copies
        /// the other addresses of its `To` header and its `cc` recipients in `cc`. For a message
        /// the inbox sent, the reply goes to that message's `Reply-To` addresses when they are not
        /// the inbox address, otherwise to its `to` recipients, and copies its `cc` recipients.
        /// The inbox address is always excluded, and duplicates are removed. Defaults to `false`.
        #[inline]
        pub const fn with_reply_all(mut self, reply_all: bool) -> Self {
            self.reply_all = Some(reply_all);
            self
        }

        /// Adds an email header. The `In-Reply-To`, `References` and `Reply-To` headers are set by
        /// Resend and cannot be set here. Sending one of them, in any letter case, returns a 422.
        /// A header name cannot be empty, and names and values cannot contain line breaks or null
        /// characters, or the request returns a 422.
        #[inline]
        pub fn with_header(mut self, name: &str, value: &str) -> Self {
            let headers = self.headers.get_or_insert_with(HashMap::new);
            let _unused = headers.insert(name.to_owned(), value.to_owned());

            self
        }

        /// Adds a file to attach, as on `POST /emails`. Each one needs `content` or `path`.
        /// The `content` of all files can total at most 25 MB. A `path` file does not count toward
        /// the 25 MB, because it is fetched when the email is sent.
        /// An empty file, a blocked file type, or a `path` that is not an `http` or `https` URL or
        /// that points to `localhost` returns a 422.
        #[inline]
        pub fn with_attachment(mut self, attachment: impl Into<CreateAttachment>) -> Self {
            self.attachments
                .get_or_insert_with(Vec::new)
                .push(attachment.into());
            self
        }

        /// Adds an `Idempotency-Key` header to the request, so the reply is sent only once. 1 to 256
        /// characters. A retry with the same key within 24 hours replays the first successful (2xx)
        /// response, with the `idempotent-replayed: true` header. A failed request is not stored, so
        /// you can retry it with the same key. Reusing the key with a different body, or while the
        /// first request is still running, returns a 409.
        ///
        /// <https://resend.com/docs/dashboard/emails/idempotency-keys>
        #[inline]
        pub fn with_idempotency_key(self, idempotency_key: &str) -> Idempotent<Self> {
            Idempotent {
                idempotency_key: Some(idempotency_key.to_owned()),
                data: self,
            }
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ReplyInboxThreadEmailResponse {
        pub id: EmailId,
    }

    /// See [relevant docs].
    ///
    /// [relevant docs]: <https://resend.com/docs/api-reference/inboxes/forward-thread-email#body-parameters>
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct ForwardInboxThreadEmailOptions {
        to: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        cc: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        bcc: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        html: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        subject: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        attachments: Option<Vec<CreateAttachment>>,
        /// Custom headers to add to the email.
        #[serde(skip_serializing_if = "Option::is_none")]
        headers: Option<HashMap<String, String>>,
    }

    impl ForwardInboxThreadEmailOptions {
        /// Creates a new [`ForwardInboxThreadEmailOptions`].
        ///
        /// - `to`: Recipient email addresses. Must name at least one address. `to`, `cc` and
        ///   `bcc` combined cannot exceed 50 recipients.
        pub fn new<S: Into<String>>(to: impl IntoIterator<Item = S>) -> Self {
            Self {
                to: to.into_iter().map(Into::into).collect(),
                cc: None,
                bcc: None,
                html: None,
                text: None,
                subject: None,
                attachments: None,
                headers: None,
            }
        }

        /// CC recipient email addresses. Counts toward the 50-recipient limit.
        #[inline]
        pub fn with_cc<S: Into<String>>(mut self, cc: impl IntoIterator<Item = S>) -> Self {
            self.cc = Some(cc.into_iter().map(Into::into).collect());
            self
        }

        /// BCC recipient email addresses. Counts toward the 50-recipient limit.
        #[inline]
        pub fn with_bcc<S: Into<String>>(mut self, bcc: impl IntoIterator<Item = S>) -> Self {
            self.bcc = Some(bcc.into_iter().map(Into::into).collect());
            self
        }

        /// An HTML note sent above the quoted original.
        #[inline]
        pub fn with_html(mut self, html: impl Into<String>) -> Self {
            self.html = Some(html.into());
            self
        }

        /// A plain-text note sent above the quoted original.
        #[inline]
        pub fn with_text(mut self, text: impl Into<String>) -> Self {
            self.text = Some(text.into());
            self
        }

        /// The subject of the forward. Cannot contain a line break. When omitted, the thread
        /// subject is used, prefixed with `Fwd:` if it does not already start with it.
        #[inline]
        pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
            self.subject = Some(subject.into());
            self
        }

        /// Adds an email header. The `In-Reply-To`, `References` and `Reply-To` headers are set by
        /// Resend and cannot be set here. Sending one of them, in any letter case, returns a 422.
        #[inline]
        pub fn with_header(mut self, name: &str, value: &str) -> Self {
            let headers = self.headers.get_or_insert_with(HashMap::new);
            let _unused = headers.insert(name.to_owned(), value.to_owned());

            self
        }

        /// Adds a file to attach, as on `POST /emails`. Each one needs `content` or `path`. They
        /// share one limit with the attachments of the original message, which the forward also
        /// sends. Together they can have at most 25 MB of `content`. A `path` file does not count
        /// toward the 25 MB, because it is fetched when the email is sent.
        #[inline]
        pub fn with_attachment(mut self, attachment: impl Into<CreateAttachment>) -> Self {
            self.attachments
                .get_or_insert_with(Vec::new)
                .push(attachment.into());
            self
        }

        /// Adds an `Idempotency-Key` header to the request.
        ///
        /// <https://resend.com/docs/dashboard/emails/idempotency-keys>
        #[inline]
        pub fn with_idempotency_key(self, idempotency_key: &str) -> Idempotent<Self> {
            Idempotent {
                idempotency_key: Some(idempotency_key.to_owned()),
                data: self,
            }
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ForwardInboxThreadEmailResponse {
        pub id: EmailId,
    }

    /// The kind of a draft.
    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum InboxDraftType {
        Standalone,
        Reply,
    }

    /// A file stored on a draft.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InboxDraftAttachment {
        pub id: String,
        pub filename: String,
        pub content_type: String,
        pub size: u64,
    }

    /// A file to store on a draft. Each one needs a `filename` and either `content` with a
    /// `content_type`, or a `path` URL that is downloaded when the request is made.
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateInboxDraftAttachment {
        #[serde(flatten)]
        content_or_path: ContentOrPath,
        filename: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        content_type: Option<String>,
    }

    impl CreateInboxDraftAttachment {
        /// Creates a new [`CreateInboxDraftAttachment`] from the content of a file.
        pub fn new(
            content: Vec<u8>,
            filename: impl Into<String>,
            content_type: impl Into<String>,
        ) -> Self {
            Self {
                content_or_path: ContentOrPath::Content(content),
                filename: filename.into(),
                content_type: Some(content_type.into()),
            }
        }

        /// Creates a new [`CreateInboxDraftAttachment`] from a URL. The file is downloaded when
        /// the request is made and stored on the draft. Its content type defaults to the one of
        /// the downloaded file.
        pub fn from_path(path: impl Into<String>, filename: impl Into<String>) -> Self {
            Self {
                content_or_path: ContentOrPath::Path(path.into()),
                filename: filename.into(),
                content_type: None,
            }
        }

        /// Sets the content type of the file.
        pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
            self.content_type = Some(content_type.into());
            self
        }
    }

    /// A single draft.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InboxDraft {
        /// The ID of the draft.
        pub id: InboxDraftId,
        /// `Reply` when the draft replies to an email in an existing thread. `Standalone` when
        /// the draft starts a new thread.
        #[serde(rename = "type")]
        pub r#type: InboxDraftType,
        /// The recipients. `None` or empty when no recipients are set.
        pub to: Option<Vec<String>>,
        /// The CC recipients.
        pub cc: Vec<String>,
        /// The BCC recipients.
        pub bcc: Vec<String>,
        /// The subject.
        pub subject: Option<String>,
        /// The HTML body.
        pub html: Option<String>,
        /// The plain text body.
        pub text: Option<String>,
        /// The Thread ID when the draft is a reply. `None` otherwise.
        pub thread_id: Option<InboxThreadId>,
        /// The Email ID the draft replies to. `None` otherwise.
        pub reply_to_email_id: Option<InboxThreadEmailId>,
        /// The ID of the outbound email queued when the draft was sent. `None` until the draft is
        /// sent.
        pub email_id: Option<EmailId>,
        /// The files stored on the draft.
        pub attachments: Vec<InboxDraftAttachment>,
        /// Timestamp when the draft was created.
        pub created_at: String,
        /// Timestamp when the draft was last saved.
        pub updated_at: String,
    }

    /// A draft as it appears in [`InboxesSvc::list_drafts`](crate::services::InboxesSvc::list_drafts).
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InboxDraftListItem {
        /// The ID of the draft.
        pub id: InboxDraftId,
        /// `Reply` when the draft replies to an email in an existing thread. `Standalone` when
        /// the draft starts a new thread.
        #[serde(rename = "type")]
        pub r#type: InboxDraftType,
        /// The recipients. `None` or empty when no recipients are set.
        pub to: Option<Vec<String>>,
        /// The CC recipients.
        pub cc: Vec<String>,
        /// The BCC recipients.
        pub bcc: Vec<String>,
        /// The subject.
        pub subject: Option<String>,
        /// A short preview of the body.
        pub snippet: Option<String>,
        /// The Thread ID when the draft is a reply. `None` otherwise.
        pub thread_id: Option<InboxThreadId>,
        /// The Email ID the draft replies to. `None` otherwise.
        pub reply_to_email_id: Option<InboxThreadEmailId>,
        /// Timestamp when the draft was last saved.
        pub updated_at: String,
        /// The files stored on the draft.
        pub attachments: Vec<InboxDraftAttachment>,
    }

    /// See [relevant docs].
    ///
    /// The combined recipients of `to`, `cc` and `bcc` cannot exceed 50. `thread_id`
    /// and `reply_to_email_id` must be sent together or not at all.
    ///
    /// [relevant docs]: <https://resend.com/docs/api-reference/inboxes/create-draft#body-parameters>
    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct CreateInboxDraftOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        to: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        cc: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        bcc: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        subject: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        html: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        attachments: Option<Vec<CreateInboxDraftAttachment>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thread_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_to_email_id: Option<String>,
    }

    impl CreateInboxDraftOptions {
        /// Creates a draft that starts a new thread.
        pub fn new() -> Self {
            Self::default()
        }

        /// Creates a reply draft in an existing thread.
        ///
        /// - `thread_id`: The Thread ID to reply in.
        /// - `reply_to_email_id`: The Email ID to reply to, as returned in `data[].id` by
        ///   [`InboxesSvc::list_thread_emails`](crate::services::InboxesSvc::list_thread_emails).
        ///
        /// A reply draft without `to` is addressed to the sender of the email it replies to, or
        /// to its Reply-To when set. For an email the inbox sent, it defaults to that email's
        /// `to` recipients. Pass an empty list to [`Self::with_to`] to leave it empty.
        pub fn reply(thread_id: impl Into<String>, reply_to_email_id: impl Into<String>) -> Self {
            Self {
                thread_id: Some(thread_id.into()),
                reply_to_email_id: Some(reply_to_email_id.into()),
                ..Self::default()
            }
        }

        /// The recipients. Each value can include a friendly name, for example
        /// `Ada Lovelace <ada@example.org>`.
        #[inline]
        pub fn with_to<S: Into<String>>(mut self, to: impl IntoIterator<Item = S>) -> Self {
            self.to = Some(to.into_iter().map(Into::into).collect());
            self
        }

        /// The CC recipients.
        #[inline]
        pub fn with_cc<S: Into<String>>(mut self, cc: impl IntoIterator<Item = S>) -> Self {
            self.cc = Some(cc.into_iter().map(Into::into).collect());
            self
        }

        /// The BCC recipients.
        #[inline]
        pub fn with_bcc<S: Into<String>>(mut self, bcc: impl IntoIterator<Item = S>) -> Self {
            self.bcc = Some(bcc.into_iter().map(Into::into).collect());
            self
        }

        /// The subject. Max 2000 characters. A string that is only whitespace, or that contains
        /// a line break, is rejected.
        #[inline]
        pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
            self.subject = Some(subject.into());
            self
        }

        /// The HTML body.
        #[inline]
        pub fn with_html(mut self, html: impl Into<String>) -> Self {
            self.html = Some(html.into());
            self
        }

        /// The plain text body.
        #[inline]
        pub fn with_text(mut self, text: impl Into<String>) -> Self {
            self.text = Some(text.into());
            self
        }

        /// Adds a file to store on the draft. The files on a draft can total at most 25 MB. An
        /// empty file or a blocked file type returns a 422.
        #[inline]
        pub fn with_attachment(mut self, attachment: CreateInboxDraftAttachment) -> Self {
            self.attachments
                .get_or_insert_with(Vec::new)
                .push(attachment);
            self
        }

        /// Adds an `Idempotency-Key` header to the request, so the draft is created only once. 1 to 256
        /// characters.
        ///
        /// <https://resend.com/docs/dashboard/emails/idempotency-keys>
        #[inline]
        pub fn with_idempotency_key(self, idempotency_key: &str) -> Idempotent<Self> {
            Idempotent {
                idempotency_key: Some(idempotency_key.to_owned()),
                data: self,
            }
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CreateInboxDraftResponse {
        /// The ID of the draft.
        pub id: InboxDraftId,
    }

    /// List of changes to apply to an [`InboxDraft`].
    ///
    /// Send at least one of `to`, `cc`, `bcc`, `subject`, `text`, `html` or `attachments`.
    /// Omitted fields are left unchanged. Setting `text` or `html` replaces the whole body, so the body part you omit
    /// becomes `null`. The merged draft must contain at least one non-empty field, and its
    /// combined recipients cannot exceed 50.
    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct UpdateInboxDraftOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        to: Option<Nullable<Vec<String>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        cc: Option<Nullable<Vec<String>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        bcc: Option<Nullable<Vec<String>>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        subject: Option<Nullable<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        html: Option<Nullable<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<Nullable<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        attachments: Option<Nullable<Vec<CreateInboxDraftAttachment>>>,
    }

    impl UpdateInboxDraftOptions {
        pub fn new() -> Self {
            Self::default()
        }

        /// The recipients. Each value can include a friendly name.
        #[inline]
        pub fn with_to<S: Into<String>>(mut self, to: impl IntoIterator<Item = S>) -> Self {
            self.to = Some(Nullable::Value(to.into_iter().map(Into::into).collect()));
            self
        }

        /// Sends `null` to clear the recipients.
        #[inline]
        pub fn clear_to(mut self) -> Self {
            self.to = Some(Nullable::Null);
            self
        }

        /// The CC recipients. Pass an empty list to clear.
        #[inline]
        pub fn with_cc<S: Into<String>>(mut self, cc: impl IntoIterator<Item = S>) -> Self {
            self.cc = Some(Nullable::Value(cc.into_iter().map(Into::into).collect()));
            self
        }

        /// Sends `null` to clear the CC recipients.
        #[inline]
        pub fn clear_cc(mut self) -> Self {
            self.cc = Some(Nullable::Null);
            self
        }

        /// The BCC recipients. Pass an empty list to clear.
        #[inline]
        pub fn with_bcc<S: Into<String>>(mut self, bcc: impl IntoIterator<Item = S>) -> Self {
            self.bcc = Some(Nullable::Value(bcc.into_iter().map(Into::into).collect()));
            self
        }

        /// Sends `null` to clear the BCC recipients.
        #[inline]
        pub fn clear_bcc(mut self) -> Self {
            self.bcc = Some(Nullable::Null);
            self
        }

        /// The subject. Max 2000 characters. Must not contain a line break.
        #[inline]
        pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
            self.subject = Some(Nullable::Value(subject.into()));
            self
        }

        /// Sends `null` to clear the subject.
        #[inline]
        pub fn clear_subject(mut self) -> Self {
            self.subject = Some(Nullable::Null);
            self
        }

        /// The HTML body.
        #[inline]
        pub fn with_html(mut self, html: impl Into<String>) -> Self {
            self.html = Some(Nullable::Value(html.into()));
            self
        }

        /// Sends `null` to clear the HTML body.
        #[inline]
        pub fn clear_html(mut self) -> Self {
            self.html = Some(Nullable::Null);
            self
        }

        /// The plain text body.
        #[inline]
        pub fn with_text(mut self, text: impl Into<String>) -> Self {
            self.text = Some(Nullable::Value(text.into()));
            self
        }

        /// Sends `null` to clear the plain text body.
        #[inline]
        pub fn clear_text(mut self) -> Self {
            self.text = Some(Nullable::Null);
            self
        }

        /// The files to store on the draft. Sending the field replaces every attachment on the
        /// draft. Pass an empty list to remove them all. The files on a draft can total at most
        /// 25 MB. An empty file or a blocked file type returns a 422.
        #[inline]
        pub fn with_attachments(
            mut self,
            attachments: impl IntoIterator<Item = CreateInboxDraftAttachment>,
        ) -> Self {
            self.attachments = Some(Nullable::Value(attachments.into_iter().collect()));
            self
        }

        /// Sends `null` to remove every attachment from the draft.
        #[inline]
        pub fn clear_attachments(mut self) -> Self {
            self.attachments = Some(Nullable::Null);
            self
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateInboxDraftResponse {
        /// The ID of the draft.
        pub id: InboxDraftId,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DeleteInboxDraftResponse {
        /// The ID of the draft.
        pub id: InboxDraftId,
        /// Indicates whether the draft was successfully deleted.
        pub deleted: bool,
    }

    /// Options for [`InboxesSvc::send_draft`](crate::services::InboxesSvc::send_draft).
    ///
    /// [`SendInboxDraftOptions::default()`] sends the draft with no body and without an
    /// `Idempotency-Key` header.
    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct SendInboxDraftOptions {
        /// Custom headers to add to the email.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub(crate) headers: Option<HashMap<String, String>>,
    }

    impl SendInboxDraftOptions {
        pub fn new() -> Self {
            Self::default()
        }

        /// Adds an email header. The `In-Reply-To`, `References` and `Reply-To` headers are set by
        /// Resend and cannot be set here. Sending one of them, in any letter case, returns a 422.
        #[inline]
        pub fn with_header(mut self, name: &str, value: &str) -> Self {
            let headers = self.headers.get_or_insert_with(HashMap::new);
            let _unused = headers.insert(name.to_owned(), value.to_owned());

            self
        }

        /// Adds an `Idempotency-Key` header to the request.
        ///
        /// <https://resend.com/docs/dashboard/emails/idempotency-keys>
        #[inline]
        pub fn with_idempotency_key(self, idempotency_key: &str) -> Idempotent<Self> {
            Idempotent {
                idempotency_key: Some(idempotency_key.to_owned()),
                data: self,
            }
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SendInboxDraftResponse {
        /// The ID of the sent email. Use it with the emails endpoints.
        pub id: EmailId,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test {
    use crate::types::{
        CreateAttachment, CreateInboxDraftAttachment, CreateInboxDraftOptions,
        CreateInboxLabelOptions, CreateInboxOptions, ForwardInboxThreadEmailOptions, Inbox,
        InboxAgent, InboxAgentAction, InboxAgentTone, InboxDraft, InboxDraftListItem,
        InboxDraftType, InboxEmailDirection, InboxFolder, InboxListItem, InboxThread,
        InboxThreadEmail, InboxThreadSearchResult, InboxThreadSummary, ListInboxLabelsResponse,
        ListInboxThreadLabelsResponse, ListInboxThreadsOptions, ReplyInboxThreadEmailOptions,
        SearchInboxThreadsOptions, SendInboxDraftOptions, UpdateInboxAgentOptions,
        UpdateInboxDraftOptions, UpdateInboxDraftResponse, UpdateInboxLabelOptions,
        UpdateInboxOptions, UpdateInboxThreadFolder, UpdateInboxThreadOptions,
    };
    use crate::{idempotent::Idempotent, list_opts::ListOptions, list_opts::ListResponse};

    use super::InboxThreadLabelsBody;

    fn query(value: &impl serde::Serialize) -> String {
        let url = reqwest::Url::parse("https://api.resend.com/").unwrap();
        #[cfg(not(feature = "blocking"))]
        let request = reqwest::Client::new()
            .get(url)
            .query(value)
            .build()
            .unwrap();
        #[cfg(feature = "blocking")]
        let request = reqwest::blocking::Client::new()
            .get(url)
            .query(value)
            .build()
            .unwrap();
        request.url().query().unwrap_or_default().to_owned()
    }

    #[test]
    fn serialize_create_inbox() {
        let opts = CreateInboxOptions::new("support@acme.com")
            .with_name("Customer Support")
            .with_from_name("Ada from Support")
            .with_forwarding(true);

        assert_eq!(
            serde_json::to_value(&opts).unwrap(),
            serde_json::json!({
                "email_address": "support@acme.com",
                "name": "Customer Support",
                "from_name": "Ada from Support",
                "forwarding": true
            })
        );

        let minimal = CreateInboxOptions::new("support@acme.com");
        assert_eq!(
            serde_json::to_value(&minimal).unwrap(),
            serde_json::json!({ "email_address": "support@acme.com" })
        );
    }

    #[test]
    fn serialize_update_inbox() {
        let opts = UpdateInboxOptions::new().with_from_name("");
        assert_eq!(
            serde_json::to_value(&opts).unwrap(),
            serde_json::json!({ "from_name": "" })
        );
    }

    #[test]
    fn deserialize_inbox() {
        let inbox = r#"{
  "object": "inbox",
  "id": "b3e2b2b6-3f0e-4c8e-9ad3-2f43a1e2c7f1",
  "name": "Customer Support",
  "email_address": "support@acme.com",
  "domain_id": "d91cd9bd-1176-453e-8fc1-35364d380206",
  "receiving_address": null,
  "from_name": "Ada from Support",
  "unread": 3,
  "drafts": 2,
  "last_received": "2026-08-05T14:03:11.229Z",
  "created_at": "2026-08-01T09:12:44.512Z",
  "updated_at": "2026-08-02T10:00:00.000Z"
}"#;
        let inbox = serde_json::from_str::<Inbox>(inbox).unwrap();
        assert_eq!(inbox.unread, 3);
        assert_eq!(inbox.updated_at, "2026-08-02T10:00:00.000Z");
        assert!(inbox.receiving_address.is_none());

        let list = r#"{
  "object": "list",
  "has_more": false,
  "data": [
    {
      "id": "b3e2b2b6-3f0e-4c8e-9ad3-2f43a1e2c7f1",
      "name": null,
      "email_address": "support@acme.com",
      "from_name": null,
      "unread": 0,
      "last_received": null,
      "created_at": "2026-08-01T09:12:44.512Z",
      "updated_at": "2026-08-02T10:00:00.000Z"
    }
  ]
}"#;
        let list = serde_json::from_str::<ListResponse<InboxListItem>>(list).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(
            list.data.first().unwrap().created_at,
            "2026-08-01T09:12:44.512Z"
        );
    }

    #[test]
    fn agent() {
        let agent = r#"{
  "object": "inbox_agent",
  "instructions": null,
  "tone": "concise_casual",
  "actions": ["draft_reply", "archive", "move_to_trash"]
}"#;
        let agent = serde_json::from_str::<InboxAgent>(agent).unwrap();
        assert!(agent.instructions.is_none());
        assert_eq!(agent.tone, Some(InboxAgentTone::ConciseCasual));
        assert_eq!(
            agent.actions,
            vec![
                InboxAgentAction::DraftReply,
                InboxAgentAction::Archive,
                InboxAgentAction::MoveToTrash
            ]
        );

        let no_tone = r#"{
  "object": "inbox_agent",
  "instructions": null,
  "tone": null,
  "actions": []
}"#;
        let no_tone = serde_json::from_str::<InboxAgent>(no_tone).unwrap();
        assert!(no_tone.tone.is_none());

        let update = UpdateInboxAgentOptions::new()
            .clear_instructions()
            .with_tone(InboxAgentTone::ExpandedProfessional)
            .with_actions([]);
        assert_eq!(
            serde_json::to_value(&update).unwrap(),
            serde_json::json!({
                "instructions": null,
                "tone": "expanded_professional",
                "actions": []
            })
        );

        assert_eq!(
            serde_json::to_value(UpdateInboxAgentOptions::new().clear_tone()).unwrap(),
            serde_json::json!({ "tone": null })
        );

        assert_eq!(
            serde_json::to_value(UpdateInboxAgentOptions::new()).unwrap(),
            serde_json::json!({})
        );
    }

    #[test]
    fn labels() {
        let create = CreateInboxLabelOptions::new("Urgent").with_color("#E93D82");
        assert_eq!(
            serde_json::to_value(&create).unwrap(),
            serde_json::json!({ "name": "Urgent", "color": "#E93D82" })
        );

        let update = UpdateInboxLabelOptions::new().with_color("#F76B15");
        assert_eq!(
            serde_json::to_value(&update).unwrap(),
            serde_json::json!({ "color": "#F76B15" })
        );

        let list = r##"{
  "object": "list",
  "data": [
    {
      "id": "7f9c1d2e-4a6b-4c3d-8e15-9b0a7c6d5e34",
      "name": "Urgent",
      "color": "#E93D82",
      "created_at": "2026-08-05T14:07:42.881Z"
    }
  ]
}"##;
        let list = serde_json::from_str::<ListInboxLabelsResponse>(list).unwrap();
        assert_eq!(list.data.first().unwrap().color, "#E93D82");
    }

    #[test]
    fn thread_filters() {
        let filters = ListInboxThreadsOptions::new()
            .with_folders([InboxFolder::Inbox, InboxFolder::Sent])
            .with_labels(["7f9c1d2e-4a6b-4c3d-8e15-9b0a7c6d5e34", "label-2"])
            .with_read(false);
        assert_eq!(
            query(&filters),
            "folders=inbox%2Csent&labels=7f9c1d2e-4a6b-4c3d-8e15-9b0a7c6d5e34%2Clabel-2&read=false"
        );

        assert_eq!(query(&ListInboxThreadsOptions::default()), "");
        assert_eq!(query(&ListInboxThreadsOptions::new().with_folders([])), "");

        let search = SearchInboxThreadsOptions::new()
            .with_query("invoice -draft")
            .with_from(["isabella@example.com", "carolina@example.com"])
            .with_has_attachment(true)
            .with_start_date("2026-09-01")
            .with_end_date("2026-09-30")
            .with_folders([InboxFolder::Inbox]);
        assert_eq!(
            query(&search),
            "query=invoice+-draft&from=isabella%40example.com%2Ccarolina%40example.com&has_attachment=true&start_date=2026-09-01&end_date=2026-09-30&folders=inbox"
        );

        let list_opts = ListOptions::default()
            .with_limit(5)
            .list_after("4d8e2a1c-9b3f-4c6d-8a21-3e5f7c9c0d12");
        assert_eq!(
            query(&list_opts),
            "limit=5&after=4d8e2a1c-9b3f-4c6d-8a21-3e5f7c9c0d12"
        );
    }

    #[test]
    fn threads() {
        let thread = r##"{
  "id": "4d8e2a1c-9b3f-4c6d-8a21-3e5f7c9c0d12",
  "subject": "Refund for order 1041",
  "from": "ada@example.org",
  "to": ["support@example.com"],
  "cc": [],
  "bcc": [],
  "labels": [{ "id": "7f9c1d2e-4a6b-4c3d-8e15-9b0a7c6d5e34", "name": "Urgent", "color": "#E93D82" }],
  "email_count": 2,
  "has_attachment": true,
  "has_draft": false,
  "read": false,
  "received_at": "2026-08-05T14:03:11.229Z",
  "folder": "inbox"
}"##;
        let parsed = serde_json::from_str::<InboxThread>(thread).unwrap();
        assert_eq!(parsed.folder, InboxFolder::Inbox);
        assert_eq!(parsed.labels.len(), 1);

        let search = r#"{
  "object": "list",
  "has_more": true,
  "data": [
    {
      "id": "4d8e2a1c-9b3f-4c6d-8a21-3e5f7c9c0d12",
      "subject": "September invoice",
      "from": null,
      "to": [],
      "cc": [],
      "bcc": [],
      "labels": [],
      "email_count": 1,
      "has_attachment": true,
      "has_draft": false,
      "read": true,
      "received_at": "2026-09-05T14:03:11.229Z",
      "folder": "archive",
      "matched_email_id": "e8a3f1c6-2b9d-4e5a-a7c4-9d1b6f3e2a80"
    }
  ]
}"#;
        let search = serde_json::from_str::<ListResponse<InboxThreadSearchResult>>(search).unwrap();
        let result = search.data.first().unwrap();
        assert_eq!(result.thread.folder, InboxFolder::Archive);
        assert!(result.matched_email_id.is_some());

        let summary = r#"{
  "object": "inbox_thread",
  "id": "4d8e2a1c-9b3f-4c6d-8a21-3e5f7c9c0d12",
  "subject": null,
  "folder": "trash",
  "labels": [],
  "read": false
}"#;
        let summary = serde_json::from_str::<InboxThreadSummary>(summary).unwrap();
        assert_eq!(summary.folder, InboxFolder::Trash);
    }

    #[test]
    fn thread_labels() {
        assert_eq!(
            serde_json::to_value(InboxThreadLabelsBody {
                labels: &["7f9c1d2e-4a6b-4c3d-8e15-9b0a7c6d5e34"]
            })
            .unwrap(),
            serde_json::json!({ "labels": ["7f9c1d2e-4a6b-4c3d-8e15-9b0a7c6d5e34"] })
        );
        assert_eq!(
            serde_json::to_value(InboxThreadLabelsBody { labels: &[] }).unwrap(),
            serde_json::json!({ "labels": [] })
        );

        let labels = r##"{
  "object": "list",
  "data": [{ "id": "7f9c1d2e-4a6b-4c3d-8e15-9b0a7c6d5e34", "name": "Urgent", "color": "#E93D82" }]
}"##;
        let labels = serde_json::from_str::<ListInboxThreadLabelsResponse>(labels).unwrap();
        assert_eq!(labels.data.first().unwrap().name, "Urgent");
    }

    #[test]
    fn deserialize_update_draft() {
        let update = r#"{ "object": "inbox_draft", "id": "c3a1e8b4-2d5f-4a7c-9e10-6b8d7f5a4c32" }"#;
        let _update = serde_json::from_str::<UpdateInboxDraftResponse>(update).unwrap();
    }

    #[test]
    fn serialize_update_thread() {
        let update = UpdateInboxThreadOptions::new()
            .with_read(true)
            .with_folder(UpdateInboxThreadFolder::Archive);
        assert_eq!(
            serde_json::to_value(update).unwrap(),
            serde_json::json!({ "read": true, "folder": "archive" })
        );

        for (folder, name) in [
            (UpdateInboxThreadFolder::Inbox, "inbox"),
            (UpdateInboxThreadFolder::Archive, "archive"),
            (UpdateInboxThreadFolder::Spam, "spam"),
            (UpdateInboxThreadFolder::Trash, "trash"),
        ] {
            let update = UpdateInboxThreadOptions::new().with_folder(folder);
            assert_eq!(
                serde_json::to_value(update).unwrap(),
                serde_json::json!({ "folder": name })
            );
        }
    }

    #[test]
    fn thread_emails() {
        let email = r#"{
  "id": "5b1a9f47-2c8d-4e6f-9a03-1d2e3f4a5b60",
  "direction": "inbound",
  "from": "Ada Lovelace <ada@example.org>",
  "to": ["support@acme.com"],
  "cc": [],
  "bcc": [],
  "reply_to": ["replies@example.org"],
  "subject": "Refund for order 1041",
  "message_id": "<1041@example.org>",
  "html": "<p>Could I get a refund for order 1041?</p>",
  "text": null,
  "attachments": [{ "id": "9d4f2b81-6c3a-4e7d-8b12-0a5c6d7e8f90", "filename": "receipt.pdf", "size": 20841 }],
  "read": false,
  "received_at": "2026-08-05T14:03:11.229Z"
}"#;
        let email = serde_json::from_str::<InboxThreadEmail>(email).unwrap();
        assert_eq!(email.direction, InboxEmailDirection::Inbound);
        assert_eq!(email.attachments.first().unwrap().size, Some(20841));

        let reply = ReplyInboxThreadEmailOptions::new()
            .with_text("Refund issued for order 1041.")
            .with_cc(["billing@acme.com"])
            .with_reply_all(true)
            .with_attachment(
                CreateAttachment::from_content(b"hi".to_vec()).with_filename("note.txt"),
            );
        assert_eq!(
            serde_json::to_value(&reply).unwrap(),
            serde_json::json!({
                "cc": ["billing@acme.com"],
                "text": "Refund issued for order 1041.",
                "reply_all": true,
                "attachments": [{ "content": [104, 105], "filename": "note.txt" }]
            })
        );

        let forward = ForwardInboxThreadEmailOptions::new(["colleague@example.org"])
            .with_html("<p>Flagging this refund request for you.</p>")
            .with_subject("Fwd: Refund for order 1041");
        assert_eq!(
            serde_json::to_value(&forward).unwrap(),
            serde_json::json!({
                "to": ["colleague@example.org"],
                "html": "<p>Flagging this refund request for you.</p>",
                "subject": "Fwd: Refund for order 1041"
            })
        );
    }

    #[test]
    fn idempotency_key() {
        let draft = CreateInboxDraftOptions::new()
            .with_text("Hi")
            .with_idempotency_key("draft-1");
        assert_eq!(draft.idempotency_key.as_deref(), Some("draft-1"));
        assert_eq!(
            serde_json::to_value(&draft).unwrap(),
            serde_json::json!({ "text": "Hi" })
        );

        let reply = ReplyInboxThreadEmailOptions::new()
            .with_text("Hi")
            .with_idempotency_key("reply-1");
        assert_eq!(reply.idempotency_key.as_deref(), Some("reply-1"));
        assert_eq!(
            serde_json::to_value(&reply).unwrap(),
            serde_json::json!({ "text": "Hi" })
        );

        let forward = ForwardInboxThreadEmailOptions::new(["a@example.com"])
            .with_idempotency_key("forward-1");
        assert_eq!(forward.idempotency_key.as_deref(), Some("forward-1"));
        assert_eq!(
            serde_json::to_value(&forward).unwrap(),
            serde_json::json!({ "to": ["a@example.com"] })
        );

        let headers = ReplyInboxThreadEmailOptions::new()
            .with_text("Hi")
            .with_header("X-Ticket-Id", "4821");
        assert_eq!(
            serde_json::to_value(&headers).unwrap(),
            serde_json::json!({ "text": "Hi", "headers": { "X-Ticket-Id": "4821" } })
        );
        let headers = ForwardInboxThreadEmailOptions::new(["a@example.com"])
            .with_header("X-Ticket-Id", "4821");
        assert_eq!(
            serde_json::to_value(&headers).unwrap(),
            serde_json::json!({ "to": ["a@example.com"], "headers": { "X-Ticket-Id": "4821" } })
        );
        let headers = SendInboxDraftOptions::new().with_header("X-Ticket-Id", "4821");
        assert_eq!(
            serde_json::to_value(&headers).unwrap(),
            serde_json::json!({ "headers": { "X-Ticket-Id": "4821" } })
        );

        let send = SendInboxDraftOptions::new().with_idempotency_key("send-1");
        assert_eq!(send.idempotency_key.as_deref(), Some("send-1"));
        let send: Idempotent<SendInboxDraftOptions> = SendInboxDraftOptions::default().into();
        assert!(send.idempotency_key.is_none());
    }

    #[test]
    fn drafts() {
        let standalone = CreateInboxDraftOptions::new()
            .with_to(["ada@example.org"])
            .with_subject("Refund for order 1041")
            .with_text("Refund issued for order 1041.");
        assert_eq!(
            serde_json::to_value(&standalone).unwrap(),
            serde_json::json!({
                "to": ["ada@example.org"],
                "subject": "Refund for order 1041",
                "text": "Refund issued for order 1041."
            })
        );

        let reply = CreateInboxDraftOptions::reply("thread", "email").with_to(Vec::<String>::new());
        assert_eq!(
            serde_json::to_value(&reply).unwrap(),
            serde_json::json!({ "to": [], "thread_id": "thread", "reply_to_email_id": "email" })
        );

        let update = UpdateInboxDraftOptions::new()
            .with_html("<p>Hi</p>")
            .clear_text()
            .clear_cc();
        assert_eq!(
            serde_json::to_value(&update).unwrap(),
            serde_json::json!({ "cc": null, "html": "<p>Hi</p>", "text": null })
        );

        let draft = r#"{
  "object": "inbox_draft",
  "id": "c3a1e8b4-2d5f-4a7c-9e10-6b8d7f5a4c32",
  "type": "standalone",
  "to": null,
  "cc": [],
  "bcc": [],
  "subject": "Refund for order 1041",
  "html": null,
  "text": "Refund issued for order 1041.",
  "thread_id": null,
  "reply_to_email_id": null,
  "email_id": null,
  "attachments": [],
  "created_at": "2026-08-05T14:12:04.110Z",
  "updated_at": "2026-08-05T14:12:04.110Z"
}"#;
        let draft = serde_json::from_str::<InboxDraft>(draft).unwrap();
        assert_eq!(draft.r#type, InboxDraftType::Standalone);
        assert!(draft.to.is_none());

        let list = r#"{
  "object": "list",
  "has_more": false,
  "data": [
    {
      "id": "c3a1e8b4-2d5f-4a7c-9e10-6b8d7f5a4c32",
      "type": "reply",
      "to": ["ada@example.org"],
      "cc": [],
      "bcc": [],
      "subject": null,
      "snippet": "Refund issued",
      "thread_id": "4d8e2a1c-9b3f-4c6d-8a21-3e5f7c9c0d12",
      "reply_to_email_id": "5b1a9f47-2c8d-4e6f-9a03-1d2e3f4a5b60",
      "attachments": [{ "id": "5c1e9d2a-7b3f-4e8a-9c6d-2f0a1b3c4d5e", "filename": "receipt.pdf", "content_type": "application/pdf", "size": 20841 }],
      "updated_at": "2026-08-05T14:12:04.110Z"
    }
  ]
}"#;
        let list = serde_json::from_str::<ListResponse<InboxDraftListItem>>(list).unwrap();
        let item = list.data.first().unwrap();
        assert_eq!(item.r#type, InboxDraftType::Reply);
        assert_eq!(
            item.attachments.first().unwrap().content_type,
            "application/pdf"
        );

        let with_attachment = CreateInboxDraftOptions::new()
            .with_text("See the receipt.")
            .with_attachment(CreateInboxDraftAttachment::new(
                b"hi".to_vec(),
                "note.txt",
                "text/plain",
            ));
        assert_eq!(
            serde_json::to_value(&with_attachment).unwrap(),
            serde_json::json!({
                "text": "See the receipt.",
                "attachments": [{ "content": [104, 105], "filename": "note.txt", "content_type": "text/plain" }]
            })
        );

        let replace = UpdateInboxDraftOptions::new().with_attachments([]);
        assert_eq!(
            serde_json::to_value(&replace).unwrap(),
            serde_json::json!({ "attachments": [] })
        );

        let clear = UpdateInboxDraftOptions::new().clear_attachments();
        assert_eq!(
            serde_json::to_value(&clear).unwrap(),
            serde_json::json!({ "attachments": null })
        );
    }

    #[test]
    fn draft_path_attachments() {
        let with_path_attachment = UpdateInboxDraftOptions::new().with_attachments([
            CreateInboxDraftAttachment::from_path("https://example.com/invoice.pdf", "invoice.pdf"),
            CreateInboxDraftAttachment::from_path("https://example.com/pic", "pic.png")
                .with_content_type("image/png"),
        ]);
        assert_eq!(
            serde_json::to_value(&with_path_attachment).unwrap(),
            serde_json::json!({
                "attachments": [
                    { "path": "https://example.com/invoice.pdf", "filename": "invoice.pdf" },
                    { "path": "https://example.com/pic", "filename": "pic.png", "content_type": "image/png" }
                ]
            })
        );
    }
}
