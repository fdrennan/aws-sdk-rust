// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[doc(inline)]
                pub use aws_smithy_client::Builder;
#[derive(Debug)]
            pub(crate) struct Handle {
                pub(crate) client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>,
                pub(crate) conf: crate::Config,
            }

            /// Client for Amazon EventBridge Pipes
                    ///
                    /// Client for invoking operations on Amazon EventBridge Pipes. Each operation on Amazon EventBridge Pipes is a method on this
                    /// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
                        /// # Examples
                        /// **Constructing a client and invoking an operation**
                        /// ```rust,no_run
                        /// # async fn docs() {
                        ///     // create a shared configuration. This can be used & shared between multiple service clients.
                        ///     let shared_config = aws_config::load_from_env().await;
                        ///     let client = aws_sdk_pipes::Client::new(&shared_config);
                        ///     // invoke an operation
                        ///     /* let rsp = client
                        ///         .<operation_name>().
                        ///         .<param>("some value")
                        ///         .send().await; */
                        /// # }
                        /// ```
                        /// **Constructing a client with custom configuration**
                        /// ```rust,no_run
                        /// use aws_config::retry::RetryConfig;
                        /// # async fn docs() {
                        /// let shared_config = aws_config::load_from_env().await;
                        /// let config = aws_sdk_pipes::config::Builder::from(&shared_config)
                        ///   .retry_config(RetryConfig::disabled())
                        ///   .build();
                        /// let client = aws_sdk_pipes::Client::from_conf(config);
                        /// # }
            #[derive(std::fmt::Debug)]
            pub struct Client {
                handle: std::sync::Arc<Handle>
            }

            impl std::clone::Clone for Client {
                fn clone(&self) -> Self {
                    Self { handle: self.handle.clone() }
                }
            }

            impl From<aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>> for Client {
                fn from(client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>) -> Self {
                    Self::with_config(client, crate::Config::builder().build())
                }
            }

            impl Client {
                /// Creates a client with the given service configuration.
                pub fn with_config(client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>, conf: crate::Config) -> Self {
                    Self {
                        handle: std::sync::Arc::new(Handle {
                            client,
                            conf,
                        })
                    }
                }

                /// Returns the client's configuration.
                pub fn conf(&self) -> &crate::Config {
                    &self.handle.conf
                }
            }
