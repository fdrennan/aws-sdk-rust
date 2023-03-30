// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateLifecyclePolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_lifecycle_policy`](crate::client::fluent_builders::CreateLifecyclePolicy).
            ///
            /// `ParseStrictResponse` impl for `CreateLifecyclePolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateLifecyclePolicy {
    _private: ()
}
impl CreateLifecyclePolicy {
    /// Creates a new builder-style object to manufacture [`CreateLifecyclePolicyInput`](crate::input::CreateLifecyclePolicyInput).
    pub fn builder() -> crate::input::create_lifecycle_policy_input::Builder {
        crate::input::create_lifecycle_policy_input::Builder::default()
    }
    /// Creates a new `CreateLifecyclePolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateLifecyclePolicy {
                type Output = std::result::Result<crate::output::CreateLifecyclePolicyOutput, crate::error::CreateLifecyclePolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_lifecycle_policy_error(response)
                     } else {
                        crate::operation_deser::parse_create_lifecycle_policy_response(response)
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

/// Operation shape for `GetLifecyclePolicies`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_lifecycle_policies`](crate::client::fluent_builders::GetLifecyclePolicies).
            ///
            /// `ParseStrictResponse` impl for `GetLifecyclePolicies`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetLifecyclePolicies {
    _private: ()
}
impl GetLifecyclePolicies {
    /// Creates a new builder-style object to manufacture [`GetLifecyclePoliciesInput`](crate::input::GetLifecyclePoliciesInput).
    pub fn builder() -> crate::input::get_lifecycle_policies_input::Builder {
        crate::input::get_lifecycle_policies_input::Builder::default()
    }
    /// Creates a new `GetLifecyclePolicies` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetLifecyclePolicies {
                type Output = std::result::Result<crate::output::GetLifecyclePoliciesOutput, crate::error::GetLifecyclePoliciesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_lifecycle_policies_error(response)
                     } else {
                        crate::operation_deser::parse_get_lifecycle_policies_response(response)
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

/// Operation shape for `UpdateLifecyclePolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_lifecycle_policy`](crate::client::fluent_builders::UpdateLifecyclePolicy).
            ///
            /// `ParseStrictResponse` impl for `UpdateLifecyclePolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateLifecyclePolicy {
    _private: ()
}
impl UpdateLifecyclePolicy {
    /// Creates a new builder-style object to manufacture [`UpdateLifecyclePolicyInput`](crate::input::UpdateLifecyclePolicyInput).
    pub fn builder() -> crate::input::update_lifecycle_policy_input::Builder {
        crate::input::update_lifecycle_policy_input::Builder::default()
    }
    /// Creates a new `UpdateLifecyclePolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateLifecyclePolicy {
                type Output = std::result::Result<crate::output::UpdateLifecyclePolicyOutput, crate::error::UpdateLifecyclePolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_lifecycle_policy_error(response)
                     } else {
                        crate::operation_deser::parse_update_lifecycle_policy_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

