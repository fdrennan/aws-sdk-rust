// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateCluster`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_cluster`](crate::client::fluent_builders::CreateCluster).
            ///
            /// `ParseStrictResponse` impl for `CreateCluster`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateCluster {
    _private: ()
}
impl CreateCluster {
    /// Creates a new builder-style object to manufacture [`CreateClusterInput`](crate::input::CreateClusterInput).
    pub fn builder() -> crate::input::create_cluster_input::Builder {
        crate::input::create_cluster_input::Builder::default()
    }
    /// Creates a new `CreateCluster` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateCluster {
                type Output = std::result::Result<crate::output::CreateClusterOutput, crate::error::CreateClusterError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_cluster_error(response)
                     } else {
                        crate::operation_deser::parse_create_cluster_response(response)
                     }
                }
            }

/// Operation shape for `CreateControlPanel`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_control_panel`](crate::client::fluent_builders::CreateControlPanel).
            ///
            /// `ParseStrictResponse` impl for `CreateControlPanel`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateControlPanel {
    _private: ()
}
impl CreateControlPanel {
    /// Creates a new builder-style object to manufacture [`CreateControlPanelInput`](crate::input::CreateControlPanelInput).
    pub fn builder() -> crate::input::create_control_panel_input::Builder {
        crate::input::create_control_panel_input::Builder::default()
    }
    /// Creates a new `CreateControlPanel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateControlPanel {
                type Output = std::result::Result<crate::output::CreateControlPanelOutput, crate::error::CreateControlPanelError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_control_panel_error(response)
                     } else {
                        crate::operation_deser::parse_create_control_panel_response(response)
                     }
                }
            }

/// Operation shape for `CreateRoutingControl`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_routing_control`](crate::client::fluent_builders::CreateRoutingControl).
            ///
            /// `ParseStrictResponse` impl for `CreateRoutingControl`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateRoutingControl {
    _private: ()
}
impl CreateRoutingControl {
    /// Creates a new builder-style object to manufacture [`CreateRoutingControlInput`](crate::input::CreateRoutingControlInput).
    pub fn builder() -> crate::input::create_routing_control_input::Builder {
        crate::input::create_routing_control_input::Builder::default()
    }
    /// Creates a new `CreateRoutingControl` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateRoutingControl {
                type Output = std::result::Result<crate::output::CreateRoutingControlOutput, crate::error::CreateRoutingControlError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_routing_control_error(response)
                     } else {
                        crate::operation_deser::parse_create_routing_control_response(response)
                     }
                }
            }

/// Operation shape for `CreateSafetyRule`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_safety_rule`](crate::client::fluent_builders::CreateSafetyRule).
            ///
            /// `ParseStrictResponse` impl for `CreateSafetyRule`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateSafetyRule {
    _private: ()
}
impl CreateSafetyRule {
    /// Creates a new builder-style object to manufacture [`CreateSafetyRuleInput`](crate::input::CreateSafetyRuleInput).
    pub fn builder() -> crate::input::create_safety_rule_input::Builder {
        crate::input::create_safety_rule_input::Builder::default()
    }
    /// Creates a new `CreateSafetyRule` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateSafetyRule {
                type Output = std::result::Result<crate::output::CreateSafetyRuleOutput, crate::error::CreateSafetyRuleError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_safety_rule_error(response)
                     } else {
                        crate::operation_deser::parse_create_safety_rule_response(response)
                     }
                }
            }

/// Operation shape for `DeleteCluster`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_cluster`](crate::client::fluent_builders::DeleteCluster).
            ///
            /// `ParseStrictResponse` impl for `DeleteCluster`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteCluster {
    _private: ()
}
impl DeleteCluster {
    /// Creates a new builder-style object to manufacture [`DeleteClusterInput`](crate::input::DeleteClusterInput).
    pub fn builder() -> crate::input::delete_cluster_input::Builder {
        crate::input::delete_cluster_input::Builder::default()
    }
    /// Creates a new `DeleteCluster` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteCluster {
                type Output = std::result::Result<crate::output::DeleteClusterOutput, crate::error::DeleteClusterError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_cluster_error(response)
                     } else {
                        crate::operation_deser::parse_delete_cluster_response(response)
                     }
                }
            }

