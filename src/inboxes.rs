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
        ListInboxThreadsOptions, ReplyInboxThreadEmailOptions, ReplyInboxThreadEmailResponse,
        SearchInboxThreadsOptions, SendInboxDraftOptions, SendInboxDraftResponse,
        UpdateInboxAgentOptions, UpdateInboxAgentResponse, UpdateInboxDraftOptions,
        UpdateInboxLabelOptions, UpdateInboxLabelResponse, UpdateInboxOptions, UpdateInboxResponse,
        UpdateInboxThreadOptions,
    },
};

/// `Resend` APIs for `/inboxes` endpoints.
#[derive(Clone, Debug)]
pub struct InboxesSvc(pub(crate) Arc<Config>);

impl InboxesSvc {
    /// Creates an inbox that sends and receives email at an address on one of your domains.
    ///
    /// The domain must belong to the team. Without `forwarding`, the domain must have receiving
    /// enabled, with DKIM and SPF verified and the inbound MX record verified or pending. With
    /// `forwarding` set to `true`, the domain only needs DKIM and SPF verified for sending, and
    /// Resend generates a `receiving_address` on the team inbound domain. A domain that does not
    /// meet these rules returns a 422 `validation_error`. An address or receiving address that is
    /// already in use returns a 409. A team that has reached the inbox limit of its plan returns
    /// a 403 `validation_error`. A blocked account returns a 451.
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
    /// The list items carry fewer fields than a single inbox. Retrieve an inbox to get all its
    /// fields. The `after` and `before` cursors take an inbox ID. A value that is not a UUID
    /// returns a 422, and an unknown ID returns a 404.
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
    /// `inbox_id_or_email` is the Inbox ID or the inbox email address, such as
    /// `support@acme.com`. A value that contains `@` must be a valid email address, and the match
    /// is case-insensitive. Any other value must be a UUID.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/get-inbox>
    #[maybe_async::maybe_async]
    pub async fn get(&self, inbox_id_or_email: &str) -> Result<Inbox> {
        let inbox_id_or_email = urlencoding::encode(inbox_id_or_email);
        let path = format!("/inboxes/{inbox_id_or_email}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<Inbox>().await?;

        Ok(content)
    }

    /// Updates the internal name of the inbox or the name recipients see when mail is sent from
    /// it. At least one of `name` or `from_name` is required.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/update-inbox>
    #[maybe_async::maybe_async]
    pub async fn update(
        &self,
        inbox_id: &str,
        update: UpdateInboxOptions,
    ) -> Result<UpdateInboxResponse> {
        let path = format!("/inboxes/{inbox_id}");

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&update)).await?;
        let content = response.json::<UpdateInboxResponse>().await?;

