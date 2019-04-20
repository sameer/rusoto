// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p><p>Describes an API key.</p> <p>Customers invoke AWS AppSync GraphQL API operations with API keys as an identity mechanism. There are two key versions:</p> <p> <b>da1</b>: This version was introduced at launch in November 2017. These keys always expire after 7 days. Key expiration is managed by Amazon DynamoDB TTL. The keys ceased to be valid after February 21, 2018 and should not be used after that date.</p> <ul> <li> <p> <code>ListApiKeys</code> returns the expiration time in milliseconds.</p> </li> <li> <p> <code>CreateApiKey</code> returns the expiration time in milliseconds.</p> </li> <li> <p> <code>UpdateApiKey</code> is not available for this key version.</p> </li> <li> <p> <code>DeleteApiKey</code> deletes the item from the table.</p> </li> <li> <p>Expiration is stored in Amazon DynamoDB as milliseconds. This results in a bug where keys are not automatically deleted because DynamoDB expects the TTL to be stored in seconds. As a one-time action, we will delete these keys from the table after February 21, 2018.</p> </li> </ul> <p> <b>da2</b>: This version was introduced in February 2018 when AppSync added support to extend key expiration.</p> <ul> <li> <p> <code>ListApiKeys</code> returns the expiration time in seconds.</p> </li> <li> <p> <code>CreateApiKey</code> returns the expiration time in seconds and accepts a user-provided expiration time in seconds.</p> </li> <li> <p> <code>UpdateApiKey</code> returns the expiration time in seconds and accepts a user-provided expiration time in seconds. Key expiration can only be updated while the key has not expired.</p> </li> <li> <p> <code>DeleteApiKey</code> deletes the item from the table.</p> </li> <li> <p>Expiration is stored in Amazon DynamoDB as seconds.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ApiKey {
    /// <p>A description of the purpose of the API key.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The time after which the API key expires. The date is represented as seconds since the epoch, rounded down to the nearest hour.</p>
    #[serde(rename = "expires")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<i64>,
    /// <p>The API key ID.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>The authorization config in case the HTTP endpoint requires authorization.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationConfig {
    /// <p><p>The authorization type required by the HTTP endpoint.</p> <ul> <li> <p> <b>AWS_IAM</b>: The authorization type is Sigv4.</p> </li> </ul></p>
    #[serde(rename = "authorizationType")]
    pub authorization_type: String,
    /// <p>The AWS IAM settings.</p>
    #[serde(rename = "awsIamConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iam_config: Option<AwsIamConfig>,
}

/// <p>The AWS IAM configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsIamConfig {
    /// <p>The signing region for AWS IAM authorization.</p>
    #[serde(rename = "signingRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_region: Option<String>,
    /// <p>The signing service name for AWS IAM authorization.</p>
    #[serde(rename = "signingServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_service_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateApiKeyRequest {
    /// <p>The ID for your GraphQL API.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>A description of the purpose of the API key.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The time from creation time after which the API key expires. The date is represented as seconds since the epoch, rounded down to the nearest hour. The default value for this parameter is 7 days from creation time. For more information, see .</p>
    #[serde(rename = "expires")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateApiKeyResponse {
    /// <p>The API key.</p>
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<ApiKey>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDataSourceRequest {
    /// <p>The API ID for the GraphQL API for the <code>DataSource</code>.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>A description of the <code>DataSource</code>.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Amazon DynamoDB settings.</p>
    #[serde(rename = "dynamodbConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamodb_config: Option<DynamodbDataSourceConfig>,
    /// <p>Amazon Elasticsearch Service settings.</p>
    #[serde(rename = "elasticsearchConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_config: Option<ElasticsearchDataSourceConfig>,
    /// <p>HTTP endpoint settings.</p>
    #[serde(rename = "httpConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_config: Option<HttpDataSourceConfig>,
    /// <p>AWS Lambda settings.</p>
    #[serde(rename = "lambdaConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_config: Option<LambdaDataSourceConfig>,
    /// <p>A user-supplied name for the <code>DataSource</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Relational database settings.</p>
    #[serde(rename = "relationalDatabaseConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_config: Option<RelationalDatabaseDataSourceConfig>,
    /// <p>The AWS IAM service role ARN for the data source. The system assumes this role when accessing the data source.</p>
    #[serde(rename = "serviceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>The type of the <code>DataSource</code>.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateDataSourceResponse {
    /// <p>The <code>DataSource</code> object.</p>
    #[serde(rename = "dataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateFunctionRequest {
    /// <p>The GraphQL API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The <code>Function</code> <code>DataSource</code> name.</p>
    #[serde(rename = "dataSourceName")]
    pub data_source_name: String,
    /// <p>The <code>Function</code> description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The <code>version</code> of the request mapping template. Currently the supported value is 2018-05-29. </p>
    #[serde(rename = "functionVersion")]
    pub function_version: String,
    /// <p>The <code>Function</code> name. The function name does not have to be unique.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The <code>Function</code> request mapping template. Functions support only the 2018-05-29 version of the request mapping template.</p>
    #[serde(rename = "requestMappingTemplate")]
    pub request_mapping_template: String,
    /// <p>The <code>Function</code> response mapping template. </p>
    #[serde(rename = "responseMappingTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mapping_template: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateFunctionResponse {
    /// <p>The <code>Function</code> object.</p>
    #[serde(rename = "functionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_configuration: Option<FunctionConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateGraphqlApiRequest {
    /// <p>The authentication type: API key, AWS IAM, or Amazon Cognito user pools.</p>
    #[serde(rename = "authenticationType")]
    pub authentication_type: String,
    /// <p>The Amazon CloudWatch Logs configuration.</p>
    #[serde(rename = "logConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<LogConfig>,
    /// <p>A user-supplied name for the <code>GraphqlApi</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The OpenID Connect configuration.</p>
    #[serde(rename = "openIDConnectConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id_connect_config: Option<OpenIDConnectConfig>,
    /// <p>The Amazon Cognito user pool configuration.</p>
    #[serde(rename = "userPoolConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_config: Option<UserPoolConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateGraphqlApiResponse {
    /// <p>The <code>GraphqlApi</code>.</p>
    #[serde(rename = "graphqlApi")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_api: Option<GraphqlApi>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateResolverRequest {
    /// <p>The ID for the GraphQL API for which the resolver is being created.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The name of the data source for which the resolver is being created.</p>
    #[serde(rename = "dataSourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_name: Option<String>,
    /// <p>The name of the field to attach the resolver to.</p>
    #[serde(rename = "fieldName")]
    pub field_name: String,
    /// <p><p>The resolver type.</p> <ul> <li> <p> <b>UNIT</b>: A UNIT resolver type. A UNIT resolver is the default resolver type. A UNIT resolver enables you to execute a GraphQL query against a single data source.</p> </li> <li> <p> <b>PIPELINE</b>: A PIPELINE resolver type. A PIPELINE resolver enables you to execute a series of <code>Function</code> in a serial manner. You can use a pipeline resolver to execute a GraphQL query against multiple data sources.</p> </li> </ul></p>
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// <p>The <code>PipelineConfig</code>.</p>
    #[serde(rename = "pipelineConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_config: Option<PipelineConfig>,
    /// <p>The mapping template to be used for requests.</p> <p>A resolver uses a request mapping template to convert a GraphQL expression into a format that a data source can understand. Mapping templates are written in Apache Velocity Template Language (VTL).</p>
    #[serde(rename = "requestMappingTemplate")]
    pub request_mapping_template: String,
    /// <p>The mapping template to be used for responses from the data source.</p>
    #[serde(rename = "responseMappingTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mapping_template: Option<String>,
    /// <p>The name of the <code>Type</code>.</p>
    #[serde(rename = "typeName")]
    pub type_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateResolverResponse {
    /// <p>The <code>Resolver</code> object.</p>
    #[serde(rename = "resolver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver: Option<Resolver>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateTypeRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The type definition, in GraphQL Schema Definition Language (SDL) format.</p> <p>For more information, see the <a href="http://graphql.org/learn/schema/">GraphQL SDL documentation</a>.</p>
    #[serde(rename = "definition")]
    pub definition: String,
    /// <p>The type format: SDL or JSON.</p>
    #[serde(rename = "format")]
    pub format: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateTypeResponse {
    /// <p>The <code>Type</code> object.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
}

/// <p>Describes a data source.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DataSource {
    /// <p>The data source ARN.</p>
    #[serde(rename = "dataSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_arn: Option<String>,
    /// <p>The description of the data source.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Amazon DynamoDB settings.</p>
    #[serde(rename = "dynamodbConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamodb_config: Option<DynamodbDataSourceConfig>,
    /// <p>Amazon Elasticsearch Service settings.</p>
    #[serde(rename = "elasticsearchConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_config: Option<ElasticsearchDataSourceConfig>,
    /// <p>HTTP endpoint settings.</p>
    #[serde(rename = "httpConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_config: Option<HttpDataSourceConfig>,
    /// <p>AWS Lambda settings.</p>
    #[serde(rename = "lambdaConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_config: Option<LambdaDataSourceConfig>,
    /// <p>The name of the data source.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Relational database settings.</p>
    #[serde(rename = "relationalDatabaseConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_config: Option<RelationalDatabaseDataSourceConfig>,
    /// <p>The AWS IAM service role ARN for the data source. The system assumes this role when accessing the data source.</p>
    #[serde(rename = "serviceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p><p>The type of the data source.</p> <ul> <li> <p> <b>AMAZON<em>DYNAMODB</b>: The data source is an Amazon DynamoDB table.</p> </li> <li> <p> <b>AMAZON</em>ELASTICSEARCH</b>: The data source is an Amazon Elasticsearch Service domain.</p> </li> <li> <p> <b>AWS<em>LAMBDA</b>: The data source is an AWS Lambda function.</p> </li> <li> <p> <b>NONE</b>: There is no data source. This type is used when you wish to invoke a GraphQL operation without connecting to a data source, such as performing data transformation with resolvers or triggering a subscription to be invoked from a mutation.</p> </li> <li> <p> <b>HTTP</b>: The data source is an HTTP endpoint.</p> </li> <li> <p> <b>RELATIONAL</em>DATABASE</b>: The data source is a relational database.</p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApiKeyRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The ID for the API key.</p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteApiKeyResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDataSourceRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The name of the data source.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteDataSourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteFunctionRequest {
    /// <p>The GraphQL API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The <code>Function</code> ID.</p>
    #[serde(rename = "functionId")]
    pub function_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteFunctionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteGraphqlApiRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteGraphqlApiResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteResolverRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The resolver field name.</p>
    #[serde(rename = "fieldName")]
    pub field_name: String,
    /// <p>The name of the resolver type.</p>
    #[serde(rename = "typeName")]
    pub type_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteResolverResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteTypeRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The type name.</p>
    #[serde(rename = "typeName")]
    pub type_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteTypeResponse {}

/// <p>Describes an Amazon DynamoDB data source configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamodbDataSourceConfig {
    /// <p>The AWS Region.</p>
    #[serde(rename = "awsRegion")]
    pub aws_region: String,
    /// <p>The table name.</p>
    #[serde(rename = "tableName")]
    pub table_name: String,
    /// <p>Set to TRUE to use Amazon Cognito credentials with this data source.</p>
    #[serde(rename = "useCallerCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_caller_credentials: Option<bool>,
}

/// <p>Describes an Elasticsearch data source configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElasticsearchDataSourceConfig {
    /// <p>The AWS Region.</p>
    #[serde(rename = "awsRegion")]
    pub aws_region: String,
    /// <p>The endpoint.</p>
    #[serde(rename = "endpoint")]
    pub endpoint: String,
}

