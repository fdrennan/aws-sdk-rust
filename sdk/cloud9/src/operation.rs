// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateEnvironmentEC2`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_environment_ec2`](crate::client::fluent_builders::CreateEnvironmentEC2).
            ///
            /// `ParseStrictResponse` impl for `CreateEnvironmentEC2`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateEnvironmentEC2 {
    _private: ()
}
impl CreateEnvironmentEC2 {
    /// Creates a new builder-style object to manufacture [`CreateEnvironmentEc2Input`](crate::input::CreateEnvironmentEc2Input).
    pub fn builder() -> crate::input::create_environment_ec2_input::Builder {
        crate::input::create_environment_ec2_input::Builder::default()
    }
    /// Creates a new `CreateEnvironmentEC2` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateEnvironmentEC2 {
                type Output = std::result::Result<crate::output::CreateEnvironmentEc2Output, crate::error::CreateEnvironmentEC2Error>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_environment_ec2_error(response)
                     } else {
                        crate::operation_deser::parse_create_environment_ec2_response(response)
                     }
                }
            }

/// Operation shape for `CreateEnvironmentMembership`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_environment_membership`](crate::client::fluent_builders::CreateEnvironmentMembership).
            ///
            /// `ParseStrictResponse` impl for `CreateEnvironmentMembership`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateEnvironmentMembership {
    _private: ()
}
impl CreateEnvironmentMembership {
    /// Creates a new builder-style object to manufacture [`CreateEnvironmentMembershipInput`](crate::input::CreateEnvironmentMembershipInput).
    pub fn builder() -> crate::input::create_environment_membership_input::Builder {
        crate::input::create_environment_membership_input::Builder::default()
    }
    /// Creates a new `CreateEnvironmentMembership` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateEnvironmentMembership {
                type Output = std::result::Result<crate::output::CreateEnvironmentMembershipOutput, crate::error::CreateEnvironmentMembershipError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_environment_membership_error(response)
                     } else {
                        crate::operation_deser::parse_create_environment_membership_response(response)
                     }
                }
            }

/// Operation shape for `DeleteEnvironment`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_environment`](crate::client::fluent_builders::DeleteEnvironment).
            ///
            /// `ParseStrictResponse` impl for `DeleteEnvironment`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteEnvironment {
    _private: ()
}
impl DeleteEnvironment {
    /// Creates a new builder-style object to manufacture [`DeleteEnvironmentInput`](crate::input::DeleteEnvironmentInput).
    pub fn builder() -> crate::input::delete_environment_input::Builder {
        crate::input::delete_environment_input::Builder::default()
    }
    /// Creates a new `DeleteEnvironment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteEnvironment {
                type Output = std::result::Result<crate::output::DeleteEnvironmentOutput, crate::error::DeleteEnvironmentError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_environment_error(response)
                     } else {
                        crate::operation_deser::parse_delete_environment_response(response)
                     }
                }
            }

/// Operation shape for `DeleteEnvironmentMembership`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_environment_membership`](crate::client::fluent_builders::DeleteEnvironmentMembership).
            ///
            /// `ParseStrictResponse` impl for `DeleteEnvironmentMembership`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteEnvironmentMembership {
    _private: ()
}
impl DeleteEnvironmentMembership {
    /// Creates a new builder-style object to manufacture [`DeleteEnvironmentMembershipInput`](crate::input::DeleteEnvironmentMembershipInput).
    pub fn builder() -> crate::input::delete_environment_membership_input::Builder {
        crate::input::delete_environment_membership_input::Builder::default()
    }
    /// Creates a new `DeleteEnvironmentMembership` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteEnvironmentMembership {
                type Output = std::result::Result<crate::output::DeleteEnvironmentMembershipOutput, crate::error::DeleteEnvironmentMembershipError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_environment_membership_error(response)
                     } else {
                        crate::operation_deser::parse_delete_environment_membership_response(response)
                     }
                }
            }

/// Operation shape for `DescribeEnvironmentMemberships`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_environment_memberships`](crate::client::fluent_builders::DescribeEnvironmentMemberships).
            ///
            /// `ParseStrictResponse` impl for `DescribeEnvironmentMemberships`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeEnvironmentMemberships {
    _private: ()
}
impl DescribeEnvironmentMemberships {
    /// Creates a new builder-style object to manufacture [`DescribeEnvironmentMembershipsInput`](crate::input::DescribeEnvironmentMembershipsInput).
    pub fn builder() -> crate::input::describe_environment_memberships_input::Builder {
        crate::input::describe_environment_memberships_input::Builder::default()
    }
    /// Creates a new `DescribeEnvironmentMemberships` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeEnvironmentMemberships {
                type Output = std::result::Result<crate::output::DescribeEnvironmentMembershipsOutput, crate::error::DescribeEnvironmentMembershipsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_environment_memberships_error(response)
                     } else {
                        crate::operation_deser::parse_describe_environment_memberships_response(response)
                     }
                }
            }

