// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateBroker`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_broker`](crate::client::fluent_builders::CreateBroker).
            ///
            /// `ParseStrictResponse` impl for `CreateBroker`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateBroker {
    _private: ()
}
impl CreateBroker {
    /// Creates a new builder-style object to manufacture [`CreateBrokerInput`](crate::input::CreateBrokerInput).
    pub fn builder() -> crate::input::create_broker_input::Builder {
        crate::input::create_broker_input::Builder::default()
    }
    /// Creates a new `CreateBroker` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateBroker {
                type Output = std::result::Result<crate::output::CreateBrokerOutput, crate::error::CreateBrokerError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_broker_error(response)
                     } else {
                        crate::operation_deser::parse_create_broker_response(response)
                     }
                }
            }

/// Operation shape for `CreateConfiguration`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_configuration`](crate::client::fluent_builders::CreateConfiguration).
            ///
            /// `ParseStrictResponse` impl for `CreateConfiguration`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateConfiguration {
    _private: ()
}
impl CreateConfiguration {
    /// Creates a new builder-style object to manufacture [`CreateConfigurationInput`](crate::input::CreateConfigurationInput).
    pub fn builder() -> crate::input::create_configuration_input::Builder {
        crate::input::create_configuration_input::Builder::default()
    }
    /// Creates a new `CreateConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateConfiguration {
                type Output = std::result::Result<crate::output::CreateConfigurationOutput, crate::error::CreateConfigurationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_configuration_error(response)
                     } else {
                        crate::operation_deser::parse_create_configuration_response(response)
                     }
                }
            }

/// Operation shape for `CreateTags`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_tags`](crate::client::fluent_builders::CreateTags).
            ///
            /// `ParseStrictResponse` impl for `CreateTags`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateTags {
    _private: ()
}
impl CreateTags {
    /// Creates a new builder-style object to manufacture [`CreateTagsInput`](crate::input::CreateTagsInput).
    pub fn builder() -> crate::input::create_tags_input::Builder {
        crate::input::create_tags_input::Builder::default()
    }
    /// Creates a new `CreateTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateTags {
                type Output = std::result::Result<crate::output::CreateTagsOutput, crate::error::CreateTagsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 204 {
                        crate::operation_deser::parse_create_tags_error(response)
                     } else {
                        crate::operation_deser::parse_create_tags_response(response)
                     }
                }
            }

/// Operation shape for `CreateUser`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_user`](crate::client::fluent_builders::CreateUser).
            ///
            /// `ParseStrictResponse` impl for `CreateUser`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateUser {
    _private: ()
}
impl CreateUser {
    /// Creates a new builder-style object to manufacture [`CreateUserInput`](crate::input::CreateUserInput).
    pub fn builder() -> crate::input::create_user_input::Builder {
        crate::input::create_user_input::Builder::default()
    }
    /// Creates a new `CreateUser` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateUser {
                type Output = std::result::Result<crate::output::CreateUserOutput, crate::error::CreateUserError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_user_error(response)
                     } else {
                        crate::operation_deser::parse_create_user_response(response)
                     }
                }
            }

/// Operation shape for `DeleteBroker`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_broker`](crate::client::fluent_builders::DeleteBroker).
            ///
            /// `ParseStrictResponse` impl for `DeleteBroker`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteBroker {
    _private: ()
}
impl DeleteBroker {
    /// Creates a new builder-style object to manufacture [`DeleteBrokerInput`](crate::input::DeleteBrokerInput).
    pub fn builder() -> crate::input::delete_broker_input::Builder {
        crate::input::delete_broker_input::Builder::default()
    }
    /// Creates a new `DeleteBroker` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteBroker {
                type Output = std::result::Result<crate::output::DeleteBrokerOutput, crate::error::DeleteBrokerError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_broker_error(response)
                     } else {
                        crate::operation_deser::parse_delete_broker_response(response)
                     }
                }
            }

/// Operation shape for `DeleteTags`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_tags`](crate::client::fluent_builders::DeleteTags).
            ///
            /// `ParseStrictResponse` impl for `DeleteTags`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteTags {
    _private: ()
}
impl DeleteTags {
    /// Creates a new builder-style object to manufacture [`DeleteTagsInput`](crate::input::DeleteTagsInput).
    pub fn builder() -> crate::input::delete_tags_input::Builder {
        crate::input::delete_tags_input::Builder::default()
    }
    /// Creates a new `DeleteTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteTags {
                type Output = std::result::Result<crate::output::DeleteTagsOutput, crate::error::DeleteTagsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 204 {
                        crate::operation_deser::parse_delete_tags_error(response)
                     } else {
                        crate::operation_deser::parse_delete_tags_response(response)
                     }
                }
            }

