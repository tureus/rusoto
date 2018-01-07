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

#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use std::io;
use std::io::Read;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>The input for the AcceptCertificateTransfer operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct AcceptCertificateTransferRequest {
    /// <p>The ID of the certificate.</p>
    #[serde(rename = "certificateId")]
    pub certificate_id: String,
    /// <p>Specifies whether the certificate is active.</p>
    #[serde(rename = "setAsActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_as_active: Option<bool>,
}

/// <p>Describes the actions associated with a rule.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// <p>Change the state of a CloudWatch alarm.</p>
    #[serde(rename = "cloudwatchAlarm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_alarm: Option<CloudwatchAlarmAction>,
    /// <p>Capture a CloudWatch metric.</p>
    #[serde(rename = "cloudwatchMetric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_metric: Option<CloudwatchMetricAction>,
    /// <p>Write to a DynamoDB table.</p>
    #[serde(rename = "dynamoDB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_db: Option<DynamoDBAction>,
    /// <p>Write to a DynamoDB table. This is a new version of the DynamoDB action. It allows you to write each attribute in an MQTT message payload into a separate DynamoDB column.</p>
    #[serde(rename = "dynamoDBv2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_d_bv_2: Option<DynamoDBv2Action>,
    /// <p>Write data to an Amazon Elasticsearch Service domain.</p>
    #[serde(rename = "elasticsearch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch: Option<ElasticsearchAction>,
    /// <p>Write to an Amazon Kinesis Firehose stream.</p>
    #[serde(rename = "firehose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firehose: Option<FirehoseAction>,
    /// <p>Write data to an Amazon Kinesis stream.</p>
    #[serde(rename = "kinesis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis: Option<KinesisAction>,
    /// <p>Invoke a Lambda function.</p>
    #[serde(rename = "lambda")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda: Option<LambdaAction>,
    /// <p>Publish to another MQTT topic.</p>
    #[serde(rename = "republish")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub republish: Option<RepublishAction>,
    /// <p>Write to an Amazon S3 bucket.</p>
    #[serde(rename = "s3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3: Option<S3Action>,
    /// <p>Send a message to a Salesforce IoT Cloud Input Stream.</p>
    #[serde(rename = "salesforce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salesforce: Option<SalesforceAction>,
    /// <p>Publish to an Amazon SNS topic.</p>
    #[serde(rename = "sns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns: Option<SnsAction>,
    /// <p>Publish to an Amazon SQS queue.</p>
    #[serde(rename = "sqs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs: Option<SqsAction>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct AddThingToThingGroupRequest {
    /// <p>The ARN of the thing to add to a group.</p>
    #[serde(rename = "thingArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
    /// <p>The ARN of the group to which you are adding a thing.</p>
    #[serde(rename = "thingGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_arn: Option<String>,
    /// <p>The name of the group to which you are adding a thing.</p>
    #[serde(rename = "thingGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_name: Option<String>,
    /// <p>The name of the thing to add to a group.</p>
    #[serde(rename = "thingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct AddThingToThingGroupResponse;

/// <p>Contains information that allowed the authorization.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Allowed {
    /// <p>A list of policies that allowed the authentication.</p>
    #[serde(rename = "policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Policy>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct AssociateTargetsWithJobRequest {
    /// <p>An optional comment string describing why the job was associated with the targets.</p>
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>A list of thing group ARNs that define the targets of the job.</p>
    #[serde(rename = "targets")]
    pub targets: Vec<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct AssociateTargetsWithJobResponse {
    /// <p>A short text description of the job.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>An ARN identifying the job.</p>
    #[serde(rename = "jobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct AttachPolicyRequest {
    /// <p>The name of the policy to attach.</p>
    #[serde(rename = "policyName")]
    pub policy_name: String,
    /// <p>The identity to which the policy is attached.</p>
    #[serde(rename = "target")]
    pub target: String,
}

/// <p>The input for the AttachPrincipalPolicy operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct AttachPrincipalPolicyRequest {
    /// <p>The policy name.</p>
    #[serde(rename = "policyName")]
    pub policy_name: String,
    /// <p>The principal, which can be a certificate ARN (as returned from the CreateCertificate operation) or an Amazon Cognito ID.</p>
    #[serde(rename = "principal")]
    pub principal: String,
}

/// <p>The input for the AttachThingPrincipal operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct AttachThingPrincipalRequest {
    /// <p>The principal, such as a certificate or other credential.</p>
    #[serde(rename = "principal")]
    pub principal: String,
    /// <p>The name of the thing.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

/// <p>The output from the AttachThingPrincipal operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct AttachThingPrincipalResponse;

/// <p>The attribute payload.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AttributePayload {
    /// <p>A JSON string containing up to three key-value pair in JSON format. For example:</p> <p> <code>{\"attributes\":{\"string1\":\"string2\"}}</code> </p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies whether the list of attributes provided in the <code>AttributePayload</code> is merged with the attributes stored in the registry, instead of overwriting them.</p> <p>To remove an attribute, call <code>UpdateThing</code> with an empty attribute value.</p> <note> <p>The <code>merge</code> attribute is only valid when calling <code>UpdateThing</code>.</p> </note>
    #[serde(rename = "merge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge: Option<bool>,
}

/// <p>A collection of authorization information.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AuthInfo {
    /// <p>The type of action for which the principal is being authorized.</p>
    #[serde(rename = "actionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    /// <p>The resources for which the principal is being authorized to perform the specified action.</p>
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
}

/// <p>The authorizer result.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct AuthResult {
    /// <p>The policies and statements that allowed the specified action.</p>
    #[serde(rename = "allowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed: Option<Allowed>,
    /// <p>The final authorization decision of this scenario. Multiple statements are taken into account when determining the authorization decision. An explicit deny statement can override multiple allow statements.</p>
    #[serde(rename = "authDecision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_decision: Option<String>,
    /// <p>Authorization information.</p>
    #[serde(rename = "authInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_info: Option<AuthInfo>,
    /// <p>The policies and statements that denied the specified action.</p>
    #[serde(rename = "denied")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denied: Option<Denied>,
    /// <p>Contains any missing context values found while evaluating policy.</p>
    #[serde(rename = "missingContextValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_context_values: Option<Vec<String>>,
}

/// <p>The authorizer description.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct AuthorizerDescription {
    /// <p>The authorizer ARN.</p>
    #[serde(rename = "authorizerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_arn: Option<String>,
    /// <p>The authorizer's Lambda function ARN.</p>
    #[serde(rename = "authorizerFunctionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_function_arn: Option<String>,
    /// <p>The authorizer name.</p>
    #[serde(rename = "authorizerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_name: Option<String>,
    /// <p>The UNIX timestamp of when the authorizer was created.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The UNIX timestamp of when the authorizer was last updated.</p>
    #[serde(rename = "lastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>The status of the authorizer.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The key used to extract the token from the HTTP headers.</p>
    #[serde(rename = "tokenKeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_name: Option<String>,
    /// <p>The public keys used to validate the token signature returned by your custom authentication service.</p>
    #[serde(rename = "tokenSigningPublicKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_signing_public_keys: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The authorizer summary.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct AuthorizerSummary {
    /// <p>The authorizer ARN.</p>
    #[serde(rename = "authorizerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_arn: Option<String>,
    /// <p>The authorizer name.</p>
    #[serde(rename = "authorizerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_name: Option<String>,
}

/// <p>A CA certificate.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct CACertificate {
    /// <p>The ARN of the CA certificate.</p>
    #[serde(rename = "certificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The ID of the CA certificate.</p>
    #[serde(rename = "certificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    /// <p>The date the CA certificate was created.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The status of the CA certificate.</p> <p>The status value REGISTER_INACTIVE is deprecated and should not be used.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Describes a CA certificate.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct CACertificateDescription {
    /// <p>Whether the CA certificate configured for auto registration of device certificates. Valid values are "ENABLE" and "DISABLE"</p>
    #[serde(rename = "autoRegistrationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_registration_status: Option<String>,
    /// <p>The CA certificate ARN.</p>
    #[serde(rename = "certificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The CA certificate ID.</p>
    #[serde(rename = "certificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    /// <p>The CA certificate data, in PEM format.</p>
    #[serde(rename = "certificatePem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_pem: Option<String>,
    /// <p>The date the CA certificate was created.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The owner of the CA certificate.</p>
    #[serde(rename = "ownedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_by: Option<String>,
    /// <p>The status of a CA certificate.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The input for the CancelCertificateTransfer operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct CancelCertificateTransferRequest {
    /// <p>The ID of the certificate.</p>
    #[serde(rename = "certificateId")]
    pub certificate_id: String,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CancelJobRequest {
    /// <p>An optional comment string describing why the job was canceled.</p>
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CancelJobResponse {
    /// <p>A short text description of the job.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The job ARN.</p>
    #[serde(rename = "jobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

/// <p>Information about a certificate.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Certificate {
    /// <p>The ARN of the certificate.</p>
    #[serde(rename = "certificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The ID of the certificate.</p>
    #[serde(rename = "certificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    /// <p>The date and time the certificate was created.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The status of the certificate.</p> <p>The status value REGISTER_INACTIVE is deprecated and should not be used.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Describes a certificate.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct CertificateDescription {
    /// <p>The certificate ID of the CA certificate used to sign this certificate.</p>
    #[serde(rename = "caCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_id: Option<String>,
    /// <p>The ARN of the certificate.</p>
    #[serde(rename = "certificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The ID of the certificate.</p>
    #[serde(rename = "certificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    /// <p>The certificate data, in PEM format.</p>
    #[serde(rename = "certificatePem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_pem: Option<String>,
    /// <p>The date and time the certificate was created.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The date and time the certificate was last modified.</p>
    #[serde(rename = "lastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>The ID of the AWS account that owns the certificate.</p>
    #[serde(rename = "ownedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_by: Option<String>,
    /// <p>The ID of the AWS account of the previous owner of the certificate.</p>
    #[serde(rename = "previousOwnedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_owned_by: Option<String>,
    /// <p>The status of the certificate.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The transfer data.</p>
    #[serde(rename = "transferData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferData>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ClearDefaultAuthorizerRequest;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ClearDefaultAuthorizerResponse;

/// <p>Describes an action that updates a CloudWatch alarm.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CloudwatchAlarmAction {
    /// <p>The CloudWatch alarm name.</p>
    #[serde(rename = "alarmName")]
    pub alarm_name: String,
    /// <p>The IAM role that allows access to the CloudWatch alarm.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>The reason for the alarm change.</p>
    #[serde(rename = "stateReason")]
    pub state_reason: String,
    /// <p>The value of the alarm state. Acceptable values are: OK, ALARM, INSUFFICIENT_DATA.</p>
    #[serde(rename = "stateValue")]
    pub state_value: String,
}

/// <p>Describes an action that captures a CloudWatch metric.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CloudwatchMetricAction {
    /// <p>The CloudWatch metric name.</p>
    #[serde(rename = "metricName")]
    pub metric_name: String,
    /// <p>The CloudWatch metric namespace name.</p>
    #[serde(rename = "metricNamespace")]
    pub metric_namespace: String,
    /// <p>An optional <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/cloudwatch_concepts.html#about_timestamp">Unix timestamp</a>.</p>
    #[serde(rename = "metricTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_timestamp: Option<String>,
    /// <p>The <a href="http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/cloudwatch_concepts.html#Unit">metric unit</a> supported by CloudWatch.</p>
    #[serde(rename = "metricUnit")]
    pub metric_unit: String,
    /// <p>The CloudWatch metric value.</p>
    #[serde(rename = "metricValue")]
    pub metric_value: String,
    /// <p>The IAM role that allows access to the CloudWatch metric.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
}

/// <p>Describes the method to use when code signing a file.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CodeSigning {
    /// <p>The ID of the AWSSignerJob which was created to sign the file.</p>
    #[serde(rename = "awsSignerJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_signer_job_id: Option<String>,
    /// <p>A custom method for code signing a file.</p>
    #[serde(rename = "customCodeSigning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_code_signing: Option<CustomCodeSigning>,
}

/// <p>Describes the certificate chain being used when code signing a file.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CodeSigningCertificateChain {
    /// <p>The name of the certificate.</p>
    #[serde(rename = "certificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
    /// <p>A base64 encoded binary representation of the code signing certificate chain.</p>
    #[serde(rename = "inlineDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_document: Option<String>,
    /// <p>A stream of the certificate chain files.</p>
    #[serde(rename = "stream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<Stream>,
}

/// <p>Describes the signature for a file.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CodeSigningSignature {
    /// <p>A base64 encoded binary representation of the code signing signature.</p>
    #[serde(rename = "inlineDocument")]
    #[serde(deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
            serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob", default)]
    pub inline_document: Option<Vec<u8>>,
    /// <p>A stream of the code signing signature.</p>
    #[serde(rename = "stream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<Stream>,
}

/// <p>Configuration.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    /// <p>True to enable the configuration.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateAuthorizerRequest {
    /// <p>The ARN of the authorizer's Lambda function.</p>
    #[serde(rename = "authorizerFunctionArn")]
    pub authorizer_function_arn: String,
    /// <p>The authorizer name.</p>
    #[serde(rename = "authorizerName")]
    pub authorizer_name: String,
    /// <p>The status of the create authorizer request.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The name of the token key used to extract the token from the HTTP headers.</p>
    #[serde(rename = "tokenKeyName")]
    pub token_key_name: String,
    /// <p>The public keys used to verify the digital signature returned by your custom authentication service.</p>
    #[serde(rename = "tokenSigningPublicKeys")]
    pub token_signing_public_keys: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateAuthorizerResponse {
    /// <p>The authorizer ARN.</p>
    #[serde(rename = "authorizerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_arn: Option<String>,
    /// <p>The authorizer's name.</p>
    #[serde(rename = "authorizerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_name: Option<String>,
}

/// <p>The input for the CreateCertificateFromCsr operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateCertificateFromCsrRequest {
    /// <p>The certificate signing request (CSR).</p>
    #[serde(rename = "certificateSigningRequest")]
    pub certificate_signing_request: String,
    /// <p>Specifies whether the certificate is active.</p>
    #[serde(rename = "setAsActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_as_active: Option<bool>,
}

/// <p>The output from the CreateCertificateFromCsr operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateCertificateFromCsrResponse {
    /// <p>The Amazon Resource Name (ARN) of the certificate. You can use the ARN as a principal for policy operations.</p>
    #[serde(rename = "certificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The ID of the certificate. Certificate management operations only take a certificateId.</p>
    #[serde(rename = "certificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    /// <p>The certificate data, in PEM format.</p>
    #[serde(rename = "certificatePem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_pem: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateJobRequest {
    /// <p>A short text description of the job.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The job document.</p>
    #[serde(rename = "document")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    /// <p>Parameters for the job document.</p>
    #[serde(rename = "documentParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>An S3 link to the job document.</p>
    #[serde(rename = "documentSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_source: Option<String>,
    /// <p>Allows you to create a staged rollout of the job.</p>
    #[serde(rename = "jobExecutionsRolloutConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_executions_rollout_config: Option<JobExecutionsRolloutConfig>,
    /// <p>A job identifier which must be unique for your AWS account. We recommend using a UUID. Alpha-numeric characters, "-" and "_" are valid for use here.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>Configuration information for pre-signed S3 URLs.</p>
    #[serde(rename = "presignedUrlConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presigned_url_config: Option<PresignedUrlConfig>,
    /// <p>Specifies whether the job will continue to run (CONTINUOUS), or will be complete after all those things specified as targets have completed the job (SNAPSHOT). If continuous, the job may also be run on a thing when a change is detected in a target. For example, a job will run on a thing when the thing is added to a target group, even after the job was completed by all things originally in the group.</p>
    #[serde(rename = "targetSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_selection: Option<String>,
    /// <p>A list of things and thing groups to which the job should be sent.</p>
    #[serde(rename = "targets")]
    pub targets: Vec<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateJobResponse {
    /// <p>The job description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The job ARN.</p>
    #[serde(rename = "jobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    /// <p>The unique identifier you assigned to this job.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

/// <p>The input for the CreateKeysAndCertificate operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateKeysAndCertificateRequest {
    /// <p>Specifies whether the certificate is active.</p>
    #[serde(rename = "setAsActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_as_active: Option<bool>,
}

/// <p>The output of the CreateKeysAndCertificate operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateKeysAndCertificateResponse {
    /// <p>The ARN of the certificate.</p>
    #[serde(rename = "certificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The ID of the certificate. AWS IoT issues a default subject name for the certificate (for example, AWS IoT Certificate).</p>
    #[serde(rename = "certificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    /// <p>The certificate data, in PEM format.</p>
    #[serde(rename = "certificatePem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_pem: Option<String>,
    /// <p>The generated key pair.</p>
    #[serde(rename = "keyPair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<KeyPair>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateOTAUpdateRequest {
    /// <p>A list of additional OTA update parameters which are name-value pairs.</p>
    #[serde(rename = "additionalParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The description of the OTA update.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The files to be streamed by the OTA update.</p>
    #[serde(rename = "files")]
    pub files: Vec<OTAUpdateFile>,
    /// <p>The ID of the OTA update to be created.</p>
    #[serde(rename = "otaUpdateId")]
    pub ota_update_id: String,
    /// <p>The IAM role that allows access to the AWS IoT Jobs service.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>Specifies whether the update will continue to run (CONTINUOUS), or will be complete after all the things specified as targets have completed the update (SNAPSHOT). If continuous, the update may also be run on a thing when a change is detected in a target. For example, an update will run on a thing when the thing is added to a target group, even after the update was completed by all things originally in the group. Valid values: CONTINUOUS | SNAPSHOT.</p>
    #[serde(rename = "targetSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_selection: Option<String>,
    /// <p>The targeted devices to receive OTA updates.</p>
    #[serde(rename = "targets")]
    pub targets: Vec<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateOTAUpdateResponse {
    /// <p>The AWS IoT job ARN associated with the OTA update.</p>
    #[serde(rename = "awsIotJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iot_job_arn: Option<String>,
    /// <p>The AWS IoT job ID associated with the OTA update.</p>
    #[serde(rename = "awsIotJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iot_job_id: Option<String>,
    /// <p>The OTA update ARN.</p>
    #[serde(rename = "otaUpdateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_arn: Option<String>,
    /// <p>The OTA update ID.</p>
    #[serde(rename = "otaUpdateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_id: Option<String>,
    /// <p>The OTA update status.</p>
    #[serde(rename = "otaUpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_status: Option<String>,
}

/// <p>The input for the CreatePolicy operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct CreatePolicyRequest {
    /// <p>The JSON document that describes the policy. <b>policyDocument</b> must have a minimum length of 1, with a maximum length of 2048, excluding whitespace.</p>
    #[serde(rename = "policyDocument")]
    pub policy_document: String,
    /// <p>The policy name.</p>
    #[serde(rename = "policyName")]
    pub policy_name: String,
}

/// <p>The output from the CreatePolicy operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreatePolicyResponse {
    /// <p>The policy ARN.</p>
    #[serde(rename = "policyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    /// <p>The JSON document that describes the policy.</p>
    #[serde(rename = "policyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    /// <p>The policy name.</p>
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// <p>The policy version ID.</p>
    #[serde(rename = "policyVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version_id: Option<String>,
}

/// <p>The input for the CreatePolicyVersion operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct CreatePolicyVersionRequest {
    /// <p>The JSON document that describes the policy. Minimum length of 1. Maximum length of 2048, excluding whitespace.</p>
    #[serde(rename = "policyDocument")]
    pub policy_document: String,
    /// <p>The policy name.</p>
    #[serde(rename = "policyName")]
    pub policy_name: String,
    /// <p>Specifies whether the policy version is set as the default. When this parameter is true, the new policy version becomes the operative version (that is, the version that is in effect for the certificates to which the policy is attached).</p>
    #[serde(rename = "setAsDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_as_default: Option<bool>,
}

/// <p>The output of the CreatePolicyVersion operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreatePolicyVersionResponse {
    /// <p>Specifies whether the policy version is the default.</p>
    #[serde(rename = "isDefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    /// <p>The policy ARN.</p>
    #[serde(rename = "policyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    /// <p>The JSON document that describes the policy.</p>
    #[serde(rename = "policyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    /// <p>The policy version ID.</p>
    #[serde(rename = "policyVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version_id: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateRoleAliasRequest {
    /// <p>How long (in seconds) the credentials will be valid.</p>
    #[serde(rename = "credentialDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_duration_seconds: Option<i64>,
    /// <p>The role alias that points to a role ARN. This allows you to change the role without having to update the device.</p>
    #[serde(rename = "roleAlias")]
    pub role_alias: String,
    /// <p>The role ARN.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateRoleAliasResponse {
    /// <p>The role alias.</p>
    #[serde(rename = "roleAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_alias: Option<String>,
    /// <p>The role alias ARN.</p>
    #[serde(rename = "roleAliasArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_alias_arn: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateStreamRequest {
    /// <p>A description of the stream.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The files to stream.</p>
    #[serde(rename = "files")]
    pub files: Vec<StreamFile>,
    /// <p>An IAM role that allows the IoT service principal assumes to access your S3 files.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>The stream ID.</p>
    #[serde(rename = "streamId")]
    pub stream_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateStreamResponse {
    /// <p>A description of the stream.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The stream ARN.</p>
    #[serde(rename = "streamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    /// <p>The stream ID.</p>
    #[serde(rename = "streamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    /// <p>The version of the stream.</p>
    #[serde(rename = "streamVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_version: Option<i64>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateThingGroupRequest {
    /// <p>The name of the parent thing group.</p>
    #[serde(rename = "parentGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_name: Option<String>,
    /// <p>The thing group name to create.</p>
    #[serde(rename = "thingGroupName")]
    pub thing_group_name: String,
    /// <p>The thing group properties.</p>
    #[serde(rename = "thingGroupProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_properties: Option<ThingGroupProperties>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateThingGroupResponse {
    /// <p>The thing group ARN.</p>
    #[serde(rename = "thingGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_arn: Option<String>,
    /// <p>The thing group ID.</p>
    #[serde(rename = "thingGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_id: Option<String>,
    /// <p>The thing group name.</p>
    #[serde(rename = "thingGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_name: Option<String>,
}

/// <p>The input for the CreateThing operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateThingRequest {
    /// <p>The attribute payload, which consists of up to three name/value pairs in a JSON document. For example:</p> <p> <code>{\"attributes\":{\"string1\":\"string2\"}}</code> </p>
    #[serde(rename = "attributePayload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_payload: Option<AttributePayload>,
    /// <p>The name of the thing to create.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
    /// <p>The name of the thing type associated with the new thing.</p>
    #[serde(rename = "thingTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<String>,
}

/// <p>The output of the CreateThing operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateThingResponse {
    /// <p>The ARN of the new thing.</p>
    #[serde(rename = "thingArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
    /// <p>The thing ID.</p>
    #[serde(rename = "thingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_id: Option<String>,
    /// <p>The name of the new thing.</p>
    #[serde(rename = "thingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
}

/// <p>The input for the CreateThingType operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateThingTypeRequest {
    /// <p>The name of the thing type.</p>
    #[serde(rename = "thingTypeName")]
    pub thing_type_name: String,
    /// <p>The ThingTypeProperties for the thing type to create. It contains information about the new thing type including a description, and a list of searchable thing attribute names.</p>
    #[serde(rename = "thingTypeProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_properties: Option<ThingTypeProperties>,
}

/// <p>The output of the CreateThingType operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct CreateThingTypeResponse {
    /// <p>The Amazon Resource Name (ARN) of the thing type.</p>
    #[serde(rename = "thingTypeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_arn: Option<String>,
    /// <p>The thing type ID.</p>
    #[serde(rename = "thingTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_id: Option<String>,
    /// <p>The name of the thing type.</p>
    #[serde(rename = "thingTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<String>,
}

/// <p>The input for the CreateTopicRule operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct CreateTopicRuleRequest {
    /// <p>The name of the rule.</p>
    #[serde(rename = "ruleName")]
    pub rule_name: String,
    /// <p>The rule payload.</p>
    #[serde(rename = "topicRulePayload")]
    pub topic_rule_payload: TopicRulePayload,
}

/// <p>Describes a custom method used to code sign a file.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CustomCodeSigning {
    /// <p>The certificate chain.</p>
    #[serde(rename = "certificateChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<CodeSigningCertificateChain>,
    /// <p>The hash algorithm used to code sign the file.</p>
    #[serde(rename = "hashAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_algorithm: Option<String>,
    /// <p>The signature for the file.</p>
    #[serde(rename = "signature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<CodeSigningSignature>,
    /// <p>The signature algorithm used to code sign the file.</p>
    #[serde(rename = "signatureAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_algorithm: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteAuthorizerRequest {
    /// <p>The name of the authorizer to delete.</p>
    #[serde(rename = "authorizerName")]
    pub authorizer_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteAuthorizerResponse;

/// <p>Input for the DeleteCACertificate operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteCACertificateRequest {
    /// <p>The ID of the certificate to delete.</p>
    #[serde(rename = "certificateId")]
    pub certificate_id: String,
}

/// <p>The output for the DeleteCACertificate operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteCACertificateResponse;

/// <p>The input for the DeleteCertificate operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteCertificateRequest {
    /// <p>The ID of the certificate.</p>
    #[serde(rename = "certificateId")]
    pub certificate_id: String,
    /// <p>Forces a certificate request to be deleted.</p>
    #[serde(rename = "forceDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete: Option<bool>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteOTAUpdateRequest {
    /// <p>The OTA update ID to delete.</p>
    #[serde(rename = "otaUpdateId")]
    pub ota_update_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteOTAUpdateResponse;

/// <p>The input for the DeletePolicy operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DeletePolicyRequest {
    /// <p>The name of the policy to delete.</p>
    #[serde(rename = "policyName")]
    pub policy_name: String,
}

/// <p>The input for the DeletePolicyVersion operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DeletePolicyVersionRequest {
    /// <p>The name of the policy.</p>
    #[serde(rename = "policyName")]
    pub policy_name: String,
    /// <p>The policy version ID.</p>
    #[serde(rename = "policyVersionId")]
    pub policy_version_id: String,
}

/// <p>The input for the DeleteRegistrationCode operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteRegistrationCodeRequest;

/// <p>The output for the DeleteRegistrationCode operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteRegistrationCodeResponse;

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteRoleAliasRequest {
    /// <p>The role alias to delete.</p>
    #[serde(rename = "roleAlias")]
    pub role_alias: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteRoleAliasResponse;

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteStreamRequest {
    /// <p>The stream ID.</p>
    #[serde(rename = "streamId")]
    pub stream_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteStreamResponse;

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteThingGroupRequest {
    /// <p>The expected version of the thing group to delete.</p>
    #[serde(rename = "expectedVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_version: Option<i64>,
    /// <p>The name of the thing group to delete.</p>
    #[serde(rename = "thingGroupName")]
    pub thing_group_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteThingGroupResponse;

/// <p>The input for the DeleteThing operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteThingRequest {
    /// <p>The expected version of the thing record in the registry. If the version of the record in the registry does not match the expected version specified in the request, the <code>DeleteThing</code> request is rejected with a <code>VersionConflictException</code>.</p>
    #[serde(rename = "expectedVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_version: Option<i64>,
    /// <p>The name of the thing to delete.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

/// <p>The output of the DeleteThing operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteThingResponse;

/// <p>The input for the DeleteThingType operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteThingTypeRequest {
    /// <p>The name of the thing type.</p>
    #[serde(rename = "thingTypeName")]
    pub thing_type_name: String,
}

/// <p>The output for the DeleteThingType operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeleteThingTypeResponse;

/// <p>The input for the DeleteTopicRule operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteTopicRuleRequest {
    /// <p>The name of the rule.</p>
    #[serde(rename = "ruleName")]
    pub rule_name: String,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DeleteV2LoggingLevelRequest {
    /// <p>The name of the resource for which you are configuring logging.</p>
    #[serde(rename = "targetName")]
    pub target_name: String,
    /// <p>The type of resource for which you are configuring logging. Must be <code>THING_Group</code>.</p>
    #[serde(rename = "targetType")]
    pub target_type: String,
}

/// <p>Contains information that denied the authorization.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Denied {
    /// <p>Information that explicitly denies the authorization. </p>
    #[serde(rename = "explicitDeny")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_deny: Option<ExplicitDeny>,
    /// <p>Information that implicitly denies the authorization. When a policy doesn't explicitly deny or allow an action on a resource it is considered an implicit deny.</p>
    #[serde(rename = "implicitDeny")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_deny: Option<ImplicitDeny>,
}

/// <p>The input for the DeprecateThingType operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DeprecateThingTypeRequest {
    /// <p>The name of the thing type to deprecate.</p>
    #[serde(rename = "thingTypeName")]
    pub thing_type_name: String,
    /// <p>Whether to undeprecate a deprecated thing type. If <b>true</b>, the thing type will not be deprecated anymore and you can associate it with things.</p>
    #[serde(rename = "undoDeprecate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undo_deprecate: Option<bool>,
}

/// <p>The output for the DeprecateThingType operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeprecateThingTypeResponse;

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeAuthorizerRequest {
    /// <p>The name of the authorizer to describe.</p>
    #[serde(rename = "authorizerName")]
    pub authorizer_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeAuthorizerResponse {
    /// <p>The authorizer description.</p>
    #[serde(rename = "authorizerDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_description: Option<AuthorizerDescription>,
}

/// <p>The input for the DescribeCACertificate operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeCACertificateRequest {
    /// <p>The CA certificate identifier.</p>
    #[serde(rename = "certificateId")]
    pub certificate_id: String,
}

/// <p>The output from the DescribeCACertificate operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeCACertificateResponse {
    /// <p>The CA certificate description.</p>
    #[serde(rename = "certificateDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_description: Option<CACertificateDescription>,
    /// <p>Information about the registration configuration.</p>
    #[serde(rename = "registrationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_config: Option<RegistrationConfig>,
}

/// <p>The input for the DescribeCertificate operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeCertificateRequest {
    /// <p>The ID of the certificate.</p>
    #[serde(rename = "certificateId")]
    pub certificate_id: String,
}

/// <p>The output of the DescribeCertificate operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeCertificateResponse {
    /// <p>The description of the certificate.</p>
    #[serde(rename = "certificateDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_description: Option<CertificateDescription>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeDefaultAuthorizerRequest;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeDefaultAuthorizerResponse {
    /// <p>The default authorizer's description.</p>
    #[serde(rename = "authorizerDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_description: Option<AuthorizerDescription>,
}

/// <p>The input for the DescribeEndpoint operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeEndpointRequest {
    /// <p>The endpoint type.</p>
    #[serde(rename = "endpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
}

/// <p>The output from the DescribeEndpoint operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeEndpointResponse {
    /// <p>The endpoint. The format of the endpoint is as follows: <i>identifier</i>.iot.<i>region</i>.amazonaws.com.</p>
    #[serde(rename = "endpointAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_address: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeEventConfigurationsRequest;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeEventConfigurationsResponse {
    /// <p>The creation date of the event configuration.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The event configurations.</p>
    #[serde(rename = "eventConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_configurations: Option<::std::collections::HashMap<String, Configuration>>,
    /// <p>The date the event configurations were last modified.</p>
    #[serde(rename = "lastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeIndexRequest {
    /// <p>The index name.</p>
    #[serde(rename = "indexName")]
    pub index_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeIndexResponse {
    /// <p>The index name.</p>
    #[serde(rename = "indexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    /// <p>The index status.</p>
    #[serde(rename = "indexStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status: Option<String>,
    /// <p>Contains a value that specifies the type of indexing performed. Valid values are:</p> <ol> <li> <p>REGISTRY – Your thing index will contain only registry data.</p> </li> <li> <p>REGISTRY_AND_SHADOW - Your thing index will contain registry and shadow data.</p> </li> </ol>
    #[serde(rename = "schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeJobExecutionRequest {
    /// <p>A string (consisting of the digits "0" through "9" which is used to specify a particular job execution on a particular device.</p>
    #[serde(rename = "executionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_number: Option<i64>,
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>The name of the thing on which the job execution is running.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeJobExecutionResponse {
    /// <p>Information about the job execution.</p>
    #[serde(rename = "execution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<JobExecution>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeJobRequest {
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeJobResponse {
    /// <p>An S3 link to the job document.</p>
    #[serde(rename = "documentSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_source: Option<String>,
    /// <p>Information about the job.</p>
    #[serde(rename = "job")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeRoleAliasRequest {
    /// <p>The role alias to describe.</p>
    #[serde(rename = "roleAlias")]
    pub role_alias: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeRoleAliasResponse {
    /// <p>The role alias description.</p>
    #[serde(rename = "roleAliasDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_alias_description: Option<RoleAliasDescription>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeStreamRequest {
    /// <p>The stream ID.</p>
    #[serde(rename = "streamId")]
    pub stream_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeStreamResponse {
    /// <p>Information about the stream.</p>
    #[serde(rename = "streamInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_info: Option<StreamInfo>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeThingGroupRequest {
    /// <p>The name of the thing group.</p>
    #[serde(rename = "thingGroupName")]
    pub thing_group_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeThingGroupResponse {
    /// <p>The thing group ARN.</p>
    #[serde(rename = "thingGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_arn: Option<String>,
    /// <p>The thing group ID.</p>
    #[serde(rename = "thingGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_id: Option<String>,
    /// <p>Thing group metadata.</p>
    #[serde(rename = "thingGroupMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_metadata: Option<ThingGroupMetadata>,
    /// <p>The name of the thing group.</p>
    #[serde(rename = "thingGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_name: Option<String>,
    /// <p>The thing group properties.</p>
    #[serde(rename = "thingGroupProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_properties: Option<ThingGroupProperties>,
    /// <p>The version of the thing group.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeThingRegistrationTaskRequest {
    /// <p>The task ID.</p>
    #[serde(rename = "taskId")]
    pub task_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeThingRegistrationTaskResponse {
    /// <p>The task creation date.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The number of things that failed to be provisioned.</p>
    #[serde(rename = "failureCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_count: Option<i64>,
    /// <p>The S3 bucket that contains the input file.</p>
    #[serde(rename = "inputFileBucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_file_bucket: Option<String>,
    /// <p>The input file key.</p>
    #[serde(rename = "inputFileKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_file_key: Option<String>,
    /// <p>The date when the task was last modified.</p>
    #[serde(rename = "lastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>The message.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The progress of the bulk provisioning task expressed as a percentage.</p>
    #[serde(rename = "percentageProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_progress: Option<i64>,
    /// <p>The role ARN that grants access to the input file bucket.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The status of the bulk thing provisioning task.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The number of things successfully provisioned.</p>
    #[serde(rename = "successCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_count: Option<i64>,
    /// <p>The task ID.</p>
    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// <p>The task's template.</p>
    #[serde(rename = "templateBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
}

/// <p>The input for the DescribeThing operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeThingRequest {
    /// <p>The name of the thing.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

/// <p>The output from the DescribeThing operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeThingResponse {
    /// <p>The thing attributes.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The default client ID.</p>
    #[serde(rename = "defaultClientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_client_id: Option<String>,
    /// <p>The ARN of the thing to describe.</p>
    #[serde(rename = "thingArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
    /// <p>The ID of the thing to describe.</p>
    #[serde(rename = "thingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_id: Option<String>,
    /// <p>The name of the thing.</p>
    #[serde(rename = "thingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
    /// <p>The thing type name.</p>
    #[serde(rename = "thingTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<String>,
    /// <p>The current version of the thing record in the registry.</p> <note> <p>To avoid unintentional changes to the information in the registry, you can pass the version information in the <code>expectedVersion</code> parameter of the <code>UpdateThing</code> and <code>DeleteThing</code> calls.</p> </note>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>The input for the DescribeThingType operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DescribeThingTypeRequest {
    /// <p>The name of the thing type.</p>
    #[serde(rename = "thingTypeName")]
    pub thing_type_name: String,
}

/// <p>The output for the DescribeThingType operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DescribeThingTypeResponse {
    /// <p>The thing type ARN.</p>
    #[serde(rename = "thingTypeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_arn: Option<String>,
    /// <p>The thing type ID.</p>
    #[serde(rename = "thingTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_id: Option<String>,
    /// <p>The ThingTypeMetadata contains additional information about the thing type including: creation date and time, a value indicating whether the thing type is deprecated, and a date and time when it was deprecated.</p>
    #[serde(rename = "thingTypeMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_metadata: Option<ThingTypeMetadata>,
    /// <p>The name of the thing type.</p>
    #[serde(rename = "thingTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<String>,
    /// <p>The ThingTypeProperties contains information about the thing type including description, and a list of searchable thing attribute names.</p>
    #[serde(rename = "thingTypeProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_properties: Option<ThingTypeProperties>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct DetachPolicyRequest {
    /// <p>The policy to detach.</p>
    #[serde(rename = "policyName")]
    pub policy_name: String,
    /// <p>The target from which the policy will be detached.</p>
    #[serde(rename = "target")]
    pub target: String,
}

/// <p>The input for the DetachPrincipalPolicy operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DetachPrincipalPolicyRequest {
    /// <p>The name of the policy to detach.</p>
    #[serde(rename = "policyName")]
    pub policy_name: String,
    /// <p>The principal.</p> <p>If the principal is a certificate, specify the certificate ARN. If the principal is an Amazon Cognito identity, specify the identity ID.</p>
    #[serde(rename = "principal")]
    pub principal: String,
}

/// <p>The input for the DetachThingPrincipal operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DetachThingPrincipalRequest {
    /// <p>If the principal is a certificate, this value must be ARN of the certificate. If the principal is an Amazon Cognito identity, this value must be the ID of the Amazon Cognito identity.</p>
    #[serde(rename = "principal")]
    pub principal: String,
    /// <p>The name of the thing.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

/// <p>The output from the DetachThingPrincipal operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct DetachThingPrincipalResponse;

/// <p>The input for the DisableTopicRuleRequest operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct DisableTopicRuleRequest {
    /// <p>The name of the rule to disable.</p>
    #[serde(rename = "ruleName")]
    pub rule_name: String,
}

/// <p>Describes an action to write to a DynamoDB table.</p> <p>The <code>tableName</code>, <code>hashKeyField</code>, and <code>rangeKeyField</code> values must match the values used when you created the table.</p> <p>The <code>hashKeyValue</code> and <code>rangeKeyvalue</code> fields use a substitution template syntax. These templates provide data at runtime. The syntax is as follows: ${<i>sql-expression</i>}.</p> <p>You can specify any valid expression in a WHERE or SELECT clause, including JSON properties, comparisons, calculations, and functions. For example, the following field uses the third level of the topic:</p> <p> <code>"hashKeyValue": "${topic(3)}"</code> </p> <p>The following field uses the timestamp:</p> <p> <code>"rangeKeyValue": "${timestamp()}"</code> </p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DynamoDBAction {
    /// <p>The hash key name.</p>
    #[serde(rename = "hashKeyField")]
    pub hash_key_field: String,
    /// <p>The hash key type. Valid values are "STRING" or "NUMBER"</p>
    #[serde(rename = "hashKeyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_key_type: Option<String>,
    /// <p>The hash key value.</p>
    #[serde(rename = "hashKeyValue")]
    pub hash_key_value: String,
    /// <p>The type of operation to be performed. This follows the substitution template, so it can be <code>${operation}</code>, but the substitution must result in one of the following: <code>INSERT</code>, <code>UPDATE</code>, or <code>DELETE</code>.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    /// <p>The action payload. This name can be customized.</p>
    #[serde(rename = "payloadField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_field: Option<String>,
    /// <p>The range key name.</p>
    #[serde(rename = "rangeKeyField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_key_field: Option<String>,
    /// <p>The range key type. Valid values are "STRING" or "NUMBER"</p>
    #[serde(rename = "rangeKeyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_key_type: Option<String>,
    /// <p>The range key value.</p>
    #[serde(rename = "rangeKeyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_key_value: Option<String>,
    /// <p>The ARN of the IAM role that grants access to the DynamoDB table.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>The name of the DynamoDB table.</p>
    #[serde(rename = "tableName")]
    pub table_name: String,
}

/// <p>Describes an action to write to a DynamoDB table.</p> <p>This DynamoDB action writes each attribute in the message payload into it's own column in the DynamoDB table.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DynamoDBv2Action {
    /// <p>Specifies the DynamoDB table to which the message data will be written. For example:</p> <p> <code>{ "dynamoDBv2": { "roleArn": "aws:iam:12341251:my-role" "putItem": { "tableName": "my-table" } } }</code> </p> <p>Each attribute in the message payload will be written to a separate column in the DynamoDB database.</p>
    #[serde(rename = "putItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put_item: Option<PutItemInput>,
    /// <p>The ARN of the IAM role that grants access to the DynamoDB table.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>The policy that has the effect on the authorization results.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct EffectivePolicy {
    /// <p>The policy ARN.</p>
    #[serde(rename = "policyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    /// <p>The IAM policy document.</p>
    #[serde(rename = "policyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    /// <p>The policy name.</p>
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

/// <p>Describes an action that writes data to an Amazon Elasticsearch Service domain.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ElasticsearchAction {
    /// <p>The endpoint of your Elasticsearch domain.</p>
    #[serde(rename = "endpoint")]
    pub endpoint: String,
    /// <p>The unique identifier for the document you are storing.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The Elasticsearch index where you want to store your data.</p>
    #[serde(rename = "index")]
    pub index: String,
    /// <p>The IAM role ARN that has access to Elasticsearch.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>The type of document you are storing.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>The input for the EnableTopicRuleRequest operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct EnableTopicRuleRequest {
    /// <p>The name of the topic rule to enable.</p>
    #[serde(rename = "ruleName")]
    pub rule_name: String,
}

/// <p>Error information.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ErrorInfo {
    /// <p>The error code.</p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>The error message.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>Information that explicitly denies authorization.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ExplicitDeny {
    /// <p>The policies that denied the authorization.</p>
    #[serde(rename = "policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Policy>>,
}

/// <p>Describes an action that writes data to an Amazon Kinesis Firehose stream.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FirehoseAction {
    /// <p>The delivery stream name.</p>
    #[serde(rename = "deliveryStreamName")]
    pub delivery_stream_name: String,
    /// <p>The IAM role that grants access to the Amazon Kinesis Firehose stream.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>A character separator that will be used to separate records written to the Firehose stream. Valid values are: '\n' (newline), '\t' (tab), '\r\n' (Windows newline), ',' (comma).</p>
    #[serde(rename = "separator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetEffectivePoliciesRequest {
    /// <p>The Cognito identity pool ID.</p>
    #[serde(rename = "cognitoIdentityPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_identity_pool_id: Option<String>,
    /// <p>The principal.</p>
    #[serde(rename = "principal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    /// <p>The thing name.</p>
    #[serde(rename = "thingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetEffectivePoliciesResponse {
    /// <p>The effective policies.</p>
    #[serde(rename = "effectivePolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_policies: Option<Vec<EffectivePolicy>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetIndexingConfigurationRequest;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetIndexingConfigurationResponse {
    /// <p>Thing indexing configuration.</p>
    #[serde(rename = "thingIndexingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_indexing_configuration: Option<ThingIndexingConfiguration>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetJobDocumentRequest {
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetJobDocumentResponse {
    /// <p>The job document content.</p>
    #[serde(rename = "document")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
}

/// <p>The input for the GetLoggingOptions operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct GetLoggingOptionsRequest;

/// <p>The output from the GetLoggingOptions operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetLoggingOptionsResponse {
    /// <p>The logging level.</p>
    #[serde(rename = "logLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    /// <p>The ARN of the IAM role that grants access.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetOTAUpdateRequest {
    /// <p>The OTA update ID.</p>
    #[serde(rename = "otaUpdateId")]
    pub ota_update_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetOTAUpdateResponse {
    /// <p>The OTA update info.</p>
    #[serde(rename = "otaUpdateInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_info: Option<OTAUpdateInfo>,
}

/// <p>The input for the GetPolicy operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct GetPolicyRequest {
    /// <p>The name of the policy.</p>
    #[serde(rename = "policyName")]
    pub policy_name: String,
}

/// <p>The output from the GetPolicy operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetPolicyResponse {
    /// <p>The default policy version ID.</p>
    #[serde(rename = "defaultVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_id: Option<String>,
    /// <p>The policy ARN.</p>
    #[serde(rename = "policyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    /// <p>The JSON document that describes the policy.</p>
    #[serde(rename = "policyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    /// <p>The policy name.</p>
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

/// <p>The input for the GetPolicyVersion operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct GetPolicyVersionRequest {
    /// <p>The name of the policy.</p>
    #[serde(rename = "policyName")]
    pub policy_name: String,
    /// <p>The policy version ID.</p>
    #[serde(rename = "policyVersionId")]
    pub policy_version_id: String,
}

/// <p>The output from the GetPolicyVersion operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetPolicyVersionResponse {
    /// <p>Specifies whether the policy version is the default.</p>
    #[serde(rename = "isDefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    /// <p>The policy ARN.</p>
    #[serde(rename = "policyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    /// <p>The JSON document that describes the policy.</p>
    #[serde(rename = "policyDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    /// <p>The policy name.</p>
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// <p>The policy version ID.</p>
    #[serde(rename = "policyVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version_id: Option<String>,
}

/// <p>The input to the GetRegistrationCode operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct GetRegistrationCodeRequest;

/// <p>The output from the GetRegistrationCode operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetRegistrationCodeResponse {
    /// <p>The CA certificate registration code.</p>
    #[serde(rename = "registrationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_code: Option<String>,
}

/// <p>The input for the GetTopicRule operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct GetTopicRuleRequest {
    /// <p>The name of the rule.</p>
    #[serde(rename = "ruleName")]
    pub rule_name: String,
}

/// <p>The output from the GetTopicRule operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetTopicRuleResponse {
    /// <p>The rule.</p>
    #[serde(rename = "rule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<TopicRule>,
    /// <p>The rule ARN.</p>
    #[serde(rename = "ruleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct GetV2LoggingOptionsRequest;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct GetV2LoggingOptionsResponse {
    /// <p>The default log level.</p>
    #[serde(rename = "defaultLogLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_log_level: Option<String>,
    /// <p>Disables all logs.</p>
    #[serde(rename = "disableAllLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_all_logs: Option<bool>,
    /// <p>The IAM role ARN AWS IoT uses to write to your CloudWatch logs.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>The name and ARN of a group.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct GroupNameAndArn {
    /// <p>The group ARN.</p>
    #[serde(rename = "groupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_arn: Option<String>,
    /// <p>The group name.</p>
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

/// <p>Information that implicitly denies authorization. When policy doesn't explicitly deny or allow an action on a resource it is considered an implicit deny.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ImplicitDeny {
    /// <p>Policies that don't contain a matching allow or deny statement for the specified action on the specified resource. </p>
    #[serde(rename = "policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Policy>>,
}

/// <p>The <code>Job</code> object contains details about a job.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Job {
    /// <p>If the job was updated, describes the reason for the update.</p>
    #[serde(rename = "comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the job was completed.</p>
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<f64>,
    /// <p>The time, in milliseconds since the epoch, when the job was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>A short text description of the job.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The parameters specified for the job document.</p>
    #[serde(rename = "documentParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>An ARN identifying the job with format "arn:aws:iot:region:account:job/jobId".</p>
    #[serde(rename = "jobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    /// <p>Allows you to create a staged rollout of a job.</p>
    #[serde(rename = "jobExecutionsRolloutConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_executions_rollout_config: Option<JobExecutionsRolloutConfig>,
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>Details about the job process.</p>
    #[serde(rename = "jobProcessDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_process_details: Option<JobProcessDetails>,
    /// <p>The time, in milliseconds since the epoch, when the job was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>Configuration for pre-signed S3 URLs.</p>
    #[serde(rename = "presignedUrlConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presigned_url_config: Option<PresignedUrlConfig>,
    /// <p>The status of the job, one of <code>IN_PROGRESS</code>, <code>CANCELED</code>, or <code>COMPLETED</code>. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Specifies whether the job will continue to run (CONTINUOUS), or will be complete after all those things specified as targets have completed the job (SNAPSHOT). If continuous, the job may also be run on a thing when a change is detected in a target. For example, a job will run on a device when the thing representing the device is added to a target group, even after the job was completed by all things originally in the group. </p>
    #[serde(rename = "targetSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_selection: Option<String>,
    /// <p>A list of IoT things and thing groups to which the job should be sent.</p>
    #[serde(rename = "targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
}

/// <p>The job execution object represents the execution of a job on a particular device.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct JobExecution {
    /// <p>A string (consisting of the digits "0" through "9") which identifies this particular job execution on this particular device. It can be used in commands which return or update job execution information. </p>
    #[serde(rename = "executionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_number: Option<i64>,
    /// <p>The unique identifier you assigned to the job when it was created.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the job execution was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The time, in milliseconds since the epoch, when the job execution was queued.</p>
    #[serde(rename = "queuedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_at: Option<f64>,
    /// <p>The time, in milliseconds since the epoch, when the job execution started.</p>
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    /// <p>The status of the job execution (IN_PROGRESS, QUEUED, FAILED, SUCCESS, CANCELED, or REJECTED).</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A collection of name/value pairs that describe the status of the job execution.</p>
    #[serde(rename = "statusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<JobExecutionStatusDetails>,
    /// <p>The ARN of the thing on which the job execution is running.</p>
    #[serde(rename = "thingArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
}

/// <p>Details of the job execution status.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct JobExecutionStatusDetails {
    /// <p>The job execution status.</p>
    #[serde(rename = "detailsMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_map: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The job execution summary.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct JobExecutionSummary {
    /// <p>A string (consisting of the digits "0" through "9") which identifies this particular job execution on this particular device. It can be used later in commands which return or update job execution information.</p>
    #[serde(rename = "executionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_number: Option<i64>,
    /// <p>The time, in milliseconds since the epoch, when the job execution was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The time, in milliseconds since the epoch, when the job execution was queued.</p>
    #[serde(rename = "queuedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_at: Option<f64>,
    /// <p>The time, in milliseconds since the epoch, when the job execution started.</p>
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    /// <p>The status of the job execution.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Contains a summary of information about job executions for a specific job.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct JobExecutionSummaryForJob {
    /// <p>Contains a subset of information about a job execution.</p>
    #[serde(rename = "jobExecutionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_execution_summary: Option<JobExecutionSummary>,
    /// <p>The ARN of the thing on which the job execution is running.</p>
    #[serde(rename = "thingArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
}

/// <p>The job execution summary for a thing.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct JobExecutionSummaryForThing {
    /// <p>Contains a subset of information about a job execution.</p>
    #[serde(rename = "jobExecutionSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_execution_summary: Option<JobExecutionSummary>,
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

/// <p>Allows you to create a staged rollout of a job.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct JobExecutionsRolloutConfig {
    /// <p>The maximum number of things that will be notified of a pending job, per minute. This parameter allows you to create a staged rollout.</p>
    #[serde(rename = "maximumPerMinute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_per_minute: Option<i64>,
}

/// <p>The job process details.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct JobProcessDetails {
    /// <p>The number of things that cancelled the job.</p>
    #[serde(rename = "numberOfCanceledThings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_canceled_things: Option<i64>,
    /// <p>The number of things that failed executing the job.</p>
    #[serde(rename = "numberOfFailedThings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_failed_things: Option<i64>,
    /// <p>The number of things currently executing the job.</p>
    #[serde(rename = "numberOfInProgressThings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_in_progress_things: Option<i64>,
    /// <p>The number of things that are awaiting execution of the job.</p>
    #[serde(rename = "numberOfQueuedThings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_queued_things: Option<i64>,
    /// <p>The number of things that rejected the job.</p>
    #[serde(rename = "numberOfRejectedThings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_rejected_things: Option<i64>,
    /// <p>The number of things that are no longer scheduled to execute the job because they have been deleted or have been removed from the group that was a target of the job.</p>
    #[serde(rename = "numberOfRemovedThings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_removed_things: Option<i64>,
    /// <p>The number of things which successfully completed the job.</p>
    #[serde(rename = "numberOfSucceededThings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_succeeded_things: Option<i64>,
    /// <p>The devices on which the job is executing.</p>
    #[serde(rename = "processingTargets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_targets: Option<Vec<String>>,
}

/// <p>The job summary.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct JobSummary {
    /// <p>The time, in milliseconds since the epoch, when the job completed.</p>
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<f64>,
    /// <p>The time, in milliseconds since the epoch, when the job was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The job ARN.</p>
    #[serde(rename = "jobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// <p>The time, in milliseconds since the epoch, when the job was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>The job summary status.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Specifies whether the job will continue to run (CONTINUOUS), or will be complete after all those things specified as targets have completed the job (SNAPSHOT). If continuous, the job may also be run on a thing when a change is detected in a target. For example, a job will run on a thing when the thing is added to a target group, even after the job was completed by all things originally in the group.</p>
    #[serde(rename = "targetSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_selection: Option<String>,
    /// <p>The ID of the thing group.</p>
    #[serde(rename = "thingGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_id: Option<String>,
}

/// <p>Describes a key pair.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct KeyPair {
    /// <p>The private key.</p>
    #[serde(rename = "PrivateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// <p>The public key.</p>
    #[serde(rename = "PublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}

/// <p>Describes an action to write data to an Amazon Kinesis stream.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct KinesisAction {
    /// <p>The partition key.</p>
    #[serde(rename = "partitionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<String>,
    /// <p>The ARN of the IAM role that grants access to the Amazon Kinesis stream.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>The name of the Amazon Kinesis stream.</p>
    #[serde(rename = "streamName")]
    pub stream_name: String,
}

/// <p>Describes an action to invoke a Lambda function.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LambdaAction {
    /// <p>The ARN of the Lambda function.</p>
    #[serde(rename = "functionArn")]
    pub function_arn: String,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListAttachedPoliciesRequest {
    /// <p>The token to retrieve the next set of results.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>When true, recursively list attached policies.</p>
    #[serde(rename = "recursive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
    /// <p>The group for which the policies will be listed.</p>
    #[serde(rename = "target")]
    pub target: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListAttachedPoliciesResponse {
    /// <p>The token to retrieve the next set of results, or ``null`` if there are no more results.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>The policies.</p>
    #[serde(rename = "policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Policy>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListAuthorizersRequest {
    /// <p>Return the list of authorizers in ascending alphabetical order.</p>
    #[serde(rename = "ascendingOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    /// <p>A marker used to get the next set of results.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The maximum number of results to return at one time.</p>
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The status of the list authorizers request.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListAuthorizersResponse {
    /// <p>The authorizers.</p>
    #[serde(rename = "authorizers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizers: Option<Vec<AuthorizerSummary>>,
    /// <p>A marker used to get the next set of results.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

/// <p>Input for the ListCACertificates operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct ListCACertificatesRequest {
    /// <p>Determines the order of the results.</p>
    #[serde(rename = "ascendingOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    /// <p>The marker for the next set of results.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The result page size.</p>
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// <p>The output from the ListCACertificates operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListCACertificatesResponse {
    /// <p>The CA certificates registered in your AWS account.</p>
    #[serde(rename = "certificates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<CACertificate>>,
    /// <p>The current position within the list of CA certificates.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

/// <p>The input to the ListCertificatesByCA operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct ListCertificatesByCARequest {
    /// <p>Specifies the order for results. If True, the results are returned in ascending order, based on the creation date.</p>
    #[serde(rename = "ascendingOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    /// <p>The ID of the CA certificate. This operation will list all registered device certificate that were signed by this CA certificate.</p>
    #[serde(rename = "caCertificateId")]
    pub ca_certificate_id: String,
    /// <p>The marker for the next set of results.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The result page size.</p>
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// <p>The output of the ListCertificatesByCA operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListCertificatesByCAResponse {
    /// <p>The device certificates signed by the specified CA certificate.</p>
    #[serde(rename = "certificates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<Certificate>>,
    /// <p>The marker for the next set of results, or null if there are no additional results.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

/// <p>The input for the ListCertificates operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct ListCertificatesRequest {
    /// <p>Specifies the order for results. If True, the results are returned in ascending order, based on the creation date.</p>
    #[serde(rename = "ascendingOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    /// <p>The marker for the next set of results.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The result page size.</p>
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// <p>The output of the ListCertificates operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListCertificatesResponse {
    /// <p>The descriptions of the certificates.</p>
    #[serde(rename = "certificates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<Certificate>>,
    /// <p>The marker for the next set of results, or null if there are no additional results.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListIndicesRequest {
    /// <p>The maximum number of results to return at one time.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token used to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListIndicesResponse {
    /// <p>The index names.</p>
    #[serde(rename = "indexNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_names: Option<Vec<String>>,
    /// <p>The token used to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListJobExecutionsForJobRequest {
    /// <p>The unique identifier you assigned to this job when it was created.</p>
    #[serde(rename = "jobId")]
    pub job_id: String,
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The status of the job.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListJobExecutionsForJobResponse {
    /// <p>A list of job execution summaries.</p>
    #[serde(rename = "executionSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_summaries: Option<Vec<JobExecutionSummaryForJob>>,
    /// <p>The token for the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListJobExecutionsForThingRequest {
    /// <p>The maximum number of results to be returned per request.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An optional filter that lets you search for jobs that have the specified status.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The thing name.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListJobExecutionsForThingResponse {
    /// <p>A list of job execution summaries.</p>
    #[serde(rename = "executionSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_summaries: Option<Vec<JobExecutionSummaryForThing>>,
    /// <p>The token for the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListJobsRequest {
    /// <p>The maximum number of results to return per request.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An optional filter that lets you search for jobs that have the specified status.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Specifies whether the job will continue to run (CONTINUOUS), or will be complete after all those things specified as targets have completed the job (SNAPSHOT). If continuous, the job may also be run on a thing when a change is detected in a target. For example, a job will run on a thing when the thing is added to a target group, even after the job was completed by all things originally in the group. </p>
    #[serde(rename = "targetSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_selection: Option<String>,
    /// <p>A filter that limits the returned jobs to those for the specified group.</p>
    #[serde(rename = "thingGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_id: Option<String>,
    /// <p>A filter that limits the returned jobs to those for the specified group.</p>
    #[serde(rename = "thingGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_name: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListJobsResponse {
    /// <p>A list of jobs.</p>
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<JobSummary>>,
    /// <p>The token for the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListOTAUpdatesRequest {
    /// <p>The maximum number of results to return at one time.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token used to retreive the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The OTA update job status.</p>
    #[serde(rename = "otaUpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_status: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListOTAUpdatesResponse {
    /// <p>A token to use to get the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of OTA update jobs.</p>
    #[serde(rename = "otaUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_updates: Option<Vec<OTAUpdateSummary>>,
}

/// <p>The input to the ListOutgoingCertificates operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct ListOutgoingCertificatesRequest {
    /// <p>Specifies the order for results. If True, the results are returned in ascending order, based on the creation date.</p>
    #[serde(rename = "ascendingOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    /// <p>The marker for the next set of results.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The result page size.</p>
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// <p>The output from the ListOutgoingCertificates operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListOutgoingCertificatesResponse {
    /// <p>The marker for the next set of results.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>The certificates that are being transferred but not yet accepted.</p>
    #[serde(rename = "outgoingCertificates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outgoing_certificates: Option<Vec<OutgoingCertificate>>,
}

/// <p>The input for the ListPolicies operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct ListPoliciesRequest {
    /// <p>Specifies the order for results. If true, the results are returned in ascending creation order.</p>
    #[serde(rename = "ascendingOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    /// <p>The marker for the next set of results.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The result page size.</p>
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

/// <p>The output from the ListPolicies operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListPoliciesResponse {
    /// <p>The marker for the next set of results, or null if there are no additional results.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>The descriptions of the policies.</p>
    #[serde(rename = "policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Policy>>,
}

/// <p>The input for the ListPolicyPrincipals operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct ListPolicyPrincipalsRequest {
    /// <p>Specifies the order for results. If true, the results are returned in ascending creation order.</p>
    #[serde(rename = "ascendingOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    /// <p>The marker for the next set of results.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The result page size.</p>
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The policy name.</p>
    #[serde(rename = "policyName")]
    pub policy_name: String,
}

/// <p>The output from the ListPolicyPrincipals operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListPolicyPrincipalsResponse {
    /// <p>The marker for the next set of results, or null if there are no additional results.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>The descriptions of the principals.</p>
    #[serde(rename = "principals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<String>>,
}

/// <p>The input for the ListPolicyVersions operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct ListPolicyVersionsRequest {
    /// <p>The policy name.</p>
    #[serde(rename = "policyName")]
    pub policy_name: String,
}

/// <p>The output from the ListPolicyVersions operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListPolicyVersionsResponse {
    /// <p>The policy versions.</p>
    #[serde(rename = "policyVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_versions: Option<Vec<PolicyVersion>>,
}

/// <p>The input for the ListPrincipalPolicies operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct ListPrincipalPoliciesRequest {
    /// <p>Specifies the order for results. If true, results are returned in ascending creation order.</p>
    #[serde(rename = "ascendingOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    /// <p>The marker for the next set of results.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The result page size.</p>
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The principal.</p>
    #[serde(rename = "principal")]
    pub principal: String,
}

/// <p>The output from the ListPrincipalPolicies operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListPrincipalPoliciesResponse {
    /// <p>The marker for the next set of results, or null if there are no additional results.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>The policies.</p>
    #[serde(rename = "policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Policy>>,
}

/// <p>The input for the ListPrincipalThings operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct ListPrincipalThingsRequest {
    /// <p>The maximum number of results to return in this operation.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token used to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The principal.</p>
    #[serde(rename = "principal")]
    pub principal: String,
}

/// <p>The output from the ListPrincipalThings operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListPrincipalThingsResponse {
    /// <p>The token used to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The things.</p>
    #[serde(rename = "things")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub things: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListRoleAliasesRequest {
    /// <p>Return the list of role aliases in ascending alphabetical order.</p>
    #[serde(rename = "ascendingOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    /// <p>A marker used to get the next set of results.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The maximum number of results to return at one time.</p>
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListRoleAliasesResponse {
    /// <p>A marker used to get the next set of results.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>The role aliases.</p>
    #[serde(rename = "roleAliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_aliases: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListStreamsRequest {
    /// <p>Set to true to return the list of streams in ascending order.</p>
    #[serde(rename = "ascendingOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending_order: Option<bool>,
    /// <p>The maximum number of results to return at a time.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token used to get the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListStreamsResponse {
    /// <p>A token used to get the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of streams.</p>
    #[serde(rename = "streams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streams: Option<Vec<StreamSummary>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListTargetsForPolicyRequest {
    /// <p>A marker used to get the next set of results.</p>
    #[serde(rename = "marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The maximum number of results to return at one time.</p>
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// <p>The policy name.</p>
    #[serde(rename = "policyName")]
    pub policy_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListTargetsForPolicyResponse {
    /// <p>A marker used to get the next set of results.</p>
    #[serde(rename = "nextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>The policy targets.</p>
    #[serde(rename = "targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListThingGroupsForThingRequest {
    /// <p>The maximum number of results to return at one time.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token used to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The thing name.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListThingGroupsForThingResponse {
    /// <p>The token used to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The thing groups.</p>
    #[serde(rename = "thingGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_groups: Option<Vec<GroupNameAndArn>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListThingGroupsRequest {
    /// <p>The maximum number of results to return at one time.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A filter that limits the results to those with the specified name prefix.</p>
    #[serde(rename = "namePrefixFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix_filter: Option<String>,
    /// <p>The token used to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A filter that limits the results to those with the specified parent group.</p>
    #[serde(rename = "parentGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group: Option<String>,
    /// <p>If true, return child groups as well.</p>
    #[serde(rename = "recursive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListThingGroupsResponse {
    /// <p>The token used to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The thing groups.</p>
    #[serde(rename = "thingGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_groups: Option<Vec<GroupNameAndArn>>,
}

/// <p>The input for the ListThingPrincipal operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct ListThingPrincipalsRequest {
    /// <p>The name of the thing.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

/// <p>The output from the ListThingPrincipals operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListThingPrincipalsResponse {
    /// <p>The principals associated with the thing.</p>
    #[serde(rename = "principals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListThingRegistrationTaskReportsRequest {
    /// <p>The maximum number of results to return per request.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of task report.</p>
    #[serde(rename = "reportType")]
    pub report_type: String,
    /// <p>The id of the task.</p>
    #[serde(rename = "taskId")]
    pub task_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListThingRegistrationTaskReportsResponse {
    /// <p>The token to retrieve the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of task report.</p>
    #[serde(rename = "reportType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_type: Option<String>,
    /// <p>Links to the task resources.</p>
    #[serde(rename = "resourceLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_links: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListThingRegistrationTasksRequest {
    /// <p>The maximum number of results to return at one time.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token used to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The status of the bulk thing provisioning task.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListThingRegistrationTasksResponse {
    /// <p>The token used to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of bulk thing provisioning task IDs.</p>
    #[serde(rename = "taskIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_ids: Option<Vec<String>>,
}

/// <p>The input for the ListThingTypes operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct ListThingTypesRequest {
    /// <p>The maximum number of results to return in this operation.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the thing type.</p>
    #[serde(rename = "thingTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<String>,
}

/// <p>The output for the ListThingTypes operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListThingTypesResponse {
    /// <p>The token for the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The thing types.</p>
    #[serde(rename = "thingTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_types: Option<Vec<ThingTypeDefinition>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListThingsInThingGroupRequest {
    /// <p>The maximum number of results to return at one time.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token used to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>When true, list things in this thing group and in all child groups as well.</p>
    #[serde(rename = "recursive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
    /// <p>The thing group name.</p>
    #[serde(rename = "thingGroupName")]
    pub thing_group_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListThingsInThingGroupResponse {
    /// <p>The token used to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The things in the specified thing group.</p>
    #[serde(rename = "things")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub things: Option<Vec<String>>,
}

/// <p>The input for the ListThings operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct ListThingsRequest {
    /// <p>The attribute name used to search for things.</p>
    #[serde(rename = "attributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    /// <p>The attribute value used to search for things.</p>
    #[serde(rename = "attributeValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<String>,
    /// <p>The maximum number of results to return in this operation.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token used to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the thing type used to search for things.</p>
    #[serde(rename = "thingTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<String>,
}

/// <p>The output from the ListThings operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListThingsResponse {
    /// <p>The token used to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The things.</p>
    #[serde(rename = "things")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub things: Option<Vec<ThingAttribute>>,
}

/// <p>The input for the ListTopicRules operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct ListTopicRulesRequest {
    /// <p>The maximum number of results to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token used to retrieve the next value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies whether the rule is disabled.</p>
    #[serde(rename = "ruleDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_disabled: Option<bool>,
    /// <p>The topic.</p>
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
}

/// <p>The output from the ListTopicRules operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListTopicRulesResponse {
    /// <p>A token used to retrieve the next value.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The rules.</p>
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<TopicRuleListItem>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct ListV2LoggingLevelsRequest {
    /// <p>The maximum number of results to return at one time.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token used to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of resource for which you are configuring logging. Must be <code>THING_Group</code>.</p>
    #[serde(rename = "targetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ListV2LoggingLevelsResponse {
    /// <p>The logging configuration for a target.</p>
    #[serde(rename = "logTargetConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_target_configurations: Option<Vec<LogTargetConfiguration>>,
    /// <p>The token used to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>A log target.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LogTarget {
    /// <p>The target name.</p>
    #[serde(rename = "targetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_name: Option<String>,
    /// <p>The target type.</p>
    #[serde(rename = "targetType")]
    pub target_type: String,
}

/// <p>The target configuration.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct LogTargetConfiguration {
    /// <p>The logging level.</p>
    #[serde(rename = "logLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    /// <p>A log target</p>
    #[serde(rename = "logTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_target: Option<LogTarget>,
}

/// <p>Describes the logging options payload.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct LoggingOptionsPayload {
    /// <p>The log level.</p>
    #[serde(rename = "logLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    /// <p>The ARN of the IAM role that grants access.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
}

/// <p>Describes a file to be associated with an OTA update.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct OTAUpdateFile {
    /// <p>A list of name/attribute pairs.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The code signing method of the file.</p>
    #[serde(rename = "codeSigning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing: Option<CodeSigning>,
    /// <p>The name of the file.</p>
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// <p>The source of the file.</p>
    #[serde(rename = "fileSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_source: Option<Stream>,
    /// <p>The file version.</p>
    #[serde(rename = "fileVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_version: Option<String>,
}

/// <p>Information about an OTA update.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct OTAUpdateInfo {
    /// <p>A collection of name/value pairs</p>
    #[serde(rename = "additionalParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The AWS IoT job ARN associated with the OTA update.</p>
    #[serde(rename = "awsIotJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iot_job_arn: Option<String>,
    /// <p>The AWS IoT job ID associated with the OTA update.</p>
    #[serde(rename = "awsIotJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iot_job_id: Option<String>,
    /// <p>The date when the OTA update was created.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>A description of the OTA update.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Error information associated with the OTA update.</p>
    #[serde(rename = "errorInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_info: Option<ErrorInfo>,
    /// <p>The date when the OTA update was last updated.</p>
    #[serde(rename = "lastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>The OTA update ARN.</p>
    #[serde(rename = "otaUpdateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_arn: Option<String>,
    /// <p>A list of files associated with the OTA update.</p>
    #[serde(rename = "otaUpdateFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_files: Option<Vec<OTAUpdateFile>>,
    /// <p>The OTA update ID.</p>
    #[serde(rename = "otaUpdateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_id: Option<String>,
    /// <p>The status of the OTA update.</p>
    #[serde(rename = "otaUpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_status: Option<String>,
    /// <p>Specifies whether the OTA update will continue to run (CONTINUOUS), or will be complete after all those things specified as targets have completed the OTA update (SNAPSHOT). If continuous, the OTA update may also be run on a thing when a change is detected in a target. For example, an OTA update will run on a thing when the thing is added to a target group, even after the OTA update was completed by all things originally in the group. </p>
    #[serde(rename = "targetSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_selection: Option<String>,
    /// <p>The targets of the OTA update.</p>
    #[serde(rename = "targets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
}

/// <p>An OTA update summary.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct OTAUpdateSummary {
    /// <p>The date when the OTA update was created.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The OTA update ARN.</p>
    #[serde(rename = "otaUpdateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_arn: Option<String>,
    /// <p>The OTA update ID.</p>
    #[serde(rename = "otaUpdateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ota_update_id: Option<String>,
}

/// <p>A certificate that has been transferred but not yet accepted.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct OutgoingCertificate {
    /// <p>The certificate ARN.</p>
    #[serde(rename = "certificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The certificate ID.</p>
    #[serde(rename = "certificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    /// <p>The certificate creation date.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The date the transfer was initiated.</p>
    #[serde(rename = "transferDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_date: Option<f64>,
    /// <p>The transfer message.</p>
    #[serde(rename = "transferMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_message: Option<String>,
    /// <p>The AWS account to which the transfer was made.</p>
    #[serde(rename = "transferredTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferred_to: Option<String>,
}

/// <p>Describes an AWS IoT policy.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Policy {
    /// <p>The policy ARN.</p>
    #[serde(rename = "policyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    /// <p>The policy name.</p>
    #[serde(rename = "policyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

/// <p>Describes a policy version.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct PolicyVersion {
    /// <p>The date and time the policy was created.</p>
    #[serde(rename = "createDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    /// <p>Specifies whether the policy version is the default.</p>
    #[serde(rename = "isDefaultVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    /// <p>The policy version ID.</p>
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

/// <p>Configuration for pre-signed S3 URLs.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PresignedUrlConfig {
    /// <p>How long (in seconds) pre-signed URLs are valid. Valid values are 60 - 3600, the default value is 3600 seconds. Pre-signed URLs are generated when Jobs receives an MQTT request for the job document.</p>
    #[serde(rename = "expiresInSec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_sec: Option<i64>,
    /// <p>The ARN of an IAM role that grants grants permission to download files from the S3 bucket where the job data/updates are stored. The role must also grant permission for IoT to download the files.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>The input for the DynamoActionVS action that specifies the DynamoDB table to which the message data will be written.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PutItemInput {
    /// <p>The table where the message data will be written</p>
    #[serde(rename = "tableName")]
    pub table_name: String,
}

/// <p>The input to the RegisterCACertificate operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct RegisterCACertificateRequest {
    /// <p>Allows this CA certificate to be used for auto registration of device certificates.</p>
    #[serde(rename = "allowAutoRegistration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_auto_registration: Option<bool>,
    /// <p>The CA certificate.</p>
    #[serde(rename = "caCertificate")]
    pub ca_certificate: String,
    /// <p>Information about the registration configuration.</p>
    #[serde(rename = "registrationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_config: Option<RegistrationConfig>,
    /// <p>A boolean value that specifies if the CA certificate is set to active.</p>
    #[serde(rename = "setAsActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_as_active: Option<bool>,
    /// <p>The private key verification certificate.</p>
    #[serde(rename = "verificationCertificate")]
    pub verification_certificate: String,
}

/// <p>The output from the RegisterCACertificateResponse operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct RegisterCACertificateResponse {
    /// <p>The CA certificate ARN.</p>
    #[serde(rename = "certificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The CA certificate identifier.</p>
    #[serde(rename = "certificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
}

/// <p>The input to the RegisterCertificate operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct RegisterCertificateRequest {
    /// <p>The CA certificate used to sign the device certificate being registered.</p>
    #[serde(rename = "caCertificatePem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_pem: Option<String>,
    /// <p>The certificate data, in PEM format.</p>
    #[serde(rename = "certificatePem")]
    pub certificate_pem: String,
    /// <p>The status of the register certificate request.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The output from the RegisterCertificate operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct RegisterCertificateResponse {
    /// <p>The certificate ARN.</p>
    #[serde(rename = "certificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The certificate identifier.</p>
    #[serde(rename = "certificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct RegisterThingRequest {
    /// <p>The parameters for provisioning a thing.</p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>The provisioning template. </p>
    #[serde(rename = "templateBody")]
    pub template_body: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct RegisterThingResponse {
    #[serde(rename = "certificatePem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_pem: Option<String>,
    /// <p>ARNs for the generated resources.</p>
    #[serde(rename = "resourceArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The registration configuration.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationConfig {
    /// <p>The ARN of the role.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The template body.</p>
    #[serde(rename = "templateBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
}

/// <p>The input for the RejectCertificateTransfer operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct RejectCertificateTransferRequest {
    /// <p>The ID of the certificate.</p>
    #[serde(rename = "certificateId")]
    pub certificate_id: String,
    /// <p>The reason the certificate transfer was rejected.</p>
    #[serde(rename = "rejectReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct RemoveThingFromThingGroupRequest {
    /// <p>The ARN of the thing to remove from the group.</p>
    #[serde(rename = "thingArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
    /// <p>The group ARN.</p>
    #[serde(rename = "thingGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_arn: Option<String>,
    /// <p>The group name.</p>
    #[serde(rename = "thingGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_name: Option<String>,
    /// <p>The name of the thing to remove from the group.</p>
    #[serde(rename = "thingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct RemoveThingFromThingGroupResponse;

/// <p>The input for the ReplaceTopicRule operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct ReplaceTopicRuleRequest {
    /// <p>The name of the rule.</p>
    #[serde(rename = "ruleName")]
    pub rule_name: String,
    /// <p>The rule payload.</p>
    #[serde(rename = "topicRulePayload")]
    pub topic_rule_payload: TopicRulePayload,
}

/// <p>Describes an action to republish to another topic.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RepublishAction {
    /// <p>The ARN of the IAM role that grants access.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>The name of the MQTT topic.</p>
    #[serde(rename = "topic")]
    pub topic: String,
}

/// <p>Role alias description.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct RoleAliasDescription {
    /// <p>The UNIX timestamp of when the role alias was created.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The number of seconds for which the credential is valid.</p>
    #[serde(rename = "credentialDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_duration_seconds: Option<i64>,
    /// <p>The UNIX timestamp of when the role alias was last modified.</p>
    #[serde(rename = "lastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>The role alias owner.</p>
    #[serde(rename = "owner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// <p>The role alias.</p>
    #[serde(rename = "roleAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_alias: Option<String>,
    /// <p>The role ARN.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>Describes an action to write data to an Amazon S3 bucket.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct S3Action {
    /// <p>The Amazon S3 bucket.</p>
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    /// <p>The Amazon S3 canned ACL that controls access to the object identified by the object key. For more information, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#canned-acl">S3 canned ACLs</a>.</p>
    #[serde(rename = "cannedAcl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_acl: Option<String>,
    /// <p>The object key.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The ARN of the IAM role that grants access.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
}

/// <p>The location in S3 the contains the files to stream.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct S3Location {
    /// <p>The S3 bucket that contains the file to stream.</p>
    #[serde(rename = "bucket")]
    pub bucket: String,
    /// <p>The name of the file within the S3 bucket to stream.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The file version.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Describes an action to write a message to a Salesforce IoT Cloud Input Stream.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SalesforceAction {
    /// <p>The token used to authenticate access to the Salesforce IoT Cloud Input Stream. The token is available from the Salesforce IoT Cloud platform after creation of the Input Stream.</p>
    #[serde(rename = "token")]
    pub token: String,
    /// <p>The URL exposed by the Salesforce IoT Cloud Input Stream. The URL is available from the Salesforce IoT Cloud platform after creation of the Input Stream.</p>
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct SearchIndexRequest {
    /// <p>The search index name.</p>
    #[serde(rename = "indexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    /// <p>The maximum number of results to return at one time.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token used to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The search query string.</p>
    #[serde(rename = "queryString")]
    pub query_string: String,
    /// <p>The query version.</p>
    #[serde(rename = "queryVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_version: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct SearchIndexResponse {
    /// <p>The token used to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The things that match the search query.</p>
    #[serde(rename = "things")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub things: Option<Vec<ThingDocument>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct SetDefaultAuthorizerRequest {
    /// <p>The authorizer name.</p>
    #[serde(rename = "authorizerName")]
    pub authorizer_name: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct SetDefaultAuthorizerResponse {
    /// <p>The authorizer ARN.</p>
    #[serde(rename = "authorizerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_arn: Option<String>,
    /// <p>The authorizer name.</p>
    #[serde(rename = "authorizerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_name: Option<String>,
}

/// <p>The input for the SetDefaultPolicyVersion operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct SetDefaultPolicyVersionRequest {
    /// <p>The policy name.</p>
    #[serde(rename = "policyName")]
    pub policy_name: String,
    /// <p>The policy version ID.</p>
    #[serde(rename = "policyVersionId")]
    pub policy_version_id: String,
}

/// <p>The input for the SetLoggingOptions operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct SetLoggingOptionsRequest {
    /// <p>The logging options payload.</p>
    #[serde(rename = "loggingOptionsPayload")]
    pub logging_options_payload: LoggingOptionsPayload,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct SetV2LoggingLevelRequest {
    /// <p>The log level.</p>
    #[serde(rename = "logLevel")]
    pub log_level: String,
    /// <p>The log target.</p>
    #[serde(rename = "logTarget")]
    pub log_target: LogTarget,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct SetV2LoggingOptionsRequest {
    /// <p>The default logging level.</p>
    #[serde(rename = "defaultLogLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_log_level: Option<String>,
    /// <p>Set to true to disable all logs, otherwise set to false.</p>
    #[serde(rename = "disableAllLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_all_logs: Option<bool>,
    /// <p>The role ARN that allows IoT to write to Cloudwatch logs.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>Describes an action to publish to an Amazon SNS topic.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SnsAction {
    /// <p>The message format of the message to publish. Optional. Accepted values are "JSON" and "RAW". The default value of the attribute is "RAW". SNS uses this setting to determine if the payload should be parsed and relevant platform-specific bits of the payload should be extracted. To read more about SNS message formats, see <a href="http://docs.aws.amazon.com/sns/latest/dg/json-formats.html">http://docs.aws.amazon.com/sns/latest/dg/json-formats.html</a> refer to their official documentation.</p>
    #[serde(rename = "messageFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_format: Option<String>,
    /// <p>The ARN of the IAM role that grants access.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>The ARN of the SNS topic.</p>
    #[serde(rename = "targetArn")]
    pub target_arn: String,
}

/// <p>Describes an action to publish data to an Amazon SQS queue.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SqsAction {
    /// <p>The URL of the Amazon SQS queue.</p>
    #[serde(rename = "queueUrl")]
    pub queue_url: String,
    /// <p>The ARN of the IAM role that grants access.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>Specifies whether to use Base64 encoding.</p>
    #[serde(rename = "useBase64")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_base_64: Option<bool>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct StartThingRegistrationTaskRequest {
    /// <p>The S3 bucket that contains the input file.</p>
    #[serde(rename = "inputFileBucket")]
    pub input_file_bucket: String,
    /// <p>The name of input file within the S3 bucket. This file contains a newline delimited JSON file. Each line contains the parameter values to provision one device (thing).</p>
    #[serde(rename = "inputFileKey")]
    pub input_file_key: String,
    /// <p>The IAM role ARN that grants permission the input file.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>The provisioning template.</p>
    #[serde(rename = "templateBody")]
    pub template_body: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct StartThingRegistrationTaskResponse {
    /// <p>The bulk thing provisioning task ID.</p>
    #[serde(rename = "taskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct StopThingRegistrationTaskRequest {
    /// <p>The bulk thing provisioning task ID.</p>
    #[serde(rename = "taskId")]
    pub task_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct StopThingRegistrationTaskResponse;

/// <p>Describes a group of files that can be streamed.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Stream {
    /// <p>The ID of a file associated with a stream.</p>
    #[serde(rename = "fileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<i64>,
    /// <p>The stream ID.</p>
    #[serde(rename = "streamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// <p>Represents a file to stream.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct StreamFile {
    /// <p>The file ID.</p>
    #[serde(rename = "fileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<i64>,
    /// <p>The location of the file in S3.</p>
    #[serde(rename = "s3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_location: Option<S3Location>,
}

/// <p>Information about a stream.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct StreamInfo {
    /// <p>The date when the stream was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The description of the stream.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The files to stream.</p>
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<StreamFile>>,
    /// <p>The date when the stream was last updated.</p>
    #[serde(rename = "lastUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    /// <p>An IAM role AWS IoT assumes to access your S3 files.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The stream ARN.</p>
    #[serde(rename = "streamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    /// <p>The stream ID.</p>
    #[serde(rename = "streamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    /// <p>The stream version.</p>
    #[serde(rename = "streamVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_version: Option<i64>,
}

/// <p>A summary of a stream.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct StreamSummary {
    /// <p>A description of the stream.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The stream ARN.</p>
    #[serde(rename = "streamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    /// <p>The stream ID.</p>
    #[serde(rename = "streamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    /// <p>The stream version.</p>
    #[serde(rename = "streamVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_version: Option<i64>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct TestAuthorizationRequest {
    /// <p>A list of authorization info objects. Simulating authorization will create a response for each <code>authInfo</code> object in the list.</p>
    #[serde(rename = "authInfos")]
    pub auth_infos: Vec<AuthInfo>,
    /// <p>The MQTT client ID.</p>
    #[serde(rename = "clientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// <p>The Cognito identity pool ID.</p>
    #[serde(rename = "cognitoIdentityPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_identity_pool_id: Option<String>,
    /// <p>When testing custom authorization, the policies specified here are treated as if they are attached to the principal being authorized.</p>
    #[serde(rename = "policyNamesToAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names_to_add: Option<Vec<String>>,
    /// <p>When testing custom authorization, the policies specified here are treated as if they are not attached to the principal being authorized.</p>
    #[serde(rename = "policyNamesToSkip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names_to_skip: Option<Vec<String>>,
    /// <p>The principal.</p>
    #[serde(rename = "principal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct TestAuthorizationResponse {
    /// <p>The authentication results.</p>
    #[serde(rename = "authResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_results: Option<Vec<AuthResult>>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct TestInvokeAuthorizerRequest {
    /// <p>The custom authorizer name.</p>
    #[serde(rename = "authorizerName")]
    pub authorizer_name: String,
    /// <p>The token returned by your custom authentication service.</p>
    #[serde(rename = "token")]
    pub token: String,
    /// <p>The signature made with the token and your custom authentication service's private key.</p>
    #[serde(rename = "tokenSignature")]
    pub token_signature: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct TestInvokeAuthorizerResponse {
    /// <p>The number of seconds after which the connection is terminated.</p>
    #[serde(rename = "disconnectAfterInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_after_in_seconds: Option<i64>,
    /// <p>True if the token is authenticated, otherwise false.</p>
    #[serde(rename = "isAuthenticated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_authenticated: Option<bool>,
    /// <p>IAM policy documents.</p>
    #[serde(rename = "policyDocuments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_documents: Option<Vec<String>>,
    /// <p>The principal ID.</p>
    #[serde(rename = "principalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    /// <p>The number of seconds after which the temporary credentials are refreshed.</p>
    #[serde(rename = "refreshAfterInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_after_in_seconds: Option<i64>,
}

/// <p>The properties of the thing, including thing name, thing type name, and a list of thing attributes.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ThingAttribute {
    /// <p>A list of thing attributes which are name-value pairs.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The thing ARN.</p>
    #[serde(rename = "thingArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_arn: Option<String>,
    /// <p>The name of the thing.</p>
    #[serde(rename = "thingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
    /// <p>The name of the thing type, if the thing has been associated with a type.</p>
    #[serde(rename = "thingTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<String>,
    /// <p>The version of the thing record in the registry.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>The thing search index document.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ThingDocument {
    /// <p>The attributes.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// <p>The thing shadow.</p>
    #[serde(rename = "shadow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow: Option<String>,
    /// <p>Thing group names.</p>
    #[serde(rename = "thingGroupNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_names: Option<Vec<String>>,
    /// <p>The thing ID.</p>
    #[serde(rename = "thingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_id: Option<String>,
    /// <p>The thing name.</p>
    #[serde(rename = "thingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
    /// <p>The thing type name.</p>
    #[serde(rename = "thingTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<String>,
}

/// <p>Thing group metadata.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ThingGroupMetadata {
    /// <p>The UNIX timestamp of when the thing group was created.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The parent thing group name.</p>
    #[serde(rename = "parentGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_name: Option<String>,
    /// <p>The root parent thing group.</p>
    #[serde(rename = "rootToParentThingGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_to_parent_thing_groups: Option<Vec<GroupNameAndArn>>,
}

/// <p>Thing group properties.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ThingGroupProperties {
    /// <p>The thing group attributes in JSON format.</p>
    #[serde(rename = "attributePayload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_payload: Option<AttributePayload>,
    /// <p>The thing group description.</p>
    #[serde(rename = "thingGroupDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_group_description: Option<String>,
}

/// <p>Thing indexing configuration.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ThingIndexingConfiguration {
    /// <p>Thing indexing mode. Valid values are: </p> <ul> <li> <p>REGISTRY – Your thing index will contain only registry data.</p> </li> <li> <p>REGISTRY_AND_SHADOW - Your thing index will contain registry and shadow data.</p> </li> <li> <p>OFF - Thing indexing is disabled.</p> </li> </ul>
    #[serde(rename = "thingIndexingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_indexing_mode: Option<String>,
}

/// <p>The definition of the thing type, including thing type name and description.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ThingTypeDefinition {
    /// <p>The thing type ARN.</p>
    #[serde(rename = "thingTypeArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_arn: Option<String>,
    /// <p>The ThingTypeMetadata contains additional information about the thing type including: creation date and time, a value indicating whether the thing type is deprecated, and a date and time when it was deprecated.</p>
    #[serde(rename = "thingTypeMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_metadata: Option<ThingTypeMetadata>,
    /// <p>The name of the thing type.</p>
    #[serde(rename = "thingTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<String>,
    /// <p>The ThingTypeProperties for the thing type.</p>
    #[serde(rename = "thingTypeProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_properties: Option<ThingTypeProperties>,
}

/// <p>The ThingTypeMetadata contains additional information about the thing type including: creation date and time, a value indicating whether the thing type is deprecated, and a date and time when time was deprecated.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct ThingTypeMetadata {
    /// <p>The date and time when the thing type was created.</p>
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>Whether the thing type is deprecated. If <b>true</b>, no new things could be associated with this type.</p>
    #[serde(rename = "deprecated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// <p>The date and time when the thing type was deprecated.</p>
    #[serde(rename = "deprecationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_date: Option<f64>,
}

/// <p>The ThingTypeProperties contains information about the thing type including: a thing type description, and a list of searchable thing attribute names.</p>
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ThingTypeProperties {
    /// <p>A list of searchable thing attribute names.</p>
    #[serde(rename = "searchableAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable_attributes: Option<Vec<String>>,
    /// <p>The description of the thing type.</p>
    #[serde(rename = "thingTypeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_description: Option<String>,
}

/// <p>Describes a rule.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct TopicRule {
    /// <p>The actions associated with the rule.</p>
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    /// <p>The version of the SQL rules engine to use when evaluating the rule.</p>
    #[serde(rename = "awsIotSqlVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iot_sql_version: Option<String>,
    /// <p>The date and time the rule was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The description of the rule.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The action to perform when an error occurs.</p>
    #[serde(rename = "errorAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_action: Option<Action>,
    /// <p>Specifies whether the rule is disabled.</p>
    #[serde(rename = "ruleDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_disabled: Option<bool>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "ruleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// <p>The SQL statement used to query the topic. When using a SQL query with multiple lines, be sure to escape the newline characters.</p>
    #[serde(rename = "sql")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql: Option<String>,
}

/// <p>Describes a rule.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct TopicRuleListItem {
    /// <p>The date and time the rule was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The rule ARN.</p>
    #[serde(rename = "ruleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
    /// <p>Specifies whether the rule is disabled.</p>
    #[serde(rename = "ruleDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_disabled: Option<bool>,
    /// <p>The name of the rule.</p>
    #[serde(rename = "ruleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// <p>The pattern for the topic names that apply.</p>
    #[serde(rename = "topicPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_pattern: Option<String>,
}

/// <p>Describes a rule.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct TopicRulePayload {
    /// <p>The actions associated with the rule.</p>
    #[serde(rename = "actions")]
    pub actions: Vec<Action>,
    /// <p>The version of the SQL rules engine to use when evaluating the rule.</p>
    #[serde(rename = "awsIotSqlVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iot_sql_version: Option<String>,
    /// <p>The description of the rule.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The action to take when an error occurs.</p>
    #[serde(rename = "errorAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_action: Option<Action>,
    /// <p>Specifies whether the rule is disabled.</p>
    #[serde(rename = "ruleDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_disabled: Option<bool>,
    /// <p>The SQL statement used to query the topic. For more information, see <a href="http://docs.aws.amazon.com/iot/latest/developerguide/iot-rules.html#aws-iot-sql-reference">AWS IoT SQL Reference</a> in the <i>AWS IoT Developer Guide</i>.</p>
    #[serde(rename = "sql")]
    pub sql: String,
}

/// <p>The input for the TransferCertificate operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct TransferCertificateRequest {
    /// <p>The ID of the certificate.</p>
    #[serde(rename = "certificateId")]
    pub certificate_id: String,
    /// <p>The AWS account.</p>
    #[serde(rename = "targetAwsAccount")]
    pub target_aws_account: String,
    /// <p>The transfer message.</p>
    #[serde(rename = "transferMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_message: Option<String>,
}

/// <p>The output from the TransferCertificate operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct TransferCertificateResponse {
    /// <p>The ARN of the certificate.</p>
    #[serde(rename = "transferredCertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferred_certificate_arn: Option<String>,
}

/// <p>Data used to transfer a certificate to an AWS account.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct TransferData {
    /// <p>The date the transfer was accepted.</p>
    #[serde(rename = "acceptDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_date: Option<f64>,
    /// <p>The date the transfer was rejected.</p>
    #[serde(rename = "rejectDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_date: Option<f64>,
    /// <p>The reason why the transfer was rejected.</p>
    #[serde(rename = "rejectReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
    /// <p>The date the transfer took place.</p>
    #[serde(rename = "transferDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_date: Option<f64>,
    /// <p>The transfer message.</p>
    #[serde(rename = "transferMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_message: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateAuthorizerRequest {
    /// <p>The ARN of the authorizer's Lambda function.</p>
    #[serde(rename = "authorizerFunctionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_function_arn: Option<String>,
    /// <p>The authorizer name.</p>
    #[serde(rename = "authorizerName")]
    pub authorizer_name: String,
    /// <p>The status of the update authorizer request.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The key used to extract the token from the HTTP headers. </p>
    #[serde(rename = "tokenKeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_name: Option<String>,
    /// <p>The public keys used to verify the token signature.</p>
    #[serde(rename = "tokenSigningPublicKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_signing_public_keys: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateAuthorizerResponse {
    /// <p>The authorizer ARN.</p>
    #[serde(rename = "authorizerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_arn: Option<String>,
    /// <p>The authorizer name.</p>
    #[serde(rename = "authorizerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_name: Option<String>,
}

/// <p>The input to the UpdateCACertificate operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateCACertificateRequest {
    /// <p>The CA certificate identifier.</p>
    #[serde(rename = "certificateId")]
    pub certificate_id: String,
    /// <p>The new value for the auto registration status. Valid values are: "ENABLE" or "DISABLE".</p>
    #[serde(rename = "newAutoRegistrationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_auto_registration_status: Option<String>,
    /// <p>The updated status of the CA certificate.</p> <p> <b>Note:</b> The status value REGISTER_INACTIVE is deprecated and should not be used.</p>
    #[serde(rename = "newStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_status: Option<String>,
    /// <p>Information about the registration configuration.</p>
    #[serde(rename = "registrationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_config: Option<RegistrationConfig>,
    /// <p>If true, remove auto registration.</p>
    #[serde(rename = "removeAutoRegistration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_auto_registration: Option<bool>,
}

/// <p>The input for the UpdateCertificate operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateCertificateRequest {
    /// <p>The ID of the certificate.</p>
    #[serde(rename = "certificateId")]
    pub certificate_id: String,
    /// <p>The new status.</p> <p> <b>Note:</b> Setting the status to PENDING_TRANSFER will result in an exception being thrown. PENDING_TRANSFER is a status used internally by AWS IoT. It is not intended for developer use.</p> <p> <b>Note:</b> The status value REGISTER_INACTIVE is deprecated and should not be used.</p>
    #[serde(rename = "newStatus")]
    pub new_status: String,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateEventConfigurationsRequest {
    /// <p>The new event configuration values.</p>
    #[serde(rename = "eventConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_configurations: Option<::std::collections::HashMap<String, Configuration>>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateEventConfigurationsResponse;

#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateIndexingConfigurationRequest {
    /// <p>Thing indexing configuration.</p>
    #[serde(rename = "thingIndexingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_indexing_configuration: Option<ThingIndexingConfiguration>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateIndexingConfigurationResponse;

#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateRoleAliasRequest {
    /// <p>The number of seconds the credential will be valid.</p>
    #[serde(rename = "credentialDurationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_duration_seconds: Option<i64>,
    /// <p>The role alias to update.</p>
    #[serde(rename = "roleAlias")]
    pub role_alias: String,
    /// <p>The role ARN.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateRoleAliasResponse {
    /// <p>The role alias.</p>
    #[serde(rename = "roleAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_alias: Option<String>,
    /// <p>The role alias ARN.</p>
    #[serde(rename = "roleAliasArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_alias_arn: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateStreamRequest {
    /// <p>The description of the stream.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The files associated with the stream.</p>
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<StreamFile>>,
    /// <p>An IAM role that allows the IoT service principal assumes to access your S3 files.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The stream ID.</p>
    #[serde(rename = "streamId")]
    pub stream_id: String,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateStreamResponse {
    /// <p>A description of the stream.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The stream ARN.</p>
    #[serde(rename = "streamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
    /// <p>The stream ID.</p>
    #[serde(rename = "streamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    /// <p>The stream version.</p>
    #[serde(rename = "streamVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_version: Option<i64>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateThingGroupRequest {
    /// <p>The expected version of the thing group. If this does not match the version of the thing group being updated, the update will fail.</p>
    #[serde(rename = "expectedVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_version: Option<i64>,
    /// <p>The thing group to update.</p>
    #[serde(rename = "thingGroupName")]
    pub thing_group_name: String,
    /// <p>The thing group properties.</p>
    #[serde(rename = "thingGroupProperties")]
    pub thing_group_properties: ThingGroupProperties,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateThingGroupResponse {
    /// <p>The version of the updated thing group.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateThingGroupsForThingRequest {
    /// <p>The groups to which the thing will be added.</p>
    #[serde(rename = "thingGroupsToAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_groups_to_add: Option<Vec<String>>,
    /// <p>The groups from which the thing will be removed.</p>
    #[serde(rename = "thingGroupsToRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_groups_to_remove: Option<Vec<String>>,
    /// <p>The thing whose group memberships will be updated.</p>
    #[serde(rename = "thingName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_name: Option<String>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateThingGroupsForThingResponse;

/// <p>The input for the UpdateThing operation.</p>
#[derive(Default, Debug, Clone, Serialize)]
pub struct UpdateThingRequest {
    /// <p>A list of thing attributes, a JSON string containing name-value pairs. For example:</p> <p> <code>{\"attributes\":{\"name1\":\"value2\"}}</code> </p> <p>This data is used to add new attributes or update existing attributes.</p>
    #[serde(rename = "attributePayload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_payload: Option<AttributePayload>,
    /// <p>The expected version of the thing record in the registry. If the version of the record in the registry does not match the expected version specified in the request, the <code>UpdateThing</code> request is rejected with a <code>VersionConflictException</code>.</p>
    #[serde(rename = "expectedVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_version: Option<i64>,
    /// <p>Remove a thing type association. If <b>true</b>, the association is removed.</p>
    #[serde(rename = "removeThingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_thing_type: Option<bool>,
    /// <p>The name of the thing to update.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
    /// <p>The name of the thing type.</p>
    #[serde(rename = "thingTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thing_type_name: Option<String>,
}

/// <p>The output from the UpdateThing operation.</p>
#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateThingResponse;

/// Errors returned by AcceptCertificateTransfer
#[derive(Debug, PartialEq)]
pub enum AcceptCertificateTransferError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You can't revert the certificate transfer because the transfer is already complete.</p>
    TransferAlreadyCompleted(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AcceptCertificateTransferError {
    pub fn from_body(body: &str) -> AcceptCertificateTransferError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        AcceptCertificateTransferError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        AcceptCertificateTransferError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AcceptCertificateTransferError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ServiceUnavailableException" => {
                        AcceptCertificateTransferError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ThrottlingException" => {
                        AcceptCertificateTransferError::Throttling(String::from(error_message))
                    }
                    "TransferAlreadyCompletedException" => {
                        AcceptCertificateTransferError::TransferAlreadyCompleted(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedException" => {
                        AcceptCertificateTransferError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        AcceptCertificateTransferError::Validation(error_message.to_string())
                    }
                    _ => AcceptCertificateTransferError::Unknown(String::from(body)),
                }
            }
            Err(_) => AcceptCertificateTransferError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AcceptCertificateTransferError {
    fn from(err: serde_json::error::Error) -> AcceptCertificateTransferError {
        AcceptCertificateTransferError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AcceptCertificateTransferError {
    fn from(err: CredentialsError) -> AcceptCertificateTransferError {
        AcceptCertificateTransferError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AcceptCertificateTransferError {
    fn from(err: HttpDispatchError) -> AcceptCertificateTransferError {
        AcceptCertificateTransferError::HttpDispatch(err)
    }
}
impl From<io::Error> for AcceptCertificateTransferError {
    fn from(err: io::Error) -> AcceptCertificateTransferError {
        AcceptCertificateTransferError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AcceptCertificateTransferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AcceptCertificateTransferError {
    fn description(&self) -> &str {
        match *self {
            AcceptCertificateTransferError::InternalFailure(ref cause) => cause,
            AcceptCertificateTransferError::InvalidRequest(ref cause) => cause,
            AcceptCertificateTransferError::ResourceNotFound(ref cause) => cause,
            AcceptCertificateTransferError::ServiceUnavailable(ref cause) => cause,
            AcceptCertificateTransferError::Throttling(ref cause) => cause,
            AcceptCertificateTransferError::TransferAlreadyCompleted(ref cause) => cause,
            AcceptCertificateTransferError::Unauthorized(ref cause) => cause,
            AcceptCertificateTransferError::Validation(ref cause) => cause,
            AcceptCertificateTransferError::Credentials(ref err) => err.description(),
            AcceptCertificateTransferError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AcceptCertificateTransferError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddThingToThingGroup
#[derive(Debug, PartialEq)]
pub enum AddThingToThingGroupError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AddThingToThingGroupError {
    pub fn from_body(body: &str) -> AddThingToThingGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        AddThingToThingGroupError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        AddThingToThingGroupError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AddThingToThingGroupError::ResourceNotFound(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        AddThingToThingGroupError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddThingToThingGroupError::Validation(error_message.to_string())
                    }
                    _ => AddThingToThingGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddThingToThingGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddThingToThingGroupError {
    fn from(err: serde_json::error::Error) -> AddThingToThingGroupError {
        AddThingToThingGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddThingToThingGroupError {
    fn from(err: CredentialsError) -> AddThingToThingGroupError {
        AddThingToThingGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddThingToThingGroupError {
    fn from(err: HttpDispatchError) -> AddThingToThingGroupError {
        AddThingToThingGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddThingToThingGroupError {
    fn from(err: io::Error) -> AddThingToThingGroupError {
        AddThingToThingGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddThingToThingGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddThingToThingGroupError {
    fn description(&self) -> &str {
        match *self {
            AddThingToThingGroupError::InternalFailure(ref cause) => cause,
            AddThingToThingGroupError::InvalidRequest(ref cause) => cause,
            AddThingToThingGroupError::ResourceNotFound(ref cause) => cause,
            AddThingToThingGroupError::Throttling(ref cause) => cause,
            AddThingToThingGroupError::Validation(ref cause) => cause,
            AddThingToThingGroupError::Credentials(ref err) => err.description(),
            AddThingToThingGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddThingToThingGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AssociateTargetsWithJob
#[derive(Debug, PartialEq)]
pub enum AssociateTargetsWithJobError {
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The number of attached entities exceeds the limit.</p>
    LimitExceeded(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AssociateTargetsWithJobError {
    pub fn from_body(body: &str) -> AssociateTargetsWithJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidRequestException" => {
                        AssociateTargetsWithJobError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        AssociateTargetsWithJobError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AssociateTargetsWithJobError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        AssociateTargetsWithJobError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ThrottlingException" => {
                        AssociateTargetsWithJobError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        AssociateTargetsWithJobError::Validation(error_message.to_string())
                    }
                    _ => AssociateTargetsWithJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateTargetsWithJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateTargetsWithJobError {
    fn from(err: serde_json::error::Error) -> AssociateTargetsWithJobError {
        AssociateTargetsWithJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateTargetsWithJobError {
    fn from(err: CredentialsError) -> AssociateTargetsWithJobError {
        AssociateTargetsWithJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateTargetsWithJobError {
    fn from(err: HttpDispatchError) -> AssociateTargetsWithJobError {
        AssociateTargetsWithJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateTargetsWithJobError {
    fn from(err: io::Error) -> AssociateTargetsWithJobError {
        AssociateTargetsWithJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateTargetsWithJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateTargetsWithJobError {
    fn description(&self) -> &str {
        match *self {
            AssociateTargetsWithJobError::InvalidRequest(ref cause) => cause,
            AssociateTargetsWithJobError::LimitExceeded(ref cause) => cause,
            AssociateTargetsWithJobError::ResourceNotFound(ref cause) => cause,
            AssociateTargetsWithJobError::ServiceUnavailable(ref cause) => cause,
            AssociateTargetsWithJobError::Throttling(ref cause) => cause,
            AssociateTargetsWithJobError::Validation(ref cause) => cause,
            AssociateTargetsWithJobError::Credentials(ref err) => err.description(),
            AssociateTargetsWithJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateTargetsWithJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AttachPolicy
#[derive(Debug, PartialEq)]
pub enum AttachPolicyError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The number of attached entities exceeds the limit.</p>
    LimitExceeded(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AttachPolicyError {
    pub fn from_body(body: &str) -> AttachPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        AttachPolicyError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        AttachPolicyError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        AttachPolicyError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AttachPolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        AttachPolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        AttachPolicyError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        AttachPolicyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        AttachPolicyError::Validation(error_message.to_string())
                    }
                    _ => AttachPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => AttachPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AttachPolicyError {
    fn from(err: serde_json::error::Error) -> AttachPolicyError {
        AttachPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AttachPolicyError {
    fn from(err: CredentialsError) -> AttachPolicyError {
        AttachPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachPolicyError {
    fn from(err: HttpDispatchError) -> AttachPolicyError {
        AttachPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for AttachPolicyError {
    fn from(err: io::Error) -> AttachPolicyError {
        AttachPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AttachPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachPolicyError {
    fn description(&self) -> &str {
        match *self {
            AttachPolicyError::InternalFailure(ref cause) => cause,
            AttachPolicyError::InvalidRequest(ref cause) => cause,
            AttachPolicyError::LimitExceeded(ref cause) => cause,
            AttachPolicyError::ResourceNotFound(ref cause) => cause,
            AttachPolicyError::ServiceUnavailable(ref cause) => cause,
            AttachPolicyError::Throttling(ref cause) => cause,
            AttachPolicyError::Unauthorized(ref cause) => cause,
            AttachPolicyError::Validation(ref cause) => cause,
            AttachPolicyError::Credentials(ref err) => err.description(),
            AttachPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AttachPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AttachPrincipalPolicy
#[derive(Debug, PartialEq)]
pub enum AttachPrincipalPolicyError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The number of attached entities exceeds the limit.</p>
    LimitExceeded(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AttachPrincipalPolicyError {
    pub fn from_body(body: &str) -> AttachPrincipalPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        AttachPrincipalPolicyError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        AttachPrincipalPolicyError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        AttachPrincipalPolicyError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AttachPrincipalPolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        AttachPrincipalPolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        AttachPrincipalPolicyError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        AttachPrincipalPolicyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        AttachPrincipalPolicyError::Validation(error_message.to_string())
                    }
                    _ => AttachPrincipalPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => AttachPrincipalPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AttachPrincipalPolicyError {
    fn from(err: serde_json::error::Error) -> AttachPrincipalPolicyError {
        AttachPrincipalPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AttachPrincipalPolicyError {
    fn from(err: CredentialsError) -> AttachPrincipalPolicyError {
        AttachPrincipalPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachPrincipalPolicyError {
    fn from(err: HttpDispatchError) -> AttachPrincipalPolicyError {
        AttachPrincipalPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for AttachPrincipalPolicyError {
    fn from(err: io::Error) -> AttachPrincipalPolicyError {
        AttachPrincipalPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AttachPrincipalPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachPrincipalPolicyError {
    fn description(&self) -> &str {
        match *self {
            AttachPrincipalPolicyError::InternalFailure(ref cause) => cause,
            AttachPrincipalPolicyError::InvalidRequest(ref cause) => cause,
            AttachPrincipalPolicyError::LimitExceeded(ref cause) => cause,
            AttachPrincipalPolicyError::ResourceNotFound(ref cause) => cause,
            AttachPrincipalPolicyError::ServiceUnavailable(ref cause) => cause,
            AttachPrincipalPolicyError::Throttling(ref cause) => cause,
            AttachPrincipalPolicyError::Unauthorized(ref cause) => cause,
            AttachPrincipalPolicyError::Validation(ref cause) => cause,
            AttachPrincipalPolicyError::Credentials(ref err) => err.description(),
            AttachPrincipalPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AttachPrincipalPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AttachThingPrincipal
#[derive(Debug, PartialEq)]
pub enum AttachThingPrincipalError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AttachThingPrincipalError {
    pub fn from_body(body: &str) -> AttachThingPrincipalError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        AttachThingPrincipalError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        AttachThingPrincipalError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AttachThingPrincipalError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        AttachThingPrincipalError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        AttachThingPrincipalError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        AttachThingPrincipalError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        AttachThingPrincipalError::Validation(error_message.to_string())
                    }
                    _ => AttachThingPrincipalError::Unknown(String::from(body)),
                }
            }
            Err(_) => AttachThingPrincipalError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AttachThingPrincipalError {
    fn from(err: serde_json::error::Error) -> AttachThingPrincipalError {
        AttachThingPrincipalError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AttachThingPrincipalError {
    fn from(err: CredentialsError) -> AttachThingPrincipalError {
        AttachThingPrincipalError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AttachThingPrincipalError {
    fn from(err: HttpDispatchError) -> AttachThingPrincipalError {
        AttachThingPrincipalError::HttpDispatch(err)
    }
}
impl From<io::Error> for AttachThingPrincipalError {
    fn from(err: io::Error) -> AttachThingPrincipalError {
        AttachThingPrincipalError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AttachThingPrincipalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachThingPrincipalError {
    fn description(&self) -> &str {
        match *self {
            AttachThingPrincipalError::InternalFailure(ref cause) => cause,
            AttachThingPrincipalError::InvalidRequest(ref cause) => cause,
            AttachThingPrincipalError::ResourceNotFound(ref cause) => cause,
            AttachThingPrincipalError::ServiceUnavailable(ref cause) => cause,
            AttachThingPrincipalError::Throttling(ref cause) => cause,
            AttachThingPrincipalError::Unauthorized(ref cause) => cause,
            AttachThingPrincipalError::Validation(ref cause) => cause,
            AttachThingPrincipalError::Credentials(ref err) => err.description(),
            AttachThingPrincipalError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AttachThingPrincipalError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CancelCertificateTransfer
#[derive(Debug, PartialEq)]
pub enum CancelCertificateTransferError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You can't revert the certificate transfer because the transfer is already complete.</p>
    TransferAlreadyCompleted(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CancelCertificateTransferError {
    pub fn from_body(body: &str) -> CancelCertificateTransferError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        CancelCertificateTransferError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CancelCertificateTransferError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CancelCertificateTransferError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ServiceUnavailableException" => {
                        CancelCertificateTransferError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ThrottlingException" => {
                        CancelCertificateTransferError::Throttling(String::from(error_message))
                    }
                    "TransferAlreadyCompletedException" => {
                        CancelCertificateTransferError::TransferAlreadyCompleted(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedException" => {
                        CancelCertificateTransferError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CancelCertificateTransferError::Validation(error_message.to_string())
                    }
                    _ => CancelCertificateTransferError::Unknown(String::from(body)),
                }
            }
            Err(_) => CancelCertificateTransferError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CancelCertificateTransferError {
    fn from(err: serde_json::error::Error) -> CancelCertificateTransferError {
        CancelCertificateTransferError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CancelCertificateTransferError {
    fn from(err: CredentialsError) -> CancelCertificateTransferError {
        CancelCertificateTransferError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CancelCertificateTransferError {
    fn from(err: HttpDispatchError) -> CancelCertificateTransferError {
        CancelCertificateTransferError::HttpDispatch(err)
    }
}
impl From<io::Error> for CancelCertificateTransferError {
    fn from(err: io::Error) -> CancelCertificateTransferError {
        CancelCertificateTransferError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CancelCertificateTransferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelCertificateTransferError {
    fn description(&self) -> &str {
        match *self {
            CancelCertificateTransferError::InternalFailure(ref cause) => cause,
            CancelCertificateTransferError::InvalidRequest(ref cause) => cause,
            CancelCertificateTransferError::ResourceNotFound(ref cause) => cause,
            CancelCertificateTransferError::ServiceUnavailable(ref cause) => cause,
            CancelCertificateTransferError::Throttling(ref cause) => cause,
            CancelCertificateTransferError::TransferAlreadyCompleted(ref cause) => cause,
            CancelCertificateTransferError::Unauthorized(ref cause) => cause,
            CancelCertificateTransferError::Validation(ref cause) => cause,
            CancelCertificateTransferError::Credentials(ref err) => err.description(),
            CancelCertificateTransferError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CancelCertificateTransferError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CancelJob
#[derive(Debug, PartialEq)]
pub enum CancelJobError {
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CancelJobError {
    pub fn from_body(body: &str) -> CancelJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidRequestException" => {
                        CancelJobError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CancelJobError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CancelJobError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        CancelJobError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => CancelJobError::Validation(error_message.to_string()),
                    _ => CancelJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => CancelJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CancelJobError {
    fn from(err: serde_json::error::Error) -> CancelJobError {
        CancelJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CancelJobError {
    fn from(err: CredentialsError) -> CancelJobError {
        CancelJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CancelJobError {
    fn from(err: HttpDispatchError) -> CancelJobError {
        CancelJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for CancelJobError {
    fn from(err: io::Error) -> CancelJobError {
        CancelJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CancelJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelJobError {
    fn description(&self) -> &str {
        match *self {
            CancelJobError::InvalidRequest(ref cause) => cause,
            CancelJobError::ResourceNotFound(ref cause) => cause,
            CancelJobError::ServiceUnavailable(ref cause) => cause,
            CancelJobError::Throttling(ref cause) => cause,
            CancelJobError::Validation(ref cause) => cause,
            CancelJobError::Credentials(ref err) => err.description(),
            CancelJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CancelJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ClearDefaultAuthorizer
#[derive(Debug, PartialEq)]
pub enum ClearDefaultAuthorizerError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ClearDefaultAuthorizerError {
    pub fn from_body(body: &str) -> ClearDefaultAuthorizerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ClearDefaultAuthorizerError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ClearDefaultAuthorizerError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ClearDefaultAuthorizerError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ClearDefaultAuthorizerError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ClearDefaultAuthorizerError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ClearDefaultAuthorizerError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ClearDefaultAuthorizerError::Validation(error_message.to_string())
                    }
                    _ => ClearDefaultAuthorizerError::Unknown(String::from(body)),
                }
            }
            Err(_) => ClearDefaultAuthorizerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ClearDefaultAuthorizerError {
    fn from(err: serde_json::error::Error) -> ClearDefaultAuthorizerError {
        ClearDefaultAuthorizerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ClearDefaultAuthorizerError {
    fn from(err: CredentialsError) -> ClearDefaultAuthorizerError {
        ClearDefaultAuthorizerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ClearDefaultAuthorizerError {
    fn from(err: HttpDispatchError) -> ClearDefaultAuthorizerError {
        ClearDefaultAuthorizerError::HttpDispatch(err)
    }
}
impl From<io::Error> for ClearDefaultAuthorizerError {
    fn from(err: io::Error) -> ClearDefaultAuthorizerError {
        ClearDefaultAuthorizerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ClearDefaultAuthorizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ClearDefaultAuthorizerError {
    fn description(&self) -> &str {
        match *self {
            ClearDefaultAuthorizerError::InternalFailure(ref cause) => cause,
            ClearDefaultAuthorizerError::InvalidRequest(ref cause) => cause,
            ClearDefaultAuthorizerError::ResourceNotFound(ref cause) => cause,
            ClearDefaultAuthorizerError::ServiceUnavailable(ref cause) => cause,
            ClearDefaultAuthorizerError::Throttling(ref cause) => cause,
            ClearDefaultAuthorizerError::Unauthorized(ref cause) => cause,
            ClearDefaultAuthorizerError::Validation(ref cause) => cause,
            ClearDefaultAuthorizerError::Credentials(ref err) => err.description(),
            ClearDefaultAuthorizerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ClearDefaultAuthorizerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAuthorizer
#[derive(Debug, PartialEq)]
pub enum CreateAuthorizerError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The number of attached entities exceeds the limit.</p>
    LimitExceeded(String),
    ///<p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateAuthorizerError {
    pub fn from_body(body: &str) -> CreateAuthorizerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        CreateAuthorizerError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateAuthorizerError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateAuthorizerError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateAuthorizerError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateAuthorizerError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        CreateAuthorizerError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateAuthorizerError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateAuthorizerError::Validation(error_message.to_string())
                    }
                    _ => CreateAuthorizerError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateAuthorizerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateAuthorizerError {
    fn from(err: serde_json::error::Error) -> CreateAuthorizerError {
        CreateAuthorizerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateAuthorizerError {
    fn from(err: CredentialsError) -> CreateAuthorizerError {
        CreateAuthorizerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAuthorizerError {
    fn from(err: HttpDispatchError) -> CreateAuthorizerError {
        CreateAuthorizerError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateAuthorizerError {
    fn from(err: io::Error) -> CreateAuthorizerError {
        CreateAuthorizerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateAuthorizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAuthorizerError {
    fn description(&self) -> &str {
        match *self {
            CreateAuthorizerError::InternalFailure(ref cause) => cause,
            CreateAuthorizerError::InvalidRequest(ref cause) => cause,
            CreateAuthorizerError::LimitExceeded(ref cause) => cause,
            CreateAuthorizerError::ResourceAlreadyExists(ref cause) => cause,
            CreateAuthorizerError::ServiceUnavailable(ref cause) => cause,
            CreateAuthorizerError::Throttling(ref cause) => cause,
            CreateAuthorizerError::Unauthorized(ref cause) => cause,
            CreateAuthorizerError::Validation(ref cause) => cause,
            CreateAuthorizerError::Credentials(ref err) => err.description(),
            CreateAuthorizerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateAuthorizerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCertificateFromCsr
#[derive(Debug, PartialEq)]
pub enum CreateCertificateFromCsrError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateCertificateFromCsrError {
    pub fn from_body(body: &str) -> CreateCertificateFromCsrError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        CreateCertificateFromCsrError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateCertificateFromCsrError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateCertificateFromCsrError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ThrottlingException" => {
                        CreateCertificateFromCsrError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateCertificateFromCsrError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateCertificateFromCsrError::Validation(error_message.to_string())
                    }
                    _ => CreateCertificateFromCsrError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateCertificateFromCsrError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateCertificateFromCsrError {
    fn from(err: serde_json::error::Error) -> CreateCertificateFromCsrError {
        CreateCertificateFromCsrError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateCertificateFromCsrError {
    fn from(err: CredentialsError) -> CreateCertificateFromCsrError {
        CreateCertificateFromCsrError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCertificateFromCsrError {
    fn from(err: HttpDispatchError) -> CreateCertificateFromCsrError {
        CreateCertificateFromCsrError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateCertificateFromCsrError {
    fn from(err: io::Error) -> CreateCertificateFromCsrError {
        CreateCertificateFromCsrError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateCertificateFromCsrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCertificateFromCsrError {
    fn description(&self) -> &str {
        match *self {
            CreateCertificateFromCsrError::InternalFailure(ref cause) => cause,
            CreateCertificateFromCsrError::InvalidRequest(ref cause) => cause,
            CreateCertificateFromCsrError::ServiceUnavailable(ref cause) => cause,
            CreateCertificateFromCsrError::Throttling(ref cause) => cause,
            CreateCertificateFromCsrError::Unauthorized(ref cause) => cause,
            CreateCertificateFromCsrError::Validation(ref cause) => cause,
            CreateCertificateFromCsrError::Credentials(ref err) => err.description(),
            CreateCertificateFromCsrError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateCertificateFromCsrError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateJob
#[derive(Debug, PartialEq)]
pub enum CreateJobError {
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The number of attached entities exceeds the limit.</p>
    LimitExceeded(String),
    ///<p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateJobError {
    pub fn from_body(body: &str) -> CreateJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidRequestException" => {
                        CreateJobError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateJobError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateJobError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateJobError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateJobError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        CreateJobError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => CreateJobError::Validation(error_message.to_string()),
                    _ => CreateJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateJobError {
    fn from(err: serde_json::error::Error) -> CreateJobError {
        CreateJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateJobError {
    fn from(err: CredentialsError) -> CreateJobError {
        CreateJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateJobError {
    fn from(err: HttpDispatchError) -> CreateJobError {
        CreateJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateJobError {
    fn from(err: io::Error) -> CreateJobError {
        CreateJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateJobError {
    fn description(&self) -> &str {
        match *self {
            CreateJobError::InvalidRequest(ref cause) => cause,
            CreateJobError::LimitExceeded(ref cause) => cause,
            CreateJobError::ResourceAlreadyExists(ref cause) => cause,
            CreateJobError::ResourceNotFound(ref cause) => cause,
            CreateJobError::ServiceUnavailable(ref cause) => cause,
            CreateJobError::Throttling(ref cause) => cause,
            CreateJobError::Validation(ref cause) => cause,
            CreateJobError::Credentials(ref err) => err.description(),
            CreateJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateKeysAndCertificate
#[derive(Debug, PartialEq)]
pub enum CreateKeysAndCertificateError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateKeysAndCertificateError {
    pub fn from_body(body: &str) -> CreateKeysAndCertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        CreateKeysAndCertificateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateKeysAndCertificateError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateKeysAndCertificateError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ThrottlingException" => {
                        CreateKeysAndCertificateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateKeysAndCertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateKeysAndCertificateError::Validation(error_message.to_string())
                    }
                    _ => CreateKeysAndCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateKeysAndCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateKeysAndCertificateError {
    fn from(err: serde_json::error::Error) -> CreateKeysAndCertificateError {
        CreateKeysAndCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateKeysAndCertificateError {
    fn from(err: CredentialsError) -> CreateKeysAndCertificateError {
        CreateKeysAndCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateKeysAndCertificateError {
    fn from(err: HttpDispatchError) -> CreateKeysAndCertificateError {
        CreateKeysAndCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateKeysAndCertificateError {
    fn from(err: io::Error) -> CreateKeysAndCertificateError {
        CreateKeysAndCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateKeysAndCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateKeysAndCertificateError {
    fn description(&self) -> &str {
        match *self {
            CreateKeysAndCertificateError::InternalFailure(ref cause) => cause,
            CreateKeysAndCertificateError::InvalidRequest(ref cause) => cause,
            CreateKeysAndCertificateError::ServiceUnavailable(ref cause) => cause,
            CreateKeysAndCertificateError::Throttling(ref cause) => cause,
            CreateKeysAndCertificateError::Unauthorized(ref cause) => cause,
            CreateKeysAndCertificateError::Validation(ref cause) => cause,
            CreateKeysAndCertificateError::Credentials(ref err) => err.description(),
            CreateKeysAndCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateKeysAndCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateOTAUpdate
#[derive(Debug, PartialEq)]
pub enum CreateOTAUpdateError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateOTAUpdateError {
    pub fn from_body(body: &str) -> CreateOTAUpdateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        CreateOTAUpdateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateOTAUpdateError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateOTAUpdateError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateOTAUpdateError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateOTAUpdateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        CreateOTAUpdateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateOTAUpdateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateOTAUpdateError::Validation(error_message.to_string())
                    }
                    _ => CreateOTAUpdateError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateOTAUpdateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateOTAUpdateError {
    fn from(err: serde_json::error::Error) -> CreateOTAUpdateError {
        CreateOTAUpdateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateOTAUpdateError {
    fn from(err: CredentialsError) -> CreateOTAUpdateError {
        CreateOTAUpdateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateOTAUpdateError {
    fn from(err: HttpDispatchError) -> CreateOTAUpdateError {
        CreateOTAUpdateError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateOTAUpdateError {
    fn from(err: io::Error) -> CreateOTAUpdateError {
        CreateOTAUpdateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateOTAUpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateOTAUpdateError {
    fn description(&self) -> &str {
        match *self {
            CreateOTAUpdateError::InternalFailure(ref cause) => cause,
            CreateOTAUpdateError::InvalidRequest(ref cause) => cause,
            CreateOTAUpdateError::ResourceAlreadyExists(ref cause) => cause,
            CreateOTAUpdateError::ResourceNotFound(ref cause) => cause,
            CreateOTAUpdateError::ServiceUnavailable(ref cause) => cause,
            CreateOTAUpdateError::Throttling(ref cause) => cause,
            CreateOTAUpdateError::Unauthorized(ref cause) => cause,
            CreateOTAUpdateError::Validation(ref cause) => cause,
            CreateOTAUpdateError::Credentials(ref err) => err.description(),
            CreateOTAUpdateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateOTAUpdateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePolicy
#[derive(Debug, PartialEq)]
pub enum CreatePolicyError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The policy documentation is not valid.</p>
    MalformedPolicy(String),
    ///<p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreatePolicyError {
    pub fn from_body(body: &str) -> CreatePolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        CreatePolicyError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreatePolicyError::InvalidRequest(String::from(error_message))
                    }
                    "MalformedPolicyException" => {
                        CreatePolicyError::MalformedPolicy(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreatePolicyError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreatePolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        CreatePolicyError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreatePolicyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreatePolicyError::Validation(error_message.to_string())
                    }
                    _ => CreatePolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreatePolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreatePolicyError {
    fn from(err: serde_json::error::Error) -> CreatePolicyError {
        CreatePolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePolicyError {
    fn from(err: CredentialsError) -> CreatePolicyError {
        CreatePolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePolicyError {
    fn from(err: HttpDispatchError) -> CreatePolicyError {
        CreatePolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePolicyError {
    fn from(err: io::Error) -> CreatePolicyError {
        CreatePolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePolicyError {
    fn description(&self) -> &str {
        match *self {
            CreatePolicyError::InternalFailure(ref cause) => cause,
            CreatePolicyError::InvalidRequest(ref cause) => cause,
            CreatePolicyError::MalformedPolicy(ref cause) => cause,
            CreatePolicyError::ResourceAlreadyExists(ref cause) => cause,
            CreatePolicyError::ServiceUnavailable(ref cause) => cause,
            CreatePolicyError::Throttling(ref cause) => cause,
            CreatePolicyError::Unauthorized(ref cause) => cause,
            CreatePolicyError::Validation(ref cause) => cause,
            CreatePolicyError::Credentials(ref err) => err.description(),
            CreatePolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreatePolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePolicyVersion
#[derive(Debug, PartialEq)]
pub enum CreatePolicyVersionError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The policy documentation is not valid.</p>
    MalformedPolicy(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    ///<p>The number of policy versions exceeds the limit.</p>
    VersionsLimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreatePolicyVersionError {
    pub fn from_body(body: &str) -> CreatePolicyVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        CreatePolicyVersionError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreatePolicyVersionError::InvalidRequest(String::from(error_message))
                    }
                    "MalformedPolicyException" => {
                        CreatePolicyVersionError::MalformedPolicy(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreatePolicyVersionError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreatePolicyVersionError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        CreatePolicyVersionError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreatePolicyVersionError::Unauthorized(String::from(error_message))
                    }
                    "VersionsLimitExceededException" => {
                        CreatePolicyVersionError::VersionsLimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreatePolicyVersionError::Validation(error_message.to_string())
                    }
                    _ => CreatePolicyVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreatePolicyVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreatePolicyVersionError {
    fn from(err: serde_json::error::Error) -> CreatePolicyVersionError {
        CreatePolicyVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePolicyVersionError {
    fn from(err: CredentialsError) -> CreatePolicyVersionError {
        CreatePolicyVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePolicyVersionError {
    fn from(err: HttpDispatchError) -> CreatePolicyVersionError {
        CreatePolicyVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePolicyVersionError {
    fn from(err: io::Error) -> CreatePolicyVersionError {
        CreatePolicyVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePolicyVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePolicyVersionError {
    fn description(&self) -> &str {
        match *self {
            CreatePolicyVersionError::InternalFailure(ref cause) => cause,
            CreatePolicyVersionError::InvalidRequest(ref cause) => cause,
            CreatePolicyVersionError::MalformedPolicy(ref cause) => cause,
            CreatePolicyVersionError::ResourceNotFound(ref cause) => cause,
            CreatePolicyVersionError::ServiceUnavailable(ref cause) => cause,
            CreatePolicyVersionError::Throttling(ref cause) => cause,
            CreatePolicyVersionError::Unauthorized(ref cause) => cause,
            CreatePolicyVersionError::VersionsLimitExceeded(ref cause) => cause,
            CreatePolicyVersionError::Validation(ref cause) => cause,
            CreatePolicyVersionError::Credentials(ref err) => err.description(),
            CreatePolicyVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreatePolicyVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateRoleAlias
#[derive(Debug, PartialEq)]
pub enum CreateRoleAliasError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The number of attached entities exceeds the limit.</p>
    LimitExceeded(String),
    ///<p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateRoleAliasError {
    pub fn from_body(body: &str) -> CreateRoleAliasError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        CreateRoleAliasError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateRoleAliasError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateRoleAliasError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateRoleAliasError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateRoleAliasError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        CreateRoleAliasError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateRoleAliasError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateRoleAliasError::Validation(error_message.to_string())
                    }
                    _ => CreateRoleAliasError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateRoleAliasError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateRoleAliasError {
    fn from(err: serde_json::error::Error) -> CreateRoleAliasError {
        CreateRoleAliasError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateRoleAliasError {
    fn from(err: CredentialsError) -> CreateRoleAliasError {
        CreateRoleAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateRoleAliasError {
    fn from(err: HttpDispatchError) -> CreateRoleAliasError {
        CreateRoleAliasError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateRoleAliasError {
    fn from(err: io::Error) -> CreateRoleAliasError {
        CreateRoleAliasError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateRoleAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRoleAliasError {
    fn description(&self) -> &str {
        match *self {
            CreateRoleAliasError::InternalFailure(ref cause) => cause,
            CreateRoleAliasError::InvalidRequest(ref cause) => cause,
            CreateRoleAliasError::LimitExceeded(ref cause) => cause,
            CreateRoleAliasError::ResourceAlreadyExists(ref cause) => cause,
            CreateRoleAliasError::ServiceUnavailable(ref cause) => cause,
            CreateRoleAliasError::Throttling(ref cause) => cause,
            CreateRoleAliasError::Unauthorized(ref cause) => cause,
            CreateRoleAliasError::Validation(ref cause) => cause,
            CreateRoleAliasError::Credentials(ref err) => err.description(),
            CreateRoleAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateRoleAliasError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateStream
#[derive(Debug, PartialEq)]
pub enum CreateStreamError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateStreamError {
    pub fn from_body(body: &str) -> CreateStreamError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        CreateStreamError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateStreamError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateStreamError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateStreamError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateStreamError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        CreateStreamError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateStreamError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateStreamError::Validation(error_message.to_string())
                    }
                    _ => CreateStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateStreamError {
    fn from(err: serde_json::error::Error) -> CreateStreamError {
        CreateStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateStreamError {
    fn from(err: CredentialsError) -> CreateStreamError {
        CreateStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateStreamError {
    fn from(err: HttpDispatchError) -> CreateStreamError {
        CreateStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateStreamError {
    fn from(err: io::Error) -> CreateStreamError {
        CreateStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateStreamError {
    fn description(&self) -> &str {
        match *self {
            CreateStreamError::InternalFailure(ref cause) => cause,
            CreateStreamError::InvalidRequest(ref cause) => cause,
            CreateStreamError::ResourceAlreadyExists(ref cause) => cause,
            CreateStreamError::ResourceNotFound(ref cause) => cause,
            CreateStreamError::ServiceUnavailable(ref cause) => cause,
            CreateStreamError::Throttling(ref cause) => cause,
            CreateStreamError::Unauthorized(ref cause) => cause,
            CreateStreamError::Validation(ref cause) => cause,
            CreateStreamError::Credentials(ref err) => err.description(),
            CreateStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateStreamError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateThing
#[derive(Debug, PartialEq)]
pub enum CreateThingError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateThingError {
    pub fn from_body(body: &str) -> CreateThingError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        CreateThingError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateThingError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateThingError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateThingError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateThingError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        CreateThingError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateThingError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateThingError::Validation(error_message.to_string())
                    }
                    _ => CreateThingError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateThingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateThingError {
    fn from(err: serde_json::error::Error) -> CreateThingError {
        CreateThingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateThingError {
    fn from(err: CredentialsError) -> CreateThingError {
        CreateThingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateThingError {
    fn from(err: HttpDispatchError) -> CreateThingError {
        CreateThingError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateThingError {
    fn from(err: io::Error) -> CreateThingError {
        CreateThingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateThingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateThingError {
    fn description(&self) -> &str {
        match *self {
            CreateThingError::InternalFailure(ref cause) => cause,
            CreateThingError::InvalidRequest(ref cause) => cause,
            CreateThingError::ResourceAlreadyExists(ref cause) => cause,
            CreateThingError::ResourceNotFound(ref cause) => cause,
            CreateThingError::ServiceUnavailable(ref cause) => cause,
            CreateThingError::Throttling(ref cause) => cause,
            CreateThingError::Unauthorized(ref cause) => cause,
            CreateThingError::Validation(ref cause) => cause,
            CreateThingError::Credentials(ref err) => err.description(),
            CreateThingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateThingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateThingGroup
#[derive(Debug, PartialEq)]
pub enum CreateThingGroupError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateThingGroupError {
    pub fn from_body(body: &str) -> CreateThingGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        CreateThingGroupError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateThingGroupError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateThingGroupError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        CreateThingGroupError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateThingGroupError::Validation(error_message.to_string())
                    }
                    _ => CreateThingGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateThingGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateThingGroupError {
    fn from(err: serde_json::error::Error) -> CreateThingGroupError {
        CreateThingGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateThingGroupError {
    fn from(err: CredentialsError) -> CreateThingGroupError {
        CreateThingGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateThingGroupError {
    fn from(err: HttpDispatchError) -> CreateThingGroupError {
        CreateThingGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateThingGroupError {
    fn from(err: io::Error) -> CreateThingGroupError {
        CreateThingGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateThingGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateThingGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateThingGroupError::InternalFailure(ref cause) => cause,
            CreateThingGroupError::InvalidRequest(ref cause) => cause,
            CreateThingGroupError::ResourceAlreadyExists(ref cause) => cause,
            CreateThingGroupError::Throttling(ref cause) => cause,
            CreateThingGroupError::Validation(ref cause) => cause,
            CreateThingGroupError::Credentials(ref err) => err.description(),
            CreateThingGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateThingGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateThingType
#[derive(Debug, PartialEq)]
pub enum CreateThingTypeError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateThingTypeError {
    pub fn from_body(body: &str) -> CreateThingTypeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        CreateThingTypeError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateThingTypeError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateThingTypeError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateThingTypeError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        CreateThingTypeError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        CreateThingTypeError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateThingTypeError::Validation(error_message.to_string())
                    }
                    _ => CreateThingTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateThingTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateThingTypeError {
    fn from(err: serde_json::error::Error) -> CreateThingTypeError {
        CreateThingTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateThingTypeError {
    fn from(err: CredentialsError) -> CreateThingTypeError {
        CreateThingTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateThingTypeError {
    fn from(err: HttpDispatchError) -> CreateThingTypeError {
        CreateThingTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateThingTypeError {
    fn from(err: io::Error) -> CreateThingTypeError {
        CreateThingTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateThingTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateThingTypeError {
    fn description(&self) -> &str {
        match *self {
            CreateThingTypeError::InternalFailure(ref cause) => cause,
            CreateThingTypeError::InvalidRequest(ref cause) => cause,
            CreateThingTypeError::ResourceAlreadyExists(ref cause) => cause,
            CreateThingTypeError::ServiceUnavailable(ref cause) => cause,
            CreateThingTypeError::Throttling(ref cause) => cause,
            CreateThingTypeError::Unauthorized(ref cause) => cause,
            CreateThingTypeError::Validation(ref cause) => cause,
            CreateThingTypeError::Credentials(ref err) => err.description(),
            CreateThingTypeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateThingTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTopicRule
#[derive(Debug, PartialEq)]
pub enum CreateTopicRuleError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The Rule-SQL expression can't be parsed correctly.</p>
    SqlParse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateTopicRuleError {
    pub fn from_body(body: &str) -> CreateTopicRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        CreateTopicRuleError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        CreateTopicRuleError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        CreateTopicRuleError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        CreateTopicRuleError::ServiceUnavailable(String::from(error_message))
                    }
                    "SqlParseException" => {
                        CreateTopicRuleError::SqlParse(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateTopicRuleError::Validation(error_message.to_string())
                    }
                    _ => CreateTopicRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateTopicRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateTopicRuleError {
    fn from(err: serde_json::error::Error) -> CreateTopicRuleError {
        CreateTopicRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateTopicRuleError {
    fn from(err: CredentialsError) -> CreateTopicRuleError {
        CreateTopicRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTopicRuleError {
    fn from(err: HttpDispatchError) -> CreateTopicRuleError {
        CreateTopicRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTopicRuleError {
    fn from(err: io::Error) -> CreateTopicRuleError {
        CreateTopicRuleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateTopicRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTopicRuleError {
    fn description(&self) -> &str {
        match *self {
            CreateTopicRuleError::Internal(ref cause) => cause,
            CreateTopicRuleError::InvalidRequest(ref cause) => cause,
            CreateTopicRuleError::ResourceAlreadyExists(ref cause) => cause,
            CreateTopicRuleError::ServiceUnavailable(ref cause) => cause,
            CreateTopicRuleError::SqlParse(ref cause) => cause,
            CreateTopicRuleError::Validation(ref cause) => cause,
            CreateTopicRuleError::Credentials(ref err) => err.description(),
            CreateTopicRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateTopicRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAuthorizer
#[derive(Debug, PartialEq)]
pub enum DeleteAuthorizerError {
    ///<p>You can't delete the resource because it is attached to one or more resources.</p>
    DeleteConflict(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteAuthorizerError {
    pub fn from_body(body: &str) -> DeleteAuthorizerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DeleteConflictException" => {
                        DeleteAuthorizerError::DeleteConflict(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        DeleteAuthorizerError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteAuthorizerError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteAuthorizerError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteAuthorizerError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeleteAuthorizerError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteAuthorizerError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteAuthorizerError::Validation(error_message.to_string())
                    }
                    _ => DeleteAuthorizerError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteAuthorizerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteAuthorizerError {
    fn from(err: serde_json::error::Error) -> DeleteAuthorizerError {
        DeleteAuthorizerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAuthorizerError {
    fn from(err: CredentialsError) -> DeleteAuthorizerError {
        DeleteAuthorizerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAuthorizerError {
    fn from(err: HttpDispatchError) -> DeleteAuthorizerError {
        DeleteAuthorizerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAuthorizerError {
    fn from(err: io::Error) -> DeleteAuthorizerError {
        DeleteAuthorizerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAuthorizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAuthorizerError {
    fn description(&self) -> &str {
        match *self {
            DeleteAuthorizerError::DeleteConflict(ref cause) => cause,
            DeleteAuthorizerError::InternalFailure(ref cause) => cause,
            DeleteAuthorizerError::InvalidRequest(ref cause) => cause,
            DeleteAuthorizerError::ResourceNotFound(ref cause) => cause,
            DeleteAuthorizerError::ServiceUnavailable(ref cause) => cause,
            DeleteAuthorizerError::Throttling(ref cause) => cause,
            DeleteAuthorizerError::Unauthorized(ref cause) => cause,
            DeleteAuthorizerError::Validation(ref cause) => cause,
            DeleteAuthorizerError::Credentials(ref err) => err.description(),
            DeleteAuthorizerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteAuthorizerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCACertificate
#[derive(Debug, PartialEq)]
pub enum DeleteCACertificateError {
    ///<p>The certificate operation is not allowed.</p>
    CertificateState(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteCACertificateError {
    pub fn from_body(body: &str) -> DeleteCACertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CertificateStateException" => {
                        DeleteCACertificateError::CertificateState(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        DeleteCACertificateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteCACertificateError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteCACertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteCACertificateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeleteCACertificateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteCACertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteCACertificateError::Validation(error_message.to_string())
                    }
                    _ => DeleteCACertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteCACertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteCACertificateError {
    fn from(err: serde_json::error::Error) -> DeleteCACertificateError {
        DeleteCACertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteCACertificateError {
    fn from(err: CredentialsError) -> DeleteCACertificateError {
        DeleteCACertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCACertificateError {
    fn from(err: HttpDispatchError) -> DeleteCACertificateError {
        DeleteCACertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteCACertificateError {
    fn from(err: io::Error) -> DeleteCACertificateError {
        DeleteCACertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteCACertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCACertificateError {
    fn description(&self) -> &str {
        match *self {
            DeleteCACertificateError::CertificateState(ref cause) => cause,
            DeleteCACertificateError::InternalFailure(ref cause) => cause,
            DeleteCACertificateError::InvalidRequest(ref cause) => cause,
            DeleteCACertificateError::ResourceNotFound(ref cause) => cause,
            DeleteCACertificateError::ServiceUnavailable(ref cause) => cause,
            DeleteCACertificateError::Throttling(ref cause) => cause,
            DeleteCACertificateError::Unauthorized(ref cause) => cause,
            DeleteCACertificateError::Validation(ref cause) => cause,
            DeleteCACertificateError::Credentials(ref err) => err.description(),
            DeleteCACertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteCACertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCertificate
#[derive(Debug, PartialEq)]
pub enum DeleteCertificateError {
    ///<p>The certificate operation is not allowed.</p>
    CertificateState(String),
    ///<p>You can't delete the resource because it is attached to one or more resources.</p>
    DeleteConflict(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteCertificateError {
    pub fn from_body(body: &str) -> DeleteCertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CertificateStateException" => {
                        DeleteCertificateError::CertificateState(String::from(error_message))
                    }
                    "DeleteConflictException" => {
                        DeleteCertificateError::DeleteConflict(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        DeleteCertificateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteCertificateError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteCertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteCertificateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeleteCertificateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteCertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteCertificateError::Validation(error_message.to_string())
                    }
                    _ => DeleteCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteCertificateError {
    fn from(err: serde_json::error::Error) -> DeleteCertificateError {
        DeleteCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteCertificateError {
    fn from(err: CredentialsError) -> DeleteCertificateError {
        DeleteCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCertificateError {
    fn from(err: HttpDispatchError) -> DeleteCertificateError {
        DeleteCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteCertificateError {
    fn from(err: io::Error) -> DeleteCertificateError {
        DeleteCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCertificateError {
    fn description(&self) -> &str {
        match *self {
            DeleteCertificateError::CertificateState(ref cause) => cause,
            DeleteCertificateError::DeleteConflict(ref cause) => cause,
            DeleteCertificateError::InternalFailure(ref cause) => cause,
            DeleteCertificateError::InvalidRequest(ref cause) => cause,
            DeleteCertificateError::ResourceNotFound(ref cause) => cause,
            DeleteCertificateError::ServiceUnavailable(ref cause) => cause,
            DeleteCertificateError::Throttling(ref cause) => cause,
            DeleteCertificateError::Unauthorized(ref cause) => cause,
            DeleteCertificateError::Validation(ref cause) => cause,
            DeleteCertificateError::Credentials(ref err) => err.description(),
            DeleteCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteOTAUpdate
#[derive(Debug, PartialEq)]
pub enum DeleteOTAUpdateError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteOTAUpdateError {
    pub fn from_body(body: &str) -> DeleteOTAUpdateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DeleteOTAUpdateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteOTAUpdateError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteOTAUpdateError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteOTAUpdateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeleteOTAUpdateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteOTAUpdateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteOTAUpdateError::Validation(error_message.to_string())
                    }
                    _ => DeleteOTAUpdateError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteOTAUpdateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteOTAUpdateError {
    fn from(err: serde_json::error::Error) -> DeleteOTAUpdateError {
        DeleteOTAUpdateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteOTAUpdateError {
    fn from(err: CredentialsError) -> DeleteOTAUpdateError {
        DeleteOTAUpdateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteOTAUpdateError {
    fn from(err: HttpDispatchError) -> DeleteOTAUpdateError {
        DeleteOTAUpdateError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteOTAUpdateError {
    fn from(err: io::Error) -> DeleteOTAUpdateError {
        DeleteOTAUpdateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteOTAUpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteOTAUpdateError {
    fn description(&self) -> &str {
        match *self {
            DeleteOTAUpdateError::InternalFailure(ref cause) => cause,
            DeleteOTAUpdateError::InvalidRequest(ref cause) => cause,
            DeleteOTAUpdateError::ResourceNotFound(ref cause) => cause,
            DeleteOTAUpdateError::ServiceUnavailable(ref cause) => cause,
            DeleteOTAUpdateError::Throttling(ref cause) => cause,
            DeleteOTAUpdateError::Unauthorized(ref cause) => cause,
            DeleteOTAUpdateError::Validation(ref cause) => cause,
            DeleteOTAUpdateError::Credentials(ref err) => err.description(),
            DeleteOTAUpdateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteOTAUpdateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePolicy
#[derive(Debug, PartialEq)]
pub enum DeletePolicyError {
    ///<p>You can't delete the resource because it is attached to one or more resources.</p>
    DeleteConflict(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeletePolicyError {
    pub fn from_body(body: &str) -> DeletePolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DeleteConflictException" => {
                        DeletePolicyError::DeleteConflict(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        DeletePolicyError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeletePolicyError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeletePolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeletePolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeletePolicyError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeletePolicyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeletePolicyError::Validation(error_message.to_string())
                    }
                    _ => DeletePolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeletePolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeletePolicyError {
    fn from(err: serde_json::error::Error) -> DeletePolicyError {
        DeletePolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeletePolicyError {
    fn from(err: CredentialsError) -> DeletePolicyError {
        DeletePolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeletePolicyError {
    fn from(err: HttpDispatchError) -> DeletePolicyError {
        DeletePolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeletePolicyError {
    fn from(err: io::Error) -> DeletePolicyError {
        DeletePolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeletePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePolicyError {
    fn description(&self) -> &str {
        match *self {
            DeletePolicyError::DeleteConflict(ref cause) => cause,
            DeletePolicyError::InternalFailure(ref cause) => cause,
            DeletePolicyError::InvalidRequest(ref cause) => cause,
            DeletePolicyError::ResourceNotFound(ref cause) => cause,
            DeletePolicyError::ServiceUnavailable(ref cause) => cause,
            DeletePolicyError::Throttling(ref cause) => cause,
            DeletePolicyError::Unauthorized(ref cause) => cause,
            DeletePolicyError::Validation(ref cause) => cause,
            DeletePolicyError::Credentials(ref err) => err.description(),
            DeletePolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeletePolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePolicyVersion
#[derive(Debug, PartialEq)]
pub enum DeletePolicyVersionError {
    ///<p>You can't delete the resource because it is attached to one or more resources.</p>
    DeleteConflict(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeletePolicyVersionError {
    pub fn from_body(body: &str) -> DeletePolicyVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DeleteConflictException" => {
                        DeletePolicyVersionError::DeleteConflict(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        DeletePolicyVersionError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeletePolicyVersionError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeletePolicyVersionError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeletePolicyVersionError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeletePolicyVersionError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeletePolicyVersionError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeletePolicyVersionError::Validation(error_message.to_string())
                    }
                    _ => DeletePolicyVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeletePolicyVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeletePolicyVersionError {
    fn from(err: serde_json::error::Error) -> DeletePolicyVersionError {
        DeletePolicyVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeletePolicyVersionError {
    fn from(err: CredentialsError) -> DeletePolicyVersionError {
        DeletePolicyVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeletePolicyVersionError {
    fn from(err: HttpDispatchError) -> DeletePolicyVersionError {
        DeletePolicyVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeletePolicyVersionError {
    fn from(err: io::Error) -> DeletePolicyVersionError {
        DeletePolicyVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeletePolicyVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePolicyVersionError {
    fn description(&self) -> &str {
        match *self {
            DeletePolicyVersionError::DeleteConflict(ref cause) => cause,
            DeletePolicyVersionError::InternalFailure(ref cause) => cause,
            DeletePolicyVersionError::InvalidRequest(ref cause) => cause,
            DeletePolicyVersionError::ResourceNotFound(ref cause) => cause,
            DeletePolicyVersionError::ServiceUnavailable(ref cause) => cause,
            DeletePolicyVersionError::Throttling(ref cause) => cause,
            DeletePolicyVersionError::Unauthorized(ref cause) => cause,
            DeletePolicyVersionError::Validation(ref cause) => cause,
            DeletePolicyVersionError::Credentials(ref err) => err.description(),
            DeletePolicyVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeletePolicyVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRegistrationCode
#[derive(Debug, PartialEq)]
pub enum DeleteRegistrationCodeError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteRegistrationCodeError {
    pub fn from_body(body: &str) -> DeleteRegistrationCodeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DeleteRegistrationCodeError::InternalFailure(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteRegistrationCodeError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteRegistrationCodeError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeleteRegistrationCodeError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteRegistrationCodeError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteRegistrationCodeError::Validation(error_message.to_string())
                    }
                    _ => DeleteRegistrationCodeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteRegistrationCodeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteRegistrationCodeError {
    fn from(err: serde_json::error::Error) -> DeleteRegistrationCodeError {
        DeleteRegistrationCodeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteRegistrationCodeError {
    fn from(err: CredentialsError) -> DeleteRegistrationCodeError {
        DeleteRegistrationCodeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteRegistrationCodeError {
    fn from(err: HttpDispatchError) -> DeleteRegistrationCodeError {
        DeleteRegistrationCodeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteRegistrationCodeError {
    fn from(err: io::Error) -> DeleteRegistrationCodeError {
        DeleteRegistrationCodeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteRegistrationCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRegistrationCodeError {
    fn description(&self) -> &str {
        match *self {
            DeleteRegistrationCodeError::InternalFailure(ref cause) => cause,
            DeleteRegistrationCodeError::ResourceNotFound(ref cause) => cause,
            DeleteRegistrationCodeError::ServiceUnavailable(ref cause) => cause,
            DeleteRegistrationCodeError::Throttling(ref cause) => cause,
            DeleteRegistrationCodeError::Unauthorized(ref cause) => cause,
            DeleteRegistrationCodeError::Validation(ref cause) => cause,
            DeleteRegistrationCodeError::Credentials(ref err) => err.description(),
            DeleteRegistrationCodeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteRegistrationCodeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRoleAlias
#[derive(Debug, PartialEq)]
pub enum DeleteRoleAliasError {
    ///<p>You can't delete the resource because it is attached to one or more resources.</p>
    DeleteConflict(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteRoleAliasError {
    pub fn from_body(body: &str) -> DeleteRoleAliasError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DeleteConflictException" => {
                        DeleteRoleAliasError::DeleteConflict(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        DeleteRoleAliasError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteRoleAliasError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteRoleAliasError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteRoleAliasError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeleteRoleAliasError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteRoleAliasError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteRoleAliasError::Validation(error_message.to_string())
                    }
                    _ => DeleteRoleAliasError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteRoleAliasError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteRoleAliasError {
    fn from(err: serde_json::error::Error) -> DeleteRoleAliasError {
        DeleteRoleAliasError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteRoleAliasError {
    fn from(err: CredentialsError) -> DeleteRoleAliasError {
        DeleteRoleAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteRoleAliasError {
    fn from(err: HttpDispatchError) -> DeleteRoleAliasError {
        DeleteRoleAliasError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteRoleAliasError {
    fn from(err: io::Error) -> DeleteRoleAliasError {
        DeleteRoleAliasError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteRoleAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRoleAliasError {
    fn description(&self) -> &str {
        match *self {
            DeleteRoleAliasError::DeleteConflict(ref cause) => cause,
            DeleteRoleAliasError::InternalFailure(ref cause) => cause,
            DeleteRoleAliasError::InvalidRequest(ref cause) => cause,
            DeleteRoleAliasError::ResourceNotFound(ref cause) => cause,
            DeleteRoleAliasError::ServiceUnavailable(ref cause) => cause,
            DeleteRoleAliasError::Throttling(ref cause) => cause,
            DeleteRoleAliasError::Unauthorized(ref cause) => cause,
            DeleteRoleAliasError::Validation(ref cause) => cause,
            DeleteRoleAliasError::Credentials(ref err) => err.description(),
            DeleteRoleAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteRoleAliasError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteStream
#[derive(Debug, PartialEq)]
pub enum DeleteStreamError {
    ///<p>You can't delete the resource because it is attached to one or more resources.</p>
    DeleteConflict(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteStreamError {
    pub fn from_body(body: &str) -> DeleteStreamError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "DeleteConflictException" => {
                        DeleteStreamError::DeleteConflict(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        DeleteStreamError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteStreamError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteStreamError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteStreamError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeleteStreamError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteStreamError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteStreamError::Validation(error_message.to_string())
                    }
                    _ => DeleteStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteStreamError {
    fn from(err: serde_json::error::Error) -> DeleteStreamError {
        DeleteStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteStreamError {
    fn from(err: CredentialsError) -> DeleteStreamError {
        DeleteStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteStreamError {
    fn from(err: HttpDispatchError) -> DeleteStreamError {
        DeleteStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteStreamError {
    fn from(err: io::Error) -> DeleteStreamError {
        DeleteStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteStreamError {
    fn description(&self) -> &str {
        match *self {
            DeleteStreamError::DeleteConflict(ref cause) => cause,
            DeleteStreamError::InternalFailure(ref cause) => cause,
            DeleteStreamError::InvalidRequest(ref cause) => cause,
            DeleteStreamError::ResourceNotFound(ref cause) => cause,
            DeleteStreamError::ServiceUnavailable(ref cause) => cause,
            DeleteStreamError::Throttling(ref cause) => cause,
            DeleteStreamError::Unauthorized(ref cause) => cause,
            DeleteStreamError::Validation(ref cause) => cause,
            DeleteStreamError::Credentials(ref err) => err.description(),
            DeleteStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteStreamError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteThing
#[derive(Debug, PartialEq)]
pub enum DeleteThingError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    ///<p>An exception thrown when the version of a thing passed to a command is different than the version specified with the --version parameter.</p>
    VersionConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteThingError {
    pub fn from_body(body: &str) -> DeleteThingError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DeleteThingError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteThingError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteThingError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteThingError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeleteThingError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteThingError::Unauthorized(String::from(error_message))
                    }
                    "VersionConflictException" => {
                        DeleteThingError::VersionConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteThingError::Validation(error_message.to_string())
                    }
                    _ => DeleteThingError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteThingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteThingError {
    fn from(err: serde_json::error::Error) -> DeleteThingError {
        DeleteThingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteThingError {
    fn from(err: CredentialsError) -> DeleteThingError {
        DeleteThingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteThingError {
    fn from(err: HttpDispatchError) -> DeleteThingError {
        DeleteThingError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteThingError {
    fn from(err: io::Error) -> DeleteThingError {
        DeleteThingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteThingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteThingError {
    fn description(&self) -> &str {
        match *self {
            DeleteThingError::InternalFailure(ref cause) => cause,
            DeleteThingError::InvalidRequest(ref cause) => cause,
            DeleteThingError::ResourceNotFound(ref cause) => cause,
            DeleteThingError::ServiceUnavailable(ref cause) => cause,
            DeleteThingError::Throttling(ref cause) => cause,
            DeleteThingError::Unauthorized(ref cause) => cause,
            DeleteThingError::VersionConflict(ref cause) => cause,
            DeleteThingError::Validation(ref cause) => cause,
            DeleteThingError::Credentials(ref err) => err.description(),
            DeleteThingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteThingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteThingGroup
#[derive(Debug, PartialEq)]
pub enum DeleteThingGroupError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>An exception thrown when the version of a thing passed to a command is different than the version specified with the --version parameter.</p>
    VersionConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteThingGroupError {
    pub fn from_body(body: &str) -> DeleteThingGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DeleteThingGroupError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteThingGroupError::InvalidRequest(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeleteThingGroupError::Throttling(String::from(error_message))
                    }
                    "VersionConflictException" => {
                        DeleteThingGroupError::VersionConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteThingGroupError::Validation(error_message.to_string())
                    }
                    _ => DeleteThingGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteThingGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteThingGroupError {
    fn from(err: serde_json::error::Error) -> DeleteThingGroupError {
        DeleteThingGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteThingGroupError {
    fn from(err: CredentialsError) -> DeleteThingGroupError {
        DeleteThingGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteThingGroupError {
    fn from(err: HttpDispatchError) -> DeleteThingGroupError {
        DeleteThingGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteThingGroupError {
    fn from(err: io::Error) -> DeleteThingGroupError {
        DeleteThingGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteThingGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteThingGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteThingGroupError::InternalFailure(ref cause) => cause,
            DeleteThingGroupError::InvalidRequest(ref cause) => cause,
            DeleteThingGroupError::Throttling(ref cause) => cause,
            DeleteThingGroupError::VersionConflict(ref cause) => cause,
            DeleteThingGroupError::Validation(ref cause) => cause,
            DeleteThingGroupError::Credentials(ref err) => err.description(),
            DeleteThingGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteThingGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteThingType
#[derive(Debug, PartialEq)]
pub enum DeleteThingTypeError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteThingTypeError {
    pub fn from_body(body: &str) -> DeleteThingTypeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DeleteThingTypeError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteThingTypeError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteThingTypeError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteThingTypeError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeleteThingTypeError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteThingTypeError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteThingTypeError::Validation(error_message.to_string())
                    }
                    _ => DeleteThingTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteThingTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteThingTypeError {
    fn from(err: serde_json::error::Error) -> DeleteThingTypeError {
        DeleteThingTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteThingTypeError {
    fn from(err: CredentialsError) -> DeleteThingTypeError {
        DeleteThingTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteThingTypeError {
    fn from(err: HttpDispatchError) -> DeleteThingTypeError {
        DeleteThingTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteThingTypeError {
    fn from(err: io::Error) -> DeleteThingTypeError {
        DeleteThingTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteThingTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteThingTypeError {
    fn description(&self) -> &str {
        match *self {
            DeleteThingTypeError::InternalFailure(ref cause) => cause,
            DeleteThingTypeError::InvalidRequest(ref cause) => cause,
            DeleteThingTypeError::ResourceNotFound(ref cause) => cause,
            DeleteThingTypeError::ServiceUnavailable(ref cause) => cause,
            DeleteThingTypeError::Throttling(ref cause) => cause,
            DeleteThingTypeError::Unauthorized(ref cause) => cause,
            DeleteThingTypeError::Validation(ref cause) => cause,
            DeleteThingTypeError::Credentials(ref err) => err.description(),
            DeleteThingTypeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteThingTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTopicRule
#[derive(Debug, PartialEq)]
pub enum DeleteTopicRuleError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteTopicRuleError {
    pub fn from_body(body: &str) -> DeleteTopicRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        DeleteTopicRuleError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteTopicRuleError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteTopicRuleError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeleteTopicRuleError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteTopicRuleError::Validation(error_message.to_string())
                    }
                    _ => DeleteTopicRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteTopicRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteTopicRuleError {
    fn from(err: serde_json::error::Error) -> DeleteTopicRuleError {
        DeleteTopicRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteTopicRuleError {
    fn from(err: CredentialsError) -> DeleteTopicRuleError {
        DeleteTopicRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTopicRuleError {
    fn from(err: HttpDispatchError) -> DeleteTopicRuleError {
        DeleteTopicRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTopicRuleError {
    fn from(err: io::Error) -> DeleteTopicRuleError {
        DeleteTopicRuleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTopicRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTopicRuleError {
    fn description(&self) -> &str {
        match *self {
            DeleteTopicRuleError::Internal(ref cause) => cause,
            DeleteTopicRuleError::InvalidRequest(ref cause) => cause,
            DeleteTopicRuleError::ServiceUnavailable(ref cause) => cause,
            DeleteTopicRuleError::Unauthorized(ref cause) => cause,
            DeleteTopicRuleError::Validation(ref cause) => cause,
            DeleteTopicRuleError::Credentials(ref err) => err.description(),
            DeleteTopicRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteTopicRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteV2LoggingLevel
#[derive(Debug, PartialEq)]
pub enum DeleteV2LoggingLevelError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteV2LoggingLevelError {
    pub fn from_body(body: &str) -> DeleteV2LoggingLevelError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        DeleteV2LoggingLevelError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeleteV2LoggingLevelError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeleteV2LoggingLevelError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteV2LoggingLevelError::Validation(error_message.to_string())
                    }
                    _ => DeleteV2LoggingLevelError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteV2LoggingLevelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteV2LoggingLevelError {
    fn from(err: serde_json::error::Error) -> DeleteV2LoggingLevelError {
        DeleteV2LoggingLevelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteV2LoggingLevelError {
    fn from(err: CredentialsError) -> DeleteV2LoggingLevelError {
        DeleteV2LoggingLevelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteV2LoggingLevelError {
    fn from(err: HttpDispatchError) -> DeleteV2LoggingLevelError {
        DeleteV2LoggingLevelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteV2LoggingLevelError {
    fn from(err: io::Error) -> DeleteV2LoggingLevelError {
        DeleteV2LoggingLevelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteV2LoggingLevelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteV2LoggingLevelError {
    fn description(&self) -> &str {
        match *self {
            DeleteV2LoggingLevelError::Internal(ref cause) => cause,
            DeleteV2LoggingLevelError::InvalidRequest(ref cause) => cause,
            DeleteV2LoggingLevelError::ServiceUnavailable(ref cause) => cause,
            DeleteV2LoggingLevelError::Validation(ref cause) => cause,
            DeleteV2LoggingLevelError::Credentials(ref err) => err.description(),
            DeleteV2LoggingLevelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteV2LoggingLevelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeprecateThingType
#[derive(Debug, PartialEq)]
pub enum DeprecateThingTypeError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeprecateThingTypeError {
    pub fn from_body(body: &str) -> DeprecateThingTypeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DeprecateThingTypeError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DeprecateThingTypeError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeprecateThingTypeError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DeprecateThingTypeError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DeprecateThingTypeError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DeprecateThingTypeError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeprecateThingTypeError::Validation(error_message.to_string())
                    }
                    _ => DeprecateThingTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeprecateThingTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeprecateThingTypeError {
    fn from(err: serde_json::error::Error) -> DeprecateThingTypeError {
        DeprecateThingTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeprecateThingTypeError {
    fn from(err: CredentialsError) -> DeprecateThingTypeError {
        DeprecateThingTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeprecateThingTypeError {
    fn from(err: HttpDispatchError) -> DeprecateThingTypeError {
        DeprecateThingTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeprecateThingTypeError {
    fn from(err: io::Error) -> DeprecateThingTypeError {
        DeprecateThingTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeprecateThingTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeprecateThingTypeError {
    fn description(&self) -> &str {
        match *self {
            DeprecateThingTypeError::InternalFailure(ref cause) => cause,
            DeprecateThingTypeError::InvalidRequest(ref cause) => cause,
            DeprecateThingTypeError::ResourceNotFound(ref cause) => cause,
            DeprecateThingTypeError::ServiceUnavailable(ref cause) => cause,
            DeprecateThingTypeError::Throttling(ref cause) => cause,
            DeprecateThingTypeError::Unauthorized(ref cause) => cause,
            DeprecateThingTypeError::Validation(ref cause) => cause,
            DeprecateThingTypeError::Credentials(ref err) => err.description(),
            DeprecateThingTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeprecateThingTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAuthorizer
#[derive(Debug, PartialEq)]
pub enum DescribeAuthorizerError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAuthorizerError {
    pub fn from_body(body: &str) -> DescribeAuthorizerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DescribeAuthorizerError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeAuthorizerError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeAuthorizerError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeAuthorizerError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DescribeAuthorizerError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeAuthorizerError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeAuthorizerError::Validation(error_message.to_string())
                    }
                    _ => DescribeAuthorizerError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAuthorizerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAuthorizerError {
    fn from(err: serde_json::error::Error) -> DescribeAuthorizerError {
        DescribeAuthorizerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAuthorizerError {
    fn from(err: CredentialsError) -> DescribeAuthorizerError {
        DescribeAuthorizerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAuthorizerError {
    fn from(err: HttpDispatchError) -> DescribeAuthorizerError {
        DescribeAuthorizerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAuthorizerError {
    fn from(err: io::Error) -> DescribeAuthorizerError {
        DescribeAuthorizerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAuthorizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAuthorizerError {
    fn description(&self) -> &str {
        match *self {
            DescribeAuthorizerError::InternalFailure(ref cause) => cause,
            DescribeAuthorizerError::InvalidRequest(ref cause) => cause,
            DescribeAuthorizerError::ResourceNotFound(ref cause) => cause,
            DescribeAuthorizerError::ServiceUnavailable(ref cause) => cause,
            DescribeAuthorizerError::Throttling(ref cause) => cause,
            DescribeAuthorizerError::Unauthorized(ref cause) => cause,
            DescribeAuthorizerError::Validation(ref cause) => cause,
            DescribeAuthorizerError::Credentials(ref err) => err.description(),
            DescribeAuthorizerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAuthorizerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCACertificate
#[derive(Debug, PartialEq)]
pub enum DescribeCACertificateError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeCACertificateError {
    pub fn from_body(body: &str) -> DescribeCACertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DescribeCACertificateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeCACertificateError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeCACertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeCACertificateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DescribeCACertificateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeCACertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeCACertificateError::Validation(error_message.to_string())
                    }
                    _ => DescribeCACertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCACertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeCACertificateError {
    fn from(err: serde_json::error::Error) -> DescribeCACertificateError {
        DescribeCACertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeCACertificateError {
    fn from(err: CredentialsError) -> DescribeCACertificateError {
        DescribeCACertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCACertificateError {
    fn from(err: HttpDispatchError) -> DescribeCACertificateError {
        DescribeCACertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeCACertificateError {
    fn from(err: io::Error) -> DescribeCACertificateError {
        DescribeCACertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeCACertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCACertificateError {
    fn description(&self) -> &str {
        match *self {
            DescribeCACertificateError::InternalFailure(ref cause) => cause,
            DescribeCACertificateError::InvalidRequest(ref cause) => cause,
            DescribeCACertificateError::ResourceNotFound(ref cause) => cause,
            DescribeCACertificateError::ServiceUnavailable(ref cause) => cause,
            DescribeCACertificateError::Throttling(ref cause) => cause,
            DescribeCACertificateError::Unauthorized(ref cause) => cause,
            DescribeCACertificateError::Validation(ref cause) => cause,
            DescribeCACertificateError::Credentials(ref err) => err.description(),
            DescribeCACertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeCACertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCertificate
#[derive(Debug, PartialEq)]
pub enum DescribeCertificateError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeCertificateError {
    pub fn from_body(body: &str) -> DescribeCertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DescribeCertificateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeCertificateError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeCertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeCertificateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DescribeCertificateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeCertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeCertificateError::Validation(error_message.to_string())
                    }
                    _ => DescribeCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeCertificateError {
    fn from(err: serde_json::error::Error) -> DescribeCertificateError {
        DescribeCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeCertificateError {
    fn from(err: CredentialsError) -> DescribeCertificateError {
        DescribeCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCertificateError {
    fn from(err: HttpDispatchError) -> DescribeCertificateError {
        DescribeCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeCertificateError {
    fn from(err: io::Error) -> DescribeCertificateError {
        DescribeCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCertificateError {
    fn description(&self) -> &str {
        match *self {
            DescribeCertificateError::InternalFailure(ref cause) => cause,
            DescribeCertificateError::InvalidRequest(ref cause) => cause,
            DescribeCertificateError::ResourceNotFound(ref cause) => cause,
            DescribeCertificateError::ServiceUnavailable(ref cause) => cause,
            DescribeCertificateError::Throttling(ref cause) => cause,
            DescribeCertificateError::Unauthorized(ref cause) => cause,
            DescribeCertificateError::Validation(ref cause) => cause,
            DescribeCertificateError::Credentials(ref err) => err.description(),
            DescribeCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDefaultAuthorizer
#[derive(Debug, PartialEq)]
pub enum DescribeDefaultAuthorizerError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeDefaultAuthorizerError {
    pub fn from_body(body: &str) -> DescribeDefaultAuthorizerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DescribeDefaultAuthorizerError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeDefaultAuthorizerError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeDefaultAuthorizerError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ServiceUnavailableException" => {
                        DescribeDefaultAuthorizerError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ThrottlingException" => {
                        DescribeDefaultAuthorizerError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeDefaultAuthorizerError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeDefaultAuthorizerError::Validation(error_message.to_string())
                    }
                    _ => DescribeDefaultAuthorizerError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeDefaultAuthorizerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeDefaultAuthorizerError {
    fn from(err: serde_json::error::Error) -> DescribeDefaultAuthorizerError {
        DescribeDefaultAuthorizerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDefaultAuthorizerError {
    fn from(err: CredentialsError) -> DescribeDefaultAuthorizerError {
        DescribeDefaultAuthorizerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDefaultAuthorizerError {
    fn from(err: HttpDispatchError) -> DescribeDefaultAuthorizerError {
        DescribeDefaultAuthorizerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDefaultAuthorizerError {
    fn from(err: io::Error) -> DescribeDefaultAuthorizerError {
        DescribeDefaultAuthorizerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDefaultAuthorizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDefaultAuthorizerError {
    fn description(&self) -> &str {
        match *self {
            DescribeDefaultAuthorizerError::InternalFailure(ref cause) => cause,
            DescribeDefaultAuthorizerError::InvalidRequest(ref cause) => cause,
            DescribeDefaultAuthorizerError::ResourceNotFound(ref cause) => cause,
            DescribeDefaultAuthorizerError::ServiceUnavailable(ref cause) => cause,
            DescribeDefaultAuthorizerError::Throttling(ref cause) => cause,
            DescribeDefaultAuthorizerError::Unauthorized(ref cause) => cause,
            DescribeDefaultAuthorizerError::Validation(ref cause) => cause,
            DescribeDefaultAuthorizerError::Credentials(ref err) => err.description(),
            DescribeDefaultAuthorizerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDefaultAuthorizerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEndpoint
#[derive(Debug, PartialEq)]
pub enum DescribeEndpointError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeEndpointError {
    pub fn from_body(body: &str) -> DescribeEndpointError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DescribeEndpointError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeEndpointError::InvalidRequest(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DescribeEndpointError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeEndpointError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeEndpointError::Validation(error_message.to_string())
                    }
                    _ => DescribeEndpointError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEndpointError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEndpointError {
    fn from(err: serde_json::error::Error) -> DescribeEndpointError {
        DescribeEndpointError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEndpointError {
    fn from(err: CredentialsError) -> DescribeEndpointError {
        DescribeEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEndpointError {
    fn from(err: HttpDispatchError) -> DescribeEndpointError {
        DescribeEndpointError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEndpointError {
    fn from(err: io::Error) -> DescribeEndpointError {
        DescribeEndpointError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEndpointError {
    fn description(&self) -> &str {
        match *self {
            DescribeEndpointError::InternalFailure(ref cause) => cause,
            DescribeEndpointError::InvalidRequest(ref cause) => cause,
            DescribeEndpointError::Throttling(ref cause) => cause,
            DescribeEndpointError::Unauthorized(ref cause) => cause,
            DescribeEndpointError::Validation(ref cause) => cause,
            DescribeEndpointError::Credentials(ref err) => err.description(),
            DescribeEndpointError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeEndpointError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEventConfigurations
#[derive(Debug, PartialEq)]
pub enum DescribeEventConfigurationsError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeEventConfigurationsError {
    pub fn from_body(body: &str) -> DescribeEventConfigurationsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DescribeEventConfigurationsError::InternalFailure(String::from(
                            error_message,
                        ))
                    }
                    "ThrottlingException" => {
                        DescribeEventConfigurationsError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeEventConfigurationsError::Validation(error_message.to_string())
                    }
                    _ => DescribeEventConfigurationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEventConfigurationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEventConfigurationsError {
    fn from(err: serde_json::error::Error) -> DescribeEventConfigurationsError {
        DescribeEventConfigurationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEventConfigurationsError {
    fn from(err: CredentialsError) -> DescribeEventConfigurationsError {
        DescribeEventConfigurationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEventConfigurationsError {
    fn from(err: HttpDispatchError) -> DescribeEventConfigurationsError {
        DescribeEventConfigurationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeEventConfigurationsError {
    fn from(err: io::Error) -> DescribeEventConfigurationsError {
        DescribeEventConfigurationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeEventConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEventConfigurationsError::InternalFailure(ref cause) => cause,
            DescribeEventConfigurationsError::Throttling(ref cause) => cause,
            DescribeEventConfigurationsError::Validation(ref cause) => cause,
            DescribeEventConfigurationsError::Credentials(ref err) => err.description(),
            DescribeEventConfigurationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEventConfigurationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeIndex
#[derive(Debug, PartialEq)]
pub enum DescribeIndexError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeIndexError {
    pub fn from_body(body: &str) -> DescribeIndexError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DescribeIndexError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeIndexError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeIndexError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeIndexError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DescribeIndexError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeIndexError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeIndexError::Validation(error_message.to_string())
                    }
                    _ => DescribeIndexError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeIndexError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeIndexError {
    fn from(err: serde_json::error::Error) -> DescribeIndexError {
        DescribeIndexError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeIndexError {
    fn from(err: CredentialsError) -> DescribeIndexError {
        DescribeIndexError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeIndexError {
    fn from(err: HttpDispatchError) -> DescribeIndexError {
        DescribeIndexError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeIndexError {
    fn from(err: io::Error) -> DescribeIndexError {
        DescribeIndexError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeIndexError {
    fn description(&self) -> &str {
        match *self {
            DescribeIndexError::InternalFailure(ref cause) => cause,
            DescribeIndexError::InvalidRequest(ref cause) => cause,
            DescribeIndexError::ResourceNotFound(ref cause) => cause,
            DescribeIndexError::ServiceUnavailable(ref cause) => cause,
            DescribeIndexError::Throttling(ref cause) => cause,
            DescribeIndexError::Unauthorized(ref cause) => cause,
            DescribeIndexError::Validation(ref cause) => cause,
            DescribeIndexError::Credentials(ref err) => err.description(),
            DescribeIndexError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeIndexError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeJob
#[derive(Debug, PartialEq)]
pub enum DescribeJobError {
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeJobError {
    pub fn from_body(body: &str) -> DescribeJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidRequestException" => {
                        DescribeJobError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeJobError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeJobError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DescribeJobError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeJobError::Validation(error_message.to_string())
                    }
                    _ => DescribeJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeJobError {
    fn from(err: serde_json::error::Error) -> DescribeJobError {
        DescribeJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeJobError {
    fn from(err: CredentialsError) -> DescribeJobError {
        DescribeJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeJobError {
    fn from(err: HttpDispatchError) -> DescribeJobError {
        DescribeJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeJobError {
    fn from(err: io::Error) -> DescribeJobError {
        DescribeJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeJobError {
    fn description(&self) -> &str {
        match *self {
            DescribeJobError::InvalidRequest(ref cause) => cause,
            DescribeJobError::ResourceNotFound(ref cause) => cause,
            DescribeJobError::ServiceUnavailable(ref cause) => cause,
            DescribeJobError::Throttling(ref cause) => cause,
            DescribeJobError::Validation(ref cause) => cause,
            DescribeJobError::Credentials(ref err) => err.description(),
            DescribeJobError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeJobExecution
#[derive(Debug, PartialEq)]
pub enum DescribeJobExecutionError {
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeJobExecutionError {
    pub fn from_body(body: &str) -> DescribeJobExecutionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidRequestException" => {
                        DescribeJobExecutionError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeJobExecutionError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeJobExecutionError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DescribeJobExecutionError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeJobExecutionError::Validation(error_message.to_string())
                    }
                    _ => DescribeJobExecutionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeJobExecutionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeJobExecutionError {
    fn from(err: serde_json::error::Error) -> DescribeJobExecutionError {
        DescribeJobExecutionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeJobExecutionError {
    fn from(err: CredentialsError) -> DescribeJobExecutionError {
        DescribeJobExecutionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeJobExecutionError {
    fn from(err: HttpDispatchError) -> DescribeJobExecutionError {
        DescribeJobExecutionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeJobExecutionError {
    fn from(err: io::Error) -> DescribeJobExecutionError {
        DescribeJobExecutionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeJobExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeJobExecutionError {
    fn description(&self) -> &str {
        match *self {
            DescribeJobExecutionError::InvalidRequest(ref cause) => cause,
            DescribeJobExecutionError::ResourceNotFound(ref cause) => cause,
            DescribeJobExecutionError::ServiceUnavailable(ref cause) => cause,
            DescribeJobExecutionError::Throttling(ref cause) => cause,
            DescribeJobExecutionError::Validation(ref cause) => cause,
            DescribeJobExecutionError::Credentials(ref err) => err.description(),
            DescribeJobExecutionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeJobExecutionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeRoleAlias
#[derive(Debug, PartialEq)]
pub enum DescribeRoleAliasError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeRoleAliasError {
    pub fn from_body(body: &str) -> DescribeRoleAliasError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DescribeRoleAliasError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeRoleAliasError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeRoleAliasError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeRoleAliasError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DescribeRoleAliasError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeRoleAliasError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeRoleAliasError::Validation(error_message.to_string())
                    }
                    _ => DescribeRoleAliasError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeRoleAliasError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeRoleAliasError {
    fn from(err: serde_json::error::Error) -> DescribeRoleAliasError {
        DescribeRoleAliasError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeRoleAliasError {
    fn from(err: CredentialsError) -> DescribeRoleAliasError {
        DescribeRoleAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeRoleAliasError {
    fn from(err: HttpDispatchError) -> DescribeRoleAliasError {
        DescribeRoleAliasError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeRoleAliasError {
    fn from(err: io::Error) -> DescribeRoleAliasError {
        DescribeRoleAliasError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeRoleAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeRoleAliasError {
    fn description(&self) -> &str {
        match *self {
            DescribeRoleAliasError::InternalFailure(ref cause) => cause,
            DescribeRoleAliasError::InvalidRequest(ref cause) => cause,
            DescribeRoleAliasError::ResourceNotFound(ref cause) => cause,
            DescribeRoleAliasError::ServiceUnavailable(ref cause) => cause,
            DescribeRoleAliasError::Throttling(ref cause) => cause,
            DescribeRoleAliasError::Unauthorized(ref cause) => cause,
            DescribeRoleAliasError::Validation(ref cause) => cause,
            DescribeRoleAliasError::Credentials(ref err) => err.description(),
            DescribeRoleAliasError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeRoleAliasError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeStream
#[derive(Debug, PartialEq)]
pub enum DescribeStreamError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeStreamError {
    pub fn from_body(body: &str) -> DescribeStreamError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DescribeStreamError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeStreamError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeStreamError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeStreamError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DescribeStreamError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeStreamError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeStreamError::Validation(error_message.to_string())
                    }
                    _ => DescribeStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeStreamError {
    fn from(err: serde_json::error::Error) -> DescribeStreamError {
        DescribeStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeStreamError {
    fn from(err: CredentialsError) -> DescribeStreamError {
        DescribeStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeStreamError {
    fn from(err: HttpDispatchError) -> DescribeStreamError {
        DescribeStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeStreamError {
    fn from(err: io::Error) -> DescribeStreamError {
        DescribeStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStreamError {
    fn description(&self) -> &str {
        match *self {
            DescribeStreamError::InternalFailure(ref cause) => cause,
            DescribeStreamError::InvalidRequest(ref cause) => cause,
            DescribeStreamError::ResourceNotFound(ref cause) => cause,
            DescribeStreamError::ServiceUnavailable(ref cause) => cause,
            DescribeStreamError::Throttling(ref cause) => cause,
            DescribeStreamError::Unauthorized(ref cause) => cause,
            DescribeStreamError::Validation(ref cause) => cause,
            DescribeStreamError::Credentials(ref err) => err.description(),
            DescribeStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeStreamError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeThing
#[derive(Debug, PartialEq)]
pub enum DescribeThingError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeThingError {
    pub fn from_body(body: &str) -> DescribeThingError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DescribeThingError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeThingError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeThingError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeThingError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DescribeThingError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeThingError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeThingError::Validation(error_message.to_string())
                    }
                    _ => DescribeThingError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeThingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeThingError {
    fn from(err: serde_json::error::Error) -> DescribeThingError {
        DescribeThingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeThingError {
    fn from(err: CredentialsError) -> DescribeThingError {
        DescribeThingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeThingError {
    fn from(err: HttpDispatchError) -> DescribeThingError {
        DescribeThingError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeThingError {
    fn from(err: io::Error) -> DescribeThingError {
        DescribeThingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeThingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeThingError {
    fn description(&self) -> &str {
        match *self {
            DescribeThingError::InternalFailure(ref cause) => cause,
            DescribeThingError::InvalidRequest(ref cause) => cause,
            DescribeThingError::ResourceNotFound(ref cause) => cause,
            DescribeThingError::ServiceUnavailable(ref cause) => cause,
            DescribeThingError::Throttling(ref cause) => cause,
            DescribeThingError::Unauthorized(ref cause) => cause,
            DescribeThingError::Validation(ref cause) => cause,
            DescribeThingError::Credentials(ref err) => err.description(),
            DescribeThingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeThingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeThingGroup
#[derive(Debug, PartialEq)]
pub enum DescribeThingGroupError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeThingGroupError {
    pub fn from_body(body: &str) -> DescribeThingGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DescribeThingGroupError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeThingGroupError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeThingGroupError::ResourceNotFound(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DescribeThingGroupError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeThingGroupError::Validation(error_message.to_string())
                    }
                    _ => DescribeThingGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeThingGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeThingGroupError {
    fn from(err: serde_json::error::Error) -> DescribeThingGroupError {
        DescribeThingGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeThingGroupError {
    fn from(err: CredentialsError) -> DescribeThingGroupError {
        DescribeThingGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeThingGroupError {
    fn from(err: HttpDispatchError) -> DescribeThingGroupError {
        DescribeThingGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeThingGroupError {
    fn from(err: io::Error) -> DescribeThingGroupError {
        DescribeThingGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeThingGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeThingGroupError {
    fn description(&self) -> &str {
        match *self {
            DescribeThingGroupError::InternalFailure(ref cause) => cause,
            DescribeThingGroupError::InvalidRequest(ref cause) => cause,
            DescribeThingGroupError::ResourceNotFound(ref cause) => cause,
            DescribeThingGroupError::Throttling(ref cause) => cause,
            DescribeThingGroupError::Validation(ref cause) => cause,
            DescribeThingGroupError::Credentials(ref err) => err.description(),
            DescribeThingGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeThingGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeThingRegistrationTask
#[derive(Debug, PartialEq)]
pub enum DescribeThingRegistrationTaskError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeThingRegistrationTaskError {
    pub fn from_body(body: &str) -> DescribeThingRegistrationTaskError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DescribeThingRegistrationTaskError::InternalFailure(String::from(
                            error_message,
                        ))
                    }
                    "InvalidRequestException" => {
                        DescribeThingRegistrationTaskError::InvalidRequest(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        DescribeThingRegistrationTaskError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ThrottlingException" => {
                        DescribeThingRegistrationTaskError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => DescribeThingRegistrationTaskError::Unauthorized(
                        String::from(error_message),
                    ),
                    "ValidationException" => {
                        DescribeThingRegistrationTaskError::Validation(error_message.to_string())
                    }
                    _ => DescribeThingRegistrationTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeThingRegistrationTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeThingRegistrationTaskError {
    fn from(err: serde_json::error::Error) -> DescribeThingRegistrationTaskError {
        DescribeThingRegistrationTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeThingRegistrationTaskError {
    fn from(err: CredentialsError) -> DescribeThingRegistrationTaskError {
        DescribeThingRegistrationTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeThingRegistrationTaskError {
    fn from(err: HttpDispatchError) -> DescribeThingRegistrationTaskError {
        DescribeThingRegistrationTaskError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeThingRegistrationTaskError {
    fn from(err: io::Error) -> DescribeThingRegistrationTaskError {
        DescribeThingRegistrationTaskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeThingRegistrationTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeThingRegistrationTaskError {
    fn description(&self) -> &str {
        match *self {
            DescribeThingRegistrationTaskError::InternalFailure(ref cause) => cause,
            DescribeThingRegistrationTaskError::InvalidRequest(ref cause) => cause,
            DescribeThingRegistrationTaskError::ResourceNotFound(ref cause) => cause,
            DescribeThingRegistrationTaskError::Throttling(ref cause) => cause,
            DescribeThingRegistrationTaskError::Unauthorized(ref cause) => cause,
            DescribeThingRegistrationTaskError::Validation(ref cause) => cause,
            DescribeThingRegistrationTaskError::Credentials(ref err) => err.description(),
            DescribeThingRegistrationTaskError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeThingRegistrationTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeThingType
#[derive(Debug, PartialEq)]
pub enum DescribeThingTypeError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeThingTypeError {
    pub fn from_body(body: &str) -> DescribeThingTypeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DescribeThingTypeError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DescribeThingTypeError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeThingTypeError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DescribeThingTypeError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DescribeThingTypeError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DescribeThingTypeError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeThingTypeError::Validation(error_message.to_string())
                    }
                    _ => DescribeThingTypeError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeThingTypeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeThingTypeError {
    fn from(err: serde_json::error::Error) -> DescribeThingTypeError {
        DescribeThingTypeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeThingTypeError {
    fn from(err: CredentialsError) -> DescribeThingTypeError {
        DescribeThingTypeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeThingTypeError {
    fn from(err: HttpDispatchError) -> DescribeThingTypeError {
        DescribeThingTypeError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeThingTypeError {
    fn from(err: io::Error) -> DescribeThingTypeError {
        DescribeThingTypeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeThingTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeThingTypeError {
    fn description(&self) -> &str {
        match *self {
            DescribeThingTypeError::InternalFailure(ref cause) => cause,
            DescribeThingTypeError::InvalidRequest(ref cause) => cause,
            DescribeThingTypeError::ResourceNotFound(ref cause) => cause,
            DescribeThingTypeError::ServiceUnavailable(ref cause) => cause,
            DescribeThingTypeError::Throttling(ref cause) => cause,
            DescribeThingTypeError::Unauthorized(ref cause) => cause,
            DescribeThingTypeError::Validation(ref cause) => cause,
            DescribeThingTypeError::Credentials(ref err) => err.description(),
            DescribeThingTypeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeThingTypeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DetachPolicy
#[derive(Debug, PartialEq)]
pub enum DetachPolicyError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The number of attached entities exceeds the limit.</p>
    LimitExceeded(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DetachPolicyError {
    pub fn from_body(body: &str) -> DetachPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DetachPolicyError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DetachPolicyError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        DetachPolicyError::LimitExceeded(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DetachPolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DetachPolicyError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DetachPolicyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DetachPolicyError::Validation(error_message.to_string())
                    }
                    _ => DetachPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DetachPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DetachPolicyError {
    fn from(err: serde_json::error::Error) -> DetachPolicyError {
        DetachPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DetachPolicyError {
    fn from(err: CredentialsError) -> DetachPolicyError {
        DetachPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachPolicyError {
    fn from(err: HttpDispatchError) -> DetachPolicyError {
        DetachPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetachPolicyError {
    fn from(err: io::Error) -> DetachPolicyError {
        DetachPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetachPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachPolicyError {
    fn description(&self) -> &str {
        match *self {
            DetachPolicyError::InternalFailure(ref cause) => cause,
            DetachPolicyError::InvalidRequest(ref cause) => cause,
            DetachPolicyError::LimitExceeded(ref cause) => cause,
            DetachPolicyError::ServiceUnavailable(ref cause) => cause,
            DetachPolicyError::Throttling(ref cause) => cause,
            DetachPolicyError::Unauthorized(ref cause) => cause,
            DetachPolicyError::Validation(ref cause) => cause,
            DetachPolicyError::Credentials(ref err) => err.description(),
            DetachPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DetachPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DetachPrincipalPolicy
#[derive(Debug, PartialEq)]
pub enum DetachPrincipalPolicyError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DetachPrincipalPolicyError {
    pub fn from_body(body: &str) -> DetachPrincipalPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DetachPrincipalPolicyError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DetachPrincipalPolicyError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DetachPrincipalPolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DetachPrincipalPolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DetachPrincipalPolicyError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DetachPrincipalPolicyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DetachPrincipalPolicyError::Validation(error_message.to_string())
                    }
                    _ => DetachPrincipalPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DetachPrincipalPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DetachPrincipalPolicyError {
    fn from(err: serde_json::error::Error) -> DetachPrincipalPolicyError {
        DetachPrincipalPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DetachPrincipalPolicyError {
    fn from(err: CredentialsError) -> DetachPrincipalPolicyError {
        DetachPrincipalPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachPrincipalPolicyError {
    fn from(err: HttpDispatchError) -> DetachPrincipalPolicyError {
        DetachPrincipalPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetachPrincipalPolicyError {
    fn from(err: io::Error) -> DetachPrincipalPolicyError {
        DetachPrincipalPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetachPrincipalPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachPrincipalPolicyError {
    fn description(&self) -> &str {
        match *self {
            DetachPrincipalPolicyError::InternalFailure(ref cause) => cause,
            DetachPrincipalPolicyError::InvalidRequest(ref cause) => cause,
            DetachPrincipalPolicyError::ResourceNotFound(ref cause) => cause,
            DetachPrincipalPolicyError::ServiceUnavailable(ref cause) => cause,
            DetachPrincipalPolicyError::Throttling(ref cause) => cause,
            DetachPrincipalPolicyError::Unauthorized(ref cause) => cause,
            DetachPrincipalPolicyError::Validation(ref cause) => cause,
            DetachPrincipalPolicyError::Credentials(ref err) => err.description(),
            DetachPrincipalPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DetachPrincipalPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DetachThingPrincipal
#[derive(Debug, PartialEq)]
pub enum DetachThingPrincipalError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DetachThingPrincipalError {
    pub fn from_body(body: &str) -> DetachThingPrincipalError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        DetachThingPrincipalError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DetachThingPrincipalError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DetachThingPrincipalError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DetachThingPrincipalError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        DetachThingPrincipalError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DetachThingPrincipalError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DetachThingPrincipalError::Validation(error_message.to_string())
                    }
                    _ => DetachThingPrincipalError::Unknown(String::from(body)),
                }
            }
            Err(_) => DetachThingPrincipalError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DetachThingPrincipalError {
    fn from(err: serde_json::error::Error) -> DetachThingPrincipalError {
        DetachThingPrincipalError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DetachThingPrincipalError {
    fn from(err: CredentialsError) -> DetachThingPrincipalError {
        DetachThingPrincipalError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DetachThingPrincipalError {
    fn from(err: HttpDispatchError) -> DetachThingPrincipalError {
        DetachThingPrincipalError::HttpDispatch(err)
    }
}
impl From<io::Error> for DetachThingPrincipalError {
    fn from(err: io::Error) -> DetachThingPrincipalError {
        DetachThingPrincipalError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DetachThingPrincipalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachThingPrincipalError {
    fn description(&self) -> &str {
        match *self {
            DetachThingPrincipalError::InternalFailure(ref cause) => cause,
            DetachThingPrincipalError::InvalidRequest(ref cause) => cause,
            DetachThingPrincipalError::ResourceNotFound(ref cause) => cause,
            DetachThingPrincipalError::ServiceUnavailable(ref cause) => cause,
            DetachThingPrincipalError::Throttling(ref cause) => cause,
            DetachThingPrincipalError::Unauthorized(ref cause) => cause,
            DetachThingPrincipalError::Validation(ref cause) => cause,
            DetachThingPrincipalError::Credentials(ref err) => err.description(),
            DetachThingPrincipalError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DetachThingPrincipalError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisableTopicRule
#[derive(Debug, PartialEq)]
pub enum DisableTopicRuleError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisableTopicRuleError {
    pub fn from_body(body: &str) -> DisableTopicRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        DisableTopicRuleError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        DisableTopicRuleError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        DisableTopicRuleError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        DisableTopicRuleError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DisableTopicRuleError::Validation(error_message.to_string())
                    }
                    _ => DisableTopicRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => DisableTopicRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DisableTopicRuleError {
    fn from(err: serde_json::error::Error) -> DisableTopicRuleError {
        DisableTopicRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisableTopicRuleError {
    fn from(err: CredentialsError) -> DisableTopicRuleError {
        DisableTopicRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisableTopicRuleError {
    fn from(err: HttpDispatchError) -> DisableTopicRuleError {
        DisableTopicRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisableTopicRuleError {
    fn from(err: io::Error) -> DisableTopicRuleError {
        DisableTopicRuleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisableTopicRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableTopicRuleError {
    fn description(&self) -> &str {
        match *self {
            DisableTopicRuleError::Internal(ref cause) => cause,
            DisableTopicRuleError::InvalidRequest(ref cause) => cause,
            DisableTopicRuleError::ServiceUnavailable(ref cause) => cause,
            DisableTopicRuleError::Unauthorized(ref cause) => cause,
            DisableTopicRuleError::Validation(ref cause) => cause,
            DisableTopicRuleError::Credentials(ref err) => err.description(),
            DisableTopicRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DisableTopicRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by EnableTopicRule
#[derive(Debug, PartialEq)]
pub enum EnableTopicRuleError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl EnableTopicRuleError {
    pub fn from_body(body: &str) -> EnableTopicRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        EnableTopicRuleError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        EnableTopicRuleError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        EnableTopicRuleError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        EnableTopicRuleError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        EnableTopicRuleError::Validation(error_message.to_string())
                    }
                    _ => EnableTopicRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => EnableTopicRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for EnableTopicRuleError {
    fn from(err: serde_json::error::Error) -> EnableTopicRuleError {
        EnableTopicRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for EnableTopicRuleError {
    fn from(err: CredentialsError) -> EnableTopicRuleError {
        EnableTopicRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for EnableTopicRuleError {
    fn from(err: HttpDispatchError) -> EnableTopicRuleError {
        EnableTopicRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for EnableTopicRuleError {
    fn from(err: io::Error) -> EnableTopicRuleError {
        EnableTopicRuleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for EnableTopicRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableTopicRuleError {
    fn description(&self) -> &str {
        match *self {
            EnableTopicRuleError::Internal(ref cause) => cause,
            EnableTopicRuleError::InvalidRequest(ref cause) => cause,
            EnableTopicRuleError::ServiceUnavailable(ref cause) => cause,
            EnableTopicRuleError::Unauthorized(ref cause) => cause,
            EnableTopicRuleError::Validation(ref cause) => cause,
            EnableTopicRuleError::Credentials(ref err) => err.description(),
            EnableTopicRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            EnableTopicRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetEffectivePolicies
#[derive(Debug, PartialEq)]
pub enum GetEffectivePoliciesError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The number of attached entities exceeds the limit.</p>
    LimitExceeded(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetEffectivePoliciesError {
    pub fn from_body(body: &str) -> GetEffectivePoliciesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        GetEffectivePoliciesError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        GetEffectivePoliciesError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        GetEffectivePoliciesError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetEffectivePoliciesError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetEffectivePoliciesError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        GetEffectivePoliciesError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetEffectivePoliciesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetEffectivePoliciesError::Validation(error_message.to_string())
                    }
                    _ => GetEffectivePoliciesError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetEffectivePoliciesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetEffectivePoliciesError {
    fn from(err: serde_json::error::Error) -> GetEffectivePoliciesError {
        GetEffectivePoliciesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetEffectivePoliciesError {
    fn from(err: CredentialsError) -> GetEffectivePoliciesError {
        GetEffectivePoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetEffectivePoliciesError {
    fn from(err: HttpDispatchError) -> GetEffectivePoliciesError {
        GetEffectivePoliciesError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetEffectivePoliciesError {
    fn from(err: io::Error) -> GetEffectivePoliciesError {
        GetEffectivePoliciesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetEffectivePoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetEffectivePoliciesError {
    fn description(&self) -> &str {
        match *self {
            GetEffectivePoliciesError::InternalFailure(ref cause) => cause,
            GetEffectivePoliciesError::InvalidRequest(ref cause) => cause,
            GetEffectivePoliciesError::LimitExceeded(ref cause) => cause,
            GetEffectivePoliciesError::ResourceNotFound(ref cause) => cause,
            GetEffectivePoliciesError::ServiceUnavailable(ref cause) => cause,
            GetEffectivePoliciesError::Throttling(ref cause) => cause,
            GetEffectivePoliciesError::Unauthorized(ref cause) => cause,
            GetEffectivePoliciesError::Validation(ref cause) => cause,
            GetEffectivePoliciesError::Credentials(ref err) => err.description(),
            GetEffectivePoliciesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetEffectivePoliciesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetIndexingConfiguration
#[derive(Debug, PartialEq)]
pub enum GetIndexingConfigurationError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetIndexingConfigurationError {
    pub fn from_body(body: &str) -> GetIndexingConfigurationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        GetIndexingConfigurationError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        GetIndexingConfigurationError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetIndexingConfigurationError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ThrottlingException" => {
                        GetIndexingConfigurationError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetIndexingConfigurationError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetIndexingConfigurationError::Validation(error_message.to_string())
                    }
                    _ => GetIndexingConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetIndexingConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetIndexingConfigurationError {
    fn from(err: serde_json::error::Error) -> GetIndexingConfigurationError {
        GetIndexingConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetIndexingConfigurationError {
    fn from(err: CredentialsError) -> GetIndexingConfigurationError {
        GetIndexingConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetIndexingConfigurationError {
    fn from(err: HttpDispatchError) -> GetIndexingConfigurationError {
        GetIndexingConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetIndexingConfigurationError {
    fn from(err: io::Error) -> GetIndexingConfigurationError {
        GetIndexingConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetIndexingConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIndexingConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetIndexingConfigurationError::InternalFailure(ref cause) => cause,
            GetIndexingConfigurationError::InvalidRequest(ref cause) => cause,
            GetIndexingConfigurationError::ServiceUnavailable(ref cause) => cause,
            GetIndexingConfigurationError::Throttling(ref cause) => cause,
            GetIndexingConfigurationError::Unauthorized(ref cause) => cause,
            GetIndexingConfigurationError::Validation(ref cause) => cause,
            GetIndexingConfigurationError::Credentials(ref err) => err.description(),
            GetIndexingConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetIndexingConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetJobDocument
#[derive(Debug, PartialEq)]
pub enum GetJobDocumentError {
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetJobDocumentError {
    pub fn from_body(body: &str) -> GetJobDocumentError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidRequestException" => {
                        GetJobDocumentError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetJobDocumentError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetJobDocumentError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        GetJobDocumentError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetJobDocumentError::Validation(error_message.to_string())
                    }
                    _ => GetJobDocumentError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetJobDocumentError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetJobDocumentError {
    fn from(err: serde_json::error::Error) -> GetJobDocumentError {
        GetJobDocumentError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetJobDocumentError {
    fn from(err: CredentialsError) -> GetJobDocumentError {
        GetJobDocumentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetJobDocumentError {
    fn from(err: HttpDispatchError) -> GetJobDocumentError {
        GetJobDocumentError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetJobDocumentError {
    fn from(err: io::Error) -> GetJobDocumentError {
        GetJobDocumentError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetJobDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetJobDocumentError {
    fn description(&self) -> &str {
        match *self {
            GetJobDocumentError::InvalidRequest(ref cause) => cause,
            GetJobDocumentError::ResourceNotFound(ref cause) => cause,
            GetJobDocumentError::ServiceUnavailable(ref cause) => cause,
            GetJobDocumentError::Throttling(ref cause) => cause,
            GetJobDocumentError::Validation(ref cause) => cause,
            GetJobDocumentError::Credentials(ref err) => err.description(),
            GetJobDocumentError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetJobDocumentError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetLoggingOptions
#[derive(Debug, PartialEq)]
pub enum GetLoggingOptionsError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetLoggingOptionsError {
    pub fn from_body(body: &str) -> GetLoggingOptionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        GetLoggingOptionsError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        GetLoggingOptionsError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetLoggingOptionsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetLoggingOptionsError::Validation(error_message.to_string())
                    }
                    _ => GetLoggingOptionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetLoggingOptionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetLoggingOptionsError {
    fn from(err: serde_json::error::Error) -> GetLoggingOptionsError {
        GetLoggingOptionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetLoggingOptionsError {
    fn from(err: CredentialsError) -> GetLoggingOptionsError {
        GetLoggingOptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetLoggingOptionsError {
    fn from(err: HttpDispatchError) -> GetLoggingOptionsError {
        GetLoggingOptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetLoggingOptionsError {
    fn from(err: io::Error) -> GetLoggingOptionsError {
        GetLoggingOptionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetLoggingOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLoggingOptionsError {
    fn description(&self) -> &str {
        match *self {
            GetLoggingOptionsError::Internal(ref cause) => cause,
            GetLoggingOptionsError::InvalidRequest(ref cause) => cause,
            GetLoggingOptionsError::ServiceUnavailable(ref cause) => cause,
            GetLoggingOptionsError::Validation(ref cause) => cause,
            GetLoggingOptionsError::Credentials(ref err) => err.description(),
            GetLoggingOptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetLoggingOptionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetOTAUpdate
#[derive(Debug, PartialEq)]
pub enum GetOTAUpdateError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetOTAUpdateError {
    pub fn from_body(body: &str) -> GetOTAUpdateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        GetOTAUpdateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        GetOTAUpdateError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetOTAUpdateError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetOTAUpdateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        GetOTAUpdateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetOTAUpdateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetOTAUpdateError::Validation(error_message.to_string())
                    }
                    _ => GetOTAUpdateError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetOTAUpdateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetOTAUpdateError {
    fn from(err: serde_json::error::Error) -> GetOTAUpdateError {
        GetOTAUpdateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetOTAUpdateError {
    fn from(err: CredentialsError) -> GetOTAUpdateError {
        GetOTAUpdateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetOTAUpdateError {
    fn from(err: HttpDispatchError) -> GetOTAUpdateError {
        GetOTAUpdateError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetOTAUpdateError {
    fn from(err: io::Error) -> GetOTAUpdateError {
        GetOTAUpdateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetOTAUpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetOTAUpdateError {
    fn description(&self) -> &str {
        match *self {
            GetOTAUpdateError::InternalFailure(ref cause) => cause,
            GetOTAUpdateError::InvalidRequest(ref cause) => cause,
            GetOTAUpdateError::ResourceNotFound(ref cause) => cause,
            GetOTAUpdateError::ServiceUnavailable(ref cause) => cause,
            GetOTAUpdateError::Throttling(ref cause) => cause,
            GetOTAUpdateError::Unauthorized(ref cause) => cause,
            GetOTAUpdateError::Validation(ref cause) => cause,
            GetOTAUpdateError::Credentials(ref err) => err.description(),
            GetOTAUpdateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetOTAUpdateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPolicy
#[derive(Debug, PartialEq)]
pub enum GetPolicyError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetPolicyError {
    pub fn from_body(body: &str) -> GetPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        GetPolicyError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        GetPolicyError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetPolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetPolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        GetPolicyError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetPolicyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => GetPolicyError::Validation(error_message.to_string()),
                    _ => GetPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetPolicyError {
    fn from(err: serde_json::error::Error) -> GetPolicyError {
        GetPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetPolicyError {
    fn from(err: CredentialsError) -> GetPolicyError {
        GetPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetPolicyError {
    fn from(err: HttpDispatchError) -> GetPolicyError {
        GetPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetPolicyError {
    fn from(err: io::Error) -> GetPolicyError {
        GetPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPolicyError {
    fn description(&self) -> &str {
        match *self {
            GetPolicyError::InternalFailure(ref cause) => cause,
            GetPolicyError::InvalidRequest(ref cause) => cause,
            GetPolicyError::ResourceNotFound(ref cause) => cause,
            GetPolicyError::ServiceUnavailable(ref cause) => cause,
            GetPolicyError::Throttling(ref cause) => cause,
            GetPolicyError::Unauthorized(ref cause) => cause,
            GetPolicyError::Validation(ref cause) => cause,
            GetPolicyError::Credentials(ref err) => err.description(),
            GetPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPolicyVersion
#[derive(Debug, PartialEq)]
pub enum GetPolicyVersionError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetPolicyVersionError {
    pub fn from_body(body: &str) -> GetPolicyVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        GetPolicyVersionError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        GetPolicyVersionError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetPolicyVersionError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetPolicyVersionError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        GetPolicyVersionError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetPolicyVersionError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetPolicyVersionError::Validation(error_message.to_string())
                    }
                    _ => GetPolicyVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetPolicyVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetPolicyVersionError {
    fn from(err: serde_json::error::Error) -> GetPolicyVersionError {
        GetPolicyVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetPolicyVersionError {
    fn from(err: CredentialsError) -> GetPolicyVersionError {
        GetPolicyVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetPolicyVersionError {
    fn from(err: HttpDispatchError) -> GetPolicyVersionError {
        GetPolicyVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetPolicyVersionError {
    fn from(err: io::Error) -> GetPolicyVersionError {
        GetPolicyVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetPolicyVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPolicyVersionError {
    fn description(&self) -> &str {
        match *self {
            GetPolicyVersionError::InternalFailure(ref cause) => cause,
            GetPolicyVersionError::InvalidRequest(ref cause) => cause,
            GetPolicyVersionError::ResourceNotFound(ref cause) => cause,
            GetPolicyVersionError::ServiceUnavailable(ref cause) => cause,
            GetPolicyVersionError::Throttling(ref cause) => cause,
            GetPolicyVersionError::Unauthorized(ref cause) => cause,
            GetPolicyVersionError::Validation(ref cause) => cause,
            GetPolicyVersionError::Credentials(ref err) => err.description(),
            GetPolicyVersionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetPolicyVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRegistrationCode
#[derive(Debug, PartialEq)]
pub enum GetRegistrationCodeError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetRegistrationCodeError {
    pub fn from_body(body: &str) -> GetRegistrationCodeError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        GetRegistrationCodeError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        GetRegistrationCodeError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetRegistrationCodeError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        GetRegistrationCodeError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetRegistrationCodeError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetRegistrationCodeError::Validation(error_message.to_string())
                    }
                    _ => GetRegistrationCodeError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetRegistrationCodeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetRegistrationCodeError {
    fn from(err: serde_json::error::Error) -> GetRegistrationCodeError {
        GetRegistrationCodeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetRegistrationCodeError {
    fn from(err: CredentialsError) -> GetRegistrationCodeError {
        GetRegistrationCodeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetRegistrationCodeError {
    fn from(err: HttpDispatchError) -> GetRegistrationCodeError {
        GetRegistrationCodeError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetRegistrationCodeError {
    fn from(err: io::Error) -> GetRegistrationCodeError {
        GetRegistrationCodeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetRegistrationCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRegistrationCodeError {
    fn description(&self) -> &str {
        match *self {
            GetRegistrationCodeError::InternalFailure(ref cause) => cause,
            GetRegistrationCodeError::InvalidRequest(ref cause) => cause,
            GetRegistrationCodeError::ServiceUnavailable(ref cause) => cause,
            GetRegistrationCodeError::Throttling(ref cause) => cause,
            GetRegistrationCodeError::Unauthorized(ref cause) => cause,
            GetRegistrationCodeError::Validation(ref cause) => cause,
            GetRegistrationCodeError::Credentials(ref err) => err.description(),
            GetRegistrationCodeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetRegistrationCodeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTopicRule
#[derive(Debug, PartialEq)]
pub enum GetTopicRuleError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetTopicRuleError {
    pub fn from_body(body: &str) -> GetTopicRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => GetTopicRuleError::Internal(String::from(error_message)),
                    "InvalidRequestException" => {
                        GetTopicRuleError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetTopicRuleError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        GetTopicRuleError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetTopicRuleError::Validation(error_message.to_string())
                    }
                    _ => GetTopicRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetTopicRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetTopicRuleError {
    fn from(err: serde_json::error::Error) -> GetTopicRuleError {
        GetTopicRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetTopicRuleError {
    fn from(err: CredentialsError) -> GetTopicRuleError {
        GetTopicRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetTopicRuleError {
    fn from(err: HttpDispatchError) -> GetTopicRuleError {
        GetTopicRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetTopicRuleError {
    fn from(err: io::Error) -> GetTopicRuleError {
        GetTopicRuleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetTopicRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTopicRuleError {
    fn description(&self) -> &str {
        match *self {
            GetTopicRuleError::Internal(ref cause) => cause,
            GetTopicRuleError::InvalidRequest(ref cause) => cause,
            GetTopicRuleError::ServiceUnavailable(ref cause) => cause,
            GetTopicRuleError::Unauthorized(ref cause) => cause,
            GetTopicRuleError::Validation(ref cause) => cause,
            GetTopicRuleError::Credentials(ref err) => err.description(),
            GetTopicRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetTopicRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetV2LoggingOptions
#[derive(Debug, PartialEq)]
pub enum GetV2LoggingOptionsError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetV2LoggingOptionsError {
    pub fn from_body(body: &str) -> GetV2LoggingOptionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        GetV2LoggingOptionsError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        GetV2LoggingOptionsError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        GetV2LoggingOptionsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetV2LoggingOptionsError::Validation(error_message.to_string())
                    }
                    _ => GetV2LoggingOptionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetV2LoggingOptionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetV2LoggingOptionsError {
    fn from(err: serde_json::error::Error) -> GetV2LoggingOptionsError {
        GetV2LoggingOptionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetV2LoggingOptionsError {
    fn from(err: CredentialsError) -> GetV2LoggingOptionsError {
        GetV2LoggingOptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetV2LoggingOptionsError {
    fn from(err: HttpDispatchError) -> GetV2LoggingOptionsError {
        GetV2LoggingOptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetV2LoggingOptionsError {
    fn from(err: io::Error) -> GetV2LoggingOptionsError {
        GetV2LoggingOptionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetV2LoggingOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetV2LoggingOptionsError {
    fn description(&self) -> &str {
        match *self {
            GetV2LoggingOptionsError::Internal(ref cause) => cause,
            GetV2LoggingOptionsError::InvalidRequest(ref cause) => cause,
            GetV2LoggingOptionsError::ServiceUnavailable(ref cause) => cause,
            GetV2LoggingOptionsError::Validation(ref cause) => cause,
            GetV2LoggingOptionsError::Credentials(ref err) => err.description(),
            GetV2LoggingOptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetV2LoggingOptionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAttachedPolicies
#[derive(Debug, PartialEq)]
pub enum ListAttachedPoliciesError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The number of attached entities exceeds the limit.</p>
    LimitExceeded(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListAttachedPoliciesError {
    pub fn from_body(body: &str) -> ListAttachedPoliciesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListAttachedPoliciesError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListAttachedPoliciesError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListAttachedPoliciesError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListAttachedPoliciesError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListAttachedPoliciesError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListAttachedPoliciesError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListAttachedPoliciesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListAttachedPoliciesError::Validation(error_message.to_string())
                    }
                    _ => ListAttachedPoliciesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAttachedPoliciesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAttachedPoliciesError {
    fn from(err: serde_json::error::Error) -> ListAttachedPoliciesError {
        ListAttachedPoliciesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAttachedPoliciesError {
    fn from(err: CredentialsError) -> ListAttachedPoliciesError {
        ListAttachedPoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAttachedPoliciesError {
    fn from(err: HttpDispatchError) -> ListAttachedPoliciesError {
        ListAttachedPoliciesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAttachedPoliciesError {
    fn from(err: io::Error) -> ListAttachedPoliciesError {
        ListAttachedPoliciesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAttachedPoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAttachedPoliciesError {
    fn description(&self) -> &str {
        match *self {
            ListAttachedPoliciesError::InternalFailure(ref cause) => cause,
            ListAttachedPoliciesError::InvalidRequest(ref cause) => cause,
            ListAttachedPoliciesError::LimitExceeded(ref cause) => cause,
            ListAttachedPoliciesError::ResourceNotFound(ref cause) => cause,
            ListAttachedPoliciesError::ServiceUnavailable(ref cause) => cause,
            ListAttachedPoliciesError::Throttling(ref cause) => cause,
            ListAttachedPoliciesError::Unauthorized(ref cause) => cause,
            ListAttachedPoliciesError::Validation(ref cause) => cause,
            ListAttachedPoliciesError::Credentials(ref err) => err.description(),
            ListAttachedPoliciesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListAttachedPoliciesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAuthorizers
#[derive(Debug, PartialEq)]
pub enum ListAuthorizersError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListAuthorizersError {
    pub fn from_body(body: &str) -> ListAuthorizersError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListAuthorizersError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListAuthorizersError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListAuthorizersError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListAuthorizersError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListAuthorizersError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListAuthorizersError::Validation(error_message.to_string())
                    }
                    _ => ListAuthorizersError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAuthorizersError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAuthorizersError {
    fn from(err: serde_json::error::Error) -> ListAuthorizersError {
        ListAuthorizersError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAuthorizersError {
    fn from(err: CredentialsError) -> ListAuthorizersError {
        ListAuthorizersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAuthorizersError {
    fn from(err: HttpDispatchError) -> ListAuthorizersError {
        ListAuthorizersError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAuthorizersError {
    fn from(err: io::Error) -> ListAuthorizersError {
        ListAuthorizersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAuthorizersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAuthorizersError {
    fn description(&self) -> &str {
        match *self {
            ListAuthorizersError::InternalFailure(ref cause) => cause,
            ListAuthorizersError::InvalidRequest(ref cause) => cause,
            ListAuthorizersError::ServiceUnavailable(ref cause) => cause,
            ListAuthorizersError::Throttling(ref cause) => cause,
            ListAuthorizersError::Unauthorized(ref cause) => cause,
            ListAuthorizersError::Validation(ref cause) => cause,
            ListAuthorizersError::Credentials(ref err) => err.description(),
            ListAuthorizersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListAuthorizersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCACertificates
#[derive(Debug, PartialEq)]
pub enum ListCACertificatesError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListCACertificatesError {
    pub fn from_body(body: &str) -> ListCACertificatesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListCACertificatesError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListCACertificatesError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListCACertificatesError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListCACertificatesError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListCACertificatesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListCACertificatesError::Validation(error_message.to_string())
                    }
                    _ => ListCACertificatesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListCACertificatesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListCACertificatesError {
    fn from(err: serde_json::error::Error) -> ListCACertificatesError {
        ListCACertificatesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListCACertificatesError {
    fn from(err: CredentialsError) -> ListCACertificatesError {
        ListCACertificatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListCACertificatesError {
    fn from(err: HttpDispatchError) -> ListCACertificatesError {
        ListCACertificatesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListCACertificatesError {
    fn from(err: io::Error) -> ListCACertificatesError {
        ListCACertificatesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListCACertificatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCACertificatesError {
    fn description(&self) -> &str {
        match *self {
            ListCACertificatesError::InternalFailure(ref cause) => cause,
            ListCACertificatesError::InvalidRequest(ref cause) => cause,
            ListCACertificatesError::ServiceUnavailable(ref cause) => cause,
            ListCACertificatesError::Throttling(ref cause) => cause,
            ListCACertificatesError::Unauthorized(ref cause) => cause,
            ListCACertificatesError::Validation(ref cause) => cause,
            ListCACertificatesError::Credentials(ref err) => err.description(),
            ListCACertificatesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListCACertificatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCertificates
#[derive(Debug, PartialEq)]
pub enum ListCertificatesError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListCertificatesError {
    pub fn from_body(body: &str) -> ListCertificatesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListCertificatesError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListCertificatesError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListCertificatesError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListCertificatesError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListCertificatesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListCertificatesError::Validation(error_message.to_string())
                    }
                    _ => ListCertificatesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListCertificatesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListCertificatesError {
    fn from(err: serde_json::error::Error) -> ListCertificatesError {
        ListCertificatesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListCertificatesError {
    fn from(err: CredentialsError) -> ListCertificatesError {
        ListCertificatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListCertificatesError {
    fn from(err: HttpDispatchError) -> ListCertificatesError {
        ListCertificatesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListCertificatesError {
    fn from(err: io::Error) -> ListCertificatesError {
        ListCertificatesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListCertificatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCertificatesError {
    fn description(&self) -> &str {
        match *self {
            ListCertificatesError::InternalFailure(ref cause) => cause,
            ListCertificatesError::InvalidRequest(ref cause) => cause,
            ListCertificatesError::ServiceUnavailable(ref cause) => cause,
            ListCertificatesError::Throttling(ref cause) => cause,
            ListCertificatesError::Unauthorized(ref cause) => cause,
            ListCertificatesError::Validation(ref cause) => cause,
            ListCertificatesError::Credentials(ref err) => err.description(),
            ListCertificatesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListCertificatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCertificatesByCA
#[derive(Debug, PartialEq)]
pub enum ListCertificatesByCAError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListCertificatesByCAError {
    pub fn from_body(body: &str) -> ListCertificatesByCAError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListCertificatesByCAError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListCertificatesByCAError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListCertificatesByCAError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListCertificatesByCAError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListCertificatesByCAError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListCertificatesByCAError::Validation(error_message.to_string())
                    }
                    _ => ListCertificatesByCAError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListCertificatesByCAError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListCertificatesByCAError {
    fn from(err: serde_json::error::Error) -> ListCertificatesByCAError {
        ListCertificatesByCAError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListCertificatesByCAError {
    fn from(err: CredentialsError) -> ListCertificatesByCAError {
        ListCertificatesByCAError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListCertificatesByCAError {
    fn from(err: HttpDispatchError) -> ListCertificatesByCAError {
        ListCertificatesByCAError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListCertificatesByCAError {
    fn from(err: io::Error) -> ListCertificatesByCAError {
        ListCertificatesByCAError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListCertificatesByCAError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCertificatesByCAError {
    fn description(&self) -> &str {
        match *self {
            ListCertificatesByCAError::InternalFailure(ref cause) => cause,
            ListCertificatesByCAError::InvalidRequest(ref cause) => cause,
            ListCertificatesByCAError::ServiceUnavailable(ref cause) => cause,
            ListCertificatesByCAError::Throttling(ref cause) => cause,
            ListCertificatesByCAError::Unauthorized(ref cause) => cause,
            ListCertificatesByCAError::Validation(ref cause) => cause,
            ListCertificatesByCAError::Credentials(ref err) => err.description(),
            ListCertificatesByCAError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListCertificatesByCAError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListIndices
#[derive(Debug, PartialEq)]
pub enum ListIndicesError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListIndicesError {
    pub fn from_body(body: &str) -> ListIndicesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListIndicesError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListIndicesError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListIndicesError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListIndicesError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListIndicesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListIndicesError::Validation(error_message.to_string())
                    }
                    _ => ListIndicesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListIndicesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListIndicesError {
    fn from(err: serde_json::error::Error) -> ListIndicesError {
        ListIndicesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListIndicesError {
    fn from(err: CredentialsError) -> ListIndicesError {
        ListIndicesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListIndicesError {
    fn from(err: HttpDispatchError) -> ListIndicesError {
        ListIndicesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListIndicesError {
    fn from(err: io::Error) -> ListIndicesError {
        ListIndicesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListIndicesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListIndicesError {
    fn description(&self) -> &str {
        match *self {
            ListIndicesError::InternalFailure(ref cause) => cause,
            ListIndicesError::InvalidRequest(ref cause) => cause,
            ListIndicesError::ServiceUnavailable(ref cause) => cause,
            ListIndicesError::Throttling(ref cause) => cause,
            ListIndicesError::Unauthorized(ref cause) => cause,
            ListIndicesError::Validation(ref cause) => cause,
            ListIndicesError::Credentials(ref err) => err.description(),
            ListIndicesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListIndicesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListJobExecutionsForJob
#[derive(Debug, PartialEq)]
pub enum ListJobExecutionsForJobError {
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListJobExecutionsForJobError {
    pub fn from_body(body: &str) -> ListJobExecutionsForJobError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidRequestException" => {
                        ListJobExecutionsForJobError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListJobExecutionsForJobError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListJobExecutionsForJobError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ThrottlingException" => {
                        ListJobExecutionsForJobError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListJobExecutionsForJobError::Validation(error_message.to_string())
                    }
                    _ => ListJobExecutionsForJobError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListJobExecutionsForJobError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListJobExecutionsForJobError {
    fn from(err: serde_json::error::Error) -> ListJobExecutionsForJobError {
        ListJobExecutionsForJobError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListJobExecutionsForJobError {
    fn from(err: CredentialsError) -> ListJobExecutionsForJobError {
        ListJobExecutionsForJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListJobExecutionsForJobError {
    fn from(err: HttpDispatchError) -> ListJobExecutionsForJobError {
        ListJobExecutionsForJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListJobExecutionsForJobError {
    fn from(err: io::Error) -> ListJobExecutionsForJobError {
        ListJobExecutionsForJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListJobExecutionsForJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListJobExecutionsForJobError {
    fn description(&self) -> &str {
        match *self {
            ListJobExecutionsForJobError::InvalidRequest(ref cause) => cause,
            ListJobExecutionsForJobError::ResourceNotFound(ref cause) => cause,
            ListJobExecutionsForJobError::ServiceUnavailable(ref cause) => cause,
            ListJobExecutionsForJobError::Throttling(ref cause) => cause,
            ListJobExecutionsForJobError::Validation(ref cause) => cause,
            ListJobExecutionsForJobError::Credentials(ref err) => err.description(),
            ListJobExecutionsForJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListJobExecutionsForJobError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListJobExecutionsForThing
#[derive(Debug, PartialEq)]
pub enum ListJobExecutionsForThingError {
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListJobExecutionsForThingError {
    pub fn from_body(body: &str) -> ListJobExecutionsForThingError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidRequestException" => {
                        ListJobExecutionsForThingError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListJobExecutionsForThingError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ServiceUnavailableException" => {
                        ListJobExecutionsForThingError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ThrottlingException" => {
                        ListJobExecutionsForThingError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListJobExecutionsForThingError::Validation(error_message.to_string())
                    }
                    _ => ListJobExecutionsForThingError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListJobExecutionsForThingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListJobExecutionsForThingError {
    fn from(err: serde_json::error::Error) -> ListJobExecutionsForThingError {
        ListJobExecutionsForThingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListJobExecutionsForThingError {
    fn from(err: CredentialsError) -> ListJobExecutionsForThingError {
        ListJobExecutionsForThingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListJobExecutionsForThingError {
    fn from(err: HttpDispatchError) -> ListJobExecutionsForThingError {
        ListJobExecutionsForThingError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListJobExecutionsForThingError {
    fn from(err: io::Error) -> ListJobExecutionsForThingError {
        ListJobExecutionsForThingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListJobExecutionsForThingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListJobExecutionsForThingError {
    fn description(&self) -> &str {
        match *self {
            ListJobExecutionsForThingError::InvalidRequest(ref cause) => cause,
            ListJobExecutionsForThingError::ResourceNotFound(ref cause) => cause,
            ListJobExecutionsForThingError::ServiceUnavailable(ref cause) => cause,
            ListJobExecutionsForThingError::Throttling(ref cause) => cause,
            ListJobExecutionsForThingError::Validation(ref cause) => cause,
            ListJobExecutionsForThingError::Credentials(ref err) => err.description(),
            ListJobExecutionsForThingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListJobExecutionsForThingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListJobs
#[derive(Debug, PartialEq)]
pub enum ListJobsError {
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListJobsError {
    pub fn from_body(body: &str) -> ListJobsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidRequestException" => {
                        ListJobsError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListJobsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListJobsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => ListJobsError::Throttling(String::from(error_message)),
                    "ValidationException" => ListJobsError::Validation(error_message.to_string()),
                    _ => ListJobsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListJobsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListJobsError {
    fn from(err: serde_json::error::Error) -> ListJobsError {
        ListJobsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListJobsError {
    fn from(err: CredentialsError) -> ListJobsError {
        ListJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListJobsError {
    fn from(err: HttpDispatchError) -> ListJobsError {
        ListJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListJobsError {
    fn from(err: io::Error) -> ListJobsError {
        ListJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListJobsError {
    fn description(&self) -> &str {
        match *self {
            ListJobsError::InvalidRequest(ref cause) => cause,
            ListJobsError::ResourceNotFound(ref cause) => cause,
            ListJobsError::ServiceUnavailable(ref cause) => cause,
            ListJobsError::Throttling(ref cause) => cause,
            ListJobsError::Validation(ref cause) => cause,
            ListJobsError::Credentials(ref err) => err.description(),
            ListJobsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListJobsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListOTAUpdates
#[derive(Debug, PartialEq)]
pub enum ListOTAUpdatesError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListOTAUpdatesError {
    pub fn from_body(body: &str) -> ListOTAUpdatesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListOTAUpdatesError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListOTAUpdatesError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListOTAUpdatesError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListOTAUpdatesError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListOTAUpdatesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListOTAUpdatesError::Validation(error_message.to_string())
                    }
                    _ => ListOTAUpdatesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListOTAUpdatesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListOTAUpdatesError {
    fn from(err: serde_json::error::Error) -> ListOTAUpdatesError {
        ListOTAUpdatesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListOTAUpdatesError {
    fn from(err: CredentialsError) -> ListOTAUpdatesError {
        ListOTAUpdatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListOTAUpdatesError {
    fn from(err: HttpDispatchError) -> ListOTAUpdatesError {
        ListOTAUpdatesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListOTAUpdatesError {
    fn from(err: io::Error) -> ListOTAUpdatesError {
        ListOTAUpdatesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListOTAUpdatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListOTAUpdatesError {
    fn description(&self) -> &str {
        match *self {
            ListOTAUpdatesError::InternalFailure(ref cause) => cause,
            ListOTAUpdatesError::InvalidRequest(ref cause) => cause,
            ListOTAUpdatesError::ServiceUnavailable(ref cause) => cause,
            ListOTAUpdatesError::Throttling(ref cause) => cause,
            ListOTAUpdatesError::Unauthorized(ref cause) => cause,
            ListOTAUpdatesError::Validation(ref cause) => cause,
            ListOTAUpdatesError::Credentials(ref err) => err.description(),
            ListOTAUpdatesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListOTAUpdatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListOutgoingCertificates
#[derive(Debug, PartialEq)]
pub enum ListOutgoingCertificatesError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListOutgoingCertificatesError {
    pub fn from_body(body: &str) -> ListOutgoingCertificatesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListOutgoingCertificatesError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListOutgoingCertificatesError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListOutgoingCertificatesError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ThrottlingException" => {
                        ListOutgoingCertificatesError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListOutgoingCertificatesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListOutgoingCertificatesError::Validation(error_message.to_string())
                    }
                    _ => ListOutgoingCertificatesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListOutgoingCertificatesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListOutgoingCertificatesError {
    fn from(err: serde_json::error::Error) -> ListOutgoingCertificatesError {
        ListOutgoingCertificatesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListOutgoingCertificatesError {
    fn from(err: CredentialsError) -> ListOutgoingCertificatesError {
        ListOutgoingCertificatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListOutgoingCertificatesError {
    fn from(err: HttpDispatchError) -> ListOutgoingCertificatesError {
        ListOutgoingCertificatesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListOutgoingCertificatesError {
    fn from(err: io::Error) -> ListOutgoingCertificatesError {
        ListOutgoingCertificatesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListOutgoingCertificatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListOutgoingCertificatesError {
    fn description(&self) -> &str {
        match *self {
            ListOutgoingCertificatesError::InternalFailure(ref cause) => cause,
            ListOutgoingCertificatesError::InvalidRequest(ref cause) => cause,
            ListOutgoingCertificatesError::ServiceUnavailable(ref cause) => cause,
            ListOutgoingCertificatesError::Throttling(ref cause) => cause,
            ListOutgoingCertificatesError::Unauthorized(ref cause) => cause,
            ListOutgoingCertificatesError::Validation(ref cause) => cause,
            ListOutgoingCertificatesError::Credentials(ref err) => err.description(),
            ListOutgoingCertificatesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListOutgoingCertificatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPolicies
#[derive(Debug, PartialEq)]
pub enum ListPoliciesError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListPoliciesError {
    pub fn from_body(body: &str) -> ListPoliciesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListPoliciesError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListPoliciesError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListPoliciesError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListPoliciesError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListPoliciesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListPoliciesError::Validation(error_message.to_string())
                    }
                    _ => ListPoliciesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPoliciesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListPoliciesError {
    fn from(err: serde_json::error::Error) -> ListPoliciesError {
        ListPoliciesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPoliciesError {
    fn from(err: CredentialsError) -> ListPoliciesError {
        ListPoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPoliciesError {
    fn from(err: HttpDispatchError) -> ListPoliciesError {
        ListPoliciesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPoliciesError {
    fn from(err: io::Error) -> ListPoliciesError {
        ListPoliciesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPoliciesError {
    fn description(&self) -> &str {
        match *self {
            ListPoliciesError::InternalFailure(ref cause) => cause,
            ListPoliciesError::InvalidRequest(ref cause) => cause,
            ListPoliciesError::ServiceUnavailable(ref cause) => cause,
            ListPoliciesError::Throttling(ref cause) => cause,
            ListPoliciesError::Unauthorized(ref cause) => cause,
            ListPoliciesError::Validation(ref cause) => cause,
            ListPoliciesError::Credentials(ref err) => err.description(),
            ListPoliciesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListPoliciesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPolicyPrincipals
#[derive(Debug, PartialEq)]
pub enum ListPolicyPrincipalsError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListPolicyPrincipalsError {
    pub fn from_body(body: &str) -> ListPolicyPrincipalsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListPolicyPrincipalsError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListPolicyPrincipalsError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListPolicyPrincipalsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListPolicyPrincipalsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListPolicyPrincipalsError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListPolicyPrincipalsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListPolicyPrincipalsError::Validation(error_message.to_string())
                    }
                    _ => ListPolicyPrincipalsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPolicyPrincipalsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListPolicyPrincipalsError {
    fn from(err: serde_json::error::Error) -> ListPolicyPrincipalsError {
        ListPolicyPrincipalsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPolicyPrincipalsError {
    fn from(err: CredentialsError) -> ListPolicyPrincipalsError {
        ListPolicyPrincipalsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPolicyPrincipalsError {
    fn from(err: HttpDispatchError) -> ListPolicyPrincipalsError {
        ListPolicyPrincipalsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPolicyPrincipalsError {
    fn from(err: io::Error) -> ListPolicyPrincipalsError {
        ListPolicyPrincipalsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPolicyPrincipalsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPolicyPrincipalsError {
    fn description(&self) -> &str {
        match *self {
            ListPolicyPrincipalsError::InternalFailure(ref cause) => cause,
            ListPolicyPrincipalsError::InvalidRequest(ref cause) => cause,
            ListPolicyPrincipalsError::ResourceNotFound(ref cause) => cause,
            ListPolicyPrincipalsError::ServiceUnavailable(ref cause) => cause,
            ListPolicyPrincipalsError::Throttling(ref cause) => cause,
            ListPolicyPrincipalsError::Unauthorized(ref cause) => cause,
            ListPolicyPrincipalsError::Validation(ref cause) => cause,
            ListPolicyPrincipalsError::Credentials(ref err) => err.description(),
            ListPolicyPrincipalsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPolicyPrincipalsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPolicyVersions
#[derive(Debug, PartialEq)]
pub enum ListPolicyVersionsError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListPolicyVersionsError {
    pub fn from_body(body: &str) -> ListPolicyVersionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListPolicyVersionsError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListPolicyVersionsError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListPolicyVersionsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListPolicyVersionsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListPolicyVersionsError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListPolicyVersionsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListPolicyVersionsError::Validation(error_message.to_string())
                    }
                    _ => ListPolicyVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPolicyVersionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListPolicyVersionsError {
    fn from(err: serde_json::error::Error) -> ListPolicyVersionsError {
        ListPolicyVersionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPolicyVersionsError {
    fn from(err: CredentialsError) -> ListPolicyVersionsError {
        ListPolicyVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPolicyVersionsError {
    fn from(err: HttpDispatchError) -> ListPolicyVersionsError {
        ListPolicyVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPolicyVersionsError {
    fn from(err: io::Error) -> ListPolicyVersionsError {
        ListPolicyVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPolicyVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPolicyVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListPolicyVersionsError::InternalFailure(ref cause) => cause,
            ListPolicyVersionsError::InvalidRequest(ref cause) => cause,
            ListPolicyVersionsError::ResourceNotFound(ref cause) => cause,
            ListPolicyVersionsError::ServiceUnavailable(ref cause) => cause,
            ListPolicyVersionsError::Throttling(ref cause) => cause,
            ListPolicyVersionsError::Unauthorized(ref cause) => cause,
            ListPolicyVersionsError::Validation(ref cause) => cause,
            ListPolicyVersionsError::Credentials(ref err) => err.description(),
            ListPolicyVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPolicyVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPrincipalPolicies
#[derive(Debug, PartialEq)]
pub enum ListPrincipalPoliciesError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListPrincipalPoliciesError {
    pub fn from_body(body: &str) -> ListPrincipalPoliciesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListPrincipalPoliciesError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListPrincipalPoliciesError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListPrincipalPoliciesError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListPrincipalPoliciesError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListPrincipalPoliciesError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListPrincipalPoliciesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListPrincipalPoliciesError::Validation(error_message.to_string())
                    }
                    _ => ListPrincipalPoliciesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPrincipalPoliciesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListPrincipalPoliciesError {
    fn from(err: serde_json::error::Error) -> ListPrincipalPoliciesError {
        ListPrincipalPoliciesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPrincipalPoliciesError {
    fn from(err: CredentialsError) -> ListPrincipalPoliciesError {
        ListPrincipalPoliciesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPrincipalPoliciesError {
    fn from(err: HttpDispatchError) -> ListPrincipalPoliciesError {
        ListPrincipalPoliciesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPrincipalPoliciesError {
    fn from(err: io::Error) -> ListPrincipalPoliciesError {
        ListPrincipalPoliciesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPrincipalPoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPrincipalPoliciesError {
    fn description(&self) -> &str {
        match *self {
            ListPrincipalPoliciesError::InternalFailure(ref cause) => cause,
            ListPrincipalPoliciesError::InvalidRequest(ref cause) => cause,
            ListPrincipalPoliciesError::ResourceNotFound(ref cause) => cause,
            ListPrincipalPoliciesError::ServiceUnavailable(ref cause) => cause,
            ListPrincipalPoliciesError::Throttling(ref cause) => cause,
            ListPrincipalPoliciesError::Unauthorized(ref cause) => cause,
            ListPrincipalPoliciesError::Validation(ref cause) => cause,
            ListPrincipalPoliciesError::Credentials(ref err) => err.description(),
            ListPrincipalPoliciesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPrincipalPoliciesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPrincipalThings
#[derive(Debug, PartialEq)]
pub enum ListPrincipalThingsError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListPrincipalThingsError {
    pub fn from_body(body: &str) -> ListPrincipalThingsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListPrincipalThingsError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListPrincipalThingsError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListPrincipalThingsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListPrincipalThingsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListPrincipalThingsError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListPrincipalThingsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListPrincipalThingsError::Validation(error_message.to_string())
                    }
                    _ => ListPrincipalThingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListPrincipalThingsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListPrincipalThingsError {
    fn from(err: serde_json::error::Error) -> ListPrincipalThingsError {
        ListPrincipalThingsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPrincipalThingsError {
    fn from(err: CredentialsError) -> ListPrincipalThingsError {
        ListPrincipalThingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPrincipalThingsError {
    fn from(err: HttpDispatchError) -> ListPrincipalThingsError {
        ListPrincipalThingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPrincipalThingsError {
    fn from(err: io::Error) -> ListPrincipalThingsError {
        ListPrincipalThingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListPrincipalThingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPrincipalThingsError {
    fn description(&self) -> &str {
        match *self {
            ListPrincipalThingsError::InternalFailure(ref cause) => cause,
            ListPrincipalThingsError::InvalidRequest(ref cause) => cause,
            ListPrincipalThingsError::ResourceNotFound(ref cause) => cause,
            ListPrincipalThingsError::ServiceUnavailable(ref cause) => cause,
            ListPrincipalThingsError::Throttling(ref cause) => cause,
            ListPrincipalThingsError::Unauthorized(ref cause) => cause,
            ListPrincipalThingsError::Validation(ref cause) => cause,
            ListPrincipalThingsError::Credentials(ref err) => err.description(),
            ListPrincipalThingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListPrincipalThingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRoleAliases
#[derive(Debug, PartialEq)]
pub enum ListRoleAliasesError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListRoleAliasesError {
    pub fn from_body(body: &str) -> ListRoleAliasesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListRoleAliasesError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListRoleAliasesError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListRoleAliasesError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListRoleAliasesError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListRoleAliasesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListRoleAliasesError::Validation(error_message.to_string())
                    }
                    _ => ListRoleAliasesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListRoleAliasesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListRoleAliasesError {
    fn from(err: serde_json::error::Error) -> ListRoleAliasesError {
        ListRoleAliasesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListRoleAliasesError {
    fn from(err: CredentialsError) -> ListRoleAliasesError {
        ListRoleAliasesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListRoleAliasesError {
    fn from(err: HttpDispatchError) -> ListRoleAliasesError {
        ListRoleAliasesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListRoleAliasesError {
    fn from(err: io::Error) -> ListRoleAliasesError {
        ListRoleAliasesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListRoleAliasesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRoleAliasesError {
    fn description(&self) -> &str {
        match *self {
            ListRoleAliasesError::InternalFailure(ref cause) => cause,
            ListRoleAliasesError::InvalidRequest(ref cause) => cause,
            ListRoleAliasesError::ServiceUnavailable(ref cause) => cause,
            ListRoleAliasesError::Throttling(ref cause) => cause,
            ListRoleAliasesError::Unauthorized(ref cause) => cause,
            ListRoleAliasesError::Validation(ref cause) => cause,
            ListRoleAliasesError::Credentials(ref err) => err.description(),
            ListRoleAliasesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListRoleAliasesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListStreams
#[derive(Debug, PartialEq)]
pub enum ListStreamsError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListStreamsError {
    pub fn from_body(body: &str) -> ListStreamsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListStreamsError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListStreamsError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListStreamsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListStreamsError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListStreamsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListStreamsError::Validation(error_message.to_string())
                    }
                    _ => ListStreamsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListStreamsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListStreamsError {
    fn from(err: serde_json::error::Error) -> ListStreamsError {
        ListStreamsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListStreamsError {
    fn from(err: CredentialsError) -> ListStreamsError {
        ListStreamsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListStreamsError {
    fn from(err: HttpDispatchError) -> ListStreamsError {
        ListStreamsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListStreamsError {
    fn from(err: io::Error) -> ListStreamsError {
        ListStreamsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListStreamsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListStreamsError {
    fn description(&self) -> &str {
        match *self {
            ListStreamsError::InternalFailure(ref cause) => cause,
            ListStreamsError::InvalidRequest(ref cause) => cause,
            ListStreamsError::ServiceUnavailable(ref cause) => cause,
            ListStreamsError::Throttling(ref cause) => cause,
            ListStreamsError::Unauthorized(ref cause) => cause,
            ListStreamsError::Validation(ref cause) => cause,
            ListStreamsError::Credentials(ref err) => err.description(),
            ListStreamsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListStreamsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTargetsForPolicy
#[derive(Debug, PartialEq)]
pub enum ListTargetsForPolicyError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The number of attached entities exceeds the limit.</p>
    LimitExceeded(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTargetsForPolicyError {
    pub fn from_body(body: &str) -> ListTargetsForPolicyError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListTargetsForPolicyError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListTargetsForPolicyError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListTargetsForPolicyError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListTargetsForPolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListTargetsForPolicyError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListTargetsForPolicyError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListTargetsForPolicyError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTargetsForPolicyError::Validation(error_message.to_string())
                    }
                    _ => ListTargetsForPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTargetsForPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTargetsForPolicyError {
    fn from(err: serde_json::error::Error) -> ListTargetsForPolicyError {
        ListTargetsForPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTargetsForPolicyError {
    fn from(err: CredentialsError) -> ListTargetsForPolicyError {
        ListTargetsForPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTargetsForPolicyError {
    fn from(err: HttpDispatchError) -> ListTargetsForPolicyError {
        ListTargetsForPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTargetsForPolicyError {
    fn from(err: io::Error) -> ListTargetsForPolicyError {
        ListTargetsForPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTargetsForPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTargetsForPolicyError {
    fn description(&self) -> &str {
        match *self {
            ListTargetsForPolicyError::InternalFailure(ref cause) => cause,
            ListTargetsForPolicyError::InvalidRequest(ref cause) => cause,
            ListTargetsForPolicyError::LimitExceeded(ref cause) => cause,
            ListTargetsForPolicyError::ResourceNotFound(ref cause) => cause,
            ListTargetsForPolicyError::ServiceUnavailable(ref cause) => cause,
            ListTargetsForPolicyError::Throttling(ref cause) => cause,
            ListTargetsForPolicyError::Unauthorized(ref cause) => cause,
            ListTargetsForPolicyError::Validation(ref cause) => cause,
            ListTargetsForPolicyError::Credentials(ref err) => err.description(),
            ListTargetsForPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTargetsForPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListThingGroups
#[derive(Debug, PartialEq)]
pub enum ListThingGroupsError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListThingGroupsError {
    pub fn from_body(body: &str) -> ListThingGroupsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListThingGroupsError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListThingGroupsError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListThingGroupsError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListThingGroupsError::Validation(error_message.to_string())
                    }
                    _ => ListThingGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListThingGroupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListThingGroupsError {
    fn from(err: serde_json::error::Error) -> ListThingGroupsError {
        ListThingGroupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListThingGroupsError {
    fn from(err: CredentialsError) -> ListThingGroupsError {
        ListThingGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListThingGroupsError {
    fn from(err: HttpDispatchError) -> ListThingGroupsError {
        ListThingGroupsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListThingGroupsError {
    fn from(err: io::Error) -> ListThingGroupsError {
        ListThingGroupsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListThingGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListThingGroupsError {
    fn description(&self) -> &str {
        match *self {
            ListThingGroupsError::InternalFailure(ref cause) => cause,
            ListThingGroupsError::InvalidRequest(ref cause) => cause,
            ListThingGroupsError::ResourceNotFound(ref cause) => cause,
            ListThingGroupsError::Validation(ref cause) => cause,
            ListThingGroupsError::Credentials(ref err) => err.description(),
            ListThingGroupsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListThingGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListThingGroupsForThing
#[derive(Debug, PartialEq)]
pub enum ListThingGroupsForThingError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListThingGroupsForThingError {
    pub fn from_body(body: &str) -> ListThingGroupsForThingError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListThingGroupsForThingError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListThingGroupsForThingError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListThingGroupsForThingError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListThingGroupsForThingError::Validation(error_message.to_string())
                    }
                    _ => ListThingGroupsForThingError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListThingGroupsForThingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListThingGroupsForThingError {
    fn from(err: serde_json::error::Error) -> ListThingGroupsForThingError {
        ListThingGroupsForThingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListThingGroupsForThingError {
    fn from(err: CredentialsError) -> ListThingGroupsForThingError {
        ListThingGroupsForThingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListThingGroupsForThingError {
    fn from(err: HttpDispatchError) -> ListThingGroupsForThingError {
        ListThingGroupsForThingError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListThingGroupsForThingError {
    fn from(err: io::Error) -> ListThingGroupsForThingError {
        ListThingGroupsForThingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListThingGroupsForThingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListThingGroupsForThingError {
    fn description(&self) -> &str {
        match *self {
            ListThingGroupsForThingError::InternalFailure(ref cause) => cause,
            ListThingGroupsForThingError::InvalidRequest(ref cause) => cause,
            ListThingGroupsForThingError::ResourceNotFound(ref cause) => cause,
            ListThingGroupsForThingError::Validation(ref cause) => cause,
            ListThingGroupsForThingError::Credentials(ref err) => err.description(),
            ListThingGroupsForThingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListThingGroupsForThingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListThingPrincipals
#[derive(Debug, PartialEq)]
pub enum ListThingPrincipalsError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListThingPrincipalsError {
    pub fn from_body(body: &str) -> ListThingPrincipalsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListThingPrincipalsError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListThingPrincipalsError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListThingPrincipalsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListThingPrincipalsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListThingPrincipalsError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListThingPrincipalsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListThingPrincipalsError::Validation(error_message.to_string())
                    }
                    _ => ListThingPrincipalsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListThingPrincipalsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListThingPrincipalsError {
    fn from(err: serde_json::error::Error) -> ListThingPrincipalsError {
        ListThingPrincipalsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListThingPrincipalsError {
    fn from(err: CredentialsError) -> ListThingPrincipalsError {
        ListThingPrincipalsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListThingPrincipalsError {
    fn from(err: HttpDispatchError) -> ListThingPrincipalsError {
        ListThingPrincipalsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListThingPrincipalsError {
    fn from(err: io::Error) -> ListThingPrincipalsError {
        ListThingPrincipalsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListThingPrincipalsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListThingPrincipalsError {
    fn description(&self) -> &str {
        match *self {
            ListThingPrincipalsError::InternalFailure(ref cause) => cause,
            ListThingPrincipalsError::InvalidRequest(ref cause) => cause,
            ListThingPrincipalsError::ResourceNotFound(ref cause) => cause,
            ListThingPrincipalsError::ServiceUnavailable(ref cause) => cause,
            ListThingPrincipalsError::Throttling(ref cause) => cause,
            ListThingPrincipalsError::Unauthorized(ref cause) => cause,
            ListThingPrincipalsError::Validation(ref cause) => cause,
            ListThingPrincipalsError::Credentials(ref err) => err.description(),
            ListThingPrincipalsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListThingPrincipalsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListThingRegistrationTaskReports
#[derive(Debug, PartialEq)]
pub enum ListThingRegistrationTaskReportsError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListThingRegistrationTaskReportsError {
    pub fn from_body(body: &str) -> ListThingRegistrationTaskReportsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListThingRegistrationTaskReportsError::InternalFailure(String::from(
                            error_message,
                        ))
                    }
                    "InvalidRequestException" => {
                        ListThingRegistrationTaskReportsError::InvalidRequest(String::from(
                            error_message,
                        ))
                    }
                    "ThrottlingException" => ListThingRegistrationTaskReportsError::Throttling(
                        String::from(error_message),
                    ),
                    "UnauthorizedException" => {
                        ListThingRegistrationTaskReportsError::Unauthorized(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        ListThingRegistrationTaskReportsError::Validation(error_message.to_string())
                    }
                    _ => ListThingRegistrationTaskReportsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListThingRegistrationTaskReportsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListThingRegistrationTaskReportsError {
    fn from(err: serde_json::error::Error) -> ListThingRegistrationTaskReportsError {
        ListThingRegistrationTaskReportsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListThingRegistrationTaskReportsError {
    fn from(err: CredentialsError) -> ListThingRegistrationTaskReportsError {
        ListThingRegistrationTaskReportsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListThingRegistrationTaskReportsError {
    fn from(err: HttpDispatchError) -> ListThingRegistrationTaskReportsError {
        ListThingRegistrationTaskReportsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListThingRegistrationTaskReportsError {
    fn from(err: io::Error) -> ListThingRegistrationTaskReportsError {
        ListThingRegistrationTaskReportsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListThingRegistrationTaskReportsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListThingRegistrationTaskReportsError {
    fn description(&self) -> &str {
        match *self {
            ListThingRegistrationTaskReportsError::InternalFailure(ref cause) => cause,
            ListThingRegistrationTaskReportsError::InvalidRequest(ref cause) => cause,
            ListThingRegistrationTaskReportsError::Throttling(ref cause) => cause,
            ListThingRegistrationTaskReportsError::Unauthorized(ref cause) => cause,
            ListThingRegistrationTaskReportsError::Validation(ref cause) => cause,
            ListThingRegistrationTaskReportsError::Credentials(ref err) => err.description(),
            ListThingRegistrationTaskReportsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListThingRegistrationTaskReportsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListThingRegistrationTasks
#[derive(Debug, PartialEq)]
pub enum ListThingRegistrationTasksError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListThingRegistrationTasksError {
    pub fn from_body(body: &str) -> ListThingRegistrationTasksError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListThingRegistrationTasksError::InternalFailure(String::from(
                            error_message,
                        ))
                    }
                    "InvalidRequestException" => {
                        ListThingRegistrationTasksError::InvalidRequest(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListThingRegistrationTasksError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListThingRegistrationTasksError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListThingRegistrationTasksError::Validation(error_message.to_string())
                    }
                    _ => ListThingRegistrationTasksError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListThingRegistrationTasksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListThingRegistrationTasksError {
    fn from(err: serde_json::error::Error) -> ListThingRegistrationTasksError {
        ListThingRegistrationTasksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListThingRegistrationTasksError {
    fn from(err: CredentialsError) -> ListThingRegistrationTasksError {
        ListThingRegistrationTasksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListThingRegistrationTasksError {
    fn from(err: HttpDispatchError) -> ListThingRegistrationTasksError {
        ListThingRegistrationTasksError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListThingRegistrationTasksError {
    fn from(err: io::Error) -> ListThingRegistrationTasksError {
        ListThingRegistrationTasksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListThingRegistrationTasksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListThingRegistrationTasksError {
    fn description(&self) -> &str {
        match *self {
            ListThingRegistrationTasksError::InternalFailure(ref cause) => cause,
            ListThingRegistrationTasksError::InvalidRequest(ref cause) => cause,
            ListThingRegistrationTasksError::Throttling(ref cause) => cause,
            ListThingRegistrationTasksError::Unauthorized(ref cause) => cause,
            ListThingRegistrationTasksError::Validation(ref cause) => cause,
            ListThingRegistrationTasksError::Credentials(ref err) => err.description(),
            ListThingRegistrationTasksError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListThingRegistrationTasksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListThingTypes
#[derive(Debug, PartialEq)]
pub enum ListThingTypesError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListThingTypesError {
    pub fn from_body(body: &str) -> ListThingTypesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListThingTypesError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListThingTypesError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListThingTypesError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListThingTypesError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListThingTypesError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListThingTypesError::Validation(error_message.to_string())
                    }
                    _ => ListThingTypesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListThingTypesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListThingTypesError {
    fn from(err: serde_json::error::Error) -> ListThingTypesError {
        ListThingTypesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListThingTypesError {
    fn from(err: CredentialsError) -> ListThingTypesError {
        ListThingTypesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListThingTypesError {
    fn from(err: HttpDispatchError) -> ListThingTypesError {
        ListThingTypesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListThingTypesError {
    fn from(err: io::Error) -> ListThingTypesError {
        ListThingTypesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListThingTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListThingTypesError {
    fn description(&self) -> &str {
        match *self {
            ListThingTypesError::InternalFailure(ref cause) => cause,
            ListThingTypesError::InvalidRequest(ref cause) => cause,
            ListThingTypesError::ServiceUnavailable(ref cause) => cause,
            ListThingTypesError::Throttling(ref cause) => cause,
            ListThingTypesError::Unauthorized(ref cause) => cause,
            ListThingTypesError::Validation(ref cause) => cause,
            ListThingTypesError::Credentials(ref err) => err.description(),
            ListThingTypesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListThingTypesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListThings
#[derive(Debug, PartialEq)]
pub enum ListThingsError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListThingsError {
    pub fn from_body(body: &str) -> ListThingsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListThingsError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListThingsError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListThingsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        ListThingsError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ListThingsError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => ListThingsError::Validation(error_message.to_string()),
                    _ => ListThingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListThingsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListThingsError {
    fn from(err: serde_json::error::Error) -> ListThingsError {
        ListThingsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListThingsError {
    fn from(err: CredentialsError) -> ListThingsError {
        ListThingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListThingsError {
    fn from(err: HttpDispatchError) -> ListThingsError {
        ListThingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListThingsError {
    fn from(err: io::Error) -> ListThingsError {
        ListThingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListThingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListThingsError {
    fn description(&self) -> &str {
        match *self {
            ListThingsError::InternalFailure(ref cause) => cause,
            ListThingsError::InvalidRequest(ref cause) => cause,
            ListThingsError::ServiceUnavailable(ref cause) => cause,
            ListThingsError::Throttling(ref cause) => cause,
            ListThingsError::Unauthorized(ref cause) => cause,
            ListThingsError::Validation(ref cause) => cause,
            ListThingsError::Credentials(ref err) => err.description(),
            ListThingsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListThingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListThingsInThingGroup
#[derive(Debug, PartialEq)]
pub enum ListThingsInThingGroupError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListThingsInThingGroupError {
    pub fn from_body(body: &str) -> ListThingsInThingGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        ListThingsInThingGroupError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListThingsInThingGroupError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListThingsInThingGroupError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListThingsInThingGroupError::Validation(error_message.to_string())
                    }
                    _ => ListThingsInThingGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListThingsInThingGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListThingsInThingGroupError {
    fn from(err: serde_json::error::Error) -> ListThingsInThingGroupError {
        ListThingsInThingGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListThingsInThingGroupError {
    fn from(err: CredentialsError) -> ListThingsInThingGroupError {
        ListThingsInThingGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListThingsInThingGroupError {
    fn from(err: HttpDispatchError) -> ListThingsInThingGroupError {
        ListThingsInThingGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListThingsInThingGroupError {
    fn from(err: io::Error) -> ListThingsInThingGroupError {
        ListThingsInThingGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListThingsInThingGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListThingsInThingGroupError {
    fn description(&self) -> &str {
        match *self {
            ListThingsInThingGroupError::InternalFailure(ref cause) => cause,
            ListThingsInThingGroupError::InvalidRequest(ref cause) => cause,
            ListThingsInThingGroupError::ResourceNotFound(ref cause) => cause,
            ListThingsInThingGroupError::Validation(ref cause) => cause,
            ListThingsInThingGroupError::Credentials(ref err) => err.description(),
            ListThingsInThingGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListThingsInThingGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTopicRules
#[derive(Debug, PartialEq)]
pub enum ListTopicRulesError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTopicRulesError {
    pub fn from_body(body: &str) -> ListTopicRulesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        ListTopicRulesError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListTopicRulesError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListTopicRulesError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTopicRulesError::Validation(error_message.to_string())
                    }
                    _ => ListTopicRulesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTopicRulesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTopicRulesError {
    fn from(err: serde_json::error::Error) -> ListTopicRulesError {
        ListTopicRulesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTopicRulesError {
    fn from(err: CredentialsError) -> ListTopicRulesError {
        ListTopicRulesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTopicRulesError {
    fn from(err: HttpDispatchError) -> ListTopicRulesError {
        ListTopicRulesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTopicRulesError {
    fn from(err: io::Error) -> ListTopicRulesError {
        ListTopicRulesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTopicRulesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTopicRulesError {
    fn description(&self) -> &str {
        match *self {
            ListTopicRulesError::Internal(ref cause) => cause,
            ListTopicRulesError::InvalidRequest(ref cause) => cause,
            ListTopicRulesError::ServiceUnavailable(ref cause) => cause,
            ListTopicRulesError::Validation(ref cause) => cause,
            ListTopicRulesError::Credentials(ref err) => err.description(),
            ListTopicRulesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListTopicRulesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListV2LoggingLevels
#[derive(Debug, PartialEq)]
pub enum ListV2LoggingLevelsError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The resource is not configured.</p>
    NotConfigured(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListV2LoggingLevelsError {
    pub fn from_body(body: &str) -> ListV2LoggingLevelsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        ListV2LoggingLevelsError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ListV2LoggingLevelsError::InvalidRequest(String::from(error_message))
                    }
                    "NotConfiguredException" => {
                        ListV2LoggingLevelsError::NotConfigured(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ListV2LoggingLevelsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListV2LoggingLevelsError::Validation(error_message.to_string())
                    }
                    _ => ListV2LoggingLevelsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListV2LoggingLevelsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListV2LoggingLevelsError {
    fn from(err: serde_json::error::Error) -> ListV2LoggingLevelsError {
        ListV2LoggingLevelsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListV2LoggingLevelsError {
    fn from(err: CredentialsError) -> ListV2LoggingLevelsError {
        ListV2LoggingLevelsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListV2LoggingLevelsError {
    fn from(err: HttpDispatchError) -> ListV2LoggingLevelsError {
        ListV2LoggingLevelsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListV2LoggingLevelsError {
    fn from(err: io::Error) -> ListV2LoggingLevelsError {
        ListV2LoggingLevelsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListV2LoggingLevelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListV2LoggingLevelsError {
    fn description(&self) -> &str {
        match *self {
            ListV2LoggingLevelsError::Internal(ref cause) => cause,
            ListV2LoggingLevelsError::InvalidRequest(ref cause) => cause,
            ListV2LoggingLevelsError::NotConfigured(ref cause) => cause,
            ListV2LoggingLevelsError::ServiceUnavailable(ref cause) => cause,
            ListV2LoggingLevelsError::Validation(ref cause) => cause,
            ListV2LoggingLevelsError::Credentials(ref err) => err.description(),
            ListV2LoggingLevelsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListV2LoggingLevelsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterCACertificate
#[derive(Debug, PartialEq)]
pub enum RegisterCACertificateError {
    ///<p>The certificate is invalid.</p>
    CertificateValidation(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The number of attached entities exceeds the limit.</p>
    LimitExceeded(String),
    ///<p>The registration code is invalid.</p>
    RegistrationCodeValidation(String),
    ///<p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RegisterCACertificateError {
    pub fn from_body(body: &str) -> RegisterCACertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CertificateValidationException" => {
                        RegisterCACertificateError::CertificateValidation(String::from(
                            error_message,
                        ))
                    }
                    "InternalFailureException" => {
                        RegisterCACertificateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        RegisterCACertificateError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        RegisterCACertificateError::LimitExceeded(String::from(error_message))
                    }
                    "RegistrationCodeValidationException" => {
                        RegisterCACertificateError::RegistrationCodeValidation(String::from(
                            error_message,
                        ))
                    }
                    "ResourceAlreadyExistsException" => {
                        RegisterCACertificateError::ResourceAlreadyExists(String::from(
                            error_message,
                        ))
                    }
                    "ServiceUnavailableException" => {
                        RegisterCACertificateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        RegisterCACertificateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        RegisterCACertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        RegisterCACertificateError::Validation(error_message.to_string())
                    }
                    _ => RegisterCACertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterCACertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterCACertificateError {
    fn from(err: serde_json::error::Error) -> RegisterCACertificateError {
        RegisterCACertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterCACertificateError {
    fn from(err: CredentialsError) -> RegisterCACertificateError {
        RegisterCACertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterCACertificateError {
    fn from(err: HttpDispatchError) -> RegisterCACertificateError {
        RegisterCACertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterCACertificateError {
    fn from(err: io::Error) -> RegisterCACertificateError {
        RegisterCACertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterCACertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterCACertificateError {
    fn description(&self) -> &str {
        match *self {
            RegisterCACertificateError::CertificateValidation(ref cause) => cause,
            RegisterCACertificateError::InternalFailure(ref cause) => cause,
            RegisterCACertificateError::InvalidRequest(ref cause) => cause,
            RegisterCACertificateError::LimitExceeded(ref cause) => cause,
            RegisterCACertificateError::RegistrationCodeValidation(ref cause) => cause,
            RegisterCACertificateError::ResourceAlreadyExists(ref cause) => cause,
            RegisterCACertificateError::ServiceUnavailable(ref cause) => cause,
            RegisterCACertificateError::Throttling(ref cause) => cause,
            RegisterCACertificateError::Unauthorized(ref cause) => cause,
            RegisterCACertificateError::Validation(ref cause) => cause,
            RegisterCACertificateError::Credentials(ref err) => err.description(),
            RegisterCACertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterCACertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterCertificate
#[derive(Debug, PartialEq)]
pub enum RegisterCertificateError {
    ///<p>Unable to verify the CA certificate used to sign the device certificate you are attempting to register. This is happens when you have registered more than one CA certificate that has the same subject field and public key.</p>
    CertificateConflict(String),
    ///<p>The certificate operation is not allowed.</p>
    CertificateState(String),
    ///<p>The certificate is invalid.</p>
    CertificateValidation(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The resource already exists.</p>
    ResourceAlreadyExists(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RegisterCertificateError {
    pub fn from_body(body: &str) -> RegisterCertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CertificateConflictException" => {
                        RegisterCertificateError::CertificateConflict(String::from(error_message))
                    }
                    "CertificateStateException" => {
                        RegisterCertificateError::CertificateState(String::from(error_message))
                    }
                    "CertificateValidationException" => {
                        RegisterCertificateError::CertificateValidation(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        RegisterCertificateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        RegisterCertificateError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceAlreadyExistsException" => {
                        RegisterCertificateError::ResourceAlreadyExists(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        RegisterCertificateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        RegisterCertificateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        RegisterCertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        RegisterCertificateError::Validation(error_message.to_string())
                    }
                    _ => RegisterCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterCertificateError {
    fn from(err: serde_json::error::Error) -> RegisterCertificateError {
        RegisterCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterCertificateError {
    fn from(err: CredentialsError) -> RegisterCertificateError {
        RegisterCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterCertificateError {
    fn from(err: HttpDispatchError) -> RegisterCertificateError {
        RegisterCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterCertificateError {
    fn from(err: io::Error) -> RegisterCertificateError {
        RegisterCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterCertificateError {
    fn description(&self) -> &str {
        match *self {
            RegisterCertificateError::CertificateConflict(ref cause) => cause,
            RegisterCertificateError::CertificateState(ref cause) => cause,
            RegisterCertificateError::CertificateValidation(ref cause) => cause,
            RegisterCertificateError::InternalFailure(ref cause) => cause,
            RegisterCertificateError::InvalidRequest(ref cause) => cause,
            RegisterCertificateError::ResourceAlreadyExists(ref cause) => cause,
            RegisterCertificateError::ServiceUnavailable(ref cause) => cause,
            RegisterCertificateError::Throttling(ref cause) => cause,
            RegisterCertificateError::Unauthorized(ref cause) => cause,
            RegisterCertificateError::Validation(ref cause) => cause,
            RegisterCertificateError::Credentials(ref err) => err.description(),
            RegisterCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RegisterCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterThing
#[derive(Debug, PartialEq)]
pub enum RegisterThingError {
    ///<p>A conflicting resource update exception. This exception is thrown when two pending updates cause a conflict.</p>
    ConflictingResourceUpdate(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The resource registration failed.</p>
    ResourceRegistrationFailure(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RegisterThingError {
    pub fn from_body(body: &str) -> RegisterThingError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConflictingResourceUpdateException" => {
                        RegisterThingError::ConflictingResourceUpdate(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        RegisterThingError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        RegisterThingError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceRegistrationFailureException" => {
                        RegisterThingError::ResourceRegistrationFailure(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        RegisterThingError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        RegisterThingError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        RegisterThingError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        RegisterThingError::Validation(error_message.to_string())
                    }
                    _ => RegisterThingError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterThingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterThingError {
    fn from(err: serde_json::error::Error) -> RegisterThingError {
        RegisterThingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterThingError {
    fn from(err: CredentialsError) -> RegisterThingError {
        RegisterThingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterThingError {
    fn from(err: HttpDispatchError) -> RegisterThingError {
        RegisterThingError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterThingError {
    fn from(err: io::Error) -> RegisterThingError {
        RegisterThingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterThingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterThingError {
    fn description(&self) -> &str {
        match *self {
            RegisterThingError::ConflictingResourceUpdate(ref cause) => cause,
            RegisterThingError::InternalFailure(ref cause) => cause,
            RegisterThingError::InvalidRequest(ref cause) => cause,
            RegisterThingError::ResourceRegistrationFailure(ref cause) => cause,
            RegisterThingError::ServiceUnavailable(ref cause) => cause,
            RegisterThingError::Throttling(ref cause) => cause,
            RegisterThingError::Unauthorized(ref cause) => cause,
            RegisterThingError::Validation(ref cause) => cause,
            RegisterThingError::Credentials(ref err) => err.description(),
            RegisterThingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RegisterThingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RejectCertificateTransfer
#[derive(Debug, PartialEq)]
pub enum RejectCertificateTransferError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You can't revert the certificate transfer because the transfer is already complete.</p>
    TransferAlreadyCompleted(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RejectCertificateTransferError {
    pub fn from_body(body: &str) -> RejectCertificateTransferError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        RejectCertificateTransferError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        RejectCertificateTransferError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        RejectCertificateTransferError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ServiceUnavailableException" => {
                        RejectCertificateTransferError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ThrottlingException" => {
                        RejectCertificateTransferError::Throttling(String::from(error_message))
                    }
                    "TransferAlreadyCompletedException" => {
                        RejectCertificateTransferError::TransferAlreadyCompleted(String::from(
                            error_message,
                        ))
                    }
                    "UnauthorizedException" => {
                        RejectCertificateTransferError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        RejectCertificateTransferError::Validation(error_message.to_string())
                    }
                    _ => RejectCertificateTransferError::Unknown(String::from(body)),
                }
            }
            Err(_) => RejectCertificateTransferError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RejectCertificateTransferError {
    fn from(err: serde_json::error::Error) -> RejectCertificateTransferError {
        RejectCertificateTransferError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RejectCertificateTransferError {
    fn from(err: CredentialsError) -> RejectCertificateTransferError {
        RejectCertificateTransferError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RejectCertificateTransferError {
    fn from(err: HttpDispatchError) -> RejectCertificateTransferError {
        RejectCertificateTransferError::HttpDispatch(err)
    }
}
impl From<io::Error> for RejectCertificateTransferError {
    fn from(err: io::Error) -> RejectCertificateTransferError {
        RejectCertificateTransferError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RejectCertificateTransferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RejectCertificateTransferError {
    fn description(&self) -> &str {
        match *self {
            RejectCertificateTransferError::InternalFailure(ref cause) => cause,
            RejectCertificateTransferError::InvalidRequest(ref cause) => cause,
            RejectCertificateTransferError::ResourceNotFound(ref cause) => cause,
            RejectCertificateTransferError::ServiceUnavailable(ref cause) => cause,
            RejectCertificateTransferError::Throttling(ref cause) => cause,
            RejectCertificateTransferError::TransferAlreadyCompleted(ref cause) => cause,
            RejectCertificateTransferError::Unauthorized(ref cause) => cause,
            RejectCertificateTransferError::Validation(ref cause) => cause,
            RejectCertificateTransferError::Credentials(ref err) => err.description(),
            RejectCertificateTransferError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RejectCertificateTransferError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveThingFromThingGroup
#[derive(Debug, PartialEq)]
pub enum RemoveThingFromThingGroupError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RemoveThingFromThingGroupError {
    pub fn from_body(body: &str) -> RemoveThingFromThingGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        RemoveThingFromThingGroupError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        RemoveThingFromThingGroupError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        RemoveThingFromThingGroupError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ThrottlingException" => {
                        RemoveThingFromThingGroupError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        RemoveThingFromThingGroupError::Validation(error_message.to_string())
                    }
                    _ => RemoveThingFromThingGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemoveThingFromThingGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RemoveThingFromThingGroupError {
    fn from(err: serde_json::error::Error) -> RemoveThingFromThingGroupError {
        RemoveThingFromThingGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveThingFromThingGroupError {
    fn from(err: CredentialsError) -> RemoveThingFromThingGroupError {
        RemoveThingFromThingGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveThingFromThingGroupError {
    fn from(err: HttpDispatchError) -> RemoveThingFromThingGroupError {
        RemoveThingFromThingGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveThingFromThingGroupError {
    fn from(err: io::Error) -> RemoveThingFromThingGroupError {
        RemoveThingFromThingGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveThingFromThingGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveThingFromThingGroupError {
    fn description(&self) -> &str {
        match *self {
            RemoveThingFromThingGroupError::InternalFailure(ref cause) => cause,
            RemoveThingFromThingGroupError::InvalidRequest(ref cause) => cause,
            RemoveThingFromThingGroupError::ResourceNotFound(ref cause) => cause,
            RemoveThingFromThingGroupError::Throttling(ref cause) => cause,
            RemoveThingFromThingGroupError::Validation(ref cause) => cause,
            RemoveThingFromThingGroupError::Credentials(ref err) => err.description(),
            RemoveThingFromThingGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveThingFromThingGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ReplaceTopicRule
#[derive(Debug, PartialEq)]
pub enum ReplaceTopicRuleError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The Rule-SQL expression can't be parsed correctly.</p>
    SqlParse(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ReplaceTopicRuleError {
    pub fn from_body(body: &str) -> ReplaceTopicRuleError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        ReplaceTopicRuleError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        ReplaceTopicRuleError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        ReplaceTopicRuleError::ServiceUnavailable(String::from(error_message))
                    }
                    "SqlParseException" => {
                        ReplaceTopicRuleError::SqlParse(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        ReplaceTopicRuleError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ReplaceTopicRuleError::Validation(error_message.to_string())
                    }
                    _ => ReplaceTopicRuleError::Unknown(String::from(body)),
                }
            }
            Err(_) => ReplaceTopicRuleError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ReplaceTopicRuleError {
    fn from(err: serde_json::error::Error) -> ReplaceTopicRuleError {
        ReplaceTopicRuleError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ReplaceTopicRuleError {
    fn from(err: CredentialsError) -> ReplaceTopicRuleError {
        ReplaceTopicRuleError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ReplaceTopicRuleError {
    fn from(err: HttpDispatchError) -> ReplaceTopicRuleError {
        ReplaceTopicRuleError::HttpDispatch(err)
    }
}
impl From<io::Error> for ReplaceTopicRuleError {
    fn from(err: io::Error) -> ReplaceTopicRuleError {
        ReplaceTopicRuleError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ReplaceTopicRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ReplaceTopicRuleError {
    fn description(&self) -> &str {
        match *self {
            ReplaceTopicRuleError::Internal(ref cause) => cause,
            ReplaceTopicRuleError::InvalidRequest(ref cause) => cause,
            ReplaceTopicRuleError::ServiceUnavailable(ref cause) => cause,
            ReplaceTopicRuleError::SqlParse(ref cause) => cause,
            ReplaceTopicRuleError::Unauthorized(ref cause) => cause,
            ReplaceTopicRuleError::Validation(ref cause) => cause,
            ReplaceTopicRuleError::Credentials(ref err) => err.description(),
            ReplaceTopicRuleError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ReplaceTopicRuleError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SearchIndex
#[derive(Debug, PartialEq)]
pub enum SearchIndexError {
    ///<p>The index is not ready.</p>
    IndexNotReady(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The query is invalid.</p>
    InvalidQuery(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SearchIndexError {
    pub fn from_body(body: &str) -> SearchIndexError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "IndexNotReadyException" => {
                        SearchIndexError::IndexNotReady(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        SearchIndexError::InternalFailure(String::from(error_message))
                    }
                    "InvalidQueryException" => {
                        SearchIndexError::InvalidQuery(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        SearchIndexError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        SearchIndexError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        SearchIndexError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        SearchIndexError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        SearchIndexError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        SearchIndexError::Validation(error_message.to_string())
                    }
                    _ => SearchIndexError::Unknown(String::from(body)),
                }
            }
            Err(_) => SearchIndexError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SearchIndexError {
    fn from(err: serde_json::error::Error) -> SearchIndexError {
        SearchIndexError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SearchIndexError {
    fn from(err: CredentialsError) -> SearchIndexError {
        SearchIndexError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SearchIndexError {
    fn from(err: HttpDispatchError) -> SearchIndexError {
        SearchIndexError::HttpDispatch(err)
    }
}
impl From<io::Error> for SearchIndexError {
    fn from(err: io::Error) -> SearchIndexError {
        SearchIndexError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SearchIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchIndexError {
    fn description(&self) -> &str {
        match *self {
            SearchIndexError::IndexNotReady(ref cause) => cause,
            SearchIndexError::InternalFailure(ref cause) => cause,
            SearchIndexError::InvalidQuery(ref cause) => cause,
            SearchIndexError::InvalidRequest(ref cause) => cause,
            SearchIndexError::ResourceNotFound(ref cause) => cause,
            SearchIndexError::ServiceUnavailable(ref cause) => cause,
            SearchIndexError::Throttling(ref cause) => cause,
            SearchIndexError::Unauthorized(ref cause) => cause,
            SearchIndexError::Validation(ref cause) => cause,
            SearchIndexError::Credentials(ref err) => err.description(),
            SearchIndexError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SearchIndexError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetDefaultAuthorizer
#[derive(Debug, PartialEq)]
pub enum SetDefaultAuthorizerError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetDefaultAuthorizerError {
    pub fn from_body(body: &str) -> SetDefaultAuthorizerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        SetDefaultAuthorizerError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        SetDefaultAuthorizerError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        SetDefaultAuthorizerError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        SetDefaultAuthorizerError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        SetDefaultAuthorizerError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        SetDefaultAuthorizerError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        SetDefaultAuthorizerError::Validation(error_message.to_string())
                    }
                    _ => SetDefaultAuthorizerError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetDefaultAuthorizerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetDefaultAuthorizerError {
    fn from(err: serde_json::error::Error) -> SetDefaultAuthorizerError {
        SetDefaultAuthorizerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetDefaultAuthorizerError {
    fn from(err: CredentialsError) -> SetDefaultAuthorizerError {
        SetDefaultAuthorizerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetDefaultAuthorizerError {
    fn from(err: HttpDispatchError) -> SetDefaultAuthorizerError {
        SetDefaultAuthorizerError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetDefaultAuthorizerError {
    fn from(err: io::Error) -> SetDefaultAuthorizerError {
        SetDefaultAuthorizerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetDefaultAuthorizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetDefaultAuthorizerError {
    fn description(&self) -> &str {
        match *self {
            SetDefaultAuthorizerError::InternalFailure(ref cause) => cause,
            SetDefaultAuthorizerError::InvalidRequest(ref cause) => cause,
            SetDefaultAuthorizerError::ResourceNotFound(ref cause) => cause,
            SetDefaultAuthorizerError::ServiceUnavailable(ref cause) => cause,
            SetDefaultAuthorizerError::Throttling(ref cause) => cause,
            SetDefaultAuthorizerError::Unauthorized(ref cause) => cause,
            SetDefaultAuthorizerError::Validation(ref cause) => cause,
            SetDefaultAuthorizerError::Credentials(ref err) => err.description(),
            SetDefaultAuthorizerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetDefaultAuthorizerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetDefaultPolicyVersion
#[derive(Debug, PartialEq)]
pub enum SetDefaultPolicyVersionError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetDefaultPolicyVersionError {
    pub fn from_body(body: &str) -> SetDefaultPolicyVersionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        SetDefaultPolicyVersionError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        SetDefaultPolicyVersionError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        SetDefaultPolicyVersionError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        SetDefaultPolicyVersionError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ThrottlingException" => {
                        SetDefaultPolicyVersionError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        SetDefaultPolicyVersionError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        SetDefaultPolicyVersionError::Validation(error_message.to_string())
                    }
                    _ => SetDefaultPolicyVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetDefaultPolicyVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetDefaultPolicyVersionError {
    fn from(err: serde_json::error::Error) -> SetDefaultPolicyVersionError {
        SetDefaultPolicyVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetDefaultPolicyVersionError {
    fn from(err: CredentialsError) -> SetDefaultPolicyVersionError {
        SetDefaultPolicyVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetDefaultPolicyVersionError {
    fn from(err: HttpDispatchError) -> SetDefaultPolicyVersionError {
        SetDefaultPolicyVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetDefaultPolicyVersionError {
    fn from(err: io::Error) -> SetDefaultPolicyVersionError {
        SetDefaultPolicyVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetDefaultPolicyVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetDefaultPolicyVersionError {
    fn description(&self) -> &str {
        match *self {
            SetDefaultPolicyVersionError::InternalFailure(ref cause) => cause,
            SetDefaultPolicyVersionError::InvalidRequest(ref cause) => cause,
            SetDefaultPolicyVersionError::ResourceNotFound(ref cause) => cause,
            SetDefaultPolicyVersionError::ServiceUnavailable(ref cause) => cause,
            SetDefaultPolicyVersionError::Throttling(ref cause) => cause,
            SetDefaultPolicyVersionError::Unauthorized(ref cause) => cause,
            SetDefaultPolicyVersionError::Validation(ref cause) => cause,
            SetDefaultPolicyVersionError::Credentials(ref err) => err.description(),
            SetDefaultPolicyVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetDefaultPolicyVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetLoggingOptions
#[derive(Debug, PartialEq)]
pub enum SetLoggingOptionsError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetLoggingOptionsError {
    pub fn from_body(body: &str) -> SetLoggingOptionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        SetLoggingOptionsError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        SetLoggingOptionsError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        SetLoggingOptionsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        SetLoggingOptionsError::Validation(error_message.to_string())
                    }
                    _ => SetLoggingOptionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetLoggingOptionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetLoggingOptionsError {
    fn from(err: serde_json::error::Error) -> SetLoggingOptionsError {
        SetLoggingOptionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetLoggingOptionsError {
    fn from(err: CredentialsError) -> SetLoggingOptionsError {
        SetLoggingOptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetLoggingOptionsError {
    fn from(err: HttpDispatchError) -> SetLoggingOptionsError {
        SetLoggingOptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetLoggingOptionsError {
    fn from(err: io::Error) -> SetLoggingOptionsError {
        SetLoggingOptionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetLoggingOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetLoggingOptionsError {
    fn description(&self) -> &str {
        match *self {
            SetLoggingOptionsError::Internal(ref cause) => cause,
            SetLoggingOptionsError::InvalidRequest(ref cause) => cause,
            SetLoggingOptionsError::ServiceUnavailable(ref cause) => cause,
            SetLoggingOptionsError::Validation(ref cause) => cause,
            SetLoggingOptionsError::Credentials(ref err) => err.description(),
            SetLoggingOptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetLoggingOptionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetV2LoggingLevel
#[derive(Debug, PartialEq)]
pub enum SetV2LoggingLevelError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The resource is not configured.</p>
    NotConfigured(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetV2LoggingLevelError {
    pub fn from_body(body: &str) -> SetV2LoggingLevelError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        SetV2LoggingLevelError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        SetV2LoggingLevelError::InvalidRequest(String::from(error_message))
                    }
                    "NotConfiguredException" => {
                        SetV2LoggingLevelError::NotConfigured(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        SetV2LoggingLevelError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        SetV2LoggingLevelError::Validation(error_message.to_string())
                    }
                    _ => SetV2LoggingLevelError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetV2LoggingLevelError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetV2LoggingLevelError {
    fn from(err: serde_json::error::Error) -> SetV2LoggingLevelError {
        SetV2LoggingLevelError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetV2LoggingLevelError {
    fn from(err: CredentialsError) -> SetV2LoggingLevelError {
        SetV2LoggingLevelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetV2LoggingLevelError {
    fn from(err: HttpDispatchError) -> SetV2LoggingLevelError {
        SetV2LoggingLevelError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetV2LoggingLevelError {
    fn from(err: io::Error) -> SetV2LoggingLevelError {
        SetV2LoggingLevelError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetV2LoggingLevelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetV2LoggingLevelError {
    fn description(&self) -> &str {
        match *self {
            SetV2LoggingLevelError::Internal(ref cause) => cause,
            SetV2LoggingLevelError::InvalidRequest(ref cause) => cause,
            SetV2LoggingLevelError::NotConfigured(ref cause) => cause,
            SetV2LoggingLevelError::ServiceUnavailable(ref cause) => cause,
            SetV2LoggingLevelError::Validation(ref cause) => cause,
            SetV2LoggingLevelError::Credentials(ref err) => err.description(),
            SetV2LoggingLevelError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetV2LoggingLevelError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetV2LoggingOptions
#[derive(Debug, PartialEq)]
pub enum SetV2LoggingOptionsError {
    ///<p>An unexpected error has occurred.</p>
    Internal(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl SetV2LoggingOptionsError {
    pub fn from_body(body: &str) -> SetV2LoggingOptionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalException" => {
                        SetV2LoggingOptionsError::Internal(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        SetV2LoggingOptionsError::InvalidRequest(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        SetV2LoggingOptionsError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        SetV2LoggingOptionsError::Validation(error_message.to_string())
                    }
                    _ => SetV2LoggingOptionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetV2LoggingOptionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetV2LoggingOptionsError {
    fn from(err: serde_json::error::Error) -> SetV2LoggingOptionsError {
        SetV2LoggingOptionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for SetV2LoggingOptionsError {
    fn from(err: CredentialsError) -> SetV2LoggingOptionsError {
        SetV2LoggingOptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetV2LoggingOptionsError {
    fn from(err: HttpDispatchError) -> SetV2LoggingOptionsError {
        SetV2LoggingOptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetV2LoggingOptionsError {
    fn from(err: io::Error) -> SetV2LoggingOptionsError {
        SetV2LoggingOptionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetV2LoggingOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetV2LoggingOptionsError {
    fn description(&self) -> &str {
        match *self {
            SetV2LoggingOptionsError::Internal(ref cause) => cause,
            SetV2LoggingOptionsError::InvalidRequest(ref cause) => cause,
            SetV2LoggingOptionsError::ServiceUnavailable(ref cause) => cause,
            SetV2LoggingOptionsError::Validation(ref cause) => cause,
            SetV2LoggingOptionsError::Credentials(ref err) => err.description(),
            SetV2LoggingOptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetV2LoggingOptionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartThingRegistrationTask
#[derive(Debug, PartialEq)]
pub enum StartThingRegistrationTaskError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StartThingRegistrationTaskError {
    pub fn from_body(body: &str) -> StartThingRegistrationTaskError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        StartThingRegistrationTaskError::InternalFailure(String::from(
                            error_message,
                        ))
                    }
                    "InvalidRequestException" => {
                        StartThingRegistrationTaskError::InvalidRequest(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        StartThingRegistrationTaskError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        StartThingRegistrationTaskError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartThingRegistrationTaskError::Validation(error_message.to_string())
                    }
                    _ => StartThingRegistrationTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartThingRegistrationTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartThingRegistrationTaskError {
    fn from(err: serde_json::error::Error) -> StartThingRegistrationTaskError {
        StartThingRegistrationTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartThingRegistrationTaskError {
    fn from(err: CredentialsError) -> StartThingRegistrationTaskError {
        StartThingRegistrationTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartThingRegistrationTaskError {
    fn from(err: HttpDispatchError) -> StartThingRegistrationTaskError {
        StartThingRegistrationTaskError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartThingRegistrationTaskError {
    fn from(err: io::Error) -> StartThingRegistrationTaskError {
        StartThingRegistrationTaskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartThingRegistrationTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartThingRegistrationTaskError {
    fn description(&self) -> &str {
        match *self {
            StartThingRegistrationTaskError::InternalFailure(ref cause) => cause,
            StartThingRegistrationTaskError::InvalidRequest(ref cause) => cause,
            StartThingRegistrationTaskError::Throttling(ref cause) => cause,
            StartThingRegistrationTaskError::Unauthorized(ref cause) => cause,
            StartThingRegistrationTaskError::Validation(ref cause) => cause,
            StartThingRegistrationTaskError::Credentials(ref err) => err.description(),
            StartThingRegistrationTaskError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartThingRegistrationTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopThingRegistrationTask
#[derive(Debug, PartialEq)]
pub enum StopThingRegistrationTaskError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StopThingRegistrationTaskError {
    pub fn from_body(body: &str) -> StopThingRegistrationTaskError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        StopThingRegistrationTaskError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        StopThingRegistrationTaskError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        StopThingRegistrationTaskError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ThrottlingException" => {
                        StopThingRegistrationTaskError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        StopThingRegistrationTaskError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        StopThingRegistrationTaskError::Validation(error_message.to_string())
                    }
                    _ => StopThingRegistrationTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopThingRegistrationTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopThingRegistrationTaskError {
    fn from(err: serde_json::error::Error) -> StopThingRegistrationTaskError {
        StopThingRegistrationTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopThingRegistrationTaskError {
    fn from(err: CredentialsError) -> StopThingRegistrationTaskError {
        StopThingRegistrationTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopThingRegistrationTaskError {
    fn from(err: HttpDispatchError) -> StopThingRegistrationTaskError {
        StopThingRegistrationTaskError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopThingRegistrationTaskError {
    fn from(err: io::Error) -> StopThingRegistrationTaskError {
        StopThingRegistrationTaskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopThingRegistrationTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopThingRegistrationTaskError {
    fn description(&self) -> &str {
        match *self {
            StopThingRegistrationTaskError::InternalFailure(ref cause) => cause,
            StopThingRegistrationTaskError::InvalidRequest(ref cause) => cause,
            StopThingRegistrationTaskError::ResourceNotFound(ref cause) => cause,
            StopThingRegistrationTaskError::Throttling(ref cause) => cause,
            StopThingRegistrationTaskError::Unauthorized(ref cause) => cause,
            StopThingRegistrationTaskError::Validation(ref cause) => cause,
            StopThingRegistrationTaskError::Credentials(ref err) => err.description(),
            StopThingRegistrationTaskError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopThingRegistrationTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TestAuthorization
#[derive(Debug, PartialEq)]
pub enum TestAuthorizationError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The number of attached entities exceeds the limit.</p>
    LimitExceeded(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl TestAuthorizationError {
    pub fn from_body(body: &str) -> TestAuthorizationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        TestAuthorizationError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        TestAuthorizationError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        TestAuthorizationError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        TestAuthorizationError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        TestAuthorizationError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        TestAuthorizationError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        TestAuthorizationError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        TestAuthorizationError::Validation(error_message.to_string())
                    }
                    _ => TestAuthorizationError::Unknown(String::from(body)),
                }
            }
            Err(_) => TestAuthorizationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TestAuthorizationError {
    fn from(err: serde_json::error::Error) -> TestAuthorizationError {
        TestAuthorizationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TestAuthorizationError {
    fn from(err: CredentialsError) -> TestAuthorizationError {
        TestAuthorizationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TestAuthorizationError {
    fn from(err: HttpDispatchError) -> TestAuthorizationError {
        TestAuthorizationError::HttpDispatch(err)
    }
}
impl From<io::Error> for TestAuthorizationError {
    fn from(err: io::Error) -> TestAuthorizationError {
        TestAuthorizationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TestAuthorizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TestAuthorizationError {
    fn description(&self) -> &str {
        match *self {
            TestAuthorizationError::InternalFailure(ref cause) => cause,
            TestAuthorizationError::InvalidRequest(ref cause) => cause,
            TestAuthorizationError::LimitExceeded(ref cause) => cause,
            TestAuthorizationError::ResourceNotFound(ref cause) => cause,
            TestAuthorizationError::ServiceUnavailable(ref cause) => cause,
            TestAuthorizationError::Throttling(ref cause) => cause,
            TestAuthorizationError::Unauthorized(ref cause) => cause,
            TestAuthorizationError::Validation(ref cause) => cause,
            TestAuthorizationError::Credentials(ref err) => err.description(),
            TestAuthorizationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            TestAuthorizationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TestInvokeAuthorizer
#[derive(Debug, PartialEq)]
pub enum TestInvokeAuthorizerError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The response is invalid.</p>
    InvalidResponse(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl TestInvokeAuthorizerError {
    pub fn from_body(body: &str) -> TestInvokeAuthorizerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        TestInvokeAuthorizerError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        TestInvokeAuthorizerError::InvalidRequest(String::from(error_message))
                    }
                    "InvalidResponseException" => {
                        TestInvokeAuthorizerError::InvalidResponse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        TestInvokeAuthorizerError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        TestInvokeAuthorizerError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        TestInvokeAuthorizerError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        TestInvokeAuthorizerError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        TestInvokeAuthorizerError::Validation(error_message.to_string())
                    }
                    _ => TestInvokeAuthorizerError::Unknown(String::from(body)),
                }
            }
            Err(_) => TestInvokeAuthorizerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TestInvokeAuthorizerError {
    fn from(err: serde_json::error::Error) -> TestInvokeAuthorizerError {
        TestInvokeAuthorizerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TestInvokeAuthorizerError {
    fn from(err: CredentialsError) -> TestInvokeAuthorizerError {
        TestInvokeAuthorizerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TestInvokeAuthorizerError {
    fn from(err: HttpDispatchError) -> TestInvokeAuthorizerError {
        TestInvokeAuthorizerError::HttpDispatch(err)
    }
}
impl From<io::Error> for TestInvokeAuthorizerError {
    fn from(err: io::Error) -> TestInvokeAuthorizerError {
        TestInvokeAuthorizerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TestInvokeAuthorizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TestInvokeAuthorizerError {
    fn description(&self) -> &str {
        match *self {
            TestInvokeAuthorizerError::InternalFailure(ref cause) => cause,
            TestInvokeAuthorizerError::InvalidRequest(ref cause) => cause,
            TestInvokeAuthorizerError::InvalidResponse(ref cause) => cause,
            TestInvokeAuthorizerError::ResourceNotFound(ref cause) => cause,
            TestInvokeAuthorizerError::ServiceUnavailable(ref cause) => cause,
            TestInvokeAuthorizerError::Throttling(ref cause) => cause,
            TestInvokeAuthorizerError::Unauthorized(ref cause) => cause,
            TestInvokeAuthorizerError::Validation(ref cause) => cause,
            TestInvokeAuthorizerError::Credentials(ref err) => err.description(),
            TestInvokeAuthorizerError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            TestInvokeAuthorizerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TransferCertificate
#[derive(Debug, PartialEq)]
pub enum TransferCertificateError {
    ///<p>The certificate operation is not allowed.</p>
    CertificateState(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You can't transfer the certificate because authorization policies are still attached.</p>
    TransferConflict(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl TransferCertificateError {
    pub fn from_body(body: &str) -> TransferCertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CertificateStateException" => {
                        TransferCertificateError::CertificateState(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        TransferCertificateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        TransferCertificateError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        TransferCertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        TransferCertificateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        TransferCertificateError::Throttling(String::from(error_message))
                    }
                    "TransferConflictException" => {
                        TransferCertificateError::TransferConflict(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        TransferCertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        TransferCertificateError::Validation(error_message.to_string())
                    }
                    _ => TransferCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => TransferCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TransferCertificateError {
    fn from(err: serde_json::error::Error) -> TransferCertificateError {
        TransferCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TransferCertificateError {
    fn from(err: CredentialsError) -> TransferCertificateError {
        TransferCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TransferCertificateError {
    fn from(err: HttpDispatchError) -> TransferCertificateError {
        TransferCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for TransferCertificateError {
    fn from(err: io::Error) -> TransferCertificateError {
        TransferCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TransferCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TransferCertificateError {
    fn description(&self) -> &str {
        match *self {
            TransferCertificateError::CertificateState(ref cause) => cause,
            TransferCertificateError::InternalFailure(ref cause) => cause,
            TransferCertificateError::InvalidRequest(ref cause) => cause,
            TransferCertificateError::ResourceNotFound(ref cause) => cause,
            TransferCertificateError::ServiceUnavailable(ref cause) => cause,
            TransferCertificateError::Throttling(ref cause) => cause,
            TransferCertificateError::TransferConflict(ref cause) => cause,
            TransferCertificateError::Unauthorized(ref cause) => cause,
            TransferCertificateError::Validation(ref cause) => cause,
            TransferCertificateError::Credentials(ref err) => err.description(),
            TransferCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            TransferCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAuthorizer
#[derive(Debug, PartialEq)]
pub enum UpdateAuthorizerError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The number of attached entities exceeds the limit.</p>
    LimitExceeded(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateAuthorizerError {
    pub fn from_body(body: &str) -> UpdateAuthorizerError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        UpdateAuthorizerError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        UpdateAuthorizerError::InvalidRequest(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateAuthorizerError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateAuthorizerError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        UpdateAuthorizerError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        UpdateAuthorizerError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateAuthorizerError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateAuthorizerError::Validation(error_message.to_string())
                    }
                    _ => UpdateAuthorizerError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateAuthorizerError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateAuthorizerError {
    fn from(err: serde_json::error::Error) -> UpdateAuthorizerError {
        UpdateAuthorizerError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateAuthorizerError {
    fn from(err: CredentialsError) -> UpdateAuthorizerError {
        UpdateAuthorizerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateAuthorizerError {
    fn from(err: HttpDispatchError) -> UpdateAuthorizerError {
        UpdateAuthorizerError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateAuthorizerError {
    fn from(err: io::Error) -> UpdateAuthorizerError {
        UpdateAuthorizerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateAuthorizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAuthorizerError {
    fn description(&self) -> &str {
        match *self {
            UpdateAuthorizerError::InternalFailure(ref cause) => cause,
            UpdateAuthorizerError::InvalidRequest(ref cause) => cause,
            UpdateAuthorizerError::LimitExceeded(ref cause) => cause,
            UpdateAuthorizerError::ResourceNotFound(ref cause) => cause,
            UpdateAuthorizerError::ServiceUnavailable(ref cause) => cause,
            UpdateAuthorizerError::Throttling(ref cause) => cause,
            UpdateAuthorizerError::Unauthorized(ref cause) => cause,
            UpdateAuthorizerError::Validation(ref cause) => cause,
            UpdateAuthorizerError::Credentials(ref err) => err.description(),
            UpdateAuthorizerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateAuthorizerError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCACertificate
#[derive(Debug, PartialEq)]
pub enum UpdateCACertificateError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateCACertificateError {
    pub fn from_body(body: &str) -> UpdateCACertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        UpdateCACertificateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        UpdateCACertificateError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateCACertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        UpdateCACertificateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        UpdateCACertificateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateCACertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateCACertificateError::Validation(error_message.to_string())
                    }
                    _ => UpdateCACertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateCACertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateCACertificateError {
    fn from(err: serde_json::error::Error) -> UpdateCACertificateError {
        UpdateCACertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateCACertificateError {
    fn from(err: CredentialsError) -> UpdateCACertificateError {
        UpdateCACertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateCACertificateError {
    fn from(err: HttpDispatchError) -> UpdateCACertificateError {
        UpdateCACertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateCACertificateError {
    fn from(err: io::Error) -> UpdateCACertificateError {
        UpdateCACertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateCACertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCACertificateError {
    fn description(&self) -> &str {
        match *self {
            UpdateCACertificateError::InternalFailure(ref cause) => cause,
            UpdateCACertificateError::InvalidRequest(ref cause) => cause,
            UpdateCACertificateError::ResourceNotFound(ref cause) => cause,
            UpdateCACertificateError::ServiceUnavailable(ref cause) => cause,
            UpdateCACertificateError::Throttling(ref cause) => cause,
            UpdateCACertificateError::Unauthorized(ref cause) => cause,
            UpdateCACertificateError::Validation(ref cause) => cause,
            UpdateCACertificateError::Credentials(ref err) => err.description(),
            UpdateCACertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateCACertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateCertificate
#[derive(Debug, PartialEq)]
pub enum UpdateCertificateError {
    ///<p>The certificate operation is not allowed.</p>
    CertificateState(String),
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateCertificateError {
    pub fn from_body(body: &str) -> UpdateCertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "CertificateStateException" => {
                        UpdateCertificateError::CertificateState(String::from(error_message))
                    }
                    "InternalFailureException" => {
                        UpdateCertificateError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        UpdateCertificateError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateCertificateError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        UpdateCertificateError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        UpdateCertificateError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateCertificateError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateCertificateError::Validation(error_message.to_string())
                    }
                    _ => UpdateCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateCertificateError {
    fn from(err: serde_json::error::Error) -> UpdateCertificateError {
        UpdateCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateCertificateError {
    fn from(err: CredentialsError) -> UpdateCertificateError {
        UpdateCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateCertificateError {
    fn from(err: HttpDispatchError) -> UpdateCertificateError {
        UpdateCertificateError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateCertificateError {
    fn from(err: io::Error) -> UpdateCertificateError {
        UpdateCertificateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateCertificateError {
    fn description(&self) -> &str {
        match *self {
            UpdateCertificateError::CertificateState(ref cause) => cause,
            UpdateCertificateError::InternalFailure(ref cause) => cause,
            UpdateCertificateError::InvalidRequest(ref cause) => cause,
            UpdateCertificateError::ResourceNotFound(ref cause) => cause,
            UpdateCertificateError::ServiceUnavailable(ref cause) => cause,
            UpdateCertificateError::Throttling(ref cause) => cause,
            UpdateCertificateError::Unauthorized(ref cause) => cause,
            UpdateCertificateError::Validation(ref cause) => cause,
            UpdateCertificateError::Credentials(ref err) => err.description(),
            UpdateCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateEventConfigurations
#[derive(Debug, PartialEq)]
pub enum UpdateEventConfigurationsError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateEventConfigurationsError {
    pub fn from_body(body: &str) -> UpdateEventConfigurationsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        UpdateEventConfigurationsError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        UpdateEventConfigurationsError::InvalidRequest(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        UpdateEventConfigurationsError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateEventConfigurationsError::Validation(error_message.to_string())
                    }
                    _ => UpdateEventConfigurationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateEventConfigurationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateEventConfigurationsError {
    fn from(err: serde_json::error::Error) -> UpdateEventConfigurationsError {
        UpdateEventConfigurationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateEventConfigurationsError {
    fn from(err: CredentialsError) -> UpdateEventConfigurationsError {
        UpdateEventConfigurationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateEventConfigurationsError {
    fn from(err: HttpDispatchError) -> UpdateEventConfigurationsError {
        UpdateEventConfigurationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateEventConfigurationsError {
    fn from(err: io::Error) -> UpdateEventConfigurationsError {
        UpdateEventConfigurationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateEventConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateEventConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            UpdateEventConfigurationsError::InternalFailure(ref cause) => cause,
            UpdateEventConfigurationsError::InvalidRequest(ref cause) => cause,
            UpdateEventConfigurationsError::Throttling(ref cause) => cause,
            UpdateEventConfigurationsError::Validation(ref cause) => cause,
            UpdateEventConfigurationsError::Credentials(ref err) => err.description(),
            UpdateEventConfigurationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateEventConfigurationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateIndexingConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateIndexingConfigurationError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateIndexingConfigurationError {
    pub fn from_body(body: &str) -> UpdateIndexingConfigurationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        UpdateIndexingConfigurationError::InternalFailure(String::from(
                            error_message,
                        ))
                    }
                    "InvalidRequestException" => UpdateIndexingConfigurationError::InvalidRequest(
                        String::from(error_message),
                    ),
                    "ServiceUnavailableException" => {
                        UpdateIndexingConfigurationError::ServiceUnavailable(String::from(
                            error_message,
                        ))
                    }
                    "ThrottlingException" => {
                        UpdateIndexingConfigurationError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateIndexingConfigurationError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateIndexingConfigurationError::Validation(error_message.to_string())
                    }
                    _ => UpdateIndexingConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateIndexingConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateIndexingConfigurationError {
    fn from(err: serde_json::error::Error) -> UpdateIndexingConfigurationError {
        UpdateIndexingConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateIndexingConfigurationError {
    fn from(err: CredentialsError) -> UpdateIndexingConfigurationError {
        UpdateIndexingConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateIndexingConfigurationError {
    fn from(err: HttpDispatchError) -> UpdateIndexingConfigurationError {
        UpdateIndexingConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateIndexingConfigurationError {
    fn from(err: io::Error) -> UpdateIndexingConfigurationError {
        UpdateIndexingConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateIndexingConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateIndexingConfigurationError {
    fn description(&self) -> &str {
        match *self {
            UpdateIndexingConfigurationError::InternalFailure(ref cause) => cause,
            UpdateIndexingConfigurationError::InvalidRequest(ref cause) => cause,
            UpdateIndexingConfigurationError::ServiceUnavailable(ref cause) => cause,
            UpdateIndexingConfigurationError::Throttling(ref cause) => cause,
            UpdateIndexingConfigurationError::Unauthorized(ref cause) => cause,
            UpdateIndexingConfigurationError::Validation(ref cause) => cause,
            UpdateIndexingConfigurationError::Credentials(ref err) => err.description(),
            UpdateIndexingConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateIndexingConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRoleAlias
#[derive(Debug, PartialEq)]
pub enum UpdateRoleAliasError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateRoleAliasError {
    pub fn from_body(body: &str) -> UpdateRoleAliasError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        UpdateRoleAliasError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        UpdateRoleAliasError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateRoleAliasError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        UpdateRoleAliasError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        UpdateRoleAliasError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateRoleAliasError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateRoleAliasError::Validation(error_message.to_string())
                    }
                    _ => UpdateRoleAliasError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateRoleAliasError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateRoleAliasError {
    fn from(err: serde_json::error::Error) -> UpdateRoleAliasError {
        UpdateRoleAliasError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateRoleAliasError {
    fn from(err: CredentialsError) -> UpdateRoleAliasError {
        UpdateRoleAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateRoleAliasError {
    fn from(err: HttpDispatchError) -> UpdateRoleAliasError {
        UpdateRoleAliasError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateRoleAliasError {
    fn from(err: io::Error) -> UpdateRoleAliasError {
        UpdateRoleAliasError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateRoleAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRoleAliasError {
    fn description(&self) -> &str {
        match *self {
            UpdateRoleAliasError::InternalFailure(ref cause) => cause,
            UpdateRoleAliasError::InvalidRequest(ref cause) => cause,
            UpdateRoleAliasError::ResourceNotFound(ref cause) => cause,
            UpdateRoleAliasError::ServiceUnavailable(ref cause) => cause,
            UpdateRoleAliasError::Throttling(ref cause) => cause,
            UpdateRoleAliasError::Unauthorized(ref cause) => cause,
            UpdateRoleAliasError::Validation(ref cause) => cause,
            UpdateRoleAliasError::Credentials(ref err) => err.description(),
            UpdateRoleAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateRoleAliasError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateStream
#[derive(Debug, PartialEq)]
pub enum UpdateStreamError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateStreamError {
    pub fn from_body(body: &str) -> UpdateStreamError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        UpdateStreamError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        UpdateStreamError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateStreamError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        UpdateStreamError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        UpdateStreamError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateStreamError::Unauthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateStreamError::Validation(error_message.to_string())
                    }
                    _ => UpdateStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateStreamError {
    fn from(err: serde_json::error::Error) -> UpdateStreamError {
        UpdateStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateStreamError {
    fn from(err: CredentialsError) -> UpdateStreamError {
        UpdateStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateStreamError {
    fn from(err: HttpDispatchError) -> UpdateStreamError {
        UpdateStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateStreamError {
    fn from(err: io::Error) -> UpdateStreamError {
        UpdateStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateStreamError {
    fn description(&self) -> &str {
        match *self {
            UpdateStreamError::InternalFailure(ref cause) => cause,
            UpdateStreamError::InvalidRequest(ref cause) => cause,
            UpdateStreamError::ResourceNotFound(ref cause) => cause,
            UpdateStreamError::ServiceUnavailable(ref cause) => cause,
            UpdateStreamError::Throttling(ref cause) => cause,
            UpdateStreamError::Unauthorized(ref cause) => cause,
            UpdateStreamError::Validation(ref cause) => cause,
            UpdateStreamError::Credentials(ref err) => err.description(),
            UpdateStreamError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateStreamError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateThing
#[derive(Debug, PartialEq)]
pub enum UpdateThingError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>You are not authorized to perform this operation.</p>
    Unauthorized(String),
    ///<p>An exception thrown when the version of a thing passed to a command is different than the version specified with the --version parameter.</p>
    VersionConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateThingError {
    pub fn from_body(body: &str) -> UpdateThingError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        UpdateThingError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        UpdateThingError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateThingError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        UpdateThingError::ServiceUnavailable(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        UpdateThingError::Throttling(String::from(error_message))
                    }
                    "UnauthorizedException" => {
                        UpdateThingError::Unauthorized(String::from(error_message))
                    }
                    "VersionConflictException" => {
                        UpdateThingError::VersionConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateThingError::Validation(error_message.to_string())
                    }
                    _ => UpdateThingError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateThingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateThingError {
    fn from(err: serde_json::error::Error) -> UpdateThingError {
        UpdateThingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateThingError {
    fn from(err: CredentialsError) -> UpdateThingError {
        UpdateThingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateThingError {
    fn from(err: HttpDispatchError) -> UpdateThingError {
        UpdateThingError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateThingError {
    fn from(err: io::Error) -> UpdateThingError {
        UpdateThingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateThingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateThingError {
    fn description(&self) -> &str {
        match *self {
            UpdateThingError::InternalFailure(ref cause) => cause,
            UpdateThingError::InvalidRequest(ref cause) => cause,
            UpdateThingError::ResourceNotFound(ref cause) => cause,
            UpdateThingError::ServiceUnavailable(ref cause) => cause,
            UpdateThingError::Throttling(ref cause) => cause,
            UpdateThingError::Unauthorized(ref cause) => cause,
            UpdateThingError::VersionConflict(ref cause) => cause,
            UpdateThingError::Validation(ref cause) => cause,
            UpdateThingError::Credentials(ref err) => err.description(),
            UpdateThingError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateThingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateThingGroup
#[derive(Debug, PartialEq)]
pub enum UpdateThingGroupError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    ///<p>An exception thrown when the version of a thing passed to a command is different than the version specified with the --version parameter.</p>
    VersionConflict(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateThingGroupError {
    pub fn from_body(body: &str) -> UpdateThingGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        UpdateThingGroupError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        UpdateThingGroupError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateThingGroupError::ResourceNotFound(String::from(error_message))
                    }
                    "ThrottlingException" => {
                        UpdateThingGroupError::Throttling(String::from(error_message))
                    }
                    "VersionConflictException" => {
                        UpdateThingGroupError::VersionConflict(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateThingGroupError::Validation(error_message.to_string())
                    }
                    _ => UpdateThingGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateThingGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateThingGroupError {
    fn from(err: serde_json::error::Error) -> UpdateThingGroupError {
        UpdateThingGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateThingGroupError {
    fn from(err: CredentialsError) -> UpdateThingGroupError {
        UpdateThingGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateThingGroupError {
    fn from(err: HttpDispatchError) -> UpdateThingGroupError {
        UpdateThingGroupError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateThingGroupError {
    fn from(err: io::Error) -> UpdateThingGroupError {
        UpdateThingGroupError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateThingGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateThingGroupError {
    fn description(&self) -> &str {
        match *self {
            UpdateThingGroupError::InternalFailure(ref cause) => cause,
            UpdateThingGroupError::InvalidRequest(ref cause) => cause,
            UpdateThingGroupError::ResourceNotFound(ref cause) => cause,
            UpdateThingGroupError::Throttling(ref cause) => cause,
            UpdateThingGroupError::VersionConflict(ref cause) => cause,
            UpdateThingGroupError::Validation(ref cause) => cause,
            UpdateThingGroupError::Credentials(ref err) => err.description(),
            UpdateThingGroupError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateThingGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateThingGroupsForThing
#[derive(Debug, PartialEq)]
pub enum UpdateThingGroupsForThingError {
    ///<p>An unexpected error has occurred.</p>
    InternalFailure(String),
    ///<p>The request is not valid.</p>
    InvalidRequest(String),
    ///<p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    ///<p>The rate exceeds the limit.</p>
    Throttling(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateThingGroupsForThingError {
    pub fn from_body(body: &str) -> UpdateThingGroupsForThingError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalFailureException" => {
                        UpdateThingGroupsForThingError::InternalFailure(String::from(error_message))
                    }
                    "InvalidRequestException" => {
                        UpdateThingGroupsForThingError::InvalidRequest(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateThingGroupsForThingError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ThrottlingException" => {
                        UpdateThingGroupsForThingError::Throttling(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateThingGroupsForThingError::Validation(error_message.to_string())
                    }
                    _ => UpdateThingGroupsForThingError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateThingGroupsForThingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateThingGroupsForThingError {
    fn from(err: serde_json::error::Error) -> UpdateThingGroupsForThingError {
        UpdateThingGroupsForThingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateThingGroupsForThingError {
    fn from(err: CredentialsError) -> UpdateThingGroupsForThingError {
        UpdateThingGroupsForThingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateThingGroupsForThingError {
    fn from(err: HttpDispatchError) -> UpdateThingGroupsForThingError {
        UpdateThingGroupsForThingError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateThingGroupsForThingError {
    fn from(err: io::Error) -> UpdateThingGroupsForThingError {
        UpdateThingGroupsForThingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateThingGroupsForThingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateThingGroupsForThingError {
    fn description(&self) -> &str {
        match *self {
            UpdateThingGroupsForThingError::InternalFailure(ref cause) => cause,
            UpdateThingGroupsForThingError::InvalidRequest(ref cause) => cause,
            UpdateThingGroupsForThingError::ResourceNotFound(ref cause) => cause,
            UpdateThingGroupsForThingError::Throttling(ref cause) => cause,
            UpdateThingGroupsForThingError::Validation(ref cause) => cause,
            UpdateThingGroupsForThingError::Credentials(ref err) => err.description(),
            UpdateThingGroupsForThingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateThingGroupsForThingError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS IoT API. AWS IoT clients implement this trait.
pub trait Iot {
    #[doc="<p>Accepts a pending certificate transfer. The default state of the certificate is INACTIVE.</p> <p>To check for pending certificate transfers, call <a>ListCertificates</a> to enumerate your certificates.</p>"]
    fn accept_certificate_transfer(
        &self,
        input: &AcceptCertificateTransferRequest,
    ) -> Result<(), AcceptCertificateTransferError>;

    #[doc = "<p>Adds a thing to a thing group.</p>"]
    fn add_thing_to_thing_group(
        &self,
        input: &AddThingToThingGroupRequest,
    ) -> Result<AddThingToThingGroupResponse, AddThingToThingGroupError>;

    #[doc="<p>Associates a group with a continuous job. The following criteria must be met: </p> <ul> <li> <p>The job must have been created with the <code>targetSelection</code> field set to \"CONTINUOUS\".</p> </li> <li> <p>The job status must currently be \"IN_PROGRESS\".</p> </li> <li> <p>The total number of targets associated with a job must not exceed 100.</p> </li> </ul>"]
    fn associate_targets_with_job(
        &self,
        input: &AssociateTargetsWithJobRequest,
    ) -> Result<AssociateTargetsWithJobResponse, AssociateTargetsWithJobError>;

    #[doc = "<p>Attaches a policy to the specified target.</p>"]
    fn attach_policy(&self, input: &AttachPolicyRequest) -> Result<(), AttachPolicyError>;

    #[doc="<p>Attaches the specified policy to the specified principal (certificate or other credential).</p> <p> <b>Note:</b> This API is deprecated. Please use <a>AttachPolicy</a> instead.</p>"]
    fn attach_principal_policy(
        &self,
        input: &AttachPrincipalPolicyRequest,
    ) -> Result<(), AttachPrincipalPolicyError>;

    #[doc = "<p>Attaches the specified principal to the specified thing.</p>"]
    fn attach_thing_principal(
        &self,
        input: &AttachThingPrincipalRequest,
    ) -> Result<AttachThingPrincipalResponse, AttachThingPrincipalError>;

    #[doc="<p>Cancels a pending transfer for the specified certificate.</p> <p> <b>Note</b> Only the transfer source account can use this operation to cancel a transfer. (Transfer destinations can use <a>RejectCertificateTransfer</a> instead.) After transfer, AWS IoT returns the certificate to the source account in the INACTIVE state. After the destination account has accepted the transfer, the transfer cannot be cancelled.</p> <p>After a certificate transfer is cancelled, the status of the certificate changes from PENDING_TRANSFER to INACTIVE.</p>"]
    fn cancel_certificate_transfer(
        &self,
        input: &CancelCertificateTransferRequest,
    ) -> Result<(), CancelCertificateTransferError>;

    #[doc = "<p>Cancels a job.</p>"]
    fn cancel_job(&self, input: &CancelJobRequest) -> Result<CancelJobResponse, CancelJobError>;

    #[doc = "<p>Clears the default authorizer.</p>"]
    fn clear_default_authorizer(
        &self,
    ) -> Result<ClearDefaultAuthorizerResponse, ClearDefaultAuthorizerError>;

    #[doc = "<p>Creates an authorizer.</p>"]
    fn create_authorizer(
        &self,
        input: &CreateAuthorizerRequest,
    ) -> Result<CreateAuthorizerResponse, CreateAuthorizerError>;

    #[doc="<p>Creates an X.509 certificate using the specified certificate signing request.</p> <p> <b>Note:</b> The CSR must include a public key that is either an RSA key with a length of at least 2048 bits or an ECC key from NIST P-256 or NIST P-384 curves. </p> <p> <b>Note:</b> Reusing the same certificate signing request (CSR) results in a distinct certificate.</p> <p>You can create multiple certificates in a batch by creating a directory, copying multiple .csr files into that directory, and then specifying that directory on the command line. The following commands show how to create a batch of certificates given a batch of CSRs.</p> <p>Assuming a set of CSRs are located inside of the directory my-csr-directory:</p> <p>On Linux and OS X, the command is:</p> <p>$ ls my-csr-directory/ | xargs -I {} aws iot create-certificate-from-csr --certificate-signing-request file://my-csr-directory/{}</p> <p>This command lists all of the CSRs in my-csr-directory and pipes each CSR file name to the aws iot create-certificate-from-csr AWS CLI command to create a certificate for the corresponding CSR.</p> <p>The aws iot create-certificate-from-csr part of the command can also be run in parallel to speed up the certificate creation process:</p> <p>$ ls my-csr-directory/ | xargs -P 10 -I {} aws iot create-certificate-from-csr --certificate-signing-request file://my-csr-directory/{}</p> <p>On Windows PowerShell, the command to create certificates for all CSRs in my-csr-directory is:</p> <p>&gt; ls -Name my-csr-directory | %{aws iot create-certificate-from-csr --certificate-signing-request file://my-csr-directory/$_}</p> <p>On a Windows command prompt, the command to create certificates for all CSRs in my-csr-directory is:</p> <p>&gt; forfiles /p my-csr-directory /c \"cmd /c aws iot create-certificate-from-csr --certificate-signing-request file://@path\"</p>"]
    fn create_certificate_from_csr(
        &self,
        input: &CreateCertificateFromCsrRequest,
    ) -> Result<CreateCertificateFromCsrResponse, CreateCertificateFromCsrError>;

    #[doc = "<p>Creates a job.</p>"]
    fn create_job(&self, input: &CreateJobRequest) -> Result<CreateJobResponse, CreateJobError>;

    #[doc="<p>Creates a 2048-bit RSA key pair and issues an X.509 certificate using the issued public key.</p> <p> <b>Note</b> This is the only time AWS IoT issues the private key for this certificate, so it is important to keep it in a secure location.</p>"]
    fn create_keys_and_certificate(
        &self,
        input: &CreateKeysAndCertificateRequest,
    ) -> Result<CreateKeysAndCertificateResponse, CreateKeysAndCertificateError>;

    #[doc = "<p>Creates an AWS IoT OTAUpdate on a target group of things or groups.</p>"]
    fn create_ota_update(
        &self,
        input: &CreateOTAUpdateRequest,
    ) -> Result<CreateOTAUpdateResponse, CreateOTAUpdateError>;

    #[doc="<p>Creates an AWS IoT policy.</p> <p>The created policy is the default version for the policy. This operation creates a policy version with a version identifier of <b>1</b> and sets <b>1</b> as the policy's default version.</p>"]
    fn create_policy(
        &self,
        input: &CreatePolicyRequest,
    ) -> Result<CreatePolicyResponse, CreatePolicyError>;

    #[doc="<p>Creates a new version of the specified AWS IoT policy. To update a policy, create a new policy version. A managed policy can have up to five versions. If the policy has five versions, you must use <a>DeletePolicyVersion</a> to delete an existing version before you create a new one.</p> <p>Optionally, you can set the new version as the policy's default version. The default version is the operative version (that is, the version that is in effect for the certificates to which the policy is attached).</p>"]
    fn create_policy_version(
        &self,
        input: &CreatePolicyVersionRequest,
    ) -> Result<CreatePolicyVersionResponse, CreatePolicyVersionError>;

    #[doc = "<p>Creates a role alias.</p>"]
    fn create_role_alias(
        &self,
        input: &CreateRoleAliasRequest,
    ) -> Result<CreateRoleAliasResponse, CreateRoleAliasError>;

    #[doc="<p>Creates a stream for delivering one or more large files in chunks over MQTT. A stream transports data bytes in chunks or blocks packaged as MQTT messages from a source like S3. You can have one or more files associated with a stream. The total size of a file associated with the stream cannot exceed more than 2 MB. The stream will be created with version 0. If a stream is created with the same streamID as a stream that existed and was deleted within last 90 days, we will resurrect that old stream by incrementing the version by 1.</p>"]
    fn create_stream(
        &self,
        input: &CreateStreamRequest,
    ) -> Result<CreateStreamResponse, CreateStreamError>;

    #[doc = "<p>Creates a thing record in the thing registry.</p>"]
    fn create_thing(
        &self,
        input: &CreateThingRequest,
    ) -> Result<CreateThingResponse, CreateThingError>;

    #[doc = "<p>Create a thing group.</p>"]
    fn create_thing_group(
        &self,
        input: &CreateThingGroupRequest,
    ) -> Result<CreateThingGroupResponse, CreateThingGroupError>;

    #[doc = "<p>Creates a new thing type.</p>"]
    fn create_thing_type(
        &self,
        input: &CreateThingTypeRequest,
    ) -> Result<CreateThingTypeResponse, CreateThingTypeError>;

    #[doc="<p>Creates a rule. Creating rules is an administrator-level action. Any user who has permission to create rules will be able to access data processed by the rule.</p>"]
    fn create_topic_rule(&self, input: &CreateTopicRuleRequest)
        -> Result<(), CreateTopicRuleError>;

    #[doc = "<p>Deletes an authorizer.</p>"]
    fn delete_authorizer(
        &self,
        input: &DeleteAuthorizerRequest,
    ) -> Result<DeleteAuthorizerResponse, DeleteAuthorizerError>;

    #[doc = "<p>Deletes a registered CA certificate.</p>"]
    fn delete_ca_certificate(
        &self,
        input: &DeleteCACertificateRequest,
    ) -> Result<DeleteCACertificateResponse, DeleteCACertificateError>;

    #[doc="<p>Deletes the specified certificate.</p> <p>A certificate cannot be deleted if it has a policy attached to it or if its status is set to ACTIVE. To delete a certificate, first use the <a>DetachPrincipalPolicy</a> API to detach all policies. Next, use the <a>UpdateCertificate</a> API to set the certificate to the INACTIVE status.</p>"]
    fn delete_certificate(
        &self,
        input: &DeleteCertificateRequest,
    ) -> Result<(), DeleteCertificateError>;

    #[doc = "<p>Delete an OTA update.</p>"]
    fn delete_ota_update(
        &self,
        input: &DeleteOTAUpdateRequest,
    ) -> Result<DeleteOTAUpdateResponse, DeleteOTAUpdateError>;

    #[doc="<p>Deletes the specified policy.</p> <p>A policy cannot be deleted if it has non-default versions or it is attached to any certificate.</p> <p>To delete a policy, use the DeletePolicyVersion API to delete all non-default versions of the policy; use the DetachPrincipalPolicy API to detach the policy from any certificate; and then use the DeletePolicy API to delete the policy.</p> <p>When a policy is deleted using DeletePolicy, its default version is deleted with it.</p>"]
    fn delete_policy(&self, input: &DeletePolicyRequest) -> Result<(), DeletePolicyError>;

    #[doc="<p>Deletes the specified version of the specified policy. You cannot delete the default version of a policy using this API. To delete the default version of a policy, use <a>DeletePolicy</a>. To find out which version of a policy is marked as the default version, use ListPolicyVersions.</p>"]
    fn delete_policy_version(
        &self,
        input: &DeletePolicyVersionRequest,
    ) -> Result<(), DeletePolicyVersionError>;

    #[doc = "<p>Deletes a CA certificate registration code.</p>"]
    fn delete_registration_code(
        &self,
    ) -> Result<DeleteRegistrationCodeResponse, DeleteRegistrationCodeError>;

    #[doc = "<p>Deletes a role alias</p>"]
    fn delete_role_alias(
        &self,
        input: &DeleteRoleAliasRequest,
    ) -> Result<DeleteRoleAliasResponse, DeleteRoleAliasError>;

    #[doc = "<p>Deletes a stream.</p>"]
    fn delete_stream(
        &self,
        input: &DeleteStreamRequest,
    ) -> Result<DeleteStreamResponse, DeleteStreamError>;

    #[doc = "<p>Deletes the specified thing.</p>"]
    fn delete_thing(
        &self,
        input: &DeleteThingRequest,
    ) -> Result<DeleteThingResponse, DeleteThingError>;

    #[doc = "<p>Deletes a thing group.</p>"]
    fn delete_thing_group(
        &self,
        input: &DeleteThingGroupRequest,
    ) -> Result<DeleteThingGroupResponse, DeleteThingGroupError>;

    #[doc="<p>Deletes the specified thing type . You cannot delete a thing type if it has things associated with it. To delete a thing type, first mark it as deprecated by calling <a>DeprecateThingType</a>, then remove any associated things by calling <a>UpdateThing</a> to change the thing type on any associated thing, and finally use <a>DeleteThingType</a> to delete the thing type.</p>"]
    fn delete_thing_type(
        &self,
        input: &DeleteThingTypeRequest,
    ) -> Result<DeleteThingTypeResponse, DeleteThingTypeError>;

    #[doc = "<p>Deletes the rule.</p>"]
    fn delete_topic_rule(&self, input: &DeleteTopicRuleRequest)
        -> Result<(), DeleteTopicRuleError>;

    #[doc = "<p>Deletes a logging level.</p>"]
    fn delete_v2_logging_level(
        &self,
        input: &DeleteV2LoggingLevelRequest,
    ) -> Result<(), DeleteV2LoggingLevelError>;

    #[doc="<p>Deprecates a thing type. You can not associate new things with deprecated thing type.</p>"]
    fn deprecate_thing_type(
        &self,
        input: &DeprecateThingTypeRequest,
    ) -> Result<DeprecateThingTypeResponse, DeprecateThingTypeError>;

    #[doc = "<p>Describes an authorizer.</p>"]
    fn describe_authorizer(
        &self,
        input: &DescribeAuthorizerRequest,
    ) -> Result<DescribeAuthorizerResponse, DescribeAuthorizerError>;

    #[doc = "<p>Describes a registered CA certificate.</p>"]
    fn describe_ca_certificate(
        &self,
        input: &DescribeCACertificateRequest,
    ) -> Result<DescribeCACertificateResponse, DescribeCACertificateError>;

    #[doc = "<p>Gets information about the specified certificate.</p>"]
    fn describe_certificate(
        &self,
        input: &DescribeCertificateRequest,
    ) -> Result<DescribeCertificateResponse, DescribeCertificateError>;

    #[doc = "<p>Describes the default authorizer.</p>"]
    fn describe_default_authorizer(
        &self,
    ) -> Result<DescribeDefaultAuthorizerResponse, DescribeDefaultAuthorizerError>;

    #[doc = "<p>Returns a unique endpoint specific to the AWS account making the call.</p>"]
    fn describe_endpoint(
        &self,
        input: &DescribeEndpointRequest,
    ) -> Result<DescribeEndpointResponse, DescribeEndpointError>;

    #[doc = "<p>Describes event configurations.</p>"]
    fn describe_event_configurations(
        &self,
    ) -> Result<DescribeEventConfigurationsResponse, DescribeEventConfigurationsError>;

    #[doc = "<p>Describes a search index.</p>"]
    fn describe_index(
        &self,
        input: &DescribeIndexRequest,
    ) -> Result<DescribeIndexResponse, DescribeIndexError>;

    #[doc = "<p>Describes a job.</p>"]
    fn describe_job(
        &self,
        input: &DescribeJobRequest,
    ) -> Result<DescribeJobResponse, DescribeJobError>;

    #[doc = "<p>Describes a job execution.</p>"]
    fn describe_job_execution(
        &self,
        input: &DescribeJobExecutionRequest,
    ) -> Result<DescribeJobExecutionResponse, DescribeJobExecutionError>;

    #[doc = "<p>Describes a role alias.</p>"]
    fn describe_role_alias(
        &self,
        input: &DescribeRoleAliasRequest,
    ) -> Result<DescribeRoleAliasResponse, DescribeRoleAliasError>;

    #[doc = "<p>Gets information about a stream.</p>"]
    fn describe_stream(
        &self,
        input: &DescribeStreamRequest,
    ) -> Result<DescribeStreamResponse, DescribeStreamError>;

    #[doc = "<p>Gets information about the specified thing.</p>"]
    fn describe_thing(
        &self,
        input: &DescribeThingRequest,
    ) -> Result<DescribeThingResponse, DescribeThingError>;

    #[doc = "<p>Describe a thing group.</p>"]
    fn describe_thing_group(
        &self,
        input: &DescribeThingGroupRequest,
    ) -> Result<DescribeThingGroupResponse, DescribeThingGroupError>;

    #[doc = "<p>Describes a bulk thing provisioning task.</p>"]
    fn describe_thing_registration_task(
        &self,
        input: &DescribeThingRegistrationTaskRequest,
    ) -> Result<DescribeThingRegistrationTaskResponse, DescribeThingRegistrationTaskError>;

    #[doc = "<p>Gets information about the specified thing type.</p>"]
    fn describe_thing_type(
        &self,
        input: &DescribeThingTypeRequest,
    ) -> Result<DescribeThingTypeResponse, DescribeThingTypeError>;

    #[doc = "<p>Detaches a policy from the specified target.</p>"]
    fn detach_policy(&self, input: &DetachPolicyRequest) -> Result<(), DetachPolicyError>;

    #[doc="<p>Removes the specified policy from the specified certificate.</p> <p> <b>Note:</b> This API is deprecated. Please use <a>DetachPolicy</a> instead.</p>"]
    fn detach_principal_policy(
        &self,
        input: &DetachPrincipalPolicyRequest,
    ) -> Result<(), DetachPrincipalPolicyError>;

    #[doc = "<p>Detaches the specified principal from the specified thing.</p>"]
    fn detach_thing_principal(
        &self,
        input: &DetachThingPrincipalRequest,
    ) -> Result<DetachThingPrincipalResponse, DetachThingPrincipalError>;

    #[doc = "<p>Disables the rule.</p>"]
    fn disable_topic_rule(
        &self,
        input: &DisableTopicRuleRequest,
    ) -> Result<(), DisableTopicRuleError>;

    #[doc = "<p>Enables the rule.</p>"]
    fn enable_topic_rule(&self, input: &EnableTopicRuleRequest)
        -> Result<(), EnableTopicRuleError>;

    #[doc = "<p>Gets effective policies.</p>"]
    fn get_effective_policies(
        &self,
        input: &GetEffectivePoliciesRequest,
    ) -> Result<GetEffectivePoliciesResponse, GetEffectivePoliciesError>;

    #[doc = "<p>Gets the search configuration.</p>"]
    fn get_indexing_configuration(
        &self,
    ) -> Result<GetIndexingConfigurationResponse, GetIndexingConfigurationError>;

    #[doc = "<p>Gets a job document.</p>"]
    fn get_job_document(
        &self,
        input: &GetJobDocumentRequest,
    ) -> Result<GetJobDocumentResponse, GetJobDocumentError>;

    #[doc = "<p>Gets the logging options.</p>"]
    fn get_logging_options(&self) -> Result<GetLoggingOptionsResponse, GetLoggingOptionsError>;

    #[doc = "<p>Gets an OTA update.</p>"]
    fn get_ota_update(
        &self,
        input: &GetOTAUpdateRequest,
    ) -> Result<GetOTAUpdateResponse, GetOTAUpdateError>;

    #[doc="<p>Gets information about the specified policy with the policy document of the default version.</p>"]
    fn get_policy(&self, input: &GetPolicyRequest) -> Result<GetPolicyResponse, GetPolicyError>;

    #[doc = "<p>Gets information about the specified policy version.</p>"]
    fn get_policy_version(
        &self,
        input: &GetPolicyVersionRequest,
    ) -> Result<GetPolicyVersionResponse, GetPolicyVersionError>;

    #[doc = "<p>Gets a registration code used to register a CA certificate with AWS IoT.</p>"]
    fn get_registration_code(
        &self,
    ) -> Result<GetRegistrationCodeResponse, GetRegistrationCodeError>;

    #[doc = "<p>Gets information about the rule.</p>"]
    fn get_topic_rule(
        &self,
        input: &GetTopicRuleRequest,
    ) -> Result<GetTopicRuleResponse, GetTopicRuleError>;

    #[doc = "<p>Gets the fine grained logging options.</p>"]
    fn get_v2_logging_options(
        &self,
    ) -> Result<GetV2LoggingOptionsResponse, GetV2LoggingOptionsError>;

    #[doc = "<p>Lists the policies attached to the specified thing group.</p>"]
    fn list_attached_policies(
        &self,
        input: &ListAttachedPoliciesRequest,
    ) -> Result<ListAttachedPoliciesResponse, ListAttachedPoliciesError>;

    #[doc = "<p>Lists the authorizers registered in your account.</p>"]
    fn list_authorizers(
        &self,
        input: &ListAuthorizersRequest,
    ) -> Result<ListAuthorizersResponse, ListAuthorizersError>;

    #[doc="<p>Lists the CA certificates registered for your AWS account.</p> <p>The results are paginated with a default page size of 25. You can use the returned marker to retrieve additional results.</p>"]
    fn list_ca_certificates(
        &self,
        input: &ListCACertificatesRequest,
    ) -> Result<ListCACertificatesResponse, ListCACertificatesError>;

    #[doc="<p>Lists the certificates registered in your AWS account.</p> <p>The results are paginated with a default page size of 25. You can use the returned marker to retrieve additional results.</p>"]
    fn list_certificates(
        &self,
        input: &ListCertificatesRequest,
    ) -> Result<ListCertificatesResponse, ListCertificatesError>;

    #[doc = "<p>List the device certificates signed by the specified CA certificate.</p>"]
    fn list_certificates_by_ca(
        &self,
        input: &ListCertificatesByCARequest,
    ) -> Result<ListCertificatesByCAResponse, ListCertificatesByCAError>;

    #[doc = "<p>Lists the search indices.</p>"]
    fn list_indices(
        &self,
        input: &ListIndicesRequest,
    ) -> Result<ListIndicesResponse, ListIndicesError>;

    #[doc = "<p>Lists the job executions for a job.</p>"]
    fn list_job_executions_for_job(
        &self,
        input: &ListJobExecutionsForJobRequest,
    ) -> Result<ListJobExecutionsForJobResponse, ListJobExecutionsForJobError>;

    #[doc = "<p>Lists the job executions for the specified thing.</p>"]
    fn list_job_executions_for_thing(
        &self,
        input: &ListJobExecutionsForThingRequest,
    ) -> Result<ListJobExecutionsForThingResponse, ListJobExecutionsForThingError>;

    #[doc = "<p>Lists jobs.</p>"]
    fn list_jobs(&self, input: &ListJobsRequest) -> Result<ListJobsResponse, ListJobsError>;

    #[doc = "<p>Lists OTA updates.</p>"]
    fn list_ota_updates(
        &self,
        input: &ListOTAUpdatesRequest,
    ) -> Result<ListOTAUpdatesResponse, ListOTAUpdatesError>;

    #[doc = "<p>Lists certificates that are being transferred but not yet accepted.</p>"]
    fn list_outgoing_certificates(
        &self,
        input: &ListOutgoingCertificatesRequest,
    ) -> Result<ListOutgoingCertificatesResponse, ListOutgoingCertificatesError>;

    #[doc = "<p>Lists your policies.</p>"]
    fn list_policies(
        &self,
        input: &ListPoliciesRequest,
    ) -> Result<ListPoliciesResponse, ListPoliciesError>;

    #[doc="<p>Lists the principals associated with the specified policy.</p> <p> <b>Note:</b> This API is deprecated. Please use <a>ListTargetsForPolicy</a> instead.</p>"]
    fn list_policy_principals(
        &self,
        input: &ListPolicyPrincipalsRequest,
    ) -> Result<ListPolicyPrincipalsResponse, ListPolicyPrincipalsError>;

    #[doc = "<p>Lists the versions of the specified policy and identifies the default version.</p>"]
    fn list_policy_versions(
        &self,
        input: &ListPolicyVersionsRequest,
    ) -> Result<ListPolicyVersionsResponse, ListPolicyVersionsError>;

    #[doc="<p>Lists the policies attached to the specified principal. If you use an Cognito identity, the ID must be in <a href=\"http://docs.aws.amazon.com/cognitoidentity/latest/APIReference/API_GetCredentialsForIdentity.html#API_GetCredentialsForIdentity_RequestSyntax\">AmazonCognito Identity format</a>.</p> <p> <b>Note:</b> This API is deprecated. Please use <a>ListAttachedPolicies</a> instead.</p>"]
    fn list_principal_policies(
        &self,
        input: &ListPrincipalPoliciesRequest,
    ) -> Result<ListPrincipalPoliciesResponse, ListPrincipalPoliciesError>;

    #[doc = "<p>Lists the things associated with the specified principal.</p>"]
    fn list_principal_things(
        &self,
        input: &ListPrincipalThingsRequest,
    ) -> Result<ListPrincipalThingsResponse, ListPrincipalThingsError>;

    #[doc = "<p>Lists the role aliases registered in your account.</p>"]
    fn list_role_aliases(
        &self,
        input: &ListRoleAliasesRequest,
    ) -> Result<ListRoleAliasesResponse, ListRoleAliasesError>;

    #[doc = "<p>Lists all of the streams in your AWS account.</p>"]
    fn list_streams(
        &self,
        input: &ListStreamsRequest,
    ) -> Result<ListStreamsResponse, ListStreamsError>;

    #[doc = "<p>List targets for the specified policy.</p>"]
    fn list_targets_for_policy(
        &self,
        input: &ListTargetsForPolicyRequest,
    ) -> Result<ListTargetsForPolicyResponse, ListTargetsForPolicyError>;

    #[doc = "<p>List the thing groups in your account.</p>"]
    fn list_thing_groups(
        &self,
        input: &ListThingGroupsRequest,
    ) -> Result<ListThingGroupsResponse, ListThingGroupsError>;

    #[doc = "<p>List the thing groups to which the specified thing belongs.</p>"]
    fn list_thing_groups_for_thing(
        &self,
        input: &ListThingGroupsForThingRequest,
    ) -> Result<ListThingGroupsForThingResponse, ListThingGroupsForThingError>;

    #[doc = "<p>Lists the principals associated with the specified thing.</p>"]
    fn list_thing_principals(
        &self,
        input: &ListThingPrincipalsRequest,
    ) -> Result<ListThingPrincipalsResponse, ListThingPrincipalsError>;

    #[doc = "<p>Information about the thing registration tasks.</p>"]
    fn list_thing_registration_task_reports(
        &self,
        input: &ListThingRegistrationTaskReportsRequest,
    ) -> Result<ListThingRegistrationTaskReportsResponse, ListThingRegistrationTaskReportsError>;

    #[doc = "<p>List bulk thing provisioning tasks.</p>"]
    fn list_thing_registration_tasks(
        &self,
        input: &ListThingRegistrationTasksRequest,
    ) -> Result<ListThingRegistrationTasksResponse, ListThingRegistrationTasksError>;

    #[doc = "<p>Lists the existing thing types.</p>"]
    fn list_thing_types(
        &self,
        input: &ListThingTypesRequest,
    ) -> Result<ListThingTypesResponse, ListThingTypesError>;

    #[doc="<p>Lists your things. Use the <b>attributeName</b> and <b>attributeValue</b> parameters to filter your things. For example, calling <code>ListThings</code> with attributeName=Color and attributeValue=Red retrieves all things in the registry that contain an attribute <b>Color</b> with the value <b>Red</b>. </p>"]
    fn list_things(&self, input: &ListThingsRequest)
        -> Result<ListThingsResponse, ListThingsError>;

    #[doc = "<p>Lists the things in the specified group.</p>"]
    fn list_things_in_thing_group(
        &self,
        input: &ListThingsInThingGroupRequest,
    ) -> Result<ListThingsInThingGroupResponse, ListThingsInThingGroupError>;

    #[doc = "<p>Lists the rules for the specific topic.</p>"]
    fn list_topic_rules(
        &self,
        input: &ListTopicRulesRequest,
    ) -> Result<ListTopicRulesResponse, ListTopicRulesError>;

    #[doc = "<p>Lists logging levels.</p>"]
    fn list_v2_logging_levels(
        &self,
        input: &ListV2LoggingLevelsRequest,
    ) -> Result<ListV2LoggingLevelsResponse, ListV2LoggingLevelsError>;

    #[doc="<p>Registers a CA certificate with AWS IoT. This CA certificate can then be used to sign device certificates, which can be then registered with AWS IoT. You can register up to 10 CA certificates per AWS account that have the same subject field. This enables you to have up to 10 certificate authorities sign your device certificates. If you have more than one CA certificate registered, make sure you pass the CA certificate when you register your device certificates with the RegisterCertificate API.</p>"]
    fn register_ca_certificate(
        &self,
        input: &RegisterCACertificateRequest,
    ) -> Result<RegisterCACertificateResponse, RegisterCACertificateError>;

    #[doc="<p>Registers a device certificate with AWS IoT. If you have more than one CA certificate that has the same subject field, you must specify the CA certificate that was used to sign the device certificate being registered.</p>"]
    fn register_certificate(
        &self,
        input: &RegisterCertificateRequest,
    ) -> Result<RegisterCertificateResponse, RegisterCertificateError>;

    #[doc = "<p>Provisions a thing.</p>"]
    fn register_thing(
        &self,
        input: &RegisterThingRequest,
    ) -> Result<RegisterThingResponse, RegisterThingError>;

    #[doc="<p>Rejects a pending certificate transfer. After AWS IoT rejects a certificate transfer, the certificate status changes from <b>PENDING_TRANSFER</b> to <b>INACTIVE</b>.</p> <p>To check for pending certificate transfers, call <a>ListCertificates</a> to enumerate your certificates.</p> <p>This operation can only be called by the transfer destination. After it is called, the certificate will be returned to the source's account in the INACTIVE state.</p>"]
    fn reject_certificate_transfer(
        &self,
        input: &RejectCertificateTransferRequest,
    ) -> Result<(), RejectCertificateTransferError>;

    #[doc = "<p>Remove the specified thing from the specified group.</p>"]
    fn remove_thing_from_thing_group(
        &self,
        input: &RemoveThingFromThingGroupRequest,
    ) -> Result<RemoveThingFromThingGroupResponse, RemoveThingFromThingGroupError>;

    #[doc="<p>Replaces the rule. You must specify all parameters for the new rule. Creating rules is an administrator-level action. Any user who has permission to create rules will be able to access data processed by the rule.</p>"]
    fn replace_topic_rule(
        &self,
        input: &ReplaceTopicRuleRequest,
    ) -> Result<(), ReplaceTopicRuleError>;

    #[doc = "<p>The query search index.</p>"]
    fn search_index(
        &self,
        input: &SearchIndexRequest,
    ) -> Result<SearchIndexResponse, SearchIndexError>;

    #[doc="<p>Sets the default authorizer. This will be used if a websocket connection is made without specifying an authorizer.</p>"]
    fn set_default_authorizer(
        &self,
        input: &SetDefaultAuthorizerRequest,
    ) -> Result<SetDefaultAuthorizerResponse, SetDefaultAuthorizerError>;

    #[doc="<p>Sets the specified version of the specified policy as the policy's default (operative) version. This action affects all certificates to which the policy is attached. To list the principals the policy is attached to, use the ListPrincipalPolicy API.</p>"]
    fn set_default_policy_version(
        &self,
        input: &SetDefaultPolicyVersionRequest,
    ) -> Result<(), SetDefaultPolicyVersionError>;

    #[doc = "<p>Sets the logging options.</p>"]
    fn set_logging_options(
        &self,
        input: &SetLoggingOptionsRequest,
    ) -> Result<(), SetLoggingOptionsError>;

    #[doc = "<p>Sets the logging level.</p>"]
    fn set_v2_logging_level(
        &self,
        input: &SetV2LoggingLevelRequest,
    ) -> Result<(), SetV2LoggingLevelError>;

    #[doc = "<p>Sets the logging options for the V2 logging service.</p>"]
    fn set_v2_logging_options(
        &self,
        input: &SetV2LoggingOptionsRequest,
    ) -> Result<(), SetV2LoggingOptionsError>;

    #[doc = "<p>Creates a bulk thing provisioning task.</p>"]
    fn start_thing_registration_task(
        &self,
        input: &StartThingRegistrationTaskRequest,
    ) -> Result<StartThingRegistrationTaskResponse, StartThingRegistrationTaskError>;

    #[doc = "<p>Cancels a bulk thing provisioning task.</p>"]
    fn stop_thing_registration_task(
        &self,
        input: &StopThingRegistrationTaskRequest,
    ) -> Result<StopThingRegistrationTaskResponse, StopThingRegistrationTaskError>;

    #[doc = "<p>Test custom authorization.</p>"]
    fn test_authorization(
        &self,
        input: &TestAuthorizationRequest,
    ) -> Result<TestAuthorizationResponse, TestAuthorizationError>;

    #[doc = "<p>Invoke the specified custom authorizer for testing purposes.</p>"]
    fn test_invoke_authorizer(
        &self,
        input: &TestInvokeAuthorizerRequest,
    ) -> Result<TestInvokeAuthorizerResponse, TestInvokeAuthorizerError>;

    #[doc="<p>Transfers the specified certificate to the specified AWS account.</p> <p>You can cancel the transfer until it is acknowledged by the recipient.</p> <p>No notification is sent to the transfer destination's account. It is up to the caller to notify the transfer target.</p> <p>The certificate being transferred must not be in the ACTIVE state. You can use the UpdateCertificate API to deactivate it.</p> <p>The certificate must not have any policies attached to it. You can use the DetachPrincipalPolicy API to detach them.</p>"]
    fn transfer_certificate(
        &self,
        input: &TransferCertificateRequest,
    ) -> Result<TransferCertificateResponse, TransferCertificateError>;

    #[doc = "<p>Updates an authorizer.</p>"]
    fn update_authorizer(
        &self,
        input: &UpdateAuthorizerRequest,
    ) -> Result<UpdateAuthorizerResponse, UpdateAuthorizerError>;

    #[doc = "<p>Updates a registered CA certificate.</p>"]
    fn update_ca_certificate(
        &self,
        input: &UpdateCACertificateRequest,
    ) -> Result<(), UpdateCACertificateError>;

    #[doc="<p>Updates the status of the specified certificate. This operation is idempotent.</p> <p>Moving a certificate from the ACTIVE state (including REVOKED) will not disconnect currently connected devices, but these devices will be unable to reconnect.</p> <p>The ACTIVE state is required to authenticate devices connecting to AWS IoT using a certificate.</p>"]
    fn update_certificate(
        &self,
        input: &UpdateCertificateRequest,
    ) -> Result<(), UpdateCertificateError>;

    #[doc = "<p>Updates the event configurations.</p>"]
    fn update_event_configurations(
        &self,
        input: &UpdateEventConfigurationsRequest,
    ) -> Result<UpdateEventConfigurationsResponse, UpdateEventConfigurationsError>;

    #[doc = "<p>Updates the search configuration.</p>"]
    fn update_indexing_configuration(
        &self,
        input: &UpdateIndexingConfigurationRequest,
    ) -> Result<UpdateIndexingConfigurationResponse, UpdateIndexingConfigurationError>;

    #[doc = "<p>Updates a role alias.</p>"]
    fn update_role_alias(
        &self,
        input: &UpdateRoleAliasRequest,
    ) -> Result<UpdateRoleAliasResponse, UpdateRoleAliasError>;

    #[doc = "<p>Updates an existing stream. The stream version will be incremented by one.</p>"]
    fn update_stream(
        &self,
        input: &UpdateStreamRequest,
    ) -> Result<UpdateStreamResponse, UpdateStreamError>;

    #[doc = "<p>Updates the data for a thing.</p>"]
    fn update_thing(
        &self,
        input: &UpdateThingRequest,
    ) -> Result<UpdateThingResponse, UpdateThingError>;

    #[doc = "<p>Update a thing group.</p>"]
    fn update_thing_group(
        &self,
        input: &UpdateThingGroupRequest,
    ) -> Result<UpdateThingGroupResponse, UpdateThingGroupError>;

    #[doc = "<p>Updates the groups to which the thing belongs.</p>"]
    fn update_thing_groups_for_thing(
        &self,
        input: &UpdateThingGroupsForThingRequest,
    ) -> Result<UpdateThingGroupsForThingResponse, UpdateThingGroupsForThingError>;
}
/// A client for the AWS IoT API.
pub struct IotClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> IotClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        IotClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> Iot for IotClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    #[doc="<p>Accepts a pending certificate transfer. The default state of the certificate is INACTIVE.</p> <p>To check for pending certificate transfers, call <a>ListCertificates</a> to enumerate your certificates.</p>"]
    fn accept_certificate_transfer(
        &self,
        input: &AcceptCertificateTransferRequest,
    ) -> Result<(), AcceptCertificateTransferError> {
        let request_uri = format!(
            "/accept-certificate-transfer/{certificate_id}",
            certificate_id = input.certificate_id
        );

        let mut request = SignedRequest::new("PATCH", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.set_as_active {
            params.put("setAsActive", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AcceptCertificateTransferError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Adds a thing to a thing group.</p>"]
    fn add_thing_to_thing_group(
        &self,
        input: &AddThingToThingGroupRequest,
    ) -> Result<AddThingToThingGroupResponse, AddThingToThingGroupError> {
        let request_uri = "/thing-groups/addThingToThingGroup";

        let mut request = SignedRequest::new("PUT", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<AddThingToThingGroupResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AddThingToThingGroupError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Associates a group with a continuous job. The following criteria must be met: </p> <ul> <li> <p>The job must have been created with the <code>targetSelection</code> field set to \"CONTINUOUS\".</p> </li> <li> <p>The job status must currently be \"IN_PROGRESS\".</p> </li> <li> <p>The total number of targets associated with a job must not exceed 100.</p> </li> </ul>"]
    fn associate_targets_with_job(
        &self,
        input: &AssociateTargetsWithJobRequest,
    ) -> Result<AssociateTargetsWithJobResponse, AssociateTargetsWithJobError> {
        let request_uri = format!("/jobs/{job_id}/targets", job_id = input.job_id);

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<AssociateTargetsWithJobResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AssociateTargetsWithJobError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Attaches a policy to the specified target.</p>"]
    fn attach_policy(&self, input: &AttachPolicyRequest) -> Result<(), AttachPolicyError> {
        let request_uri = format!(
            "/target-policies/{policy_name}",
            policy_name = input.policy_name
        );

        let mut request = SignedRequest::new("PUT", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AttachPolicyError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Attaches the specified policy to the specified principal (certificate or other credential).</p> <p> <b>Note:</b> This API is deprecated. Please use <a>AttachPolicy</a> instead.</p>"]
    fn attach_principal_policy(
        &self,
        input: &AttachPrincipalPolicyRequest,
    ) -> Result<(), AttachPrincipalPolicyError> {
        let request_uri = format!(
            "/principal-policies/{policy_name}",
            policy_name = input.policy_name
        );

        let mut request = SignedRequest::new("PUT", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.add_header("x-amzn-iot-principal", &input.principal);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AttachPrincipalPolicyError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Attaches the specified principal to the specified thing.</p>"]
    fn attach_thing_principal(
        &self,
        input: &AttachThingPrincipalRequest,
    ) -> Result<AttachThingPrincipalResponse, AttachThingPrincipalError> {
        let request_uri = format!(
            "/things/{thing_name}/principals",
            thing_name = input.thing_name
        );

        let mut request = SignedRequest::new("PUT", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.add_header("x-amzn-principal", &input.principal);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<AttachThingPrincipalResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(AttachThingPrincipalError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Cancels a pending transfer for the specified certificate.</p> <p> <b>Note</b> Only the transfer source account can use this operation to cancel a transfer. (Transfer destinations can use <a>RejectCertificateTransfer</a> instead.) After transfer, AWS IoT returns the certificate to the source account in the INACTIVE state. After the destination account has accepted the transfer, the transfer cannot be cancelled.</p> <p>After a certificate transfer is cancelled, the status of the certificate changes from PENDING_TRANSFER to INACTIVE.</p>"]
    fn cancel_certificate_transfer(
        &self,
        input: &CancelCertificateTransferRequest,
    ) -> Result<(), CancelCertificateTransferError> {
        let request_uri = format!(
            "/cancel-certificate-transfer/{certificate_id}",
            certificate_id = input.certificate_id
        );

        let mut request = SignedRequest::new("PATCH", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CancelCertificateTransferError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Cancels a job.</p>"]
    fn cancel_job(&self, input: &CancelJobRequest) -> Result<CancelJobResponse, CancelJobError> {
        let request_uri = format!("/jobs/{job_id}/cancel", job_id = input.job_id);

        let mut request = SignedRequest::new("PUT", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CancelJobResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CancelJobError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Clears the default authorizer.</p>"]
    fn clear_default_authorizer(
        &self,
    ) -> Result<ClearDefaultAuthorizerResponse, ClearDefaultAuthorizerError> {
        let request_uri = "/default-authorizer";

        let mut request = SignedRequest::new("DELETE", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<ClearDefaultAuthorizerResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ClearDefaultAuthorizerError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Creates an authorizer.</p>"]
    fn create_authorizer(
        &self,
        input: &CreateAuthorizerRequest,
    ) -> Result<CreateAuthorizerResponse, CreateAuthorizerError> {
        let request_uri = format!(
            "/authorizer/{authorizer_name}",
            authorizer_name = input.authorizer_name
        );

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateAuthorizerResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateAuthorizerError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Creates an X.509 certificate using the specified certificate signing request.</p> <p> <b>Note:</b> The CSR must include a public key that is either an RSA key with a length of at least 2048 bits or an ECC key from NIST P-256 or NIST P-384 curves. </p> <p> <b>Note:</b> Reusing the same certificate signing request (CSR) results in a distinct certificate.</p> <p>You can create multiple certificates in a batch by creating a directory, copying multiple .csr files into that directory, and then specifying that directory on the command line. The following commands show how to create a batch of certificates given a batch of CSRs.</p> <p>Assuming a set of CSRs are located inside of the directory my-csr-directory:</p> <p>On Linux and OS X, the command is:</p> <p>$ ls my-csr-directory/ | xargs -I {} aws iot create-certificate-from-csr --certificate-signing-request file://my-csr-directory/{}</p> <p>This command lists all of the CSRs in my-csr-directory and pipes each CSR file name to the aws iot create-certificate-from-csr AWS CLI command to create a certificate for the corresponding CSR.</p> <p>The aws iot create-certificate-from-csr part of the command can also be run in parallel to speed up the certificate creation process:</p> <p>$ ls my-csr-directory/ | xargs -P 10 -I {} aws iot create-certificate-from-csr --certificate-signing-request file://my-csr-directory/{}</p> <p>On Windows PowerShell, the command to create certificates for all CSRs in my-csr-directory is:</p> <p>&gt; ls -Name my-csr-directory | %{aws iot create-certificate-from-csr --certificate-signing-request file://my-csr-directory/$_}</p> <p>On a Windows command prompt, the command to create certificates for all CSRs in my-csr-directory is:</p> <p>&gt; forfiles /p my-csr-directory /c \"cmd /c aws iot create-certificate-from-csr --certificate-signing-request file://@path\"</p>"]
    fn create_certificate_from_csr(
        &self,
        input: &CreateCertificateFromCsrRequest,
    ) -> Result<CreateCertificateFromCsrResponse, CreateCertificateFromCsrError> {
        let request_uri = "/certificates";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.set_as_active {
            params.put("setAsActive", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<CreateCertificateFromCsrResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateCertificateFromCsrError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Creates a job.</p>"]
    fn create_job(&self, input: &CreateJobRequest) -> Result<CreateJobResponse, CreateJobError> {
        let request_uri = format!("/jobs/{job_id}", job_id = input.job_id);

        let mut request = SignedRequest::new("PUT", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateJobResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateJobError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Creates a 2048-bit RSA key pair and issues an X.509 certificate using the issued public key.</p> <p> <b>Note</b> This is the only time AWS IoT issues the private key for this certificate, so it is important to keep it in a secure location.</p>"]
    fn create_keys_and_certificate(
        &self,
        input: &CreateKeysAndCertificateRequest,
    ) -> Result<CreateKeysAndCertificateResponse, CreateKeysAndCertificateError> {
        let request_uri = "/keys-and-certificate";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.set_as_active {
            params.put("setAsActive", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<CreateKeysAndCertificateResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateKeysAndCertificateError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Creates an AWS IoT OTAUpdate on a target group of things or groups.</p>"]
    fn create_ota_update(
        &self,
        input: &CreateOTAUpdateRequest,
    ) -> Result<CreateOTAUpdateResponse, CreateOTAUpdateError> {
        let request_uri = format!(
            "/otaUpdates/{ota_update_id}",
            ota_update_id = input.ota_update_id
        );

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateOTAUpdateResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateOTAUpdateError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Creates an AWS IoT policy.</p> <p>The created policy is the default version for the policy. This operation creates a policy version with a version identifier of <b>1</b> and sets <b>1</b> as the policy's default version.</p>"]
    fn create_policy(
        &self,
        input: &CreatePolicyRequest,
    ) -> Result<CreatePolicyResponse, CreatePolicyError> {
        let request_uri = format!("/policies/{policy_name}", policy_name = input.policy_name);

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreatePolicyResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreatePolicyError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Creates a new version of the specified AWS IoT policy. To update a policy, create a new policy version. A managed policy can have up to five versions. If the policy has five versions, you must use <a>DeletePolicyVersion</a> to delete an existing version before you create a new one.</p> <p>Optionally, you can set the new version as the policy's default version. The default version is the operative version (that is, the version that is in effect for the certificates to which the policy is attached).</p>"]
    fn create_policy_version(
        &self,
        input: &CreatePolicyVersionRequest,
    ) -> Result<CreatePolicyVersionResponse, CreatePolicyVersionError> {
        let request_uri = format!(
            "/policies/{policy_name}/version",
            policy_name = input.policy_name
        );

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.set_as_default {
            params.put("setAsDefault", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreatePolicyVersionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreatePolicyVersionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Creates a role alias.</p>"]
    fn create_role_alias(
        &self,
        input: &CreateRoleAliasRequest,
    ) -> Result<CreateRoleAliasResponse, CreateRoleAliasError> {
        let request_uri = format!("/role-aliases/{role_alias}", role_alias = input.role_alias);

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateRoleAliasResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateRoleAliasError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Creates a stream for delivering one or more large files in chunks over MQTT. A stream transports data bytes in chunks or blocks packaged as MQTT messages from a source like S3. You can have one or more files associated with a stream. The total size of a file associated with the stream cannot exceed more than 2 MB. The stream will be created with version 0. If a stream is created with the same streamID as a stream that existed and was deleted within last 90 days, we will resurrect that old stream by incrementing the version by 1.</p>"]
    fn create_stream(
        &self,
        input: &CreateStreamRequest,
    ) -> Result<CreateStreamResponse, CreateStreamError> {
        let request_uri = format!("/streams/{stream_id}", stream_id = input.stream_id);

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateStreamResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateStreamError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Creates a thing record in the thing registry.</p>"]
    fn create_thing(
        &self,
        input: &CreateThingRequest,
    ) -> Result<CreateThingResponse, CreateThingError> {
        let request_uri = format!("/things/{thing_name}", thing_name = input.thing_name);

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateThingResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateThingError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Create a thing group.</p>"]
    fn create_thing_group(
        &self,
        input: &CreateThingGroupRequest,
    ) -> Result<CreateThingGroupResponse, CreateThingGroupError> {
        let request_uri = format!(
            "/thing-groups/{thing_group_name}",
            thing_group_name = input.thing_group_name
        );

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateThingGroupResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateThingGroupError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Creates a new thing type.</p>"]
    fn create_thing_type(
        &self,
        input: &CreateThingTypeRequest,
    ) -> Result<CreateThingTypeResponse, CreateThingTypeError> {
        let request_uri = format!(
            "/thing-types/{thing_type_name}",
            thing_type_name = input.thing_type_name
        );

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<CreateThingTypeResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateThingTypeError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Creates a rule. Creating rules is an administrator-level action. Any user who has permission to create rules will be able to access data processed by the rule.</p>"]
    fn create_topic_rule(
        &self,
        input: &CreateTopicRuleRequest,
    ) -> Result<(), CreateTopicRuleError> {
        let request_uri = format!("/rules/{rule_name}", rule_name = input.rule_name);

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(&input.topic_rule_payload).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateTopicRuleError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Deletes an authorizer.</p>"]
    fn delete_authorizer(
        &self,
        input: &DeleteAuthorizerRequest,
    ) -> Result<DeleteAuthorizerResponse, DeleteAuthorizerError> {
        let request_uri = format!(
            "/authorizer/{authorizer_name}",
            authorizer_name = input.authorizer_name
        );

        let mut request = SignedRequest::new("DELETE", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteAuthorizerResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteAuthorizerError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Deletes a registered CA certificate.</p>"]
    fn delete_ca_certificate(
        &self,
        input: &DeleteCACertificateRequest,
    ) -> Result<DeleteCACertificateResponse, DeleteCACertificateError> {
        let request_uri = format!(
            "/cacertificate/{ca_certificate_id}",
            ca_certificate_id = input.certificate_id
        );

        let mut request = SignedRequest::new("DELETE", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteCACertificateResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteCACertificateError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Deletes the specified certificate.</p> <p>A certificate cannot be deleted if it has a policy attached to it or if its status is set to ACTIVE. To delete a certificate, first use the <a>DetachPrincipalPolicy</a> API to detach all policies. Next, use the <a>UpdateCertificate</a> API to set the certificate to the INACTIVE status.</p>"]
    fn delete_certificate(
        &self,
        input: &DeleteCertificateRequest,
    ) -> Result<(), DeleteCertificateError> {
        let request_uri = format!(
            "/certificates/{certificate_id}",
            certificate_id = input.certificate_id
        );

        let mut request = SignedRequest::new("DELETE", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.force_delete {
            params.put("forceDelete", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteCertificateError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Delete an OTA update.</p>"]
    fn delete_ota_update(
        &self,
        input: &DeleteOTAUpdateRequest,
    ) -> Result<DeleteOTAUpdateResponse, DeleteOTAUpdateError> {
        let request_uri = format!(
            "/otaUpdates/{ota_update_id}",
            ota_update_id = input.ota_update_id
        );

        let mut request = SignedRequest::new("DELETE", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteOTAUpdateResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteOTAUpdateError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Deletes the specified policy.</p> <p>A policy cannot be deleted if it has non-default versions or it is attached to any certificate.</p> <p>To delete a policy, use the DeletePolicyVersion API to delete all non-default versions of the policy; use the DetachPrincipalPolicy API to detach the policy from any certificate; and then use the DeletePolicy API to delete the policy.</p> <p>When a policy is deleted using DeletePolicy, its default version is deleted with it.</p>"]
    fn delete_policy(&self, input: &DeletePolicyRequest) -> Result<(), DeletePolicyError> {
        let request_uri = format!("/policies/{policy_name}", policy_name = input.policy_name);

        let mut request = SignedRequest::new("DELETE", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeletePolicyError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Deletes the specified version of the specified policy. You cannot delete the default version of a policy using this API. To delete the default version of a policy, use <a>DeletePolicy</a>. To find out which version of a policy is marked as the default version, use ListPolicyVersions.</p>"]
    fn delete_policy_version(
        &self,
        input: &DeletePolicyVersionRequest,
    ) -> Result<(), DeletePolicyVersionError> {
        let request_uri = format!(
            "/policies/{policy_name}/version/{policy_version_id}",
            policy_name = input.policy_name,
            policy_version_id = input.policy_version_id
        );

        let mut request = SignedRequest::new("DELETE", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeletePolicyVersionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Deletes a CA certificate registration code.</p>"]
    fn delete_registration_code(
        &self,
    ) -> Result<DeleteRegistrationCodeResponse, DeleteRegistrationCodeError> {
        let request_uri = "/registrationcode";

        let mut request = SignedRequest::new("DELETE", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<DeleteRegistrationCodeResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteRegistrationCodeError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Deletes a role alias</p>"]
    fn delete_role_alias(
        &self,
        input: &DeleteRoleAliasRequest,
    ) -> Result<DeleteRoleAliasResponse, DeleteRoleAliasError> {
        let request_uri = format!("/role-aliases/{role_alias}", role_alias = input.role_alias);

        let mut request = SignedRequest::new("DELETE", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteRoleAliasResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteRoleAliasError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Deletes a stream.</p>"]
    fn delete_stream(
        &self,
        input: &DeleteStreamRequest,
    ) -> Result<DeleteStreamResponse, DeleteStreamError> {
        let request_uri = format!("/streams/{stream_id}", stream_id = input.stream_id);

        let mut request = SignedRequest::new("DELETE", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteStreamResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteStreamError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Deletes the specified thing.</p>"]
    fn delete_thing(
        &self,
        input: &DeleteThingRequest,
    ) -> Result<DeleteThingResponse, DeleteThingError> {
        let request_uri = format!("/things/{thing_name}", thing_name = input.thing_name);

        let mut request = SignedRequest::new("DELETE", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.expected_version {
            params.put("expectedVersion", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteThingResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteThingError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Deletes a thing group.</p>"]
    fn delete_thing_group(
        &self,
        input: &DeleteThingGroupRequest,
    ) -> Result<DeleteThingGroupResponse, DeleteThingGroupError> {
        let request_uri = format!(
            "/thing-groups/{thing_group_name}",
            thing_group_name = input.thing_group_name
        );

        let mut request = SignedRequest::new("DELETE", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.expected_version {
            params.put("expectedVersion", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteThingGroupResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteThingGroupError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Deletes the specified thing type . You cannot delete a thing type if it has things associated with it. To delete a thing type, first mark it as deprecated by calling <a>DeprecateThingType</a>, then remove any associated things by calling <a>UpdateThing</a> to change the thing type on any associated thing, and finally use <a>DeleteThingType</a> to delete the thing type.</p>"]
    fn delete_thing_type(
        &self,
        input: &DeleteThingTypeRequest,
    ) -> Result<DeleteThingTypeResponse, DeleteThingTypeError> {
        let request_uri = format!(
            "/thing-types/{thing_type_name}",
            thing_type_name = input.thing_type_name
        );

        let mut request = SignedRequest::new("DELETE", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteThingTypeResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteThingTypeError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Deletes the rule.</p>"]
    fn delete_topic_rule(
        &self,
        input: &DeleteTopicRuleRequest,
    ) -> Result<(), DeleteTopicRuleError> {
        let request_uri = format!("/rules/{rule_name}", rule_name = input.rule_name);

        let mut request = SignedRequest::new("DELETE", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteTopicRuleError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Deletes a logging level.</p>"]
    fn delete_v2_logging_level(
        &self,
        input: &DeleteV2LoggingLevelRequest,
    ) -> Result<(), DeleteV2LoggingLevelError> {
        let request_uri = "/v2LoggingLevel";

        let mut request = SignedRequest::new("DELETE", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        params.put("targetName", &input.target_name);
        params.put("targetType", &input.target_type);
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteV2LoggingLevelError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Deprecates a thing type. You can not associate new things with deprecated thing type.</p>"]
    fn deprecate_thing_type(
        &self,
        input: &DeprecateThingTypeRequest,
    ) -> Result<DeprecateThingTypeResponse, DeprecateThingTypeError> {
        let request_uri = format!(
            "/thing-types/{thing_type_name}/deprecate",
            thing_type_name = input.thing_type_name
        );

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeprecateThingTypeResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeprecateThingTypeError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Describes an authorizer.</p>"]
    fn describe_authorizer(
        &self,
        input: &DescribeAuthorizerRequest,
    ) -> Result<DescribeAuthorizerResponse, DescribeAuthorizerError> {
        let request_uri = format!(
            "/authorizer/{authorizer_name}",
            authorizer_name = input.authorizer_name
        );

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeAuthorizerResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeAuthorizerError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Describes a registered CA certificate.</p>"]
    fn describe_ca_certificate(
        &self,
        input: &DescribeCACertificateRequest,
    ) -> Result<DescribeCACertificateResponse, DescribeCACertificateError> {
        let request_uri = format!(
            "/cacertificate/{ca_certificate_id}",
            ca_certificate_id = input.certificate_id
        );

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<DescribeCACertificateResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeCACertificateError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Gets information about the specified certificate.</p>"]
    fn describe_certificate(
        &self,
        input: &DescribeCertificateRequest,
    ) -> Result<DescribeCertificateResponse, DescribeCertificateError> {
        let request_uri = format!(
            "/certificates/{certificate_id}",
            certificate_id = input.certificate_id
        );

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeCertificateResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeCertificateError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Describes the default authorizer.</p>"]
    fn describe_default_authorizer(
        &self,
    ) -> Result<DescribeDefaultAuthorizerResponse, DescribeDefaultAuthorizerError> {
        let request_uri = "/default-authorizer";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<DescribeDefaultAuthorizerResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeDefaultAuthorizerError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Returns a unique endpoint specific to the AWS account making the call.</p>"]
    fn describe_endpoint(
        &self,
        input: &DescribeEndpointRequest,
    ) -> Result<DescribeEndpointResponse, DescribeEndpointError> {
        let request_uri = "/endpoint";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.endpoint_type {
            params.put("endpointType", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeEndpointResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeEndpointError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Describes event configurations.</p>"]
    fn describe_event_configurations(
        &self,
    ) -> Result<DescribeEventConfigurationsResponse, DescribeEventConfigurationsError> {
        let request_uri = "/event-configurations";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<DescribeEventConfigurationsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeEventConfigurationsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Describes a search index.</p>"]
    fn describe_index(
        &self,
        input: &DescribeIndexRequest,
    ) -> Result<DescribeIndexResponse, DescribeIndexError> {
        let request_uri = format!("/indices/{index_name}", index_name = input.index_name);

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeIndexResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeIndexError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Describes a job.</p>"]
    fn describe_job(
        &self,
        input: &DescribeJobRequest,
    ) -> Result<DescribeJobResponse, DescribeJobError> {
        let request_uri = format!("/jobs/{job_id}", job_id = input.job_id);

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeJobResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeJobError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Describes a job execution.</p>"]
    fn describe_job_execution(
        &self,
        input: &DescribeJobExecutionRequest,
    ) -> Result<DescribeJobExecutionResponse, DescribeJobExecutionError> {
        let request_uri = format!(
            "/things/{thing_name}/jobs/{job_id}",
            job_id = input.job_id,
            thing_name = input.thing_name
        );

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.execution_number {
            params.put("executionNumber", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeJobExecutionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeJobExecutionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Describes a role alias.</p>"]
    fn describe_role_alias(
        &self,
        input: &DescribeRoleAliasRequest,
    ) -> Result<DescribeRoleAliasResponse, DescribeRoleAliasError> {
        let request_uri = format!("/role-aliases/{role_alias}", role_alias = input.role_alias);

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeRoleAliasResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeRoleAliasError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Gets information about a stream.</p>"]
    fn describe_stream(
        &self,
        input: &DescribeStreamRequest,
    ) -> Result<DescribeStreamResponse, DescribeStreamError> {
        let request_uri = format!("/streams/{stream_id}", stream_id = input.stream_id);

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeStreamResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeStreamError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Gets information about the specified thing.</p>"]
    fn describe_thing(
        &self,
        input: &DescribeThingRequest,
    ) -> Result<DescribeThingResponse, DescribeThingError> {
        let request_uri = format!("/things/{thing_name}", thing_name = input.thing_name);

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeThingResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeThingError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Describe a thing group.</p>"]
    fn describe_thing_group(
        &self,
        input: &DescribeThingGroupRequest,
    ) -> Result<DescribeThingGroupResponse, DescribeThingGroupError> {
        let request_uri = format!(
            "/thing-groups/{thing_group_name}",
            thing_group_name = input.thing_group_name
        );

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeThingGroupResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeThingGroupError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Describes a bulk thing provisioning task.</p>"]
    fn describe_thing_registration_task(
        &self,
        input: &DescribeThingRegistrationTaskRequest,
    ) -> Result<DescribeThingRegistrationTaskResponse, DescribeThingRegistrationTaskError> {
        let request_uri = format!(
            "/thing-registration-tasks/{task_id}",
            task_id = input.task_id
        );

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<DescribeThingRegistrationTaskResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeThingRegistrationTaskError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Gets information about the specified thing type.</p>"]
    fn describe_thing_type(
        &self,
        input: &DescribeThingTypeRequest,
    ) -> Result<DescribeThingTypeResponse, DescribeThingTypeError> {
        let request_uri = format!(
            "/thing-types/{thing_type_name}",
            thing_type_name = input.thing_type_name
        );

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeThingTypeResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeThingTypeError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Detaches a policy from the specified target.</p>"]
    fn detach_policy(&self, input: &DetachPolicyRequest) -> Result<(), DetachPolicyError> {
        let request_uri = format!(
            "/target-policies/{policy_name}",
            policy_name = input.policy_name
        );

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DetachPolicyError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Removes the specified policy from the specified certificate.</p> <p> <b>Note:</b> This API is deprecated. Please use <a>DetachPolicy</a> instead.</p>"]
    fn detach_principal_policy(
        &self,
        input: &DetachPrincipalPolicyRequest,
    ) -> Result<(), DetachPrincipalPolicyError> {
        let request_uri = format!(
            "/principal-policies/{policy_name}",
            policy_name = input.policy_name
        );

        let mut request = SignedRequest::new("DELETE", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.add_header("x-amzn-iot-principal", &input.principal);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DetachPrincipalPolicyError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Detaches the specified principal from the specified thing.</p>"]
    fn detach_thing_principal(
        &self,
        input: &DetachThingPrincipalRequest,
    ) -> Result<DetachThingPrincipalResponse, DetachThingPrincipalError> {
        let request_uri = format!(
            "/things/{thing_name}/principals",
            thing_name = input.thing_name
        );

        let mut request = SignedRequest::new("DELETE", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.add_header("x-amzn-principal", &input.principal);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DetachThingPrincipalResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DetachThingPrincipalError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Disables the rule.</p>"]
    fn disable_topic_rule(
        &self,
        input: &DisableTopicRuleRequest,
    ) -> Result<(), DisableTopicRuleError> {
        let request_uri = format!("/rules/{rule_name}/disable", rule_name = input.rule_name);

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DisableTopicRuleError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Enables the rule.</p>"]
    fn enable_topic_rule(
        &self,
        input: &EnableTopicRuleRequest,
    ) -> Result<(), EnableTopicRuleError> {
        let request_uri = format!("/rules/{rule_name}/enable", rule_name = input.rule_name);

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(EnableTopicRuleError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Gets effective policies.</p>"]
    fn get_effective_policies(
        &self,
        input: &GetEffectivePoliciesRequest,
    ) -> Result<GetEffectivePoliciesResponse, GetEffectivePoliciesError> {
        let request_uri = "/effective-policies";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.thing_name {
            params.put("thingName", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetEffectivePoliciesResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetEffectivePoliciesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Gets the search configuration.</p>"]
    fn get_indexing_configuration(
        &self,
    ) -> Result<GetIndexingConfigurationResponse, GetIndexingConfigurationError> {
        let request_uri = "/indexing/config";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<GetIndexingConfigurationResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetIndexingConfigurationError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Gets a job document.</p>"]
    fn get_job_document(
        &self,
        input: &GetJobDocumentRequest,
    ) -> Result<GetJobDocumentResponse, GetJobDocumentError> {
        let request_uri = format!("/jobs/{job_id}/job-document", job_id = input.job_id);

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetJobDocumentResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetJobDocumentError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Gets the logging options.</p>"]
    fn get_logging_options(&self) -> Result<GetLoggingOptionsResponse, GetLoggingOptionsError> {
        let request_uri = "/loggingOptions";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetLoggingOptionsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetLoggingOptionsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Gets an OTA update.</p>"]
    fn get_ota_update(
        &self,
        input: &GetOTAUpdateRequest,
    ) -> Result<GetOTAUpdateResponse, GetOTAUpdateError> {
        let request_uri = format!(
            "/otaUpdates/{ota_update_id}",
            ota_update_id = input.ota_update_id
        );

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetOTAUpdateResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetOTAUpdateError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Gets information about the specified policy with the policy document of the default version.</p>"]
    fn get_policy(&self, input: &GetPolicyRequest) -> Result<GetPolicyResponse, GetPolicyError> {
        let request_uri = format!("/policies/{policy_name}", policy_name = input.policy_name);

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetPolicyResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetPolicyError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Gets information about the specified policy version.</p>"]
    fn get_policy_version(
        &self,
        input: &GetPolicyVersionRequest,
    ) -> Result<GetPolicyVersionResponse, GetPolicyVersionError> {
        let request_uri = format!(
            "/policies/{policy_name}/version/{policy_version_id}",
            policy_name = input.policy_name,
            policy_version_id = input.policy_version_id
        );

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetPolicyVersionResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetPolicyVersionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Gets a registration code used to register a CA certificate with AWS IoT.</p>"]
    fn get_registration_code(
        &self,
    ) -> Result<GetRegistrationCodeResponse, GetRegistrationCodeError> {
        let request_uri = "/registrationcode";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetRegistrationCodeResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetRegistrationCodeError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Gets information about the rule.</p>"]
    fn get_topic_rule(
        &self,
        input: &GetTopicRuleRequest,
    ) -> Result<GetTopicRuleResponse, GetTopicRuleError> {
        let request_uri = format!("/rules/{rule_name}", rule_name = input.rule_name);

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetTopicRuleResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetTopicRuleError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Gets the fine grained logging options.</p>"]
    fn get_v2_logging_options(
        &self,
    ) -> Result<GetV2LoggingOptionsResponse, GetV2LoggingOptionsError> {
        let request_uri = "/v2LoggingOptions";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetV2LoggingOptionsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetV2LoggingOptionsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Lists the policies attached to the specified thing group.</p>"]
    fn list_attached_policies(
        &self,
        input: &ListAttachedPoliciesRequest,
    ) -> Result<ListAttachedPoliciesResponse, ListAttachedPoliciesError> {
        let request_uri = format!("/attached-policies/{target}", target = input.target);

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("pageSize", x);
        }
        if let Some(ref x) = input.recursive {
            params.put("recursive", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListAttachedPoliciesResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListAttachedPoliciesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Lists the authorizers registered in your account.</p>"]
    fn list_authorizers(
        &self,
        input: &ListAuthorizersRequest,
    ) -> Result<ListAuthorizersResponse, ListAuthorizersError> {
        let request_uri = "/authorizers/";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.ascending_order {
            params.put("isAscendingOrder", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("pageSize", x);
        }
        if let Some(ref x) = input.status {
            params.put("status", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListAuthorizersResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListAuthorizersError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Lists the CA certificates registered for your AWS account.</p> <p>The results are paginated with a default page size of 25. You can use the returned marker to retrieve additional results.</p>"]
    fn list_ca_certificates(
        &self,
        input: &ListCACertificatesRequest,
    ) -> Result<ListCACertificatesResponse, ListCACertificatesError> {
        let request_uri = "/cacertificates";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.ascending_order {
            params.put("isAscendingOrder", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("pageSize", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListCACertificatesResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListCACertificatesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Lists the certificates registered in your AWS account.</p> <p>The results are paginated with a default page size of 25. You can use the returned marker to retrieve additional results.</p>"]
    fn list_certificates(
        &self,
        input: &ListCertificatesRequest,
    ) -> Result<ListCertificatesResponse, ListCertificatesError> {
        let request_uri = "/certificates";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.ascending_order {
            params.put("isAscendingOrder", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("pageSize", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListCertificatesResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListCertificatesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>List the device certificates signed by the specified CA certificate.</p>"]
    fn list_certificates_by_ca(
        &self,
        input: &ListCertificatesByCARequest,
    ) -> Result<ListCertificatesByCAResponse, ListCertificatesByCAError> {
        let request_uri = format!(
            "/certificates-by-ca/{ca_certificate_id}",
            ca_certificate_id = input.ca_certificate_id
        );

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.ascending_order {
            params.put("isAscendingOrder", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("pageSize", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListCertificatesByCAResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListCertificatesByCAError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Lists the search indices.</p>"]
    fn list_indices(
        &self,
        input: &ListIndicesRequest,
    ) -> Result<ListIndicesResponse, ListIndicesError> {
        let request_uri = "/indices";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListIndicesResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListIndicesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Lists the job executions for a job.</p>"]
    fn list_job_executions_for_job(
        &self,
        input: &ListJobExecutionsForJobRequest,
    ) -> Result<ListJobExecutionsForJobResponse, ListJobExecutionsForJobError> {
        let request_uri = format!("/jobs/{job_id}/things", job_id = input.job_id);

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.status {
            params.put("status", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<ListJobExecutionsForJobResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListJobExecutionsForJobError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Lists the job executions for the specified thing.</p>"]
    fn list_job_executions_for_thing(
        &self,
        input: &ListJobExecutionsForThingRequest,
    ) -> Result<ListJobExecutionsForThingResponse, ListJobExecutionsForThingError> {
        let request_uri = format!("/things/{thing_name}/jobs", thing_name = input.thing_name);

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.status {
            params.put("status", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<ListJobExecutionsForThingResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListJobExecutionsForThingError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Lists jobs.</p>"]
    fn list_jobs(&self, input: &ListJobsRequest) -> Result<ListJobsResponse, ListJobsError> {
        let request_uri = "/jobs";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.status {
            params.put("status", x);
        }
        if let Some(ref x) = input.target_selection {
            params.put("targetSelection", x);
        }
        if let Some(ref x) = input.thing_group_id {
            params.put("thingGroupId", x);
        }
        if let Some(ref x) = input.thing_group_name {
            params.put("thingGroupName", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListJobsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListJobsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Lists OTA updates.</p>"]
    fn list_ota_updates(
        &self,
        input: &ListOTAUpdatesRequest,
    ) -> Result<ListOTAUpdatesResponse, ListOTAUpdatesError> {
        let request_uri = "/otaUpdates";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.ota_update_status {
            params.put("otaUpdateStatus", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListOTAUpdatesResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListOTAUpdatesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Lists certificates that are being transferred but not yet accepted.</p>"]
    fn list_outgoing_certificates(
        &self,
        input: &ListOutgoingCertificatesRequest,
    ) -> Result<ListOutgoingCertificatesResponse, ListOutgoingCertificatesError> {
        let request_uri = "/certificates-out-going";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.ascending_order {
            params.put("isAscendingOrder", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("pageSize", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<ListOutgoingCertificatesResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListOutgoingCertificatesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Lists your policies.</p>"]
    fn list_policies(
        &self,
        input: &ListPoliciesRequest,
    ) -> Result<ListPoliciesResponse, ListPoliciesError> {
        let request_uri = "/policies";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.ascending_order {
            params.put("isAscendingOrder", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("pageSize", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListPoliciesResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListPoliciesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Lists the principals associated with the specified policy.</p> <p> <b>Note:</b> This API is deprecated. Please use <a>ListTargetsForPolicy</a> instead.</p>"]
    fn list_policy_principals(
        &self,
        input: &ListPolicyPrincipalsRequest,
    ) -> Result<ListPolicyPrincipalsResponse, ListPolicyPrincipalsError> {
        let request_uri = "/policy-principals";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.add_header("x-amzn-iot-policy", &input.policy_name);
        let mut params = Params::new();
        if let Some(ref x) = input.ascending_order {
            params.put("isAscendingOrder", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("pageSize", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListPolicyPrincipalsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListPolicyPrincipalsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Lists the versions of the specified policy and identifies the default version.</p>"]
    fn list_policy_versions(
        &self,
        input: &ListPolicyVersionsRequest,
    ) -> Result<ListPolicyVersionsResponse, ListPolicyVersionsError> {
        let request_uri = format!(
            "/policies/{policy_name}/version",
            policy_name = input.policy_name
        );

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListPolicyVersionsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListPolicyVersionsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Lists the policies attached to the specified principal. If you use an Cognito identity, the ID must be in <a href=\"http://docs.aws.amazon.com/cognitoidentity/latest/APIReference/API_GetCredentialsForIdentity.html#API_GetCredentialsForIdentity_RequestSyntax\">AmazonCognito Identity format</a>.</p> <p> <b>Note:</b> This API is deprecated. Please use <a>ListAttachedPolicies</a> instead.</p>"]
    fn list_principal_policies(
        &self,
        input: &ListPrincipalPoliciesRequest,
    ) -> Result<ListPrincipalPoliciesResponse, ListPrincipalPoliciesError> {
        let request_uri = "/principal-policies";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.add_header("x-amzn-iot-principal", &input.principal);
        let mut params = Params::new();
        if let Some(ref x) = input.ascending_order {
            params.put("isAscendingOrder", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("pageSize", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<ListPrincipalPoliciesResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListPrincipalPoliciesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Lists the things associated with the specified principal.</p>"]
    fn list_principal_things(
        &self,
        input: &ListPrincipalThingsRequest,
    ) -> Result<ListPrincipalThingsResponse, ListPrincipalThingsError> {
        let request_uri = "/principals/things";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.add_header("x-amzn-principal", &input.principal);
        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListPrincipalThingsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListPrincipalThingsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Lists the role aliases registered in your account.</p>"]
    fn list_role_aliases(
        &self,
        input: &ListRoleAliasesRequest,
    ) -> Result<ListRoleAliasesResponse, ListRoleAliasesError> {
        let request_uri = "/role-aliases";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.ascending_order {
            params.put("isAscendingOrder", x);
        }
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("pageSize", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListRoleAliasesResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListRoleAliasesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Lists all of the streams in your AWS account.</p>"]
    fn list_streams(
        &self,
        input: &ListStreamsRequest,
    ) -> Result<ListStreamsResponse, ListStreamsError> {
        let request_uri = "/streams";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.ascending_order {
            params.put("isAscendingOrder", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListStreamsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListStreamsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>List targets for the specified policy.</p>"]
    fn list_targets_for_policy(
        &self,
        input: &ListTargetsForPolicyRequest,
    ) -> Result<ListTargetsForPolicyResponse, ListTargetsForPolicyError> {
        let request_uri = format!(
            "/policy-targets/{policy_name}",
            policy_name = input.policy_name
        );

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("marker", x);
        }
        if let Some(ref x) = input.page_size {
            params.put("pageSize", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListTargetsForPolicyResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListTargetsForPolicyError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>List the thing groups in your account.</p>"]
    fn list_thing_groups(
        &self,
        input: &ListThingGroupsRequest,
    ) -> Result<ListThingGroupsResponse, ListThingGroupsError> {
        let request_uri = "/thing-groups";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.name_prefix_filter {
            params.put("namePrefixFilter", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.parent_group {
            params.put("parentGroup", x);
        }
        if let Some(ref x) = input.recursive {
            params.put("recursive", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListThingGroupsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListThingGroupsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>List the thing groups to which the specified thing belongs.</p>"]
    fn list_thing_groups_for_thing(
        &self,
        input: &ListThingGroupsForThingRequest,
    ) -> Result<ListThingGroupsForThingResponse, ListThingGroupsForThingError> {
        let request_uri = format!(
            "/things/{thing_name}/thing-groups",
            thing_name = input.thing_name
        );

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<ListThingGroupsForThingResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListThingGroupsForThingError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Lists the principals associated with the specified thing.</p>"]
    fn list_thing_principals(
        &self,
        input: &ListThingPrincipalsRequest,
    ) -> Result<ListThingPrincipalsResponse, ListThingPrincipalsError> {
        let request_uri = format!(
            "/things/{thing_name}/principals",
            thing_name = input.thing_name
        );

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListThingPrincipalsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListThingPrincipalsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Information about the thing registration tasks.</p>"]
    fn list_thing_registration_task_reports(
        &self,
        input: &ListThingRegistrationTaskReportsRequest,
    ) -> Result<ListThingRegistrationTaskReportsResponse, ListThingRegistrationTaskReportsError>
    {
        let request_uri = format!(
            "/thing-registration-tasks/{task_id}/reports",
            task_id = input.task_id
        );

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        params.put("reportType", &input.report_type);
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListThingRegistrationTaskReportsResponse>(
                    &body,
                ).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListThingRegistrationTaskReportsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>List bulk thing provisioning tasks.</p>"]
    fn list_thing_registration_tasks(
        &self,
        input: &ListThingRegistrationTasksRequest,
    ) -> Result<ListThingRegistrationTasksResponse, ListThingRegistrationTasksError> {
        let request_uri = "/thing-registration-tasks";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.status {
            params.put("status", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<ListThingRegistrationTasksResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListThingRegistrationTasksError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Lists the existing thing types.</p>"]
    fn list_thing_types(
        &self,
        input: &ListThingTypesRequest,
    ) -> Result<ListThingTypesResponse, ListThingTypesError> {
        let request_uri = "/thing-types";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.thing_type_name {
            params.put("thingTypeName", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListThingTypesResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListThingTypesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Lists your things. Use the <b>attributeName</b> and <b>attributeValue</b> parameters to filter your things. For example, calling <code>ListThings</code> with attributeName=Color and attributeValue=Red retrieves all things in the registry that contain an attribute <b>Color</b> with the value <b>Red</b>. </p>"]
    fn list_things(
        &self,
        input: &ListThingsRequest,
    ) -> Result<ListThingsResponse, ListThingsError> {
        let request_uri = "/things";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.attribute_name {
            params.put("attributeName", x);
        }
        if let Some(ref x) = input.attribute_value {
            params.put("attributeValue", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.thing_type_name {
            params.put("thingTypeName", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListThingsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListThingsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Lists the things in the specified group.</p>"]
    fn list_things_in_thing_group(
        &self,
        input: &ListThingsInThingGroupRequest,
    ) -> Result<ListThingsInThingGroupResponse, ListThingsInThingGroupError> {
        let request_uri = format!(
            "/thing-groups/{thing_group_name}/things",
            thing_group_name = input.thing_group_name
        );

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.recursive {
            params.put("recursive", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<ListThingsInThingGroupResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListThingsInThingGroupError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Lists the rules for the specific topic.</p>"]
    fn list_topic_rules(
        &self,
        input: &ListTopicRulesRequest,
    ) -> Result<ListTopicRulesResponse, ListTopicRulesError> {
        let request_uri = "/rules";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.rule_disabled {
            params.put("ruleDisabled", x);
        }
        if let Some(ref x) = input.topic {
            params.put("topic", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListTopicRulesResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListTopicRulesError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Lists logging levels.</p>"]
    fn list_v2_logging_levels(
        &self,
        input: &ListV2LoggingLevelsRequest,
    ) -> Result<ListV2LoggingLevelsResponse, ListV2LoggingLevelsError> {
        let request_uri = "/v2LoggingLevel";

        let mut request = SignedRequest::new("GET", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.target_type {
            params.put("targetType", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListV2LoggingLevelsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListV2LoggingLevelsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Registers a CA certificate with AWS IoT. This CA certificate can then be used to sign device certificates, which can be then registered with AWS IoT. You can register up to 10 CA certificates per AWS account that have the same subject field. This enables you to have up to 10 certificate authorities sign your device certificates. If you have more than one CA certificate registered, make sure you pass the CA certificate when you register your device certificates with the RegisterCertificate API.</p>"]
    fn register_ca_certificate(
        &self,
        input: &RegisterCACertificateRequest,
    ) -> Result<RegisterCACertificateResponse, RegisterCACertificateError> {
        let request_uri = "/cacertificate";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.allow_auto_registration {
            params.put("allowAutoRegistration", x);
        }
        if let Some(ref x) = input.set_as_active {
            params.put("setAsActive", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<RegisterCACertificateResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(RegisterCACertificateError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Registers a device certificate with AWS IoT. If you have more than one CA certificate that has the same subject field, you must specify the CA certificate that was used to sign the device certificate being registered.</p>"]
    fn register_certificate(
        &self,
        input: &RegisterCertificateRequest,
    ) -> Result<RegisterCertificateResponse, RegisterCertificateError> {
        let request_uri = "/certificate/register";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<RegisterCertificateResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(RegisterCertificateError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Provisions a thing.</p>"]
    fn register_thing(
        &self,
        input: &RegisterThingRequest,
    ) -> Result<RegisterThingResponse, RegisterThingError> {
        let request_uri = "/things";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<RegisterThingResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(RegisterThingError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Rejects a pending certificate transfer. After AWS IoT rejects a certificate transfer, the certificate status changes from <b>PENDING_TRANSFER</b> to <b>INACTIVE</b>.</p> <p>To check for pending certificate transfers, call <a>ListCertificates</a> to enumerate your certificates.</p> <p>This operation can only be called by the transfer destination. After it is called, the certificate will be returned to the source's account in the INACTIVE state.</p>"]
    fn reject_certificate_transfer(
        &self,
        input: &RejectCertificateTransferRequest,
    ) -> Result<(), RejectCertificateTransferError> {
        let request_uri = format!(
            "/reject-certificate-transfer/{certificate_id}",
            certificate_id = input.certificate_id
        );

        let mut request = SignedRequest::new("PATCH", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(RejectCertificateTransferError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Remove the specified thing from the specified group.</p>"]
    fn remove_thing_from_thing_group(
        &self,
        input: &RemoveThingFromThingGroupRequest,
    ) -> Result<RemoveThingFromThingGroupResponse, RemoveThingFromThingGroupError> {
        let request_uri = "/thing-groups/removeThingFromThingGroup";

        let mut request = SignedRequest::new("PUT", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<RemoveThingFromThingGroupResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(RemoveThingFromThingGroupError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Replaces the rule. You must specify all parameters for the new rule. Creating rules is an administrator-level action. Any user who has permission to create rules will be able to access data processed by the rule.</p>"]
    fn replace_topic_rule(
        &self,
        input: &ReplaceTopicRuleRequest,
    ) -> Result<(), ReplaceTopicRuleError> {
        let request_uri = format!("/rules/{rule_name}", rule_name = input.rule_name);

        let mut request = SignedRequest::new("PATCH", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(&input.topic_rule_payload).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ReplaceTopicRuleError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>The query search index.</p>"]
    fn search_index(
        &self,
        input: &SearchIndexRequest,
    ) -> Result<SearchIndexResponse, SearchIndexError> {
        let request_uri = "/indices/search";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<SearchIndexResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(SearchIndexError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Sets the default authorizer. This will be used if a websocket connection is made without specifying an authorizer.</p>"]
    fn set_default_authorizer(
        &self,
        input: &SetDefaultAuthorizerRequest,
    ) -> Result<SetDefaultAuthorizerResponse, SetDefaultAuthorizerError> {
        let request_uri = "/default-authorizer";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<SetDefaultAuthorizerResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(SetDefaultAuthorizerError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Sets the specified version of the specified policy as the policy's default (operative) version. This action affects all certificates to which the policy is attached. To list the principals the policy is attached to, use the ListPrincipalPolicy API.</p>"]
    fn set_default_policy_version(
        &self,
        input: &SetDefaultPolicyVersionRequest,
    ) -> Result<(), SetDefaultPolicyVersionError> {
        let request_uri = format!(
            "/policies/{policy_name}/version/{policy_version_id}",
            policy_name = input.policy_name,
            policy_version_id = input.policy_version_id
        );

        let mut request = SignedRequest::new("PATCH", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(SetDefaultPolicyVersionError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Sets the logging options.</p>"]
    fn set_logging_options(
        &self,
        input: &SetLoggingOptionsRequest,
    ) -> Result<(), SetLoggingOptionsError> {
        let request_uri = "/loggingOptions";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(&input.logging_options_payload).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(SetLoggingOptionsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Sets the logging level.</p>"]
    fn set_v2_logging_level(
        &self,
        input: &SetV2LoggingLevelRequest,
    ) -> Result<(), SetV2LoggingLevelError> {
        let request_uri = "/v2LoggingLevel";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(SetV2LoggingLevelError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Sets the logging options for the V2 logging service.</p>"]
    fn set_v2_logging_options(
        &self,
        input: &SetV2LoggingOptionsRequest,
    ) -> Result<(), SetV2LoggingOptionsError> {
        let request_uri = "/v2LoggingOptions";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(SetV2LoggingOptionsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Creates a bulk thing provisioning task.</p>"]
    fn start_thing_registration_task(
        &self,
        input: &StartThingRegistrationTaskRequest,
    ) -> Result<StartThingRegistrationTaskResponse, StartThingRegistrationTaskError> {
        let request_uri = "/thing-registration-tasks";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<StartThingRegistrationTaskResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StartThingRegistrationTaskError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Cancels a bulk thing provisioning task.</p>"]
    fn stop_thing_registration_task(
        &self,
        input: &StopThingRegistrationTaskRequest,
    ) -> Result<StopThingRegistrationTaskResponse, StopThingRegistrationTaskError> {
        let request_uri = format!(
            "/thing-registration-tasks/{task_id}/cancel",
            task_id = input.task_id
        );

        let mut request = SignedRequest::new("PUT", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<StopThingRegistrationTaskResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(StopThingRegistrationTaskError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Test custom authorization.</p>"]
    fn test_authorization(
        &self,
        input: &TestAuthorizationRequest,
    ) -> Result<TestAuthorizationResponse, TestAuthorizationError> {
        let request_uri = "/test-authorization";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.client_id {
            params.put("clientId", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<TestAuthorizationResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(TestAuthorizationError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Invoke the specified custom authorizer for testing purposes.</p>"]
    fn test_invoke_authorizer(
        &self,
        input: &TestInvokeAuthorizerRequest,
    ) -> Result<TestInvokeAuthorizerResponse, TestInvokeAuthorizerError> {
        let request_uri = format!(
            "/authorizer/{authorizer_name}/test",
            authorizer_name = input.authorizer_name
        );

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<TestInvokeAuthorizerResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(TestInvokeAuthorizerError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Transfers the specified certificate to the specified AWS account.</p> <p>You can cancel the transfer until it is acknowledged by the recipient.</p> <p>No notification is sent to the transfer destination's account. It is up to the caller to notify the transfer target.</p> <p>The certificate being transferred must not be in the ACTIVE state. You can use the UpdateCertificate API to deactivate it.</p> <p>The certificate must not have any policies attached to it. You can use the DetachPrincipalPolicy API to detach them.</p>"]
    fn transfer_certificate(
        &self,
        input: &TransferCertificateRequest,
    ) -> Result<TransferCertificateResponse, TransferCertificateError> {
        let request_uri = format!(
            "/transfer-certificate/{certificate_id}",
            certificate_id = input.certificate_id
        );

        let mut request = SignedRequest::new("PATCH", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("targetAwsAccount", &input.target_aws_account);
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<TransferCertificateResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(TransferCertificateError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Updates an authorizer.</p>"]
    fn update_authorizer(
        &self,
        input: &UpdateAuthorizerRequest,
    ) -> Result<UpdateAuthorizerResponse, UpdateAuthorizerError> {
        let request_uri = format!(
            "/authorizer/{authorizer_name}",
            authorizer_name = input.authorizer_name
        );

        let mut request = SignedRequest::new("PUT", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateAuthorizerResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateAuthorizerError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Updates a registered CA certificate.</p>"]
    fn update_ca_certificate(
        &self,
        input: &UpdateCACertificateRequest,
    ) -> Result<(), UpdateCACertificateError> {
        let request_uri = format!(
            "/cacertificate/{ca_certificate_id}",
            ca_certificate_id = input.certificate_id
        );

        let mut request = SignedRequest::new("PUT", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.new_auto_registration_status {
            params.put("newAutoRegistrationStatus", x);
        }
        if let Some(ref x) = input.new_status {
            params.put("newStatus", x);
        }
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateCACertificateError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc="<p>Updates the status of the specified certificate. This operation is idempotent.</p> <p>Moving a certificate from the ACTIVE state (including REVOKED) will not disconnect currently connected devices, but these devices will be unable to reconnect.</p> <p>The ACTIVE state is required to authenticate devices connecting to AWS IoT using a certificate.</p>"]
    fn update_certificate(
        &self,
        input: &UpdateCertificateRequest,
    ) -> Result<(), UpdateCertificateError> {
        let request_uri = format!(
            "/certificates/{certificate_id}",
            certificate_id = input.certificate_id
        );

        let mut request = SignedRequest::new("PUT", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());

        let mut params = Params::new();
        params.put("newStatus", &input.new_status);
        request.set_params(params);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let result = ();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateCertificateError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Updates the event configurations.</p>"]
    fn update_event_configurations(
        &self,
        input: &UpdateEventConfigurationsRequest,
    ) -> Result<UpdateEventConfigurationsResponse, UpdateEventConfigurationsError> {
        let request_uri = "/event-configurations";

        let mut request = SignedRequest::new("PATCH", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<UpdateEventConfigurationsResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateEventConfigurationsError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Updates the search configuration.</p>"]
    fn update_indexing_configuration(
        &self,
        input: &UpdateIndexingConfigurationRequest,
    ) -> Result<UpdateIndexingConfigurationResponse, UpdateIndexingConfigurationError> {
        let request_uri = "/indexing/config";

        let mut request = SignedRequest::new("POST", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<UpdateIndexingConfigurationResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateIndexingConfigurationError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Updates a role alias.</p>"]
    fn update_role_alias(
        &self,
        input: &UpdateRoleAliasRequest,
    ) -> Result<UpdateRoleAliasResponse, UpdateRoleAliasError> {
        let request_uri = format!("/role-aliases/{role_alias}", role_alias = input.role_alias);

        let mut request = SignedRequest::new("PUT", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateRoleAliasResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateRoleAliasError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Updates an existing stream. The stream version will be incremented by one.</p>"]
    fn update_stream(
        &self,
        input: &UpdateStreamRequest,
    ) -> Result<UpdateStreamResponse, UpdateStreamError> {
        let request_uri = format!("/streams/{stream_id}", stream_id = input.stream_id);

        let mut request = SignedRequest::new("PUT", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateStreamResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateStreamError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Updates the data for a thing.</p>"]
    fn update_thing(
        &self,
        input: &UpdateThingRequest,
    ) -> Result<UpdateThingResponse, UpdateThingError> {
        let request_uri = format!("/things/{thing_name}", thing_name = input.thing_name);

        let mut request = SignedRequest::new("PATCH", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateThingResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateThingError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Update a thing group.</p>"]
    fn update_thing_group(
        &self,
        input: &UpdateThingGroupRequest,
    ) -> Result<UpdateThingGroupResponse, UpdateThingGroupError> {
        let request_uri = format!(
            "/thing-groups/{thing_group_name}",
            thing_group_name = input.thing_group_name
        );

        let mut request = SignedRequest::new("PATCH", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateThingGroupResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateThingGroupError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }

    #[doc = "<p>Updates the groups to which the thing belongs.</p>"]
    fn update_thing_groups_for_thing(
        &self,
        input: &UpdateThingGroupsForThingRequest,
    ) -> Result<UpdateThingGroupsForThingResponse, UpdateThingGroupsForThingError> {
        let request_uri = "/thing-groups/updateThingGroupsForThing";

        let mut request = SignedRequest::new("PUT", "execute-api", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.set_endpoint_prefix("iot".to_string());
        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result =
                    serde_json::from_slice::<UpdateThingGroupsForThingResponse>(&body).unwrap();

                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateThingGroupsForThingError::from_body(
                    String::from_utf8_lossy(&body).as_ref(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
