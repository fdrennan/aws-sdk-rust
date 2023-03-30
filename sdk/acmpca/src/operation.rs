// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateCertificateAuthority`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_certificate_authority`](crate::client::fluent_builders::CreateCertificateAuthority).
            ///
            /// `ParseStrictResponse` impl for `CreateCertificateAuthority`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateCertificateAuthority {
    _private: ()
}
impl CreateCertificateAuthority {
    /// Creates a new builder-style object to manufacture [`CreateCertificateAuthorityInput`](crate::input::CreateCertificateAuthorityInput).
    pub fn builder() -> crate::input::create_certificate_authority_input::Builder {
        crate::input::create_certificate_authority_input::Builder::default()
    }
    /// Creates a new `CreateCertificateAuthority` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateCertificateAuthority {
                type Output = std::result::Result<crate::output::CreateCertificateAuthorityOutput, crate::error::CreateCertificateAuthorityError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_certificate_authority_error(response)
                     } else {
                        crate::operation_deser::parse_create_certificate_authority_response(response)
                     }
                }
            }

/// Operation shape for `CreateCertificateAuthorityAuditReport`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_certificate_authority_audit_report`](crate::client::fluent_builders::CreateCertificateAuthorityAuditReport).
            ///
            /// `ParseStrictResponse` impl for `CreateCertificateAuthorityAuditReport`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateCertificateAuthorityAuditReport {
    _private: ()
}
impl CreateCertificateAuthorityAuditReport {
    /// Creates a new builder-style object to manufacture [`CreateCertificateAuthorityAuditReportInput`](crate::input::CreateCertificateAuthorityAuditReportInput).
    pub fn builder() -> crate::input::create_certificate_authority_audit_report_input::Builder {
        crate::input::create_certificate_authority_audit_report_input::Builder::default()
    }
    /// Creates a new `CreateCertificateAuthorityAuditReport` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateCertificateAuthorityAuditReport {
                type Output = std::result::Result<crate::output::CreateCertificateAuthorityAuditReportOutput, crate::error::CreateCertificateAuthorityAuditReportError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_certificate_authority_audit_report_error(response)
                     } else {
                        crate::operation_deser::parse_create_certificate_authority_audit_report_response(response)
                     }
                }
            }

/// Operation shape for `CreatePermission`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_permission`](crate::client::fluent_builders::CreatePermission).
            ///
            /// `ParseStrictResponse` impl for `CreatePermission`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreatePermission {
    _private: ()
}
impl CreatePermission {
    /// Creates a new builder-style object to manufacture [`CreatePermissionInput`](crate::input::CreatePermissionInput).
    pub fn builder() -> crate::input::create_permission_input::Builder {
        crate::input::create_permission_input::Builder::default()
    }
    /// Creates a new `CreatePermission` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreatePermission {
                type Output = std::result::Result<crate::output::CreatePermissionOutput, crate::error::CreatePermissionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_permission_error(response)
                     } else {
                        crate::operation_deser::parse_create_permission_response(response)
                     }
                }
            }

/// Operation shape for `DeleteCertificateAuthority`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_certificate_authority`](crate::client::fluent_builders::DeleteCertificateAuthority).
            ///
            /// `ParseStrictResponse` impl for `DeleteCertificateAuthority`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteCertificateAuthority {
    _private: ()
}
impl DeleteCertificateAuthority {
    /// Creates a new builder-style object to manufacture [`DeleteCertificateAuthorityInput`](crate::input::DeleteCertificateAuthorityInput).
    pub fn builder() -> crate::input::delete_certificate_authority_input::Builder {
        crate::input::delete_certificate_authority_input::Builder::default()
    }
    /// Creates a new `DeleteCertificateAuthority` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteCertificateAuthority {
                type Output = std::result::Result<crate::output::DeleteCertificateAuthorityOutput, crate::error::DeleteCertificateAuthorityError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_certificate_authority_error(response)
                     } else {
                        crate::operation_deser::parse_delete_certificate_authority_response(response)
                     }
                }
            }