/// Operation shape for `DeleteUser`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_user`](crate::client::fluent_builders::DeleteUser).
            ///
            /// `ParseStrictResponse` impl for `DeleteUser`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteUser {
    _private: ()
}
impl DeleteUser {
    /// Creates a new builder-style object to manufacture [`DeleteUserInput`](crate::input::DeleteUserInput).
    pub fn builder() -> crate::input::delete_user_input::Builder {
        crate::input::delete_user_input::Builder::default()
    }
    /// Creates a new `DeleteUser` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteUser {
                type Output = std::result::Result<crate::output::DeleteUserOutput, crate::error::DeleteUserError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_user_error(response)
                     } else {
                        crate::operation_deser::parse_delete_user_response(response)
                     }
                }
            }

/// Operation shape for `DescribeBroker`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_broker`](crate::client::fluent_builders::DescribeBroker).
            ///
            /// `ParseStrictResponse` impl for `DescribeBroker`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeBroker {
    _private: ()
}
impl DescribeBroker {
    /// Creates a new builder-style object to manufacture [`DescribeBrokerInput`](crate::input::DescribeBrokerInput).
    pub fn builder() -> crate::input::describe_broker_input::Builder {
        crate::input::describe_broker_input::Builder::default()
    }
    /// Creates a new `DescribeBroker` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeBroker {
                type Output = std::result::Result<crate::output::DescribeBrokerOutput, crate::error::DescribeBrokerError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_broker_error(response)
                     } else {
                        crate::operation_deser::parse_describe_broker_response(response)
                     }
                }
            }

/// Operation shape for `DescribeBrokerEngineTypes`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_broker_engine_types`](crate::client::fluent_builders::DescribeBrokerEngineTypes).
            ///
            /// `ParseStrictResponse` impl for `DescribeBrokerEngineTypes`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeBrokerEngineTypes {
    _private: ()
}
impl DescribeBrokerEngineTypes {
    /// Creates a new builder-style object to manufacture [`DescribeBrokerEngineTypesInput`](crate::input::DescribeBrokerEngineTypesInput).
    pub fn builder() -> crate::input::describe_broker_engine_types_input::Builder {
        crate::input::describe_broker_engine_types_input::Builder::default()
    }
    /// Creates a new `DescribeBrokerEngineTypes` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeBrokerEngineTypes {
                type Output = std::result::Result<crate::output::DescribeBrokerEngineTypesOutput, crate::error::DescribeBrokerEngineTypesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_broker_engine_types_error(response)
                     } else {
                        crate::operation_deser::parse_describe_broker_engine_types_response(response)
                     }
                }
            }

/// Operation shape for `DescribeBrokerInstanceOptions`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_broker_instance_options`](crate::client::fluent_builders::DescribeBrokerInstanceOptions).
            ///
            /// `ParseStrictResponse` impl for `DescribeBrokerInstanceOptions`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeBrokerInstanceOptions {
    _private: ()
}
impl DescribeBrokerInstanceOptions {
    /// Creates a new builder-style object to manufacture [`DescribeBrokerInstanceOptionsInput`](crate::input::DescribeBrokerInstanceOptionsInput).
    pub fn builder() -> crate::input::describe_broker_instance_options_input::Builder {
        crate::input::describe_broker_instance_options_input::Builder::default()
    }
    /// Creates a new `DescribeBrokerInstanceOptions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeBrokerInstanceOptions {
                type Output = std::result::Result<crate::output::DescribeBrokerInstanceOptionsOutput, crate::error::DescribeBrokerInstanceOptionsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_broker_instance_options_error(response)
                     } else {
                        crate::operation_deser::parse_describe_broker_instance_options_response(response)
                     }
                }
            }

/// Operation shape for `DescribeConfiguration`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_configuration`](crate::client::fluent_builders::DescribeConfiguration).
            ///
            /// `ParseStrictResponse` impl for `DescribeConfiguration`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeConfiguration {
    _private: ()
}
impl DescribeConfiguration {
    /// Creates a new builder-style object to manufacture [`DescribeConfigurationInput`](crate::input::DescribeConfigurationInput).
    pub fn builder() -> crate::input::describe_configuration_input::Builder {
        crate::input::describe_configuration_input::Builder::default()
    }
    /// Creates a new `DescribeConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeConfiguration {
                type Output = std::result::Result<crate::output::DescribeConfigurationOutput, crate::error::DescribeConfigurationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_configuration_error(response)
                     } else {
                        crate::operation_deser::parse_describe_configuration_response(response)
                     }
                }
            }

