// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Fluent builder constructing a request to `CreateApplication`.
            ///
/// <p>Creates a Fleet Hub for AWS IoT Device Management web application.</p> <note> 
/// <p>Fleet Hub for AWS IoT Device Management is in public preview and is subject to change.</p> 
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateApplication {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::create_application_input::Builder
            }
impl CreateApplication  {
    /// Creates a new `CreateApplication`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::CreateApplication, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::CreateApplicationError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::CreateApplicationOutput, aws_smithy_http::result::SdkError<crate::error::CreateApplicationError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>The name of the web application.</p>
    pub fn application_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.application_name(input.into());
        self
    }
    /// <p>The name of the web application.</p>
    pub fn set_application_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_application_name(input);
        self
    }
    /// <p>An optional description of the web application.</p>
    pub fn application_description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.application_description(input.into());
        self
    }
    /// <p>An optional description of the web application.</p>
    pub fn set_application_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_application_description(input);
        self
    }
    /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The ARN of the role that the web application assumes when it interacts with AWS IoT Core.</p> <note> 
    /// <p>The name of the role must be in the form <code>AWSIotFleetHub_<i>random_string</i> </code>.</p> 
    /// </note>
    pub fn role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The ARN of the role that the web application assumes when it interacts with AWS IoT Core.</p> <note> 
    /// <p>The name of the role must be in the form <code>AWSIotFleetHub_<i>random_string</i> </code>.</p> 
    /// </note>
    pub fn set_role_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A set of key/value pairs that you can use to manage the web application resource.</p>
    pub fn tags(mut self, k: impl Into<std::string::String>, v: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>A set of key/value pairs that you can use to manage the web application resource.</p>
    pub fn set_tags(mut self, input: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}

/// Fluent builder constructing a request to `DeleteApplication`.
            ///
/// <p>Deletes a Fleet Hub for AWS IoT Device Management web application.</p> <note> 
/// <p>Fleet Hub for AWS IoT Device Management is in public preview and is subject to change.</p> 
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteApplication {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::delete_application_input::Builder
            }
impl DeleteApplication  {
    /// Creates a new `DeleteApplication`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::DeleteApplication, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::DeleteApplicationError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::DeleteApplicationOutput, aws_smithy_http::result::SdkError<crate::error::DeleteApplicationError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>The unique Id of the web application.</p>
    pub fn application_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The unique Id of the web application.</p>
    pub fn set_application_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
}

/// Fluent builder constructing a request to `DescribeApplication`.
            ///
/// <p>Gets information about a Fleet Hub for AWS IoT Device Management web application.</p> <note> 
/// <p>Fleet Hub for AWS IoT Device Management is in public preview and is subject to change.</p> 
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeApplication {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::describe_application_input::Builder
            }
impl DescribeApplication  {
    /// Creates a new `DescribeApplication`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::DescribeApplication, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::DescribeApplicationError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::DescribeApplicationOutput, aws_smithy_http::result::SdkError<crate::error::DescribeApplicationError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>The unique Id of the web application.</p>
    pub fn application_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The unique Id of the web application.</p>
    pub fn set_application_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
}

/// Fluent builder constructing a request to `ListApplications`.
            ///
/// <p>Gets a list of Fleet Hub for AWS IoT Device Management web applications for the current account.</p> <note> 
/// <p>Fleet Hub for AWS IoT Device Management is in public preview and is subject to change.</p> 
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListApplications {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::list_applications_input::Builder
            }
impl ListApplications  {
    /// Creates a new `ListApplications`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::ListApplications, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::ListApplicationsError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::ListApplicationsOutput, aws_smithy_http::result::SdkError<crate::error::ListApplicationsError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// Create a paginator for this request
                        ///
                        /// Paginators are used by calling [`send().await`](crate::paginator::ListApplicationsPaginator::send) which returns a `Stream`.
                        pub fn into_paginator(self) -> crate::paginator::ListApplicationsPaginator {
                            crate::paginator::ListApplicationsPaginator::new(self.handle, self.inner)
                        }
    /// <p>A token used to get the next set of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token used to get the next set of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}

/// Fluent builder constructing a request to `ListTagsForResource`.
            ///
/// <p>Lists the tags for the specified resource.</p> <note> 
/// <p>Fleet Hub for AWS IoT Device Management is in public preview and is subject to change.</p> 
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::list_tags_for_resource_input::Builder
            }
impl ListTagsForResource  {
    /// Creates a new `ListTagsForResource`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::ListTagsForResource, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::ListTagsForResourceOutput, aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>The ARN of the resource.</p>
    pub fn resource_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resource_arn(input.into());
        self
    }
    /// <p>The ARN of the resource.</p>
    pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_resource_arn(input);
        self
    }
}