/// Operation shape for `DescribeEnvironments`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_environments`](crate::client::fluent_builders::DescribeEnvironments).
            ///
            /// `ParseStrictResponse` impl for `DescribeEnvironments`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeEnvironments {
    _private: ()
}
impl DescribeEnvironments {
    /// Creates a new builder-style object to manufacture [`DescribeEnvironmentsInput`](crate::input::DescribeEnvironmentsInput).
    pub fn builder() -> crate::input::describe_environments_input::Builder {
        crate::input::describe_environments_input::Builder::default()
    }
    /// Creates a new `DescribeEnvironments` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeEnvironments {
                type Output = std::result::Result<crate::output::DescribeEnvironmentsOutput, crate::error::DescribeEnvironmentsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_environments_error(response)
                     } else {
                        crate::operation_deser::parse_describe_environments_response(response)
                     }
                }
            }

/// Operation shape for `DescribeEnvironmentStatus`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_environment_status`](crate::client::fluent_builders::DescribeEnvironmentStatus).
            ///
            /// `ParseStrictResponse` impl for `DescribeEnvironmentStatus`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeEnvironmentStatus {
    _private: ()
}
impl DescribeEnvironmentStatus {
    /// Creates a new builder-style object to manufacture [`DescribeEnvironmentStatusInput`](crate::input::DescribeEnvironmentStatusInput).
    pub fn builder() -> crate::input::describe_environment_status_input::Builder {
        crate::input::describe_environment_status_input::Builder::default()
    }
    /// Creates a new `DescribeEnvironmentStatus` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeEnvironmentStatus {
                type Output = std::result::Result<crate::output::DescribeEnvironmentStatusOutput, crate::error::DescribeEnvironmentStatusError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_environment_status_error(response)
                     } else {
                        crate::operation_deser::parse_describe_environment_status_response(response)
                     }
                }
            }

/// Operation shape for `ListEnvironments`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_environments`](crate::client::fluent_builders::ListEnvironments).
            ///
            /// `ParseStrictResponse` impl for `ListEnvironments`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListEnvironments {
    _private: ()
}
impl ListEnvironments {
    /// Creates a new builder-style object to manufacture [`ListEnvironmentsInput`](crate::input::ListEnvironmentsInput).
    pub fn builder() -> crate::input::list_environments_input::Builder {
        crate::input::list_environments_input::Builder::default()
    }
    /// Creates a new `ListEnvironments` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListEnvironments {
                type Output = std::result::Result<crate::output::ListEnvironmentsOutput, crate::error::ListEnvironmentsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_environments_error(response)
                     } else {
                        crate::operation_deser::parse_list_environments_response(response)
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

/// Operation shape for `UpdateEnvironment`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_environment`](crate::client::fluent_builders::UpdateEnvironment).
            ///
            /// `ParseStrictResponse` impl for `UpdateEnvironment`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateEnvironment {
    _private: ()
}
impl UpdateEnvironment {
    /// Creates a new builder-style object to manufacture [`UpdateEnvironmentInput`](crate::input::UpdateEnvironmentInput).
    pub fn builder() -> crate::input::update_environment_input::Builder {
        crate::input::update_environment_input::Builder::default()
    }
    /// Creates a new `UpdateEnvironment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateEnvironment {
                type Output = std::result::Result<crate::output::UpdateEnvironmentOutput, crate::error::UpdateEnvironmentError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_environment_error(response)
                     } else {
                        crate::operation_deser::parse_update_environment_response(response)
                     }
                }
            }

/// Operation shape for `UpdateEnvironmentMembership`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_environment_membership`](crate::client::fluent_builders::UpdateEnvironmentMembership).
            ///
            /// `ParseStrictResponse` impl for `UpdateEnvironmentMembership`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateEnvironmentMembership {
    _private: ()
}
impl UpdateEnvironmentMembership {
    /// Creates a new builder-style object to manufacture [`UpdateEnvironmentMembershipInput`](crate::input::UpdateEnvironmentMembershipInput).
    pub fn builder() -> crate::input::update_environment_membership_input::Builder {
        crate::input::update_environment_membership_input::Builder::default()
    }
    /// Creates a new `UpdateEnvironmentMembership` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateEnvironmentMembership {
                type Output = std::result::Result<crate::output::UpdateEnvironmentMembershipOutput, crate::error::UpdateEnvironmentMembershipError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_environment_membership_error(response)
                     } else {
                        crate::operation_deser::parse_update_environment_membership_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