/// Operation shape for `DescribeConfigurationRevision`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_configuration_revision`](crate::client::fluent_builders::DescribeConfigurationRevision).
            ///
            /// `ParseStrictResponse` impl for `DescribeConfigurationRevision`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeConfigurationRevision {
    _private: ()
}
impl DescribeConfigurationRevision {
    /// Creates a new builder-style object to manufacture [`DescribeConfigurationRevisionInput`](crate::input::DescribeConfigurationRevisionInput).
    pub fn builder() -> crate::input::describe_configuration_revision_input::Builder {
        crate::input::describe_configuration_revision_input::Builder::default()
    }
    /// Creates a new `DescribeConfigurationRevision` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeConfigurationRevision {
                type Output = std::result::Result<crate::output::DescribeConfigurationRevisionOutput, crate::error::DescribeConfigurationRevisionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_configuration_revision_error(response)
                     } else {
                        crate::operation_deser::parse_describe_configuration_revision_response(response)
                     }
                }
            }

/// Operation shape for `DescribeUser`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_user`](crate::client::fluent_builders::DescribeUser).
            ///
            /// `ParseStrictResponse` impl for `DescribeUser`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeUser {
    _private: ()
}
impl DescribeUser {
    /// Creates a new builder-style object to manufacture [`DescribeUserInput`](crate::input::DescribeUserInput).
    pub fn builder() -> crate::input::describe_user_input::Builder {
        crate::input::describe_user_input::Builder::default()
    }
    /// Creates a new `DescribeUser` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeUser {
                type Output = std::result::Result<crate::output::DescribeUserOutput, crate::error::DescribeUserError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_user_error(response)
                     } else {
                        crate::operation_deser::parse_describe_user_response(response)
                     }
                }
            }

/// Operation shape for `ListBrokers`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_brokers`](crate::client::fluent_builders::ListBrokers).
            ///
            /// `ParseStrictResponse` impl for `ListBrokers`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListBrokers {
    _private: ()
}
impl ListBrokers {
    /// Creates a new builder-style object to manufacture [`ListBrokersInput`](crate::input::ListBrokersInput).
    pub fn builder() -> crate::input::list_brokers_input::Builder {
        crate::input::list_brokers_input::Builder::default()
    }
    /// Creates a new `ListBrokers` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListBrokers {
                type Output = std::result::Result<crate::output::ListBrokersOutput, crate::error::ListBrokersError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_brokers_error(response)
                     } else {
                        crate::operation_deser::parse_list_brokers_response(response)
                     }
                }
            }

/// Operation shape for `ListConfigurationRevisions`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_configuration_revisions`](crate::client::fluent_builders::ListConfigurationRevisions).
            ///
            /// `ParseStrictResponse` impl for `ListConfigurationRevisions`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListConfigurationRevisions {
    _private: ()
}
impl ListConfigurationRevisions {
    /// Creates a new builder-style object to manufacture [`ListConfigurationRevisionsInput`](crate::input::ListConfigurationRevisionsInput).
    pub fn builder() -> crate::input::list_configuration_revisions_input::Builder {
        crate::input::list_configuration_revisions_input::Builder::default()
    }
    /// Creates a new `ListConfigurationRevisions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListConfigurationRevisions {
                type Output = std::result::Result<crate::output::ListConfigurationRevisionsOutput, crate::error::ListConfigurationRevisionsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_configuration_revisions_error(response)
                     } else {
                        crate::operation_deser::parse_list_configuration_revisions_response(response)
                     }
                }
            }

/// Operation shape for `ListConfigurations`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_configurations`](crate::client::fluent_builders::ListConfigurations).
            ///
            /// `ParseStrictResponse` impl for `ListConfigurations`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListConfigurations {
    _private: ()
}
impl ListConfigurations {
    /// Creates a new builder-style object to manufacture [`ListConfigurationsInput`](crate::input::ListConfigurationsInput).
    pub fn builder() -> crate::input::list_configurations_input::Builder {
        crate::input::list_configurations_input::Builder::default()
    }
    /// Creates a new `ListConfigurations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListConfigurations {
                type Output = std::result::Result<crate::output::ListConfigurationsOutput, crate::error::ListConfigurationsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_configurations_error(response)
                     } else {
                        crate::operation_deser::parse_list_configurations_response(response)
                     }
                }
            }

