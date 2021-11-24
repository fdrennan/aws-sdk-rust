// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_associate_encryption_config_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateEncryptionConfigInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.client_request_token {
        object.key("clientRequestToken").string(var_1);
    }
    if let Some(var_2) = &input.encryption_config {
        let mut array_3 = object.key("encryptionConfig").start_array();
        for item_4 in var_2 {
            {
                let mut object_5 = array_3.value().start_object();
                crate::json_ser::serialize_structure_crate_model_encryption_config(
                    &mut object_5,
                    item_4,
                )?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_associate_identity_provider_config_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateIdentityProviderConfigInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_6) = &input.client_request_token {
        object.key("clientRequestToken").string(var_6);
    }
    if let Some(var_7) = &input.oidc {
        let mut object_8 = object.key("oidc").start_object();
        crate::json_ser::serialize_structure_crate_model_oidc_identity_provider_config_request(
            &mut object_8,
            var_7,
        )?;
        object_8.finish();
    }
    if let Some(var_9) = &input.tags {
        let mut object_10 = object.key("tags").start_object();
        for (key_11, value_12) in var_9 {
            {
                object_10.key(key_11).string(value_12);
            }
        }
        object_10.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_addon_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAddonInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_13) = &input.addon_name {
        object.key("addonName").string(var_13);
    }
    if let Some(var_14) = &input.addon_version {
        object.key("addonVersion").string(var_14);
    }
    if let Some(var_15) = &input.client_request_token {
        object.key("clientRequestToken").string(var_15);
    }
    if let Some(var_16) = &input.resolve_conflicts {
        object.key("resolveConflicts").string(var_16.as_str());
    }
    if let Some(var_17) = &input.service_account_role_arn {
        object.key("serviceAccountRoleArn").string(var_17);
    }
    if let Some(var_18) = &input.tags {
        let mut object_19 = object.key("tags").start_object();
        for (key_20, value_21) in var_18 {
            {
                object_19.key(key_20).string(value_21);
            }
        }
        object_19.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_cluster_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateClusterInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_22) = &input.client_request_token {
        object.key("clientRequestToken").string(var_22);
    }
    if let Some(var_23) = &input.encryption_config {
        let mut array_24 = object.key("encryptionConfig").start_array();
        for item_25 in var_23 {
            {
                let mut object_26 = array_24.value().start_object();
                crate::json_ser::serialize_structure_crate_model_encryption_config(
                    &mut object_26,
                    item_25,
                )?;
                object_26.finish();
            }
        }
        array_24.finish();
    }
    if let Some(var_27) = &input.kubernetes_network_config {
        let mut object_28 = object.key("kubernetesNetworkConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_kubernetes_network_config_request(
            &mut object_28,
            var_27,
        )?;
        object_28.finish();
    }
    if let Some(var_29) = &input.logging {
        let mut object_30 = object.key("logging").start_object();
        crate::json_ser::serialize_structure_crate_model_logging(&mut object_30, var_29)?;
        object_30.finish();
    }
    if let Some(var_31) = &input.name {
        object.key("name").string(var_31);
    }
    if let Some(var_32) = &input.resources_vpc_config {
        let mut object_33 = object.key("resourcesVpcConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_vpc_config_request(
            &mut object_33,
            var_32,
        )?;
        object_33.finish();
    }
    if let Some(var_34) = &input.role_arn {
        object.key("roleArn").string(var_34);
    }
    if let Some(var_35) = &input.tags {
        let mut object_36 = object.key("tags").start_object();
        for (key_37, value_38) in var_35 {
            {
                object_36.key(key_37).string(value_38);
            }
        }
        object_36.finish();
    }
    if let Some(var_39) = &input.version {
        object.key("version").string(var_39);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_fargate_profile_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateFargateProfileInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_40) = &input.client_request_token {
        object.key("clientRequestToken").string(var_40);
    }
    if let Some(var_41) = &input.fargate_profile_name {
        object.key("fargateProfileName").string(var_41);
    }
    if let Some(var_42) = &input.pod_execution_role_arn {
        object.key("podExecutionRoleArn").string(var_42);
    }
    if let Some(var_43) = &input.selectors {
        let mut array_44 = object.key("selectors").start_array();
        for item_45 in var_43 {
            {
                let mut object_46 = array_44.value().start_object();
                crate::json_ser::serialize_structure_crate_model_fargate_profile_selector(
                    &mut object_46,
                    item_45,
                )?;
                object_46.finish();
            }
        }
        array_44.finish();
    }
    if let Some(var_47) = &input.subnets {
        let mut array_48 = object.key("subnets").start_array();
        for item_49 in var_47 {
            {
                array_48.value().string(item_49);
            }
        }
        array_48.finish();
    }
    if let Some(var_50) = &input.tags {
        let mut object_51 = object.key("tags").start_object();
        for (key_52, value_53) in var_50 {
            {
                object_51.key(key_52).string(value_53);
            }
        }
        object_51.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_nodegroup_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateNodegroupInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_54) = &input.ami_type {
        object.key("amiType").string(var_54.as_str());
    }
    if let Some(var_55) = &input.capacity_type {
        object.key("capacityType").string(var_55.as_str());
    }
    if let Some(var_56) = &input.client_request_token {
        object.key("clientRequestToken").string(var_56);
    }
    if let Some(var_57) = &input.disk_size {
        object.key("diskSize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_57).into()),
        );
    }
    if let Some(var_58) = &input.instance_types {
        let mut array_59 = object.key("instanceTypes").start_array();
        for item_60 in var_58 {
            {
                array_59.value().string(item_60);
            }
        }
        array_59.finish();
    }
    if let Some(var_61) = &input.labels {
        let mut object_62 = object.key("labels").start_object();
        for (key_63, value_64) in var_61 {
            {
                object_62.key(key_63).string(value_64);
            }
        }
        object_62.finish();
    }
    if let Some(var_65) = &input.launch_template {
        let mut object_66 = object.key("launchTemplate").start_object();
        crate::json_ser::serialize_structure_crate_model_launch_template_specification(
            &mut object_66,
            var_65,
        )?;
        object_66.finish();
    }
    if let Some(var_67) = &input.node_role {
        object.key("nodeRole").string(var_67);
    }
    if let Some(var_68) = &input.nodegroup_name {
        object.key("nodegroupName").string(var_68);
    }
    if let Some(var_69) = &input.release_version {
        object.key("releaseVersion").string(var_69);
    }
    if let Some(var_70) = &input.remote_access {
        let mut object_71 = object.key("remoteAccess").start_object();
        crate::json_ser::serialize_structure_crate_model_remote_access_config(
            &mut object_71,
            var_70,
        )?;
        object_71.finish();
    }
    if let Some(var_72) = &input.scaling_config {
        let mut object_73 = object.key("scalingConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_nodegroup_scaling_config(
            &mut object_73,
            var_72,
        )?;
        object_73.finish();
    }
    if let Some(var_74) = &input.subnets {
        let mut array_75 = object.key("subnets").start_array();
        for item_76 in var_74 {
            {
                array_75.value().string(item_76);
            }
        }
        array_75.finish();
    }
    if let Some(var_77) = &input.tags {
        let mut object_78 = object.key("tags").start_object();
        for (key_79, value_80) in var_77 {
            {
                object_78.key(key_79).string(value_80);
            }
        }
        object_78.finish();
    }
    if let Some(var_81) = &input.taints {
        let mut array_82 = object.key("taints").start_array();
        for item_83 in var_81 {
            {
                let mut object_84 = array_82.value().start_object();
                crate::json_ser::serialize_structure_crate_model_taint(&mut object_84, item_83)?;
                object_84.finish();
            }
        }
        array_82.finish();
    }
    if let Some(var_85) = &input.update_config {
        let mut object_86 = object.key("updateConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_nodegroup_update_config(
            &mut object_86,
            var_85,
        )?;
        object_86.finish();
    }
    if let Some(var_87) = &input.version {
        object.key("version").string(var_87);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_identity_provider_config_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeIdentityProviderConfigInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_88) = &input.identity_provider_config {
        let mut object_89 = object.key("identityProviderConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_identity_provider_config(
            &mut object_89,
            var_88,
        )?;
        object_89.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_disassociate_identity_provider_config_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisassociateIdentityProviderConfigInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_90) = &input.client_request_token {
        object.key("clientRequestToken").string(var_90);
    }
    if let Some(var_91) = &input.identity_provider_config {
        let mut object_92 = object.key("identityProviderConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_identity_provider_config(
            &mut object_92,
            var_91,
        )?;
        object_92.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_register_cluster_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RegisterClusterInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_93) = &input.client_request_token {
        object.key("clientRequestToken").string(var_93);
    }
    if let Some(var_94) = &input.connector_config {
        let mut object_95 = object.key("connectorConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_connector_config_request(
            &mut object_95,
            var_94,
        )?;
        object_95.finish();
    }
    if let Some(var_96) = &input.name {
        object.key("name").string(var_96);
    }
    if let Some(var_97) = &input.tags {
        let mut object_98 = object.key("tags").start_object();
        for (key_99, value_100) in var_97 {
            {
                object_98.key(key_99).string(value_100);
            }
        }
        object_98.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_101) = &input.tags {
        let mut object_102 = object.key("tags").start_object();
        for (key_103, value_104) in var_101 {
            {
                object_102.key(key_103).string(value_104);
            }
        }
        object_102.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_addon_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateAddonInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_105) = &input.addon_version {
        object.key("addonVersion").string(var_105);
    }
    if let Some(var_106) = &input.client_request_token {
        object.key("clientRequestToken").string(var_106);
    }
    if let Some(var_107) = &input.resolve_conflicts {
        object.key("resolveConflicts").string(var_107.as_str());
    }
    if let Some(var_108) = &input.service_account_role_arn {
        object.key("serviceAccountRoleArn").string(var_108);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_cluster_config_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateClusterConfigInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_109) = &input.client_request_token {
        object.key("clientRequestToken").string(var_109);
    }
    if let Some(var_110) = &input.logging {
        let mut object_111 = object.key("logging").start_object();
        crate::json_ser::serialize_structure_crate_model_logging(&mut object_111, var_110)?;
        object_111.finish();
    }
    if let Some(var_112) = &input.resources_vpc_config {
        let mut object_113 = object.key("resourcesVpcConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_vpc_config_request(
            &mut object_113,
            var_112,
        )?;
        object_113.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_cluster_version_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateClusterVersionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_114) = &input.client_request_token {
        object.key("clientRequestToken").string(var_114);
    }
    if let Some(var_115) = &input.version {
        object.key("version").string(var_115);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_nodegroup_config_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateNodegroupConfigInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_116) = &input.client_request_token {
        object.key("clientRequestToken").string(var_116);
    }
    if let Some(var_117) = &input.labels {
        let mut object_118 = object.key("labels").start_object();
        crate::json_ser::serialize_structure_crate_model_update_labels_payload(
            &mut object_118,
            var_117,
        )?;
        object_118.finish();
    }
    if let Some(var_119) = &input.scaling_config {
        let mut object_120 = object.key("scalingConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_nodegroup_scaling_config(
            &mut object_120,
            var_119,
        )?;
        object_120.finish();
    }
    if let Some(var_121) = &input.taints {
        let mut object_122 = object.key("taints").start_object();
        crate::json_ser::serialize_structure_crate_model_update_taints_payload(
            &mut object_122,
            var_121,
        )?;
        object_122.finish();
    }
    if let Some(var_123) = &input.update_config {
        let mut object_124 = object.key("updateConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_nodegroup_update_config(
            &mut object_124,
            var_123,
        )?;
        object_124.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_nodegroup_version_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateNodegroupVersionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_125) = &input.client_request_token {
        object.key("clientRequestToken").string(var_125);
    }
    if input.force {
        object.key("force").boolean(input.force);
    }
    if let Some(var_126) = &input.launch_template {
        let mut object_127 = object.key("launchTemplate").start_object();
        crate::json_ser::serialize_structure_crate_model_launch_template_specification(
            &mut object_127,
            var_126,
        )?;
        object_127.finish();
    }
    if let Some(var_128) = &input.release_version {
        object.key("releaseVersion").string(var_128);
    }
    if let Some(var_129) = &input.version {
        object.key("version").string(var_129);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_encryption_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EncryptionConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_130) = &input.resources {
        let mut array_131 = object.key("resources").start_array();
        for item_132 in var_130 {
            {
                array_131.value().string(item_132);
            }
        }
        array_131.finish();
    }
    if let Some(var_133) = &input.provider {
        let mut object_134 = object.key("provider").start_object();
        crate::json_ser::serialize_structure_crate_model_provider(&mut object_134, var_133)?;
        object_134.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_oidc_identity_provider_config_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OidcIdentityProviderConfigRequest,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_135) = &input.identity_provider_config_name {
        object.key("identityProviderConfigName").string(var_135);
    }
    if let Some(var_136) = &input.issuer_url {
        object.key("issuerUrl").string(var_136);
    }
    if let Some(var_137) = &input.client_id {
        object.key("clientId").string(var_137);
    }
    if let Some(var_138) = &input.username_claim {
        object.key("usernameClaim").string(var_138);
    }
    if let Some(var_139) = &input.username_prefix {
        object.key("usernamePrefix").string(var_139);
    }
    if let Some(var_140) = &input.groups_claim {
        object.key("groupsClaim").string(var_140);
    }
    if let Some(var_141) = &input.groups_prefix {
        object.key("groupsPrefix").string(var_141);
    }
    if let Some(var_142) = &input.required_claims {
        let mut object_143 = object.key("requiredClaims").start_object();
        for (key_144, value_145) in var_142 {
            {
                object_143.key(key_144).string(value_145);
            }
        }
        object_143.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_kubernetes_network_config_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::KubernetesNetworkConfigRequest,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_146) = &input.service_ipv4_cidr {
        object.key("serviceIpv4Cidr").string(var_146);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_logging(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Logging,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_147) = &input.cluster_logging {
        let mut array_148 = object.key("clusterLogging").start_array();
        for item_149 in var_147 {
            {
                let mut object_150 = array_148.value().start_object();
                crate::json_ser::serialize_structure_crate_model_log_setup(
                    &mut object_150,
                    item_149,
                )?;
                object_150.finish();
            }
        }
        array_148.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_vpc_config_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::VpcConfigRequest,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_151) = &input.subnet_ids {
        let mut array_152 = object.key("subnetIds").start_array();
        for item_153 in var_151 {
            {
                array_152.value().string(item_153);
            }
        }
        array_152.finish();
    }
    if let Some(var_154) = &input.security_group_ids {
        let mut array_155 = object.key("securityGroupIds").start_array();
        for item_156 in var_154 {
            {
                array_155.value().string(item_156);
            }
        }
        array_155.finish();
    }
    if let Some(var_157) = &input.endpoint_public_access {
        object.key("endpointPublicAccess").boolean(*var_157);
    }
    if let Some(var_158) = &input.endpoint_private_access {
        object.key("endpointPrivateAccess").boolean(*var_158);
    }
    if let Some(var_159) = &input.public_access_cidrs {
        let mut array_160 = object.key("publicAccessCidrs").start_array();
        for item_161 in var_159 {
            {
                array_160.value().string(item_161);
            }
        }
        array_160.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_fargate_profile_selector(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FargateProfileSelector,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_162) = &input.namespace {
        object.key("namespace").string(var_162);
    }
    if let Some(var_163) = &input.labels {
        let mut object_164 = object.key("labels").start_object();
        for (key_165, value_166) in var_163 {
            {
                object_164.key(key_165).string(value_166);
            }
        }
        object_164.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_launch_template_specification(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LaunchTemplateSpecification,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_167) = &input.name {
        object.key("name").string(var_167);
    }
    if let Some(var_168) = &input.version {
        object.key("version").string(var_168);
    }
    if let Some(var_169) = &input.id {
        object.key("id").string(var_169);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_remote_access_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RemoteAccessConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_170) = &input.ec2_ssh_key {
        object.key("ec2SshKey").string(var_170);
    }
    if let Some(var_171) = &input.source_security_groups {
        let mut array_172 = object.key("sourceSecurityGroups").start_array();
        for item_173 in var_171 {
            {
                array_172.value().string(item_173);
            }
        }
        array_172.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_nodegroup_scaling_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NodegroupScalingConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_174) = &input.min_size {
        object.key("minSize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_174).into()),
        );
    }
    if let Some(var_175) = &input.max_size {
        object.key("maxSize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_175).into()),
        );
    }
    if let Some(var_176) = &input.desired_size {
        object.key("desiredSize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_176).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_taint(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Taint,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_177) = &input.key {
        object.key("key").string(var_177);
    }
    if let Some(var_178) = &input.value {
        object.key("value").string(var_178);
    }
    if let Some(var_179) = &input.effect {
        object.key("effect").string(var_179.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_nodegroup_update_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NodegroupUpdateConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_180) = &input.max_unavailable {
        object.key("maxUnavailable").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_180).into()),
        );
    }
    if let Some(var_181) = &input.max_unavailable_percentage {
        object.key("maxUnavailablePercentage").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_181).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_identity_provider_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IdentityProviderConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_182) = &input.r#type {
        object.key("type").string(var_182);
    }
    if let Some(var_183) = &input.name {
        object.key("name").string(var_183);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_connector_config_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ConnectorConfigRequest,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_184) = &input.role_arn {
        object.key("roleArn").string(var_184);
    }
    if let Some(var_185) = &input.provider {
        object.key("provider").string(var_185.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_update_labels_payload(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateLabelsPayload,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_186) = &input.add_or_update_labels {
        let mut object_187 = object.key("addOrUpdateLabels").start_object();
        for (key_188, value_189) in var_186 {
            {
                object_187.key(key_188).string(value_189);
            }
        }
        object_187.finish();
    }
    if let Some(var_190) = &input.remove_labels {
        let mut array_191 = object.key("removeLabels").start_array();
        for item_192 in var_190 {
            {
                array_191.value().string(item_192);
            }
        }
        array_191.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_update_taints_payload(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::UpdateTaintsPayload,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_193) = &input.add_or_update_taints {
        let mut array_194 = object.key("addOrUpdateTaints").start_array();
        for item_195 in var_193 {
            {
                let mut object_196 = array_194.value().start_object();
                crate::json_ser::serialize_structure_crate_model_taint(&mut object_196, item_195)?;
                object_196.finish();
            }
        }
        array_194.finish();
    }
    if let Some(var_197) = &input.remove_taints {
        let mut array_198 = object.key("removeTaints").start_array();
        for item_199 in var_197 {
            {
                let mut object_200 = array_198.value().start_object();
                crate::json_ser::serialize_structure_crate_model_taint(&mut object_200, item_199)?;
                object_200.finish();
            }
        }
        array_198.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_provider(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Provider,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_201) = &input.key_arn {
        object.key("keyArn").string(var_201);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_log_setup(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LogSetup,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_202) = &input.types {
        let mut array_203 = object.key("types").start_array();
        for item_204 in var_202 {
            {
                array_203.value().string(item_204.as_str());
            }
        }
        array_203.finish();
    }
    if let Some(var_205) = &input.enabled {
        object.key("enabled").boolean(*var_205);
    }
    Ok(())
}
