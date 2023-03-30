// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateContainer`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_container`](crate::client::fluent_builders::CreateContainer).
            ///
            /// `ParseStrictResponse` impl for `CreateContainer`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateContainer {
    _private: ()
}
impl CreateContainer {
    /// Creates a new builder-style object to manufacture [`CreateContainerInput`](crate::input::CreateContainerInput).
    pub fn builder() -> crate::input::create_container_input::Builder {
        crate::input::create_container_input::Builder::default()
    }
    /// Creates a new `CreateContainer` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateContainer {
                type Output = std::result::Result<crate::output::CreateContainerOutput, crate::error::CreateContainerError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_container_error(response)
                     } else {
                        crate::operation_deser::parse_create_container_response(response)
                     }
                }
            }

/// Operation shape for `DeleteContainer`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_container`](crate::client::fluent_builders::DeleteContainer).
            ///
            /// `ParseStrictResponse` impl for `DeleteContainer`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteContainer {
    _private: ()
}
impl DeleteContainer {
    /// Creates a new builder-style object to manufacture [`DeleteContainerInput`](crate::input::DeleteContainerInput).
    pub fn builder() -> crate::input::delete_container_input::Builder {
        crate::input::delete_container_input::Builder::default()
    }
    /// Creates a new `DeleteContainer` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteContainer {
                type Output = std::result::Result<crate::output::DeleteContainerOutput, crate::error::DeleteContainerError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_container_error(response)
                     } else {
                        crate::operation_deser::parse_delete_container_response(response)
                     }
                }
            }

/// Operation shape for `DeleteContainerPolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_container_policy`](crate::client::fluent_builders::DeleteContainerPolicy).
            ///
            /// `ParseStrictResponse` impl for `DeleteContainerPolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteContainerPolicy {
    _private: ()
}
impl DeleteContainerPolicy {
    /// Creates a new builder-style object to manufacture [`DeleteContainerPolicyInput`](crate::input::DeleteContainerPolicyInput).
    pub fn builder() -> crate::input::delete_container_policy_input::Builder {
        crate::input::delete_container_policy_input::Builder::default()
    }
    /// Creates a new `DeleteContainerPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteContainerPolicy {
                type Output = std::result::Result<crate::output::DeleteContainerPolicyOutput, crate::error::DeleteContainerPolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_container_policy_error(response)
                     } else {
                        crate::operation_deser::parse_delete_container_policy_response(response)
                     }
                }
            }

/// Operation shape for `DeleteCorsPolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_cors_policy`](crate::client::fluent_builders::DeleteCorsPolicy).
            ///
            /// `ParseStrictResponse` impl for `DeleteCorsPolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteCorsPolicy {
    _private: ()
}
impl DeleteCorsPolicy {
    /// Creates a new builder-style object to manufacture [`DeleteCorsPolicyInput`](crate::input::DeleteCorsPolicyInput).
    pub fn builder() -> crate::input::delete_cors_policy_input::Builder {
        crate::input::delete_cors_policy_input::Builder::default()
    }
    /// Creates a new `DeleteCorsPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteCorsPolicy {
                type Output = std::result::Result<crate::output::DeleteCorsPolicyOutput, crate::error::DeleteCorsPolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_cors_policy_error(response)
                     } else {
                        crate::operation_deser::parse_delete_cors_policy_response(response)
                     }
                }
            }

/// Operation shape for `DeleteLifecyclePolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_lifecycle_policy`](crate::client::fluent_builders::DeleteLifecyclePolicy).
            ///
            /// `ParseStrictResponse` impl for `DeleteLifecyclePolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteLifecyclePolicy {
    _private: ()
}
impl DeleteLifecyclePolicy {
    /// Creates a new builder-style object to manufacture [`DeleteLifecyclePolicyInput`](crate::input::DeleteLifecyclePolicyInput).
    pub fn builder() -> crate::input::delete_lifecycle_policy_input::Builder {
        crate::input::delete_lifecycle_policy_input::Builder::default()
    }
    /// Creates a new `DeleteLifecyclePolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteLifecyclePolicy {
                type Output = std::result::Result<crate::output::DeleteLifecyclePolicyOutput, crate::error::DeleteLifecyclePolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_lifecycle_policy_error(response)
                     } else {
                        crate::operation_deser::parse_delete_lifecycle_policy_response(response)
                     }
                }
            }

