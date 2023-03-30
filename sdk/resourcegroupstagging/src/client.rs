// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[doc(inline)]
                pub use aws_smithy_client::Builder;
#[derive(Debug)]
            pub(crate) struct Handle {
                pub(crate) client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>,
                pub(crate) conf: crate::Config,
            }

            /// Client for AWS Resource Groups Tagging API
                    ///
                    /// Client for invoking operations on AWS Resource Groups Tagging API. Each operation on AWS Resource Groups Tagging API is a method on this
                    /// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
                        /// # Examples
                        /// **Constructing a client and invoking an operation**
                        /// ```rust,no_run
                        /// # async fn docs() {
                        ///     // create a shared configuration. This can be used & shared between multiple service clients.
                        ///     let shared_config = aws_config::load_from_env().await;
                        ///     let client = aws_sdk_resourcegroupstagging::Client::new(&shared_config);
                        ///     // invoke an operation
                        ///     /* let rsp = client
                        ///         .<operation_name>().
                        ///         .<param>("some value")
                        ///         .send().await; */
                        /// # }
                        /// ```
                        /// **Constructing a client with custom configuration**
                        /// ```rust,no_run
                        /// use aws_config::retry::RetryConfig;
                        /// # async fn docs() {
                        /// let shared_config = aws_config::load_from_env().await;
                        /// let config = aws_sdk_resourcegroupstagging::config::Builder::from(&shared_config)
                        ///   .retry_config(RetryConfig::disabled())
                        ///   .build();
                        /// let client = aws_sdk_resourcegroupstagging::Client::from_conf(config);
                        /// # }
            #[derive(std::fmt::Debug)]
            pub struct Client {
                handle: std::sync::Arc<Handle>
            }

            impl std::clone::Clone for Client {
                fn clone(&self) -> Self {
                    Self { handle: self.handle.clone() }
                }
            }

            impl From<aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>> for Client {
                fn from(client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>) -> Self {
                    Self::with_config(client, crate::Config::builder().build())
                }
            }

            impl Client {
                /// Creates a client with the given service configuration.
                pub fn with_config(client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>, conf: crate::Config) -> Self {
                    Self {
                        handle: std::sync::Arc::new(Handle {
                            client,
                            conf,
                        })
                    }
                }

                /// Returns the client's configuration.
                pub fn conf(&self) -> &crate::Config {
                    &self.handle.conf
                }
            }
impl Client  {
    /// Constructs a fluent builder for the [`DescribeReportCreation`](crate::client::fluent_builders::DescribeReportCreation) operation.
                        ///
                        /// - The fluent builder takes no input, just [`send`](crate::client::fluent_builders::DescribeReportCreation::send) it.
                        
