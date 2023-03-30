// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateAlertManagerDefinition`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_alert_manager_definition`](crate::client::fluent_builders::CreateAlertManagerDefinition).
            ///
            /// `ParseStrictResponse` impl for `CreateAlertManagerDefinition`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateAlertManagerDefinition {
    _private: ()
}
impl CreateAlertManagerDefinition {
    /// Creates a new builder-style object to manufacture [`CreateAlertManagerDefinitionInput`](crate::input::CreateAlertManagerDefinitionInput).
    pub fn builder() -> crate::input::create_alert_manager_definition_input::Builder {
        crate::input::create_alert_manager_definition_input::Builder::default()
    }
    /// Creates a new `CreateAlertManagerDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateAlertManagerDefinition {
                type Output = std::result::Result<crate::output::CreateAlertManagerDefinitionOutput, crate::error::CreateAlertManagerDefinitionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 202 {
                        crate::operation_deser::parse_create_alert_manager_definition_error(response)
                     } else {
                        crate::operation_deser::parse_create_alert_manager_definition_response(response)
                     }
                }
            }

/// Operation shape for `CreateLoggingConfiguration`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_logging_configuration`](crate::client::fluent_builders::CreateLoggingConfiguration).
            ///
            /// `ParseStrictResponse` impl for `CreateLoggingConfiguration`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateLoggingConfiguration {
    _private: ()
}
impl CreateLoggingConfiguration {
    /// Creates a new builder-style object to manufacture [`CreateLoggingConfigurationInput`](crate::input::CreateLoggingConfigurationInput).
    pub fn builder() -> crate::input::create_logging_configuration_input::Builder {
        crate::input::create_logging_configuration_input::Builder::default()
    }
    /// Creates a new `CreateLoggingConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateLoggingConfiguration {
                type Output = std::result::Result<crate::output::CreateLoggingConfigurationOutput, crate::error::CreateLoggingConfigurationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 202 {
                        crate::operation_deser::parse_create_logging_configuration_error(response)
                     } else {
                        crate::operation_deser::parse_create_logging_configuration_response(response)
                     }
                }
            }

/// Operation shape for `CreateRuleGroupsNamespace`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_rule_groups_namespace`](crate::client::fluent_builders::CreateRuleGroupsNamespace).
            ///
            /// `ParseStrictResponse` impl for `CreateRuleGroupsNamespace`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateRuleGroupsNamespace {
    _private: ()
}
impl CreateRuleGroupsNamespace {
    /// Creates a new builder-style object to manufacture [`CreateRuleGroupsNamespaceInput`](crate::input::CreateRuleGroupsNamespaceInput).
    pub fn builder() -> crate::input::create_rule_groups_namespace_input::Builder {
        crate::input::create_rule_groups_namespace_input::Builder::default()
    }
    /// Creates a new `CreateRuleGroupsNamespace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateRuleGroupsNamespace {
                type Output = std::result::Result<crate::output::CreateRuleGroupsNamespaceOutput, crate::error::CreateRuleGroupsNamespaceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 202 {
                        crate::operation_deser::parse_create_rule_groups_namespace_error(response)
                     } else {
                        crate::operation_deser::parse_create_rule_groups_namespace_response(response)
                     }
                }
            }

/// Operation shape for `CreateWorkspace`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_workspace`](crate::client::fluent_builders::CreateWorkspace).
            ///
            /// `ParseStrictResponse` impl for `CreateWorkspace`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateWorkspace {
    _private: ()
}
impl CreateWorkspace {
    /// Creates a new builder-style object to manufacture [`CreateWorkspaceInput`](crate::input::CreateWorkspaceInput).
    pub fn builder() -> crate::input::create_workspace_input::Builder {
        crate::input::create_workspace_input::Builder::default()
    }
    /// Creates a new `CreateWorkspace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateWorkspace {
                type Output = std::result::Result<crate::output::CreateWorkspaceOutput, crate::error::CreateWorkspaceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 202 {
                        crate::operation_deser::parse_create_workspace_error(response)
                     } else {
                        crate::operation_deser::parse_create_workspace_response(response)
                     }
                }
            }

