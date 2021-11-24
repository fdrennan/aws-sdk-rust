// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Represents the Amazon Simple Storage Service (Amazon S3) location where AWS Application Cost Profiler
/// reports are generated and then written to.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct S3Location {
    /// <p>Name of the S3 bucket.</p>
    pub bucket: std::option::Option<std::string::String>,
    /// <p>Prefix for the location to write to.</p>
    pub prefix: std::option::Option<std::string::String>,
}
impl S3Location {
    /// <p>Name of the S3 bucket.</p>
    pub fn bucket(&self) -> std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>Prefix for the location to write to.</p>
    pub fn prefix(&self) -> std::option::Option<&str> {
        self.prefix.as_deref()
    }
}
impl std::fmt::Debug for S3Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("S3Location");
        formatter.field("bucket", &self.bucket);
        formatter.field("prefix", &self.prefix);
        formatter.finish()
    }
}
/// See [`S3Location`](crate::model::S3Location)
pub mod s3_location {
    /// A builder for [`S3Location`](crate::model::S3Location)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) bucket: std::option::Option<std::string::String>,
        pub(crate) prefix: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>Name of the S3 bucket.</p>
        pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
            self.bucket = Some(input.into());
            self
        }
        /// <p>Name of the S3 bucket.</p>
        pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.bucket = input;
            self
        }
        /// <p>Prefix for the location to write to.</p>
        pub fn prefix(mut self, input: impl Into<std::string::String>) -> Self {
            self.prefix = Some(input.into());
            self
        }
        /// <p>Prefix for the location to write to.</p>
        pub fn set_prefix(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.prefix = input;
            self
        }
        /// Consumes the builder and constructs a [`S3Location`](crate::model::S3Location)
        pub fn build(self) -> crate::model::S3Location {
            crate::model::S3Location {
                bucket: self.bucket,
                prefix: self.prefix,
            }
        }
    }
}
impl S3Location {
    /// Creates a new builder-style object to manufacture [`S3Location`](crate::model::S3Location)
    pub fn builder() -> crate::model::s3_location::Builder {
        crate::model::s3_location::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum Format {
    #[allow(missing_docs)] // documentation missing in model
    Csv,
    #[allow(missing_docs)] // documentation missing in model
    Parquet,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for Format {
    fn from(s: &str) -> Self {
        match s {
            "CSV" => Format::Csv,
            "PARQUET" => Format::Parquet,
            other => Format::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for Format {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Format::from(s))
    }
}
impl Format {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            Format::Csv => "CSV",
            Format::Parquet => "PARQUET",
            Format::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["CSV", "PARQUET"]
    }
}
impl AsRef<str> for Format {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum ReportFrequency {
    #[allow(missing_docs)] // documentation missing in model
    All,
    #[allow(missing_docs)] // documentation missing in model
    Daily,
    #[allow(missing_docs)] // documentation missing in model
    Monthly,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ReportFrequency {
    fn from(s: &str) -> Self {
        match s {
            "ALL" => ReportFrequency::All,
            "DAILY" => ReportFrequency::Daily,
            "MONTHLY" => ReportFrequency::Monthly,
            other => ReportFrequency::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ReportFrequency {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ReportFrequency::from(s))
    }
}
impl ReportFrequency {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ReportFrequency::All => "ALL",
            ReportFrequency::Daily => "DAILY",
            ReportFrequency::Monthly => "MONTHLY",
            ReportFrequency::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["ALL", "DAILY", "MONTHLY"]
    }
}
impl AsRef<str> for ReportFrequency {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>The configuration of a report in AWS Application Cost Profiler.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ReportDefinition {
    /// <p>The ID of the report.</p>
    pub report_id: std::option::Option<std::string::String>,
    /// <p>Description of the report</p>
    pub report_description: std::option::Option<std::string::String>,
    /// <p>The cadence at which the report is generated.</p>
    pub report_frequency: std::option::Option<crate::model::ReportFrequency>,
    /// <p>The format used for the generated reports.</p>
    pub format: std::option::Option<crate::model::Format>,
    /// <p>The location in Amazon Simple Storage Service (Amazon S3) the reports should be saved to.</p>
    pub destination_s3_location: std::option::Option<crate::model::S3Location>,
    /// <p>Timestamp (milliseconds) when this report definition was created.</p>
    pub created_at: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>Timestamp (milliseconds) when this report definition was last updated.</p>
    pub last_updated_at: std::option::Option<aws_smithy_types::DateTime>,
}
impl ReportDefinition {
    /// <p>The ID of the report.</p>
    pub fn report_id(&self) -> std::option::Option<&str> {
        self.report_id.as_deref()
    }
    /// <p>Description of the report</p>
    pub fn report_description(&self) -> std::option::Option<&str> {
        self.report_description.as_deref()
    }
    /// <p>The cadence at which the report is generated.</p>
    pub fn report_frequency(&self) -> std::option::Option<&crate::model::ReportFrequency> {
        self.report_frequency.as_ref()
    }
    /// <p>The format used for the generated reports.</p>
    pub fn format(&self) -> std::option::Option<&crate::model::Format> {
        self.format.as_ref()
    }
    /// <p>The location in Amazon Simple Storage Service (Amazon S3) the reports should be saved to.</p>
    pub fn destination_s3_location(&self) -> std::option::Option<&crate::model::S3Location> {
        self.destination_s3_location.as_ref()
    }
    /// <p>Timestamp (milliseconds) when this report definition was created.</p>
    pub fn created_at(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.created_at.as_ref()
    }
    /// <p>Timestamp (milliseconds) when this report definition was last updated.</p>
    pub fn last_updated_at(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.last_updated_at.as_ref()
    }
}
impl std::fmt::Debug for ReportDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ReportDefinition");
        formatter.field("report_id", &self.report_id);
        formatter.field("report_description", &self.report_description);
        formatter.field("report_frequency", &self.report_frequency);
        formatter.field("format", &self.format);
        formatter.field("destination_s3_location", &self.destination_s3_location);
        formatter.field("created_at", &self.created_at);
        formatter.field("last_updated_at", &self.last_updated_at);
        formatter.finish()
    }
}
/// See [`ReportDefinition`](crate::model::ReportDefinition)
pub mod report_definition {
    /// A builder for [`ReportDefinition`](crate::model::ReportDefinition)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) report_id: std::option::Option<std::string::String>,
        pub(crate) report_description: std::option::Option<std::string::String>,
        pub(crate) report_frequency: std::option::Option<crate::model::ReportFrequency>,
        pub(crate) format: std::option::Option<crate::model::Format>,
        pub(crate) destination_s3_location: std::option::Option<crate::model::S3Location>,
        pub(crate) created_at: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) last_updated_at: std::option::Option<aws_smithy_types::DateTime>,
    }
    impl Builder {
        /// <p>The ID of the report.</p>
        pub fn report_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.report_id = Some(input.into());
            self
        }
        /// <p>The ID of the report.</p>
        pub fn set_report_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.report_id = input;
            self
        }
        /// <p>Description of the report</p>
        pub fn report_description(mut self, input: impl Into<std::string::String>) -> Self {
            self.report_description = Some(input.into());
            self
        }
        /// <p>Description of the report</p>
        pub fn set_report_description(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.report_description = input;
            self
        }
        /// <p>The cadence at which the report is generated.</p>
        pub fn report_frequency(mut self, input: crate::model::ReportFrequency) -> Self {
            self.report_frequency = Some(input);
            self
        }
        /// <p>The cadence at which the report is generated.</p>
        pub fn set_report_frequency(
            mut self,
            input: std::option::Option<crate::model::ReportFrequency>,
        ) -> Self {
            self.report_frequency = input;
            self
        }
        /// <p>The format used for the generated reports.</p>
        pub fn format(mut self, input: crate::model::Format) -> Self {
            self.format = Some(input);
            self
        }
        /// <p>The format used for the generated reports.</p>
        pub fn set_format(mut self, input: std::option::Option<crate::model::Format>) -> Self {
            self.format = input;
            self
        }
        /// <p>The location in Amazon Simple Storage Service (Amazon S3) the reports should be saved to.</p>
        pub fn destination_s3_location(mut self, input: crate::model::S3Location) -> Self {
            self.destination_s3_location = Some(input);
            self
        }
        /// <p>The location in Amazon Simple Storage Service (Amazon S3) the reports should be saved to.</p>
        pub fn set_destination_s3_location(
            mut self,
            input: std::option::Option<crate::model::S3Location>,
        ) -> Self {
            self.destination_s3_location = input;
            self
        }
        /// <p>Timestamp (milliseconds) when this report definition was created.</p>
        pub fn created_at(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.created_at = Some(input);
            self
        }
        /// <p>Timestamp (milliseconds) when this report definition was created.</p>
        pub fn set_created_at(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.created_at = input;
            self
        }
        /// <p>Timestamp (milliseconds) when this report definition was last updated.</p>
        pub fn last_updated_at(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.last_updated_at = Some(input);
            self
        }
        /// <p>Timestamp (milliseconds) when this report definition was last updated.</p>
        pub fn set_last_updated_at(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.last_updated_at = input;
            self
        }
        /// Consumes the builder and constructs a [`ReportDefinition`](crate::model::ReportDefinition)
        pub fn build(self) -> crate::model::ReportDefinition {
            crate::model::ReportDefinition {
                report_id: self.report_id,
                report_description: self.report_description,
                report_frequency: self.report_frequency,
                format: self.format,
                destination_s3_location: self.destination_s3_location,
                created_at: self.created_at,
                last_updated_at: self.last_updated_at,
            }
        }
    }
}
impl ReportDefinition {
    /// Creates a new builder-style object to manufacture [`ReportDefinition`](crate::model::ReportDefinition)
    pub fn builder() -> crate::model::report_definition::Builder {
        crate::model::report_definition::Builder::default()
    }
}

