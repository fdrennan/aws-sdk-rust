#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>WAF</fullname>
//! <note>
//! <p>This is the latest version of the <b>WAF</b> API,
//! released in November, 2019. The names of the entities that you use to access this API,
//! like endpoints and namespaces, all have the versioning information added, like "V2" or
//! "v2", to distinguish from the prior version. We recommend migrating your resources to
//! this version, because it has a number of significant improvements.</p>
//! <p>If you used WAF prior to this release, you can't use this WAFV2 API to access any
//! WAF resources that you created before. You can access your old rules, web ACLs, and
//! other WAF resources only through the WAF Classic APIs. The WAF Classic APIs
//! have retained the prior names, endpoints, and namespaces. </p>
//! <p>For information, including how to migrate your WAF resources to this version,
//! see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">WAF Developer Guide</a>. </p>
//! </note>
//! <p>WAF is a web application firewall that lets you monitor the HTTP and HTTPS
//! requests that are forwarded to Amazon CloudFront, an Amazon API Gateway REST API, an Application Load Balancer, or an AppSync
//! GraphQL API. WAF also lets you control access to your content. Based on conditions that
//! you specify, such as the IP addresses that requests originate from or the values of query
//! strings, the Amazon API Gateway REST API, CloudFront distribution, the Application Load Balancer, or the AppSync GraphQL
//! API responds to requests either with the requested content or with an HTTP 403 status code
//! (Forbidden). You also can configure CloudFront to return a custom error page when a request is
//! blocked.</p>
//! <p>This API guide is for developers who need detailed information about WAF API actions,
//! data types, and errors. For detailed information about WAF features and an overview of
//! how to use WAF, see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/what-is-aws-waf.html">WAF Developer
//! Guide</a>.</p>
//! <p>You can make calls using the endpoints listed in <a href="https://docs.aws.amazon.com/general/latest/gr/waf.html">WAF endpoints and quotas</a>. </p>
//! <ul>
//! <li>
//! <p>For regional applications, you can use any of the endpoints in the list.
//! A regional application can be an Application Load Balancer (ALB), an Amazon API Gateway REST API, or an AppSync GraphQL API. </p>
//! </li>
//! <li>
//! <p>For Amazon CloudFront applications, you must use the API endpoint listed for
//! US East (N. Virginia): us-east-1.</p>
//! </li>
//! </ul>
//! <p>Alternatively, you can use one of the Amazon Web Services SDKs to access an API that's tailored to the
//! programming language or platform that you're using. For more information, see <a href="http://aws.amazon.com/tools/#SDKs">Amazon Web Services SDKs</a>.</p>
//! <p>We currently provide two versions of the WAF API: this API and the prior versions,
//! the classic WAF APIs. This new API provides the same functionality as the older versions,
//! with the following major improvements:</p>
//! <ul>
//! <li>
//! <p>You use one API for both global and regional applications. Where you need to
//! distinguish the scope, you specify a <code>Scope</code> parameter and set it to
//! <code>CLOUDFRONT</code> or <code>REGIONAL</code>. </p>
//! </li>
//! <li>
//! <p>You can define a web ACL or rule group with a single call, and update it with a
//! single call. You define all rule specifications in JSON format, and pass them to your
//! rule group or web ACL calls.</p>
//! </li>
//! <li>
//! <p>The limits WAF places on the use of rules more closely reflects the cost of
//! running each type of rule. Rule groups include capacity settings, so you know the
//! maximum cost of a rule group when you use it.</p>
//! </li>
//! </ul>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
#[cfg(feature = "client")]
pub mod client;
/// Configuration for the service.
pub mod config;
/// Errors that can occur when calling the service.
pub mod error;
mod error_meta;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
/// Data structures used by operation inputs/outputs.
pub mod model;
mod no_credentials;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use aws_smithy_http::byte_stream::ByteStream;
pub use aws_smithy_http::result::SdkError;
pub use aws_smithy_types::Blob;
pub use aws_smithy_types::DateTime;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("wafv2", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
