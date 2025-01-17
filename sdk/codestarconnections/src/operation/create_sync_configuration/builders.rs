// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_sync_configuration::_create_sync_configuration_output::CreateSyncConfigurationOutputBuilder;

pub use crate::operation::create_sync_configuration::_create_sync_configuration_input::CreateSyncConfigurationInputBuilder;

impl CreateSyncConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_sync_configuration::CreateSyncConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_sync_configuration::CreateSyncConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_sync_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateSyncConfiguration`.
///
/// <p>Creates a sync configuration which allows Amazon Web Services to sync content from a Git repository to update a specified Amazon Web Services resource. Parameters for the sync configuration are determined by the sync type.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateSyncConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_sync_configuration::builders::CreateSyncConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_sync_configuration::CreateSyncConfigurationOutput,
        crate::operation::create_sync_configuration::CreateSyncConfigurationError,
    > for CreateSyncConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_sync_configuration::CreateSyncConfigurationOutput,
            crate::operation::create_sync_configuration::CreateSyncConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateSyncConfigurationFluentBuilder {
    /// Creates a new `CreateSyncConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateSyncConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::create_sync_configuration::builders::CreateSyncConfigurationInputBuilder {
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
        crate::operation::create_sync_configuration::CreateSyncConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_sync_configuration::CreateSyncConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_sync_configuration::CreateSyncConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_sync_configuration::CreateSyncConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_sync_configuration::CreateSyncConfigurationOutput,
        crate::operation::create_sync_configuration::CreateSyncConfigurationError,
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
    /// <p>The branch in the repository from which changes will be synced.</p>
    pub fn branch(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.branch(input.into());
        self
    }
    /// <p>The branch in the repository from which changes will be synced.</p>
    pub fn set_branch(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_branch(input);
        self
    }
    /// <p>The branch in the repository from which changes will be synced.</p>
    pub fn get_branch(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_branch()
    }
    /// <p>The file name of the configuration file that manages syncing between the connection and the repository. This configuration file is stored in the repository.</p>
    pub fn config_file(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.config_file(input.into());
        self
    }
    /// <p>The file name of the configuration file that manages syncing between the connection and the repository. This configuration file is stored in the repository.</p>
    pub fn set_config_file(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_config_file(input);
        self
    }
    /// <p>The file name of the configuration file that manages syncing between the connection and the repository. This configuration file is stored in the repository.</p>
    pub fn get_config_file(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_config_file()
    }
    /// <p>The ID of the repository link created for the connection. A repository link allows Git sync to monitor and sync changes to files in a specified Git repository.</p>
    pub fn repository_link_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.repository_link_id(input.into());
        self
    }
    /// <p>The ID of the repository link created for the connection. A repository link allows Git sync to monitor and sync changes to files in a specified Git repository.</p>
    pub fn set_repository_link_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_repository_link_id(input);
        self
    }
    /// <p>The ID of the repository link created for the connection. A repository link allows Git sync to monitor and sync changes to files in a specified Git repository.</p>
    pub fn get_repository_link_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_repository_link_id()
    }
    /// <p>The name of the Amazon Web Services resource (for example, a CloudFormation stack in the case of CFN_STACK_SYNC) that will be synchronized from the linked repository.</p>
    pub fn resource_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_name(input.into());
        self
    }
    /// <p>The name of the Amazon Web Services resource (for example, a CloudFormation stack in the case of CFN_STACK_SYNC) that will be synchronized from the linked repository.</p>
    pub fn set_resource_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_name(input);
        self
    }
    /// <p>The name of the Amazon Web Services resource (for example, a CloudFormation stack in the case of CFN_STACK_SYNC) that will be synchronized from the linked repository.</p>
    pub fn get_resource_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_name()
    }
    /// <p>The ARN of the IAM role that grants permission for Amazon Web Services to use Git sync to update a given Amazon Web Services resource on your behalf.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The ARN of the IAM role that grants permission for Amazon Web Services to use Git sync to update a given Amazon Web Services resource on your behalf.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>The ARN of the IAM role that grants permission for Amazon Web Services to use Git sync to update a given Amazon Web Services resource on your behalf.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_role_arn()
    }
    /// <p>The type of sync configuration.</p>
    pub fn sync_type(mut self, input: crate::types::SyncConfigurationType) -> Self {
        self.inner = self.inner.sync_type(input);
        self
    }
    /// <p>The type of sync configuration.</p>
    pub fn set_sync_type(mut self, input: ::std::option::Option<crate::types::SyncConfigurationType>) -> Self {
        self.inner = self.inner.set_sync_type(input);
        self
    }
    /// <p>The type of sync configuration.</p>
    pub fn get_sync_type(&self) -> &::std::option::Option<crate::types::SyncConfigurationType> {
        self.inner.get_sync_type()
    }
}
