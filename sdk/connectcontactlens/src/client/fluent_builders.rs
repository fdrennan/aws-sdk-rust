// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Fluent builder constructing a request to `ListRealtimeContactAnalysisSegments`.
            ///
/// <p>Provides a list of analysis segments for a real-time analysis session.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListRealtimeContactAnalysisSegments {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::input::list_realtime_contact_analysis_segments_input::Builder
            }
impl ListRealtimeContactAnalysisSegments  {
    /// Creates a new `ListRealtimeContactAnalysisSegments`.
                    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
                        Self { handle, inner: Default::default() }
                    }
    
                    /// Consume this builder, creating a customizable operation that can be modified before being
                    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::operation::customize::CustomizableOperation<crate::operation::ListRealtimeContactAnalysisSegments, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::error::ListRealtimeContactAnalysisSegmentsError>
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
                    pub async fn send(self) -> std::result::Result<crate::output::ListRealtimeContactAnalysisSegmentsOutput, aws_smithy_http::result::SdkError<crate::error::ListRealtimeContactAnalysisSegmentsError>>
                     {
                        let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                            .make_operation(&self.handle.conf)
                            .await
                            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                        self.handle.client.call(op).await
                    }
    /// Create a paginator for this request
                        ///
                        /// Paginators are used by calling [`send().await`](crate::paginator::ListRealtimeContactAnalysisSegmentsPaginator::send) which returns a `Stream`.
                        pub fn into_paginator(self) -> crate::paginator::ListRealtimeContactAnalysisSegmentsPaginator {
                            crate::paginator::ListRealtimeContactAnalysisSegmentsPaginator::new(self.handle, self.inner)
                        }
    /// <p>The identifier of the Amazon Connect instance.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The identifier of the contact.</p>
    pub fn contact_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.contact_id(input.into());
        self
    }
    /// <p>The identifier of the contact.</p>
    pub fn set_contact_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_contact_id(input);
        self
    }
    /// <p>The maximimum number of results to return per page.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximimum number of results to return per page.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}