                        /// - On success, responds with [`DescribeReportCreationOutput`](crate::output::DescribeReportCreationOutput) with field(s):
                        ///   - [`status(Option<String>)`](crate::output::DescribeReportCreationOutput::status): <p>Reports the status of the operation.</p>  <p>The operation status can be one of the following:</p>  <ul>   <li> <p> <code>RUNNING</code> - Report creation is in progress.</p> </li>   <li> <p> <code>SUCCEEDED</code> - Report creation is complete. You can open the report from the Amazon S3 bucket that you specified when you ran <code>StartReportCreation</code>.</p> </li>   <li> <p> <code>FAILED</code> - Report creation timed out or the Amazon S3 bucket is not accessible. </p> </li>   <li> <p> <code>NO REPORT</code> - No report was generated in the last 90 days.</p> </li>  </ul>
    ///   - [`s3_location(Option<String>)`](crate::output::DescribeReportCreationOutput::s3_location): <p>The path to the Amazon S3 bucket where the report was stored on creation.</p>
    ///   - [`start_date(Option<String>)`](crate::output::DescribeReportCreationOutput::start_date): <p>The date and time that the report was started. </p>
    ///   - [`error_message(Option<String>)`](crate::output::DescribeReportCreationOutput::error_message): <p>Details of the common errors that all operations return.</p>
                        /// - On failure, responds with [`SdkError<DescribeReportCreationError>`](crate::error::DescribeReportCreationError)
    pub fn describe_report_creation(&self) -> crate::client::fluent_builders::DescribeReportCreation {
                            crate::client::fluent_builders::DescribeReportCreation::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`GetComplianceSummary`](crate::client::fluent_builders::GetComplianceSummary) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetComplianceSummary::into_paginator).
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`target_id_filters(Vec<String>)`](crate::client::fluent_builders::GetComplianceSummary::target_id_filters) / [`set_target_id_filters(Option<Vec<String>>)`](crate::client::fluent_builders::GetComplianceSummary::set_target_id_filters): <p>Specifies target identifiers (usually, specific account IDs) to limit the output by. If you use this parameter, the count of returned noncompliant resources includes only resources with the specified target IDs.</p>
    ///   - [`region_filters(Vec<String>)`](crate::client::fluent_builders::GetComplianceSummary::region_filters) / [`set_region_filters(Option<Vec<String>>)`](crate::client::fluent_builders::GetComplianceSummary::set_region_filters): <p>Specifies a list of Amazon Web Services Regions to limit the output to. If you use this parameter, the count of returned noncompliant resources includes only resources in the specified Regions.</p>
    ///   - [`resource_type_filters(Vec<String>)`](crate::client::fluent_builders::GetComplianceSummary::resource_type_filters) / [`set_resource_type_filters(Option<Vec<String>>)`](crate::client::fluent_builders::GetComplianceSummary::set_resource_type_filters): <p>Specifies that you want the response to include information for only resources of the specified types. The format of each resource type is <code>service[:resourceType]</code>. For example, specifying a resource type of <code>ec2</code> returns all Amazon EC2 resources (which includes EC2 instances). Specifying a resource type of <code>ec2:instance</code> returns only EC2 instances.</p>  <p>The string for each service name and resource type is the same as that embedded in a resource's Amazon Resource Name (ARN). Consult the <i> <a href="https://docs.aws.amazon.com/general/latest/gr/">Amazon Web Services General Reference</a> </i> for the following:</p>  <ul>   <li> <p>For a list of service name strings, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">Amazon Web Services Service Namespaces</a>.</p> </li>   <li> <p>For resource type strings, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arns-syntax">Example ARNs</a>.</p> </li>   <li> <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p> </li>  </ul>  <p>You can specify multiple resource types by using a comma separated array. The array can include up to 100 items. Note that the length constraint requirement applies to each resource type filter. </p>
    ///   - [`tag_key_filters(Vec<String>)`](crate::client::fluent_builders::GetComplianceSummary::tag_key_filters) / [`set_tag_key_filters(Option<Vec<String>>)`](crate::client::fluent_builders::GetComplianceSummary::set_tag_key_filters): <p>Specifies that you want the response to include information for only resources that have tags with the specified tag keys. If you use this parameter, the count of returned noncompliant resources includes only resources that have the specified tag keys.</p>
    ///   - [`group_by(Vec<GroupByAttribute>)`](crate::client::fluent_builders::GetComplianceSummary::group_by) / [`set_group_by(Option<Vec<GroupByAttribute>>)`](crate::client::fluent_builders::GetComplianceSummary::set_group_by): <p>Specifies a list of attributes to group the counts of noncompliant resources by. If supplied, the counts are sorted by those attributes.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetComplianceSummary::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetComplianceSummary::set_max_results): <p>Specifies the maximum number of results to be returned in each page. A query can return fewer than this maximum, even if there are more results still to return. You should always check the <code>PaginationToken</code> response value to see if there are more results. You can specify a minimum of 1 and a maximum value of 100.</p>
    ///   - [`pagination_token(impl Into<String>)`](crate::client::fluent_builders::GetComplianceSummary::pagination_token) / [`set_pagination_token(Option<String>)`](crate::client::fluent_builders::GetComplianceSummary::set_pagination_token): <p>Specifies a <code>PaginationToken</code> response value from a previous request to indicate that you want the next page of results. Leave this parameter empty in your initial request.</p>
                        /// - On success, responds with [`GetComplianceSummaryOutput`](crate::output::GetComplianceSummaryOutput) with field(s):
                        ///   - [`summary_list(Option<Vec<Summary>>)`](crate::output::GetComplianceSummaryOutput::summary_list): <p>A table that shows counts of noncompliant resources.</p>
    ///   - [`pagination_token(Option<String>)`](crate::output::GetComplianceSummaryOutput::pagination_token): <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
                        /// - On failure, responds with [`SdkError<GetComplianceSummaryError>`](crate::error::GetComplianceSummaryError)
    pub fn get_compliance_summary(&self) -> crate::client::fluent_builders::GetComplianceSummary {
                            crate::client::fluent_builders::GetComplianceSummary::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`GetResources`](crate::client::fluent_builders::GetResources) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetResources::into_paginator).
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`pagination_token(impl Into<String>)`](crate::client::fluent_builders::GetResources::pagination_token) / [`set_pagination_token(Option<String>)`](crate::client::fluent_builders::GetResources::set_pagination_token): <p>Specifies a <code>PaginationToken</code> response value from a previous request to indicate that you want the next page of results. Leave this parameter empty in your initial request.</p>
    ///   - [`tag_filters(Vec<TagFilter>)`](crate::client::fluent_builders::GetResources::tag_filters) / [`set_tag_filters(Option<Vec<TagFilter>>)`](crate::client::fluent_builders::GetResources::set_tag_filters): <p>Specifies a list of TagFilters (keys and values) to restrict the output to only those resources that have tags with the specified keys and, if included, the specified values. Each <code>TagFilter</code> must contain a key with values optional. A request can include up to 50 keys, and each key can include up to 20 values. </p>  <p>Note the following when deciding how to use TagFilters:</p>  <ul>   <li> <p>If you <i>don't</i> specify a <code>TagFilter</code>, the response includes all resources that are currently tagged or ever had a tag. Resources that currently don't have tags are shown with an empty tag set, like this: <code>"Tags": []</code>.</p> </li>   <li> <p>If you specify more than one filter in a single request, the response returns only those resources that satisfy all filters.</p> </li>   <li> <p>If you specify a filter that contains more than one value for a key, the response returns resources that match <i>any</i> of the specified values for that key.</p> </li>   <li> <p>If you don't specify a value for a key, the response returns all resources that are tagged with that key, with any or no value.</p> <p>For example, for the following filters: <code>filter1= {keyA,{value1}}</code>, <code>filter2={keyB,{value2,value3,value4}}</code>, <code>filter3= {keyC}</code>:</p>    <ul>     <li> <p> <code>GetResources({filter1})</code> returns resources tagged with <code>key1=value1</code> </p> </li>     <li> <p> <code>GetResources({filter2})</code> returns resources tagged with <code>key2=value2</code> or <code>key2=value3</code> or <code>key2=value4</code> </p> </li>     <li> <p> <code>GetResources({filter3})</code> returns resources tagged with any tag with the key <code>key3</code>, and with any or no value</p> </li>     <li> <p> <code>GetResources({filter1,filter2,filter3})</code> returns resources tagged with <code>(key1=value1) and (key2=value2 or key2=value3 or key2=value4) and (key3, any or no value)</code> </p> </li>    </ul> </li>  </ul>
    ///   - [`resources_per_page(i32)`](crate::client::fluent_builders::GetResources::resources_per_page) / [`set_resources_per_page(Option<i32>)`](crate::client::fluent_builders::GetResources::set_resources_per_page): <p>Specifies the maximum number of results to be returned in each page. A query can return fewer than this maximum, even if there are more results still to return. You should always check the <code>PaginationToken</code> response value to see if there are more results. You can specify a minimum of 1 and a maximum value of 100.</p>
    ///   - [`tags_per_page(i32)`](crate::client::fluent_builders::GetResources::tags_per_page) / [`set_tags_per_page(Option<i32>)`](crate::client::fluent_builders::GetResources::set_tags_per_page): <p>Amazon Web Services recommends using <code>ResourcesPerPage</code> instead of this parameter.</p>  <p>A limit that restricts the number of tags (key and value pairs) returned by <code>GetResources</code> in paginated output. A resource with no tags is counted as having one tag (one key and value pair).</p>  <p> <code>GetResources</code> does not split a resource and its associated tags across pages. If the specified <code>TagsPerPage</code> would cause such a break, a <code>PaginationToken</code> is returned in place of the affected resource and its tags. Use that token in another request to get the remaining data. For example, if you specify a <code>TagsPerPage</code> of <code>100</code> and the account has 22 resources with 10 tags each (meaning that each resource has 10 key and value pairs), the output will consist of three pages. The first page displays the first 10 resources, each with its 10 tags. The second page displays the next 10 resources, each with its 10 tags. The third page displays the remaining 2 resources, each with its 10 tags.</p>  <p>You can set <code>TagsPerPage</code> to a minimum of 100 items up to a maximum of 500 items.</p>
    ///   - [`resource_type_filters(Vec<String>)`](crate::client::fluent_builders::GetResources::resource_type_filters) / [`set_resource_type_filters(Option<Vec<String>>)`](crate::client::fluent_builders::GetResources::set_resource_type_filters): <p>Specifies the resource types that you want included in the response. The format of each resource type is <code>service[:resourceType]</code>. For example, specifying a resource type of <code>ec2</code> returns all Amazon EC2 resources (which includes EC2 instances). Specifying a resource type of <code>ec2:instance</code> returns only EC2 instances. </p>  <p>The string for each service name and resource type is the same as that embedded in a resource's Amazon Resource Name (ARN). For the list of services whose resources you can use in this parameter, see <a href="https://docs.aws.amazon.com/resourcegroupstagging/latest/APIReference/supported-services.html">Services that support the Resource Groups Tagging API</a>.</p>  <p>You can specify multiple resource types by using an array. The array can include up to 100 items. Note that the length constraint requirement applies to each resource type filter. For example, the following string would limit the response to only Amazon EC2 instances, Amazon S3 buckets, or any Audit Manager resource:</p>  <p> <code>ec2:instance,s3:bucket,auditmanager</code> </p>
    ///   - [`include_compliance_details(bool)`](crate::client::fluent_builders::GetResources::include_compliance_details) / [`set_include_compliance_details(Option<bool>)`](crate::client::fluent_builders::GetResources::set_include_compliance_details): <p>Specifies whether to include details regarding the compliance with the effective tag policy. Set this to <code>true</code> to determine whether resources are compliant with the tag policy and to get details.</p>
    ///   - [`exclude_compliant_resources(bool)`](crate::client::fluent_builders::GetResources::exclude_compliant_resources) / [`set_exclude_compliant_resources(Option<bool>)`](crate::client::fluent_builders::GetResources::set_exclude_compliant_resources): <p>Specifies whether to exclude resources that are compliant with the tag policy. Set this to <code>true</code> if you are interested in retrieving information on noncompliant resources only.</p>  <p>You can use this parameter only if the <code>IncludeComplianceDetails</code> parameter is also set to <code>true</code>.</p>
    ///   - [`resource_arn_list(Vec<String>)`](crate::client::fluent_builders::GetResources::resource_arn_list) / [`set_resource_arn_list(Option<Vec<String>>)`](crate::client::fluent_builders::GetResources::set_resource_arn_list): <p>Specifies a list of ARNs of resources for which you want to retrieve tag data. You can't specify both this parameter and any of the pagination parameters (<code>ResourcesPerPage</code>, <code>TagsPerPage</code>, <code>PaginationToken</code>) in the same request. If you specify both, you get an <code>Invalid Parameter</code> exception.</p>  <p>If a resource specified by this parameter doesn't exist, it doesn't generate an error; it simply isn't included in the response.</p>  <p>An ARN (Amazon Resource Name) uniquely identifies a resource. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
                        /// - On success, responds with [`GetResourcesOutput`](crate::output::GetResourcesOutput) with field(s):
                        ///   - [`pagination_token(Option<String>)`](crate::output::GetResourcesOutput::pagination_token): <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
    ///   - [`resource_tag_mapping_list(Option<Vec<ResourceTagMapping>>)`](crate::output::GetResourcesOutput::resource_tag_mapping_list): <p>A list of resource ARNs and the tags (keys and values) associated with each.</p>
                        /// - On failure, responds with [`SdkError<GetResourcesError>`](crate::error::GetResourcesError)
    pub fn get_resources(&self) -> crate::client::fluent_builders::GetResources {
                            crate::client::fluent_builders::GetResources::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`GetTagKeys`](crate::client::fluent_builders::GetTagKeys) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetTagKeys::into_paginator).
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`pagination_token(impl Into<String>)`](crate::client::fluent_builders::GetTagKeys::pagination_token) / [`set_pagination_token(Option<String>)`](crate::client::fluent_builders::GetTagKeys::set_pagination_token): <p>Specifies a <code>PaginationToken</code> response value from a previous request to indicate that you want the next page of results. Leave this parameter empty in your initial request.</p>
                        /// - On success, responds with [`GetTagKeysOutput`](crate::output::GetTagKeysOutput) with field(s):
                        ///   - [`pagination_token(Option<String>)`](crate::output::GetTagKeysOutput::pagination_token): <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
    ///   - [`tag_keys(Option<Vec<String>>)`](crate::output::GetTagKeysOutput::tag_keys): <p>A list of all tag keys in the Amazon Web Services account.</p>
                        /// - On failure, responds with [`SdkError<GetTagKeysError>`](crate::error::GetTagKeysError)
    pub fn get_tag_keys(&self) -> crate::client::fluent_builders::GetTagKeys {
                            crate::client::fluent_builders::GetTagKeys::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`GetTagValues`](crate::client::fluent_builders::GetTagValues) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetTagValues::into_paginator).
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`pagination_token(impl Into<String>)`](crate::client::fluent_builders::GetTagValues::pagination_token) / [`set_pagination_token(Option<String>)`](crate::client::fluent_builders::GetTagValues::set_pagination_token): <p>Specifies a <code>PaginationToken</code> response value from a previous request to indicate that you want the next page of results. Leave this parameter empty in your initial request.</p>
    ///   - [`key(impl Into<String>)`](crate::client::fluent_builders::GetTagValues::key) / [`set_key(Option<String>)`](crate::client::fluent_builders::GetTagValues::set_key): <p>Specifies the tag key for which you want to list all existing values that are currently used in the specified Amazon Web Services Region for the calling account.</p>
                        /// - On success, responds with [`GetTagValuesOutput`](crate::output::GetTagValuesOutput) with field(s):
                        ///   - [`pagination_token(Option<String>)`](crate::output::GetTagValuesOutput::pagination_token): <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
    ///   - [`tag_values(Option<Vec<String>>)`](crate::output::GetTagValuesOutput::tag_values): <p>A list of all tag values for the specified key currently used in the specified Amazon Web Services Region for the calling account.</p>
                        /// - On failure, responds with [`SdkError<GetTagValuesError>`](crate::error::GetTagValuesError)
    pub fn get_tag_values(&self) -> crate::client::fluent_builders::GetTagValues {
                            crate::client::fluent_builders::GetTagValues::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`StartReportCreation`](crate::client::fluent_builders::StartReportCreation) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`s3_bucket(impl Into<String>)`](crate::client::fluent_builders::StartReportCreation::s3_bucket) / [`set_s3_bucket(Option<String>)`](crate::client::fluent_builders::StartReportCreation::set_s3_bucket): <p>The name of the Amazon S3 bucket where the report will be stored; for example:</p>  <p> <code>awsexamplebucket</code> </p>  <p>For more information on S3 bucket requirements, including an example bucket policy, see the example S3 bucket policy on this page.</p>
                        /// - On success, responds with [`StartReportCreationOutput`](crate::output::StartReportCreationOutput)
                        
                        /// - On failure, responds with [`SdkError<StartReportCreationError>`](crate::error::StartReportCreationError)
    pub fn start_report_creation(&self) -> crate::client::fluent_builders::StartReportCreation {
                            crate::client::fluent_builders::StartReportCreation::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`TagResources`](crate::client::fluent_builders::TagResources) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`resource_arn_list(Vec<String>)`](crate::client::fluent_builders::TagResources::resource_arn_list) / [`set_resource_arn_list(Option<Vec<String>>)`](crate::client::fluent_builders::TagResources::set_resource_arn_list): <p>Specifies the list of ARNs of the resources that you want to apply tags to.</p>  <p>An ARN (Amazon Resource Name) uniquely identifies a resource. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::TagResources::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::TagResources::set_tags): <p>Specifies a list of tags that you want to add to the specified resources. A tag consists of a key and a value that you define.</p>
                        /// - On success, responds with [`TagResourcesOutput`](crate::output::TagResourcesOutput) with field(s):
                        ///   - [`failed_resources_map(Option<HashMap<String, FailureInfo>>)`](crate::output::TagResourcesOutput::failed_resources_map): <p>A map containing a key-value pair for each failed item that couldn't be tagged. The key is the ARN of the failed resource. The value is a <code>FailureInfo</code> object that contains an error code, a status code, and an error message. If there are no errors, the <code>FailedResourcesMap</code> is empty.</p>
                        /// - On failure, responds with [`SdkError<TagResourcesError>`](crate::error::TagResourcesError)
    pub fn tag_resources(&self) -> crate::client::fluent_builders::TagResources {
                            crate::client::fluent_builders::TagResources::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`UntagResources`](crate::client::fluent_builders::UntagResources) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`resource_arn_list(Vec<String>)`](crate::client::fluent_builders::UntagResources::resource_arn_list) / [`set_resource_arn_list(Option<Vec<String>>)`](crate::client::fluent_builders::UntagResources::set_resource_arn_list): <p>Specifies a list of ARNs of the resources that you want to remove tags from.</p>  <p>An ARN (Amazon Resource Name) uniquely identifies a resource. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    ///   - [`tag_keys(Vec<String>)`](crate::client::fluent_builders::UntagResources::tag_keys) / [`set_tag_keys(Option<Vec<String>>)`](crate::client::fluent_builders::UntagResources::set_tag_keys): <p>Specifies a list of tag keys that you want to remove from the specified resources.</p>
                        /// - On success, responds with [`UntagResourcesOutput`](crate::output::UntagResourcesOutput) with field(s):
                        ///   - [`failed_resources_map(Option<HashMap<String, FailureInfo>>)`](crate::output::UntagResourcesOutput::failed_resources_map): <p>A map containing a key-value pair for each failed item that couldn't be untagged. The key is the ARN of the failed resource. The value is a <code>FailureInfo</code> object that contains an error code, a status code, and an error message. If there are no errors, the <code>FailedResourcesMap</code> is empty.</p>
                        /// - On failure, responds with [`SdkError<UntagResourcesError>`](crate::error::UntagResourcesError)
    pub fn untag_resources(&self) -> crate::client::fluent_builders::UntagResources {
                            crate::client::fluent_builders::UntagResources::new(self.handle.clone())
                        }
}