/// Operation shape for `DeletePermission`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_permission`](crate::client::fluent_builders::DeletePermission).
            ///
            /// `ParseStrictResponse` impl for `DeletePermission`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeletePermission {
    _private: ()
}
impl DeletePermission {
    /// Creates a new builder-style object to manufacture [`DeletePermissionInput`](crate::input::DeletePermissionInput).
    pub fn builder() -> crate::input::delete_permission_input::Builder {
        crate::input::delete_permission_input::Builder::default()
    }
    /// Creates a new `DeletePermission` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeletePermission {
                type Output = std::result::Result<crate::output::DeletePermissionOutput, crate::error::DeletePermissionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_permission_error(response)
                     } else {
                        crate::operation_deser::parse_delete_permission_response(response)
                     }
                }
            }

/// Operation shape for `DeletePolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_policy`](crate::client::fluent_builders::DeletePolicy).
            ///
            /// `ParseStrictResponse` impl for `DeletePolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeletePolicy {
    _private: ()
}
impl DeletePolicy {
    /// Creates a new builder-style object to manufacture [`DeletePolicyInput`](crate::input::DeletePolicyInput).
    pub fn builder() -> crate::input::delete_policy_input::Builder {
        crate::input::delete_policy_input::Builder::default()
    }
    /// Creates a new `DeletePolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeletePolicy {
                type Output = std::result::Result<crate::output::DeletePolicyOutput, crate::error::DeletePolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_policy_error(response)
                     } else {
                        crate::operation_deser::parse_delete_policy_response(response)
                     }
                }
            }

/// Operation shape for `DescribeCertificateAuthority`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_certificate_authority`](crate::client::fluent_builders::DescribeCertificateAuthority).
            ///
            /// `ParseStrictResponse` impl for `DescribeCertificateAuthority`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeCertificateAuthority {
    _private: ()
}
impl DescribeCertificateAuthority {
    /// Creates a new builder-style object to manufacture [`DescribeCertificateAuthorityInput`](crate::input::DescribeCertificateAuthorityInput).
    pub fn builder() -> crate::input::describe_certificate_authority_input::Builder {
        crate::input::describe_certificate_authority_input::Builder::default()
    }
    /// Creates a new `DescribeCertificateAuthority` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeCertificateAuthority {
                type Output = std::result::Result<crate::output::DescribeCertificateAuthorityOutput, crate::error::DescribeCertificateAuthorityError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_certificate_authority_error(response)
                     } else {
                        crate::operation_deser::parse_describe_certificate_authority_response(response)
                     }
                }
            }

/// Operation shape for `DescribeCertificateAuthorityAuditReport`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_certificate_authority_audit_report`](crate::client::fluent_builders::DescribeCertificateAuthorityAuditReport).
            ///
            /// `ParseStrictResponse` impl for `DescribeCertificateAuthorityAuditReport`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeCertificateAuthorityAuditReport {
    _private: ()
}
impl DescribeCertificateAuthorityAuditReport {
    /// Creates a new builder-style object to manufacture [`DescribeCertificateAuthorityAuditReportInput`](crate::input::DescribeCertificateAuthorityAuditReportInput).
    pub fn builder() -> crate::input::describe_certificate_authority_audit_report_input::Builder {
        crate::input::describe_certificate_authority_audit_report_input::Builder::default()
    }
    /// Creates a new `DescribeCertificateAuthorityAuditReport` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeCertificateAuthorityAuditReport {
                type Output = std::result::Result<crate::output::DescribeCertificateAuthorityAuditReportOutput, crate::error::DescribeCertificateAuthorityAuditReportError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_certificate_authority_audit_report_error(response)
                     } else {
                        crate::operation_deser::parse_describe_certificate_authority_audit_report_response(response)
                     }
                }
            }

/// Operation shape for `GetCertificate`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_certificate`](crate::client::fluent_builders::GetCertificate).
            ///
            /// `ParseStrictResponse` impl for `GetCertificate`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetCertificate {
    _private: ()
}
impl GetCertificate {
    /// Creates a new builder-style object to manufacture [`GetCertificateInput`](crate::input::GetCertificateInput).
    pub fn builder() -> crate::input::get_certificate_input::Builder {
        crate::input::get_certificate_input::Builder::default()
    }
    /// Creates a new `GetCertificate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetCertificate {
                type Output = std::result::Result<crate::output::GetCertificateOutput, crate::error::GetCertificateError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_certificate_error(response)
                     } else {
                        crate::operation_deser::parse_get_certificate_response(response)
                     }
                }
            }

