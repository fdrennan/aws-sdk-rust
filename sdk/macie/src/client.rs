// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[doc(inline)]
                pub use aws_smithy_client::Builder;
#[derive(Debug)]
            pub(crate) struct Handle {
                pub(crate) client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>,
                pub(crate) conf: crate::Config,
            }

            /// Client for Amazon Macie
                    ///
                    /// Client for invoking operations on Amazon Macie. Each operation on Amazon Macie is a method on this
                    /// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
                        /// # Examples
                        /// **Constructing a client and invoking an operation**
                        /// ```rust,no_run
                        /// # async fn docs() {
                        ///     // create a shared configuration. This can be used & shared between multiple service clients.
                        ///     let shared_config = aws_config::load_from_env().await;
                        ///     let client = aws_sdk_macie::Client::new(&shared_config);
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
                        /// let config = aws_sdk_macie::config::Builder::from(&shared_config)
                        ///   .retry_config(RetryConfig::disabled())
                        ///   .build();
                        /// let client = aws_sdk_macie::Client::from_conf(config);
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
    /// Constructs a fluent builder for the [`AssociateMemberAccount`](crate::client::fluent_builders::AssociateMemberAccount) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`member_account_id(impl Into<String>)`](crate::client::fluent_builders::AssociateMemberAccount::member_account_id) / [`set_member_account_id(Option<String>)`](crate::client::fluent_builders::AssociateMemberAccount::set_member_account_id): <p>(Discontinued) The ID of the Amazon Web Services account that you want to associate with Amazon Macie Classic as a member account.</p>
                        /// - On success, responds with [`AssociateMemberAccountOutput`](crate::output::AssociateMemberAccountOutput)
                        
                        /// - On failure, responds with [`SdkError<AssociateMemberAccountError>`](crate::error::AssociateMemberAccountError)
    pub fn associate_member_account(&self) -> crate::client::fluent_builders::AssociateMemberAccount {
                            crate::client::fluent_builders::AssociateMemberAccount::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`AssociateS3Resources`](crate::client::fluent_builders::AssociateS3Resources) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`member_account_id(impl Into<String>)`](crate::client::fluent_builders::AssociateS3Resources::member_account_id) / [`set_member_account_id(Option<String>)`](crate::client::fluent_builders::AssociateS3Resources::set_member_account_id): <p>(Discontinued) The ID of the Amazon Macie Classic member account whose resources you want to associate with Macie Classic.</p>
    ///   - [`s3_resources(Vec<S3ResourceClassification>)`](crate::client::fluent_builders::AssociateS3Resources::s3_resources) / [`set_s3_resources(Option<Vec<S3ResourceClassification>>)`](crate::client::fluent_builders::AssociateS3Resources::set_s3_resources): <p>(Discontinued) The S3 resources that you want to associate with Amazon Macie Classic for monitoring and data classification.</p>
                        /// - On success, responds with [`AssociateS3ResourcesOutput`](crate::output::AssociateS3ResourcesOutput) with field(s):
                        ///   - [`failed_s3_resources(Option<Vec<FailedS3Resource>>)`](crate::output::AssociateS3ResourcesOutput::failed_s3_resources): <p>(Discontinued) S3 resources that couldn't be associated with Amazon Macie Classic. An error code and an error message are provided for each failed item.</p>
                        /// - On failure, responds with [`SdkError<AssociateS3ResourcesError>`](crate::error::AssociateS3ResourcesError)
    pub fn associate_s3_resources(&self) -> crate::client::fluent_builders::AssociateS3Resources {
                            crate::client::fluent_builders::AssociateS3Resources::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`DisassociateMemberAccount`](crate::client::fluent_builders::DisassociateMemberAccount) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`member_account_id(impl Into<String>)`](crate::client::fluent_builders::DisassociateMemberAccount::member_account_id) / [`set_member_account_id(Option<String>)`](crate::client::fluent_builders::DisassociateMemberAccount::set_member_account_id): <p>(Discontinued) The ID of the member account that you want to remove from Amazon Macie Classic.</p>
                        /// - On success, responds with [`DisassociateMemberAccountOutput`](crate::output::DisassociateMemberAccountOutput)
                        
                        /// - On failure, responds with [`SdkError<DisassociateMemberAccountError>`](crate::error::DisassociateMemberAccountError)
    pub fn disassociate_member_account(&self) -> crate::client::fluent_builders::DisassociateMemberAccount {
                            crate::client::fluent_builders::DisassociateMemberAccount::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`DisassociateS3Resources`](crate::client::fluent_builders::DisassociateS3Resources) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`member_account_id(impl Into<String>)`](crate::client::fluent_builders::DisassociateS3Resources::member_account_id) / [`set_member_account_id(Option<String>)`](crate::client::fluent_builders::DisassociateS3Resources::set_member_account_id): <p>(Discontinued) The ID of the Amazon Macie Classic member account whose resources you want to remove from being monitored by Macie Classic.</p>
    ///   - [`associated_s3_resources(Vec<S3Resource>)`](crate::client::fluent_builders::DisassociateS3Resources::associated_s3_resources) / [`set_associated_s3_resources(Option<Vec<S3Resource>>)`](crate::client::fluent_builders::DisassociateS3Resources::set_associated_s3_resources): <p>(Discontinued) The S3 resources (buckets or prefixes) that you want to remove from being monitored and classified by Amazon Macie Classic.</p>
                        /// - On success, responds with [`DisassociateS3ResourcesOutput`](crate::output::DisassociateS3ResourcesOutput) with field(s):
                        ///   - [`failed_s3_resources(Option<Vec<FailedS3Resource>>)`](crate::output::DisassociateS3ResourcesOutput::failed_s3_resources): <p>(Discontinued) S3 resources that couldn't be removed from being monitored and classified by Amazon Macie Classic. An error code and an error message are provided for each failed item. </p>
                        /// - On failure, responds with [`SdkError<DisassociateS3ResourcesError>`](crate::error::DisassociateS3ResourcesError)
    pub fn disassociate_s3_resources(&self) -> crate::client::fluent_builders::DisassociateS3Resources {
                            crate::client::fluent_builders::DisassociateS3Resources::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`ListMemberAccounts`](crate::client::fluent_builders::ListMemberAccounts) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListMemberAccounts::into_paginator).
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListMemberAccounts::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListMemberAccounts::set_next_token): <p>(Discontinued) Use this parameter when paginating results. Set the value of this parameter to null on your first call to the <code>ListMemberAccounts</code> action. Subsequent calls to the action fill <code>nextToken</code> in the request with the value of <code>nextToken</code> from the previous response to continue listing data.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListMemberAccounts::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListMemberAccounts::set_max_results): <p>(Discontinued) Use this parameter to indicate the maximum number of items that you want in the response. The default value is 250.</p>
                        /// - On success, responds with [`ListMemberAccountsOutput`](crate::output::ListMemberAccountsOutput) with field(s):
                        ///   - [`member_accounts(Option<Vec<MemberAccount>>)`](crate::output::ListMemberAccountsOutput::member_accounts): <p>(Discontinued) A list of the Amazon Macie Classic member accounts returned by the action. The current Macie Classic administrator account is also included in this list.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListMemberAccountsOutput::next_token): <p>(Discontinued) When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <code>nextToken</code> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null. </p>
                        /// - On failure, responds with [`SdkError<ListMemberAccountsError>`](crate::error::ListMemberAccountsError)
    pub fn list_member_accounts(&self) -> crate::client::fluent_builders::ListMemberAccounts {
                            crate::client::fluent_builders::ListMemberAccounts::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`ListS3Resources`](crate::client::fluent_builders::ListS3Resources) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListS3Resources::into_paginator).
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`member_account_id(impl Into<String>)`](crate::client::fluent_builders::ListS3Resources::member_account_id) / [`set_member_account_id(Option<String>)`](crate::client::fluent_builders::ListS3Resources::set_member_account_id): <p>(Discontinued) The Amazon Macie Classic member account ID whose associated S3 resources you want to list. </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListS3Resources::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListS3Resources::set_next_token): <p>(Discontinued) Use this parameter when paginating results. Set its value to null on your first call to the <code>ListS3Resources</code> action. Subsequent calls to the action fill <code>nextToken</code> in the request with the value of <code>nextToken</code> from the previous response to continue listing data. </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListS3Resources::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListS3Resources::set_max_results): <p>(Discontinued) Use this parameter to indicate the maximum number of items that you want in the response. The default value is 250. </p>
                        /// - On success, responds with [`ListS3ResourcesOutput`](crate::output::ListS3ResourcesOutput) with field(s):
                        ///   - [`s3_resources(Option<Vec<S3ResourceClassification>>)`](crate::output::ListS3ResourcesOutput::s3_resources): <p>(Discontinued) A list of the associated S3 resources returned by the action.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListS3ResourcesOutput::next_token): <p>(Discontinued) When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <code>nextToken</code> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null. </p>
                        /// - On failure, responds with [`SdkError<ListS3ResourcesError>`](crate::error::ListS3ResourcesError)
    pub fn list_s3_resources(&self) -> crate::client::fluent_builders::ListS3Resources {
                            crate::client::fluent_builders::ListS3Resources::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`UpdateS3Resources`](crate::client::fluent_builders::UpdateS3Resources) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`member_account_id(impl Into<String>)`](crate::client::fluent_builders::UpdateS3Resources::member_account_id) / [`set_member_account_id(Option<String>)`](crate::client::fluent_builders::UpdateS3Resources::set_member_account_id): <p>(Discontinued) The Amazon Web Services account ID of the Amazon Macie Classic member account whose S3 resources' classification types you want to update.</p>
    ///   - [`s3_resources_update(Vec<S3ResourceClassificationUpdate>)`](crate::client::fluent_builders::UpdateS3Resources::s3_resources_update) / [`set_s3_resources_update(Option<Vec<S3ResourceClassificationUpdate>>)`](crate::client::fluent_builders::UpdateS3Resources::set_s3_resources_update): <p>(Discontinued) The S3 resources whose classification types you want to update.</p>
                        /// - On success, responds with [`UpdateS3ResourcesOutput`](crate::output::UpdateS3ResourcesOutput) with field(s):
                        ///   - [`failed_s3_resources(Option<Vec<FailedS3Resource>>)`](crate::output::UpdateS3ResourcesOutput::failed_s3_resources): <p>(Discontinued) The S3 resources whose classification types can't be updated. An error code and an error message are provided for each failed item.</p>
                        /// - On failure, responds with [`SdkError<UpdateS3ResourcesError>`](crate::error::UpdateS3ResourcesError)
    pub fn update_s3_resources(&self) -> crate::client::fluent_builders::UpdateS3Resources {
                            crate::client::fluent_builders::UpdateS3Resources::new(self.handle.clone())
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

