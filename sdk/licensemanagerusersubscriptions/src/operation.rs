// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AssociateUser`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`associate_user`](crate::client::fluent_builders::AssociateUser).
            ///
            /// `ParseStrictResponse` impl for `AssociateUser`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AssociateUser {
    _private: ()
}
impl AssociateUser {
    /// Creates a new builder-style object to manufacture [`AssociateUserInput`](crate::input::AssociateUserInput).
    pub fn builder() -> crate::input::associate_user_input::Builder {
        crate::input::associate_user_input::Builder::default()
    }
    /// Creates a new `AssociateUser` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssociateUser {
                type Output = std::result::Result<crate::output::AssociateUserOutput, crate::error::AssociateUserError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_associate_user_error(response)
                     } else {
                        crate::operation_deser::parse_associate_user_response(response)
                     }
                }
            }

/// Operation shape for `DeregisterIdentityProvider`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`deregister_identity_provider`](crate::client::fluent_builders::DeregisterIdentityProvider).
            ///
            /// `ParseStrictResponse` impl for `DeregisterIdentityProvider`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeregisterIdentityProvider {
    _private: ()
}
impl DeregisterIdentityProvider {
    /// Creates a new builder-style object to manufacture [`DeregisterIdentityProviderInput`](crate::input::DeregisterIdentityProviderInput).
    pub fn builder() -> crate::input::deregister_identity_provider_input::Builder {
        crate::input::deregister_identity_provider_input::Builder::default()
    }
    /// Creates a new `DeregisterIdentityProvider` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeregisterIdentityProvider {
                type Output = std::result::Result<crate::output::DeregisterIdentityProviderOutput, crate::error::DeregisterIdentityProviderError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_deregister_identity_provider_error(response)
                     } else {
                        crate::operation_deser::parse_deregister_identity_provider_response(response)
                     }
                }
            }

/// Operation shape for `DisassociateUser`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`disassociate_user`](crate::client::fluent_builders::DisassociateUser).
            ///
            /// `ParseStrictResponse` impl for `DisassociateUser`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DisassociateUser {
    _private: ()
}
impl DisassociateUser {
    /// Creates a new builder-style object to manufacture [`DisassociateUserInput`](crate::input::DisassociateUserInput).
    pub fn builder() -> crate::input::disassociate_user_input::Builder {
        crate::input::disassociate_user_input::Builder::default()
    }
    /// Creates a new `DisassociateUser` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisassociateUser {
                type Output = std::result::Result<crate::output::DisassociateUserOutput, crate::error::DisassociateUserError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_disassociate_user_error(response)
                     } else {
                        crate::operation_deser::parse_disassociate_user_response(response)
                     }
                }
            }

/// Operation shape for `ListIdentityProviders`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_identity_providers`](crate::client::fluent_builders::ListIdentityProviders).
            ///
            /// `ParseStrictResponse` impl for `ListIdentityProviders`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListIdentityProviders {
    _private: ()
}
impl ListIdentityProviders {
    /// Creates a new builder-style object to manufacture [`ListIdentityProvidersInput`](crate::input::ListIdentityProvidersInput).
    pub fn builder() -> crate::input::list_identity_providers_input::Builder {
        crate::input::list_identity_providers_input::Builder::default()
    }
    /// Creates a new `ListIdentityProviders` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListIdentityProviders {
                type Output = std::result::Result<crate::output::ListIdentityProvidersOutput, crate::error::ListIdentityProvidersError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_identity_providers_error(response)
                     } else {
                        crate::operation_deser::parse_list_identity_providers_response(response)
                     }
                }
            }

/// Operation shape for `ListInstances`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_instances`](crate::client::fluent_builders::ListInstances).
            ///
            /// `ParseStrictResponse` impl for `ListInstances`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListInstances {
    _private: ()
}
impl ListInstances {
    /// Creates a new builder-style object to manufacture [`ListInstancesInput`](crate::input::ListInstancesInput).
    pub fn builder() -> crate::input::list_instances_input::Builder {
        crate::input::list_instances_input::Builder::default()
    }
    /// Creates a new `ListInstances` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListInstances {
                type Output = std::result::Result<crate::output::ListInstancesOutput, crate::error::ListInstancesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_instances_error(response)
                     } else {
                        crate::operation_deser::parse_list_instances_response(response)
                     }
                }
            }

/// Operation shape for `ListProductSubscriptions`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_product_subscriptions`](crate::client::fluent_builders::ListProductSubscriptions).
            ///
            /// `ParseStrictResponse` impl for `ListProductSubscriptions`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListProductSubscriptions {
    _private: ()
}
impl ListProductSubscriptions {
    /// Creates a new builder-style object to manufacture [`ListProductSubscriptionsInput`](crate::input::ListProductSubscriptionsInput).
    pub fn builder() -> crate::input::list_product_subscriptions_input::Builder {
        crate::input::list_product_subscriptions_input::Builder::default()
    }
    /// Creates a new `ListProductSubscriptions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListProductSubscriptions {
                type Output = std::result::Result<crate::output::ListProductSubscriptionsOutput, crate::error::ListProductSubscriptionsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_product_subscriptions_error(response)
                     } else {
                        crate::operation_deser::parse_list_product_subscriptions_response(response)
                     }
                }
            }