/// Operation shape for `DeleteMetricPolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_metric_policy`](crate::client::fluent_builders::DeleteMetricPolicy).
            ///
            /// `ParseStrictResponse` impl for `DeleteMetricPolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteMetricPolicy {
    _private: ()
}
impl DeleteMetricPolicy {
    /// Creates a new builder-style object to manufacture [`DeleteMetricPolicyInput`](crate::input::DeleteMetricPolicyInput).
    pub fn builder() -> crate::input::delete_metric_policy_input::Builder {
        crate::input::delete_metric_policy_input::Builder::default()
    }
    /// Creates a new `DeleteMetricPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteMetricPolicy {
                type Output = std::result::Result<crate::output::DeleteMetricPolicyOutput, crate::error::DeleteMetricPolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_metric_policy_error(response)
                     } else {
                        crate::operation_deser::parse_delete_metric_policy_response(response)
                     }
                }
            }

/// Operation shape for `DescribeContainer`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_container`](crate::client::fluent_builders::DescribeContainer).
            ///
            /// `ParseStrictResponse` impl for `DescribeContainer`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeContainer {
    _private: ()
}
impl DescribeContainer {
    /// Creates a new builder-style object to manufacture [`DescribeContainerInput`](crate::input::DescribeContainerInput).
    pub fn builder() -> crate::input::describe_container_input::Builder {
        crate::input::describe_container_input::Builder::default()
    }
    /// Creates a new `DescribeContainer` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeContainer {
                type Output = std::result::Result<crate::output::DescribeContainerOutput, crate::error::DescribeContainerError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_container_error(response)
                     } else {
                        crate::operation_deser::parse_describe_container_response(response)
                     }
                }
            }

/// Operation shape for `GetContainerPolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_container_policy`](crate::client::fluent_builders::GetContainerPolicy).
            ///
            /// `ParseStrictResponse` impl for `GetContainerPolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetContainerPolicy {
    _private: ()
}
impl GetContainerPolicy {
    /// Creates a new builder-style object to manufacture [`GetContainerPolicyInput`](crate::input::GetContainerPolicyInput).
    pub fn builder() -> crate::input::get_container_policy_input::Builder {
        crate::input::get_container_policy_input::Builder::default()
    }
    /// Creates a new `GetContainerPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetContainerPolicy {
                type Output = std::result::Result<crate::output::GetContainerPolicyOutput, crate::error::GetContainerPolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_container_policy_error(response)
                     } else {
                        crate::operation_deser::parse_get_container_policy_response(response)
                     }
                }
            }

/// Operation shape for `GetCorsPolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_cors_policy`](crate::client::fluent_builders::GetCorsPolicy).
            ///
            /// `ParseStrictResponse` impl for `GetCorsPolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetCorsPolicy {
    _private: ()
}
impl GetCorsPolicy {
    /// Creates a new builder-style object to manufacture [`GetCorsPolicyInput`](crate::input::GetCorsPolicyInput).
    pub fn builder() -> crate::input::get_cors_policy_input::Builder {
        crate::input::get_cors_policy_input::Builder::default()
    }
    /// Creates a new `GetCorsPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetCorsPolicy {
                type Output = std::result::Result<crate::output::GetCorsPolicyOutput, crate::error::GetCorsPolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_cors_policy_error(response)
                     } else {
                        crate::operation_deser::parse_get_cors_policy_response(response)
                     }
                }
            }

/// Operation shape for `GetLifecyclePolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_lifecycle_policy`](crate::client::fluent_builders::GetLifecyclePolicy).
            ///
            /// `ParseStrictResponse` impl for `GetLifecyclePolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetLifecyclePolicy {
    _private: ()
}
impl GetLifecyclePolicy {
    /// Creates a new builder-style object to manufacture [`GetLifecyclePolicyInput`](crate::input::GetLifecyclePolicyInput).
    pub fn builder() -> crate::input::get_lifecycle_policy_input::Builder {
        crate::input::get_lifecycle_policy_input::Builder::default()
    }
    /// Creates a new `GetLifecyclePolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetLifecyclePolicy {
                type Output = std::result::Result<crate::output::GetLifecyclePolicyOutput, crate::error::GetLifecyclePolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_lifecycle_policy_error(response)
                     } else {
                        crate::operation_deser::parse_get_lifecycle_policy_response(response)
                     }
                }
            }