impl Client  {
    /// Constructs a fluent builder for the [`CreatePipe`](crate::client::fluent_builders::CreatePipe) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreatePipe::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreatePipe::set_name): <p>The name of the pipe.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreatePipe::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreatePipe::set_description): <p>A description of the pipe.</p>
    ///   - [`desired_state(RequestedPipeState)`](crate::client::fluent_builders::CreatePipe::desired_state) / [`set_desired_state(Option<RequestedPipeState>)`](crate::client::fluent_builders::CreatePipe::set_desired_state): <p>The state the pipe should be in.</p>
    ///   - [`source(impl Into<String>)`](crate::client::fluent_builders::CreatePipe::source) / [`set_source(Option<String>)`](crate::client::fluent_builders::CreatePipe::set_source): <p>The ARN of the source resource.</p>
    ///   - [`source_parameters(PipeSourceParameters)`](crate::client::fluent_builders::CreatePipe::source_parameters) / [`set_source_parameters(Option<PipeSourceParameters>)`](crate::client::fluent_builders::CreatePipe::set_source_parameters): <p>The parameters required to set up a source for your pipe.</p>
    ///   - [`enrichment(impl Into<String>)`](crate::client::fluent_builders::CreatePipe::enrichment) / [`set_enrichment(Option<String>)`](crate::client::fluent_builders::CreatePipe::set_enrichment): <p>The ARN of the enrichment resource.</p>
    ///   - [`enrichment_parameters(PipeEnrichmentParameters)`](crate::client::fluent_builders::CreatePipe::enrichment_parameters) / [`set_enrichment_parameters(Option<PipeEnrichmentParameters>)`](crate::client::fluent_builders::CreatePipe::set_enrichment_parameters): <p>The parameters required to set up enrichment on your pipe.</p>
    ///   - [`target(impl Into<String>)`](crate::client::fluent_builders::CreatePipe::target) / [`set_target(Option<String>)`](crate::client::fluent_builders::CreatePipe::set_target): <p>The ARN of the target resource.</p>
    ///   - [`target_parameters(PipeTargetParameters)`](crate::client::fluent_builders::CreatePipe::target_parameters) / [`set_target_parameters(Option<PipeTargetParameters>)`](crate::client::fluent_builders::CreatePipe::set_target_parameters): <p>The parameters required to set up a target for your pipe.</p>
    ///   - [`role_arn(impl Into<String>)`](crate::client::fluent_builders::CreatePipe::role_arn) / [`set_role_arn(Option<String>)`](crate::client::fluent_builders::CreatePipe::set_role_arn): <p>The ARN of the role that allows the pipe to send data to the target.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreatePipe::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreatePipe::set_tags): <p>The list of key-value pairs to associate with the pipe.</p>
                        /// - On success, responds with [`CreatePipeOutput`](crate::output::CreatePipeOutput) with field(s):
                        ///   - [`arn(Option<String>)`](crate::output::CreatePipeOutput::arn): <p>The ARN of the pipe.</p>
    ///   - [`name(Option<String>)`](crate::output::CreatePipeOutput::name): <p>The name of the pipe.</p>
    ///   - [`desired_state(Option<RequestedPipeState>)`](crate::output::CreatePipeOutput::desired_state): <p>The state the pipe should be in.</p>
    ///   - [`current_state(Option<PipeState>)`](crate::output::CreatePipeOutput::current_state): <p>The state the pipe is in.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::output::CreatePipeOutput::creation_time): <p>The time the pipe was created.</p>
    ///   - [`last_modified_time(Option<DateTime>)`](crate::output::CreatePipeOutput::last_modified_time): <p>When the pipe was last updated, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
                        /// - On failure, responds with [`SdkError<CreatePipeError>`](crate::error::CreatePipeError)
    pub fn create_pipe(&self) -> crate::client::fluent_builders::CreatePipe {
                            crate::client::fluent_builders::CreatePipe::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`DeletePipe`](crate::client::fluent_builders::DeletePipe) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::DeletePipe::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::DeletePipe::set_name): <p>The name of the pipe.</p>
                        /// - On success, responds with [`DeletePipeOutput`](crate::output::DeletePipeOutput) with field(s):
                        ///   - [`arn(Option<String>)`](crate::output::DeletePipeOutput::arn): <p>The ARN of the pipe.</p>
    ///   - [`name(Option<String>)`](crate::output::DeletePipeOutput::name): <p>The name of the pipe.</p>
    ///   - [`desired_state(Option<RequestedPipeStateDescribeResponse>)`](crate::output::DeletePipeOutput::desired_state): <p>The state the pipe should be in.</p>
    ///   - [`current_state(Option<PipeState>)`](crate::output::DeletePipeOutput::current_state): <p>The state the pipe is in.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::output::DeletePipeOutput::creation_time): <p>The time the pipe was created.</p>
    ///   - [`last_modified_time(Option<DateTime>)`](crate::output::DeletePipeOutput::last_modified_time): <p>When the pipe was last updated, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
                        /// - On failure, responds with [`SdkError<DeletePipeError>`](crate::error::DeletePipeError)
    pub fn delete_pipe(&self) -> crate::client::fluent_builders::DeletePipe {
                            crate::client::fluent_builders::DeletePipe::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`DescribePipe`](crate::client::fluent_builders::DescribePipe) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::DescribePipe::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::DescribePipe::set_name): <p>The name of the pipe.</p>
                        /// - On success, responds with [`DescribePipeOutput`](crate::output::DescribePipeOutput) with field(s):
                        ///   - [`arn(Option<String>)`](crate::output::DescribePipeOutput::arn): <p>The ARN of the pipe.</p>
    ///   - [`name(Option<String>)`](crate::output::DescribePipeOutput::name): <p>The name of the pipe.</p>
    ///   - [`description(Option<String>)`](crate::output::DescribePipeOutput::description): <p>A description of the pipe.</p>
    ///   - [`desired_state(Option<RequestedPipeStateDescribeResponse>)`](crate::output::DescribePipeOutput::desired_state): <p>The state the pipe should be in.</p>
    ///   - [`current_state(Option<PipeState>)`](crate::output::DescribePipeOutput::current_state): <p>The state the pipe is in.</p>
    ///   - [`state_reason(Option<String>)`](crate::output::DescribePipeOutput::state_reason): <p>The reason the pipe is in its current state.</p>
    ///   - [`source(Option<String>)`](crate::output::DescribePipeOutput::source): <p>The ARN of the source resource.</p>
    ///   - [`source_parameters(Option<PipeSourceParameters>)`](crate::output::DescribePipeOutput::source_parameters): <p>The parameters required to set up a source for your pipe.</p>
    ///   - [`enrichment(Option<String>)`](crate::output::DescribePipeOutput::enrichment): <p>The ARN of the enrichment resource.</p>
    ///   - [`enrichment_parameters(Option<PipeEnrichmentParameters>)`](crate::output::DescribePipeOutput::enrichment_parameters): <p>The parameters required to set up enrichment on your pipe.</p>
    ///   - [`target(Option<String>)`](crate::output::DescribePipeOutput::target): <p>The ARN of the target resource.</p>
    ///   - [`target_parameters(Option<PipeTargetParameters>)`](crate::output::DescribePipeOutput::target_parameters): <p>The parameters required to set up a target for your pipe.</p>
    ///   - [`role_arn(Option<String>)`](crate::output::DescribePipeOutput::role_arn): <p>The ARN of the role that allows the pipe to send data to the target.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::DescribePipeOutput::tags): <p>The list of key-value pairs to associate with the pipe.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::output::DescribePipeOutput::creation_time): <p>The time the pipe was created.</p>
    ///   - [`last_modified_time(Option<DateTime>)`](crate::output::DescribePipeOutput::last_modified_time): <p>When the pipe was last updated, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
                        /// - On failure, responds with [`SdkError<DescribePipeError>`](crate::error::DescribePipeError)
    pub fn describe_pipe(&self) -> crate::client::fluent_builders::DescribePipe {
                            crate::client::fluent_builders::DescribePipe::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`ListPipes`](crate::client::fluent_builders::ListPipes) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListPipes::into_paginator).
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`name_prefix(impl Into<String>)`](crate::client::fluent_builders::ListPipes::name_prefix) / [`set_name_prefix(Option<String>)`](crate::client::fluent_builders::ListPipes::set_name_prefix): <p>A value that will return a subset of the pipes associated with this account. For example, <code>"NamePrefix": "ABC"</code> will return all endpoints with "ABC" in the name.</p>
    ///   - [`desired_state(RequestedPipeState)`](crate::client::fluent_builders::ListPipes::desired_state) / [`set_desired_state(Option<RequestedPipeState>)`](crate::client::fluent_builders::ListPipes::set_desired_state): <p>The state the pipe should be in.</p>
    ///   - [`current_state(PipeState)`](crate::client::fluent_builders::ListPipes::current_state) / [`set_current_state(Option<PipeState>)`](crate::client::fluent_builders::ListPipes::set_current_state): <p>The state the pipe is in.</p>
    ///   - [`source_prefix(impl Into<String>)`](crate::client::fluent_builders::ListPipes::source_prefix) / [`set_source_prefix(Option<String>)`](crate::client::fluent_builders::ListPipes::set_source_prefix): <p>The prefix matching the pipe source.</p>
    ///   - [`target_prefix(impl Into<String>)`](crate::client::fluent_builders::ListPipes::target_prefix) / [`set_target_prefix(Option<String>)`](crate::client::fluent_builders::ListPipes::set_target_prefix): <p>The prefix matching the pipe target.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListPipes::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListPipes::set_next_token): <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an HTTP 400 InvalidToken error.</p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::ListPipes::limit) / [`set_limit(Option<i32>)`](crate::client::fluent_builders::ListPipes::set_limit): <p>The maximum number of pipes to include in the response.</p>
                        /// - On success, responds with [`ListPipesOutput`](crate::output::ListPipesOutput) with field(s):
                        ///   - [`pipes(Option<Vec<Pipe>>)`](crate::output::ListPipesOutput::pipes): <p>The pipes returned by the call.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListPipesOutput::next_token): <p>If <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. Each pagination token expires after 24 hours. Using an expired pagination token will return an HTTP 400 InvalidToken error.</p>
                        /// - On failure, responds with [`SdkError<ListPipesError>`](crate::error::ListPipesError)
    pub fn list_pipes(&self) -> crate::client::fluent_builders::ListPipes {
                            crate::client::fluent_builders::ListPipes::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`ListTagsForResource`](crate::client::fluent_builders::ListTagsForResource) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::ListTagsForResource::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::ListTagsForResource::set_resource_arn): <p>The ARN of the pipe for which you want to view tags.</p>
                        /// - On success, responds with [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput) with field(s):
                        ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::ListTagsForResourceOutput::tags): <p>The list of key-value pairs to associate with the pipe.</p>
                        /// - On failure, responds with [`SdkError<ListTagsForResourceError>`](crate::error::ListTagsForResourceError)
    pub fn list_tags_for_resource(&self) -> crate::client::fluent_builders::ListTagsForResource {
                            crate::client::fluent_builders::ListTagsForResource::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`StartPipe`](crate::client::fluent_builders::StartPipe) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::StartPipe::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::StartPipe::set_name): <p>The name of the pipe.</p>
                        /// - On success, responds with [`StartPipeOutput`](crate::output::StartPipeOutput) with field(s):
                        ///   - [`arn(Option<String>)`](crate::output::StartPipeOutput::arn): <p>The ARN of the pipe.</p>
    ///   - [`name(Option<String>)`](crate::output::StartPipeOutput::name): <p>The name of the pipe.</p>
    ///   - [`desired_state(Option<RequestedPipeState>)`](crate::output::StartPipeOutput::desired_state): <p>The state the pipe should be in.</p>
    ///   - [`current_state(Option<PipeState>)`](crate::output::StartPipeOutput::current_state): <p>The state the pipe is in.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::output::StartPipeOutput::creation_time): <p>The time the pipe was created.</p>
    ///   - [`last_modified_time(Option<DateTime>)`](crate::output::StartPipeOutput::last_modified_time): <p>When the pipe was last updated, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
                        /// - On failure, responds with [`SdkError<StartPipeError>`](crate::error::StartPipeError)
    pub fn start_pipe(&self) -> crate::client::fluent_builders::StartPipe {
                            crate::client::fluent_builders::StartPipe::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`StopPipe`](crate::client::fluent_builders::StopPipe) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::StopPipe::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::StopPipe::set_name): <p>The name of the pipe.</p>
                        /// - On success, responds with [`StopPipeOutput`](crate::output::StopPipeOutput) with field(s):
                        ///   - [`arn(Option<String>)`](crate::output::StopPipeOutput::arn): <p>The ARN of the pipe.</p>
    ///   - [`name(Option<String>)`](crate::output::StopPipeOutput::name): <p>The name of the pipe.</p>
    ///   - [`desired_state(Option<RequestedPipeState>)`](crate::output::StopPipeOutput::desired_state): <p>The state the pipe should be in.</p>
    ///   - [`current_state(Option<PipeState>)`](crate::output::StopPipeOutput::current_state): <p>The state the pipe is in.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::output::StopPipeOutput::creation_time): <p>The time the pipe was created.</p>
    ///   - [`last_modified_time(Option<DateTime>)`](crate::output::StopPipeOutput::last_modified_time): <p>When the pipe was last updated, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
                        /// - On failure, responds with [`SdkError<StopPipeError>`](crate::error::StopPipeError)
    pub fn stop_pipe(&self) -> crate::client::fluent_builders::StopPipe {
                            crate::client::fluent_builders::StopPipe::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`TagResource`](crate::client::fluent_builders::TagResource) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::TagResource::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::TagResource::set_resource_arn): <p>The ARN of the pipe.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::TagResource::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::TagResource::set_tags): <p>The list of key-value pairs associated with the pipe.</p>
                        /// - On success, responds with [`TagResourceOutput`](crate::output::TagResourceOutput)
                        
                        /// - On failure, responds with [`SdkError<TagResourceError>`](crate::error::TagResourceError)
    pub fn tag_resource(&self) -> crate::client::fluent_builders::TagResource {
                            crate::client::fluent_builders::TagResource::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`UntagResource`](crate::client::fluent_builders::UntagResource) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::UntagResource::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::UntagResource::set_resource_arn): <p>The ARN of the pipe.</p>
    ///   - [`tag_keys(Vec<String>)`](crate::client::fluent_builders::UntagResource::tag_keys) / [`set_tag_keys(Option<Vec<String>>)`](crate::client::fluent_builders::UntagResource::set_tag_keys): <p>The list of tag keys to remove from the pipe.</p>
                        /// - On success, responds with [`UntagResourceOutput`](crate::output::UntagResourceOutput)
                        
                        /// - On failure, responds with [`SdkError<UntagResourceError>`](crate::error::UntagResourceError)
    pub fn untag_resource(&self) -> crate::client::fluent_builders::UntagResource {
                            crate::client::fluent_builders::UntagResource::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`UpdatePipe`](crate::client::fluent_builders::UpdatePipe) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdatePipe::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdatePipe::set_name): <p>The name of the pipe.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdatePipe::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdatePipe::set_description): <p>A description of the pipe.</p>
    ///   - [`desired_state(RequestedPipeState)`](crate::client::fluent_builders::UpdatePipe::desired_state) / [`set_desired_state(Option<RequestedPipeState>)`](crate::client::fluent_builders::UpdatePipe::set_desired_state): <p>The state the pipe should be in.</p>
    ///   - [`source_parameters(UpdatePipeSourceParameters)`](crate::client::fluent_builders::UpdatePipe::source_parameters) / [`set_source_parameters(Option<UpdatePipeSourceParameters>)`](crate::client::fluent_builders::UpdatePipe::set_source_parameters): <p>The parameters required to set up a source for your pipe.</p>
    ///   - [`enrichment(impl Into<String>)`](crate::client::fluent_builders::UpdatePipe::enrichment) / [`set_enrichment(Option<String>)`](crate::client::fluent_builders::UpdatePipe::set_enrichment): <p>The ARN of the enrichment resource.</p>
    ///   - [`enrichment_parameters(PipeEnrichmentParameters)`](crate::client::fluent_builders::UpdatePipe::enrichment_parameters) / [`set_enrichment_parameters(Option<PipeEnrichmentParameters>)`](crate::client::fluent_builders::UpdatePipe::set_enrichment_parameters): <p>The parameters required to set up enrichment on your pipe.</p>
    ///   - [`target(impl Into<String>)`](crate::client::fluent_builders::UpdatePipe::target) / [`set_target(Option<String>)`](crate::client::fluent_builders::UpdatePipe::set_target): <p>The ARN of the target resource.</p>
    ///   - [`target_parameters(PipeTargetParameters)`](crate::client::fluent_builders::UpdatePipe::target_parameters) / [`set_target_parameters(Option<PipeTargetParameters>)`](crate::client::fluent_builders::UpdatePipe::set_target_parameters): <p>The parameters required to set up a target for your pipe.</p>
    ///   - [`role_arn(impl Into<String>)`](crate::client::fluent_builders::UpdatePipe::role_arn) / [`set_role_arn(Option<String>)`](crate::client::fluent_builders::UpdatePipe::set_role_arn): <p>The ARN of the role that allows the pipe to send data to the target.</p>
                        /// - On success, responds with [`UpdatePipeOutput`](crate::output::UpdatePipeOutput) with field(s):
                        ///   - [`arn(Option<String>)`](crate::output::UpdatePipeOutput::arn): <p>The ARN of the pipe.</p>
    ///   - [`name(Option<String>)`](crate::output::UpdatePipeOutput::name): <p>The name of the pipe.</p>
    ///   - [`desired_state(Option<RequestedPipeState>)`](crate::output::UpdatePipeOutput::desired_state): <p>The state the pipe should be in.</p>
    ///   - [`current_state(Option<PipeState>)`](crate::output::UpdatePipeOutput::current_state): <p>The state the pipe is in.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::output::UpdatePipeOutput::creation_time): <p>The time the pipe was created.</p>
    ///   - [`last_modified_time(Option<DateTime>)`](crate::output::UpdatePipeOutput::last_modified_time): <p>When the pipe was last updated, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
                        /// - On failure, responds with [`SdkError<UpdatePipeError>`](crate::error::UpdatePipeError)
    pub fn update_pipe(&self) -> crate::client::fluent_builders::UpdatePipe {
                            crate::client::fluent_builders::UpdatePipe::new(self.handle.clone())
                        }
}