/// <p>A function is a reusable entity. Multiple functions can be used to compose the resolver logic.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FunctionConfiguration {
    /// <p>The name of the <code>DataSource</code>.</p>
    #[serde(rename = "dataSourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_name: Option<String>,
    /// <p>The <code>Function</code> description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the <code>Function</code> object.</p>
    #[serde(rename = "functionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    /// <p>A unique ID representing the <code>Function</code> object.</p>
    #[serde(rename = "functionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_id: Option<String>,
    /// <p>The version of the request mapping template. Currently only the 2018-05-29 version of the template is supported.</p>
    #[serde(rename = "functionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
    /// <p>The name of the <code>Function</code> object.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The <code>Function</code> request mapping template. Functions support only the 2018-05-29 version of the request mapping template.</p>
    #[serde(rename = "requestMappingTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_mapping_template: Option<String>,
    /// <p>The <code>Function</code> response mapping template.</p>
    #[serde(rename = "responseMappingTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mapping_template: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDataSourceRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The name of the data source.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDataSourceResponse {
    /// <p>The <code>DataSource</code> object.</p>
    #[serde(rename = "dataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFunctionRequest {
    /// <p>The GraphQL API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The <code>Function</code> ID.</p>
    #[serde(rename = "functionId")]
    pub function_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetFunctionResponse {
    /// <p>The <code>Function</code> object.</p>
    #[serde(rename = "functionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_configuration: Option<FunctionConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGraphqlApiRequest {
    /// <p>The API ID for the GraphQL API.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetGraphqlApiResponse {
    /// <p>The <code>GraphqlApi</code> object.</p>
    #[serde(rename = "graphqlApi")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_api: Option<GraphqlApi>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIntrospectionSchemaRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The schema format: SDL or JSON.</p>
    #[serde(rename = "format")]
    pub format: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetIntrospectionSchemaResponse {
    /// <p>The schema, in GraphQL Schema Definition Language (SDL) format.</p> <p>For more information, see the <a href="http://graphql.org/learn/schema/">GraphQL SDL documentation</a>.</p>
    pub schema: Option<bytes::Bytes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetResolverRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The resolver field name.</p>
    #[serde(rename = "fieldName")]
    pub field_name: String,
    /// <p>The resolver type name.</p>
    #[serde(rename = "typeName")]
    pub type_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetResolverResponse {
    /// <p>The <code>Resolver</code> object.</p>
    #[serde(rename = "resolver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver: Option<Resolver>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSchemaCreationStatusRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSchemaCreationStatusResponse {
    /// <p>Detailed information about the status of the schema creation operation.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// <p>The current state of the schema (PROCESSING, ACTIVE, or DELETING). Once the schema is in the ACTIVE state, you can add data.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTypeRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The type format: SDL or JSON.</p>
    #[serde(rename = "format")]
    pub format: String,
    /// <p>The type name.</p>
    #[serde(rename = "typeName")]
    pub type_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetTypeResponse {
    /// <p>The <code>Type</code> object.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
}

/// <p>Describes a GraphQL API.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GraphqlApi {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    /// <p>The ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The authentication type.</p>
    #[serde(rename = "authenticationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    /// <p>The Amazon CloudWatch Logs configuration.</p>
    #[serde(rename = "logConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<LogConfig>,
    /// <p>The API name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The OpenID Connect configuration.</p>
    #[serde(rename = "openIDConnectConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id_connect_config: Option<OpenIDConnectConfig>,
    /// <p>The URIs.</p>
    #[serde(rename = "uris")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uris: Option<::std::collections::HashMap<String, String>>,
    /// <p>The Amazon Cognito user pool configuration.</p>
    #[serde(rename = "userPoolConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_config: Option<UserPoolConfig>,
}

/// <p>Describes an HTTP data source configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HttpDataSourceConfig {
    /// <p>The authorization config in case the HTTP endpoint requires authorization.</p>
    #[serde(rename = "authorizationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_config: Option<AuthorizationConfig>,
    /// <p>The HTTP URL endpoint. You can either specify the domain name or IP, and port combination, and the URL scheme must be HTTP or HTTPS. If the port is not specified, AWS AppSync uses the default port 80 for the HTTP endpoint and port 443 for HTTPS endpoints.</p>
    #[serde(rename = "endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}

/// <p>Describes an AWS Lambda data source configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LambdaDataSourceConfig {
    /// <p>The ARN for the Lambda function.</p>
    #[serde(rename = "lambdaFunctionArn")]
    pub lambda_function_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListApiKeysRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The maximum number of results you want the request to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListApiKeysResponse {
    /// <p>The <code>ApiKey</code> objects.</p>
    #[serde(rename = "apiKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_keys: Option<Vec<ApiKey>>,
    /// <p>An identifier to be passed in the next request to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDataSourcesRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The maximum number of results you want the request to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListDataSourcesResponse {
    /// <p>The <code>DataSource</code> objects.</p>
    #[serde(rename = "dataSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<DataSource>>,
    /// <p>An identifier to be passed in the next request to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFunctionsRequest {
    /// <p>The GraphQL API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The maximum number of results you want the request to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListFunctionsResponse {
    /// <p>A list of <code>Function</code> objects.</p>
    #[serde(rename = "functions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<FunctionConfiguration>>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListGraphqlApisRequest {
    /// <p>The maximum number of results you want the request to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListGraphqlApisResponse {
    /// <p>The <code>GraphqlApi</code> objects.</p>
    #[serde(rename = "graphqlApis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_apis: Option<Vec<GraphqlApi>>,
    /// <p>An identifier to be passed in the next request to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListResolversByFunctionRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The Function ID.</p>
    #[serde(rename = "functionId")]
    pub function_id: String,
    /// <p>The maximum number of results you want the request to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An identifier that was returned from the previous call to this operation, which you can use to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListResolversByFunctionResponse {
    /// <p>An identifier that can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of resolvers.</p>
    #[serde(rename = "resolvers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolvers: Option<Vec<Resolver>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListResolversRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The maximum number of results you want the request to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type name.</p>
    #[serde(rename = "typeName")]
    pub type_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListResolversResponse {
    /// <p>An identifier to be passed in the next request to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The <code>Resolver</code> objects.</p>
    #[serde(rename = "resolvers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolvers: Option<Vec<Resolver>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTypesRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The type format: SDL or JSON.</p>
    #[serde(rename = "format")]
    pub format: String,
    /// <p>The maximum number of results you want the request to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTypesResponse {
    /// <p>An identifier to be passed in the next request to this operation to return the next set of items in the list.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The <code>Type</code> objects.</p>
    #[serde(rename = "types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Type>>,
}

/// <p>The CloudWatch Logs configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogConfig {
    /// <p>The service role that AWS AppSync will assume to publish to Amazon CloudWatch logs in your account. </p>
    #[serde(rename = "cloudWatchLogsRoleArn")]
    pub cloud_watch_logs_role_arn: String,
    /// <p><p>The field logging level. Values can be NONE, ERROR, or ALL. </p> <ul> <li> <p> <b>NONE</b>: No field-level logs are captured.</p> </li> <li> <p> <b>ERROR</b>: Logs the following information only for the fields that are in error:</p> <ul> <li> <p>The error section in the server response.</p> </li> <li> <p>Field-level errors.</p> </li> <li> <p>The generated request/response functions that got resolved for error fields.</p> </li> </ul> </li> <li> <p> <b>ALL</b>: The following information is logged for all fields in the query:</p> <ul> <li> <p>Field-level tracing information.</p> </li> <li> <p>The generated request/response functions that got resolved for each field.</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "fieldLogLevel")]
    pub field_log_level: String,
}

/// <p>Describes an OpenID Connect configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenIDConnectConfig {
    /// <p>The number of milliseconds a token is valid after being authenticated.</p>
    #[serde(rename = "authTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_ttl: Option<i64>,
    /// <p>The client identifier of the Relying party at the OpenID identity provider. This identifier is typically obtained when the Relying party is registered with the OpenID identity provider. You can specify a regular expression so the AWS AppSync can validate against multiple client identifiers at a time.</p>
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>The number of milliseconds a token is valid after being issued to a user.</p>
    #[serde(rename = "iatTTL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat_ttl: Option<i64>,
    /// <p>The issuer for the OpenID Connect configuration. The issuer returned by discovery must exactly match the value of <code>iss</code> in the ID token.</p>
    #[serde(rename = "issuer")]
    pub issuer: String,
}

/// <p>The pipeline configuration for a resolver of kind <code>PIPELINE</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PipelineConfig {
    /// <p>A list of <code>Function</code> objects.</p>
    #[serde(rename = "functions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<String>>,
}

