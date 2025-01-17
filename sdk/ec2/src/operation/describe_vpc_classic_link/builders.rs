// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_vpc_classic_link::_describe_vpc_classic_link_output::DescribeVpcClassicLinkOutputBuilder;

pub use crate::operation::describe_vpc_classic_link::_describe_vpc_classic_link_input::DescribeVpcClassicLinkInputBuilder;

impl DescribeVpcClassicLinkInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_vpc_classic_link::DescribeVpcClassicLinkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_vpc_classic_link::DescribeVpcClassicLinkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_vpc_classic_link();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeVpcClassicLink`.
///
/// <note>
/// <p>This action is deprecated.</p>
/// </note>
/// <p>Describes the ClassicLink status of the specified VPCs.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeVpcClassicLinkFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_vpc_classic_link::builders::DescribeVpcClassicLinkInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_vpc_classic_link::DescribeVpcClassicLinkOutput,
        crate::operation::describe_vpc_classic_link::DescribeVpcClassicLinkError,
    > for DescribeVpcClassicLinkFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_vpc_classic_link::DescribeVpcClassicLinkOutput,
            crate::operation::describe_vpc_classic_link::DescribeVpcClassicLinkError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeVpcClassicLinkFluentBuilder {
    /// Creates a new `DescribeVpcClassicLink`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeVpcClassicLink as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_vpc_classic_link::builders::DescribeVpcClassicLinkInputBuilder {
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
        crate::operation::describe_vpc_classic_link::DescribeVpcClassicLinkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_vpc_classic_link::DescribeVpcClassicLinkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_vpc_classic_link::DescribeVpcClassicLink::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_vpc_classic_link::DescribeVpcClassicLink::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_vpc_classic_link::DescribeVpcClassicLinkOutput,
        crate::operation::describe_vpc_classic_link::DescribeVpcClassicLinkError,
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
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters.</p>
    /// <ul>
    /// <li>
    /// <p><code>is-classic-link-enabled</code> - Whether the VPC is enabled for ClassicLink (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p></li>
    /// <li>
    /// <p><code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p></li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li>
    /// <p><code>is-classic-link-enabled</code> - Whether the VPC is enabled for ClassicLink (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p></li>
    /// <li>
    /// <p><code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p></li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li>
    /// <p><code>is-classic-link-enabled</code> - Whether the VPC is enabled for ClassicLink (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p></li>
    /// <li>
    /// <p><code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p></li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filters()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// Appends an item to `VpcIds`.
    ///
    /// To override the contents of this collection use [`set_vpc_ids`](Self::set_vpc_ids).
    ///
    /// <p>The VPCs for which you want to describe the ClassicLink status.</p>
    pub fn vpc_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpc_ids(input.into());
        self
    }
    /// <p>The VPCs for which you want to describe the ClassicLink status.</p>
    pub fn set_vpc_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_vpc_ids(input);
        self
    }
    /// <p>The VPCs for which you want to describe the ClassicLink status.</p>
    pub fn get_vpc_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_vpc_ids()
    }
}