/// Operation shape for `GetCertificateAuthorityCertificate`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_certificate_authority_certificate`](crate::client::fluent_builders::GetCertificateAuthorityCertificate).
            ///
            /// `ParseStrictResponse` impl for `GetCertificateAuthorityCertificate`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetCertificateAuthorityCertificate {
    _private: ()
}
impl GetCertificateAuthorityCertificate {
    /// Creates a new builder-style object to manufacture [`GetCertificateAuthorityCertificateInput`](crate::input::GetCertificateAuthorityCertificateInput).
    pub fn builder() -> crate::input::get_certificate_authority_certificate_input::Builder {
        crate::input::get_certificate_authority_certificate_input::Builder::default()
    }
    /// Creates a new `GetCertificateAuthorityCertificate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetCertificateAuthorityCertificate {
                type Output = std::result::Result<crate::output::GetCertificateAuthorityCertificateOutput, crate::error::GetCertificateAuthorityCertificateError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_certificate_authority_certificate_error(response)
                     } else {
                        crate::operation_deser::parse_get_certificate_authority_certificate_response(response)
                     }
                }
            }

/// Operation shape for `GetCertificateAuthorityCsr`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_certificate_authority_csr`](crate::client::fluent_builders::GetCertificateAuthorityCsr).
            ///
            /// `ParseStrictResponse` impl for `GetCertificateAuthorityCsr`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetCertificateAuthorityCsr {
    _private: ()
}
impl GetCertificateAuthorityCsr {
    /// Creates a new builder-style object to manufacture [`GetCertificateAuthorityCsrInput`](crate::input::GetCertificateAuthorityCsrInput).
    pub fn builder() -> crate::input::get_certificate_authority_csr_input::Builder {
        crate::input::get_certificate_authority_csr_input::Builder::default()
    }
    /// Creates a new `GetCertificateAuthorityCsr` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetCertificateAuthorityCsr {
                type Output = std::result::Result<crate::output::GetCertificateAuthorityCsrOutput, crate::error::GetCertificateAuthorityCsrError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_certificate_authority_csr_error(response)
                     } else {
                        crate::operation_deser::parse_get_certificate_authority_csr_response(response)
                     }
                }
            }

/// Operation shape for `GetPolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_policy`](crate::client::fluent_builders::GetPolicy).
            ///
            /// `ParseStrictResponse` impl for `GetPolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetPolicy {
    _private: ()
}
impl GetPolicy {
    /// Creates a new builder-style object to manufacture [`GetPolicyInput`](crate::input::GetPolicyInput).
    pub fn builder() -> crate::input::get_policy_input::Builder {
        crate::input::get_policy_input::Builder::default()
    }
    /// Creates a new `GetPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetPolicy {
                type Output = std::result::Result<crate::output::GetPolicyOutput, crate::error::GetPolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_policy_error(response)
                     } else {
                        crate::operation_deser::parse_get_policy_response(response)
                     }
                }
            }

/// Operation shape for `ImportCertificateAuthorityCertificate`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`import_certificate_authority_certificate`](crate::client::fluent_builders::ImportCertificateAuthorityCertificate).
            ///
            /// `ParseStrictResponse` impl for `ImportCertificateAuthorityCertificate`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ImportCertificateAuthorityCertificate {
    _private: ()
}
impl ImportCertificateAuthorityCertificate {
    /// Creates a new builder-style object to manufacture [`ImportCertificateAuthorityCertificateInput`](crate::input::ImportCertificateAuthorityCertificateInput).
    pub fn builder() -> crate::input::import_certificate_authority_certificate_input::Builder {
        crate::input::import_certificate_authority_certificate_input::Builder::default()
    }
    /// Creates a new `ImportCertificateAuthorityCertificate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ImportCertificateAuthorityCertificate {
                type Output = std::result::Result<crate::output::ImportCertificateAuthorityCertificateOutput, crate::error::ImportCertificateAuthorityCertificateError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_import_certificate_authority_certificate_error(response)
                     } else {
                        crate::operation_deser::parse_import_certificate_authority_certificate_response(response)
                     }
                }
            }