/// Operation shape for `DeleteControlPanel`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_control_panel`](crate::client::fluent_builders::DeleteControlPanel).
            ///
            /// `ParseStrictResponse` impl for `DeleteControlPanel`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteControlPanel {
    _private: ()
}
impl DeleteControlPanel {
    /// Creates a new builder-style object to manufacture [`DeleteControlPanelInput`](crate::input::DeleteControlPanelInput).
    pub fn builder() -> crate::input::delete_control_panel_input::Builder {
        crate::input::delete_control_panel_input::Builder::default()
    }
    /// Creates a new `DeleteControlPanel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteControlPanel {
                type Output = std::result::Result<crate::output::DeleteControlPanelOutput, crate::error::DeleteControlPanelError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_control_panel_error(response)
                     } else {
                        crate::operation_deser::parse_delete_control_panel_response(response)
                     }
                }
            }

/// Operation shape for `DeleteRoutingControl`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_routing_control`](crate::client::fluent_builders::DeleteRoutingControl).
            ///
            /// `ParseStrictResponse` impl for `DeleteRoutingControl`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteRoutingControl {
    _private: ()
}
impl DeleteRoutingControl {
    /// Creates a new builder-style object to manufacture [`DeleteRoutingControlInput`](crate::input::DeleteRoutingControlInput).
    pub fn builder() -> crate::input::delete_routing_control_input::Builder {
        crate::input::delete_routing_control_input::Builder::default()
    }
    /// Creates a new `DeleteRoutingControl` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteRoutingControl {
                type Output = std::result::Result<crate::output::DeleteRoutingControlOutput, crate::error::DeleteRoutingControlError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_routing_control_error(response)
                     } else {
                        crate::operation_deser::parse_delete_routing_control_response(response)
                     }
                }
            }

/// Operation shape for `DeleteSafetyRule`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_safety_rule`](crate::client::fluent_builders::DeleteSafetyRule).
            ///
            /// `ParseStrictResponse` impl for `DeleteSafetyRule`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteSafetyRule {
    _private: ()
}
impl DeleteSafetyRule {
    /// Creates a new builder-style object to manufacture [`DeleteSafetyRuleInput`](crate::input::DeleteSafetyRuleInput).
    pub fn builder() -> crate::input::delete_safety_rule_input::Builder {
        crate::input::delete_safety_rule_input::Builder::default()
    }
    /// Creates a new `DeleteSafetyRule` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteSafetyRule {
                type Output = std::result::Result<crate::output::DeleteSafetyRuleOutput, crate::error::DeleteSafetyRuleError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_safety_rule_error(response)
                     } else {
                        crate::operation_deser::parse_delete_safety_rule_response(response)
                     }
                }
            }

/// Operation shape for `DescribeCluster`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_cluster`](crate::client::fluent_builders::DescribeCluster).
            ///
            /// `ParseStrictResponse` impl for `DescribeCluster`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeCluster {
    _private: ()
}
impl DescribeCluster {
    /// Creates a new builder-style object to manufacture [`DescribeClusterInput`](crate::input::DescribeClusterInput).
    pub fn builder() -> crate::input::describe_cluster_input::Builder {
        crate::input::describe_cluster_input::Builder::default()
    }
    /// Creates a new `DescribeCluster` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeCluster {
                type Output = std::result::Result<crate::output::DescribeClusterOutput, crate::error::DescribeClusterError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_cluster_error(response)
                     } else {
                        crate::operation_deser::parse_describe_cluster_response(response)
                     }
                }
            }

/// Operation shape for `DescribeControlPanel`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_control_panel`](crate::client::fluent_builders::DescribeControlPanel).
            ///
            /// `ParseStrictResponse` impl for `DescribeControlPanel`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeControlPanel {
    _private: ()
}
impl DescribeControlPanel {
    /// Creates a new builder-style object to manufacture [`DescribeControlPanelInput`](crate::input::DescribeControlPanelInput).
    pub fn builder() -> crate::input::describe_control_panel_input::Builder {
        crate::input::describe_control_panel_input::Builder::default()
    }
    /// Creates a new `DescribeControlPanel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeControlPanel {
                type Output = std::result::Result<crate::output::DescribeControlPanelOutput, crate::error::DescribeControlPanelError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_control_panel_error(response)
                     } else {
                        crate::operation_deser::parse_describe_control_panel_response(response)
                     }
                }
            }

