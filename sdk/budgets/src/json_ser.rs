// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_budget_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateBudgetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.account_id {
        object.key("AccountId").string(var_1);
    }
    if let Some(var_2) = &input.budget {
        let mut object_3 = object.key("Budget").start_object();
        crate::json_ser::serialize_structure_crate_model_budget(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.notifications_with_subscribers {
        let mut array_5 = object.key("NotificationsWithSubscribers").start_array();
        for item_6 in var_4 {
            {
                let mut object_7 = array_5.value().start_object();
                crate::json_ser::serialize_structure_crate_model_notification_with_subscribers(
                    &mut object_7,
                    item_6,
                )?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_budget_action_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateBudgetActionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_8) = &input.account_id {
        object.key("AccountId").string(var_8);
    }
    if let Some(var_9) = &input.budget_name {
        object.key("BudgetName").string(var_9);
    }
    if let Some(var_10) = &input.notification_type {
        object.key("NotificationType").string(var_10.as_str());
    }
    if let Some(var_11) = &input.action_type {
        object.key("ActionType").string(var_11.as_str());
    }
    if let Some(var_12) = &input.action_threshold {
        let mut object_13 = object.key("ActionThreshold").start_object();
        crate::json_ser::serialize_structure_crate_model_action_threshold(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.definition {
        let mut object_15 = object.key("Definition").start_object();
        crate::json_ser::serialize_structure_crate_model_definition(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.execution_role_arn {
        object.key("ExecutionRoleArn").string(var_16);
    }
    if let Some(var_17) = &input.approval_model {
        object.key("ApprovalModel").string(var_17.as_str());
    }
    if let Some(var_18) = &input.subscribers {
        let mut array_19 = object.key("Subscribers").start_array();
        for item_20 in var_18 {
            {
                let mut object_21 = array_19.value().start_object();
                crate::json_ser::serialize_structure_crate_model_subscriber(
                    &mut object_21,
                    item_20,
                )?;
                object_21.finish();
            }
        }
        array_19.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_notification_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateNotificationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_22) = &input.account_id {
        object.key("AccountId").string(var_22);
    }
    if let Some(var_23) = &input.budget_name {
        object.key("BudgetName").string(var_23);
    }
    if let Some(var_24) = &input.notification {
        let mut object_25 = object.key("Notification").start_object();
        crate::json_ser::serialize_structure_crate_model_notification(&mut object_25, var_24)?;
        object_25.finish();
    }
    if let Some(var_26) = &input.subscribers {
        let mut array_27 = object.key("Subscribers").start_array();
        for item_28 in var_26 {
            {
                let mut object_29 = array_27.value().start_object();
                crate::json_ser::serialize_structure_crate_model_subscriber(
                    &mut object_29,
                    item_28,
                )?;
                object_29.finish();
            }
        }
        array_27.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_subscriber_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSubscriberInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_30) = &input.account_id {
        object.key("AccountId").string(var_30);
    }
    if let Some(var_31) = &input.budget_name {
        object.key("BudgetName").string(var_31);
    }
    if let Some(var_32) = &input.notification {
        let mut object_33 = object.key("Notification").start_object();
        crate::json_ser::serialize_structure_crate_model_notification(&mut object_33, var_32)?;
        object_33.finish();
    }
    if let Some(var_34) = &input.subscriber {
        let mut object_35 = object.key("Subscriber").start_object();
        crate::json_ser::serialize_structure_crate_model_subscriber(&mut object_35, var_34)?;
        object_35.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_budget_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteBudgetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_36) = &input.account_id {
        object.key("AccountId").string(var_36);
    }
    if let Some(var_37) = &input.budget_name {
        object.key("BudgetName").string(var_37);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_budget_action_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteBudgetActionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_38) = &input.account_id {
        object.key("AccountId").string(var_38);
    }
    if let Some(var_39) = &input.budget_name {
        object.key("BudgetName").string(var_39);
    }
    if let Some(var_40) = &input.action_id {
        object.key("ActionId").string(var_40);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_notification_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteNotificationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_41) = &input.account_id {
        object.key("AccountId").string(var_41);
    }
    if let Some(var_42) = &input.budget_name {
        object.key("BudgetName").string(var_42);
    }
    if let Some(var_43) = &input.notification {
        let mut object_44 = object.key("Notification").start_object();
        crate::json_ser::serialize_structure_crate_model_notification(&mut object_44, var_43)?;
        object_44.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_subscriber_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteSubscriberInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_45) = &input.account_id {
        object.key("AccountId").string(var_45);
    }
    if let Some(var_46) = &input.budget_name {
        object.key("BudgetName").string(var_46);
    }
    if let Some(var_47) = &input.notification {
        let mut object_48 = object.key("Notification").start_object();
        crate::json_ser::serialize_structure_crate_model_notification(&mut object_48, var_47)?;
        object_48.finish();
    }
    if let Some(var_49) = &input.subscriber {
        let mut object_50 = object.key("Subscriber").start_object();
        crate::json_ser::serialize_structure_crate_model_subscriber(&mut object_50, var_49)?;
        object_50.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_budget_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeBudgetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_51) = &input.account_id {
        object.key("AccountId").string(var_51);
    }
    if let Some(var_52) = &input.budget_name {
        object.key("BudgetName").string(var_52);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_budget_action_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeBudgetActionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_53) = &input.account_id {
        object.key("AccountId").string(var_53);
    }
    if let Some(var_54) = &input.budget_name {
        object.key("BudgetName").string(var_54);
    }
    if let Some(var_55) = &input.action_id {
        object.key("ActionId").string(var_55);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_budget_action_histories_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeBudgetActionHistoriesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_56) = &input.account_id {
        object.key("AccountId").string(var_56);
    }
    if let Some(var_57) = &input.budget_name {
        object.key("BudgetName").string(var_57);
    }
    if let Some(var_58) = &input.action_id {
        object.key("ActionId").string(var_58);
    }
    if let Some(var_59) = &input.time_period {
        let mut object_60 = object.key("TimePeriod").start_object();
        crate::json_ser::serialize_structure_crate_model_time_period(&mut object_60, var_59)?;
        object_60.finish();
    }
    if let Some(var_61) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_61).into()),
        );
    }
    if let Some(var_62) = &input.next_token {
        object.key("NextToken").string(var_62);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_budget_actions_for_account_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeBudgetActionsForAccountInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_63) = &input.account_id {
        object.key("AccountId").string(var_63);
    }
    if let Some(var_64) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_64).into()),
        );
    }
    if let Some(var_65) = &input.next_token {
        object.key("NextToken").string(var_65);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_budget_actions_for_budget_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeBudgetActionsForBudgetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_66) = &input.account_id {
        object.key("AccountId").string(var_66);
    }
    if let Some(var_67) = &input.budget_name {
        object.key("BudgetName").string(var_67);
    }
    if let Some(var_68) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_68).into()),
        );
    }
    if let Some(var_69) = &input.next_token {
        object.key("NextToken").string(var_69);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_budget_performance_history_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeBudgetPerformanceHistoryInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_70) = &input.account_id {
        object.key("AccountId").string(var_70);
    }
    if let Some(var_71) = &input.budget_name {
        object.key("BudgetName").string(var_71);
    }
    if let Some(var_72) = &input.time_period {
        let mut object_73 = object.key("TimePeriod").start_object();
        crate::json_ser::serialize_structure_crate_model_time_period(&mut object_73, var_72)?;
        object_73.finish();
    }
    if let Some(var_74) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_74).into()),
        );
    }
    if let Some(var_75) = &input.next_token {
        object.key("NextToken").string(var_75);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_budgets_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeBudgetsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_76) = &input.account_id {
        object.key("AccountId").string(var_76);
    }
    if let Some(var_77) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_77).into()),
        );
    }
    if let Some(var_78) = &input.next_token {
        object.key("NextToken").string(var_78);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_notifications_for_budget_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeNotificationsForBudgetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_79) = &input.account_id {
        object.key("AccountId").string(var_79);
    }
    if let Some(var_80) = &input.budget_name {
        object.key("BudgetName").string(var_80);
    }
    if let Some(var_81) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_81).into()),
        );
    }
    if let Some(var_82) = &input.next_token {
        object.key("NextToken").string(var_82);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_subscribers_for_notification_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeSubscribersForNotificationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_83) = &input.account_id {
        object.key("AccountId").string(var_83);
    }
    if let Some(var_84) = &input.budget_name {
        object.key("BudgetName").string(var_84);
    }
    if let Some(var_85) = &input.notification {
        let mut object_86 = object.key("Notification").start_object();
        crate::json_ser::serialize_structure_crate_model_notification(&mut object_86, var_85)?;
        object_86.finish();
    }
    if let Some(var_87) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_87).into()),
        );
    }
    if let Some(var_88) = &input.next_token {
        object.key("NextToken").string(var_88);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_execute_budget_action_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ExecuteBudgetActionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_89) = &input.account_id {
        object.key("AccountId").string(var_89);
    }
    if let Some(var_90) = &input.budget_name {
        object.key("BudgetName").string(var_90);
    }
    if let Some(var_91) = &input.action_id {
        object.key("ActionId").string(var_91);
    }
    if let Some(var_92) = &input.execution_type {
        object.key("ExecutionType").string(var_92.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_budget_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateBudgetInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_93) = &input.account_id {
        object.key("AccountId").string(var_93);
    }
    if let Some(var_94) = &input.new_budget {
        let mut object_95 = object.key("NewBudget").start_object();
        crate::json_ser::serialize_structure_crate_model_budget(&mut object_95, var_94)?;
        object_95.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_budget_action_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateBudgetActionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_96) = &input.account_id {
        object.key("AccountId").string(var_96);
    }
    if let Some(var_97) = &input.budget_name {
        object.key("BudgetName").string(var_97);
    }
    if let Some(var_98) = &input.action_id {
        object.key("ActionId").string(var_98);
    }
    if let Some(var_99) = &input.notification_type {
        object.key("NotificationType").string(var_99.as_str());
    }
    if let Some(var_100) = &input.action_threshold {
        let mut object_101 = object.key("ActionThreshold").start_object();
        crate::json_ser::serialize_structure_crate_model_action_threshold(
            &mut object_101,
            var_100,
        )?;
        object_101.finish();
    }
    if let Some(var_102) = &input.definition {
        let mut object_103 = object.key("Definition").start_object();
        crate::json_ser::serialize_structure_crate_model_definition(&mut object_103, var_102)?;
        object_103.finish();
    }
    if let Some(var_104) = &input.execution_role_arn {
        object.key("ExecutionRoleArn").string(var_104);
    }
    if let Some(var_105) = &input.approval_model {
        object.key("ApprovalModel").string(var_105.as_str());
    }
    if let Some(var_106) = &input.subscribers {
        let mut array_107 = object.key("Subscribers").start_array();
        for item_108 in var_106 {
            {
                let mut object_109 = array_107.value().start_object();
                crate::json_ser::serialize_structure_crate_model_subscriber(
                    &mut object_109,
                    item_108,
                )?;
                object_109.finish();
            }
        }
        array_107.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_notification_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateNotificationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_110) = &input.account_id {
        object.key("AccountId").string(var_110);
    }
    if let Some(var_111) = &input.budget_name {
        object.key("BudgetName").string(var_111);
    }
    if let Some(var_112) = &input.old_notification {
        let mut object_113 = object.key("OldNotification").start_object();
        crate::json_ser::serialize_structure_crate_model_notification(&mut object_113, var_112)?;
        object_113.finish();
    }
    if let Some(var_114) = &input.new_notification {
        let mut object_115 = object.key("NewNotification").start_object();
        crate::json_ser::serialize_structure_crate_model_notification(&mut object_115, var_114)?;
        object_115.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_subscriber_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSubscriberInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_116) = &input.account_id {
        object.key("AccountId").string(var_116);
    }
    if let Some(var_117) = &input.budget_name {
        object.key("BudgetName").string(var_117);
    }
    if let Some(var_118) = &input.notification {
        let mut object_119 = object.key("Notification").start_object();
        crate::json_ser::serialize_structure_crate_model_notification(&mut object_119, var_118)?;
        object_119.finish();
    }
    if let Some(var_120) = &input.old_subscriber {
        let mut object_121 = object.key("OldSubscriber").start_object();
        crate::json_ser::serialize_structure_crate_model_subscriber(&mut object_121, var_120)?;
        object_121.finish();
    }
    if let Some(var_122) = &input.new_subscriber {
        let mut object_123 = object.key("NewSubscriber").start_object();
        crate::json_ser::serialize_structure_crate_model_subscriber(&mut object_123, var_122)?;
        object_123.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_budget(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Budget,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_124) = &input.budget_name {
        object.key("BudgetName").string(var_124);
    }
    if let Some(var_125) = &input.budget_limit {
        let mut object_126 = object.key("BudgetLimit").start_object();
        crate::json_ser::serialize_structure_crate_model_spend(&mut object_126, var_125)?;
        object_126.finish();
    }
    if let Some(var_127) = &input.planned_budget_limits {
        let mut object_128 = object.key("PlannedBudgetLimits").start_object();
        for (key_129, value_130) in var_127 {
            {
                let mut object_131 = object_128.key(key_129).start_object();
                crate::json_ser::serialize_structure_crate_model_spend(&mut object_131, value_130)?;
                object_131.finish();
            }
        }
        object_128.finish();
    }
    if let Some(var_132) = &input.cost_filters {
        let mut object_133 = object.key("CostFilters").start_object();
        for (key_134, value_135) in var_132 {
            {
                let mut array_136 = object_133.key(key_134).start_array();
                for item_137 in value_135 {
                    {
                        array_136.value().string(item_137);
                    }
                }
                array_136.finish();
            }
        }
        object_133.finish();
    }
    if let Some(var_138) = &input.cost_types {
        let mut object_139 = object.key("CostTypes").start_object();
        crate::json_ser::serialize_structure_crate_model_cost_types(&mut object_139, var_138)?;
        object_139.finish();
    }
    if let Some(var_140) = &input.time_unit {
        object.key("TimeUnit").string(var_140.as_str());
    }
    if let Some(var_141) = &input.time_period {
        let mut object_142 = object.key("TimePeriod").start_object();
        crate::json_ser::serialize_structure_crate_model_time_period(&mut object_142, var_141)?;
        object_142.finish();
    }
    if let Some(var_143) = &input.calculated_spend {
        let mut object_144 = object.key("CalculatedSpend").start_object();
        crate::json_ser::serialize_structure_crate_model_calculated_spend(
            &mut object_144,
            var_143,
        )?;
        object_144.finish();
    }
    if let Some(var_145) = &input.budget_type {
        object.key("BudgetType").string(var_145.as_str());
    }
    if let Some(var_146) = &input.last_updated_time {
        object
            .key("LastUpdatedTime")
            .date_time(var_146, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub fn serialize_structure_crate_model_notification_with_subscribers(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NotificationWithSubscribers,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_147) = &input.notification {
        let mut object_148 = object.key("Notification").start_object();
        crate::json_ser::serialize_structure_crate_model_notification(&mut object_148, var_147)?;
        object_148.finish();
    }
    if let Some(var_149) = &input.subscribers {
        let mut array_150 = object.key("Subscribers").start_array();
        for item_151 in var_149 {
            {
                let mut object_152 = array_150.value().start_object();
                crate::json_ser::serialize_structure_crate_model_subscriber(
                    &mut object_152,
                    item_151,
                )?;
                object_152.finish();
            }
        }
        array_150.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_action_threshold(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ActionThreshold,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    {
        object.key("ActionThresholdValue").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((input.action_threshold_value).into()),
        );
    }
    if let Some(var_153) = &input.action_threshold_type {
        object.key("ActionThresholdType").string(var_153.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_definition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Definition,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_154) = &input.iam_action_definition {
        let mut object_155 = object.key("IamActionDefinition").start_object();
        crate::json_ser::serialize_structure_crate_model_iam_action_definition(
            &mut object_155,
            var_154,
        )?;
        object_155.finish();
    }
    if let Some(var_156) = &input.scp_action_definition {
        let mut object_157 = object.key("ScpActionDefinition").start_object();
        crate::json_ser::serialize_structure_crate_model_scp_action_definition(
            &mut object_157,
            var_156,
        )?;
        object_157.finish();
    }
    if let Some(var_158) = &input.ssm_action_definition {
        let mut object_159 = object.key("SsmActionDefinition").start_object();
        crate::json_ser::serialize_structure_crate_model_ssm_action_definition(
            &mut object_159,
            var_158,
        )?;
        object_159.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_subscriber(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Subscriber,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_160) = &input.subscription_type {
        object.key("SubscriptionType").string(var_160.as_str());
    }
    if let Some(var_161) = &input.address {
        object.key("Address").string(var_161);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_notification(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Notification,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_162) = &input.notification_type {
        object.key("NotificationType").string(var_162.as_str());
    }
    if let Some(var_163) = &input.comparison_operator {
        object.key("ComparisonOperator").string(var_163.as_str());
    }
    {
        object.key("Threshold").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((input.threshold).into()),
        );
    }
    if let Some(var_164) = &input.threshold_type {
        object.key("ThresholdType").string(var_164.as_str());
    }
    if let Some(var_165) = &input.notification_state {
        object.key("NotificationState").string(var_165.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_time_period(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TimePeriod,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_166) = &input.start {
        object
            .key("Start")
            .date_time(var_166, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_167) = &input.end {
        object
            .key("End")
            .date_time(var_167, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub fn serialize_structure_crate_model_spend(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Spend,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_168) = &input.amount {
        object.key("Amount").string(var_168);
    }
    if let Some(var_169) = &input.unit {
        object.key("Unit").string(var_169);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_cost_types(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CostTypes,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_170) = &input.include_tax {
        object.key("IncludeTax").boolean(*var_170);
    }
    if let Some(var_171) = &input.include_subscription {
        object.key("IncludeSubscription").boolean(*var_171);
    }
    if let Some(var_172) = &input.use_blended {
        object.key("UseBlended").boolean(*var_172);
    }
    if let Some(var_173) = &input.include_refund {
        object.key("IncludeRefund").boolean(*var_173);
    }
    if let Some(var_174) = &input.include_credit {
        object.key("IncludeCredit").boolean(*var_174);
    }
    if let Some(var_175) = &input.include_upfront {
        object.key("IncludeUpfront").boolean(*var_175);
    }
    if let Some(var_176) = &input.include_recurring {
        object.key("IncludeRecurring").boolean(*var_176);
    }
    if let Some(var_177) = &input.include_other_subscription {
        object.key("IncludeOtherSubscription").boolean(*var_177);
    }
    if let Some(var_178) = &input.include_support {
        object.key("IncludeSupport").boolean(*var_178);
    }
    if let Some(var_179) = &input.include_discount {
        object.key("IncludeDiscount").boolean(*var_179);
    }
    if let Some(var_180) = &input.use_amortized {
        object.key("UseAmortized").boolean(*var_180);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_calculated_spend(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CalculatedSpend,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_181) = &input.actual_spend {
        let mut object_182 = object.key("ActualSpend").start_object();
        crate::json_ser::serialize_structure_crate_model_spend(&mut object_182, var_181)?;
        object_182.finish();
    }
    if let Some(var_183) = &input.forecasted_spend {
        let mut object_184 = object.key("ForecastedSpend").start_object();
        crate::json_ser::serialize_structure_crate_model_spend(&mut object_184, var_183)?;
        object_184.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_iam_action_definition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::IamActionDefinition,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_185) = &input.policy_arn {
        object.key("PolicyArn").string(var_185);
    }
    if let Some(var_186) = &input.roles {
        let mut array_187 = object.key("Roles").start_array();
        for item_188 in var_186 {
            {
                array_187.value().string(item_188);
            }
        }
        array_187.finish();
    }
    if let Some(var_189) = &input.groups {
        let mut array_190 = object.key("Groups").start_array();
        for item_191 in var_189 {
            {
                array_190.value().string(item_191);
            }
        }
        array_190.finish();
    }
    if let Some(var_192) = &input.users {
        let mut array_193 = object.key("Users").start_array();
        for item_194 in var_192 {
            {
                array_193.value().string(item_194);
            }
        }
        array_193.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_scp_action_definition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ScpActionDefinition,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_195) = &input.policy_id {
        object.key("PolicyId").string(var_195);
    }
    if let Some(var_196) = &input.target_ids {
        let mut array_197 = object.key("TargetIds").start_array();
        for item_198 in var_196 {
            {
                array_197.value().string(item_198);
            }
        }
        array_197.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_ssm_action_definition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SsmActionDefinition,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_199) = &input.action_sub_type {
        object.key("ActionSubType").string(var_199.as_str());
    }
    if let Some(var_200) = &input.region {
        object.key("Region").string(var_200);
    }
    if let Some(var_201) = &input.instance_ids {
        let mut array_202 = object.key("InstanceIds").start_array();
        for item_203 in var_201 {
            {
                array_202.value().string(item_203);
            }
        }
        array_202.finish();
    }
    Ok(())
}