/// Operation shape for `IssueCertificate`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`issue_certificate`](crate::client::fluent_builders::IssueCertificate).
            ///
            /// `ParseStrictResponse` impl for `IssueCertificate`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct IssueCertificate {
    _private: ()
}
impl IssueCertificate {
    /// Creates a new builder-style object to manufacture [`IssueCertificateInput`](crate::input::IssueCertificateInput).
    pub fn builder() -> crate::input::issue_certificate_input::Builder {
        crate::input::issue_certificate_input::Builder::default()
    }
    /// Creates a new `IssueCertificate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for IssueCertificate {
                type Output = std::result::Result<crate::output::IssueCertificateOutput, crate::error::IssueCertificateError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_issue_certificate_error(response)
                     } else {
                        crate::operation_deser::parse_issue_certificate_response(response)
                     }
                }
            }

/// Operation shape for `ListCertificateAuthorities`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_certificate_authorities`](crate::client::fluent_builders::ListCertificateAuthorities).
            ///
            /// `ParseStrictResponse` impl for `ListCertificateAuthorities`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListCertificateAuthorities {
    _private: ()
}
impl ListCertificateAuthorities {
    /// Creates a new builder-style object to manufacture [`ListCertificateAuthoritiesInput`](crate::input::ListCertificateAuthoritiesInput).
    pub fn builder() -> crate::input::list_certificate_authorities_input::Builder {
        crate::input::list_certificate_authorities_input::Builder::default()
    }
    /// Creates a new `ListCertificateAuthorities` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListCertificateAuthorities {
                type Output = std::result::Result<crate::output::ListCertificateAuthoritiesOutput, crate::error::ListCertificateAuthoritiesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_certificate_authorities_error(response)
                     } else {
                        crate::operation_deser::parse_list_certificate_authorities_response(response)
                     }
                }
            }

/// Operation shape for `ListPermissions`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_permissions`](crate::client::fluent_builders::ListPermissions).
            ///
            /// `ParseStrictResponse` impl for `ListPermissions`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListPermissions {
    _private: ()
}
impl ListPermissions {
    /// Creates a new builder-style object to manufacture [`ListPermissionsInput`](crate::input::ListPermissionsInput).
    pub fn builder() -> crate::input::list_permissions_input::Builder {
        crate::input::list_permissions_input::Builder::default()
    }
    /// Creates a new `ListPermissions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListPermissions {
                type Output = std::result::Result<crate::output::ListPermissionsOutput, crate::error::ListPermissionsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_permissions_error(response)
                     } else {
                        crate::operation_deser::parse_list_permissions_response(response)
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

/// Operation shape for `PutPolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_policy`](crate::client::fluent_builders::PutPolicy).
            ///
            /// `ParseStrictResponse` impl for `PutPolicy`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutPolicy {
    _private: ()
}
impl PutPolicy {
    /// Creates a new builder-style object to manufacture [`PutPolicyInput`](crate::input::PutPolicyInput).
    pub fn builder() -> crate::input::put_policy_input::Builder {
        crate::input::put_policy_input::Builder::default()
    }
    /// Creates a new `PutPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutPolicy {
                type Output = std::result::Result<crate::output::PutPolicyOutput, crate::error::PutPolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_put_policy_error(response)
                     } else {
                        crate::operation_deser::parse_put_policy_response(response)
                     }
                }
            }

/// Operation shape for `RestoreCertificateAuthority`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`restore_certificate_authority`](crate::client::fluent_builders::RestoreCertificateAuthority).
            ///
            /// `ParseStrictResponse` impl for `RestoreCertificateAuthority`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct RestoreCertificateAuthority {
    _private: ()
}
impl RestoreCertificateAuthority {
    /// Creates a new builder-style object to manufacture [`RestoreCertificateAuthorityInput`](crate::input::RestoreCertificateAuthorityInput).
    pub fn builder() -> crate::input::restore_certificate_authority_input::Builder {
        crate::input::restore_certificate_authority_input::Builder::default()
    }
    /// Creates a new `RestoreCertificateAuthority` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RestoreCertificateAuthority {
                type Output = std::result::Result<crate::output::RestoreCertificateAuthorityOutput, crate::error::RestoreCertificateAuthorityError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_restore_certificate_authority_error(response)
                     } else {
                        crate::operation_deser::parse_restore_certificate_authority_response(response)
                     }
                }
            }

