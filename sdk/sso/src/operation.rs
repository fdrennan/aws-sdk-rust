// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `GetRoleCredentials`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_role_credentials`](crate::client::fluent_builders::GetRoleCredentials).
            ///
            /// `ParseStrictResponse` impl for `GetRoleCredentials`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetRoleCredentials {
    _private: ()
}
impl GetRoleCredentials {
    /// Creates a new builder-style object to manufacture [`GetRoleCredentialsInput`](crate::input::GetRoleCredentialsInput).
    pub fn builder() -> crate::input::get_role_credentials_input::Builder {
        crate::input::get_role_credentials_input::Builder::default()
    }
    /// Creates a new `GetRoleCredentials` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetRoleCredentials {
                type Output = std::result::Result<crate::output::GetRoleCredentialsOutput, crate::error::GetRoleCredentialsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_role_credentials_error(response)
                     } else {
                        crate::operation_deser::parse_get_role_credentials_response(response)
                     }
                }
            }

/// Operation shape for `ListAccountRoles`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_account_roles`](crate::client::fluent_builders::ListAccountRoles).
            ///
            /// `ParseStrictResponse` impl for `ListAccountRoles`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListAccountRoles {
    _private: ()
}
impl ListAccountRoles {
    /// Creates a new builder-style object to manufacture [`ListAccountRolesInput`](crate::input::ListAccountRolesInput).
    pub fn builder() -> crate::input::list_account_roles_input::Builder {
        crate::input::list_account_roles_input::Builder::default()
    }
    /// Creates a new `ListAccountRoles` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAccountRoles {
                type Output = std::result::Result<crate::output::ListAccountRolesOutput, crate::error::ListAccountRolesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_account_roles_error(response)
                     } else {
                        crate::operation_deser::parse_list_account_roles_response(response)
                     }
                }
            }

/// Operation shape for `ListAccounts`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_accounts`](crate::client::fluent_builders::ListAccounts).
            ///
            /// `ParseStrictResponse` impl for `ListAccounts`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListAccounts {
    _private: ()
}
impl ListAccounts {
    /// Creates a new builder-style object to manufacture [`ListAccountsInput`](crate::input::ListAccountsInput).
    pub fn builder() -> crate::input::list_accounts_input::Builder {
        crate::input::list_accounts_input::Builder::default()
    }
    /// Creates a new `ListAccounts` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAccounts {
                type Output = std::result::Result<crate::output::ListAccountsOutput, crate::error::ListAccountsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_accounts_error(response)
                     } else {
                        crate::operation_deser::parse_list_accounts_response(response)
                     }
                }
            }

/// Operation shape for `Logout`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`logout`](crate::client::fluent_builders::Logout).
            ///
            /// `ParseStrictResponse` impl for `Logout`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct Logout {
    _private: ()
}
impl Logout {
    /// Creates a new builder-style object to manufacture [`LogoutInput`](crate::input::LogoutInput).
    pub fn builder() -> crate::input::logout_input::Builder {
        crate::input::logout_input::Builder::default()
    }
    /// Creates a new `Logout` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for Logout {
                type Output = std::result::Result<crate::output::LogoutOutput, crate::error::LogoutError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_logout_error(response)
                     } else {
                        crate::operation_deser::parse_logout_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

