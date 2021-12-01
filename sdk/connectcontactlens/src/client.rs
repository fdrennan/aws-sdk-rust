// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = aws_smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    client: aws_smithy_client::Client<C, M, R>,
    conf: crate::Config,
}

/// Client for Amazon Connect Contact Lens
///
/// Client for invoking operations on Amazon Connect Contact Lens. Each operation on Amazon Connect Contact Lens is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_connectcontactlens::Client::new(&shared_config);
///     // invoke an operation
///     /* let rsp = client
///         .<operationname>().
///         .<param>("some value")
///         .send().await; */
/// # }
/// ```
/// **Constructing a client with custom configuration**
/// ```rust,no_run
/// use aws_config::RetryConfig;
/// # async fn docs() {
///     let shared_config = aws_config::load_from_env().await;
///     let config = aws_sdk_connectcontactlens::config::Builder::from(&shared_config)
///         .retry_config(RetryConfig::disabled())
///         .build();
///     let client = aws_sdk_connectcontactlens::Client::from_conf(config);
/// # }
#[derive(std::fmt::Debug)]
pub struct Client<
    C = aws_smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = aws_smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use aws_smithy_client::Builder;

impl<C, M, R> From<aws_smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: aws_smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    /// Creates a client with the given service configuration.
    pub fn with_config(client: aws_smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: aws_smithy_client::bounds::SmithyConnector,
    M: aws_smithy_client::bounds::SmithyMiddleware<C>,
    R: aws_smithy_client::retry::NewRequestPolicy,
{
    /// Constructs a fluent builder for the `ListRealtimeContactAnalysisSegments` operation.
    ///
    /// See [`ListRealtimeContactAnalysisSegments`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments) for more information about the
    /// operation and its arguments.
    pub fn list_realtime_contact_analysis_segments(
        &self,
    ) -> fluent_builders::ListRealtimeContactAnalysisSegments<C, M, R> {
        fluent_builders::ListRealtimeContactAnalysisSegments::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    //!
    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    //!
    /// Fluent builder constructing a request to `ListRealtimeContactAnalysisSegments`.
    ///
    /// <p>Provides a list of analysis segments for a real-time analysis session.</p>
    #[derive(std::fmt::Debug)]
    pub struct ListRealtimeContactAnalysisSegments<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_realtime_contact_analysis_segments_input::Builder,
    }
    impl<C, M, R> ListRealtimeContactAnalysisSegments<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `ListRealtimeContactAnalysisSegments`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
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
        ) -> std::result::Result<
            crate::output::ListRealtimeContactAnalysisSegmentsOutput,
            aws_smithy_http::result::SdkError<
                crate::error::ListRealtimeContactAnalysisSegmentsError,
            >,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListRealtimeContactAnalysisSegmentsInputOperationOutputAlias,
                crate::output::ListRealtimeContactAnalysisSegmentsOutput,
                crate::error::ListRealtimeContactAnalysisSegmentsError,
                crate::input::ListRealtimeContactAnalysisSegmentsInputOperationRetryAlias,
            >,
        {
            let input = self.inner.build().map_err(|err| {
                aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
            })?;
            let op = input
                .make_operation(&self.handle.conf)
                .await
                .map_err(|err| {
                    aws_smithy_http::result::SdkError::ConstructionFailure(err.into())
                })?;
            self.handle.client.call(op).await
        }
        /// <p>The identifier of the Amazon Connect instance.</p>
        pub fn instance_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.instance_id(inp);
            self
        }
        /// <p>The identifier of the Amazon Connect instance.</p>
        pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_instance_id(input);
            self
        }
        /// <p>The identifier of the contact.</p>
        pub fn contact_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.contact_id(inp);
            self
        }
        /// <p>The identifier of the contact.</p>
        pub fn set_contact_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_contact_id(input);
            self
        }
        /// <p>The maximimum number of results to return per page.</p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        /// <p>The maximimum number of results to return per page.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The token for the next set of results. Use the value returned in the previous
        /// response in the next request to retrieve the next set of results.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        /// <p>The token for the next set of results. Use the value returned in the previous
        /// response in the next request to retrieve the next set of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
    }
}
impl<C> Client<C, aws_hyper::AwsMiddleware, aws_smithy_client::retry::Standard> {
    /// Creates a client with the given service config and connector override.
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut client = aws_hyper::Client::new(conn)
            .with_retry_config(retry_config.into())
            .with_timeout_config(timeout_config);

        client.set_sleep_impl(sleep_impl);
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        aws_smithy_client::erase::DynConnector,
        aws_hyper::AwsMiddleware,
        aws_smithy_client::retry::Standard,
    >
{
    /// Creates a new client from a shared config.
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf.retry_config.as_ref().cloned().unwrap_or_default();
        let timeout_config = conf.timeout_config.as_ref().cloned().unwrap_or_default();
        let sleep_impl = conf.sleep_impl.clone();
        let mut client = aws_hyper::Client::https()
            .with_retry_config(retry_config.into())
            .with_timeout_config(timeout_config);

        client.set_sleep_impl(sleep_impl);
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
