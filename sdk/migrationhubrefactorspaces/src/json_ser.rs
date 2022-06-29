// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_application_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateApplicationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.api_gateway_proxy {
        let mut object_2 = object.key("ApiGatewayProxy").start_object();
        crate::json_ser::serialize_structure_crate_model_api_gateway_proxy_input(
            &mut object_2,
            var_1,
        )?;
        object_2.finish();
    }
    if let Some(var_3) = &input.client_token {
        object.key("ClientToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.name {
        object.key("Name").string(var_4.as_str());
    }
    if let Some(var_5) = &input.proxy_type {
        object.key("ProxyType").string(var_5.as_str());
    }
    if let Some(var_6) = &input.tags {
        let mut object_7 = object.key("Tags").start_object();
        for (key_8, value_9) in var_6 {
            {
                object_7.key(key_8).string(value_9.as_str());
            }
        }
        object_7.finish();
    }
    if let Some(var_10) = &input.vpc_id {
        object.key("VpcId").string(var_10.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_environment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateEnvironmentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_11) = &input.client_token {
        object.key("ClientToken").string(var_11.as_str());
    }
    if let Some(var_12) = &input.description {
        object.key("Description").string(var_12.as_str());
    }
    if let Some(var_13) = &input.name {
        object.key("Name").string(var_13.as_str());
    }
    if let Some(var_14) = &input.network_fabric_type {
        object.key("NetworkFabricType").string(var_14.as_str());
    }
    if let Some(var_15) = &input.tags {
        let mut object_16 = object.key("Tags").start_object();
        for (key_17, value_18) in var_15 {
            {
                object_16.key(key_17).string(value_18.as_str());
            }
        }
        object_16.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_route_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateRouteInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_19) = &input.client_token {
        object.key("ClientToken").string(var_19.as_str());
    }
    if let Some(var_20) = &input.default_route {
        let mut object_21 = object.key("DefaultRoute").start_object();
        crate::json_ser::serialize_structure_crate_model_default_route_input(
            &mut object_21,
            var_20,
        )?;
        object_21.finish();
    }
    if let Some(var_22) = &input.route_type {
        object.key("RouteType").string(var_22.as_str());
    }
    if let Some(var_23) = &input.service_identifier {
        object.key("ServiceIdentifier").string(var_23.as_str());
    }
    if let Some(var_24) = &input.tags {
        let mut object_25 = object.key("Tags").start_object();
        for (key_26, value_27) in var_24 {
            {
                object_25.key(key_26).string(value_27.as_str());
            }
        }
        object_25.finish();
    }
    if let Some(var_28) = &input.uri_path_route {
        let mut object_29 = object.key("UriPathRoute").start_object();
        crate::json_ser::serialize_structure_crate_model_uri_path_route_input(
            &mut object_29,
            var_28,
        )?;
        object_29.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_service_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateServiceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_30) = &input.client_token {
        object.key("ClientToken").string(var_30.as_str());
    }
    if let Some(var_31) = &input.description {
        object.key("Description").string(var_31.as_str());
    }
    if let Some(var_32) = &input.endpoint_type {
        object.key("EndpointType").string(var_32.as_str());
    }
    if let Some(var_33) = &input.lambda_endpoint {
        let mut object_34 = object.key("LambdaEndpoint").start_object();
        crate::json_ser::serialize_structure_crate_model_lambda_endpoint_input(
            &mut object_34,
            var_33,
        )?;
        object_34.finish();
    }
    if let Some(var_35) = &input.name {
        object.key("Name").string(var_35.as_str());
    }
    if let Some(var_36) = &input.tags {
        let mut object_37 = object.key("Tags").start_object();
        for (key_38, value_39) in var_36 {
            {
                object_37.key(key_38).string(value_39.as_str());
            }
        }
        object_37.finish();
    }
    if let Some(var_40) = &input.url_endpoint {
        let mut object_41 = object.key("UrlEndpoint").start_object();
        crate::json_ser::serialize_structure_crate_model_url_endpoint_input(
            &mut object_41,
            var_40,
        )?;
        object_41.finish();
    }
    if let Some(var_42) = &input.vpc_id {
        object.key("VpcId").string(var_42.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_resource_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutResourcePolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_43) = &input.policy {
        object.key("Policy").string(var_43.as_str());
    }
    if let Some(var_44) = &input.resource_arn {
        object.key("ResourceArn").string(var_44.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_45) = &input.tags {
        let mut object_46 = object.key("Tags").start_object();
        for (key_47, value_48) in var_45 {
            {
                object_46.key(key_47).string(value_48.as_str());
            }
        }
        object_46.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_route_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateRouteInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_49) = &input.activation_state {
        object.key("ActivationState").string(var_49.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_api_gateway_proxy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ApiGatewayProxyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_50) = &input.endpoint_type {
        object.key("EndpointType").string(var_50.as_str());
    }
    if let Some(var_51) = &input.stage_name {
        object.key("StageName").string(var_51.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_default_route_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DefaultRouteInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_52) = &input.activation_state {
        object.key("ActivationState").string(var_52.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_uri_path_route_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UriPathRouteInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_53) = &input.source_path {
        object.key("SourcePath").string(var_53.as_str());
    }
    if let Some(var_54) = &input.activation_state {
        object.key("ActivationState").string(var_54.as_str());
    }
    if let Some(var_55) = &input.methods {
        let mut array_56 = object.key("Methods").start_array();
        for item_57 in var_55 {
            {
                array_56.value().string(item_57.as_str());
            }
        }
        array_56.finish();
    }
    if let Some(var_58) = &input.include_child_paths {
        object.key("IncludeChildPaths").boolean(*var_58);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lambda_endpoint_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LambdaEndpointInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_59) = &input.arn {
        object.key("Arn").string(var_59.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_url_endpoint_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UrlEndpointInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_60) = &input.url {
        object.key("Url").string(var_60.as_str());
    }
    if let Some(var_61) = &input.health_url {
        object.key("HealthUrl").string(var_61.as_str());
    }
    Ok(())
}
