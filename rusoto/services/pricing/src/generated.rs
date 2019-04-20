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

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>The values of a given attribute, such as <code>Throughput Optimized HDD</code> or <code>Provisioned IOPS</code> for the <code>Amazon EC2</code> <code>volumeType</code> attribute.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AttributeValue {
    /// <p>The specific value of an <code>attributeName</code>.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeServicesRequest {
    /// <p>The format version that you want the response to be in.</p> <p>Valid values are: <code>aws_v1</code> </p>
    #[serde(rename = "FormatVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_version: Option<String>,
    /// <p>The maximum number of results that you want returned in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The code for the service whose information you want to retrieve, such as <code>AmazonEC2</code>. You can use the <code>ServiceCode</code> to filter the results in a <code>GetProducts</code> call. To retrieve a list of all services, leave this blank.</p>
    #[serde(rename = "ServiceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeServicesResponse {
    /// <p>The format version of the response. For example, <code>aws_v1</code>.</p>
    #[serde(rename = "FormatVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_version: Option<String>,
    /// <p>The pagination token for the next set of retreivable results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The service metadata for the service or services in the response.</p>
    #[serde(rename = "Services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
}

/// <p>The constraints that you want all returned products to match.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Filter {
    /// <p>The product metadata field that you want to filter on. You can filter by just the service code to see all products for a specific service, filter by just the attribute name to see a specific attribute for multiple services, or use both a service code and an attribute name to retrieve only products that match both fields.</p> <p>Valid values include: <code>ServiceCode</code>, and all attribute names</p> <p>For example, you can filter by the <code>AmazonEC2</code> service code and the <code>volumeType</code> attribute name to get the prices for only Amazon EC2 volumes.</p>
    #[serde(rename = "Field")]
    pub field: String,
    /// <p>The type of filter that you want to use.</p> <p>Valid values are: <code>TERM_MATCH</code>. <code>TERM_MATCH</code> returns only products that match both the given filter field and the given value.</p>
    #[serde(rename = "Type")]
    pub type_: String,
    /// <p>The service code or attribute value that you want to filter by. If you are filtering by service code this is the actual service code, such as <code>AmazonEC2</code>. If you are filtering by attribute name, this is the attribute value that you want the returned products to match, such as a <code>Provisioned IOPS</code> volume.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAttributeValuesRequest {
    /// <p>The name of the attribute that you want to retrieve the values for, such as <code>volumeType</code>.</p>
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
    /// <p>The maximum number of results to return in response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The service code for the service whose attributes you want to retrieve. For example, if you want the retrieve an EC2 attribute, use <code>AmazonEC2</code>.</p>
    #[serde(rename = "ServiceCode")]
    pub service_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAttributeValuesResponse {
    /// <p>The list of values for an attribute. For example, <code>Throughput Optimized HDD</code> and <code>Provisioned IOPS</code> are two available values for the <code>AmazonEC2</code> <code>volumeType</code>.</p>
    #[serde(rename = "AttributeValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_values: Option<Vec<AttributeValue>>,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetProductsRequest {
    /// <p>The list of filters that limit the returned products. only products that match all filters are returned.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The format version that you want the response to be in.</p> <p>Valid values are: <code>aws_v1</code> </p>
    #[serde(rename = "FormatVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_version: Option<String>,
    /// <p>The maximum number of results to return in the response.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The code for the service whose products you want to retrieve. </p>
    #[serde(rename = "ServiceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetProductsResponse {
    /// <p>The format version of the response. For example, aws_v1.</p>
    #[serde(rename = "FormatVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_version: Option<String>,
    /// <p>The pagination token that indicates the next set of results to retrieve.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of products that match your filters. The list contains both the product metadata and the price information.</p>
    #[serde(rename = "PriceList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_list: Option<Vec<String>>,
}

/// <p>The metadata for a service, such as the service code and available attribute names.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Service {
    /// <p>The attributes that are available for this service.</p>
    #[serde(rename = "AttributeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_names: Option<Vec<String>>,
    /// <p>The code for the AWS service.</p>
    #[serde(rename = "ServiceCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
}