/// Operation shape for `GetMetricPolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_metric_policy`](crate::client::fluent_builders::GetMetricPolicy).
            ///
            /// `ParseStrictResponse` impl for `GetMetricPolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetMetricPolicy {
    _private: ()
}
impl GetMetricPolicy {
    /// Creates a new builder-style object to manufacture [`GetMetricPolicyInput`](crate::input::GetMetricPolicyInput).
    pub fn builder() -> crate::input::get_metric_policy_input::Builder {
        crate::input::get_metric_policy_input::Builder::default()
    }
    /// Creates a new `GetMetricPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetMetricPolicy {
                type Output = std::result::Result<crate::output::GetMetricPolicyOutput, crate::error::GetMetricPolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_metric_policy_error(response)
                     } else {
                        crate::operation_deser::parse_get_metric_policy_response(response)
                     }
                }
            }

/// Operation shape for `ListContainers`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_containers`](crate::client::fluent_builders::ListContainers).
            ///
            /// `ParseStrictResponse` impl for `ListContainers`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListContainers {
    _private: ()
}
impl ListContainers {
    /// Creates a new builder-style object to manufacture [`ListContainersInput`](crate::input::ListContainersInput).
    pub fn builder() -> crate::input::list_containers_input::Builder {
        crate::input::list_containers_input::Builder::default()
    }
    /// Creates a new `ListContainers` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListContainers {
                type Output = std::result::Result<crate::output::ListContainersOutput, crate::error::ListContainersError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_containers_error(response)
                     } else {
                        crate::operation_deser::parse_list_containers_response(response)
                     }
                }
            }

/// Operation shape for `ListTagsForResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_tags_for_resource`](crate::client::fluent_builders::ListTagsForResource).
            ///
            /// `ParseStrictResponse` impl for `ListTagsForResource`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: ()
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput).
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
                type Output = std::result::Result<crate::output::ListTagsForResourceOutput, crate::error::ListTagsForResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_tags_for_resource_error(response)
                     } else {
                        crate::operation_deser::parse_list_tags_for_resource_response(response)
                     }
                }
            }

/// Operation shape for `PutContainerPolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_container_policy`](crate::client::fluent_builders::PutContainerPolicy).
            ///
            /// `ParseStrictResponse` impl for `PutContainerPolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutContainerPolicy {
    _private: ()
}
impl PutContainerPolicy {
    /// Creates a new builder-style object to manufacture [`PutContainerPolicyInput`](crate::input::PutContainerPolicyInput).
    pub fn builder() -> crate::input::put_container_policy_input::Builder {
        crate::input::put_container_policy_input::Builder::default()
    }
    /// Creates a new `PutContainerPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutContainerPolicy {
                type Output = std::result::Result<crate::output::PutContainerPolicyOutput, crate::error::PutContainerPolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_put_container_policy_error(response)
                     } else {
                        crate::operation_deser::parse_put_container_policy_response(response)
                     }
                }
            }

/// Operation shape for `PutCorsPolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_cors_policy`](crate::client::fluent_builders::PutCorsPolicy).
            ///
            /// `ParseStrictResponse` impl for `PutCorsPolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutCorsPolicy {
    _private: ()
}
impl PutCorsPolicy {
    /// Creates a new builder-style object to manufacture [`PutCorsPolicyInput`](crate::input::PutCorsPolicyInput).
    pub fn builder() -> crate::input::put_cors_policy_input::Builder {
        crate::input::put_cors_policy_input::Builder::default()
    }
    /// Creates a new `PutCorsPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutCorsPolicy {
                type Output = std::result::Result<crate::output::PutCorsPolicyOutput, crate::error::PutCorsPolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_put_cors_policy_error(response)
                     } else {
                        crate::operation_deser::parse_put_cors_policy_response(response)
                     }
                }
            }