/// Operation shape for `DeleteAlertManagerDefinition`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_alert_manager_definition`](crate::client::fluent_builders::DeleteAlertManagerDefinition).
            ///
            /// `ParseStrictResponse` impl for `DeleteAlertManagerDefinition`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteAlertManagerDefinition {
    _private: ()
}
impl DeleteAlertManagerDefinition {
    /// Creates a new builder-style object to manufacture [`DeleteAlertManagerDefinitionInput`](crate::input::DeleteAlertManagerDefinitionInput).
    pub fn builder() -> crate::input::delete_alert_manager_definition_input::Builder {
        crate::input::delete_alert_manager_definition_input::Builder::default()
    }
    /// Creates a new `DeleteAlertManagerDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteAlertManagerDefinition {
                type Output = std::result::Result<crate::output::DeleteAlertManagerDefinitionOutput, crate::error::DeleteAlertManagerDefinitionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 202 {
                        crate::operation_deser::parse_delete_alert_manager_definition_error(response)
                     } else {
                        crate::operation_deser::parse_delete_alert_manager_definition_response(response)
                     }
                }
            }

/// Operation shape for `DeleteLoggingConfiguration`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_logging_configuration`](crate::client::fluent_builders::DeleteLoggingConfiguration).
            ///
            /// `ParseStrictResponse` impl for `DeleteLoggingConfiguration`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteLoggingConfiguration {
    _private: ()
}
impl DeleteLoggingConfiguration {
    /// Creates a new builder-style object to manufacture [`DeleteLoggingConfigurationInput`](crate::input::DeleteLoggingConfigurationInput).
    pub fn builder() -> crate::input::delete_logging_configuration_input::Builder {
        crate::input::delete_logging_configuration_input::Builder::default()
    }
    /// Creates a new `DeleteLoggingConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteLoggingConfiguration {
                type Output = std::result::Result<crate::output::DeleteLoggingConfigurationOutput, crate::error::DeleteLoggingConfigurationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 202 {
                        crate::operation_deser::parse_delete_logging_configuration_error(response)
                     } else {
                        crate::operation_deser::parse_delete_logging_configuration_response(response)
                     }
                }
            }

/// Operation shape for `DeleteRuleGroupsNamespace`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_rule_groups_namespace`](crate::client::fluent_builders::DeleteRuleGroupsNamespace).
            ///
            /// `ParseStrictResponse` impl for `DeleteRuleGroupsNamespace`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteRuleGroupsNamespace {
    _private: ()
}
impl DeleteRuleGroupsNamespace {
    /// Creates a new builder-style object to manufacture [`DeleteRuleGroupsNamespaceInput`](crate::input::DeleteRuleGroupsNamespaceInput).
    pub fn builder() -> crate::input::delete_rule_groups_namespace_input::Builder {
        crate::input::delete_rule_groups_namespace_input::Builder::default()
    }
    /// Creates a new `DeleteRuleGroupsNamespace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteRuleGroupsNamespace {
                type Output = std::result::Result<crate::output::DeleteRuleGroupsNamespaceOutput, crate::error::DeleteRuleGroupsNamespaceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 202 {
                        crate::operation_deser::parse_delete_rule_groups_namespace_error(response)
                     } else {
                        crate::operation_deser::parse_delete_rule_groups_namespace_response(response)
                     }
                }
            }

/// Operation shape for `DeleteWorkspace`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_workspace`](crate::client::fluent_builders::DeleteWorkspace).
            ///
            /// `ParseStrictResponse` impl for `DeleteWorkspace`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteWorkspace {
    _private: ()
}
impl DeleteWorkspace {
    /// Creates a new builder-style object to manufacture [`DeleteWorkspaceInput`](crate::input::DeleteWorkspaceInput).
    pub fn builder() -> crate::input::delete_workspace_input::Builder {
        crate::input::delete_workspace_input::Builder::default()
    }
    /// Creates a new `DeleteWorkspace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteWorkspace {
                type Output = std::result::Result<crate::output::DeleteWorkspaceOutput, crate::error::DeleteWorkspaceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 202 {
                        crate::operation_deser::parse_delete_workspace_error(response)
                     } else {
                        crate::operation_deser::parse_delete_workspace_response(response)
                     }
                }
            }

/// Operation shape for `DescribeAlertManagerDefinition`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_alert_manager_definition`](crate::client::fluent_builders::DescribeAlertManagerDefinition).
            ///
            /// `ParseStrictResponse` impl for `DescribeAlertManagerDefinition`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeAlertManagerDefinition {
    _private: ()
}
impl DescribeAlertManagerDefinition {
    /// Creates a new builder-style object to manufacture [`DescribeAlertManagerDefinitionInput`](crate::input::DescribeAlertManagerDefinitionInput).
    pub fn builder() -> crate::input::describe_alert_manager_definition_input::Builder {
        crate::input::describe_alert_manager_definition_input::Builder::default()
    }
    /// Creates a new `DescribeAlertManagerDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeAlertManagerDefinition {
                type Output = std::result::Result<crate::output::DescribeAlertManagerDefinitionOutput, crate::error::DescribeAlertManagerDefinitionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_alert_manager_definition_error(response)
                     } else {
                        crate::operation_deser::parse_describe_alert_manager_definition_response(response)
                     }
                }
            }

