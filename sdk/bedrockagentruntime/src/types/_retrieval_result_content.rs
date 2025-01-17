// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Content of a retrieval result.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RetrievalResultContent {
    /// Content of a retrieval result in text
    pub text: ::std::string::String,
}
impl RetrievalResultContent {
    /// Content of a retrieval result in text
    pub fn text(&self) -> &str {
        use std::ops::Deref;
        self.text.deref()
    }
}
impl RetrievalResultContent {
    /// Creates a new builder-style object to manufacture [`RetrievalResultContent`](crate::types::RetrievalResultContent).
    pub fn builder() -> crate::types::builders::RetrievalResultContentBuilder {
        crate::types::builders::RetrievalResultContentBuilder::default()
    }
}

/// A builder for [`RetrievalResultContent`](crate::types::RetrievalResultContent).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RetrievalResultContentBuilder {
    pub(crate) text: ::std::option::Option<::std::string::String>,
}
impl RetrievalResultContentBuilder {
    /// Content of a retrieval result in text
    /// This field is required.
    pub fn text(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.text = ::std::option::Option::Some(input.into());
        self
    }
    /// Content of a retrieval result in text
    pub fn set_text(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.text = input;
        self
    }
    /// Content of a retrieval result in text
    pub fn get_text(&self) -> &::std::option::Option<::std::string::String> {
        &self.text
    }
    /// Consumes the builder and constructs a [`RetrievalResultContent`](crate::types::RetrievalResultContent).
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](crate::types::builders::RetrievalResultContentBuilder::text)
    pub fn build(self) -> ::std::result::Result<crate::types::RetrievalResultContent, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::RetrievalResultContent {
            text: self.text.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "text",
                    "text was not specified but it is required when building RetrievalResultContent",
                )
            })?,
        })
    }
}