impl Client {
    /// Creates a new client from an [SDK Config](aws_types::sdk_config::SdkConfig).
                    ///
                    /// # Panics
                    ///
                    /// - This method will panic if the `sdk_config` is missing an async sleep implementation. If you experience this panic, set
                    ///     the `sleep_impl` on the Config passed into this function to fix it.
                    /// - This method will panic if the `sdk_config` is missing an HTTP connector. If you experience this panic, set the
                    ///     `http_connector` on the Config passed into this function to fix it.
                    pub fn new(sdk_config: &aws_types::sdk_config::SdkConfig) -> Self {
                        Self::from_conf(sdk_config.into())
                    }
    
                    /// Creates a new client from the service [`Config`](crate::Config).
                    ///
                    /// # Panics
                    ///
                    /// - This method will panic if the `conf` is missing an async sleep implementation. If you experience this panic, set
                    ///     the `sleep_impl` on the Config passed into this function to fix it.
                    /// - This method will panic if the `conf` is missing an HTTP connector. If you experience this panic, set the
                    ///     `http_connector` on the Config passed into this function to fix it.
                    pub fn from_conf(conf: crate::Config) -> Self {
                        let retry_config = conf.retry_config().cloned().unwrap_or_else(aws_smithy_types::retry::RetryConfig::disabled);
                        let timeout_config = conf.timeout_config().cloned().unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
                        let sleep_impl = conf.sleep_impl();
                        if (retry_config.has_retry() || timeout_config.has_timeouts()) && sleep_impl.is_none() {
                            panic!("An async sleep implementation is required for retries or timeouts to work. \
                                    Set the `sleep_impl` on the Config passed into this function to fix this panic.");
                        }
    
                        let connector = conf.http_connector().and_then(|c| {
                            let timeout_config = conf
                                .timeout_config()
                                .cloned()
                                .unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
                            let connector_settings = aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(
                                &timeout_config,
                            );
                            c.connector(&connector_settings, conf.sleep_impl())
                        });
    
                        let builder = aws_smithy_client::Builder::new();
    
                        let builder = match connector {
                            // Use provided connector
                            Some(c) => builder.connector(c),
                            None =>{
                                #[cfg(any(feature = "rustls", feature = "native-tls"))]
                                {
                                    // Use default connector based on enabled features
                                    builder.dyn_https_connector(aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(&timeout_config))
                                }
                                #[cfg(not(any(feature = "rustls", feature = "native-tls")))]
                                {
                                    panic!("No HTTP connector was available. Enable the `rustls` or `native-tls` crate feature or set a connector to fix this.");
                                }
                            }
                        };
                        let mut builder = builder
                            .middleware(aws_smithy_client::erase::DynMiddleware::new(crate::middleware::DefaultMiddleware::new()))
                            .retry_config(retry_config.into())
                            .operation_timeout_config(timeout_config.into());
                        builder.set_sleep_impl(sleep_impl);
                        let client = builder.build();
    
                        Self { handle: std::sync::Arc::new(Handle { client, conf }) }
                    }
}

/// Utilities to ergonomically construct a request to the service.
/// 
/// Fluent builders are created through the [`Client`](crate::client::Client) by calling
/// one if its operation methods. After parameters are set using the builder methods,
/// the `send` method can be called to initiate the request.
pub mod fluent_builders;