/// Operation shape for `DescribeLoggingConfiguration`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_logging_configuration`](crate::client::fluent_builders::DescribeLoggingConfiguration).
            ///
            /// `ParseStrictResponse` impl for `DescribeLoggingConfiguration`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeLoggingConfiguration {
    _private: ()
}
impl DescribeLoggingConfiguration {
    /// Creates a new builder-style object to manufacture [`DescribeLoggingConfigurationInput`](crate::input::DescribeLoggingConfigurationInput).
    pub fn builder() -> crate::input::describe_logging_configuration_input::Builder {
        crate::input::describe_logging_configuration_input::Builder::default()
    }
    /// Creates a new `DescribeLoggingConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeLoggingConfiguration {
                type Output = std::result::Result<crate::output::DescribeLoggingConfigurationOutput, crate::error::DescribeLoggingConfigurationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_logging_configuration_error(response)
                     } else {
                        crate::operation_deser::parse_describe_logging_configuration_response(response)
                     }
                }
            }

/// Operation shape for `DescribeRuleGroupsNamespace`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_rule_groups_namespace`](crate::client::fluent_builders::DescribeRuleGroupsNamespace).
            ///
            /// `ParseStrictResponse` impl for `DescribeRuleGroupsNamespace`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeRuleGroupsNamespace {
    _private: ()
}
impl DescribeRuleGroupsNamespace {
    /// Creates a new builder-style object to manufacture [`DescribeRuleGroupsNamespaceInput`](crate::input::DescribeRuleGroupsNamespaceInput).
    pub fn builder() -> crate::input::describe_rule_groups_namespace_input::Builder {
        crate::input::describe_rule_groups_namespace_input::Builder::default()
    }
    /// Creates a new `DescribeRuleGroupsNamespace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeRuleGroupsNamespace {
                type Output = std::result::Result<crate::output::DescribeRuleGroupsNamespaceOutput, crate::error::DescribeRuleGroupsNamespaceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_rule_groups_namespace_error(response)
                     } else {
                        crate::operation_deser::parse_describe_rule_groups_namespace_response(response)
                     }
                }
            }

/// Operation shape for `DescribeWorkspace`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_workspace`](crate::client::fluent_builders::DescribeWorkspace).
            ///
            /// `ParseStrictResponse` impl for `DescribeWorkspace`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeWorkspace {
    _private: ()
}
impl DescribeWorkspace {
    /// Creates a new builder-style object to manufacture [`DescribeWorkspaceInput`](crate::input::DescribeWorkspaceInput).
    pub fn builder() -> crate::input::describe_workspace_input::Builder {
        crate::input::describe_workspace_input::Builder::default()
    }
    /// Creates a new `DescribeWorkspace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeWorkspace {
                type Output = std::result::Result<crate::output::DescribeWorkspaceOutput, crate::error::DescribeWorkspaceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_workspace_error(response)
                     } else {
                        crate::operation_deser::parse_describe_workspace_response(response)
                     }
                }
            }

/// Operation shape for `ListRuleGroupsNamespaces`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_rule_groups_namespaces`](crate::client::fluent_builders::ListRuleGroupsNamespaces).
            ///
            /// `ParseStrictResponse` impl for `ListRuleGroupsNamespaces`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListRuleGroupsNamespaces {
    _private: ()
}
impl ListRuleGroupsNamespaces {
    /// Creates a new builder-style object to manufacture [`ListRuleGroupsNamespacesInput`](crate::input::ListRuleGroupsNamespacesInput).
    pub fn builder() -> crate::input::list_rule_groups_namespaces_input::Builder {
        crate::input::list_rule_groups_namespaces_input::Builder::default()
    }
    /// Creates a new `ListRuleGroupsNamespaces` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListRuleGroupsNamespaces {
                type Output = std::result::Result<crate::output::ListRuleGroupsNamespacesOutput, crate::error::ListRuleGroupsNamespacesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_rule_groups_namespaces_error(response)
                     } else {
                        crate::operation_deser::parse_list_rule_groups_namespaces_response(response)
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

/// Operation shape for `ListWorkspaces`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_workspaces`](crate::client::fluent_builders::ListWorkspaces).
            ///
            /// `ParseStrictResponse` impl for `ListWorkspaces`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListWorkspaces {
    _private: ()
}
impl ListWorkspaces {
    /// Creates a new builder-style object to manufacture [`ListWorkspacesInput`](crate::input::ListWorkspacesInput).
    pub fn builder() -> crate::input::list_workspaces_input::Builder {
        crate::input::list_workspaces_input::Builder::default()
    }
    /// Creates a new `ListWorkspaces` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListWorkspaces {
                type Output = std::result::Result<crate::output::ListWorkspacesOutput, crate::error::ListWorkspacesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_workspaces_error(response)
                     } else {
                        crate::operation_deser::parse_list_workspaces_response(response)
                     }
                }
            }