/// Fluent builder constructing a request to `TagResource`.
            ///
/// <p>Adds to or modifies the tags of the specified resource. Tags are metadata which can be used to manage a resource.</p> <note> 
/// <p>Fleet Hub for AWS IoT Device Management is in public preview and is subject to change.</p> 
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::tag_resource_input::Builder
            }
impl TagResource  {
    /// Creates a new `TagResource`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::TagResource, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::TagResourceError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::TagResourceOutput, aws_smithy_http::result::SdkError<crate::error::TagResourceError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>The ARN of the resource.</p>
    pub fn resource_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resource_arn(input.into());
        self
    }
    /// <p>The ARN of the resource.</p>
    pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_resource_arn(input);
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The new or modified tags for the resource.</p>
    pub fn tags(mut self, k: impl Into<std::string::String>, v: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The new or modified tags for the resource.</p>
    pub fn set_tags(mut self, input: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}

/// Fluent builder constructing a request to `UntagResource`.
            ///
/// <p>Removes the specified tags (metadata) from the resource.</p> <note> 
/// <p>Fleet Hub for AWS IoT Device Management is in public preview and is subject to change.</p> 
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::untag_resource_input::Builder
            }
impl UntagResource  {
    /// Creates a new `UntagResource`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::UntagResource, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::UntagResourceError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::UntagResourceOutput, aws_smithy_http::result::SdkError<crate::error::UntagResourceError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>The ARN of the resource.</p>
    pub fn resource_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resource_arn(input.into());
        self
    }
    /// <p>The ARN of the resource.</p>
    pub fn set_resource_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_resource_arn(input);
        self
    }
    /// Appends an item to `tagKeys`.
    ///
    /// To override the contents of this collection use [`set_tag_keys`](Self::set_tag_keys).
    ///
    /// <p>A list of the keys of the tags to be removed from the resource.</p>
    pub fn tag_keys(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.tag_keys(input.into());
        self
    }
    /// <p>A list of the keys of the tags to be removed from the resource.</p>
    pub fn set_tag_keys(mut self, input: std::option::Option<std::vec::Vec<std::string::String>>) -> Self {
        self.inner = self.inner.set_tag_keys(input);
        self
    }
}

/// Fluent builder constructing a request to `UpdateApplication`.
            ///
/// <p>Updates information about a Fleet Hub for a AWS IoT Device Management web application.</p> <note> 
/// <p>Fleet Hub for AWS IoT Device Management is in public preview and is subject to change.</p> 
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateApplication {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::update_application_input::Builder
            }
impl UpdateApplication  {
    /// Creates a new `UpdateApplication`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::UpdateApplication, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::UpdateApplicationError>
                    >  {
                        let handle = self.handle.clone();
                        let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                    }
    
                    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::output::UpdateApplicationOutput, aws_smithy_http::result::SdkError<crate::error::UpdateApplicationError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// <p>The unique Id of the web application.</p>
    pub fn application_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The unique Id of the web application.</p>
    pub fn set_application_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>The name of the web application.</p>
    pub fn application_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.application_name(input.into());
        self
    }
    /// <p>The name of the web application.</p>
    pub fn set_application_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_application_name(input);
        self
    }
    /// <p>An optional description of the web application.</p>
    pub fn application_description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.application_description(input.into());
        self
    }
    /// <p>An optional description of the web application.</p>
    pub fn set_application_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_application_description(input);
        self
    }
    /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
}

