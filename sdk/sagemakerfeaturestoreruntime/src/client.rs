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

/// Client for Amazon SageMaker Feature Store Runtime
///
/// Client for invoking operations on Amazon SageMaker Feature Store Runtime. Each operation on Amazon SageMaker Feature Store Runtime is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_sagemakerfeaturestoreruntime::Client::new(&shared_config);
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
///     let config = aws_sdk_sagemakerfeaturestoreruntime::config::Builder::from(&shared_config)
///         .retry_config(RetryConfig::disabled())
///         .build();
///     let client = aws_sdk_sagemakerfeaturestoreruntime::Client::from_conf(config);
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
    /// Constructs a fluent builder for the `BatchGetRecord` operation.
    ///
    /// See [`BatchGetRecord`](crate::client::fluent_builders::BatchGetRecord) for more information about the
    /// operation and its arguments.
    pub fn batch_get_record(&self) -> fluent_builders::BatchGetRecord<C, M, R> {
        fluent_builders::BatchGetRecord::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `DeleteRecord` operation.
    ///
    /// See [`DeleteRecord`](crate::client::fluent_builders::DeleteRecord) for more information about the
    /// operation and its arguments.
    pub fn delete_record(&self) -> fluent_builders::DeleteRecord<C, M, R> {
        fluent_builders::DeleteRecord::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `GetRecord` operation.
    ///
    /// See [`GetRecord`](crate::client::fluent_builders::GetRecord) for more information about the
    /// operation and its arguments.
    pub fn get_record(&self) -> fluent_builders::GetRecord<C, M, R> {
        fluent_builders::GetRecord::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the `PutRecord` operation.
    ///
    /// See [`PutRecord`](crate::client::fluent_builders::PutRecord) for more information about the
    /// operation and its arguments.
    pub fn put_record(&self) -> fluent_builders::PutRecord<C, M, R> {
        fluent_builders::PutRecord::new(self.handle.clone())
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
    /// Fluent builder constructing a request to `BatchGetRecord`.
    ///
    /// <p>Retrieves a batch of <code>Records</code> from a <code>FeatureGroup</code>.</p>
    #[derive(std::fmt::Debug)]
    pub struct BatchGetRecord<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::batch_get_record_input::Builder,
    }
    impl<C, M, R> BatchGetRecord<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `BatchGetRecord`.
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
            crate::output::BatchGetRecordOutput,
            aws_smithy_http::result::SdkError<crate::error::BatchGetRecordError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::BatchGetRecordInputOperationOutputAlias,
                crate::output::BatchGetRecordOutput,
                crate::error::BatchGetRecordError,
                crate::input::BatchGetRecordInputOperationRetryAlias,
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
        /// Appends an item to `Identifiers`.
        ///
        /// To override the contents of this collection use [`set_identifiers`](Self::set_identifiers).
        ///
        /// <p>A list of <code>FeatureGroup</code> names, with their corresponding <code>RecordIdentifier</code> value, and Feature name
        /// that have been requested to be retrieved in batch.</p>
        pub fn identifiers(
            mut self,
            inp: impl Into<crate::model::BatchGetRecordIdentifier>,
        ) -> Self {
            self.inner = self.inner.identifiers(inp);
            self
        }
        /// <p>A list of <code>FeatureGroup</code> names, with their corresponding <code>RecordIdentifier</code> value, and Feature name
        /// that have been requested to be retrieved in batch.</p>
        pub fn set_identifiers(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::BatchGetRecordIdentifier>>,
        ) -> Self {
            self.inner = self.inner.set_identifiers(input);
            self
        }
    }
    /// Fluent builder constructing a request to `DeleteRecord`.
    ///
    /// <p>Deletes a <code>Record</code> from a <code>FeatureGroup</code>. A new record will show
    /// up in the <code>OfflineStore</code> when the <code>DeleteRecord</code> API is called. This
    /// record will have a value of <code>True</code> in the <code>is_deleted</code> column.</p>
    #[derive(std::fmt::Debug)]
    pub struct DeleteRecord<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::delete_record_input::Builder,
    }
    impl<C, M, R> DeleteRecord<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `DeleteRecord`.
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
            crate::output::DeleteRecordOutput,
            aws_smithy_http::result::SdkError<crate::error::DeleteRecordError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DeleteRecordInputOperationOutputAlias,
                crate::output::DeleteRecordOutput,
                crate::error::DeleteRecordError,
                crate::input::DeleteRecordInputOperationRetryAlias,
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
        /// <p>The name of the feature group to delete the record from. </p>
        pub fn feature_group_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.feature_group_name(inp);
            self
        }
        /// <p>The name of the feature group to delete the record from. </p>
        pub fn set_feature_group_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_feature_group_name(input);
            self
        }
        /// <p>The value for the <code>RecordIdentifier</code> that uniquely identifies the record, in
        /// string format. </p>
        pub fn record_identifier_value_as_string(
            mut self,
            inp: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.record_identifier_value_as_string(inp);
            self
        }
        /// <p>The value for the <code>RecordIdentifier</code> that uniquely identifies the record, in
        /// string format. </p>
        pub fn set_record_identifier_value_as_string(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_record_identifier_value_as_string(input);
            self
        }
        /// <p>Timestamp indicating when the deletion event occurred. <code>EventTime</code> can be
        /// used to query data at a certain point in time.</p>
        pub fn event_time(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.event_time(inp);
            self
        }
        /// <p>Timestamp indicating when the deletion event occurred. <code>EventTime</code> can be
        /// used to query data at a certain point in time.</p>
        pub fn set_event_time(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_event_time(input);
            self
        }
    }
    /// Fluent builder constructing a request to `GetRecord`.
    ///
    /// <p>Use for <code>OnlineStore</code> serving from a <code>FeatureStore</code>. Only the
    /// latest records stored in the <code>OnlineStore</code> can be retrieved. If no Record with
    /// <code>RecordIdentifierValue</code> is found, then an empty result is returned. </p>
    #[derive(std::fmt::Debug)]
    pub struct GetRecord<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_record_input::Builder,
    }
    impl<C, M, R> GetRecord<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `GetRecord`.
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
            crate::output::GetRecordOutput,
            aws_smithy_http::result::SdkError<crate::error::GetRecordError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetRecordInputOperationOutputAlias,
                crate::output::GetRecordOutput,
                crate::error::GetRecordError,
                crate::input::GetRecordInputOperationRetryAlias,
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
        /// <p>The name of the feature group in which you want to put the records.</p>
        pub fn feature_group_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.feature_group_name(inp);
            self
        }
        /// <p>The name of the feature group in which you want to put the records.</p>
        pub fn set_feature_group_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_feature_group_name(input);
            self
        }
        /// <p>The value that corresponds to <code>RecordIdentifier</code> type and uniquely identifies
        /// the record in the <code>FeatureGroup</code>. </p>
        pub fn record_identifier_value_as_string(
            mut self,
            inp: impl Into<std::string::String>,
        ) -> Self {
            self.inner = self.inner.record_identifier_value_as_string(inp);
            self
        }
        /// <p>The value that corresponds to <code>RecordIdentifier</code> type and uniquely identifies
        /// the record in the <code>FeatureGroup</code>. </p>
        pub fn set_record_identifier_value_as_string(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_record_identifier_value_as_string(input);
            self
        }
        /// Appends an item to `FeatureNames`.
        ///
        /// To override the contents of this collection use [`set_feature_names`](Self::set_feature_names).
        ///
        /// <p>List of names of Features to be retrieved. If not specified, the latest value for all
        /// the Features are returned.</p>
        pub fn feature_names(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.feature_names(inp);
            self
        }
        /// <p>List of names of Features to be retrieved. If not specified, the latest value for all
        /// the Features are returned.</p>
        pub fn set_feature_names(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.inner = self.inner.set_feature_names(input);
            self
        }
    }
    /// Fluent builder constructing a request to `PutRecord`.
    ///
    /// <p>Used for data ingestion into the <code>FeatureStore</code>. The <code>PutRecord</code>
    /// API writes to both the <code>OnlineStore</code> and <code>OfflineStore</code>. If the
    /// record is the latest record for the <code>recordIdentifier</code>, the record is written to
    /// both the <code>OnlineStore</code> and <code>OfflineStore</code>. If the record is a
    /// historic record, it is written only to the <code>OfflineStore</code>.</p>
    #[derive(std::fmt::Debug)]
    pub struct PutRecord<
        C = aws_smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = aws_smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::put_record_input::Builder,
    }
    impl<C, M, R> PutRecord<C, M, R>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
        M: aws_smithy_client::bounds::SmithyMiddleware<C>,
        R: aws_smithy_client::retry::NewRequestPolicy,
    {
        /// Creates a new `PutRecord`.
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
            crate::output::PutRecordOutput,
            aws_smithy_http::result::SdkError<crate::error::PutRecordError>,
        >
        where
            R::Policy: aws_smithy_client::bounds::SmithyRetryPolicy<
                crate::input::PutRecordInputOperationOutputAlias,
                crate::output::PutRecordOutput,
                crate::error::PutRecordError,
                crate::input::PutRecordInputOperationRetryAlias,
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
        /// <p>The name of the feature group that you want to insert the record into.</p>
        pub fn feature_group_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.feature_group_name(inp);
            self
        }
        /// <p>The name of the feature group that you want to insert the record into.</p>
        pub fn set_feature_group_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_feature_group_name(input);
            self
        }
        /// Appends an item to `Record`.
        ///
        /// To override the contents of this collection use [`set_record`](Self::set_record).
        ///
        /// <p>List of FeatureValues to be inserted. This will be a full over-write. If you only want
        /// to update few of the feature values, do the following:</p>
        /// <ul>
        /// <li>
        /// <p>Use <code>GetRecord</code> to retrieve the latest record.</p>
        /// </li>
        /// <li>
        /// <p>Update the record returned from <code>GetRecord</code>. </p>
        /// </li>
        /// <li>
        /// <p>Use <code>PutRecord</code> to update feature values.</p>
        /// </li>
        /// </ul>
        pub fn record(mut self, inp: impl Into<crate::model::FeatureValue>) -> Self {
            self.inner = self.inner.record(inp);
            self
        }
        /// <p>List of FeatureValues to be inserted. This will be a full over-write. If you only want
        /// to update few of the feature values, do the following:</p>
        /// <ul>
        /// <li>
        /// <p>Use <code>GetRecord</code> to retrieve the latest record.</p>
        /// </li>
        /// <li>
        /// <p>Update the record returned from <code>GetRecord</code>. </p>
        /// </li>
        /// <li>
        /// <p>Use <code>PutRecord</code> to update feature values.</p>
        /// </li>
        /// </ul>
        pub fn set_record(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::FeatureValue>>,
        ) -> Self {
            self.inner = self.inner.set_record(input);
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
