// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_schema_mapping::_create_schema_mapping_output::CreateSchemaMappingOutputBuilder;

pub use crate::operation::create_schema_mapping::_create_schema_mapping_input::CreateSchemaMappingInputBuilder;

impl CreateSchemaMappingInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_schema_mapping::CreateSchemaMappingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_schema_mapping::CreateSchemaMappingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_schema_mapping();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateSchemaMapping`.
///
/// <p>Creates a schema mapping, which defines the schema of the input customer records table. The <code>SchemaMapping</code> also provides Entity Resolution with some metadata about the table, such as the attribute types of the columns and which columns to match on.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateSchemaMappingFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_schema_mapping::builders::CreateSchemaMappingInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_schema_mapping::CreateSchemaMappingOutput,
        crate::operation::create_schema_mapping::CreateSchemaMappingError,
    > for CreateSchemaMappingFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_schema_mapping::CreateSchemaMappingOutput,
            crate::operation::create_schema_mapping::CreateSchemaMappingError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateSchemaMappingFluentBuilder {
    /// Creates a new `CreateSchemaMapping`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateSchemaMapping as a reference.
    pub fn as_input(&self) -> &crate::operation::create_schema_mapping::builders::CreateSchemaMappingInputBuilder {
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
        crate::operation::create_schema_mapping::CreateSchemaMappingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_schema_mapping::CreateSchemaMappingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_schema_mapping::CreateSchemaMapping::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_schema_mapping::CreateSchemaMapping::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_schema_mapping::CreateSchemaMappingOutput,
        crate::operation::create_schema_mapping::CreateSchemaMappingError,
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
    /// <p>The name of the schema. There can't be multiple <code>SchemaMappings</code> with the same name.</p>
    pub fn schema_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.schema_name(input.into());
        self
    }
    /// <p>The name of the schema. There can't be multiple <code>SchemaMappings</code> with the same name.</p>
    pub fn set_schema_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_schema_name(input);
        self
    }
    /// <p>The name of the schema. There can't be multiple <code>SchemaMappings</code> with the same name.</p>
    pub fn get_schema_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_schema_name()
    }
    /// <p>A description of the schema.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the schema.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description of the schema.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// Appends an item to `mappedInputFields`.
    ///
    /// To override the contents of this collection use [`set_mapped_input_fields`](Self::set_mapped_input_fields).
    ///
    /// <p>A list of <code>MappedInputFields</code>. Each <code>MappedInputField</code> corresponds to a column the source data table, and contains column name plus additional information that Entity Resolution uses for matching.</p>
    pub fn mapped_input_fields(mut self, input: crate::types::SchemaInputAttribute) -> Self {
        self.inner = self.inner.mapped_input_fields(input);
        self
    }
    /// <p>A list of <code>MappedInputFields</code>. Each <code>MappedInputField</code> corresponds to a column the source data table, and contains column name plus additional information that Entity Resolution uses for matching.</p>
    pub fn set_mapped_input_fields(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SchemaInputAttribute>>) -> Self {
        self.inner = self.inner.set_mapped_input_fields(input);
        self
    }
    /// <p>A list of <code>MappedInputFields</code>. Each <code>MappedInputField</code> corresponds to a column the source data table, and contains column name plus additional information that Entity Resolution uses for matching.</p>
    pub fn get_mapped_input_fields(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SchemaInputAttribute>> {
        self.inner.get_mapped_input_fields()
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags used to organize, track, or control access for this resource.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags used to organize, track, or control access for this resource.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags used to organize, track, or control access for this resource.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
