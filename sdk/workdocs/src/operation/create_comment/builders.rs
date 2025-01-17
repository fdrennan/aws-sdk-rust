// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_comment::_create_comment_output::CreateCommentOutputBuilder;

pub use crate::operation::create_comment::_create_comment_input::CreateCommentInputBuilder;

impl CreateCommentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_comment::CreateCommentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_comment::CreateCommentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_comment();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateComment`.
///
/// <p>Adds a new comment to the specified document version.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateCommentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_comment::builders::CreateCommentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_comment::CreateCommentOutput,
        crate::operation::create_comment::CreateCommentError,
    > for CreateCommentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_comment::CreateCommentOutput,
            crate::operation::create_comment::CreateCommentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateCommentFluentBuilder {
    /// Creates a new `CreateComment`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateComment as a reference.
    pub fn as_input(&self) -> &crate::operation::create_comment::builders::CreateCommentInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_comment::CreateCommentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_comment::CreateCommentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_comment::CreateComment::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_comment::CreateComment::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_comment::CreateCommentOutput,
        crate::operation::create_comment::CreateCommentError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p>
    pub fn authentication_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.authentication_token(input.into());
        self
    }
    /// <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p>
    pub fn set_authentication_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_authentication_token(input);
        self
    }
    /// <p>Amazon WorkDocs authentication token. Not required when using Amazon Web Services administrator credentials to access the API.</p>
    pub fn get_authentication_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_authentication_token()
    }
    /// <p>The ID of the document.</p>
    pub fn document_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.document_id(input.into());
        self
    }
    /// <p>The ID of the document.</p>
    pub fn set_document_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_document_id(input);
        self
    }
    /// <p>The ID of the document.</p>
    pub fn get_document_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_document_id()
    }
    /// <p>The ID of the document version.</p>
    pub fn version_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.version_id(input.into());
        self
    }
    /// <p>The ID of the document version.</p>
    pub fn set_version_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_version_id(input);
        self
    }
    /// <p>The ID of the document version.</p>
    pub fn get_version_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_version_id()
    }
    /// <p>The ID of the parent comment.</p>
    pub fn parent_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.parent_id(input.into());
        self
    }
    /// <p>The ID of the parent comment.</p>
    pub fn set_parent_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_parent_id(input);
        self
    }
    /// <p>The ID of the parent comment.</p>
    pub fn get_parent_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_parent_id()
    }
    /// <p>The ID of the root comment in the thread.</p>
    pub fn thread_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.thread_id(input.into());
        self
    }
    /// <p>The ID of the root comment in the thread.</p>
    pub fn set_thread_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_thread_id(input);
        self
    }
    /// <p>The ID of the root comment in the thread.</p>
    pub fn get_thread_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_thread_id()
    }
    /// <p>The text of the comment.</p>
    pub fn text(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.text(input.into());
        self
    }
    /// <p>The text of the comment.</p>
    pub fn set_text(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_text(input);
        self
    }
    /// <p>The text of the comment.</p>
    pub fn get_text(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_text()
    }
    /// <p>The visibility of the comment. Options are either PRIVATE, where the comment is visible only to the comment author and document owner and co-owners, or PUBLIC, where the comment is visible to document owners, co-owners, and contributors.</p>
    pub fn visibility(mut self, input: crate::types::CommentVisibilityType) -> Self {
        self.inner = self.inner.visibility(input);
        self
    }
    /// <p>The visibility of the comment. Options are either PRIVATE, where the comment is visible only to the comment author and document owner and co-owners, or PUBLIC, where the comment is visible to document owners, co-owners, and contributors.</p>
    pub fn set_visibility(mut self, input: ::std::option::Option<crate::types::CommentVisibilityType>) -> Self {
        self.inner = self.inner.set_visibility(input);
        self
    }
    /// <p>The visibility of the comment. Options are either PRIVATE, where the comment is visible only to the comment author and document owner and co-owners, or PUBLIC, where the comment is visible to document owners, co-owners, and contributors.</p>
    pub fn get_visibility(&self) -> &::std::option::Option<crate::types::CommentVisibilityType> {
        self.inner.get_visibility()
    }
    /// <p>Set this parameter to TRUE to send an email out to the document collaborators after the comment is created.</p>
    pub fn notify_collaborators(mut self, input: bool) -> Self {
        self.inner = self.inner.notify_collaborators(input);
        self
    }
    /// <p>Set this parameter to TRUE to send an email out to the document collaborators after the comment is created.</p>
    pub fn set_notify_collaborators(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_notify_collaborators(input);
        self
    }
    /// <p>Set this parameter to TRUE to send an email out to the document collaborators after the comment is created.</p>
    pub fn get_notify_collaborators(&self) -> &::std::option::Option<bool> {
        self.inner.get_notify_collaborators()
    }
}