/// Operation shape for `RevokeCertificate`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`revoke_certificate`](crate::client::fluent_builders::RevokeCertificate).
            ///
            /// `ParseStrictResponse` impl for `RevokeCertificate`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct RevokeCertificate {
    _private: ()
}
impl RevokeCertificate {
    /// Creates a new builder-style object to manufacture [`RevokeCertificateInput`](crate::input::RevokeCertificateInput).
    pub fn builder() -> crate::input::revoke_certificate_input::Builder {
        crate::input::revoke_certificate_input::Builder::default()
    }
    /// Creates a new `RevokeCertificate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RevokeCertificate {
                type Output = std::result::Result<crate::output::RevokeCertificateOutput, crate::error::RevokeCertificateError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_revoke_certificate_error(response)
                     } else {
                        crate::operation_deser::parse_revoke_certificate_response(response)
                     }
                }
            }

/// Operation shape for `TagCertificateAuthority`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`tag_certificate_authority`](crate::client::fluent_builders::TagCertificateAuthority).
            ///
            /// `ParseStrictResponse` impl for `TagCertificateAuthority`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct TagCertificateAuthority {
    _private: ()
}
impl TagCertificateAuthority {
    /// Creates a new builder-style object to manufacture [`TagCertificateAuthorityInput`](crate::input::TagCertificateAuthorityInput).
    pub fn builder() -> crate::input::tag_certificate_authority_input::Builder {
        crate::input::tag_certificate_authority_input::Builder::default()
    }
    /// Creates a new `TagCertificateAuthority` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagCertificateAuthority {
                type Output = std::result::Result<crate::output::TagCertificateAuthorityOutput, crate::error::TagCertificateAuthorityError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_tag_certificate_authority_error(response)
                     } else {
                        crate::operation_deser::parse_tag_certificate_authority_response(response)
                     }
                }
            }

/// Operation shape for `UntagCertificateAuthority`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`untag_certificate_authority`](crate::client::fluent_builders::UntagCertificateAuthority).
            ///
            /// `ParseStrictResponse` impl for `UntagCertificateAuthority`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UntagCertificateAuthority {
    _private: ()
}
impl UntagCertificateAuthority {
    /// Creates a new builder-style object to manufacture [`UntagCertificateAuthorityInput`](crate::input::UntagCertificateAuthorityInput).
    pub fn builder() -> crate::input::untag_certificate_authority_input::Builder {
        crate::input::untag_certificate_authority_input::Builder::default()
    }
    /// Creates a new `UntagCertificateAuthority` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagCertificateAuthority {
                type Output = std::result::Result<crate::output::UntagCertificateAuthorityOutput, crate::error::UntagCertificateAuthorityError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_untag_certificate_authority_error(response)
                     } else {
                        crate::operation_deser::parse_untag_certificate_authority_response(response)
                     }
                }
            }

/// Operation shape for `UpdateCertificateAuthority`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_certificate_authority`](crate::client::fluent_builders::UpdateCertificateAuthority).
            ///
            /// `ParseStrictResponse` impl for `UpdateCertificateAuthority`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateCertificateAuthority {
    _private: ()
}
impl UpdateCertificateAuthority {
    /// Creates a new builder-style object to manufacture [`UpdateCertificateAuthorityInput`](crate::input::UpdateCertificateAuthorityInput).
    pub fn builder() -> crate::input::update_certificate_authority_input::Builder {
        crate::input::update_certificate_authority_input::Builder::default()
    }
    /// Creates a new `UpdateCertificateAuthority` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateCertificateAuthority {
                type Output = std::result::Result<crate::output::UpdateCertificateAuthorityOutput, crate::error::UpdateCertificateAuthorityError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_certificate_authority_error(response)
                     } else {
                        crate::operation_deser::parse_update_certificate_authority_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

