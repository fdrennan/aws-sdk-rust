// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[doc(inline)]
                pub use aws_smithy_client::Builder;
#[derive(Debug)]
            pub(crate) struct Handle {
                pub(crate) client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>,
                pub(crate) conf: crate::Config,
            }

            /// Client for Amazon Forecast Query Service
                    ///
                    /// Client for invoking operations on Amazon Forecast Query Service. Each operation on Amazon Forecast Query Service is a method on this
                    /// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
                        /// # Examples
                        /// **Constructing a client and invoking an operation**
                        /// ```rust,no_run
                        /// # async fn docs() {
                        ///     // create a shared configuration. This can be used & shared between multiple service clients.
                        ///     let shared_config = aws_config::load_from_env().await;
                        ///     let client = aws_sdk_forecastquery::Client::new(&shared_config);
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
                        /// let config = aws_sdk_forecastquery::config::Builder::from(&shared_config)
                        ///   .retry_config(RetryConfig::disabled())
                        ///   .build();
                        /// let client = aws_sdk_forecastquery::Client::from_conf(config);
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
    /// Constructs a fluent builder for the [`QueryForecast`](crate::client::fluent_builders::QueryForecast) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`forecast_arn(impl Into<String>)`](crate::client::fluent_builders::QueryForecast::forecast_arn) / [`set_forecast_arn(Option<String>)`](crate::client::fluent_builders::QueryForecast::set_forecast_arn): <p>The Amazon Resource Name (ARN) of the forecast to query.</p>
    ///   - [`start_date(impl Into<String>)`](crate::client::fluent_builders::QueryForecast::start_date) / [`set_start_date(Option<String>)`](crate::client::fluent_builders::QueryForecast::set_start_date): <p>The start date for the forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss (ISO 8601 format). For example, 2015-01-01T08:00:00.</p>
    ///   - [`end_date(impl Into<String>)`](crate::client::fluent_builders::QueryForecast::end_date) / [`set_end_date(Option<String>)`](crate::client::fluent_builders::QueryForecast::set_end_date): <p>The end date for the forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss (ISO 8601 format). For example, 2015-01-01T20:00:00. </p>
    ///   - [`filters(HashMap<String, String>)`](crate::client::fluent_builders::QueryForecast::filters) / [`set_filters(Option<HashMap<String, String>>)`](crate::client::fluent_builders::QueryForecast::set_filters): <p>The filtering criteria to apply when retrieving the forecast. For example, to get the forecast for <code>client_21</code> in the electricity usage dataset, specify the following:</p>  <p> <code>{"item_id" : "client_21"}</code> </p>  <p>To get the full forecast, use the <a href="https://docs.aws.amazon.com/en_us/forecast/latest/dg/API_CreateForecastExportJob.html">CreateForecastExportJob</a> operation.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::QueryForecast::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::QueryForecast::set_next_token): <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
                        /// - On success, responds with [`QueryForecastOutput`](crate::output::QueryForecastOutput) with field(s):
                        ///   - [`forecast(Option<Forecast>)`](crate::output::QueryForecastOutput::forecast): <p>The forecast.</p>
                        /// - On failure, responds with [`SdkError<QueryForecastError>`](crate::error::QueryForecastError)
    pub fn query_forecast(&self) -> crate::client::fluent_builders::QueryForecast {
                            crate::client::fluent_builders::QueryForecast::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`QueryWhatIfForecast`](crate::client::fluent_builders::QueryWhatIfForecast) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`what_if_forecast_arn(impl Into<String>)`](crate::client::fluent_builders::QueryWhatIfForecast::what_if_forecast_arn) / [`set_what_if_forecast_arn(Option<String>)`](crate::client::fluent_builders::QueryWhatIfForecast::set_what_if_forecast_arn): <p>The Amazon Resource Name (ARN) of the what-if forecast to query.</p>
    ///   - [`start_date(impl Into<String>)`](crate::client::fluent_builders::QueryWhatIfForecast::start_date) / [`set_start_date(Option<String>)`](crate::client::fluent_builders::QueryWhatIfForecast::set_start_date): <p>The start date for the what-if forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss (ISO 8601 format). For example, 2015-01-01T08:00:00.</p>
    ///   - [`end_date(impl Into<String>)`](crate::client::fluent_builders::QueryWhatIfForecast::end_date) / [`set_end_date(Option<String>)`](crate::client::fluent_builders::QueryWhatIfForecast::set_end_date): <p>The end date for the what-if forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss (ISO 8601 format). For example, 2015-01-01T20:00:00. </p>
    ///   - [`filters(HashMap<String, String>)`](crate::client::fluent_builders::QueryWhatIfForecast::filters) / [`set_filters(Option<HashMap<String, String>>)`](crate::client::fluent_builders::QueryWhatIfForecast::set_filters): <p>The filtering criteria to apply when retrieving the forecast. For example, to get the forecast for <code>client_21</code> in the electricity usage dataset, specify the following:</p>  <p> <code>{"item_id" : "client_21"}</code> </p>  <p>To get the full what-if forecast, use the <a href="https://docs.aws.amazon.com/en_us/forecast/latest/dg/API_CreateWhatIfForecastExport.html">CreateForecastExportJob</a> operation.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::QueryWhatIfForecast::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::QueryWhatIfForecast::set_next_token): <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
                        /// - On success, responds with [`QueryWhatIfForecastOutput`](crate::output::QueryWhatIfForecastOutput) with field(s):
                        ///   - [`forecast(Option<Forecast>)`](crate::output::QueryWhatIfForecastOutput::forecast): <p>Provides information about a forecast. Returned as part of the <code>QueryForecast</code> response.</p>
                        /// - On failure, responds with [`SdkError<QueryWhatIfForecastError>`](crate::error::QueryWhatIfForecastError)
    pub fn query_what_if_forecast(&self) -> crate::client::fluent_builders::QueryWhatIfForecast {
                            crate::client::fluent_builders::QueryWhatIfForecast::new(self.handle.clone())
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