        Ok(content)
    }

    /// Removes the inbox. An inbox that does not exist or was already removed returns a 404
    /// `not_found`.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/delete-inbox>
    #[maybe_async::maybe_async]
    pub async fn delete(&self, inbox_id: &str) -> Result<DeleteInboxResponse> {
        let path = format!("/inboxes/{inbox_id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<DeleteInboxResponse>().await?;

        Ok(content)
    }

    /// Returns the agent settings of the inbox.
    ///
    /// An inbox without a configured agent returns empty settings: `instructions` and `tone` are
    /// `None` and `enabled_actions` is empty.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/get-agent>
    #[maybe_async::maybe_async]
    pub async fn get_agent(&self, inbox_id: &str) -> Result<InboxAgent> {
        let path = format!("/inboxes/{inbox_id}/agent");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<InboxAgent>().await?;

        Ok(content)
    }

    /// Updates the agent settings of the inbox. The agent is created when the inbox has none.
    /// Omitted fields keep their current value.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/update-agent>
    #[maybe_async::maybe_async]
    pub async fn update_agent(
        &self,
        inbox_id: &str,
        update: UpdateInboxAgentOptions,
    ) -> Result<UpdateInboxAgentResponse> {
        let path = format!("/inboxes/{inbox_id}/agent");

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&update)).await?;
        let content = response.json::<UpdateInboxAgentResponse>().await?;

        Ok(content)
    }

    /// Returns every label on the inbox, oldest first. The list is not paginated.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/list-labels>
    #[maybe_async::maybe_async]
    pub async fn list_labels(&self, inbox_id: &str) -> Result<ListInboxLabelsResponse> {
        let path = format!("/inboxes/{inbox_id}/labels");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<ListInboxLabelsResponse>().await?;

        Ok(content)
    }

    /// Create a label on an inbox.
    ///
    /// A label is a named, colored tag that belongs to one inbox. An inbox can have at most 100
    /// labels. Label names are unique per inbox, compared without case. A duplicate name or a
    /// full inbox returns a 422 `validation_error`.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/create-label>
    #[maybe_async::maybe_async]
    pub async fn create_label(
        &self,
        inbox_id: &str,
        label: CreateInboxLabelOptions,
    ) -> Result<CreateInboxLabelResponse> {
        let path = format!("/inboxes/{inbox_id}/labels");

        let request = self.0.build(Method::POST, &path);
        let response = self.0.send(request.json(&label)).await?;
        let content = response.json::<CreateInboxLabelResponse>().await?;

        Ok(content)
    }

    /// Renames or recolors a label. Omitted fields keep their current value.
    ///
    /// A new name must not match another label on the inbox, compared without case, or the
    /// request returns a 422 `validation_error`.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/update-label>
    #[maybe_async::maybe_async]
    pub async fn update_label(
        &self,
        inbox_id: &str,
        label_id: &str,
        update: UpdateInboxLabelOptions,
    ) -> Result<UpdateInboxLabelResponse> {
        let path = format!("/inboxes/{inbox_id}/labels/{label_id}");

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
        inbox_id: &str,
        label_id: &str,
    ) -> Result<DeleteInboxLabelResponse> {
        let path = format!("/inboxes/{inbox_id}/labels/{label_id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<DeleteInboxLabelResponse>().await?;

        Ok(content)
    }

    /// Returns the threads in an inbox, most recently active first.
    ///
    /// A thread is a conversation made up of one or more emails. The `after` and `before` cursors
    /// take a thread ID, and only one of them can be passed. A cursor thread that is not in the
    /// selected folders returns a 404. To find threads by text, sender, recipient, attachment, or
    /// date, use [`InboxesSvc::search_threads`].
    ///
    /// <https://resend.com/docs/api-reference/inboxes/list-threads>
    #[maybe_async::maybe_async]
    pub async fn list_threads<T>(
        &self,
        inbox_id: &str,
        filters: ListInboxThreadsOptions,
        list_opts: ListOptions<T>,
    ) -> Result<ListResponse<InboxThread>> {
        let path = format!("/inboxes/{inbox_id}/threads");

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
    /// Results are ordered by latest activity, newest first, and paginate like
    /// [`InboxesSvc::list_threads`]. Search can be up to about a minute behind new emails.
    /// Different parameters combine, so a thread must match all of them. `folders`, `labels`, and
    /// `read` apply to the thread. `query`, `from`, `to`, `cc`, `bcc`, `has_attachment`,
    /// `start_date`, and `end_date` must all match the same email in the thread. When any of these
    /// email filters is set, the matching email must itself be in one of `folders`. For `sent`,
    /// that means an email the inbox sent. When more than 10,000 threads match, search covers the
    /// 10,000 with the newest matching email.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/search-threads>
    #[maybe_async::maybe_async]
    pub async fn search_threads<T>(
        &self,
        inbox_id: &str,
        search: SearchInboxThreadsOptions,
        list_opts: ListOptions<T>,
    ) -> Result<ListResponse<InboxThreadSearchResult>> {
        let path = format!("/inboxes/{inbox_id}/threads/search");

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

    /// Returns the thread summary. The emails in the thread are listed with
    /// [`InboxesSvc::list_thread_emails`].
    ///
    /// <https://resend.com/docs/api-reference/inboxes/get-thread>
    #[maybe_async::maybe_async]
    pub async fn get_thread(&self, inbox_id: &str, thread_id: &str) -> Result<InboxThreadSummary> {
        let path = format!("/inboxes/{inbox_id}/threads/{thread_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<InboxThreadSummary>().await?;

        Ok(content)
    }

    /// Marks a thread read or unread, moves it to a folder, or adds and removes labels.
    ///
    /// At least one of `read`, `folder`, `add_label_ids`, or `remove_label_ids` is required. A
    /// label ID cannot be both added and removed in the same request. Removing a label from a
    /// thread keeps the label itself. Moving a thread out of `trash` restores it and clears its
    /// deletion deadline. The changes apply in this order: `read`, `folder`, added labels, removed
    /// labels. They are not atomic, so a 404 for a label leaves the earlier changes applied.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/update-thread>
    #[maybe_async::maybe_async]
    pub async fn update_thread(
        &self,
        inbox_id: &str,
        thread_id: &str,
        update: UpdateInboxThreadOptions,
    ) -> Result<InboxThreadSummary> {
        let path = format!("/inboxes/{inbox_id}/threads/{thread_id}");

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&update)).await?;
        let content = response.json::<InboxThreadSummary>().await?;

        Ok(content)
    }

    /// Moves the thread to `trash` and schedules it for deletion in 30 days.
    ///
    /// This is reversible. To restore the thread, update its `folder` to `inbox`, `archive`, or
    /// `spam`, which also clears the deletion deadline.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/delete-thread>
    #[maybe_async::maybe_async]
    pub async fn delete_thread(
        &self,
        inbox_id: &str,
        thread_id: &str,
    ) -> Result<DeleteInboxThreadResponse> {
        let path = format!("/inboxes/{inbox_id}/threads/{thread_id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<DeleteInboxThreadResponse>().await?;

        Ok(content)
    }

    /// Returns the messages of a thread, oldest first.
    ///
    /// Pass the `id` of the last message as `after` to get the next, newer page. Pass the `id` of
    /// the first message as `before` to get the previous, older page. You cannot combine `after`
    /// and `before`. A cursor that is not a message of the thread returns a 404.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/list-thread-emails>
    #[maybe_async::maybe_async]
    pub async fn list_thread_emails<T>(
        &self,
        inbox_id: &str,
        thread_id: &str,
        list_opts: ListOptions<T>,
    ) -> Result<ListResponse<InboxThreadEmail>> {
        let path = format!("/inboxes/{inbox_id}/threads/{thread_id}/emails");

        let request = self.0.build(Method::GET, &path).query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<InboxThreadEmail>>().await?;

        Ok(content)
    }

    /// Retrieve a single thread email.
    ///
    /// `email_id` is the Email ID, as returned in `data[].id` by
    /// [`InboxesSvc::list_thread_emails`].
    ///
    /// <https://resend.com/docs/api-reference/inboxes/get-thread-email>
    #[maybe_async::maybe_async]
    pub async fn get_thread_email(
        &self,
        inbox_id: &str,
        thread_id: &str,
        email_id: &str,
    ) -> Result<InboxThreadEmail> {
        let path = format!("/inboxes/{inbox_id}/threads/{thread_id}/emails/{email_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<InboxThreadEmail>().await?;

        Ok(content)
    }

    /// Sends a reply from the inbox address right away, with no stored draft.
    ///
    /// The `to` recipients come from the message you reply to. For a message the inbox received,
    /// the reply goes to its `Reply-To` addresses, or to its sender when it has no `Reply-To`. For
    /// a message the inbox sent, the reply goes to that message's `to` recipients. Set
    /// `reply_all` to also copy the other recipients of the message. The reply threads under the
    /// message through the `In-Reply-To` and `References` headers.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/reply-thread-email>
    #[maybe_async::maybe_async]
    pub async fn reply_thread_email(
        &self,
        inbox_id: &str,
        thread_id: &str,
        email_id: &str,
        reply: impl Into<Idempotent<ReplyInboxThreadEmailOptions>>,
    ) -> Result<ReplyInboxThreadEmailResponse> {
        let path = format!("/inboxes/{inbox_id}/threads/{thread_id}/emails/{email_id}/reply");
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
    /// The original message is quoted under a forwarded-message banner, below the optional
    /// `html` or `text` note. The attachments of the original message are sent with the forward,
    /// together with any `attachments` you add. The request fails when they exceed 10 files or
    /// 25 MB of `content` in total. An original attachment that was sent from a URL (`path`)
    /// cannot be forwarded, and the request returns a 422.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/forward-thread-email>
    #[maybe_async::maybe_async]
    pub async fn forward_thread_email(
        &self,
        inbox_id: &str,
        thread_id: &str,
        email_id: &str,
        forward: impl Into<Idempotent<ForwardInboxThreadEmailOptions>>,
    ) -> Result<ForwardInboxThreadEmailResponse> {
        let path = format!("/inboxes/{inbox_id}/threads/{thread_id}/emails/{email_id}/forward");
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
    /// The `after` and `before` cursors take a Draft ID from the current page. A cursor that is
    /// not an unsent draft on this inbox returns a 404.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/list-drafts>
    #[maybe_async::maybe_async]
    pub async fn list_drafts<T>(
        &self,
        inbox_id: &str,
        list_opts: ListOptions<T>,
    ) -> Result<ListResponse<InboxDraftListItem>> {
        let path = format!("/inboxes/{inbox_id}/drafts");

        let request = self.0.build(Method::GET, &path).query(&list_opts);
        let response = self.0.send(request).await?;
        let content = response.json::<ListResponse<InboxDraftListItem>>().await?;

        Ok(content)
    }

    /// Creates an unsent draft on the inbox.
    ///
    /// Use [`CreateInboxDraftOptions::new`] for a draft that starts a new thread, and
    /// [`CreateInboxDraftOptions::reply`] for a reply draft in an existing thread. At least one of
    /// `to`, `cc`, `bcc`, `subject`, `text`, `html` or `attachments` must be non-empty. The
    /// combined recipients of `to`, `cc` and `bcc` cannot exceed 50. Only one unsent reply draft
    /// can target an email, so a reply draft request for an email that already has one returns
    /// the ID of the existing draft. That request ignores every other field, except
    /// `attachments`, which replaces the attachments of the existing draft when sent. A reply
    /// draft created without `subject` stores an empty subject and gets no `Re:` prefix, so set
    /// one before you send it.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/create-draft>
    #[maybe_async::maybe_async]
    pub async fn create_draft(
        &self,
        inbox_id: &str,
        draft: impl Into<Idempotent<CreateInboxDraftOptions>>,
    ) -> Result<CreateInboxDraftResponse> {
        let path = format!("/inboxes/{inbox_id}/drafts");
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
    pub async fn get_draft(&self, inbox_id: &str, draft_id: &str) -> Result<InboxDraft> {
        let path = format!("/inboxes/{inbox_id}/drafts/{draft_id}");

        let request = self.0.build(Method::GET, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<InboxDraft>().await?;

        Ok(content)
    }

    /// Updates the content of an unsent draft. Omitted fields are left unchanged.
    ///
    /// Send at least one of `to`, `cc`, `bcc`, `subject`, `text`, `html` or `attachments`.
    /// Sending `attachments` replaces every attachment on the draft. A request that sets `text` or
    /// `html` replaces the whole body. The body part you omit from that request becomes
    /// `null`. The merged draft must still contain at least one non-empty field, and its combined
    /// recipients cannot exceed 50. A draft that was already sent, or that changed during the
    /// request, returns a 409.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/update-draft>
    #[maybe_async::maybe_async]
    pub async fn update_draft(
        &self,
        inbox_id: &str,
        draft_id: &str,
        update: UpdateInboxDraftOptions,
    ) -> Result<InboxDraft> {
        let path = format!("/inboxes/{inbox_id}/drafts/{draft_id}");

        let request = self.0.build(Method::PATCH, &path);
        let response = self.0.send(request.json(&update)).await?;
        let content = response.json::<InboxDraft>().await?;

        Ok(content)
    }

    /// Discards an unsent draft. A draft that was already sent, or that changed during the
    /// request, returns a 409.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/delete-draft>
    #[maybe_async::maybe_async]
    pub async fn delete_draft(
        &self,
        inbox_id: &str,
        draft_id: &str,
    ) -> Result<DeleteInboxDraftResponse> {
        let path = format!("/inboxes/{inbox_id}/drafts/{draft_id}");

        let request = self.0.build(Method::DELETE, &path);
        let response = self.0.send(request).await?;
        let content = response.json::<DeleteInboxDraftResponse>().await?;

        Ok(content)
    }

    /// Sends the draft from the inbox address.
    ///
    /// A reply draft is sent in its thread. A draft without a thread starts a new thread. The
    /// draft must have at least one `to` recipient, a non-empty `subject`, and `html` or `text`,
    /// or the request returns a 422. A draft that was already sent returns a 409. The request
    /// body is optional and takes only `headers`. Use [`SendInboxDraftOptions::with_header`] to
    /// add custom headers, and [`SendInboxDraftOptions::with_idempotency_key`] to send the draft
    /// only once.
    ///
    /// <https://resend.com/docs/api-reference/inboxes/send-draft>
    #[maybe_async::maybe_async]
    pub async fn send_draft(
        &self,
        inbox_id: &str,
        draft_id: &str,
        opts: impl Into<Idempotent<SendInboxDraftOptions>>,
    ) -> Result<SendInboxDraftResponse> {
        let path = format!("/inboxes/{inbox_id}/drafts/{draft_id}/send");
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

#[allow(unreachable_pub)]
pub mod types {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    use crate::idempotent::Idempotent;
    use crate::types::{CreateAttachment, DomainId, EmailId};

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
        /// The object type. Always `inbox`.
        pub object: String,
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
        ///
        /// - `email_address`: The inbox email address on one of your domains. The API trims and
        ///   lowercases it. The part before the `@` is at most 64 characters and contains only
        ///   letters, numbers, and `. _ % + -`. It cannot start or end with a dot or contain two
        ///   dots in a row.
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
        /// `Name <email>` address. Max 78 characters. It cannot contain a colon, `<`, `>`, line
        /// breaks, or other control characters. An email address inside the name must be on a
        /// domain the team has verified for sending. An empty string leaves it unset.
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
        /// Type of the response object.
        pub object: String,
        /// The ID of the inbox.
        pub id: InboxId,
    }

    /// List of changes to apply to an [`Inbox`]. Provide at least one of `name` or `from_name`.
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
        /// `Name <email>` address. Max 78 characters. It cannot contain a colon, `<`, `>`, line
        /// breaks, or other control characters. An email address inside the name must be on a
        /// domain the team has verified for sending. An empty string removes it.
        #[inline]
        pub fn with_from_name(mut self, from_name: impl Into<String>) -> Self {
            self.from_name = Some(from_name.into());
            self
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateInboxResponse {
        /// The object type.
        pub object: String,
        /// The ID of the inbox.
        pub id: InboxId,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DeleteInboxResponse {
        /// The object type.
        pub object: String,
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
        ArchiveThread,
        DeleteThread,
    }

    /// The agent settings of an inbox.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InboxAgent {
        /// The object type. Always `inbox_agent`.
        pub object: String,
        /// Instructions the agent follows when it handles the threads of the inbox. `None` when
        /// no instructions are set.
        pub instructions: Option<String>,
        /// The tone the agent writes in, such as `friendly and concise`. `None` when no tone is
        /// set.
        pub tone: Option<String>,
        /// The actions the agent can take. Empty when no actions are enabled.
        pub enabled_actions: Vec<InboxAgentAction>,
    }

    /// List of changes to apply to an [`InboxAgent`].
    ///
    /// At least one of `instructions`, `tone`, or `enabled_actions` must be provided. Omitted
    /// fields keep their current value.
    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct UpdateInboxAgentOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        instructions: Option<Nullable<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tone: Option<Nullable<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        enabled_actions: Option<Vec<InboxAgentAction>>,
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

        /// The tone the agent writes in, such as `friendly and concise`. Max 64 characters.
        /// Leading and trailing whitespace is trimmed, and the trimmed value must not be empty.
        #[inline]
        pub fn with_tone(mut self, tone: impl Into<String>) -> Self {
            self.tone = Some(Nullable::Value(tone.into()));
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
        pub fn with_enabled_actions(
            mut self,
            enabled_actions: impl IntoIterator<Item = InboxAgentAction>,
        ) -> Self {
            self.enabled_actions = Some(enabled_actions.into_iter().collect());
            self
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateInboxAgentResponse {
        /// The object type. Always `inbox_agent`.
        pub object: String,
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
        /// Type of the response object.
        pub object: String,
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
        /// Type of the response object.
        pub object: String,
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
        /// Type of the response object.
        pub object: String,
        /// The ID of the updated label.
        pub id: InboxLabelId,
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DeleteInboxLabelResponse {
        /// Type of the response object.
        pub object: String,
        /// The ID of the removed label.
        pub id: InboxLabelId,
        /// Indicates whether the label was deleted.
        pub deleted: bool,
    }

    /// The folder a thread is in. Every thread is in exactly one folder.
    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum InboxFolder {
        /// Holds threads with at least one received email.
        Inbox,
        /// Holds threads moved there.
        Archive,
        /// Holds threads moved there.
        Spam,
        /// Holds threads whose emails are all outbound, and is assigned automatically.
        Sent,
        /// Holds threads scheduled for deletion in 30 days.
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
        /// The ID of the label.
        pub id: InboxLabelId,
        /// The name of the label.
        pub name: String,
        /// The color of the label, as a `#RRGGBB` hex color in uppercase.
        pub color: String,
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

    /// Where `query` matched the email in `matched_email_id`, as text fragments with each match
    /// wrapped in `**`. Only keys with a match are present.
    #[must_use]
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct InboxThreadSearchHighlights {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub subject: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub body: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub from: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub to: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cc: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bcc: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attachments: Option<Vec<String>>,
    }

    /// A thread returned by [`InboxesSvc::search_threads`](crate::services::InboxesSvc::search_threads).
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InboxThreadSearchResult {
        #[serde(flatten)]
        pub thread: InboxThread,
        /// Identifies the email in the thread that matched the search. When `query` has words or
        /// phrases, it is the most relevant match. Otherwise, it is the newest. The get, reply and
        /// forward thread email endpoints accept it as `email_id`, but it is not equal to the `id`
        /// those endpoints return. `None` when the search index returns no matched email for the
        /// thread.
        pub matched_email_id: Option<InboxThreadEmailId>,
        /// Where `query` matched the email in `matched_email_id`. Empty when there is no `query`.
        pub highlights: InboxThreadSearchHighlights,
    }

    /// The summary of a single thread.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InboxThreadSummary {
        /// The object type.
        pub object: String,
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

        /// The folders to list. A thread is returned when any of its emails is in one of the
        /// folders, so a thread in `inbox` with an email you sent matches both `inbox` and
        /// `sent`. Defaults to `inbox`, or to `inbox,archive,sent` when `labels` is set. `spam`
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
        /// covers that whole day. A timestamp is used as given. An `end_date` before
        /// `start_date` returns a 422 `validation_error`.
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
    ///
    /// At least one of `read`, `folder`, `add_label_ids`, or `remove_label_ids` is required. A
    /// label ID cannot be both added and removed in the same request.
    #[must_use]
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct UpdateInboxThreadOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        read: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        folder: Option<UpdateInboxThreadFolder>,
        #[serde(skip_serializing_if = "Option::is_none")]
        add_label_ids: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        remove_label_ids: Option<Vec<String>>,
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

        /// Label IDs to apply to the thread. Between 1 and 10 IDs. A label ID that does not exist
        /// in the inbox returns a 404.
        #[inline]
        pub fn with_add_label_ids<S: AsRef<str>>(
            mut self,
            label_ids: impl IntoIterator<Item = S>,
        ) -> Self {
            self.add_label_ids = Some(
                label_ids
                    .into_iter()
                    .map(|label_id| label_id.as_ref().to_owned())
                    .collect(),
            );
            self
        }

        /// Label IDs to remove from the thread. Between 1 and 10 IDs. The labels themselves are
        /// kept. A label ID that does not exist in the inbox returns a 404.
        #[inline]
        pub fn with_remove_label_ids<S: AsRef<str>>(
            mut self,
            label_ids: impl IntoIterator<Item = S>,
        ) -> Self {
            self.remove_label_ids = Some(
                label_ids
                    .into_iter()
                    .map(|label_id| label_id.as_ref().to_owned())
                    .collect(),
            );
            self
        }
    }

    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DeleteInboxThreadResponse {
        /// The object type.
        pub object: String,
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
        /// The inbox received the message. A message the inbox sent to itself is `Inbound`, so a
        /// reply to it goes to its sender.
        Inbound,
        /// The inbox sent the message.
        Outbound,
    }

    /// An attachment of a thread email.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InboxThreadEmailAttachment {
        /// The ID of the attachment.
        pub id: String,
        /// The filename of the attachment.
        pub filename: Option<String>,
        /// The size of the attachment in bytes.
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
    /// At least one of `html` or `text` must contain more than whitespace.
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

        /// Adds a file to attach, as on `POST /emails`. Each one needs `content` or `path`. Up to
        /// 10 files. The `content` of all files can total at most 25 MB. A `path` file counts
        /// toward the 10 files only, because it is fetched when the email is sent.
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
        /// The ID of the sent email. Use it with the emails endpoints.
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
        /// A header name cannot be empty, and names and values cannot contain line breaks or null
        /// characters, or the request returns a 422.
        #[inline]
        pub fn with_header(mut self, name: &str, value: &str) -> Self {
            let headers = self.headers.get_or_insert_with(HashMap::new);
            let _unused = headers.insert(name.to_owned(), value.to_owned());

            self
        }

        /// Adds a file to attach, as on `POST /emails`. Each one needs `content` or `path`. They
        /// share one limit with the attachments of the original message, which the forward also
        /// sends. Together they can have up to 10 files and at most 25 MB of `content`. A `path`
        /// file counts toward the 10 files only, because it is fetched when the email is sent.
        /// An empty file, a blocked file type, or a `path` that is not an `http` or `https` URL or
        /// that points to `localhost` returns a 422.
        #[inline]
        pub fn with_attachment(mut self, attachment: impl Into<CreateAttachment>) -> Self {
            self.attachments
                .get_or_insert_with(Vec::new)
                .push(attachment.into());
            self
        }

        /// Adds an `Idempotency-Key` header to the request, so the forward is sent only once. 1 to 256
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
    pub struct ForwardInboxThreadEmailResponse {
        /// The ID of the sent email. Use it with the emails endpoints.
        pub id: EmailId,
    }

    /// The kind of a draft.
    #[must_use]
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum InboxDraftType {
        /// The draft starts a new thread.
        Standalone,
        /// The draft replies to an email in an existing thread.
        Reply,
    }

    /// A file stored on a draft.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InboxDraftAttachment {
        /// The ID of the attachment.
        pub id: String,
        /// The filename of the attachment.
        pub filename: String,
        /// The MIME type of the attachment.
        pub content_type: String,
        /// The size of the attachment in bytes.
        pub size: u64,
    }

    /// A file to store on a draft. Each one needs `content`, `filename` and `content_type`.
    #[must_use]
    #[derive(Debug, Clone, Serialize)]
    pub struct CreateInboxDraftAttachment {
        content: Vec<u8>,
        filename: String,
        content_type: String,
    }

    impl CreateInboxDraftAttachment {
        /// Creates a new [`CreateInboxDraftAttachment`].
        ///
        /// - `content`: The content of the file.
        /// - `filename`: The filename of the attachment.
        /// - `content_type`: The MIME type of the attachment, such as `application/pdf`.
        pub fn new(
            content: Vec<u8>,
            filename: impl Into<String>,
            content_type: impl Into<String>,
        ) -> Self {
            Self {
                content,
                filename: filename.into(),
                content_type: content_type.into(),
            }
        }
    }

    /// A single draft.
    #[must_use]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InboxDraft {
        /// The object type.
        pub object: String,
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
        /// The files stored on the draft.
        pub attachments: Vec<InboxDraftAttachment>,
        /// Timestamp when the draft was last saved.
        pub updated_at: String,
    }

    /// See [relevant docs].
    ///
    /// At least one of `to`, `cc`, `bcc`, `subject`, `text`, `html` or `attachments` must be
    /// non-empty. The combined recipients of `to`, `cc` and `bcc` cannot exceed 50.
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
        thread_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_to_email_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        attachments: Option<Vec<CreateInboxDraftAttachment>>,
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
    pub struct CreateInboxDraftResponse {
        /// Type of the response object.
        pub object: String,
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
    pub struct DeleteInboxDraftResponse {
        /// The object type.
        pub object: String,
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
        /// A header name cannot be empty, and names and values cannot contain line breaks or null
        /// characters, or the request returns a 422.
        #[inline]
        pub fn with_header(mut self, name: &str, value: &str) -> Self {
            let headers = self.headers.get_or_insert_with(HashMap::new);
            let _unused = headers.insert(name.to_owned(), value.to_owned());

            self
        }

        /// Adds an `Idempotency-Key` header to the request, so the draft is sent only once. 1 to 256
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
        InboxAgent, InboxAgentAction, InboxDraft, InboxDraftListItem, InboxDraftType,
        InboxEmailDirection, InboxFolder, InboxListItem, InboxThread, InboxThreadEmail,
        InboxThreadSearchResult, InboxThreadSummary, ListInboxLabelsResponse,
        ListInboxThreadsOptions, ReplyInboxThreadEmailOptions, SearchInboxThreadsOptions,
        SendInboxDraftOptions, UpdateInboxAgentOptions, UpdateInboxDraftOptions,
        UpdateInboxLabelOptions, UpdateInboxOptions, UpdateInboxThreadFolder,
        UpdateInboxThreadOptions,
    };
    use crate::{idempotent::Idempotent, list_opts::ListOptions, list_opts::ListResponse};

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
  "created_at": "2026-08-01T09:12:44.512Z"
}"#;
        let inbox = serde_json::from_str::<Inbox>(inbox).unwrap();
        assert_eq!(inbox.unread, 3);
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
      "last_received": null
    }
  ]
}"#;
        let list = serde_json::from_str::<ListResponse<InboxListItem>>(list).unwrap();
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn agent() {
        let agent = r#"{
  "object": "inbox_agent",
  "instructions": null,
  "tone": "friendly and concise",
  "enabled_actions": ["draft_reply", "add_labels"]
}"#;
        let agent = serde_json::from_str::<InboxAgent>(agent).unwrap();
        assert!(agent.instructions.is_none());
        assert_eq!(
            agent.enabled_actions,
            vec![InboxAgentAction::DraftReply, InboxAgentAction::AddLabels]
        );

        let update = UpdateInboxAgentOptions::new()
            .clear_instructions()
            .with_tone("friendly and concise")
            .with_enabled_actions([]);
        assert_eq!(
            serde_json::to_value(&update).unwrap(),
            serde_json::json!({
                "instructions": null,
                "tone": "friendly and concise",
                "enabled_actions": []
            })
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
      "matched_email_id": "e8a3f1c6-2b9d-4e5a-a7c4-9d1b6f3e2a80",
      "highlights": {
        "subject": ["September **invoice**"],
        "attachments": ["**invoice**-2026-09.pdf"]
      }
    }
  ]
}"#;
        let search = serde_json::from_str::<ListResponse<InboxThreadSearchResult>>(search).unwrap();
        let result = search.data.first().unwrap();
        assert_eq!(result.thread.folder, InboxFolder::Archive);
        assert!(result.highlights.body.is_none());
        assert_eq!(result.highlights.subject.as_ref().unwrap().len(), 1);

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
    fn serialize_update_thread() {
        let update = UpdateInboxThreadOptions::new()
            .with_read(true)
            .with_folder(UpdateInboxThreadFolder::Archive)
            .with_add_label_ids(["7f9c1d2e-4a6b-4c3d-8e15-9b0a7c6d5e34"])
            .with_remove_label_ids(["2b6d0f4e-8a1c-4e3b-9d57-1c0e8a6f4b21"]);
        assert_eq!(
            serde_json::to_value(&update).unwrap(),
            serde_json::json!({
                "read": true,
                "folder": "archive",
                "add_label_ids": ["7f9c1d2e-4a6b-4c3d-8e15-9b0a7c6d5e34"],
                "remove_label_ids": ["2b6d0f4e-8a1c-4e3b-9d57-1c0e8a6f4b21"]
            })
        );

        for (folder, name) in [
            (UpdateInboxThreadFolder::Inbox, "inbox"),
            (UpdateInboxThreadFolder::Archive, "archive"),
            (UpdateInboxThreadFolder::Spam, "spam"),
            (UpdateInboxThreadFolder::Trash, "trash"),
        ] {
            let update = UpdateInboxThreadOptions::new().with_folder(folder);
            assert_eq!(
                serde_json::to_value(&update).unwrap(),
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
}
