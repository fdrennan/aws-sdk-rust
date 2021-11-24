// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct StartSnapshotOutput {
    /// <p>The description of the snapshot.</p>
    pub description: std::option::Option<std::string::String>,
    /// <p>The ID of the snapshot.</p>
    pub snapshot_id: std::option::Option<std::string::String>,
    /// <p>The Amazon Web Services account ID of the snapshot owner.</p>
    pub owner_id: std::option::Option<std::string::String>,
    /// <p>The status of the snapshot.</p>
    pub status: std::option::Option<crate::model::Status>,
    /// <p>The timestamp when the snapshot was created.</p>
    pub start_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The size of the volume, in GiB.</p>
    pub volume_size: std::option::Option<i64>,
    /// <p>The size of the blocks in the snapshot, in bytes.</p>
    pub block_size: std::option::Option<i32>,
    /// <p>The tags applied to the snapshot. You can specify up to 50 tags per snapshot. For more
    /// information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html"> Tagging your Amazon EC2
    /// resources</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
    /// <p>The ID of the parent snapshot.</p>
    pub parent_snapshot_id: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the Key Management Service (KMS) key used to encrypt the snapshot.</p>
    pub kms_key_arn: std::option::Option<std::string::String>,
}
impl StartSnapshotOutput {
    /// <p>The description of the snapshot.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The ID of the snapshot.</p>
    pub fn snapshot_id(&self) -> std::option::Option<&str> {
        self.snapshot_id.as_deref()
    }
    /// <p>The Amazon Web Services account ID of the snapshot owner.</p>
    pub fn owner_id(&self) -> std::option::Option<&str> {
        self.owner_id.as_deref()
    }
    /// <p>The status of the snapshot.</p>
    pub fn status(&self) -> std::option::Option<&crate::model::Status> {
        self.status.as_ref()
    }
    /// <p>The timestamp when the snapshot was created.</p>
    pub fn start_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.start_time.as_ref()
    }
    /// <p>The size of the volume, in GiB.</p>
    pub fn volume_size(&self) -> std::option::Option<i64> {
        self.volume_size
    }
    /// <p>The size of the blocks in the snapshot, in bytes.</p>
    pub fn block_size(&self) -> std::option::Option<i32> {
        self.block_size
    }
    /// <p>The tags applied to the snapshot. You can specify up to 50 tags per snapshot. For more
    /// information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html"> Tagging your Amazon EC2
    /// resources</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::model::Tag]> {
        self.tags.as_deref()
    }
    /// <p>The ID of the parent snapshot.</p>
    pub fn parent_snapshot_id(&self) -> std::option::Option<&str> {
        self.parent_snapshot_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the Key Management Service (KMS) key used to encrypt the snapshot.</p>
    pub fn kms_key_arn(&self) -> std::option::Option<&str> {
        self.kms_key_arn.as_deref()
    }
}
impl std::fmt::Debug for StartSnapshotOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("StartSnapshotOutput");
        formatter.field("description", &self.description);
        formatter.field("snapshot_id", &self.snapshot_id);
        formatter.field("owner_id", &self.owner_id);
        formatter.field("status", &self.status);
        formatter.field("start_time", &self.start_time);
        formatter.field("volume_size", &self.volume_size);
        formatter.field("block_size", &self.block_size);
        formatter.field("tags", &self.tags);
        formatter.field("parent_snapshot_id", &self.parent_snapshot_id);
        formatter.field("kms_key_arn", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
/// See [`StartSnapshotOutput`](crate::output::StartSnapshotOutput)
pub mod start_snapshot_output {
    /// A builder for [`StartSnapshotOutput`](crate::output::StartSnapshotOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) description: std::option::Option<std::string::String>,
        pub(crate) snapshot_id: std::option::Option<std::string::String>,
        pub(crate) owner_id: std::option::Option<std::string::String>,
        pub(crate) status: std::option::Option<crate::model::Status>,
        pub(crate) start_time: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) volume_size: std::option::Option<i64>,
        pub(crate) block_size: std::option::Option<i32>,
        pub(crate) tags: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        pub(crate) parent_snapshot_id: std::option::Option<std::string::String>,
        pub(crate) kms_key_arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The description of the snapshot.</p>
        pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
            self.description = Some(input.into());
            self
        }
        /// <p>The description of the snapshot.</p>
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.description = input;
            self
        }
        /// <p>The ID of the snapshot.</p>
        pub fn snapshot_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.snapshot_id = Some(input.into());
            self
        }
        /// <p>The ID of the snapshot.</p>
        pub fn set_snapshot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.snapshot_id = input;
            self
        }
        /// <p>The Amazon Web Services account ID of the snapshot owner.</p>
        pub fn owner_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.owner_id = Some(input.into());
            self
        }
        /// <p>The Amazon Web Services account ID of the snapshot owner.</p>
        pub fn set_owner_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.owner_id = input;
            self
        }
        /// <p>The status of the snapshot.</p>
        pub fn status(mut self, input: crate::model::Status) -> Self {
            self.status = Some(input);
            self
        }
        /// <p>The status of the snapshot.</p>
        pub fn set_status(mut self, input: std::option::Option<crate::model::Status>) -> Self {
            self.status = input;
            self
        }
        /// <p>The timestamp when the snapshot was created.</p>
        pub fn start_time(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.start_time = Some(input);
            self
        }
        /// <p>The timestamp when the snapshot was created.</p>
        pub fn set_start_time(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.start_time = input;
            self
        }
        /// <p>The size of the volume, in GiB.</p>
        pub fn volume_size(mut self, input: i64) -> Self {
            self.volume_size = Some(input);
            self
        }
        /// <p>The size of the volume, in GiB.</p>
        pub fn set_volume_size(mut self, input: std::option::Option<i64>) -> Self {
            self.volume_size = input;
            self
        }
        /// <p>The size of the blocks in the snapshot, in bytes.</p>
        pub fn block_size(mut self, input: i32) -> Self {
            self.block_size = Some(input);
            self
        }
        /// <p>The size of the blocks in the snapshot, in bytes.</p>
        pub fn set_block_size(mut self, input: std::option::Option<i32>) -> Self {
            self.block_size = input;
            self
        }
        /// Appends an item to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>The tags applied to the snapshot. You can specify up to 50 tags per snapshot. For more
        /// information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html"> Tagging your Amazon EC2
        /// resources</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
        pub fn tags(mut self, input: impl Into<crate::model::Tag>) -> Self {
            let mut v = self.tags.unwrap_or_default();
            v.push(input.into());
            self.tags = Some(v);
            self
        }
        /// <p>The tags applied to the snapshot. You can specify up to 50 tags per snapshot. For more
        /// information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html"> Tagging your Amazon EC2
        /// resources</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        ) -> Self {
            self.tags = input;
            self
        }
        /// <p>The ID of the parent snapshot.</p>
        pub fn parent_snapshot_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.parent_snapshot_id = Some(input.into());
            self
        }
        /// <p>The ID of the parent snapshot.</p>
        pub fn set_parent_snapshot_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.parent_snapshot_id = input;
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the Key Management Service (KMS) key used to encrypt the snapshot.</p>
        pub fn kms_key_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.kms_key_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the Key Management Service (KMS) key used to encrypt the snapshot.</p>
        pub fn set_kms_key_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.kms_key_arn = input;
            self
        }
        /// Consumes the builder and constructs a [`StartSnapshotOutput`](crate::output::StartSnapshotOutput)
        pub fn build(self) -> crate::output::StartSnapshotOutput {
            crate::output::StartSnapshotOutput {
                description: self.description,
                snapshot_id: self.snapshot_id,
                owner_id: self.owner_id,
                status: self.status,
                start_time: self.start_time,
                volume_size: self.volume_size,
                block_size: self.block_size,
                tags: self.tags,
                parent_snapshot_id: self.parent_snapshot_id,
                kms_key_arn: self.kms_key_arn,
            }
        }
    }
}
impl StartSnapshotOutput {
    /// Creates a new builder-style object to manufacture [`StartSnapshotOutput`](crate::output::StartSnapshotOutput)
    pub fn builder() -> crate::output::start_snapshot_output::Builder {
        crate::output::start_snapshot_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PutSnapshotBlockOutput {
    /// <p>The SHA256 checksum generated for the block data by Amazon EBS.</p>
    pub checksum: std::option::Option<std::string::String>,
    /// <p>The algorithm used by Amazon EBS to generate the checksum.</p>
    pub checksum_algorithm: std::option::Option<crate::model::ChecksumAlgorithm>,
}
impl PutSnapshotBlockOutput {
    /// <p>The SHA256 checksum generated for the block data by Amazon EBS.</p>
    pub fn checksum(&self) -> std::option::Option<&str> {
        self.checksum.as_deref()
    }
    /// <p>The algorithm used by Amazon EBS to generate the checksum.</p>
    pub fn checksum_algorithm(&self) -> std::option::Option<&crate::model::ChecksumAlgorithm> {
        self.checksum_algorithm.as_ref()
    }
}
impl std::fmt::Debug for PutSnapshotBlockOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("PutSnapshotBlockOutput");
        formatter.field("checksum", &self.checksum);
        formatter.field("checksum_algorithm", &self.checksum_algorithm);
        formatter.finish()
    }
}
/// See [`PutSnapshotBlockOutput`](crate::output::PutSnapshotBlockOutput)
pub mod put_snapshot_block_output {
    /// A builder for [`PutSnapshotBlockOutput`](crate::output::PutSnapshotBlockOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) checksum: std::option::Option<std::string::String>,
        pub(crate) checksum_algorithm: std::option::Option<crate::model::ChecksumAlgorithm>,
    }
    impl Builder {
        /// <p>The SHA256 checksum generated for the block data by Amazon EBS.</p>
        pub fn checksum(mut self, input: impl Into<std::string::String>) -> Self {
            self.checksum = Some(input.into());
            self
        }
        /// <p>The SHA256 checksum generated for the block data by Amazon EBS.</p>
        pub fn set_checksum(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.checksum = input;
            self
        }
        /// <p>The algorithm used by Amazon EBS to generate the checksum.</p>
        pub fn checksum_algorithm(mut self, input: crate::model::ChecksumAlgorithm) -> Self {
            self.checksum_algorithm = Some(input);
            self
        }
        /// <p>The algorithm used by Amazon EBS to generate the checksum.</p>
        pub fn set_checksum_algorithm(
            mut self,
            input: std::option::Option<crate::model::ChecksumAlgorithm>,
        ) -> Self {
            self.checksum_algorithm = input;
            self
        }
        /// Consumes the builder and constructs a [`PutSnapshotBlockOutput`](crate::output::PutSnapshotBlockOutput)
        pub fn build(self) -> crate::output::PutSnapshotBlockOutput {
            crate::output::PutSnapshotBlockOutput {
                checksum: self.checksum,
                checksum_algorithm: self.checksum_algorithm,
            }
        }
    }
}
impl PutSnapshotBlockOutput {
    /// Creates a new builder-style object to manufacture [`PutSnapshotBlockOutput`](crate::output::PutSnapshotBlockOutput)
    pub fn builder() -> crate::output::put_snapshot_block_output::Builder {
        crate::output::put_snapshot_block_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListSnapshotBlocksOutput {
    /// <p>An array of objects containing information about the blocks.</p>
    pub blocks: std::option::Option<std::vec::Vec<crate::model::Block>>,
    /// <p>The time when the <code>BlockToken</code> expires.</p>
    pub expiry_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The size of the volume in GB.</p>
    pub volume_size: std::option::Option<i64>,
    /// <p>The size of the blocks in the snapshot, in bytes.</p>
    pub block_size: std::option::Option<i32>,
    /// <p>The token to use to retrieve the next page of results. This value is null when there
    /// are no more results to return.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl ListSnapshotBlocksOutput {
    /// <p>An array of objects containing information about the blocks.</p>
    pub fn blocks(&self) -> std::option::Option<&[crate::model::Block]> {
        self.blocks.as_deref()
    }
    /// <p>The time when the <code>BlockToken</code> expires.</p>
    pub fn expiry_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.expiry_time.as_ref()
    }
    /// <p>The size of the volume in GB.</p>
    pub fn volume_size(&self) -> std::option::Option<i64> {
        self.volume_size
    }
    /// <p>The size of the blocks in the snapshot, in bytes.</p>
    pub fn block_size(&self) -> std::option::Option<i32> {
        self.block_size
    }
    /// <p>The token to use to retrieve the next page of results. This value is null when there
    /// are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for ListSnapshotBlocksOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListSnapshotBlocksOutput");
        formatter.field("blocks", &"*** Sensitive Data Redacted ***");
        formatter.field("expiry_time", &self.expiry_time);
        formatter.field("volume_size", &self.volume_size);
        formatter.field("block_size", &self.block_size);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListSnapshotBlocksOutput`](crate::output::ListSnapshotBlocksOutput)
pub mod list_snapshot_blocks_output {
    /// A builder for [`ListSnapshotBlocksOutput`](crate::output::ListSnapshotBlocksOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) blocks: std::option::Option<std::vec::Vec<crate::model::Block>>,
        pub(crate) expiry_time: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) volume_size: std::option::Option<i64>,
        pub(crate) block_size: std::option::Option<i32>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `blocks`.
        ///
        /// To override the contents of this collection use [`set_blocks`](Self::set_blocks).
        ///
        /// <p>An array of objects containing information about the blocks.</p>
        pub fn blocks(mut self, input: impl Into<crate::model::Block>) -> Self {
            let mut v = self.blocks.unwrap_or_default();
            v.push(input.into());
            self.blocks = Some(v);
            self
        }
        /// <p>An array of objects containing information about the blocks.</p>
        pub fn set_blocks(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Block>>,
        ) -> Self {
            self.blocks = input;
            self
        }
        /// <p>The time when the <code>BlockToken</code> expires.</p>
        pub fn expiry_time(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.expiry_time = Some(input);
            self
        }
        /// <p>The time when the <code>BlockToken</code> expires.</p>
        pub fn set_expiry_time(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.expiry_time = input;
            self
        }
        /// <p>The size of the volume in GB.</p>
        pub fn volume_size(mut self, input: i64) -> Self {
            self.volume_size = Some(input);
            self
        }
        /// <p>The size of the volume in GB.</p>
        pub fn set_volume_size(mut self, input: std::option::Option<i64>) -> Self {
            self.volume_size = input;
            self
        }
        /// <p>The size of the blocks in the snapshot, in bytes.</p>
        pub fn block_size(mut self, input: i32) -> Self {
            self.block_size = Some(input);
            self
        }
        /// <p>The size of the blocks in the snapshot, in bytes.</p>
        pub fn set_block_size(mut self, input: std::option::Option<i32>) -> Self {
            self.block_size = input;
            self
        }
        /// <p>The token to use to retrieve the next page of results. This value is null when there
        /// are no more results to return.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token to use to retrieve the next page of results. This value is null when there
        /// are no more results to return.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListSnapshotBlocksOutput`](crate::output::ListSnapshotBlocksOutput)
        pub fn build(self) -> crate::output::ListSnapshotBlocksOutput {
            crate::output::ListSnapshotBlocksOutput {
                blocks: self.blocks,
                expiry_time: self.expiry_time,
                volume_size: self.volume_size,
                block_size: self.block_size,
                next_token: self.next_token,
            }
        }
    }
}
impl ListSnapshotBlocksOutput {
    /// Creates a new builder-style object to manufacture [`ListSnapshotBlocksOutput`](crate::output::ListSnapshotBlocksOutput)
    pub fn builder() -> crate::output::list_snapshot_blocks_output::Builder {
        crate::output::list_snapshot_blocks_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListChangedBlocksOutput {
    /// <p>An array of objects containing information about the changed blocks.</p>
    pub changed_blocks: std::option::Option<std::vec::Vec<crate::model::ChangedBlock>>,
    /// <p>The time when the <code>BlockToken</code> expires.</p>
    pub expiry_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The size of the volume in GB.</p>
    pub volume_size: std::option::Option<i64>,
    /// <p>The size of the blocks in the snapshot, in bytes.</p>
    pub block_size: std::option::Option<i32>,
    /// <p>The token to use to retrieve the next page of results. This value is null when there
    /// are no more results to return.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl ListChangedBlocksOutput {
    /// <p>An array of objects containing information about the changed blocks.</p>
    pub fn changed_blocks(&self) -> std::option::Option<&[crate::model::ChangedBlock]> {
        self.changed_blocks.as_deref()
    }
    /// <p>The time when the <code>BlockToken</code> expires.</p>
    pub fn expiry_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.expiry_time.as_ref()
    }
    /// <p>The size of the volume in GB.</p>
    pub fn volume_size(&self) -> std::option::Option<i64> {
        self.volume_size
    }
    /// <p>The size of the blocks in the snapshot, in bytes.</p>
    pub fn block_size(&self) -> std::option::Option<i32> {
        self.block_size
    }
    /// <p>The token to use to retrieve the next page of results. This value is null when there
    /// are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for ListChangedBlocksOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListChangedBlocksOutput");
        formatter.field("changed_blocks", &self.changed_blocks);
        formatter.field("expiry_time", &self.expiry_time);
        formatter.field("volume_size", &self.volume_size);
        formatter.field("block_size", &self.block_size);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListChangedBlocksOutput`](crate::output::ListChangedBlocksOutput)
pub mod list_changed_blocks_output {
    /// A builder for [`ListChangedBlocksOutput`](crate::output::ListChangedBlocksOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) changed_blocks: std::option::Option<std::vec::Vec<crate::model::ChangedBlock>>,
        pub(crate) expiry_time: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) volume_size: std::option::Option<i64>,
        pub(crate) block_size: std::option::Option<i32>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `changed_blocks`.
        ///
        /// To override the contents of this collection use [`set_changed_blocks`](Self::set_changed_blocks).
        ///
        /// <p>An array of objects containing information about the changed blocks.</p>
        pub fn changed_blocks(mut self, input: impl Into<crate::model::ChangedBlock>) -> Self {
            let mut v = self.changed_blocks.unwrap_or_default();
            v.push(input.into());
            self.changed_blocks = Some(v);
            self
        }
        /// <p>An array of objects containing information about the changed blocks.</p>
        pub fn set_changed_blocks(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ChangedBlock>>,
        ) -> Self {
            self.changed_blocks = input;
            self
        }
        /// <p>The time when the <code>BlockToken</code> expires.</p>
        pub fn expiry_time(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.expiry_time = Some(input);
            self
        }
        /// <p>The time when the <code>BlockToken</code> expires.</p>
        pub fn set_expiry_time(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.expiry_time = input;
            self
        }
        /// <p>The size of the volume in GB.</p>
        pub fn volume_size(mut self, input: i64) -> Self {
            self.volume_size = Some(input);
            self
        }
        /// <p>The size of the volume in GB.</p>
        pub fn set_volume_size(mut self, input: std::option::Option<i64>) -> Self {
            self.volume_size = input;
            self
        }
        /// <p>The size of the blocks in the snapshot, in bytes.</p>
        pub fn block_size(mut self, input: i32) -> Self {
            self.block_size = Some(input);
            self
        }
        /// <p>The size of the blocks in the snapshot, in bytes.</p>
        pub fn set_block_size(mut self, input: std::option::Option<i32>) -> Self {
            self.block_size = input;
            self
        }
        /// <p>The token to use to retrieve the next page of results. This value is null when there
        /// are no more results to return.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token to use to retrieve the next page of results. This value is null when there
        /// are no more results to return.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListChangedBlocksOutput`](crate::output::ListChangedBlocksOutput)
        pub fn build(self) -> crate::output::ListChangedBlocksOutput {
            crate::output::ListChangedBlocksOutput {
                changed_blocks: self.changed_blocks,
                expiry_time: self.expiry_time,
                volume_size: self.volume_size,
                block_size: self.block_size,
                next_token: self.next_token,
            }
        }
    }
}
impl ListChangedBlocksOutput {
    /// Creates a new builder-style object to manufacture [`ListChangedBlocksOutput`](crate::output::ListChangedBlocksOutput)
    pub fn builder() -> crate::output::list_changed_blocks_output::Builder {
        crate::output::list_changed_blocks_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
pub struct GetSnapshotBlockOutput {
    /// <p>The size of the data in the block.</p>
    pub data_length: std::option::Option<i32>,
    /// <p>The data content of the block.</p>
    pub block_data: aws_smithy_http::byte_stream::ByteStream,
    /// <p>The checksum generated for the block, which is Base64 encoded.</p>
    pub checksum: std::option::Option<std::string::String>,
    /// <p>The algorithm used to generate the checksum for the block, such as SHA256.</p>
    pub checksum_algorithm: std::option::Option<crate::model::ChecksumAlgorithm>,
}
impl GetSnapshotBlockOutput {
    /// <p>The size of the data in the block.</p>
    pub fn data_length(&self) -> std::option::Option<i32> {
        self.data_length
    }
    /// <p>The data content of the block.</p>
    pub fn block_data(&self) -> &aws_smithy_http::byte_stream::ByteStream {
        &self.block_data
    }
    /// <p>The checksum generated for the block, which is Base64 encoded.</p>
    pub fn checksum(&self) -> std::option::Option<&str> {
        self.checksum.as_deref()
    }
    /// <p>The algorithm used to generate the checksum for the block, such as SHA256.</p>
    pub fn checksum_algorithm(&self) -> std::option::Option<&crate::model::ChecksumAlgorithm> {
        self.checksum_algorithm.as_ref()
    }
}
impl std::fmt::Debug for GetSnapshotBlockOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetSnapshotBlockOutput");
        formatter.field("data_length", &self.data_length);
        formatter.field("block_data", &"*** Sensitive Data Redacted ***");
        formatter.field("checksum", &self.checksum);
        formatter.field("checksum_algorithm", &self.checksum_algorithm);
        formatter.finish()
    }
}
/// See [`GetSnapshotBlockOutput`](crate::output::GetSnapshotBlockOutput)
pub mod get_snapshot_block_output {
    /// A builder for [`GetSnapshotBlockOutput`](crate::output::GetSnapshotBlockOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) data_length: std::option::Option<i32>,
        pub(crate) block_data: std::option::Option<aws_smithy_http::byte_stream::ByteStream>,
        pub(crate) checksum: std::option::Option<std::string::String>,
        pub(crate) checksum_algorithm: std::option::Option<crate::model::ChecksumAlgorithm>,
    }
    impl Builder {
        /// <p>The size of the data in the block.</p>
        pub fn data_length(mut self, input: i32) -> Self {
            self.data_length = Some(input);
            self
        }
        /// <p>The size of the data in the block.</p>
        pub fn set_data_length(mut self, input: std::option::Option<i32>) -> Self {
            self.data_length = input;
            self
        }
        /// <p>The data content of the block.</p>
        pub fn block_data(mut self, input: aws_smithy_http::byte_stream::ByteStream) -> Self {
            self.block_data = Some(input);
            self
        }
        /// <p>The data content of the block.</p>
        pub fn set_block_data(
            mut self,
            input: std::option::Option<aws_smithy_http::byte_stream::ByteStream>,
        ) -> Self {
            self.block_data = input;
            self
        }
        /// <p>The checksum generated for the block, which is Base64 encoded.</p>
        pub fn checksum(mut self, input: impl Into<std::string::String>) -> Self {
            self.checksum = Some(input.into());
            self
        }
        /// <p>The checksum generated for the block, which is Base64 encoded.</p>
        pub fn set_checksum(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.checksum = input;
            self
        }
        /// <p>The algorithm used to generate the checksum for the block, such as SHA256.</p>
        pub fn checksum_algorithm(mut self, input: crate::model::ChecksumAlgorithm) -> Self {
            self.checksum_algorithm = Some(input);
            self
        }
        /// <p>The algorithm used to generate the checksum for the block, such as SHA256.</p>
        pub fn set_checksum_algorithm(
            mut self,
            input: std::option::Option<crate::model::ChecksumAlgorithm>,
        ) -> Self {
            self.checksum_algorithm = input;
            self
        }
        /// Consumes the builder and constructs a [`GetSnapshotBlockOutput`](crate::output::GetSnapshotBlockOutput)
        pub fn build(self) -> crate::output::GetSnapshotBlockOutput {
            crate::output::GetSnapshotBlockOutput {
                data_length: self.data_length,
                block_data: self.block_data.unwrap_or_default(),
                checksum: self.checksum,
                checksum_algorithm: self.checksum_algorithm,
            }
        }
    }
}
impl GetSnapshotBlockOutput {
    /// Creates a new builder-style object to manufacture [`GetSnapshotBlockOutput`](crate::output::GetSnapshotBlockOutput)
    pub fn builder() -> crate::output::get_snapshot_block_output::Builder {
        crate::output::get_snapshot_block_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CompleteSnapshotOutput {
    /// <p>The status of the snapshot.</p>
    pub status: std::option::Option<crate::model::Status>,
}
impl CompleteSnapshotOutput {
    /// <p>The status of the snapshot.</p>
    pub fn status(&self) -> std::option::Option<&crate::model::Status> {
        self.status.as_ref()
    }
}
impl std::fmt::Debug for CompleteSnapshotOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CompleteSnapshotOutput");
        formatter.field("status", &self.status);
        formatter.finish()
    }
}
/// See [`CompleteSnapshotOutput`](crate::output::CompleteSnapshotOutput)
pub mod complete_snapshot_output {
    /// A builder for [`CompleteSnapshotOutput`](crate::output::CompleteSnapshotOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) status: std::option::Option<crate::model::Status>,
    }
    impl Builder {
        /// <p>The status of the snapshot.</p>
        pub fn status(mut self, input: crate::model::Status) -> Self {
            self.status = Some(input);
            self
        }
        /// <p>The status of the snapshot.</p>
        pub fn set_status(mut self, input: std::option::Option<crate::model::Status>) -> Self {
            self.status = input;
            self
        }
        /// Consumes the builder and constructs a [`CompleteSnapshotOutput`](crate::output::CompleteSnapshotOutput)
        pub fn build(self) -> crate::output::CompleteSnapshotOutput {
            crate::output::CompleteSnapshotOutput {
                status: self.status,
            }
        }
    }
}
impl CompleteSnapshotOutput {
    /// Creates a new builder-style object to manufacture [`CompleteSnapshotOutput`](crate::output::CompleteSnapshotOutput)
    pub fn builder() -> crate::output::complete_snapshot_output::Builder {
        crate::output::complete_snapshot_output::Builder::default()
    }
}
