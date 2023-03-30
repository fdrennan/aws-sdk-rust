// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CancelRotateSecret`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`cancel_rotate_secret`](crate::client::fluent_builders::CancelRotateSecret).
            ///
            /// `ParseStrictResponse` impl for `CancelRotateSecret`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CancelRotateSecret {
    _private: ()
}
impl CancelRotateSecret {
    /// Creates a new builder-style object to manufacture [`CancelRotateSecretInput`](crate::input::CancelRotateSecretInput).
    pub fn builder() -> crate::input::cancel_rotate_secret_input::Builder {
        crate::input::cancel_rotate_secret_input::Builder::default()
    }
    /// Creates a new `CancelRotateSecret` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CancelRotateSecret {
                type Output = std::result::Result<crate::output::CancelRotateSecretOutput, crate::error::CancelRotateSecretError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_cancel_rotate_secret_error(response)
                     } else {
                        crate::operation_deser::parse_cancel_rotate_secret_response(response)
                     }
                }
            }

/// Operation shape for `CreateSecret`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_secret`](crate::client::fluent_builders::CreateSecret).
            ///
            /// `ParseStrictResponse` impl for `CreateSecret`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateSecret {
    _private: ()
}
impl CreateSecret {
    /// Creates a new builder-style object to manufacture [`CreateSecretInput`](crate::input::CreateSecretInput).
    pub fn builder() -> crate::input::create_secret_input::Builder {
        crate::input::create_secret_input::Builder::default()
    }
    /// Creates a new `CreateSecret` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateSecret {
                type Output = std::result::Result<crate::output::CreateSecretOutput, crate::error::CreateSecretError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_secret_error(response)
                     } else {
                        crate::operation_deser::parse_create_secret_response(response)
                     }
                }
            }

/// Operation shape for `DeleteResourcePolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_resource_policy`](crate::client::fluent_builders::DeleteResourcePolicy).
            ///
            /// `ParseStrictResponse` impl for `DeleteResourcePolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteResourcePolicy {
    _private: ()
}
impl DeleteResourcePolicy {
    /// Creates a new builder-style object to manufacture [`DeleteResourcePolicyInput`](crate::input::DeleteResourcePolicyInput).
    pub fn builder() -> crate::input::delete_resource_policy_input::Builder {
        crate::input::delete_resource_policy_input::Builder::default()
    }
    /// Creates a new `DeleteResourcePolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteResourcePolicy {
                type Output = std::result::Result<crate::output::DeleteResourcePolicyOutput, crate::error::DeleteResourcePolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_resource_policy_error(response)
                     } else {
                        crate::operation_deser::parse_delete_resource_policy_response(response)
                     }
                }
            }

/// Operation shape for `DeleteSecret`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_secret`](crate::client::fluent_builders::DeleteSecret).
            ///
            /// `ParseStrictResponse` impl for `DeleteSecret`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteSecret {
    _private: ()
}
impl DeleteSecret {
    /// Creates a new builder-style object to manufacture [`DeleteSecretInput`](crate::input::DeleteSecretInput).
    pub fn builder() -> crate::input::delete_secret_input::Builder {
        crate::input::delete_secret_input::Builder::default()
    }
    /// Creates a new `DeleteSecret` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteSecret {
                type Output = std::result::Result<crate::output::DeleteSecretOutput, crate::error::DeleteSecretError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_secret_error(response)
                     } else {
                        crate::operation_deser::parse_delete_secret_response(response)
                     }
                }
            }

/// Operation shape for `DescribeSecret`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_secret`](crate::client::fluent_builders::DescribeSecret).
            ///
            /// `ParseStrictResponse` impl for `DescribeSecret`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeSecret {
    _private: ()
}
impl DescribeSecret {
    /// Creates a new builder-style object to manufacture [`DescribeSecretInput`](crate::input::DescribeSecretInput).
    pub fn builder() -> crate::input::describe_secret_input::Builder {
        crate::input::describe_secret_input::Builder::default()
    }
    /// Creates a new `DescribeSecret` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeSecret {
                type Output = std::result::Result<crate::output::DescribeSecretOutput, crate::error::DescribeSecretError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_secret_error(response)
                     } else {
                        crate::operation_deser::parse_describe_secret_response(response)
                     }
                }
            }