/// Operation shape for `DescribeRoutingControl`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_routing_control`](crate::client::fluent_builders::DescribeRoutingControl).
            ///
            /// `ParseStrictResponse` impl for `DescribeRoutingControl`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeRoutingControl {
    _private: ()
}
impl DescribeRoutingControl {
    /// Creates a new builder-style object to manufacture [`DescribeRoutingControlInput`](crate::input::DescribeRoutingControlInput).
    pub fn builder() -> crate::input::describe_routing_control_input::Builder {
        crate::input::describe_routing_control_input::Builder::default()
    }
    /// Creates a new `DescribeRoutingControl` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeRoutingControl {
                type Output = std::result::Result<crate::output::DescribeRoutingControlOutput, crate::error::DescribeRoutingControlError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_routing_control_error(response)
                     } else {
                        crate::operation_deser::parse_describe_routing_control_response(response)
                     }
                }
            }

/// Operation shape for `DescribeSafetyRule`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_safety_rule`](crate::client::fluent_builders::DescribeSafetyRule).
            ///
            /// `ParseStrictResponse` impl for `DescribeSafetyRule`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeSafetyRule {
    _private: ()
}
impl DescribeSafetyRule {
    /// Creates a new builder-style object to manufacture [`DescribeSafetyRuleInput`](crate::input::DescribeSafetyRuleInput).
    pub fn builder() -> crate::input::describe_safety_rule_input::Builder {
        crate::input::describe_safety_rule_input::Builder::default()
    }
    /// Creates a new `DescribeSafetyRule` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeSafetyRule {
                type Output = std::result::Result<crate::output::DescribeSafetyRuleOutput, crate::error::DescribeSafetyRuleError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_safety_rule_error(response)
                     } else {
                        crate::operation_deser::parse_describe_safety_rule_response(response)
                     }
                }
            }

/// Operation shape for `ListAssociatedRoute53HealthChecks`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_associated_route53_health_checks`](crate::client::fluent_builders::ListAssociatedRoute53HealthChecks).
            ///
            /// `ParseStrictResponse` impl for `ListAssociatedRoute53HealthChecks`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListAssociatedRoute53HealthChecks {
    _private: ()
}
impl ListAssociatedRoute53HealthChecks {
    /// Creates a new builder-style object to manufacture [`ListAssociatedRoute53HealthChecksInput`](crate::input::ListAssociatedRoute53HealthChecksInput).
    pub fn builder() -> crate::input::list_associated_route53_health_checks_input::Builder {
        crate::input::list_associated_route53_health_checks_input::Builder::default()
    }
    /// Creates a new `ListAssociatedRoute53HealthChecks` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAssociatedRoute53HealthChecks {
                type Output = std::result::Result<crate::output::ListAssociatedRoute53HealthChecksOutput, crate::error::ListAssociatedRoute53HealthChecksError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_associated_route53_health_checks_error(response)
                     } else {
                        crate::operation_deser::parse_list_associated_route53_health_checks_response(response)
                     }
                }
            }

/// Operation shape for `ListClusters`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_clusters`](crate::client::fluent_builders::ListClusters).
            ///
            /// `ParseStrictResponse` impl for `ListClusters`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListClusters {
    _private: ()
}
impl ListClusters {
    /// Creates a new builder-style object to manufacture [`ListClustersInput`](crate::input::ListClustersInput).
    pub fn builder() -> crate::input::list_clusters_input::Builder {
        crate::input::list_clusters_input::Builder::default()
    }
    /// Creates a new `ListClusters` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListClusters {
                type Output = std::result::Result<crate::output::ListClustersOutput, crate::error::ListClustersError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_clusters_error(response)
                     } else {
                        crate::operation_deser::parse_list_clusters_response(response)
                     }
                }
            }

/// Operation shape for `ListControlPanels`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_control_panels`](crate::client::fluent_builders::ListControlPanels).
            ///
            /// `ParseStrictResponse` impl for `ListControlPanels`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListControlPanels {
    _private: ()
}
impl ListControlPanels {
    /// Creates a new builder-style object to manufacture [`ListControlPanelsInput`](crate::input::ListControlPanelsInput).
    pub fn builder() -> crate::input::list_control_panels_input::Builder {
        crate::input::list_control_panels_input::Builder::default()
    }
    /// Creates a new `ListControlPanels` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListControlPanels {
                type Output = std::result::Result<crate::output::ListControlPanelsOutput, crate::error::ListControlPanelsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_control_panels_error(response)
                     } else {
                        crate::operation_deser::parse_list_control_panels_response(response)
                     }
                }
            }