impl Client {
    /// Creates a new client from an [SDK Config](aws_types::sdk_config::SdkConfig).
                    ///
                    /// # Panics
                    ///
                    /// - This method will panic if the `sdk_config` is missing an async sleep implementation. If you experience this panic, set
                    ///     the `sleep_impl` on the Config passed into this function to fix it.
                    /// - This method will panic if the `sdk_config` is missing an HTTP connector. If you experience this panic, set the
                    ///     `http_connector` on the Config passed into this function to fix it.
                    pub fn new(sdk_config: &aws_types::sdk_config::SdkConfig) -> Self {
                        Self::from_conf(sdk_config.into())
                    }
    
                    /// Creates a new client from the service [`Config`](crate::Config).
                    ///
                    /// # Panics
                    ///
                    /// - This method will panic if the `conf` is missing an async sleep implementation. If you experience this panic, set
                    ///     the `sleep_impl` on the Config passed into this function to fix it.
                    /// - This method will panic if the `conf` is missing an HTTP connector. If you experience this panic, set the
                    ///     `http_connector` on the Config passed into this function to fix it.
                    pub fn from_conf(conf: crate::Config) -> Self {
                        let retry_config = conf.retry_config().cloned().unwrap_or_else(aws_smithy_types::retry::RetryConfig::disabled);
                        let timeout_config = conf.timeout_config().cloned().unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
                        let sleep_impl = conf.sleep_impl();
                        if (retry_config.has_retry() || timeout_config.has_timeouts()) && sleep_impl.is_none() {
                            panic!("An async sleep implementation is required for retries or timeouts to work. \
                                    Set the `sleep_impl` on the Config passed into this function to fix this panic.");
                        }
    
                        let connector = conf.http_connector().and_then(|c| {
                            let timeout_config = conf
                                .timeout_config()
                                .cloned()
                                .unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
                            let connector_settings = aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(
                                &timeout_config,
                            );
                            c.connector(&connector_settings, conf.sleep_impl())
                        });
    
                        let builder = aws_smithy_client::Builder::new();
    
                        let builder = match connector {
                            // Use provided connector
                            Some(c) => builder.connector(c),
                            None =>{
                                #[cfg(any(feature = "rustls", feature = "native-tls"))]
                                {
                                    // Use default connector based on enabled features
                                    builder.dyn_https_connector(aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(&timeout_config))
                                }
                                #[cfg(not(any(feature = "rustls", feature = "native-tls")))]
                                {
                                    panic!("No HTTP connector was available. Enable the `rustls` or `native-tls` crate feature or set a connector to fix this.");
                                }
                            }
                        };
                        let mut builder = builder
                            .middleware(aws_smithy_client::erase::DynMiddleware::new(crate::middleware::DefaultMiddleware::new()))
                            .retry_config(retry_config.into())
                            .operation_timeout_config(timeout_config.into());
                        builder.set_sleep_impl(sleep_impl);
                        let client = builder.build();
    
                        Self { handle: std::sync::Arc::new(Handle { client, conf }) }
                    }
}

/// Utilities to ergonomically construct a request to the service.
/// 
/// Fluent builders are created through the [`Client`](crate::client::Client) by calling
/// one if its operation methods. After parameters are set using the builder methods,
/// the `send` method can be called to initiate the request.
pub mod fluent_builders;