/// Operation shape for `ListUserAssociations`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_user_associations`](crate::client::fluent_builders::ListUserAssociations).
            ///
            /// `ParseStrictResponse` impl for `ListUserAssociations`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListUserAssociations {
    _private: ()
}
impl ListUserAssociations {
    /// Creates a new builder-style object to manufacture [`ListUserAssociationsInput`](crate::input::ListUserAssociationsInput).
    pub fn builder() -> crate::input::list_user_associations_input::Builder {
        crate::input::list_user_associations_input::Builder::default()
    }
    /// Creates a new `ListUserAssociations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListUserAssociations {
                type Output = std::result::Result<crate::output::ListUserAssociationsOutput, crate::error::ListUserAssociationsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_user_associations_error(response)
                     } else {
                        crate::operation_deser::parse_list_user_associations_response(response)
                     }
                }
            }

/// Operation shape for `RegisterIdentityProvider`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`register_identity_provider`](crate::client::fluent_builders::RegisterIdentityProvider).
            ///
            /// `ParseStrictResponse` impl for `RegisterIdentityProvider`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct RegisterIdentityProvider {
    _private: ()
}
impl RegisterIdentityProvider {
    /// Creates a new builder-style object to manufacture [`RegisterIdentityProviderInput`](crate::input::RegisterIdentityProviderInput).
    pub fn builder() -> crate::input::register_identity_provider_input::Builder {
        crate::input::register_identity_provider_input::Builder::default()
    }
    /// Creates a new `RegisterIdentityProvider` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RegisterIdentityProvider {
                type Output = std::result::Result<crate::output::RegisterIdentityProviderOutput, crate::error::RegisterIdentityProviderError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_register_identity_provider_error(response)
                     } else {
                        crate::operation_deser::parse_register_identity_provider_response(response)
                     }
                }
            }

/// Operation shape for `StartProductSubscription`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`start_product_subscription`](crate::client::fluent_builders::StartProductSubscription).
            ///
            /// `ParseStrictResponse` impl for `StartProductSubscription`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StartProductSubscription {
    _private: ()
}
impl StartProductSubscription {
    /// Creates a new builder-style object to manufacture [`StartProductSubscriptionInput`](crate::input::StartProductSubscriptionInput).
    pub fn builder() -> crate::input::start_product_subscription_input::Builder {
        crate::input::start_product_subscription_input::Builder::default()
    }
    /// Creates a new `StartProductSubscription` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartProductSubscription {
                type Output = std::result::Result<crate::output::StartProductSubscriptionOutput, crate::error::StartProductSubscriptionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_start_product_subscription_error(response)
                     } else {
                        crate::operation_deser::parse_start_product_subscription_response(response)
                     }
                }
            }

/// Operation shape for `StopProductSubscription`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`stop_product_subscription`](crate::client::fluent_builders::StopProductSubscription).
            ///
            /// `ParseStrictResponse` impl for `StopProductSubscription`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StopProductSubscription {
    _private: ()
}
impl StopProductSubscription {
    /// Creates a new builder-style object to manufacture [`StopProductSubscriptionInput`](crate::input::StopProductSubscriptionInput).
    pub fn builder() -> crate::input::stop_product_subscription_input::Builder {
        crate::input::stop_product_subscription_input::Builder::default()
    }
    /// Creates a new `StopProductSubscription` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopProductSubscription {
                type Output = std::result::Result<crate::output::StopProductSubscriptionOutput, crate::error::StopProductSubscriptionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_stop_product_subscription_error(response)
                     } else {
                        crate::operation_deser::parse_stop_product_subscription_response(response)
                     }
                }
            }

/// Operation shape for `UpdateIdentityProviderSettings`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_identity_provider_settings`](crate::client::fluent_builders::UpdateIdentityProviderSettings).
            ///
            /// `ParseStrictResponse` impl for `UpdateIdentityProviderSettings`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateIdentityProviderSettings {
    _private: ()
}
impl UpdateIdentityProviderSettings {
    /// Creates a new builder-style object to manufacture [`UpdateIdentityProviderSettingsInput`](crate::input::UpdateIdentityProviderSettingsInput).
    pub fn builder() -> crate::input::update_identity_provider_settings_input::Builder {
        crate::input::update_identity_provider_settings_input::Builder::default()
    }
    /// Creates a new `UpdateIdentityProviderSettings` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateIdentityProviderSettings {
                type Output = std::result::Result<crate::output::UpdateIdentityProviderSettingsOutput, crate::error::UpdateIdentityProviderSettingsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_identity_provider_settings_error(response)
                     } else {
                        crate::operation_deser::parse_update_identity_provider_settings_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

