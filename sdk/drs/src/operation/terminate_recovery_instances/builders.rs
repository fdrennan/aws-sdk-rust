// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::terminate_recovery_instances::_terminate_recovery_instances_output::TerminateRecoveryInstancesOutputBuilder;

pub use crate::operation::terminate_recovery_instances::_terminate_recovery_instances_input::TerminateRecoveryInstancesInputBuilder;

impl TerminateRecoveryInstancesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::terminate_recovery_instances::TerminateRecoveryInstancesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::terminate_recovery_instances::TerminateRecoveryInstancesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.terminate_recovery_instances();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `TerminateRecoveryInstances`.
///
/// <p>Initiates a Job for terminating the EC2 resources associated with the specified Recovery Instances, and then will delete the Recovery Instances from the Elastic Disaster Recovery service.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct TerminateRecoveryInstancesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::terminate_recovery_instances::builders::TerminateRecoveryInstancesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::terminate_recovery_instances::TerminateRecoveryInstancesOutput,
        crate::operation::terminate_recovery_instances::TerminateRecoveryInstancesError,
    > for TerminateRecoveryInstancesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::terminate_recovery_instances::TerminateRecoveryInstancesOutput,
            crate::operation::terminate_recovery_instances::TerminateRecoveryInstancesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl TerminateRecoveryInstancesFluentBuilder {
    /// Creates a new `TerminateRecoveryInstances`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the TerminateRecoveryInstances as a reference.
    pub fn as_input(&self) -> &crate::operation::terminate_recovery_instances::builders::TerminateRecoveryInstancesInputBuilder {
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
        crate::operation::terminate_recovery_instances::TerminateRecoveryInstancesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::terminate_recovery_instances::TerminateRecoveryInstancesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::terminate_recovery_instances::TerminateRecoveryInstances::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::terminate_recovery_instances::TerminateRecoveryInstances::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::terminate_recovery_instances::TerminateRecoveryInstancesOutput,
        crate::operation::terminate_recovery_instances::TerminateRecoveryInstancesError,
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
    /// Appends an item to `recoveryInstanceIDs`.
    ///
    /// To override the contents of this collection use [`set_recovery_instance_ids`](Self::set_recovery_instance_ids).
    ///
    /// <p>The IDs of the Recovery Instances that should be terminated.</p>
    pub fn recovery_instance_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.recovery_instance_ids(input.into());
        self
    }
    /// <p>The IDs of the Recovery Instances that should be terminated.</p>
    pub fn set_recovery_instance_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_recovery_instance_ids(input);
        self
    }
    /// <p>The IDs of the Recovery Instances that should be terminated.</p>
    pub fn get_recovery_instance_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_recovery_instance_ids()
    }
}