/// <p>Represents the Amazon Simple Storage Service (Amazon S3) location where usage data is read
/// from.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SourceS3Location {
    /// <p>Name of the bucket.</p>
    pub bucket: std::option::Option<std::string::String>,
    /// <p>Key of the object.</p>
    pub key: std::option::Option<std::string::String>,
    /// <p>Region of the bucket. Only required for Regions that are disabled by default.
    /// For more infomration about Regions that are disabled by default, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande-manage.html#rande-manage-enable">
    /// Enabling a Region</a> in the <i>AWS General Reference guide</i>.</p>
    pub region: std::option::Option<crate::model::S3BucketRegion>,
}
impl SourceS3Location {
    /// <p>Name of the bucket.</p>
    pub fn bucket(&self) -> std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>Key of the object.</p>
    pub fn key(&self) -> std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>Region of the bucket. Only required for Regions that are disabled by default.
    /// For more infomration about Regions that are disabled by default, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande-manage.html#rande-manage-enable">
    /// Enabling a Region</a> in the <i>AWS General Reference guide</i>.</p>
    pub fn region(&self) -> std::option::Option<&crate::model::S3BucketRegion> {
        self.region.as_ref()
    }
}
impl std::fmt::Debug for SourceS3Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SourceS3Location");
        formatter.field("bucket", &self.bucket);
        formatter.field("key", &self.key);
        formatter.field("region", &self.region);
        formatter.finish()
    }
}
/// See [`SourceS3Location`](crate::model::SourceS3Location)
pub mod source_s3_location {
    /// A builder for [`SourceS3Location`](crate::model::SourceS3Location)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) bucket: std::option::Option<std::string::String>,
        pub(crate) key: std::option::Option<std::string::String>,
        pub(crate) region: std::option::Option<crate::model::S3BucketRegion>,
    }
    impl Builder {
        /// <p>Name of the bucket.</p>
        pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
            self.bucket = Some(input.into());
            self
        }
        /// <p>Name of the bucket.</p>
        pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.bucket = input;
            self
        }
        /// <p>Key of the object.</p>
        pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
            self.key = Some(input.into());
            self
        }
        /// <p>Key of the object.</p>
        pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.key = input;
            self
        }
        /// <p>Region of the bucket. Only required for Regions that are disabled by default.
        /// For more infomration about Regions that are disabled by default, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande-manage.html#rande-manage-enable">
        /// Enabling a Region</a> in the <i>AWS General Reference guide</i>.</p>
        pub fn region(mut self, input: crate::model::S3BucketRegion) -> Self {
            self.region = Some(input);
            self
        }
        /// <p>Region of the bucket. Only required for Regions that are disabled by default.
        /// For more infomration about Regions that are disabled by default, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande-manage.html#rande-manage-enable">
        /// Enabling a Region</a> in the <i>AWS General Reference guide</i>.</p>
        pub fn set_region(
            mut self,
            input: std::option::Option<crate::model::S3BucketRegion>,
        ) -> Self {
            self.region = input;
            self
        }
        /// Consumes the builder and constructs a [`SourceS3Location`](crate::model::SourceS3Location)
        pub fn build(self) -> crate::model::SourceS3Location {
            crate::model::SourceS3Location {
                bucket: self.bucket,
                key: self.key,
                region: self.region,
            }
        }
    }
}
impl SourceS3Location {
    /// Creates a new builder-style object to manufacture [`SourceS3Location`](crate::model::SourceS3Location)
    pub fn builder() -> crate::model::source_s3_location::Builder {
        crate::model::source_s3_location::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum S3BucketRegion {
    #[allow(missing_docs)] // documentation missing in model
    AfSouth1,
    #[allow(missing_docs)] // documentation missing in model
    ApEast1,
    #[allow(missing_docs)] // documentation missing in model
    EuSouth1,
    #[allow(missing_docs)] // documentation missing in model
    MeSouth1,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for S3BucketRegion {
    fn from(s: &str) -> Self {
        match s {
            "af-south-1" => S3BucketRegion::AfSouth1,
            "ap-east-1" => S3BucketRegion::ApEast1,
            "eu-south-1" => S3BucketRegion::EuSouth1,
            "me-south-1" => S3BucketRegion::MeSouth1,
            other => S3BucketRegion::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for S3BucketRegion {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(S3BucketRegion::from(s))
    }
}
impl S3BucketRegion {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            S3BucketRegion::AfSouth1 => "af-south-1",
            S3BucketRegion::ApEast1 => "ap-east-1",
            S3BucketRegion::EuSouth1 => "eu-south-1",
            S3BucketRegion::MeSouth1 => "me-south-1",
            S3BucketRegion::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["af-south-1", "ap-east-1", "eu-south-1", "me-south-1"]
    }
}
impl AsRef<str> for S3BucketRegion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