/// Operation shape for `PutLifecyclePolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_lifecycle_policy`](crate::client::fluent_builders::PutLifecyclePolicy).
            ///
            /// `ParseStrictResponse` impl for `PutLifecyclePolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutLifecyclePolicy {
    _private: ()
}
impl PutLifecyclePolicy {
    /// Creates a new builder-style object to manufacture [`PutLifecyclePolicyInput`](crate::input::PutLifecyclePolicyInput).
    pub fn builder() -> crate::input::put_lifecycle_policy_input::Builder {
        crate::input::put_lifecycle_policy_input::Builder::default()
    }
    /// Creates a new `PutLifecyclePolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutLifecyclePolicy {
                type Output = std::result::Result<crate::output::PutLifecyclePolicyOutput, crate::error::PutLifecyclePolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_put_lifecycle_policy_error(response)
                     } else {
                        crate::operation_deser::parse_put_lifecycle_policy_response(response)
                     }
                }
            }

/// Operation shape for `PutMetricPolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_metric_policy`](crate::client::fluent_builders::PutMetricPolicy).
            ///
            /// `ParseStrictResponse` impl for `PutMetricPolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutMetricPolicy {
    _private: ()
}
impl PutMetricPolicy {
    /// Creates a new builder-style object to manufacture [`PutMetricPolicyInput`](crate::input::PutMetricPolicyInput).
    pub fn builder() -> crate::input::put_metric_policy_input::Builder {
        crate::input::put_metric_policy_input::Builder::default()
    }
    /// Creates a new `PutMetricPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutMetricPolicy {
                type Output = std::result::Result<crate::output::PutMetricPolicyOutput, crate::error::PutMetricPolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_put_metric_policy_error(response)
                     } else {
                        crate::operation_deser::parse_put_metric_policy_response(response)
                     }
                }
            }

/// Operation shape for `StartAccessLogging`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`start_access_logging`](crate::client::fluent_builders::StartAccessLogging).
            ///
            /// `ParseStrictResponse` impl for `StartAccessLogging`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StartAccessLogging {
    _private: ()
}
impl StartAccessLogging {
    /// Creates a new builder-style object to manufacture [`StartAccessLoggingInput`](crate::input::StartAccessLoggingInput).
    pub fn builder() -> crate::input::start_access_logging_input::Builder {
        crate::input::start_access_logging_input::Builder::default()
    }
    /// Creates a new `StartAccessLogging` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartAccessLogging {
                type Output = std::result::Result<crate::output::StartAccessLoggingOutput, crate::error::StartAccessLoggingError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_start_access_logging_error(response)
                     } else {
                        crate::operation_deser::parse_start_access_logging_response(response)
                     }
                }
            }

/// Operation shape for `StopAccessLogging`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`stop_access_logging`](crate::client::fluent_builders::StopAccessLogging).
            ///
            /// `ParseStrictResponse` impl for `StopAccessLogging`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StopAccessLogging {
    _private: ()
}
impl StopAccessLogging {
    /// Creates a new builder-style object to manufacture [`StopAccessLoggingInput`](crate::input::StopAccessLoggingInput).
    pub fn builder() -> crate::input::stop_access_logging_input::Builder {
        crate::input::stop_access_logging_input::Builder::default()
    }
    /// Creates a new `StopAccessLogging` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopAccessLogging {
                type Output = std::result::Result<crate::output::StopAccessLoggingOutput, crate::error::StopAccessLoggingError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_stop_access_logging_error(response)
                     } else {
                        crate::operation_deser::parse_stop_access_logging_response(response)
                     }
                }
            }

/// Operation shape for `TagResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`tag_resource`](crate::client::fluent_builders::TagResource).
            ///
            /// `ParseStrictResponse` impl for `TagResource`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct TagResource {
    _private: ()
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput).
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
                type Output = std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_tag_resource_error(response)
                     } else {
                        crate::operation_deser::parse_tag_resource_response(response)
                     }
                }
            }

/// Operation shape for `UntagResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`untag_resource`](crate::client::fluent_builders::UntagResource).
            ///
            /// `ParseStrictResponse` impl for `UntagResource`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UntagResource {
    _private: ()
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput).
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
                type Output = std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_untag_resource_error(response)
                     } else {
                        crate::operation_deser::parse_untag_resource_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

