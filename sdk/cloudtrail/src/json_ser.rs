// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_add_tags_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddTagsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.resource_id {
        object.key("ResourceId").string(var_1);
    }
    if let Some(var_2) = &input.tags_list {
        let mut array_3 = object.key("TagsList").start_array();
        for item_4 in var_2 {
            {
                let mut object_5 = array_3.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_trail_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateTrailInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_6) = &input.name {
        object.key("Name").string(var_6);
    }
    if let Some(var_7) = &input.s3_bucket_name {
        object.key("S3BucketName").string(var_7);
    }
    if let Some(var_8) = &input.s3_key_prefix {
        object.key("S3KeyPrefix").string(var_8);
    }
    if let Some(var_9) = &input.sns_topic_name {
        object.key("SnsTopicName").string(var_9);
    }
    if let Some(var_10) = &input.include_global_service_events {
        object.key("IncludeGlobalServiceEvents").boolean(*var_10);
    }
    if let Some(var_11) = &input.is_multi_region_trail {
        object.key("IsMultiRegionTrail").boolean(*var_11);
    }
    if let Some(var_12) = &input.enable_log_file_validation {
        object.key("EnableLogFileValidation").boolean(*var_12);
    }
    if let Some(var_13) = &input.cloud_watch_logs_log_group_arn {
        object.key("CloudWatchLogsLogGroupArn").string(var_13);
    }
    if let Some(var_14) = &input.cloud_watch_logs_role_arn {
        object.key("CloudWatchLogsRoleArn").string(var_14);
    }
    if let Some(var_15) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_15);
    }
    if let Some(var_16) = &input.is_organization_trail {
        object.key("IsOrganizationTrail").boolean(*var_16);
    }
    if let Some(var_17) = &input.tags_list {
        let mut array_18 = object.key("TagsList").start_array();
        for item_19 in var_17 {
            {
                let mut object_20 = array_18.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_20, item_19)?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_trail_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteTrailInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_21) = &input.name {
        object.key("Name").string(var_21);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_trails_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeTrailsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_22) = &input.trail_name_list {
        let mut array_23 = object.key("trailNameList").start_array();
        for item_24 in var_22 {
            {
                array_23.value().string(item_24);
            }
        }
        array_23.finish();
    }
    if let Some(var_25) = &input.include_shadow_trails {
        object.key("includeShadowTrails").boolean(*var_25);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_event_selectors_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetEventSelectorsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_26) = &input.trail_name {
        object.key("TrailName").string(var_26);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_insight_selectors_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetInsightSelectorsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_27) = &input.trail_name {
        object.key("TrailName").string(var_27);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_trail_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetTrailInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_28) = &input.name {
        object.key("Name").string(var_28);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_trail_status_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetTrailStatusInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_29) = &input.name {
        object.key("Name").string(var_29);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_public_keys_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListPublicKeysInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_30) = &input.start_time {
        object
            .key("StartTime")
            .date_time(var_30, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_31) = &input.end_time {
        object
            .key("EndTime")
            .date_time(var_31, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_32) = &input.next_token {
        object.key("NextToken").string(var_32);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_33) = &input.resource_id_list {
        let mut array_34 = object.key("ResourceIdList").start_array();
        for item_35 in var_33 {
            {
                array_34.value().string(item_35);
            }
        }
        array_34.finish();
    }
    if let Some(var_36) = &input.next_token {
        object.key("NextToken").string(var_36);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_trails_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTrailsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_37) = &input.next_token {
        object.key("NextToken").string(var_37);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_lookup_events_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::LookupEventsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_38) = &input.lookup_attributes {
        let mut array_39 = object.key("LookupAttributes").start_array();
        for item_40 in var_38 {
            {
                let mut object_41 = array_39.value().start_object();
                crate::json_ser::serialize_structure_crate_model_lookup_attribute(
                    &mut object_41,
                    item_40,
                )?;
                object_41.finish();
            }
        }
        array_39.finish();
    }
    if let Some(var_42) = &input.start_time {
        object
            .key("StartTime")
            .date_time(var_42, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_43) = &input.end_time {
        object
            .key("EndTime")
            .date_time(var_43, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_44) = &input.event_category {
        object.key("EventCategory").string(var_44.as_str());
    }
    if let Some(var_45) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_45).into()),
        );
    }
    if let Some(var_46) = &input.next_token {
        object.key("NextToken").string(var_46);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_event_selectors_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutEventSelectorsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_47) = &input.trail_name {
        object.key("TrailName").string(var_47);
    }
    if let Some(var_48) = &input.event_selectors {
        let mut array_49 = object.key("EventSelectors").start_array();
        for item_50 in var_48 {
            {
                let mut object_51 = array_49.value().start_object();
                crate::json_ser::serialize_structure_crate_model_event_selector(
                    &mut object_51,
                    item_50,
                )?;
                object_51.finish();
            }
        }
        array_49.finish();
    }
    if let Some(var_52) = &input.advanced_event_selectors {
        let mut array_53 = object.key("AdvancedEventSelectors").start_array();
        for item_54 in var_52 {
            {
                let mut object_55 = array_53.value().start_object();
                crate::json_ser::serialize_structure_crate_model_advanced_event_selector(
                    &mut object_55,
                    item_54,
                )?;
                object_55.finish();
            }
        }
        array_53.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_insight_selectors_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutInsightSelectorsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_56) = &input.trail_name {
        object.key("TrailName").string(var_56);
    }
    if let Some(var_57) = &input.insight_selectors {
        let mut array_58 = object.key("InsightSelectors").start_array();
        for item_59 in var_57 {
            {
                let mut object_60 = array_58.value().start_object();
                crate::json_ser::serialize_structure_crate_model_insight_selector(
                    &mut object_60,
                    item_59,
                )?;
                object_60.finish();
            }
        }
        array_58.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_remove_tags_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RemoveTagsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_61) = &input.resource_id {
        object.key("ResourceId").string(var_61);
    }
    if let Some(var_62) = &input.tags_list {
        let mut array_63 = object.key("TagsList").start_array();
        for item_64 in var_62 {
            {
                let mut object_65 = array_63.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_65, item_64)?;
                object_65.finish();
            }
        }
        array_63.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_logging_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartLoggingInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_66) = &input.name {
        object.key("Name").string(var_66);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_stop_logging_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopLoggingInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_67) = &input.name {
        object.key("Name").string(var_67);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_trail_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateTrailInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_68) = &input.name {
        object.key("Name").string(var_68);
    }
    if let Some(var_69) = &input.s3_bucket_name {
        object.key("S3BucketName").string(var_69);
    }
    if let Some(var_70) = &input.s3_key_prefix {
        object.key("S3KeyPrefix").string(var_70);
    }
    if let Some(var_71) = &input.sns_topic_name {
        object.key("SnsTopicName").string(var_71);
    }
    if let Some(var_72) = &input.include_global_service_events {
        object.key("IncludeGlobalServiceEvents").boolean(*var_72);
    }
    if let Some(var_73) = &input.is_multi_region_trail {
        object.key("IsMultiRegionTrail").boolean(*var_73);
    }
    if let Some(var_74) = &input.enable_log_file_validation {
        object.key("EnableLogFileValidation").boolean(*var_74);
    }
    if let Some(var_75) = &input.cloud_watch_logs_log_group_arn {
        object.key("CloudWatchLogsLogGroupArn").string(var_75);
    }
    if let Some(var_76) = &input.cloud_watch_logs_role_arn {
        object.key("CloudWatchLogsRoleArn").string(var_76);
    }
    if let Some(var_77) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_77);
    }
    if let Some(var_78) = &input.is_organization_trail {
        object.key("IsOrganizationTrail").boolean(*var_78);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_79) = &input.key {
        object.key("Key").string(var_79);
    }
    if let Some(var_80) = &input.value {
        object.key("Value").string(var_80);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lookup_attribute(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LookupAttribute,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_81) = &input.attribute_key {
        object.key("AttributeKey").string(var_81.as_str());
    }
    if let Some(var_82) = &input.attribute_value {
        object.key("AttributeValue").string(var_82);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_event_selector(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::EventSelector,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_83) = &input.read_write_type {
        object.key("ReadWriteType").string(var_83.as_str());
    }
    if let Some(var_84) = &input.include_management_events {
        object.key("IncludeManagementEvents").boolean(*var_84);
    }
    if let Some(var_85) = &input.data_resources {
        let mut array_86 = object.key("DataResources").start_array();
        for item_87 in var_85 {
            {
                let mut object_88 = array_86.value().start_object();
                crate::json_ser::serialize_structure_crate_model_data_resource(
                    &mut object_88,
                    item_87,
                )?;
                object_88.finish();
            }
        }
        array_86.finish();
    }
    if let Some(var_89) = &input.exclude_management_event_sources {
        let mut array_90 = object.key("ExcludeManagementEventSources").start_array();
        for item_91 in var_89 {
            {
                array_90.value().string(item_91);
            }
        }
        array_90.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_advanced_event_selector(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AdvancedEventSelector,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_92) = &input.name {
        object.key("Name").string(var_92);
    }
    if let Some(var_93) = &input.field_selectors {
        let mut array_94 = object.key("FieldSelectors").start_array();
        for item_95 in var_93 {
            {
                let mut object_96 = array_94.value().start_object();
                crate::json_ser::serialize_structure_crate_model_advanced_field_selector(
                    &mut object_96,
                    item_95,
                )?;
                object_96.finish();
            }
        }
        array_94.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_insight_selector(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InsightSelector,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_97) = &input.insight_type {
        object.key("InsightType").string(var_97.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_data_resource(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DataResource,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_98) = &input.r#type {
        object.key("Type").string(var_98);
    }
    if let Some(var_99) = &input.values {
        let mut array_100 = object.key("Values").start_array();
        for item_101 in var_99 {
            {
                array_100.value().string(item_101);
            }
        }
        array_100.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_advanced_field_selector(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AdvancedFieldSelector,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_102) = &input.field {
        object.key("Field").string(var_102);
    }
    if let Some(var_103) = &input.equals {
        let mut array_104 = object.key("Equals").start_array();
        for item_105 in var_103 {
            {
                array_104.value().string(item_105);
            }
        }
        array_104.finish();
    }
    if let Some(var_106) = &input.starts_with {
        let mut array_107 = object.key("StartsWith").start_array();
        for item_108 in var_106 {
            {
                array_107.value().string(item_108);
            }
        }
        array_107.finish();
    }
    if let Some(var_109) = &input.ends_with {
        let mut array_110 = object.key("EndsWith").start_array();
        for item_111 in var_109 {
            {
                array_110.value().string(item_111);
            }
        }
        array_110.finish();
    }
    if let Some(var_112) = &input.not_equals {
        let mut array_113 = object.key("NotEquals").start_array();
        for item_114 in var_112 {
            {
                array_113.value().string(item_114);
            }
        }
        array_113.finish();
    }
    if let Some(var_115) = &input.not_starts_with {
        let mut array_116 = object.key("NotStartsWith").start_array();
        for item_117 in var_115 {
            {
                array_116.value().string(item_117);
            }
        }
        array_116.finish();
    }
    if let Some(var_118) = &input.not_ends_with {
        let mut array_119 = object.key("NotEndsWith").start_array();
        for item_120 in var_118 {
            {
                array_119.value().string(item_120);
            }
        }
        array_119.finish();
    }
    Ok(())
}