/// Errors returned by DescribeServices
#[derive(Debug, PartialEq)]
pub enum DescribeServicesError {
    /// <p>The pagination token expired. Try again without a pagination token.</p>
    ExpiredNextToken(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters had an invalid value.</p>
    InvalidParameter(String),
    /// <p>The requested resource can't be found.</p>
    NotFound(String),
}

impl DescribeServicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeServicesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ExpiredNextTokenException" => {
                    return RusotoError::Service(DescribeServicesError::ExpiredNextToken(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeServicesError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeServicesError::InvalidNextToken(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeServicesError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeServicesError::NotFound(String::from(
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
impl fmt::Display for DescribeServicesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeServicesError {
    fn description(&self) -> &str {
        match *self {
            DescribeServicesError::ExpiredNextToken(ref cause) => cause,
            DescribeServicesError::InternalError(ref cause) => cause,
            DescribeServicesError::InvalidNextToken(ref cause) => cause,
            DescribeServicesError::InvalidParameter(ref cause) => cause,
            DescribeServicesError::NotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAttributeValues
#[derive(Debug, PartialEq)]
pub enum GetAttributeValuesError {
    /// <p>The pagination token expired. Try again without a pagination token.</p>
    ExpiredNextToken(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters had an invalid value.</p>
    InvalidParameter(String),
    /// <p>The requested resource can't be found.</p>
    NotFound(String),
}

impl GetAttributeValuesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAttributeValuesError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ExpiredNextTokenException" => {
                    return RusotoError::Service(GetAttributeValuesError::ExpiredNextToken(
                        String::from(error_message),
                    ))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(GetAttributeValuesError::InternalError(
                        String::from(error_message),
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(GetAttributeValuesError::InvalidNextToken(
                        String::from(error_message),
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetAttributeValuesError::InvalidParameter(
                        String::from(error_message),
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetAttributeValuesError::NotFound(String::from(
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
impl fmt::Display for GetAttributeValuesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAttributeValuesError {
    fn description(&self) -> &str {
        match *self {
            GetAttributeValuesError::ExpiredNextToken(ref cause) => cause,
            GetAttributeValuesError::InternalError(ref cause) => cause,
            GetAttributeValuesError::InvalidNextToken(ref cause) => cause,
            GetAttributeValuesError::InvalidParameter(ref cause) => cause,
            GetAttributeValuesError::NotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by GetProducts
#[derive(Debug, PartialEq)]
pub enum GetProductsError {
    /// <p>The pagination token expired. Try again without a pagination token.</p>
    ExpiredNextToken(String),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalError(String),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextToken(String),
    /// <p>One or more parameters had an invalid value.</p>
    InvalidParameter(String),
    /// <p>The requested resource can't be found.</p>
    NotFound(String),
}

impl GetProductsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetProductsError> {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "ExpiredNextTokenException" => {
                    return RusotoError::Service(GetProductsError::ExpiredNextToken(String::from(
                        error_message,
                    )))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(GetProductsError::InternalError(String::from(
                        error_message,
                    )))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(GetProductsError::InvalidNextToken(String::from(
                        error_message,
                    )))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetProductsError::InvalidParameter(String::from(
                        error_message,
                    )))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetProductsError::NotFound(String::from(
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
impl fmt::Display for GetProductsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetProductsError {
    fn description(&self) -> &str {
        match *self {
            GetProductsError::ExpiredNextToken(ref cause) => cause,
            GetProductsError::InternalError(ref cause) => cause,
            GetProductsError::InvalidNextToken(ref cause) => cause,
            GetProductsError::InvalidParameter(ref cause) => cause,
            GetProductsError::NotFound(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Pricing API. AWS Pricing clients implement this trait.
pub trait Pricing {
    /// <p>Returns the metadata for one service or a list of the metadata for all services. Use this without a service code to get the service codes for all services. Use it with a service code, such as <code>AmazonEC2</code>, to get information specific to that service, such as the attribute names available for that service. For example, some of the attribute names available for EC2 are <code>volumeType</code>, <code>maxIopsVolume</code>, <code>operation</code>, <code>locationType</code>, and <code>instanceCapacity10xlarge</code>.</p>
    fn describe_services(
        &self,
        input: DescribeServicesRequest,
    ) -> RusotoFuture<DescribeServicesResponse, DescribeServicesError>;

    /// <p>Returns a list of attribute values. Attibutes are similar to the details in a Price List API offer file. For a list of available attributes, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/reading-an-offer.html#pps-defs">Offer File Definitions</a> in the <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/billing-what-is.html">AWS Billing and Cost Management User Guide</a>.</p>
    fn get_attribute_values(
        &self,
        input: GetAttributeValuesRequest,
    ) -> RusotoFuture<GetAttributeValuesResponse, GetAttributeValuesError>;

    /// <p>Returns a list of all products that match the filter criteria.</p>
    fn get_products(
        &self,
        input: GetProductsRequest,
    ) -> RusotoFuture<GetProductsResponse, GetProductsError>;
}
/// A client for the AWS Pricing API.
#[derive(Clone)]
pub struct PricingClient {
    client: Client,
    region: region::Region,
}

impl PricingClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> PricingClient {
        PricingClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> PricingClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        PricingClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Pricing for PricingClient {
    /// <p>Returns the metadata for one service or a list of the metadata for all services. Use this without a service code to get the service codes for all services. Use it with a service code, such as <code>AmazonEC2</code>, to get information specific to that service, such as the attribute names available for that service. For example, some of the attribute names available for EC2 are <code>volumeType</code>, <code>maxIopsVolume</code>, <code>operation</code>, <code>locationType</code>, and <code>instanceCapacity10xlarge</code>.</p>
    fn describe_services(
        &self,
        input: DescribeServicesRequest,
    ) -> RusotoFuture<DescribeServicesResponse, DescribeServicesError> {
        let mut request = SignedRequest::new("POST", "pricing", &self.region, "/");
        request.set_endpoint_prefix("api.pricing".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSPriceListService.DescribeServices");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<DescribeServicesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeServicesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of attribute values. Attibutes are similar to the details in a Price List API offer file. For a list of available attributes, see <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/reading-an-offer.html#pps-defs">Offer File Definitions</a> in the <a href="http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/billing-what-is.html">AWS Billing and Cost Management User Guide</a>.</p>
    fn get_attribute_values(
        &self,
        input: GetAttributeValuesRequest,
    ) -> RusotoFuture<GetAttributeValuesResponse, GetAttributeValuesError> {
        let mut request = SignedRequest::new("POST", "pricing", &self.region, "/");
        request.set_endpoint_prefix("api.pricing".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSPriceListService.GetAttributeValues");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<GetAttributeValuesResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAttributeValuesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of all products that match the filter criteria.</p>
    fn get_products(
        &self,
        input: GetProductsRequest,
    ) -> RusotoFuture<GetProductsResponse, GetProductsError> {
        let mut request = SignedRequest::new("POST", "pricing", &self.region, "/");
        request.set_endpoint_prefix("api.pricing".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSPriceListService.GetProducts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body.as_ref() == b"null" {
                        body = bytes::Bytes::from_static(b"{}");
                    }

                    serde_json::from_str::<GetProductsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetProductsError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
