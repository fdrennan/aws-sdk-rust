#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>Directory Service</fullname>
//! <p>Directory Service is a web service that makes it easy for you to setup and run directories in the
//! Amazon Web Services cloud, or connect your Amazon Web Services resources with an existing self-managed Microsoft Active
//! Directory. This guide provides detailed information about Directory Service operations, data types,
//! parameters, and errors. For information about Directory Services features, see <a href="https://aws.amazon.com/directoryservice/">Directory Service</a> and the <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/what_is.html">Directory Service
//! Administration Guide</a>.</p>
//! <note>
//! <p>Amazon Web Services provides SDKs that consist of libraries and sample code for various
//! programming languages and platforms (Java, Ruby, .Net, iOS, Android, etc.). The SDKs
//! provide a convenient way to create programmatic access to Directory Service and other Amazon Web Services
//! services. For more information about the Amazon Web Services SDKs, including how to download and
//! install them, see <a href="http://aws.amazon.com/tools/">Tools for Amazon Web
//! Services</a>.</p>
//! </note>

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
    aws_http::user_agent::ApiMetadata::new("directoryservice", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