/// Operation shape for `PutAlertManagerDefinition`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_alert_manager_definition`](crate::client::fluent_builders::PutAlertManagerDefinition).
            ///
            /// `ParseStrictResponse` impl for `PutAlertManagerDefinition`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutAlertManagerDefinition {
    _private: ()
}
impl PutAlertManagerDefinition {
    /// Creates a new builder-style object to manufacture [`PutAlertManagerDefinitionInput`](crate::input::PutAlertManagerDefinitionInput).
    pub fn builder() -> crate::input::put_alert_manager_definition_input::Builder {
        crate::input::put_alert_manager_definition_input::Builder::default()
    }
    /// Creates a new `PutAlertManagerDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutAlertManagerDefinition {
                type Output = std::result::Result<crate::output::PutAlertManagerDefinitionOutput, crate::error::PutAlertManagerDefinitionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 202 {
                        crate::operation_deser::parse_put_alert_manager_definition_error(response)
                     } else {
                        crate::operation_deser::parse_put_alert_manager_definition_response(response)
                     }
                }
            }

/// Operation shape for `PutRuleGroupsNamespace`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_rule_groups_namespace`](crate::client::fluent_builders::PutRuleGroupsNamespace).
            ///
            /// `ParseStrictResponse` impl for `PutRuleGroupsNamespace`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutRuleGroupsNamespace {
    _private: ()
}
impl PutRuleGroupsNamespace {
    /// Creates a new builder-style object to manufacture [`PutRuleGroupsNamespaceInput`](crate::input::PutRuleGroupsNamespaceInput).
    pub fn builder() -> crate::input::put_rule_groups_namespace_input::Builder {
        crate::input::put_rule_groups_namespace_input::Builder::default()
    }
    /// Creates a new `PutRuleGroupsNamespace` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutRuleGroupsNamespace {
                type Output = std::result::Result<crate::output::PutRuleGroupsNamespaceOutput, crate::error::PutRuleGroupsNamespaceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 202 {
                        crate::operation_deser::parse_put_rule_groups_namespace_error(response)
                     } else {
                        crate::operation_deser::parse_put_rule_groups_namespace_response(response)
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

/// Operation shape for `UpdateLoggingConfiguration`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_logging_configuration`](crate::client::fluent_builders::UpdateLoggingConfiguration).
            ///
            /// `ParseStrictResponse` impl for `UpdateLoggingConfiguration`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateLoggingConfiguration {
    _private: ()
}
impl UpdateLoggingConfiguration {
    /// Creates a new builder-style object to manufacture [`UpdateLoggingConfigurationInput`](crate::input::UpdateLoggingConfigurationInput).
    pub fn builder() -> crate::input::update_logging_configuration_input::Builder {
        crate::input::update_logging_configuration_input::Builder::default()
    }
    /// Creates a new `UpdateLoggingConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateLoggingConfiguration {
                type Output = std::result::Result<crate::output::UpdateLoggingConfigurationOutput, crate::error::UpdateLoggingConfigurationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 202 {
                        crate::operation_deser::parse_update_logging_configuration_error(response)
                     } else {
                        crate::operation_deser::parse_update_logging_configuration_response(response)
                     }
                }
            }

/// Operation shape for `UpdateWorkspaceAlias`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_workspace_alias`](crate::client::fluent_builders::UpdateWorkspaceAlias).
            ///
            /// `ParseStrictResponse` impl for `UpdateWorkspaceAlias`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateWorkspaceAlias {
    _private: ()
}
impl UpdateWorkspaceAlias {
    /// Creates a new builder-style object to manufacture [`UpdateWorkspaceAliasInput`](crate::input::UpdateWorkspaceAliasInput).
    pub fn builder() -> crate::input::update_workspace_alias_input::Builder {
        crate::input::update_workspace_alias_input::Builder::default()
    }
    /// Creates a new `UpdateWorkspaceAlias` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateWorkspaceAlias {
                type Output = std::result::Result<crate::output::UpdateWorkspaceAliasOutput, crate::error::UpdateWorkspaceAliasError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 204 {
                        crate::operation_deser::parse_update_workspace_alias_error(response)
                     } else {
                        crate::operation_deser::parse_update_workspace_alias_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

