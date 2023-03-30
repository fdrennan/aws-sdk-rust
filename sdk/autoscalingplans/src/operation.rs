// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateScalingPlan`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_scaling_plan`](crate::client::fluent_builders::CreateScalingPlan).
            ///
            /// `ParseStrictResponse` impl for `CreateScalingPlan`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateScalingPlan {
    _private: ()
}
impl CreateScalingPlan {
    /// Creates a new builder-style object to manufacture [`CreateScalingPlanInput`](crate::input::CreateScalingPlanInput).
    pub fn builder() -> crate::input::create_scaling_plan_input::Builder {
        crate::input::create_scaling_plan_input::Builder::default()
    }
    /// Creates a new `CreateScalingPlan` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateScalingPlan {
                type Output = std::result::Result<crate::output::CreateScalingPlanOutput, crate::error::CreateScalingPlanError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_scaling_plan_error(response)
                     } else {
                        crate::operation_deser::parse_create_scaling_plan_response(response)
                     }
                }
            }

/// Operation shape for `DeleteScalingPlan`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_scaling_plan`](crate::client::fluent_builders::DeleteScalingPlan).
            ///
            /// `ParseStrictResponse` impl for `DeleteScalingPlan`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteScalingPlan {
    _private: ()
}
impl DeleteScalingPlan {
    /// Creates a new builder-style object to manufacture [`DeleteScalingPlanInput`](crate::input::DeleteScalingPlanInput).
    pub fn builder() -> crate::input::delete_scaling_plan_input::Builder {
        crate::input::delete_scaling_plan_input::Builder::default()
    }
    /// Creates a new `DeleteScalingPlan` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteScalingPlan {
                type Output = std::result::Result<crate::output::DeleteScalingPlanOutput, crate::error::DeleteScalingPlanError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_scaling_plan_error(response)
                     } else {
                        crate::operation_deser::parse_delete_scaling_plan_response(response)
                     }
                }
            }

/// Operation shape for `DescribeScalingPlanResources`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_scaling_plan_resources`](crate::client::fluent_builders::DescribeScalingPlanResources).
            ///
            /// `ParseStrictResponse` impl for `DescribeScalingPlanResources`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeScalingPlanResources {
    _private: ()
}
impl DescribeScalingPlanResources {
    /// Creates a new builder-style object to manufacture [`DescribeScalingPlanResourcesInput`](crate::input::DescribeScalingPlanResourcesInput).
    pub fn builder() -> crate::input::describe_scaling_plan_resources_input::Builder {
        crate::input::describe_scaling_plan_resources_input::Builder::default()
    }
    /// Creates a new `DescribeScalingPlanResources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeScalingPlanResources {
                type Output = std::result::Result<crate::output::DescribeScalingPlanResourcesOutput, crate::error::DescribeScalingPlanResourcesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_scaling_plan_resources_error(response)
                     } else {
                        crate::operation_deser::parse_describe_scaling_plan_resources_response(response)
                     }
                }
            }

/// Operation shape for `DescribeScalingPlans`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_scaling_plans`](crate::client::fluent_builders::DescribeScalingPlans).
            ///
            /// `ParseStrictResponse` impl for `DescribeScalingPlans`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeScalingPlans {
    _private: ()
}
impl DescribeScalingPlans {
    /// Creates a new builder-style object to manufacture [`DescribeScalingPlansInput`](crate::input::DescribeScalingPlansInput).
    pub fn builder() -> crate::input::describe_scaling_plans_input::Builder {
        crate::input::describe_scaling_plans_input::Builder::default()
    }
    /// Creates a new `DescribeScalingPlans` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeScalingPlans {
                type Output = std::result::Result<crate::output::DescribeScalingPlansOutput, crate::error::DescribeScalingPlansError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_scaling_plans_error(response)
                     } else {
                        crate::operation_deser::parse_describe_scaling_plans_response(response)
                     }
                }
            }

/// Operation shape for `GetScalingPlanResourceForecastData`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_scaling_plan_resource_forecast_data`](crate::client::fluent_builders::GetScalingPlanResourceForecastData).
            ///
            /// `ParseStrictResponse` impl for `GetScalingPlanResourceForecastData`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetScalingPlanResourceForecastData {
    _private: ()
}
impl GetScalingPlanResourceForecastData {
    /// Creates a new builder-style object to manufacture [`GetScalingPlanResourceForecastDataInput`](crate::input::GetScalingPlanResourceForecastDataInput).
    pub fn builder() -> crate::input::get_scaling_plan_resource_forecast_data_input::Builder {
        crate::input::get_scaling_plan_resource_forecast_data_input::Builder::default()
    }
    /// Creates a new `GetScalingPlanResourceForecastData` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetScalingPlanResourceForecastData {
                type Output = std::result::Result<crate::output::GetScalingPlanResourceForecastDataOutput, crate::error::GetScalingPlanResourceForecastDataError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_scaling_plan_resource_forecast_data_error(response)
                     } else {
                        crate::operation_deser::parse_get_scaling_plan_resource_forecast_data_response(response)
                     }
                }
            }

/// Operation shape for `UpdateScalingPlan`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_scaling_plan`](crate::client::fluent_builders::UpdateScalingPlan).
            ///
            /// `ParseStrictResponse` impl for `UpdateScalingPlan`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateScalingPlan {
    _private: ()
}
impl UpdateScalingPlan {
    /// Creates a new builder-style object to manufacture [`UpdateScalingPlanInput`](crate::input::UpdateScalingPlanInput).
    pub fn builder() -> crate::input::update_scaling_plan_input::Builder {
        crate::input::update_scaling_plan_input::Builder::default()
    }
    /// Creates a new `UpdateScalingPlan` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateScalingPlan {
                type Output = std::result::Result<crate::output::UpdateScalingPlanOutput, crate::error::UpdateScalingPlanError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_scaling_plan_error(response)
                     } else {
                        crate::operation_deser::parse_update_scaling_plan_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