/// <p>The Amazon RDS HTTP endpoint configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RdsHttpEndpointConfig {
    /// <p>AWS Region for RDS HTTP endpoint.</p>
    #[serde(rename = "awsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    /// <p>AWS secret store ARN for database credentials.</p>
    #[serde(rename = "awsSecretStoreArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_secret_store_arn: Option<String>,
    /// <p>Logical database name.</p>
    #[serde(rename = "databaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>Amazon RDS cluster identifier.</p>
    #[serde(rename = "dbClusterIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_identifier: Option<String>,
    /// <p>Logical schema name.</p>
    #[serde(rename = "schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

/// <p>Describes a relational database data source configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationalDatabaseDataSourceConfig {
    /// <p>Amazon RDS HTTP endpoint settings.</p>
    #[serde(rename = "rdsHttpEndpointConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_http_endpoint_config: Option<RdsHttpEndpointConfig>,
    /// <p><p>Source type for the relational database.</p> <ul> <li> <p> <b>RDS<em>HTTP</em>ENDPOINT</b>: The relational database source type is an Amazon RDS HTTP endpoint.</p> </li> </ul></p>
    #[serde(rename = "relationalDatabaseSourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_source_type: Option<String>,
}

/// <p>Describes a resolver.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Resolver {
    /// <p>The resolver data source name.</p>
    #[serde(rename = "dataSourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_name: Option<String>,
    /// <p>The resolver field name.</p>
    #[serde(rename = "fieldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// <p><p>The resolver type.</p> <ul> <li> <p> <b>UNIT</b>: A UNIT resolver type. A UNIT resolver is the default resolver type. A UNIT resolver enables you to execute a GraphQL query against a single data source.</p> </li> <li> <p> <b>PIPELINE</b>: A PIPELINE resolver type. A PIPELINE resolver enables you to execute a series of <code>Function</code> in a serial manner. You can use a pipeline resolver to execute a GraphQL query against multiple data sources.</p> </li> </ul></p>
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// <p>The <code>PipelineConfig</code>.</p>
    #[serde(rename = "pipelineConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_config: Option<PipelineConfig>,
    /// <p>The request mapping template.</p>
    #[serde(rename = "requestMappingTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_mapping_template: Option<String>,
    /// <p>The resolver ARN.</p>
    #[serde(rename = "resolverArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_arn: Option<String>,
    /// <p>The response mapping template.</p>
    #[serde(rename = "responseMappingTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mapping_template: Option<String>,
    /// <p>The resolver type name.</p>
    #[serde(rename = "typeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartSchemaCreationRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The schema definition, in GraphQL schema language format.</p>
    #[serde(rename = "definition")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub definition: bytes::Bytes,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartSchemaCreationResponse {
    /// <p>The current state of the schema (PROCESSING, ACTIVE, or DELETING). When the schema is in the ACTIVE state, you can add data.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Describes a type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Type {
    /// <p>The type ARN.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The type definition.</p>
    #[serde(rename = "definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    /// <p>The type description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The type format: SDL or JSON.</p>
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>The type name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateApiKeyRequest {
    /// <p>The ID for the GraphQL API.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>A description of the purpose of the API key.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The time from update time after which the API key expires. The date is represented as seconds since the epoch. For more information, see .</p>
    #[serde(rename = "expires")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<i64>,
    /// <p>The API key ID.</p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateApiKeyResponse {
    /// <p>The API key.</p>
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<ApiKey>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDataSourceRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The new description for the data source.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The new Amazon DynamoDB configuration.</p>
    #[serde(rename = "dynamodbConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamodb_config: Option<DynamodbDataSourceConfig>,
    /// <p>The new Elasticsearch Service configuration.</p>
    #[serde(rename = "elasticsearchConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_config: Option<ElasticsearchDataSourceConfig>,
    /// <p>The new HTTP endpoint configuration.</p>
    #[serde(rename = "httpConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_config: Option<HttpDataSourceConfig>,
    /// <p>The new AWS Lambda configuration.</p>
    #[serde(rename = "lambdaConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_config: Option<LambdaDataSourceConfig>,
    /// <p>The new name for the data source.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The new relational database configuration.</p>
    #[serde(rename = "relationalDatabaseConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_config: Option<RelationalDatabaseDataSourceConfig>,
    /// <p>The new service role ARN for the data source.</p>
    #[serde(rename = "serviceRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    /// <p>The new data source type.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateDataSourceResponse {
    /// <p>The updated <code>DataSource</code> object.</p>
    #[serde(rename = "dataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateFunctionRequest {
    /// <p>The GraphQL API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The <code>Function</code> <code>DataSource</code> name.</p>
    #[serde(rename = "dataSourceName")]
    pub data_source_name: String,
    /// <p>The <code>Function</code> description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The function ID.</p>
    #[serde(rename = "functionId")]
    pub function_id: String,
    /// <p>The <code>version</code> of the request mapping template. Currently the supported value is 2018-05-29. </p>
    #[serde(rename = "functionVersion")]
    pub function_version: String,
    /// <p>The <code>Function</code> name.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The <code>Function</code> request mapping template. Functions support only the 2018-05-29 version of the request mapping template.</p>
    #[serde(rename = "requestMappingTemplate")]
    pub request_mapping_template: String,
    /// <p>The <code>Function</code> request mapping template. </p>
    #[serde(rename = "responseMappingTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mapping_template: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateFunctionResponse {
    /// <p>The <code>Function</code> object.</p>
    #[serde(rename = "functionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_configuration: Option<FunctionConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGraphqlApiRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The new authentication type for the <code>GraphqlApi</code> object.</p>
    #[serde(rename = "authenticationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    /// <p>The Amazon CloudWatch Logs configuration for the <code>GraphqlApi</code> object.</p>
    #[serde(rename = "logConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<LogConfig>,
    /// <p>The new name for the <code>GraphqlApi</code> object.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The OpenID Connect configuration for the <code>GraphqlApi</code> object.</p>
    #[serde(rename = "openIDConnectConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id_connect_config: Option<OpenIDConnectConfig>,
    /// <p>The new Amazon Cognito user pool configuration for the <code>GraphqlApi</code> object.</p>
    #[serde(rename = "userPoolConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_config: Option<UserPoolConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateGraphqlApiResponse {
    /// <p>The updated <code>GraphqlApi</code> object.</p>
    #[serde(rename = "graphqlApi")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_api: Option<GraphqlApi>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateResolverRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The new data source name.</p>
    #[serde(rename = "dataSourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_name: Option<String>,
    /// <p>The new field name.</p>
    #[serde(rename = "fieldName")]
    pub field_name: String,
    /// <p><p>The resolver type.</p> <ul> <li> <p> <b>UNIT</b>: A UNIT resolver type. A UNIT resolver is the default resolver type. A UNIT resolver enables you to execute a GraphQL query against a single data source.</p> </li> <li> <p> <b>PIPELINE</b>: A PIPELINE resolver type. A PIPELINE resolver enables you to execute a series of <code>Function</code> in a serial manner. You can use a pipeline resolver to execute a GraphQL query against multiple data sources.</p> </li> </ul></p>
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// <p>The <code>PipelineConfig</code>.</p>
    #[serde(rename = "pipelineConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_config: Option<PipelineConfig>,
    /// <p>The new request mapping template.</p>
    #[serde(rename = "requestMappingTemplate")]
    pub request_mapping_template: String,
    /// <p>The new response mapping template.</p>
    #[serde(rename = "responseMappingTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mapping_template: Option<String>,
    /// <p>The new type name.</p>
    #[serde(rename = "typeName")]
    pub type_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateResolverResponse {
    /// <p>The updated <code>Resolver</code> object.</p>
    #[serde(rename = "resolver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver: Option<Resolver>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateTypeRequest {
    /// <p>The API ID.</p>
    #[serde(rename = "apiId")]
    pub api_id: String,
    /// <p>The new definition.</p>
    #[serde(rename = "definition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    /// <p>The new type format: SDL or JSON.</p>
    #[serde(rename = "format")]
    pub format: String,
    /// <p>The new type name.</p>
    #[serde(rename = "typeName")]
    pub type_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateTypeResponse {
    /// <p>The updated <code>Type</code> object.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
}

/// <p>Describes an Amazon Cognito user pool configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserPoolConfig {
    /// <p>A regular expression for validating the incoming Amazon Cognito user pool app client ID.</p>
    #[serde(rename = "appIdClientRegex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id_client_regex: Option<String>,
    /// <p>The AWS Region in which the user pool was created.</p>
    #[serde(rename = "awsRegion")]
    pub aws_region: String,
    /// <p>The action that you want your GraphQL API to take when a request that uses Amazon Cognito user pool authentication doesn't match the Amazon Cognito user pool configuration.</p>
    #[serde(rename = "defaultAction")]
    pub default_action: String,
    /// <p>The user pool ID.</p>
    #[serde(rename = "userPoolId")]
    pub user_pool_id: String,
}

/// Errors returned by CreateApiKey
#[derive(Debug, PartialEq)]
pub enum CreateApiKeyError {
    /// <p>The API key exceeded a limit. Try your request again.</p>
    ApiKeyLimitExceeded(String),
    /// <p>The API key expiration must be set to a value between 1 and 365 days from creation (for <code>CreateApiKey</code>) or from update (for <code>UpdateApiKey</code>).</p>
    ApiKeyValidityOutOfBounds(String),
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl CreateApiKeyError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateApiKeyError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ApiKeyLimitExceededException" => {
                    return RusotoError::Service(CreateApiKeyError::ApiKeyLimitExceeded(
                        String::from(error_message),
                    ))
                }
                "ApiKeyValidityOutOfBoundsException" => {
                    return RusotoError::Service(CreateApiKeyError::ApiKeyValidityOutOfBounds(
                        String::from(error_message),
                    ))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateApiKeyError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateApiKeyError::InternalFailure(String::from(
                        error_message,
                    )))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateApiKeyError::LimitExceeded(String::from(
                        error_message,
                    )))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateApiKeyError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateApiKeyError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateApiKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateApiKeyError {
    fn description(&self) -> &str {
        match *self {
            CreateApiKeyError::ApiKeyLimitExceeded(ref cause) => cause,
            CreateApiKeyError::ApiKeyValidityOutOfBounds(ref cause) => cause,
            CreateApiKeyError::BadRequest(ref cause) => cause,
            CreateApiKeyError::InternalFailure(ref cause) => cause,
            CreateApiKeyError::LimitExceeded(ref cause) => cause,
            CreateApiKeyError::NotFound(ref cause) => cause,
            CreateApiKeyError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDataSource
#[derive(Debug, PartialEq)]
pub enum CreateDataSourceError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl CreateDataSourceError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDataSourceError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(CreateDataSourceError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateDataSourceError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateDataSourceError::InternalFailure(
                        String::from(error_message),
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateDataSourceError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateDataSourceError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDataSourceError {
    fn description(&self) -> &str {
        match *self {
            CreateDataSourceError::BadRequest(ref cause) => cause,
            CreateDataSourceError::ConcurrentModification(ref cause) => cause,
            CreateDataSourceError::InternalFailure(ref cause) => cause,
            CreateDataSourceError::NotFound(ref cause) => cause,
            CreateDataSourceError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateFunction
#[derive(Debug, PartialEq)]
pub enum CreateFunctionError {
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl CreateFunctionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFunctionError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateFunctionError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateFunctionError::InternalFailure(
                        String::from(error_message),
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateFunctionError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateFunctionError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateFunctionError {
    fn description(&self) -> &str {
        match *self {
            CreateFunctionError::ConcurrentModification(ref cause) => cause,
            CreateFunctionError::InternalFailure(ref cause) => cause,
            CreateFunctionError::NotFound(ref cause) => cause,
            CreateFunctionError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateGraphqlApi
#[derive(Debug, PartialEq)]
pub enum CreateGraphqlApiError {
    /// <p>The GraphQL API exceeded a limit. Try your request again.</p>
    ApiLimitExceeded(String),
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl CreateGraphqlApiError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGraphqlApiError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ApiLimitExceededException" => {
                    return RusotoError::Service(CreateGraphqlApiError::ApiLimitExceeded(
                        String::from(error_message),
                    ))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateGraphqlApiError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateGraphqlApiError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateGraphqlApiError::InternalFailure(
                        String::from(error_message),
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateGraphqlApiError::LimitExceeded(
                        String::from(error_message),
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateGraphqlApiError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateGraphqlApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateGraphqlApiError {
    fn description(&self) -> &str {
        match *self {
            CreateGraphqlApiError::ApiLimitExceeded(ref cause) => cause,
            CreateGraphqlApiError::BadRequest(ref cause) => cause,
            CreateGraphqlApiError::ConcurrentModification(ref cause) => cause,
            CreateGraphqlApiError::InternalFailure(ref cause) => cause,
            CreateGraphqlApiError::LimitExceeded(ref cause) => cause,
            CreateGraphqlApiError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateResolver
#[derive(Debug, PartialEq)]
pub enum CreateResolverError {
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl CreateResolverError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateResolverError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateResolverError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateResolverError::InternalFailure(
                        String::from(error_message),
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateResolverError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateResolverError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateResolverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateResolverError {
    fn description(&self) -> &str {
        match *self {
            CreateResolverError::ConcurrentModification(ref cause) => cause,
            CreateResolverError::InternalFailure(ref cause) => cause,
            CreateResolverError::NotFound(ref cause) => cause,
            CreateResolverError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateType
#[derive(Debug, PartialEq)]
pub enum CreateTypeError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl CreateTypeError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTypeError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(CreateTypeError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateTypeError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(CreateTypeError::InternalFailure(String::from(
                        error_message,
                    )))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateTypeError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateTypeError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTypeError {
    fn description(&self) -> &str {
        match *self {
            CreateTypeError::BadRequest(ref cause) => cause,
            CreateTypeError::ConcurrentModification(ref cause) => cause,
            CreateTypeError::InternalFailure(ref cause) => cause,
            CreateTypeError::NotFound(ref cause) => cause,
            CreateTypeError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApiKey
#[derive(Debug, PartialEq)]
pub enum DeleteApiKeyError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl DeleteApiKeyError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteApiKeyError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteApiKeyError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteApiKeyError::InternalFailure(String::from(
                        error_message,
                    )))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteApiKeyError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteApiKeyError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteApiKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApiKeyError {
    fn description(&self) -> &str {
        match *self {
            DeleteApiKeyError::BadRequest(ref cause) => cause,
            DeleteApiKeyError::InternalFailure(ref cause) => cause,
            DeleteApiKeyError::NotFound(ref cause) => cause,
            DeleteApiKeyError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDataSource
#[derive(Debug, PartialEq)]
pub enum DeleteDataSourceError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl DeleteDataSourceError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDataSourceError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteDataSourceError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteDataSourceError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteDataSourceError::InternalFailure(
                        String::from(error_message),
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDataSourceError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteDataSourceError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDataSourceError {
    fn description(&self) -> &str {
        match *self {
            DeleteDataSourceError::BadRequest(ref cause) => cause,
            DeleteDataSourceError::ConcurrentModification(ref cause) => cause,
            DeleteDataSourceError::InternalFailure(ref cause) => cause,
            DeleteDataSourceError::NotFound(ref cause) => cause,
            DeleteDataSourceError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteFunction
#[derive(Debug, PartialEq)]
pub enum DeleteFunctionError {
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl DeleteFunctionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFunctionError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteFunctionError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteFunctionError::InternalFailure(
                        String::from(error_message),
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteFunctionError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteFunctionError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteFunctionError {
    fn description(&self) -> &str {
        match *self {
            DeleteFunctionError::ConcurrentModification(ref cause) => cause,
            DeleteFunctionError::InternalFailure(ref cause) => cause,
            DeleteFunctionError::NotFound(ref cause) => cause,
            DeleteFunctionError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteGraphqlApi
#[derive(Debug, PartialEq)]
pub enum DeleteGraphqlApiError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl DeleteGraphqlApiError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGraphqlApiError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteGraphqlApiError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteGraphqlApiError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteGraphqlApiError::InternalFailure(
                        String::from(error_message),
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteGraphqlApiError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteGraphqlApiError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteGraphqlApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteGraphqlApiError {
    fn description(&self) -> &str {
        match *self {
            DeleteGraphqlApiError::BadRequest(ref cause) => cause,
            DeleteGraphqlApiError::ConcurrentModification(ref cause) => cause,
            DeleteGraphqlApiError::InternalFailure(ref cause) => cause,
            DeleteGraphqlApiError::NotFound(ref cause) => cause,
            DeleteGraphqlApiError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteResolver
#[derive(Debug, PartialEq)]
pub enum DeleteResolverError {
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl DeleteResolverError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteResolverError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteResolverError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteResolverError::InternalFailure(
                        String::from(error_message),
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteResolverError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteResolverError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteResolverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteResolverError {
    fn description(&self) -> &str {
        match *self {
            DeleteResolverError::ConcurrentModification(ref cause) => cause,
            DeleteResolverError::InternalFailure(ref cause) => cause,
            DeleteResolverError::NotFound(ref cause) => cause,
            DeleteResolverError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteType
#[derive(Debug, PartialEq)]
pub enum DeleteTypeError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl DeleteTypeError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTypeError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteTypeError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteTypeError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteTypeError::InternalFailure(String::from(
                        error_message,
                    )))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteTypeError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteTypeError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTypeError {
    fn description(&self) -> &str {
        match *self {
            DeleteTypeError::BadRequest(ref cause) => cause,
            DeleteTypeError::ConcurrentModification(ref cause) => cause,
            DeleteTypeError::InternalFailure(ref cause) => cause,
            DeleteTypeError::NotFound(ref cause) => cause,
            DeleteTypeError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDataSource
#[derive(Debug, PartialEq)]
pub enum GetDataSourceError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl GetDataSourceError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDataSourceError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(GetDataSourceError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(GetDataSourceError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetDataSourceError::InternalFailure(String::from(
                        error_message,
                    )))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDataSourceError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetDataSourceError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDataSourceError {
    fn description(&self) -> &str {
        match *self {
            GetDataSourceError::BadRequest(ref cause) => cause,
            GetDataSourceError::ConcurrentModification(ref cause) => cause,
            GetDataSourceError::InternalFailure(ref cause) => cause,
            GetDataSourceError::NotFound(ref cause) => cause,
            GetDataSourceError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetFunction
#[derive(Debug, PartialEq)]
pub enum GetFunctionError {
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl GetFunctionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFunctionError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(GetFunctionError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetFunctionError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetFunctionError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFunctionError {
    fn description(&self) -> &str {
        match *self {
            GetFunctionError::ConcurrentModification(ref cause) => cause,
            GetFunctionError::NotFound(ref cause) => cause,
            GetFunctionError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetGraphqlApi
#[derive(Debug, PartialEq)]
pub enum GetGraphqlApiError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl GetGraphqlApiError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetGraphqlApiError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(GetGraphqlApiError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetGraphqlApiError::InternalFailure(String::from(
                        error_message,
                    )))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetGraphqlApiError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetGraphqlApiError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetGraphqlApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGraphqlApiError {
    fn description(&self) -> &str {
        match *self {
            GetGraphqlApiError::BadRequest(ref cause) => cause,
            GetGraphqlApiError::InternalFailure(ref cause) => cause,
            GetGraphqlApiError::NotFound(ref cause) => cause,
            GetGraphqlApiError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetIntrospectionSchema
#[derive(Debug, PartialEq)]
pub enum GetIntrospectionSchemaError {
    /// <p>The GraphQL schema is not valid.</p>
    GraphQLSchema(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl GetIntrospectionSchemaError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetIntrospectionSchemaError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "GraphQLSchemaException" => {
                    return RusotoError::Service(GetIntrospectionSchemaError::GraphQLSchema(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetIntrospectionSchemaError::InternalFailure(
                        String::from(error_message),
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetIntrospectionSchemaError::NotFound(
                        String::from(error_message),
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetIntrospectionSchemaError::Unauthorized(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetIntrospectionSchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIntrospectionSchemaError {
    fn description(&self) -> &str {
        match *self {
            GetIntrospectionSchemaError::GraphQLSchema(ref cause) => cause,
            GetIntrospectionSchemaError::InternalFailure(ref cause) => cause,
            GetIntrospectionSchemaError::NotFound(ref cause) => cause,
            GetIntrospectionSchemaError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetResolver
#[derive(Debug, PartialEq)]
pub enum GetResolverError {
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl GetResolverError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResolverError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(GetResolverError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetResolverError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetResolverError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetResolverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetResolverError {
    fn description(&self) -> &str {
        match *self {
            GetResolverError::ConcurrentModification(ref cause) => cause,
            GetResolverError::NotFound(ref cause) => cause,
            GetResolverError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSchemaCreationStatus
#[derive(Debug, PartialEq)]
pub enum GetSchemaCreationStatusError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl GetSchemaCreationStatusError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSchemaCreationStatusError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(GetSchemaCreationStatusError::BadRequest(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetSchemaCreationStatusError::InternalFailure(
                        String::from(error_message),
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetSchemaCreationStatusError::NotFound(
                        String::from(error_message),
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetSchemaCreationStatusError::Unauthorized(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSchemaCreationStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSchemaCreationStatusError {
    fn description(&self) -> &str {
        match *self {
            GetSchemaCreationStatusError::BadRequest(ref cause) => cause,
            GetSchemaCreationStatusError::InternalFailure(ref cause) => cause,
            GetSchemaCreationStatusError::NotFound(ref cause) => cause,
            GetSchemaCreationStatusError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetType
#[derive(Debug, PartialEq)]
pub enum GetTypeError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl GetTypeError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTypeError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(GetTypeError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(GetTypeError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(GetTypeError::InternalFailure(String::from(
                        error_message,
                    )))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetTypeError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetTypeError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTypeError {
    fn description(&self) -> &str {
        match *self {
            GetTypeError::BadRequest(ref cause) => cause,
            GetTypeError::ConcurrentModification(ref cause) => cause,
            GetTypeError::InternalFailure(ref cause) => cause,
            GetTypeError::NotFound(ref cause) => cause,
            GetTypeError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ListApiKeys
#[derive(Debug, PartialEq)]
pub enum ListApiKeysError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl ListApiKeysError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListApiKeysError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(ListApiKeysError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListApiKeysError::InternalFailure(String::from(
                        error_message,
                    )))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListApiKeysError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListApiKeysError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListApiKeysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListApiKeysError {
    fn description(&self) -> &str {
        match *self {
            ListApiKeysError::BadRequest(ref cause) => cause,
            ListApiKeysError::InternalFailure(ref cause) => cause,
            ListApiKeysError::NotFound(ref cause) => cause,
            ListApiKeysError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDataSources
#[derive(Debug, PartialEq)]
pub enum ListDataSourcesError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl ListDataSourcesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDataSourcesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(ListDataSourcesError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListDataSourcesError::InternalFailure(
                        String::from(error_message),
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListDataSourcesError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListDataSourcesError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListDataSourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDataSourcesError {
    fn description(&self) -> &str {
        match *self {
            ListDataSourcesError::BadRequest(ref cause) => cause,
            ListDataSourcesError::InternalFailure(ref cause) => cause,
            ListDataSourcesError::NotFound(ref cause) => cause,
            ListDataSourcesError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ListFunctions
#[derive(Debug, PartialEq)]
pub enum ListFunctionsError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl ListFunctionsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFunctionsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(ListFunctionsError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListFunctionsError::InternalFailure(String::from(
                        error_message,
                    )))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListFunctionsError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListFunctionsError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListFunctionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFunctionsError {
    fn description(&self) -> &str {
        match *self {
            ListFunctionsError::BadRequest(ref cause) => cause,
            ListFunctionsError::InternalFailure(ref cause) => cause,
            ListFunctionsError::NotFound(ref cause) => cause,
            ListFunctionsError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ListGraphqlApis
#[derive(Debug, PartialEq)]
pub enum ListGraphqlApisError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl ListGraphqlApisError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGraphqlApisError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(ListGraphqlApisError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListGraphqlApisError::InternalFailure(
                        String::from(error_message),
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListGraphqlApisError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListGraphqlApisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListGraphqlApisError {
    fn description(&self) -> &str {
        match *self {
            ListGraphqlApisError::BadRequest(ref cause) => cause,
            ListGraphqlApisError::InternalFailure(ref cause) => cause,
            ListGraphqlApisError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ListResolvers
#[derive(Debug, PartialEq)]
pub enum ListResolversError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl ListResolversError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResolversError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(ListResolversError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListResolversError::InternalFailure(String::from(
                        error_message,
                    )))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListResolversError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListResolversError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListResolversError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListResolversError {
    fn description(&self) -> &str {
        match *self {
            ListResolversError::BadRequest(ref cause) => cause,
            ListResolversError::InternalFailure(ref cause) => cause,
            ListResolversError::NotFound(ref cause) => cause,
            ListResolversError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ListResolversByFunction
#[derive(Debug, PartialEq)]
pub enum ListResolversByFunctionError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl ListResolversByFunctionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResolversByFunctionError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(ListResolversByFunctionError::BadRequest(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListResolversByFunctionError::InternalFailure(
                        String::from(error_message),
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListResolversByFunctionError::NotFound(
                        String::from(error_message),
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListResolversByFunctionError::Unauthorized(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListResolversByFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListResolversByFunctionError {
    fn description(&self) -> &str {
        match *self {
            ListResolversByFunctionError::BadRequest(ref cause) => cause,
            ListResolversByFunctionError::InternalFailure(ref cause) => cause,
            ListResolversByFunctionError::NotFound(ref cause) => cause,
            ListResolversByFunctionError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTypes
#[derive(Debug, PartialEq)]
pub enum ListTypesError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl ListTypesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTypesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(ListTypesError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(ListTypesError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(ListTypesError::InternalFailure(String::from(
                        error_message,
                    )))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListTypesError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ListTypesError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTypesError {
    fn description(&self) -> &str {
        match *self {
            ListTypesError::BadRequest(ref cause) => cause,
            ListTypesError::ConcurrentModification(ref cause) => cause,
            ListTypesError::InternalFailure(ref cause) => cause,
            ListTypesError::NotFound(ref cause) => cause,
            ListTypesError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by StartSchemaCreation
#[derive(Debug, PartialEq)]
pub enum StartSchemaCreationError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl StartSchemaCreationError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartSchemaCreationError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(StartSchemaCreationError::BadRequest(
                        String::from(error_message),
                    ))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(StartSchemaCreationError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(StartSchemaCreationError::InternalFailure(
                        String::from(error_message),
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StartSchemaCreationError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(StartSchemaCreationError::Unauthorized(
                        String::from(error_message),
                    ))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartSchemaCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartSchemaCreationError {
    fn description(&self) -> &str {
        match *self {
            StartSchemaCreationError::BadRequest(ref cause) => cause,
            StartSchemaCreationError::ConcurrentModification(ref cause) => cause,
            StartSchemaCreationError::InternalFailure(ref cause) => cause,
            StartSchemaCreationError::NotFound(ref cause) => cause,
            StartSchemaCreationError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApiKey
#[derive(Debug, PartialEq)]
pub enum UpdateApiKeyError {
    /// <p>The API key expiration must be set to a value between 1 and 365 days from creation (for <code>CreateApiKey</code>) or from update (for <code>UpdateApiKey</code>).</p>
    ApiKeyValidityOutOfBounds(String),
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The request exceeded a limit. Try your request again.</p>
    LimitExceeded(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl UpdateApiKeyError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateApiKeyError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ApiKeyValidityOutOfBoundsException" => {
                    return RusotoError::Service(UpdateApiKeyError::ApiKeyValidityOutOfBounds(
                        String::from(error_message),
                    ))
                }
                "BadRequestException" => {
                    return RusotoError::Service(UpdateApiKeyError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateApiKeyError::InternalFailure(String::from(
                        error_message,
                    )))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateApiKeyError::LimitExceeded(String::from(
                        error_message,
                    )))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateApiKeyError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateApiKeyError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateApiKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApiKeyError {
    fn description(&self) -> &str {
        match *self {
            UpdateApiKeyError::ApiKeyValidityOutOfBounds(ref cause) => cause,
            UpdateApiKeyError::BadRequest(ref cause) => cause,
            UpdateApiKeyError::InternalFailure(ref cause) => cause,
            UpdateApiKeyError::LimitExceeded(ref cause) => cause,
            UpdateApiKeyError::NotFound(ref cause) => cause,
            UpdateApiKeyError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDataSource
#[derive(Debug, PartialEq)]
pub enum UpdateDataSourceError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl UpdateDataSourceError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDataSourceError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateDataSourceError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateDataSourceError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateDataSourceError::InternalFailure(
                        String::from(error_message),
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateDataSourceError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateDataSourceError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDataSourceError {
    fn description(&self) -> &str {
        match *self {
            UpdateDataSourceError::BadRequest(ref cause) => cause,
            UpdateDataSourceError::ConcurrentModification(ref cause) => cause,
            UpdateDataSourceError::InternalFailure(ref cause) => cause,
            UpdateDataSourceError::NotFound(ref cause) => cause,
            UpdateDataSourceError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateFunction
#[derive(Debug, PartialEq)]
pub enum UpdateFunctionError {
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl UpdateFunctionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFunctionError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateFunctionError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateFunctionError::InternalFailure(
                        String::from(error_message),
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateFunctionError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateFunctionError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateFunctionError {
    fn description(&self) -> &str {
        match *self {
            UpdateFunctionError::ConcurrentModification(ref cause) => cause,
            UpdateFunctionError::InternalFailure(ref cause) => cause,
            UpdateFunctionError::NotFound(ref cause) => cause,
            UpdateFunctionError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateGraphqlApi
#[derive(Debug, PartialEq)]
pub enum UpdateGraphqlApiError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl UpdateGraphqlApiError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGraphqlApiError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateGraphqlApiError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateGraphqlApiError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateGraphqlApiError::InternalFailure(
                        String::from(error_message),
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateGraphqlApiError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateGraphqlApiError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateGraphqlApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGraphqlApiError {
    fn description(&self) -> &str {
        match *self {
            UpdateGraphqlApiError::BadRequest(ref cause) => cause,
            UpdateGraphqlApiError::ConcurrentModification(ref cause) => cause,
            UpdateGraphqlApiError::InternalFailure(ref cause) => cause,
            UpdateGraphqlApiError::NotFound(ref cause) => cause,
            UpdateGraphqlApiError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateResolver
#[derive(Debug, PartialEq)]
pub enum UpdateResolverError {
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl UpdateResolverError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateResolverError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateResolverError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateResolverError::InternalFailure(
                        String::from(error_message),
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateResolverError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateResolverError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateResolverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateResolverError {
    fn description(&self) -> &str {
        match *self {
            UpdateResolverError::ConcurrentModification(ref cause) => cause,
            UpdateResolverError::InternalFailure(ref cause) => cause,
            UpdateResolverError::NotFound(ref cause) => cause,
            UpdateResolverError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateType
#[derive(Debug, PartialEq)]
pub enum UpdateTypeError {
    /// <p>The request is not well formed. For example, a value is invalid or a required field is missing. Check the field values, and then try again. </p>
    BadRequest(String),
    /// <p>Another modification is in progress at this time and it must complete before you can make your change. </p>
    ConcurrentModification(String),
    /// <p>An internal AWS AppSync error occurred. Try your request again.</p>
    InternalFailure(String),
    /// <p>The resource specified in the request was not found. Check the resource, and then try again.</p>
    NotFound(String),
    /// <p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
}

impl UpdateTypeError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateTypeError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateTypeError::BadRequest(String::from(
                        error_message,
                    )))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateTypeError::ConcurrentModification(
                        String::from(error_message),
                    ))
                }
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateTypeError::InternalFailure(String::from(
                        error_message,
                    )))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateTypeError::NotFound(String::from(
                        error_message,
                    )))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateTypeError::Unauthorized(String::from(
                        error_message,
                    )))
                }
                "ValidationException" => return RusotoError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateTypeError {
    fn description(&self) -> &str {
        match *self {
            UpdateTypeError::BadRequest(ref cause) => cause,
            UpdateTypeError::ConcurrentModification(ref cause) => cause,
            UpdateTypeError::InternalFailure(ref cause) => cause,
            UpdateTypeError::NotFound(ref cause) => cause,
            UpdateTypeError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWSAppSync API. AWSAppSync clients implement this trait.
pub trait AppSync {
    /// <p>Creates a unique key that you can distribute to clients who are executing your API.</p>
    fn create_api_key(
        &self,
        input: CreateApiKeyRequest,
    ) -> RusotoFuture<CreateApiKeyResponse, CreateApiKeyError>;

    /// <p>Creates a <code>DataSource</code> object.</p>
    fn create_data_source(
        &self,
        input: CreateDataSourceRequest,
    ) -> RusotoFuture<CreateDataSourceResponse, CreateDataSourceError>;

    /// <p>Creates a <code>Function</code> object.</p> <p>A function is a reusable entity. Multiple functions can be used to compose the resolver logic.</p>
    fn create_function(
        &self,
        input: CreateFunctionRequest,
    ) -> RusotoFuture<CreateFunctionResponse, CreateFunctionError>;

    /// <p>Creates a <code>GraphqlApi</code> object.</p>
    fn create_graphql_api(
        &self,
        input: CreateGraphqlApiRequest,
    ) -> RusotoFuture<CreateGraphqlApiResponse, CreateGraphqlApiError>;

    /// <p>Creates a <code>Resolver</code> object.</p> <p>A resolver converts incoming requests into a format that a data source can understand and converts the data source's responses into GraphQL.</p>
    fn create_resolver(
        &self,
        input: CreateResolverRequest,
    ) -> RusotoFuture<CreateResolverResponse, CreateResolverError>;

    /// <p>Creates a <code>Type</code> object.</p>
    fn create_type(
        &self,
        input: CreateTypeRequest,
    ) -> RusotoFuture<CreateTypeResponse, CreateTypeError>;

    /// <p>Deletes an API key.</p>
    fn delete_api_key(
        &self,
        input: DeleteApiKeyRequest,
    ) -> RusotoFuture<DeleteApiKeyResponse, DeleteApiKeyError>;

    /// <p>Deletes a <code>DataSource</code> object.</p>
    fn delete_data_source(
        &self,
        input: DeleteDataSourceRequest,
    ) -> RusotoFuture<DeleteDataSourceResponse, DeleteDataSourceError>;

    /// <p>Deletes a <code>Function</code>.</p>
    fn delete_function(
        &self,
        input: DeleteFunctionRequest,
    ) -> RusotoFuture<DeleteFunctionResponse, DeleteFunctionError>;

    /// <p>Deletes a <code>GraphqlApi</code> object.</p>
    fn delete_graphql_api(
        &self,
        input: DeleteGraphqlApiRequest,
    ) -> RusotoFuture<DeleteGraphqlApiResponse, DeleteGraphqlApiError>;

    /// <p>Deletes a <code>Resolver</code> object.</p>
    fn delete_resolver(
        &self,
        input: DeleteResolverRequest,
    ) -> RusotoFuture<DeleteResolverResponse, DeleteResolverError>;

    /// <p>Deletes a <code>Type</code> object.</p>
    fn delete_type(
        &self,
        input: DeleteTypeRequest,
    ) -> RusotoFuture<DeleteTypeResponse, DeleteTypeError>;

    /// <p>Retrieves a <code>DataSource</code> object.</p>
    fn get_data_source(
        &self,
        input: GetDataSourceRequest,
    ) -> RusotoFuture<GetDataSourceResponse, GetDataSourceError>;

    /// <p>Get a <code>Function</code>.</p>
    fn get_function(
        &self,
        input: GetFunctionRequest,
    ) -> RusotoFuture<GetFunctionResponse, GetFunctionError>;

    /// <p>Retrieves a <code>GraphqlApi</code> object.</p>
    fn get_graphql_api(
        &self,
        input: GetGraphqlApiRequest,
    ) -> RusotoFuture<GetGraphqlApiResponse, GetGraphqlApiError>;

    /// <p>Retrieves the introspection schema for a GraphQL API.</p>
    fn get_introspection_schema(
        &self,
        input: GetIntrospectionSchemaRequest,
    ) -> RusotoFuture<GetIntrospectionSchemaResponse, GetIntrospectionSchemaError>;

    /// <p>Retrieves a <code>Resolver</code> object.</p>
    fn get_resolver(
        &self,
        input: GetResolverRequest,
    ) -> RusotoFuture<GetResolverResponse, GetResolverError>;

    /// <p>Retrieves the current status of a schema creation operation.</p>
    fn get_schema_creation_status(
        &self,
        input: GetSchemaCreationStatusRequest,
    ) -> RusotoFuture<GetSchemaCreationStatusResponse, GetSchemaCreationStatusError>;

    /// <p>Retrieves a <code>Type</code> object.</p>
    fn get_type(&self, input: GetTypeRequest) -> RusotoFuture<GetTypeResponse, GetTypeError>;

    /// <p><p>Lists the API keys for a given API.</p> <note> <p>API keys are deleted automatically sometime after they expire. However, they may still be included in the response until they have actually been deleted. You can safely call <code>DeleteApiKey</code> to manually delete a key before it&#39;s automatically deleted.</p> </note></p>
    fn list_api_keys(
        &self,
        input: ListApiKeysRequest,
    ) -> RusotoFuture<ListApiKeysResponse, ListApiKeysError>;

    /// <p>Lists the data sources for a given API.</p>
    fn list_data_sources(
        &self,
        input: ListDataSourcesRequest,
    ) -> RusotoFuture<ListDataSourcesResponse, ListDataSourcesError>;

    /// <p>List multiple functions.</p>
    fn list_functions(
        &self,
        input: ListFunctionsRequest,
    ) -> RusotoFuture<ListFunctionsResponse, ListFunctionsError>;

    /// <p>Lists your GraphQL APIs.</p>
    fn list_graphql_apis(
        &self,
        input: ListGraphqlApisRequest,
    ) -> RusotoFuture<ListGraphqlApisResponse, ListGraphqlApisError>;

    /// <p>Lists the resolvers for a given API and type.</p>
    fn list_resolvers(
        &self,
        input: ListResolversRequest,
    ) -> RusotoFuture<ListResolversResponse, ListResolversError>;

    /// <p>List the resolvers that are associated with a specific function.</p>
    fn list_resolvers_by_function(
        &self,
        input: ListResolversByFunctionRequest,
    ) -> RusotoFuture<ListResolversByFunctionResponse, ListResolversByFunctionError>;

    /// <p>Lists the types for a given API.</p>
    fn list_types(
        &self,
        input: ListTypesRequest,
    ) -> RusotoFuture<ListTypesResponse, ListTypesError>;

    /// <p>Adds a new schema to your GraphQL API.</p> <p>This operation is asynchronous. Use to determine when it has completed.</p>
    fn start_schema_creation(
        &self,
        input: StartSchemaCreationRequest,
    ) -> RusotoFuture<StartSchemaCreationResponse, StartSchemaCreationError>;

    /// <p>Updates an API key.</p>
    fn update_api_key(
        &self,
        input: UpdateApiKeyRequest,
    ) -> RusotoFuture<UpdateApiKeyResponse, UpdateApiKeyError>;

    /// <p>Updates a <code>DataSource</code> object.</p>
    fn update_data_source(
        &self,
        input: UpdateDataSourceRequest,
    ) -> RusotoFuture<UpdateDataSourceResponse, UpdateDataSourceError>;

    /// <p>Updates a <code>Function</code> object.</p>
    fn update_function(
        &self,
        input: UpdateFunctionRequest,
    ) -> RusotoFuture<UpdateFunctionResponse, UpdateFunctionError>;

    /// <p>Updates a <code>GraphqlApi</code> object.</p>
    fn update_graphql_api(
        &self,
        input: UpdateGraphqlApiRequest,
    ) -> RusotoFuture<UpdateGraphqlApiResponse, UpdateGraphqlApiError>;

    /// <p>Updates a <code>Resolver</code> object.</p>
    fn update_resolver(
        &self,
        input: UpdateResolverRequest,
    ) -> RusotoFuture<UpdateResolverResponse, UpdateResolverError>;

    /// <p>Updates a <code>Type</code> object.</p>
    fn update_type(
        &self,
        input: UpdateTypeRequest,
    ) -> RusotoFuture<UpdateTypeResponse, UpdateTypeError>;
}
/// A client for the AWSAppSync API.
#[derive(Clone)]
pub struct AppSyncClient {
    client: Client,
    region: region::Region,
}

impl AppSyncClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> AppSyncClient {
        AppSyncClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> AppSyncClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        AppSyncClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl AppSync for AppSyncClient {
    /// <p>Creates a unique key that you can distribute to clients who are executing your API.</p>
    fn create_api_key(
        &self,
        input: CreateApiKeyRequest,
    ) -> RusotoFuture<CreateApiKeyResponse, CreateApiKeyError> {
        let request_uri = format!("/v1/apis/{api_id}/apikeys", api_id = input.api_id);

        let mut request = SignedRequest::new("POST", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateApiKeyResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateApiKeyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a <code>DataSource</code> object.</p>
    fn create_data_source(
        &self,
        input: CreateDataSourceRequest,
    ) -> RusotoFuture<CreateDataSourceResponse, CreateDataSourceError> {
        let request_uri = format!("/v1/apis/{api_id}/datasources", api_id = input.api_id);

        let mut request = SignedRequest::new("POST", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateDataSourceResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDataSourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a <code>Function</code> object.</p> <p>A function is a reusable entity. Multiple functions can be used to compose the resolver logic.</p>
    fn create_function(
        &self,
        input: CreateFunctionRequest,
    ) -> RusotoFuture<CreateFunctionResponse, CreateFunctionError> {
        let request_uri = format!("/v1/apis/{api_id}/functions", api_id = input.api_id);

        let mut request = SignedRequest::new("POST", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateFunctionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateFunctionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a <code>GraphqlApi</code> object.</p>
    fn create_graphql_api(
        &self,
        input: CreateGraphqlApiRequest,
    ) -> RusotoFuture<CreateGraphqlApiResponse, CreateGraphqlApiError> {
        let request_uri = "/v1/apis";

        let mut request = SignedRequest::new("POST", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateGraphqlApiResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateGraphqlApiError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a <code>Resolver</code> object.</p> <p>A resolver converts incoming requests into a format that a data source can understand and converts the data source's responses into GraphQL.</p>
    fn create_resolver(
        &self,
        input: CreateResolverRequest,
    ) -> RusotoFuture<CreateResolverResponse, CreateResolverError> {
        let request_uri = format!(
            "/v1/apis/{api_id}/types/{type_name}/resolvers",
            api_id = input.api_id,
            type_name = input.type_name
        );

        let mut request = SignedRequest::new("POST", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateResolverResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateResolverError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a <code>Type</code> object.</p>
    fn create_type(
        &self,
        input: CreateTypeRequest,
    ) -> RusotoFuture<CreateTypeResponse, CreateTypeError> {
        let request_uri = format!("/v1/apis/{api_id}/types", api_id = input.api_id);

        let mut request = SignedRequest::new("POST", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateTypeResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateTypeError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an API key.</p>
    fn delete_api_key(
        &self,
        input: DeleteApiKeyRequest,
    ) -> RusotoFuture<DeleteApiKeyResponse, DeleteApiKeyError> {
        let request_uri = format!(
            "/v1/apis/{api_id}/apikeys/{id}",
            api_id = input.api_id,
            id = input.id
        );

        let mut request = SignedRequest::new("DELETE", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteApiKeyResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteApiKeyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a <code>DataSource</code> object.</p>
    fn delete_data_source(
        &self,
        input: DeleteDataSourceRequest,
    ) -> RusotoFuture<DeleteDataSourceResponse, DeleteDataSourceError> {
        let request_uri = format!(
            "/v1/apis/{api_id}/datasources/{name}",
            api_id = input.api_id,
            name = input.name
        );

        let mut request = SignedRequest::new("DELETE", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteDataSourceResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDataSourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a <code>Function</code>.</p>
    fn delete_function(
        &self,
        input: DeleteFunctionRequest,
    ) -> RusotoFuture<DeleteFunctionResponse, DeleteFunctionError> {
        let request_uri = format!(
            "/v1/apis/{api_id}/functions/{function_id}",
            api_id = input.api_id,
            function_id = input.function_id
        );

        let mut request = SignedRequest::new("DELETE", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteFunctionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteFunctionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a <code>GraphqlApi</code> object.</p>
    fn delete_graphql_api(
        &self,
        input: DeleteGraphqlApiRequest,
    ) -> RusotoFuture<DeleteGraphqlApiResponse, DeleteGraphqlApiError> {
        let request_uri = format!("/v1/apis/{api_id}", api_id = input.api_id);

        let mut request = SignedRequest::new("DELETE", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteGraphqlApiResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteGraphqlApiError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a <code>Resolver</code> object.</p>
    fn delete_resolver(
        &self,
        input: DeleteResolverRequest,
    ) -> RusotoFuture<DeleteResolverResponse, DeleteResolverError> {
        let request_uri = format!(
            "/v1/apis/{api_id}/types/{type_name}/resolvers/{field_name}",
            api_id = input.api_id,
            field_name = input.field_name,
            type_name = input.type_name
        );

        let mut request = SignedRequest::new("DELETE", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteResolverResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteResolverError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a <code>Type</code> object.</p>
    fn delete_type(
        &self,
        input: DeleteTypeRequest,
    ) -> RusotoFuture<DeleteTypeResponse, DeleteTypeError> {
        let request_uri = format!(
            "/v1/apis/{api_id}/types/{type_name}",
            api_id = input.api_id,
            type_name = input.type_name
        );

        let mut request = SignedRequest::new("DELETE", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DeleteTypeResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteTypeError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a <code>DataSource</code> object.</p>
    fn get_data_source(
        &self,
        input: GetDataSourceRequest,
    ) -> RusotoFuture<GetDataSourceResponse, GetDataSourceError> {
        let request_uri = format!(
            "/v1/apis/{api_id}/datasources/{name}",
            api_id = input.api_id,
            name = input.name
        );

        let mut request = SignedRequest::new("GET", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetDataSourceResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDataSourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Get a <code>Function</code>.</p>
    fn get_function(
        &self,
        input: GetFunctionRequest,
    ) -> RusotoFuture<GetFunctionResponse, GetFunctionError> {
        let request_uri = format!(
            "/v1/apis/{api_id}/functions/{function_id}",
            api_id = input.api_id,
            function_id = input.function_id
        );

        let mut request = SignedRequest::new("GET", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetFunctionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetFunctionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a <code>GraphqlApi</code> object.</p>
    fn get_graphql_api(
        &self,
        input: GetGraphqlApiRequest,
    ) -> RusotoFuture<GetGraphqlApiResponse, GetGraphqlApiError> {
        let request_uri = format!("/v1/apis/{api_id}", api_id = input.api_id);

        let mut request = SignedRequest::new("GET", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetGraphqlApiResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetGraphqlApiError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the introspection schema for a GraphQL API.</p>
    fn get_introspection_schema(
        &self,
        input: GetIntrospectionSchemaRequest,
    ) -> RusotoFuture<GetIntrospectionSchemaResponse, GetIntrospectionSchemaError> {
        let request_uri = format!("/v1/apis/{api_id}/schema", api_id = input.api_id);

        let mut request = SignedRequest::new("GET", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("format", &input.format);
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut result = GetIntrospectionSchemaResponse::default();
                    result.schema = Some(response.body);

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetIntrospectionSchemaError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves a <code>Resolver</code> object.</p>
    fn get_resolver(
        &self,
        input: GetResolverRequest,
    ) -> RusotoFuture<GetResolverResponse, GetResolverError> {
        let request_uri = format!(
            "/v1/apis/{api_id}/types/{type_name}/resolvers/{field_name}",
            api_id = input.api_id,
            field_name = input.field_name,
            type_name = input.type_name
        );

        let mut request = SignedRequest::new("GET", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetResolverResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetResolverError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the current status of a schema creation operation.</p>
    fn get_schema_creation_status(
        &self,
        input: GetSchemaCreationStatusRequest,
    ) -> RusotoFuture<GetSchemaCreationStatusResponse, GetSchemaCreationStatusError> {
        let request_uri = format!("/v1/apis/{api_id}/schemacreation", api_id = input.api_id);

        let mut request = SignedRequest::new("GET", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetSchemaCreationStatusResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetSchemaCreationStatusError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves a <code>Type</code> object.</p>
    fn get_type(&self, input: GetTypeRequest) -> RusotoFuture<GetTypeResponse, GetTypeError> {
        let request_uri = format!(
            "/v1/apis/{api_id}/types/{type_name}",
            api_id = input.api_id,
            type_name = input.type_name
        );

        let mut request = SignedRequest::new("GET", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("format", &input.format);
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetTypeResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTypeError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Lists the API keys for a given API.</p> <note> <p>API keys are deleted automatically sometime after they expire. However, they may still be included in the response until they have actually been deleted. You can safely call <code>DeleteApiKey</code> to manually delete a key before it&#39;s automatically deleted.</p> </note></p>
    fn list_api_keys(
        &self,
        input: ListApiKeysRequest,
    ) -> RusotoFuture<ListApiKeysResponse, ListApiKeysError> {
        let request_uri = format!("/v1/apis/{api_id}/apikeys", api_id = input.api_id);

        let mut request = SignedRequest::new("GET", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListApiKeysResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListApiKeysError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the data sources for a given API.</p>
    fn list_data_sources(
        &self,
        input: ListDataSourcesRequest,
    ) -> RusotoFuture<ListDataSourcesResponse, ListDataSourcesError> {
        let request_uri = format!("/v1/apis/{api_id}/datasources", api_id = input.api_id);

        let mut request = SignedRequest::new("GET", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListDataSourcesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListDataSourcesError::from_response(response))),
                )
            }
        })
    }

    /// <p>List multiple functions.</p>
    fn list_functions(
        &self,
        input: ListFunctionsRequest,
    ) -> RusotoFuture<ListFunctionsResponse, ListFunctionsError> {
        let request_uri = format!("/v1/apis/{api_id}/functions", api_id = input.api_id);

        let mut request = SignedRequest::new("GET", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListFunctionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListFunctionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists your GraphQL APIs.</p>
    fn list_graphql_apis(
        &self,
        input: ListGraphqlApisRequest,
    ) -> RusotoFuture<ListGraphqlApisResponse, ListGraphqlApisError> {
        let request_uri = "/v1/apis";

        let mut request = SignedRequest::new("GET", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListGraphqlApisResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListGraphqlApisError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the resolvers for a given API and type.</p>
    fn list_resolvers(
        &self,
        input: ListResolversRequest,
    ) -> RusotoFuture<ListResolversResponse, ListResolversError> {
        let request_uri = format!(
            "/v1/apis/{api_id}/types/{type_name}/resolvers",
            api_id = input.api_id,
            type_name = input.type_name
        );

        let mut request = SignedRequest::new("GET", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListResolversResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListResolversError::from_response(response))),
                )
            }
        })
    }

    /// <p>List the resolvers that are associated with a specific function.</p>
    fn list_resolvers_by_function(
        &self,
        input: ListResolversByFunctionRequest,
    ) -> RusotoFuture<ListResolversByFunctionResponse, ListResolversByFunctionError> {
        let request_uri = format!(
            "/v1/apis/{api_id}/functions/{function_id}/resolvers",
            api_id = input.api_id,
            function_id = input.function_id
        );

        let mut request = SignedRequest::new("GET", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListResolversByFunctionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListResolversByFunctionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists the types for a given API.</p>
    fn list_types(
        &self,
        input: ListTypesRequest,
    ) -> RusotoFuture<ListTypesResponse, ListTypesError> {
        let request_uri = format!("/v1/apis/{api_id}/types", api_id = input.api_id);

        let mut request = SignedRequest::new("GET", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("format", &input.format);
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListTypesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListTypesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Adds a new schema to your GraphQL API.</p> <p>This operation is asynchronous. Use to determine when it has completed.</p>
    fn start_schema_creation(
        &self,
        input: StartSchemaCreationRequest,
    ) -> RusotoFuture<StartSchemaCreationResponse, StartSchemaCreationError> {
        let request_uri = format!("/v1/apis/{api_id}/schemacreation", api_id = input.api_id);

        let mut request = SignedRequest::new("POST", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<StartSchemaCreationResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartSchemaCreationError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates an API key.</p>
    fn update_api_key(
        &self,
        input: UpdateApiKeyRequest,
    ) -> RusotoFuture<UpdateApiKeyResponse, UpdateApiKeyError> {
        let request_uri = format!(
            "/v1/apis/{api_id}/apikeys/{id}",
            api_id = input.api_id,
            id = input.id
        );

        let mut request = SignedRequest::new("POST", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateApiKeyResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateApiKeyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a <code>DataSource</code> object.</p>
    fn update_data_source(
        &self,
        input: UpdateDataSourceRequest,
    ) -> RusotoFuture<UpdateDataSourceResponse, UpdateDataSourceError> {
        let request_uri = format!(
            "/v1/apis/{api_id}/datasources/{name}",
            api_id = input.api_id,
            name = input.name
        );

        let mut request = SignedRequest::new("POST", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateDataSourceResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateDataSourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a <code>Function</code> object.</p>
    fn update_function(
        &self,
        input: UpdateFunctionRequest,
    ) -> RusotoFuture<UpdateFunctionResponse, UpdateFunctionError> {
        let request_uri = format!(
            "/v1/apis/{api_id}/functions/{function_id}",
            api_id = input.api_id,
            function_id = input.function_id
        );

        let mut request = SignedRequest::new("POST", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateFunctionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateFunctionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a <code>GraphqlApi</code> object.</p>
    fn update_graphql_api(
        &self,
        input: UpdateGraphqlApiRequest,
    ) -> RusotoFuture<UpdateGraphqlApiResponse, UpdateGraphqlApiError> {
        let request_uri = format!("/v1/apis/{api_id}", api_id = input.api_id);

        let mut request = SignedRequest::new("POST", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateGraphqlApiResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateGraphqlApiError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a <code>Resolver</code> object.</p>
    fn update_resolver(
        &self,
        input: UpdateResolverRequest,
    ) -> RusotoFuture<UpdateResolverResponse, UpdateResolverError> {
        let request_uri = format!(
            "/v1/apis/{api_id}/types/{type_name}/resolvers/{field_name}",
            api_id = input.api_id,
            field_name = input.field_name,
            type_name = input.type_name
        );

        let mut request = SignedRequest::new("POST", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateResolverResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateResolverError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a <code>Type</code> object.</p>
    fn update_type(
        &self,
        input: UpdateTypeRequest,
    ) -> RusotoFuture<UpdateTypeResponse, UpdateTypeError> {
        let request_uri = format!(
            "/v1/apis/{api_id}/types/{type_name}",
            api_id = input.api_id,
            type_name = input.type_name
        );

        let mut request = SignedRequest::new("POST", "appsync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.as_ref() == b"null" || body.is_empty() {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateTypeResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateTypeError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