/// Operation shape for `ListTags`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_tags`](crate::client::fluent_builders::ListTags).
            ///
            /// `ParseStrictResponse` impl for `ListTags`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTags {
    _private: ()
}
impl ListTags {
    /// Creates a new builder-style object to manufacture [`ListTagsInput`](crate::input::ListTagsInput).
    pub fn builder() -> crate::input::list_tags_input::Builder {
        crate::input::list_tags_input::Builder::default()
    }
    /// Creates a new `ListTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTags {
                type Output = std::result::Result<crate::output::ListTagsOutput, crate::error::ListTagsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_tags_error(response)
                     } else {
                        crate::operation_deser::parse_list_tags_response(response)
                     }
                }
            }

/// Operation shape for `ListUsers`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_users`](crate::client::fluent_builders::ListUsers).
            ///
            /// `ParseStrictResponse` impl for `ListUsers`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListUsers {
    _private: ()
}
impl ListUsers {
    /// Creates a new builder-style object to manufacture [`ListUsersInput`](crate::input::ListUsersInput).
    pub fn builder() -> crate::input::list_users_input::Builder {
        crate::input::list_users_input::Builder::default()
    }
    /// Creates a new `ListUsers` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListUsers {
                type Output = std::result::Result<crate::output::ListUsersOutput, crate::error::ListUsersError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_users_error(response)
                     } else {
                        crate::operation_deser::parse_list_users_response(response)
                     }
                }
            }

/// Operation shape for `RebootBroker`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`reboot_broker`](crate::client::fluent_builders::RebootBroker).
            ///
            /// `ParseStrictResponse` impl for `RebootBroker`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct RebootBroker {
    _private: ()
}
impl RebootBroker {
    /// Creates a new builder-style object to manufacture [`RebootBrokerInput`](crate::input::RebootBrokerInput).
    pub fn builder() -> crate::input::reboot_broker_input::Builder {
        crate::input::reboot_broker_input::Builder::default()
    }
    /// Creates a new `RebootBroker` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RebootBroker {
                type Output = std::result::Result<crate::output::RebootBrokerOutput, crate::error::RebootBrokerError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_reboot_broker_error(response)
                     } else {
                        crate::operation_deser::parse_reboot_broker_response(response)
                     }
                }
            }

/// Operation shape for `UpdateBroker`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_broker`](crate::client::fluent_builders::UpdateBroker).
            ///
            /// `ParseStrictResponse` impl for `UpdateBroker`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateBroker {
    _private: ()
}
impl UpdateBroker {
    /// Creates a new builder-style object to manufacture [`UpdateBrokerInput`](crate::input::UpdateBrokerInput).
    pub fn builder() -> crate::input::update_broker_input::Builder {
        crate::input::update_broker_input::Builder::default()
    }
    /// Creates a new `UpdateBroker` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateBroker {
                type Output = std::result::Result<crate::output::UpdateBrokerOutput, crate::error::UpdateBrokerError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_broker_error(response)
                     } else {
                        crate::operation_deser::parse_update_broker_response(response)
                     }
                }
            }

/// Operation shape for `UpdateConfiguration`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_configuration`](crate::client::fluent_builders::UpdateConfiguration).
            ///
            /// `ParseStrictResponse` impl for `UpdateConfiguration`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateConfiguration {
    _private: ()
}
impl UpdateConfiguration {
    /// Creates a new builder-style object to manufacture [`UpdateConfigurationInput`](crate::input::UpdateConfigurationInput).
    pub fn builder() -> crate::input::update_configuration_input::Builder {
        crate::input::update_configuration_input::Builder::default()
    }
    /// Creates a new `UpdateConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateConfiguration {
                type Output = std::result::Result<crate::output::UpdateConfigurationOutput, crate::error::UpdateConfigurationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_configuration_error(response)
                     } else {
                        crate::operation_deser::parse_update_configuration_response(response)
                     }
                }
            }

/// Operation shape for `UpdateUser`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_user`](crate::client::fluent_builders::UpdateUser).
            ///
            /// `ParseStrictResponse` impl for `UpdateUser`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateUser {
    _private: ()
}
impl UpdateUser {
    /// Creates a new builder-style object to manufacture [`UpdateUserInput`](crate::input::UpdateUserInput).
    pub fn builder() -> crate::input::update_user_input::Builder {
        crate::input::update_user_input::Builder::default()
    }
    /// Creates a new `UpdateUser` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateUser {
                type Output = std::result::Result<crate::output::UpdateUserOutput, crate::error::UpdateUserError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_user_error(response)
                     } else {
                        crate::operation_deser::parse_update_user_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