/// Operation shape for `GetRandomPassword`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_random_password`](crate::client::fluent_builders::GetRandomPassword).
            ///
            /// `ParseStrictResponse` impl for `GetRandomPassword`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetRandomPassword {
    _private: ()
}
impl GetRandomPassword {
    /// Creates a new builder-style object to manufacture [`GetRandomPasswordInput`](crate::input::GetRandomPasswordInput).
    pub fn builder() -> crate::input::get_random_password_input::Builder {
        crate::input::get_random_password_input::Builder::default()
    }
    /// Creates a new `GetRandomPassword` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetRandomPassword {
                type Output = std::result::Result<crate::output::GetRandomPasswordOutput, crate::error::GetRandomPasswordError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_random_password_error(response)
                     } else {
                        crate::operation_deser::parse_get_random_password_response(response)
                     }
                }
            }

/// Operation shape for `GetResourcePolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_resource_policy`](crate::client::fluent_builders::GetResourcePolicy).
            ///
            /// `ParseStrictResponse` impl for `GetResourcePolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetResourcePolicy {
    _private: ()
}
impl GetResourcePolicy {
    /// Creates a new builder-style object to manufacture [`GetResourcePolicyInput`](crate::input::GetResourcePolicyInput).
    pub fn builder() -> crate::input::get_resource_policy_input::Builder {
        crate::input::get_resource_policy_input::Builder::default()
    }
    /// Creates a new `GetResourcePolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetResourcePolicy {
                type Output = std::result::Result<crate::output::GetResourcePolicyOutput, crate::error::GetResourcePolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_resource_policy_error(response)
                     } else {
                        crate::operation_deser::parse_get_resource_policy_response(response)
                     }
                }
            }

/// Operation shape for `GetSecretValue`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_secret_value`](crate::client::fluent_builders::GetSecretValue).
            ///
            /// `ParseStrictResponse` impl for `GetSecretValue`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetSecretValue {
    _private: ()
}
impl GetSecretValue {
    /// Creates a new builder-style object to manufacture [`GetSecretValueInput`](crate::input::GetSecretValueInput).
    pub fn builder() -> crate::input::get_secret_value_input::Builder {
        crate::input::get_secret_value_input::Builder::default()
    }
    /// Creates a new `GetSecretValue` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetSecretValue {
                type Output = std::result::Result<crate::output::GetSecretValueOutput, crate::error::GetSecretValueError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_secret_value_error(response)
                     } else {
                        crate::operation_deser::parse_get_secret_value_response(response)
                     }
                }
            }

/// Operation shape for `ListSecrets`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_secrets`](crate::client::fluent_builders::ListSecrets).
            ///
            /// `ParseStrictResponse` impl for `ListSecrets`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListSecrets {
    _private: ()
}
impl ListSecrets {
    /// Creates a new builder-style object to manufacture [`ListSecretsInput`](crate::input::ListSecretsInput).
    pub fn builder() -> crate::input::list_secrets_input::Builder {
        crate::input::list_secrets_input::Builder::default()
    }
    /// Creates a new `ListSecrets` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListSecrets {
                type Output = std::result::Result<crate::output::ListSecretsOutput, crate::error::ListSecretsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_secrets_error(response)
                     } else {
                        crate::operation_deser::parse_list_secrets_response(response)
                     }
                }
            }

/// Operation shape for `ListSecretVersionIds`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_secret_version_ids`](crate::client::fluent_builders::ListSecretVersionIds).
            ///
            /// `ParseStrictResponse` impl for `ListSecretVersionIds`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListSecretVersionIds {
    _private: ()
}
impl ListSecretVersionIds {
    /// Creates a new builder-style object to manufacture [`ListSecretVersionIdsInput`](crate::input::ListSecretVersionIdsInput).
    pub fn builder() -> crate::input::list_secret_version_ids_input::Builder {
        crate::input::list_secret_version_ids_input::Builder::default()
    }
    /// Creates a new `ListSecretVersionIds` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListSecretVersionIds {
                type Output = std::result::Result<crate::output::ListSecretVersionIdsOutput, crate::error::ListSecretVersionIdsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_secret_version_ids_error(response)
                     } else {
                        crate::operation_deser::parse_list_secret_version_ids_response(response)
                     }
                }
            }