/// Operation shape for `ListRoutingControls`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_routing_controls`](crate::client::fluent_builders::ListRoutingControls).
            ///
            /// `ParseStrictResponse` impl for `ListRoutingControls`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListRoutingControls {
    _private: ()
}
impl ListRoutingControls {
    /// Creates a new builder-style object to manufacture [`ListRoutingControlsInput`](crate::input::ListRoutingControlsInput).
    pub fn builder() -> crate::input::list_routing_controls_input::Builder {
        crate::input::list_routing_controls_input::Builder::default()
    }
    /// Creates a new `ListRoutingControls` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListRoutingControls {
                type Output = std::result::Result<crate::output::ListRoutingControlsOutput, crate::error::ListRoutingControlsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_routing_controls_error(response)
                     } else {
                        crate::operation_deser::parse_list_routing_controls_response(response)
                     }
                }
            }

/// Operation shape for `ListSafetyRules`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_safety_rules`](crate::client::fluent_builders::ListSafetyRules).
            ///
            /// `ParseStrictResponse` impl for `ListSafetyRules`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListSafetyRules {
    _private: ()
}
impl ListSafetyRules {
    /// Creates a new builder-style object to manufacture [`ListSafetyRulesInput`](crate::input::ListSafetyRulesInput).
    pub fn builder() -> crate::input::list_safety_rules_input::Builder {
        crate::input::list_safety_rules_input::Builder::default()
    }
    /// Creates a new `ListSafetyRules` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListSafetyRules {
                type Output = std::result::Result<crate::output::ListSafetyRulesOutput, crate::error::ListSafetyRulesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_safety_rules_error(response)
                     } else {
                        crate::operation_deser::parse_list_safety_rules_response(response)
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

/// Operation shape for `UpdateControlPanel`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_control_panel`](crate::client::fluent_builders::UpdateControlPanel).
            ///
            /// `ParseStrictResponse` impl for `UpdateControlPanel`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateControlPanel {
    _private: ()
}
impl UpdateControlPanel {
    /// Creates a new builder-style object to manufacture [`UpdateControlPanelInput`](crate::input::UpdateControlPanelInput).
    pub fn builder() -> crate::input::update_control_panel_input::Builder {
        crate::input::update_control_panel_input::Builder::default()
    }
    /// Creates a new `UpdateControlPanel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateControlPanel {
                type Output = std::result::Result<crate::output::UpdateControlPanelOutput, crate::error::UpdateControlPanelError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_control_panel_error(response)
                     } else {
                        crate::operation_deser::parse_update_control_panel_response(response)
                     }
                }
            }

/// Operation shape for `UpdateRoutingControl`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_routing_control`](crate::client::fluent_builders::UpdateRoutingControl).
            ///
            /// `ParseStrictResponse` impl for `UpdateRoutingControl`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateRoutingControl {
    _private: ()
}
impl UpdateRoutingControl {
    /// Creates a new builder-style object to manufacture [`UpdateRoutingControlInput`](crate::input::UpdateRoutingControlInput).
    pub fn builder() -> crate::input::update_routing_control_input::Builder {
        crate::input::update_routing_control_input::Builder::default()
    }
    /// Creates a new `UpdateRoutingControl` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateRoutingControl {
                type Output = std::result::Result<crate::output::UpdateRoutingControlOutput, crate::error::UpdateRoutingControlError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_routing_control_error(response)
                     } else {
                        crate::operation_deser::parse_update_routing_control_response(response)
                     }
                }
            }

/// Operation shape for `UpdateSafetyRule`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_safety_rule`](crate::client::fluent_builders::UpdateSafetyRule).
            ///
            /// `ParseStrictResponse` impl for `UpdateSafetyRule`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateSafetyRule {
    _private: ()
}
impl UpdateSafetyRule {
    /// Creates a new builder-style object to manufacture [`UpdateSafetyRuleInput`](crate::input::UpdateSafetyRuleInput).
    pub fn builder() -> crate::input::update_safety_rule_input::Builder {
        crate::input::update_safety_rule_input::Builder::default()
    }
    /// Creates a new `UpdateSafetyRule` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateSafetyRule {
                type Output = std::result::Result<crate::output::UpdateSafetyRuleOutput, crate::error::UpdateSafetyRuleError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_safety_rule_error(response)
                     } else {
                        crate::operation_deser::parse_update_safety_rule_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