/// Operation shape for `PutResourcePolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_resource_policy`](crate::client::fluent_builders::PutResourcePolicy).
            ///
            /// `ParseStrictResponse` impl for `PutResourcePolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutResourcePolicy {
    _private: ()
}
impl PutResourcePolicy {
    /// Creates a new builder-style object to manufacture [`PutResourcePolicyInput`](crate::input::PutResourcePolicyInput).
    pub fn builder() -> crate::input::put_resource_policy_input::Builder {
        crate::input::put_resource_policy_input::Builder::default()
    }
    /// Creates a new `PutResourcePolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutResourcePolicy {
                type Output = std::result::Result<crate::output::PutResourcePolicyOutput, crate::error::PutResourcePolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_put_resource_policy_error(response)
                     } else {
                        crate::operation_deser::parse_put_resource_policy_response(response)
                     }
                }
            }

/// Operation shape for `PutSecretValue`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_secret_value`](crate::client::fluent_builders::PutSecretValue).
            ///
            /// `ParseStrictResponse` impl for `PutSecretValue`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutSecretValue {
    _private: ()
}
impl PutSecretValue {
    /// Creates a new builder-style object to manufacture [`PutSecretValueInput`](crate::input::PutSecretValueInput).
    pub fn builder() -> crate::input::put_secret_value_input::Builder {
        crate::input::put_secret_value_input::Builder::default()
    }
    /// Creates a new `PutSecretValue` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutSecretValue {
                type Output = std::result::Result<crate::output::PutSecretValueOutput, crate::error::PutSecretValueError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_put_secret_value_error(response)
                     } else {
                        crate::operation_deser::parse_put_secret_value_response(response)
                     }
                }
            }

/// Operation shape for `RemoveRegionsFromReplication`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`remove_regions_from_replication`](crate::client::fluent_builders::RemoveRegionsFromReplication).
            ///
            /// `ParseStrictResponse` impl for `RemoveRegionsFromReplication`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct RemoveRegionsFromReplication {
    _private: ()
}
impl RemoveRegionsFromReplication {
    /// Creates a new builder-style object to manufacture [`RemoveRegionsFromReplicationInput`](crate::input::RemoveRegionsFromReplicationInput).
    pub fn builder() -> crate::input::remove_regions_from_replication_input::Builder {
        crate::input::remove_regions_from_replication_input::Builder::default()
    }
    /// Creates a new `RemoveRegionsFromReplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RemoveRegionsFromReplication {
                type Output = std::result::Result<crate::output::RemoveRegionsFromReplicationOutput, crate::error::RemoveRegionsFromReplicationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_remove_regions_from_replication_error(response)
                     } else {
                        crate::operation_deser::parse_remove_regions_from_replication_response(response)
                     }
                }
            }

/// Operation shape for `ReplicateSecretToRegions`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`replicate_secret_to_regions`](crate::client::fluent_builders::ReplicateSecretToRegions).
            ///
            /// `ParseStrictResponse` impl for `ReplicateSecretToRegions`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ReplicateSecretToRegions {
    _private: ()
}
impl ReplicateSecretToRegions {
    /// Creates a new builder-style object to manufacture [`ReplicateSecretToRegionsInput`](crate::input::ReplicateSecretToRegionsInput).
    pub fn builder() -> crate::input::replicate_secret_to_regions_input::Builder {
        crate::input::replicate_secret_to_regions_input::Builder::default()
    }
    /// Creates a new `ReplicateSecretToRegions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ReplicateSecretToRegions {
                type Output = std::result::Result<crate::output::ReplicateSecretToRegionsOutput, crate::error::ReplicateSecretToRegionsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_replicate_secret_to_regions_error(response)
                     } else {
                        crate::operation_deser::parse_replicate_secret_to_regions_response(response)
                     }
                }
            }

/// Operation shape for `RestoreSecret`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`restore_secret`](crate::client::fluent_builders::RestoreSecret).
            ///
            /// `ParseStrictResponse` impl for `RestoreSecret`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct RestoreSecret {
    _private: ()
}
impl RestoreSecret {
    /// Creates a new builder-style object to manufacture [`RestoreSecretInput`](crate::input::RestoreSecretInput).
    pub fn builder() -> crate::input::restore_secret_input::Builder {
        crate::input::restore_secret_input::Builder::default()
    }
    /// Creates a new `RestoreSecret` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RestoreSecret {
                type Output = std::result::Result<crate::output::RestoreSecretOutput, crate::error::RestoreSecretError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_restore_secret_error(response)
                     } else {
                        crate::operation_deser::parse_restore_secret_response(response)
                     }
                }
            }

/// Operation shape for `RotateSecret`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`rotate_secret`](crate::client::fluent_builders::RotateSecret).
            ///
            /// `ParseStrictResponse` impl for `RotateSecret`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct RotateSecret {
    _private: ()
}
impl RotateSecret {
    /// Creates a new builder-style object to manufacture [`RotateSecretInput`](crate::input::RotateSecretInput).
    pub fn builder() -> crate::input::rotate_secret_input::Builder {
        crate::input::rotate_secret_input::Builder::default()
    }
    /// Creates a new `RotateSecret` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RotateSecret {
                type Output = std::result::Result<crate::output::RotateSecretOutput, crate::error::RotateSecretError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_rotate_secret_error(response)
                     } else {
                        crate::operation_deser::parse_rotate_secret_response(response)
                     }
                }
            }

/// Operation shape for `StopReplicationToReplica`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`stop_replication_to_replica`](crate::client::fluent_builders::StopReplicationToReplica).
            ///
            /// `ParseStrictResponse` impl for `StopReplicationToReplica`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StopReplicationToReplica {
    _private: ()
}
impl StopReplicationToReplica {
    /// Creates a new builder-style object to manufacture [`StopReplicationToReplicaInput`](crate::input::StopReplicationToReplicaInput).
    pub fn builder() -> crate::input::stop_replication_to_replica_input::Builder {
        crate::input::stop_replication_to_replica_input::Builder::default()
    }
    /// Creates a new `StopReplicationToReplica` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopReplicationToReplica {
                type Output = std::result::Result<crate::output::StopReplicationToReplicaOutput, crate::error::StopReplicationToReplicaError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_stop_replication_to_replica_error(response)
                     } else {
                        crate::operation_deser::parse_stop_replication_to_replica_response(response)
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

/// Operation shape for `UpdateSecret`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_secret`](crate::client::fluent_builders::UpdateSecret).
            ///
            /// `ParseStrictResponse` impl for `UpdateSecret`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateSecret {
    _private: ()
}
impl UpdateSecret {
    /// Creates a new builder-style object to manufacture [`UpdateSecretInput`](crate::input::UpdateSecretInput).
    pub fn builder() -> crate::input::update_secret_input::Builder {
        crate::input::update_secret_input::Builder::default()
    }
    /// Creates a new `UpdateSecret` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateSecret {
                type Output = std::result::Result<crate::output::UpdateSecretOutput, crate::error::UpdateSecretError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_secret_error(response)
                     } else {
                        crate::operation_deser::parse_update_secret_response(response)
                     }
                }
            }

/// Operation shape for `UpdateSecretVersionStage`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_secret_version_stage`](crate::client::fluent_builders::UpdateSecretVersionStage).
            ///
            /// `ParseStrictResponse` impl for `UpdateSecretVersionStage`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateSecretVersionStage {
    _private: ()
}
impl UpdateSecretVersionStage {
    /// Creates a new builder-style object to manufacture [`UpdateSecretVersionStageInput`](crate::input::UpdateSecretVersionStageInput).
    pub fn builder() -> crate::input::update_secret_version_stage_input::Builder {
        crate::input::update_secret_version_stage_input::Builder::default()
    }
    /// Creates a new `UpdateSecretVersionStage` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateSecretVersionStage {
                type Output = std::result::Result<crate::output::UpdateSecretVersionStageOutput, crate::error::UpdateSecretVersionStageError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_secret_version_stage_error(response)
                     } else {
                        crate::operation_deser::parse_update_secret_version_stage_response(response)
                     }
                }
            }

/// Operation shape for `ValidateResourcePolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`validate_resource_policy`](crate::client::fluent_builders::ValidateResourcePolicy).
            ///
            /// `ParseStrictResponse` impl for `ValidateResourcePolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ValidateResourcePolicy {
    _private: ()
}
impl ValidateResourcePolicy {
    /// Creates a new builder-style object to manufacture [`ValidateResourcePolicyInput`](crate::input::ValidateResourcePolicyInput).
    pub fn builder() -> crate::input::validate_resource_policy_input::Builder {
        crate::input::validate_resource_policy_input::Builder::default()
    }
    /// Creates a new `ValidateResourcePolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ValidateResourcePolicy {
                type Output = std::result::Result<crate::output::ValidateResourcePolicyOutput, crate::error::ValidateResourcePolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_validate_resource_policy_error(response)
                     } else {
                        crate::operation_deser::parse_validate_resource_policy_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

