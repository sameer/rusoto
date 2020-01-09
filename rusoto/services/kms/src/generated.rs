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
#![allow(warnings)]

use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};
use std::error::Error;
use std::fmt;

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p>Contains information about an alias.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AliasListEntry {
    /// <p>String that contains the key ARN.</p>
    #[serde(rename = "AliasArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_arn: Option<String>,
    /// <p>String that contains the alias. This value begins with <code>alias/</code>.</p>
    #[serde(rename = "AliasName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    /// <p>String that contains the key identifier referred to by the alias.</p>
    #[serde(rename = "TargetKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_key_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelKeyDeletionRequest {
    /// <p>The unique identifier for the customer master key (CMK) for which to cancel deletion.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelKeyDeletionResponse {
    /// <p>The unique identifier of the master key for which deletion is canceled.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ConnectCustomKeyStoreRequest {
    /// <p>Enter the key store ID of the custom key store that you want to connect. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p>
    #[serde(rename = "CustomKeyStoreId")]
    pub custom_key_store_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConnectCustomKeyStoreResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAliasRequest {
    /// <p>Specifies the alias name. This value must begin with <code>alias/</code> followed by a name, such as <code>alias/ExampleAlias</code>. The alias name cannot begin with <code>alias/aws/</code>. The <code>alias/aws/</code> prefix is reserved for AWS managed CMKs.</p>
    #[serde(rename = "AliasName")]
    pub alias_name: String,
    /// <p>Identifies the CMK to which the alias refers. Specify the key ID or the Amazon Resource Name (ARN) of the CMK. You cannot specify another alias. For help finding the key ID and ARN, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/viewing-keys.html#find-cmk-id-arn">Finding the Key ID and ARN</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "TargetKeyId")]
    pub target_key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCustomKeyStoreRequest {
    /// <p>Identifies the AWS CloudHSM cluster for the custom key store. Enter the cluster ID of any active AWS CloudHSM cluster that is not already associated with a custom key store. To find the cluster ID, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation.</p>
    #[serde(rename = "CloudHsmClusterId")]
    pub cloud_hsm_cluster_id: String,
    /// <p>Specifies a friendly name for the custom key store. The name must be unique in your AWS account.</p>
    #[serde(rename = "CustomKeyStoreName")]
    pub custom_key_store_name: String,
    /// <p>Enter the password of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-concepts.html#concept-kmsuser"> <code>kmsuser</code> crypto user (CU) account</a> in the specified AWS CloudHSM cluster. AWS KMS logs into the cluster as this user to manage key material on your behalf.</p> <p>This parameter tells AWS KMS the <code>kmsuser</code> account password; it does not change the password in the AWS CloudHSM cluster.</p>
    #[serde(rename = "KeyStorePassword")]
    pub key_store_password: String,
    /// <p>Enter the content of the trust anchor certificate for the cluster. This is the content of the <code>customerCA.crt</code> file that you created when you <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/initialize-cluster.html">initialized the cluster</a>.</p>
    #[serde(rename = "TrustAnchorCertificate")]
    pub trust_anchor_certificate: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCustomKeyStoreResponse {
    /// <p>A unique identifier for the new custom key store.</p>
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateGrantRequest {
    /// <p>Allows a cryptographic operation only when the encryption context matches or includes the encryption context specified in this structure. For more information about encryption context, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    #[serde(rename = "Constraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<GrantConstraints>,
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>The principal that is given permission to perform the operations that the grant permits.</p> <p>To specify the principal, use the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an AWS principal. Valid AWS principals include AWS accounts (root), IAM users, IAM roles, federated users, and assumed role users. For examples of the ARN syntax to use for specifying a principal, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">AWS Identity and Access Management (IAM)</a> in the Example ARNs section of the <i>AWS General Reference</i>.</p>
    #[serde(rename = "GranteePrincipal")]
    pub grantee_principal: String,
    /// <p>The unique identifier for the customer master key (CMK) that the grant applies to.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK. To specify a CMK in a different AWS account, you must use the key ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>A friendly name for identifying the grant. Use this value to prevent the unintended creation of duplicate grants when retrying this request.</p> <p>When this value is absent, all <code>CreateGrant</code> requests result in a new grant with a unique <code>GrantId</code> even if all the supplied parameters are identical. This can result in unintended duplicates when you retry the <code>CreateGrant</code> request.</p> <p>When this value is present, you can retry a <code>CreateGrant</code> request with identical parameters; if the grant already exists, the original <code>GrantId</code> is returned without creating a new grant. Note that the returned grant token is unique with every <code>CreateGrant</code> request, even when a duplicate <code>GrantId</code> is returned. All grant tokens obtained in this way can be used interchangeably.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of operations that the grant permits.</p>
    #[serde(rename = "Operations")]
    pub operations: Vec<String>,
    /// <p>The principal that is given permission to retire the grant by using <a>RetireGrant</a> operation.</p> <p>To specify the principal, use the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an AWS principal. Valid AWS principals include AWS accounts (root), IAM users, federated users, and assumed role users. For examples of the ARN syntax to use for specifying a principal, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">AWS Identity and Access Management (IAM)</a> in the Example ARNs section of the <i>AWS General Reference</i>.</p>
    #[serde(rename = "RetiringPrincipal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retiring_principal: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateGrantResponse {
    /// <p>The unique identifier for the grant.</p> <p>You can use the <code>GrantId</code> in a subsequent <a>RetireGrant</a> or <a>RevokeGrant</a> operation.</p>
    #[serde(rename = "GrantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<String>,
    /// <p>The grant token.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateKeyRequest {
    /// <p>A flag to indicate whether to bypass the key policy lockout safety check.</p> <important> <p>Setting this value to true increases the risk that the CMK becomes unmanageable. Do not set this value to true indiscriminately.</p> <p>For more information, refer to the scenario in the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam">Default Key Policy</a> section in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> </important> <p>Use this parameter only when you include a policy in the request and you intend to prevent the principal that is making the request from making a subsequent <a>PutKeyPolicy</a> request on the CMK.</p> <p>The default value is false.</p>
    #[serde(rename = "BypassPolicyLockoutSafetyCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_policy_lockout_safety_check: Option<bool>,
    /// <p>Creates the CMK in the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> and the key material in its associated AWS CloudHSM cluster. To create a CMK in a custom key store, you must also specify the <code>Origin</code> parameter with a value of <code>AWS_CLOUDHSM</code>. The AWS CloudHSM cluster that is associated with the custom key store must have at least two active HSMs, each in a different Availability Zone in the Region.</p> <p>This parameter is valid only for symmetric CMKs. You cannot create an asymmetric CMK in a custom key store.</p> <p>To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>The response includes the custom key store ID and the ID of the AWS CloudHSM cluster.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p>
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
    /// <p><p>Specifies the type of CMK to create. The <code>CustomerMasterKeySpec</code> determines whether the CMK contains a symmetric key or an asymmetric key pair. It also determines the encryption algorithms or signing algorithms that the CMK supports. You can&#39;t change the <code>CustomerMasterKeySpec</code> after the CMK is created. To further restrict the algorithms that can be used with the CMK, use its key policy or IAM policy.</p> <p>For help with choosing a key spec for your CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html#cmk-key-spec">Selecting a Customer Master Key Spec</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The default value, <code>SYMMETRIC<em>DEFAULT</code>, creates a CMK with a 256-bit symmetric key.</p> <p>AWS KMS supports the following key specs for CMKs:</p> <ul> <li> <p>Symmetric key (default)</p> <ul> <li> <p> <code>SYMMETRIC</em>DEFAULT</code> (AES-256-GCM)</p> </li> </ul> </li> <li> <p>Asymmetric RSA key pairs</p> <ul> <li> <p> <code>RSA<em>2048</code> </p> </li> <li> <p> <code>RSA</em>3072</code> </p> </li> <li> <p> <code>RSA<em>4096</code> </p> </li> </ul> </li> <li> <p>Asymmetric NIST-recommended elliptic curve key pairs</p> <ul> <li> <p> <code>ECC</em>NIST<em>P256</code> (secp256r1)</p> </li> <li> <p> <code>ECC</em>NIST<em>P384</code> (secp384r1)</p> </li> <li> <p> <code>ECC</em>NIST<em>P521</code> (secp521r1)</p> </li> </ul> </li> <li> <p>Other asymmetric elliptic curve key pairs</p> <ul> <li> <p> <code>ECC</em>SECG_P256K1</code> (secp256k1), commonly used for cryptocurrencies.</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "CustomerMasterKeySpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_master_key_spec: Option<String>,
    /// <p>A description of the CMK.</p> <p>Use a description that helps you decide whether the CMK is appropriate for a task.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p><p>Determines the cryptographic operations for which you can use the CMK. The default value is <code>ENCRYPT<em>DECRYPT</code>. This parameter is required only for asymmetric CMKs. You can&#39;t change the <code>KeyUsage</code> value after the CMK is created.</p> <p>Select only one valid value.</p> <ul> <li> <p>For symmetric CMKs, omit the parameter or specify <code>ENCRYPT</em>DECRYPT</code>.</p> </li> <li> <p>For asymmetric CMKs with RSA key material, specify <code>ENCRYPT<em>DECRYPT</code> or <code>SIGN</em>VERIFY</code>.</p> </li> <li> <p>For asymmetric CMKs with ECC key material, specify <code>SIGN_VERIFY</code>.</p> </li> </ul></p>
    #[serde(rename = "KeyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<String>,
    /// <p>The source of the key material for the CMK. You cannot change the origin after you create the CMK. The default is <code>AWS_KMS</code>, which means AWS KMS creates the key material.</p> <p>When the parameter value is <code>EXTERNAL</code>, AWS KMS creates a CMK without key material so that you can import key material from your existing key management infrastructure. For more information about importing key material into AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>. This value is valid only for symmetric CMKs.</p> <p>When the parameter value is <code>AWS_CLOUDHSM</code>, AWS KMS creates the CMK in an AWS KMS <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> and creates its key material in the associated AWS CloudHSM cluster. You must also use the <code>CustomKeyStoreId</code> parameter to identify the custom key store. This value is valid only for symmetric CMKs.</p>
    #[serde(rename = "Origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// <p>The key policy to attach to the CMK.</p> <p>If you provide a key policy, it must meet the following criteria:</p> <ul> <li> <p>If you don't set <code>BypassPolicyLockoutSafetyCheck</code> to true, the key policy must allow the principal that is making the <code>CreateKey</code> request to make a subsequent <a>PutKeyPolicy</a> request on the CMK. This reduces the risk that the CMK becomes unmanageable. For more information, refer to the scenario in the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam">Default Key Policy</a> section of the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> </li> <li> <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to AWS KMS. When you create a new AWS principal (for example, an IAM user or role), you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to AWS KMS. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>AWS Identity and Access Management User Guide</i>.</p> </li> </ul> <p>If you do not provide a key policy, AWS KMS attaches a default key policy to the CMK. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default">Default Key Policy</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The key policy size limit is 32 kilobytes (32768 bytes).</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>One or more tags. Each tag consists of a tag key and a tag value. Both the tag key and the tag value are required, but the tag value can be an empty (null) string.</p> <p>When you add tags to an AWS resource, AWS generates a cost allocation report with usage and costs aggregated by tags. For information about adding, changing, deleting and listing tags for CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">Tagging Keys</a>.</p> <p>Use this parameter to tag the CMK when it is created. To add tags to an existing CMK, use the <a>TagResource</a> operation.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateKeyResponse {
    /// <p>Metadata associated with the CMK.</p>
    #[serde(rename = "KeyMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_metadata: Option<KeyMetadata>,
}

/// <p>Contains information about each custom key store in the custom key store list.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CustomKeyStoresListEntry {
    /// <p>A unique identifier for the AWS CloudHSM cluster that is associated with the custom key store.</p>
    #[serde(rename = "CloudHsmClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_hsm_cluster_id: Option<String>,
    /// <p>Describes the connection error. Valid values are:</p> <ul> <li> <p> <code>CLUSTER_NOT_FOUND</code> - AWS KMS cannot find the AWS CloudHSM cluster with the specified cluster ID.</p> </li> <li> <p> <code>INSUFFICIENT_CLOUDHSM_HSMS</code> - The associated AWS CloudHSM cluster does not contain any active HSMs. To connect a custom key store to its AWS CloudHSM cluster, the cluster must contain at least one active HSM.</p> </li> <li> <p> <code>INTERNAL_ERROR</code> - AWS KMS could not complete the request due to an internal error. Retry the request. For <code>ConnectCustomKeyStore</code> requests, disconnect the custom key store before trying to connect again.</p> </li> <li> <p> <code>INVALID_CREDENTIALS</code> - AWS KMS does not have the correct password for the <code>kmsuser</code> crypto user in the AWS CloudHSM cluster.</p> </li> <li> <p> <code>NETWORK_ERRORS</code> - Network errors are preventing AWS KMS from connecting to the custom key store.</p> </li> <li> <p> <code>USER_LOCKED_OUT</code> - The <code>kmsuser</code> CU account is locked out of the associated AWS CloudHSM cluster due to too many failed password attempts. Before you can connect your custom key store to its AWS CloudHSM cluster, you must change the <code>kmsuser</code> account password and update the password value for the custom key store.</p> </li> </ul> <p>For help with connection failures, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting Custom Key Stores</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "ConnectionErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_error_code: Option<String>,
    /// <p>Indicates whether the custom key store is connected to its AWS CloudHSM cluster.</p> <p>You can create and use CMKs in your custom key stores only when its connection state is <code>CONNECTED</code>.</p> <p>The value is <code>DISCONNECTED</code> if the key store has never been connected or you use the <a>DisconnectCustomKeyStore</a> operation to disconnect it. If the value is <code>CONNECTED</code> but you are having trouble using the custom key store, make sure that its associated AWS CloudHSM cluster is active and contains at least one active HSM.</p> <p>A value of <code>FAILED</code> indicates that an attempt to connect was unsuccessful. For help resolving a connection failure, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting a Custom Key Store</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "ConnectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    /// <p>The date and time when the custom key store was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>A unique identifier for the custom key store.</p>
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
    /// <p>The user-specified friendly name for the custom key store.</p>
    #[serde(rename = "CustomKeyStoreName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_name: Option<String>,
    /// <p>The trust anchor certificate of the associated AWS CloudHSM cluster. When you <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/initialize-cluster.html#sign-csr">initialize the cluster</a>, you create this certificate and save it in the <code>customerCA.crt</code> file.</p>
    #[serde(rename = "TrustAnchorCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_anchor_certificate: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DecryptRequest {
    /// <p>Ciphertext to be decrypted. The blob includes metadata.</p>
    #[serde(rename = "CiphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub ciphertext_blob: bytes::Bytes,
    /// <p>Specifies the encryption algorithm that will be used to decrypt the ciphertext. Specify the same algorithm that was used to encrypt the data. If you specify a different algorithm, the <code>Decrypt</code> operation fails.</p> <p>This parameter is required only when the ciphertext was encrypted under an asymmetric CMK. The default value, <code>SYMMETRIC_DEFAULT</code>, represents the only supported algorithm that is valid for symmetric CMKs.</p>
    #[serde(rename = "EncryptionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<String>,
    /// <p>Specifies the encryption context to use when decrypting the data. An encryption context is valid only for cryptographic operations with a symmetric CMK. The standard asymmetric encryption algorithms that AWS KMS uses do not support an encryption context.</p> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represents additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is optional when encrypting with a symmetric CMK, but it is highly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "EncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Specifies the customer master key (CMK) that AWS KMS will use to decrypt the ciphertext. Enter a key ID of the CMK that was used to encrypt the ciphertext.</p> <p>If you specify a <code>KeyId</code> value, the <code>Decrypt</code> operation succeeds only if the specified CMK was used to encrypt the ciphertext.</p> <p>This parameter is required only when the ciphertext was encrypted under an asymmetric CMK. Otherwise, AWS KMS uses the metadata that it adds to the ciphertext blob to determine which CMK was used to encrypt the ciphertext. However, you can use this parameter to ensure that a particular CMK (of any kind) is used to decrypt the ciphertext.</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DecryptResponse {
    /// <p>The encryption algorithm that was used to decrypt the ciphertext.</p>
    #[serde(rename = "EncryptionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<String>,
    /// <p>The ARN of the customer master key that was used to perform the decryption.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>Decrypted plaintext data. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    #[serde(rename = "Plaintext")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<bytes::Bytes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAliasRequest {
    /// <p>The alias to be deleted. The alias name must begin with <code>alias/</code> followed by the alias name, such as <code>alias/ExampleAlias</code>.</p>
    #[serde(rename = "AliasName")]
    pub alias_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteCustomKeyStoreRequest {
    /// <p>Enter the ID of the custom key store you want to delete. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p>
    #[serde(rename = "CustomKeyStoreId")]
    pub custom_key_store_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteCustomKeyStoreResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteImportedKeyMaterialRequest {
    /// <p>Identifies the CMK from which you are deleting imported key material. The <code>Origin</code> of the CMK must be <code>EXTERNAL</code>.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeCustomKeyStoresRequest {
    /// <p>Gets only information about the specified custom key store. Enter the key store ID.</p> <p>By default, this operation gets information about all custom key stores in the account and region. To limit the output to a particular custom key store, you can use either the <code>CustomKeyStoreId</code> or <code>CustomKeyStoreName</code> parameter, but not both.</p>
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
    /// <p>Gets only information about the specified custom key store. Enter the friendly name of the custom key store.</p> <p>By default, this operation gets information about all custom key stores in the account and region. To limit the output to a particular custom key store, you can use either the <code>CustomKeyStoreId</code> or <code>CustomKeyStoreName</code> parameter, but not both.</p>
    #[serde(rename = "CustomKeyStoreName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_name: Option<String>,
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeCustomKeyStoresResponse {
    /// <p>Contains metadata about each custom key store.</p>
    #[serde(rename = "CustomKeyStores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_stores: Option<Vec<CustomKeyStoresListEntry>>,
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "Truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeKeyRequest {
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Describes the specified customer master key (CMK). </p> <p>If you specify a predefined AWS alias (an AWS alias with no key ID), KMS associates the alias with an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">AWS managed CMK</a> and returns its <code>KeyId</code> and <code>Arn</code> in the response.</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeKeyResponse {
    /// <p>Metadata associated with the key.</p>
    #[serde(rename = "KeyMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_metadata: Option<KeyMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisableKeyRequest {
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisableKeyRotationRequest {
    /// <p>Identifies a symmetric customer master key (CMK). You cannot enable automatic rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html#asymmetric-cmks">asymmetric CMKs</a>, CMKs with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or CMKs in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisconnectCustomKeyStoreRequest {
    /// <p>Enter the ID of the custom key store you want to disconnect. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p>
    #[serde(rename = "CustomKeyStoreId")]
    pub custom_key_store_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisconnectCustomKeyStoreResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EnableKeyRequest {
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EnableKeyRotationRequest {
    /// <p>Identifies a symmetric customer master key (CMK). You cannot enable automatic rotation of asymmetric CMKs, CMKs with imported key material, or CMKs in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct EncryptRequest {
    /// <p>Specifies the encryption algorithm that AWS KMS will use to encrypt the plaintext message. The algorithm must be compatible with the CMK that you specify.</p> <p>This parameter is required only for asymmetric CMKs. The default value, <code>SYMMETRIC_DEFAULT</code>, is the algorithm used for symmetric CMKs. If you are using an asymmetric CMK, we recommend RSAES_OAEP_SHA_256.</p>
    #[serde(rename = "EncryptionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<String>,
    /// <p>Specifies the encryption context that will be used to encrypt the data. An encryption context is valid only for cryptographic operations with a symmetric CMK. The standard asymmetric encryption algorithms that AWS KMS uses do not support an encryption context. </p> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represents additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is optional when encrypting with a symmetric CMK, but it is highly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "EncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>Data to be encrypted.</p>
    #[serde(rename = "Plaintext")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub plaintext: bytes::Bytes,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EncryptResponse {
    /// <p>The encrypted plaintext. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    #[serde(rename = "CiphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<bytes::Bytes>,
    /// <p>The encryption algorithm that was used to encrypt the plaintext.</p>
    #[serde(rename = "EncryptionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<String>,
    /// <p>The ID of the key used during encryption.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GenerateDataKeyPairRequest {
    /// <p>Specifies the encryption context that will be used when encrypting the private key in the data key pair.</p> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represents additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is optional when encrypting with a symmetric CMK, but it is highly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "EncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Specifies the symmetric CMK that encrypts the private key in the data key pair. You cannot specify an asymmetric CMKs.</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>Determines the type of data key pair that is generated. </p> <p>The AWS KMS rule that restricts the use of asymmetric RSA CMKs to encrypt and decrypt or to sign and verify (but not both), and the rule that permits you to use ECC CMKs only to sign and verify, are not effective outside of AWS KMS.</p>
    #[serde(rename = "KeyPairSpec")]
    pub key_pair_spec: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GenerateDataKeyPairResponse {
    /// <p>The identifier of the CMK that encrypted the private key.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The type of data key pair that was generated.</p>
    #[serde(rename = "KeyPairSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair_spec: Option<String>,
    /// <p>The encrypted copy of the private key. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    #[serde(rename = "PrivateKeyCiphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_ciphertext_blob: Option<bytes::Bytes>,
    /// <p>The plaintext copy of the private key. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    #[serde(rename = "PrivateKeyPlaintext")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_plaintext: Option<bytes::Bytes>,
    /// <p>The public key (in plaintext).</p>
    #[serde(rename = "PublicKey")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<bytes::Bytes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GenerateDataKeyPairWithoutPlaintextRequest {
    /// <p>Specifies the encryption context that will be used when encrypting the private key in the data key pair.</p> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represents additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is optional when encrypting with a symmetric CMK, but it is highly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "EncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Specifies the CMK that encrypts the private key in the data key pair. You must specify a symmetric CMK. You cannot use an asymmetric CMK. </p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>Determines the type of data key pair that is generated.</p> <p>The AWS KMS rule that restricts the use of asymmetric RSA CMKs to encrypt and decrypt or to sign and verify (but not both), and the rule that permits you to use ECC CMKs only to sign and verify, are not effective outside of AWS KMS.</p>
    #[serde(rename = "KeyPairSpec")]
    pub key_pair_spec: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GenerateDataKeyPairWithoutPlaintextResponse {
    /// <p>Specifies the CMK that encrypted the private key in the data key pair. You must specify a symmetric CMK. You cannot use an asymmetric CMK. </p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The type of data key pair that was generated.</p>
    #[serde(rename = "KeyPairSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair_spec: Option<String>,
    /// <p>The encrypted copy of the private key. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    #[serde(rename = "PrivateKeyCiphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_ciphertext_blob: Option<bytes::Bytes>,
    /// <p>The public key (in plaintext).</p>
    #[serde(rename = "PublicKey")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<bytes::Bytes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GenerateDataKeyRequest {
    /// <p>Specifies the encryption context that will be used when encrypting the data key.</p> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represents additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is optional when encrypting with a symmetric CMK, but it is highly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "EncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Identifies the symmetric CMK that encrypts the data key.</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>Specifies the length of the data key. Use <code>AES_128</code> to generate a 128-bit symmetric key, or <code>AES_256</code> to generate a 256-bit symmetric key.</p> <p>You must specify either the <code>KeySpec</code> or the <code>NumberOfBytes</code> parameter (but not both) in every <code>GenerateDataKey</code> request.</p>
    #[serde(rename = "KeySpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_spec: Option<String>,
    /// <p>Specifies the length of the data key in bytes. For example, use the value 64 to generate a 512-bit data key (64 bytes is 512 bits). For 128-bit (16-byte) and 256-bit (32-byte) data keys, use the <code>KeySpec</code> parameter.</p> <p>You must specify either the <code>KeySpec</code> or the <code>NumberOfBytes</code> parameter (but not both) in every <code>GenerateDataKey</code> request.</p>
    #[serde(rename = "NumberOfBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bytes: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GenerateDataKeyResponse {
    /// <p>The encrypted copy of the data key. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    #[serde(rename = "CiphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<bytes::Bytes>,
    /// <p>The identifier of the CMK that encrypted the data key.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The plaintext data key. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded. Use this data key to encrypt your data outside of KMS. Then, remove it from memory as soon as possible.</p>
    #[serde(rename = "Plaintext")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<bytes::Bytes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GenerateDataKeyWithoutPlaintextRequest {
    /// <p>Specifies the encryption context that will be used when encrypting the data key.</p> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represents additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is optional when encrypting with a symmetric CMK, but it is highly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "EncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>The identifier of the symmetric customer master key (CMK) that encrypts the data key.</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>The length of the data key. Use <code>AES_128</code> to generate a 128-bit symmetric key, or <code>AES_256</code> to generate a 256-bit symmetric key.</p>
    #[serde(rename = "KeySpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_spec: Option<String>,
    /// <p>The length of the data key in bytes. For example, use the value 64 to generate a 512-bit data key (64 bytes is 512 bits). For common key lengths (128-bit and 256-bit symmetric keys), we recommend that you use the <code>KeySpec</code> field instead of this one.</p>
    #[serde(rename = "NumberOfBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bytes: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GenerateDataKeyWithoutPlaintextResponse {
    /// <p>The encrypted data key. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    #[serde(rename = "CiphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<bytes::Bytes>,
    /// <p>The identifier of the CMK that encrypted the data key.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GenerateRandomRequest {
    /// <p>Generates the random byte string in the AWS CloudHSM cluster that is associated with the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p>
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
    /// <p>The length of the byte string.</p>
    #[serde(rename = "NumberOfBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_bytes: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GenerateRandomResponse {
    /// <p>The random byte string. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    #[serde(rename = "Plaintext")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plaintext: Option<bytes::Bytes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetKeyPolicyRequest {
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>Specifies the name of the key policy. The only valid name is <code>default</code>. To get the names of key policies, use <a>ListKeyPolicies</a>.</p>
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetKeyPolicyResponse {
    /// <p>A key policy document in JSON format.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetKeyRotationStatusRequest {
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK. To specify a CMK in a different AWS account, you must use the key ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetKeyRotationStatusResponse {
    /// <p>A Boolean value that specifies whether key rotation is enabled.</p>
    #[serde(rename = "KeyRotationEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_enabled: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetParametersForImportRequest {
    /// <p>The identifier of the symmetric CMK into which you will import key material. The <code>Origin</code> of the CMK must be <code>EXTERNAL</code>.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>The algorithm you will use to encrypt the key material before importing it with <a>ImportKeyMaterial</a>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys-encrypt-key-material.html">Encrypt the Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "WrappingAlgorithm")]
    pub wrapping_algorithm: String,
    /// <p>The type of wrapping key (public key) to return in the response. Only 2048-bit RSA public keys are supported.</p>
    #[serde(rename = "WrappingKeySpec")]
    pub wrapping_key_spec: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetParametersForImportResponse {
    /// <p>The import token to send in a subsequent <a>ImportKeyMaterial</a> request.</p>
    #[serde(rename = "ImportToken")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_token: Option<bytes::Bytes>,
    /// <p>The identifier of the CMK to use in a subsequent <a>ImportKeyMaterial</a> request. This is the same CMK specified in the <code>GetParametersForImport</code> request.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The time at which the import token and public key are no longer valid. After this time, you cannot use them to make an <a>ImportKeyMaterial</a> request and you must send another <code>GetParametersForImport</code> request to get new ones.</p>
    #[serde(rename = "ParametersValidTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters_valid_to: Option<f64>,
    /// <p>The public key to use to encrypt the key material before importing it with <a>ImportKeyMaterial</a>.</p>
    #[serde(rename = "PublicKey")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<bytes::Bytes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPublicKeyRequest {
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Identifies the asymmetric CMK that includes the public key.</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPublicKeyResponse {
    /// <p>The type of the of the public key that was downloaded.</p>
    #[serde(rename = "CustomerMasterKeySpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_master_key_spec: Option<String>,
    /// <p>The encryption algorithms that AWS KMS supports for this key. </p> <p>This information is critical. If a public key encrypts data outside of AWS KMS by using an unsupported encryption algorithm, the ciphertext cannot be decrypted. </p> <p>This field appears in the response only when the <code>KeyUsage</code> of the public key is <code>ENCRYPT_DECRYPT</code>.</p>
    #[serde(rename = "EncryptionAlgorithms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithms: Option<Vec<String>>,
    /// <p>The identifier of the asymmetric CMK from which the public key was downloaded.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The permitted use of the public key. Valid values are <code>ENCRYPT_DECRYPT</code> or <code>SIGN_VERIFY</code>. </p> <p>This information is critical. If a public key with <code>SIGN_VERIFY</code> key usage encrypts data outside of AWS KMS, the ciphertext cannot be decrypted. </p>
    #[serde(rename = "KeyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<String>,
    /// <p>The exported public key. </p> <p>This value is returned as a binary <a href="https://www.itu.int/ITU-T/studygroups/com17/languages/X.690-0207.pdf">Distinguished Encoding Rules</a> (DER)-encoded object. To decode it, use an ASN.1 parsing tool, such as <a href="https://www.openssl.org/docs/man1.0.2/man1/asn1parse.html">OpenSSL asn1parse</a>.</p>
    #[serde(rename = "PublicKey")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<bytes::Bytes>,
    /// <p>The signing algorithms that AWS KMS supports for this key.</p> <p>This field appears in the response only when the <code>KeyUsage</code> of the public key is <code>SIGN_VERIFY</code>.</p>
    #[serde(rename = "SigningAlgorithms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_algorithms: Option<Vec<String>>,
}

/// <p><p>Use this structure to allow cryptographic operations in the grant only when the operation request includes the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">encryption context</a>.</p> <p>AWS KMS applies the grant constraints only when the grant allows a cryptographic operation that accepts an encryption context as input, such as the following.</p> <ul> <li> <p> <a>Encrypt</a> </p> </li> <li> <p> <a>Decrypt</a> </p> </li> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyWithoutPlaintext</a> </p> </li> <li> <p> <a>ReEncrypt</a> </p> </li> </ul> <p>AWS KMS does not apply the grant constraints to other operations, such as <a>DescribeKey</a> or <a>ScheduleKeyDeletion</a>.</p> <important> <p>In a cryptographic operation, the encryption context in the decryption operation must be an exact, case-sensitive match for the keys and values in the encryption context of the encryption operation. Only the order of the pairs can vary.</p> <p>However, in a grant constraint, the key in each key-value pair is not case sensitive, but the value is case sensitive.</p> <p>To avoid confusion, do not use multiple encryption context pairs that differ only by case. To require a fully case-sensitive encryption context, use the <code>kms:EncryptionContext:</code> and <code>kms:EncryptionContextKeys</code> conditions in an IAM or key policy. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/policy-conditions.html#conditions-kms-encryption-context">kms:EncryptionContext:</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> </important></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GrantConstraints {
    /// <p>A list of key-value pairs that must match the encryption context in the cryptographic operation request. The grant allows the operation only when the encryption context in the request is the same as the encryption context specified in this constraint.</p>
    #[serde(rename = "EncryptionContextEquals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context_equals: Option<::std::collections::HashMap<String, String>>,
    /// <p>A list of key-value pairs that must be included in the encryption context of the cryptographic operation request. The grant allows the cryptographic operation only when the encryption context in the request includes the key-value pairs specified in this constraint, although it can include additional key-value pairs.</p>
    #[serde(rename = "EncryptionContextSubset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context_subset: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Contains information about an entry in a list of grants.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GrantListEntry {
    /// <p>A list of key-value pairs that must be present in the encryption context of certain subsequent operations that the grant allows.</p>
    #[serde(rename = "Constraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<GrantConstraints>,
    /// <p>The date and time when the grant was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>The unique identifier for the grant.</p>
    #[serde(rename = "GrantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<String>,
    /// <p>The principal that receives the grant's permissions.</p>
    #[serde(rename = "GranteePrincipal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee_principal: Option<String>,
    /// <p>The AWS account under which the grant was issued.</p>
    #[serde(rename = "IssuingAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_account: Option<String>,
    /// <p>The unique identifier for the customer master key (CMK) to which the grant applies.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The friendly name that identifies the grant. If a name was provided in the <a>CreateGrant</a> request, that name is returned. Otherwise this value is null.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The list of operations permitted by the grant.</p>
    #[serde(rename = "Operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<String>>,
    /// <p>The principal that can retire the grant.</p>
    #[serde(rename = "RetiringPrincipal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retiring_principal: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportKeyMaterialRequest {
    /// <p>The encrypted key material to import. The key material must be encrypted with the public wrapping key that <a>GetParametersForImport</a> returned, using the wrapping algorithm that you specified in the same <code>GetParametersForImport</code> request.</p>
    #[serde(rename = "EncryptedKeyMaterial")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub encrypted_key_material: bytes::Bytes,
    /// <p>Specifies whether the key material expires. The default is <code>KEY_MATERIAL_EXPIRES</code>, in which case you must include the <code>ValidTo</code> parameter. When this parameter is set to <code>KEY_MATERIAL_DOES_NOT_EXPIRE</code>, you must omit the <code>ValidTo</code> parameter.</p>
    #[serde(rename = "ExpirationModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_model: Option<String>,
    /// <p>The import token that you received in the response to a previous <a>GetParametersForImport</a> request. It must be from the same response that contained the public key that you used to encrypt the key material.</p>
    #[serde(rename = "ImportToken")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub import_token: bytes::Bytes,
    /// <p>The identifier of the symmetric CMK that receives the imported key material. The CMK's <code>Origin</code> must be <code>EXTERNAL</code>. This must be the same CMK specified in the <code>KeyID</code> parameter of the corresponding <a>GetParametersForImport</a> request.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>The time at which the imported key material expires. When the key material expires, AWS KMS deletes the key material and the CMK becomes unusable. You must omit this parameter when the <code>ExpirationModel</code> parameter is set to <code>KEY_MATERIAL_DOES_NOT_EXPIRE</code>. Otherwise it is required.</p>
    #[serde(rename = "ValidTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportKeyMaterialResponse {}

/// <p>Contains information about each entry in the key list.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KeyListEntry {
    /// <p>ARN of the key.</p>
    #[serde(rename = "KeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_arn: Option<String>,
    /// <p>Unique identifier of the key.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

/// <p>Contains metadata about a customer master key (CMK).</p> <p>This data type is used as a response element for the <a>CreateKey</a> and <a>DescribeKey</a> operations.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KeyMetadata {
    /// <p>The twelve-digit account ID of the AWS account that owns the CMK.</p>
    #[serde(rename = "AWSAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the CMK. For examples, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kms">AWS Key Management Service (AWS KMS)</a> in the Example ARNs section of the <i>AWS General Reference</i>.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The cluster ID of the AWS CloudHSM cluster that contains the key material for the CMK. When you create a CMK in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, AWS KMS creates the key material for the CMK in the associated AWS CloudHSM cluster. This value is present only when the CMK is created in a custom key store.</p>
    #[serde(rename = "CloudHsmClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_hsm_cluster_id: Option<String>,
    /// <p>The date and time when the CMK was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>A unique identifier for the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> that contains the CMK. This value is present only when the CMK is created in a custom key store.</p>
    #[serde(rename = "CustomKeyStoreId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key_store_id: Option<String>,
    /// <p>Describes the type of key material in the CMK.</p>
    #[serde(rename = "CustomerMasterKeySpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_master_key_spec: Option<String>,
    /// <p>The date and time after which AWS KMS deletes the CMK. This value is present only when <code>KeyState</code> is <code>PendingDeletion</code>.</p>
    #[serde(rename = "DeletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<f64>,
    /// <p>The description of the CMK.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Specifies whether the CMK is enabled. When <code>KeyState</code> is <code>Enabled</code> this value is true, otherwise it is false.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>A list of encryption algorithms that the CMK supports. You cannot use the CMK with other encryption algorithms within AWS KMS.</p> <p>This field appears only when the <code>KeyUsage</code> of the CMK is <code>ENCRYPT_DECRYPT</code>.</p>
    #[serde(rename = "EncryptionAlgorithms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithms: Option<Vec<String>>,
    /// <p>Specifies whether the CMK's key material expires. This value is present only when <code>Origin</code> is <code>EXTERNAL</code>, otherwise this value is omitted.</p>
    #[serde(rename = "ExpirationModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_model: Option<String>,
    /// <p>The globally unique identifier for the CMK.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>The manager of the CMK. CMKs in your AWS account are either customer managed or AWS managed. For more information about the difference, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "KeyManager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_manager: Option<String>,
    /// <p>The state of the CMK.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects the Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "KeyState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_state: Option<String>,
    /// <p>The cryptographic operations for which you can use the CMK.</p>
    #[serde(rename = "KeyUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<String>,
    /// <p>The source of the CMK's key material. When this value is <code>AWS_KMS</code>, AWS KMS created the key material. When this value is <code>EXTERNAL</code>, the key material was imported from your existing key management infrastructure or the CMK lacks key material. When this value is <code>AWS_CLOUDHSM</code>, the key material was created in the AWS CloudHSM cluster associated with a custom key store.</p>
    #[serde(rename = "Origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// <p>A list of signing algorithms that the CMK supports. You cannot use the CMK with other signing algorithms within AWS KMS.</p> <p>This field appears only when the <code>KeyUsage</code> of the CMK is <code>SIGN_VERIFY</code>.</p>
    #[serde(rename = "SigningAlgorithms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_algorithms: Option<Vec<String>>,
    /// <p>The time at which the imported key material expires. When the key material expires, AWS KMS deletes the key material and the CMK becomes unusable. This value is present only for CMKs whose <code>Origin</code> is <code>EXTERNAL</code> and whose <code>ExpirationModel</code> is <code>KEY_MATERIAL_EXPIRES</code>, otherwise this value is omitted.</p>
    #[serde(rename = "ValidTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAliasesRequest {
    /// <p>Lists only aliases that refer to the specified CMK. The value of this parameter can be the ID or Amazon Resource Name (ARN) of a CMK in the caller's account and region. You cannot use an alias name or alias ARN in this value.</p> <p>This parameter is optional. If you omit it, <code>ListAliases</code> returns all aliases in the account and region.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListAliasesResponse {
    /// <p>A list of aliases.</p>
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<AliasListEntry>>,
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "Truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListGrantsRequest {
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK. To specify a CMK in a different AWS account, you must use the key ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListGrantsResponse {
    /// <p>A list of grants.</p>
    #[serde(rename = "Grants")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grants: Option<Vec<GrantListEntry>>,
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "Truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListKeyPoliciesRequest {
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 1000, inclusive. If you do not include a value, it defaults to 100.</p> <p>Only one policy can be attached to a key.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListKeyPoliciesResponse {
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A list of key policy names. The only valid value is <code>default</code>.</p>
    #[serde(rename = "PolicyNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<Vec<String>>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "Truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListKeysRequest {
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 1000, inclusive. If you do not include a value, it defaults to 100.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListKeysResponse {
    /// <p>A list of customer master keys (CMKs).</p>
    #[serde(rename = "Keys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<KeyListEntry>>,
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "Truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListResourceTagsRequest {
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 50, inclusive. If you do not include a value, it defaults to 50.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p> <p>Do not attempt to construct this value. Use only the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListResourceTagsResponse {
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p> <p>Do not assume or infer any information from this value.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A list of tags. Each tag consists of a tag key and a tag value.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
    #[serde(rename = "Truncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListRetirableGrantsRequest {
    /// <p>Use this parameter to specify the maximum number of items to return. When this value is present, AWS KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The retiring principal for which to list grants.</p> <p>To specify the retiring principal, use the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an AWS principal. Valid AWS principals include AWS accounts (root), IAM users, federated users, and assumed role users. For examples of the ARN syntax for specifying a principal, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">AWS Identity and Access Management (IAM)</a> in the Example ARNs section of the <i>Amazon Web Services General Reference</i>.</p>
    #[serde(rename = "RetiringPrincipal")]
    pub retiring_principal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutKeyPolicyRequest {
    /// <p>A flag to indicate whether to bypass the key policy lockout safety check.</p> <important> <p>Setting this value to true increases the risk that the CMK becomes unmanageable. Do not set this value to true indiscriminately.</p> <p>For more information, refer to the scenario in the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam">Default Key Policy</a> section in the <i>AWS Key Management Service Developer Guide</i>.</p> </important> <p>Use this parameter only when you intend to prevent the principal that is making the request from making a subsequent <code>PutKeyPolicy</code> request on the CMK.</p> <p>The default value is false.</p>
    #[serde(rename = "BypassPolicyLockoutSafetyCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_policy_lockout_safety_check: Option<bool>,
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>The key policy to attach to the CMK.</p> <p>The key policy must meet the following criteria:</p> <ul> <li> <p>If you don't set <code>BypassPolicyLockoutSafetyCheck</code> to true, the key policy must allow the principal that is making the <code>PutKeyPolicy</code> request to make a subsequent <code>PutKeyPolicy</code> request on the CMK. This reduces the risk that the CMK becomes unmanageable. For more information, refer to the scenario in the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default-allow-root-enable-iam">Default Key Policy</a> section of the <i>AWS Key Management Service Developer Guide</i>.</p> </li> <li> <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to AWS KMS. When you create a new AWS principal (for example, an IAM user or role), you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to AWS KMS. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>AWS Identity and Access Management User Guide</i>.</p> </li> </ul> <p>The key policy size limit is 32 kilobytes (32768 bytes).</p>
    #[serde(rename = "Policy")]
    pub policy: String,
    /// <p>The name of the key policy. The only valid value is <code>default</code>.</p>
    #[serde(rename = "PolicyName")]
    pub policy_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ReEncryptRequest {
    /// <p>Ciphertext of the data to reencrypt.</p>
    #[serde(rename = "CiphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub ciphertext_blob: bytes::Bytes,
    /// <p>Specifies the encryption algorithm that AWS KMS will use to reecrypt the data after it has decrypted it. The default value, <code>SYMMETRIC_DEFAULT</code>, represents the encryption algorithm used for symmetric CMKs.</p> <p>This parameter is required only when the destination CMK is an asymmetric CMK.</p>
    #[serde(rename = "DestinationEncryptionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_encryption_algorithm: Option<String>,
    /// <p>Specifies that encryption context to use when the reencrypting the data.</p> <p>A destination encryption context is valid only when the destination CMK is a symmetric CMK. The standard ciphertext format for asymmetric CMKs does not include fields for metadata.</p> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represents additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is optional when encrypting with a symmetric CMK, but it is highly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "DestinationEncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A unique identifier for the CMK that is used to reencrypt the data. Specify a symmetric or asymmetric CMK with a <code>KeyUsage</code> value of <code>ENCRYPT_DECRYPT</code>. To find the <code>KeyUsage</code> value of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "DestinationKeyId")]
    pub destination_key_id: String,
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Specifies the encryption algorithm that AWS KMS will use to decrypt the ciphertext before it is reencrypted. The default value, <code>SYMMETRIC_DEFAULT</code>, represents the algorithm used for symmetric CMKs.</p> <p>Specify the same algorithm that was used to encrypt the ciphertext. If you specify a different algorithm, the decrypt attempt fails.</p> <p>This parameter is required only when the ciphertext was encrypted under an asymmetric CMK.</p>
    #[serde(rename = "SourceEncryptionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_encryption_algorithm: Option<String>,
    /// <p>Specifies the encryption context to use to decrypt the ciphertext. Enter the same encryption context that was used to encrypt the ciphertext.</p> <p>An <i>encryption context</i> is a collection of non-secret key-value pairs that represents additional authenticated data. When you use an encryption context to encrypt data, you must specify the same (an exact case-sensitive match) encryption context to decrypt the data. An encryption context is optional when encrypting with a symmetric CMK, but it is highly recommended.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "SourceEncryptionContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_encryption_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>A unique identifier for the CMK that is used to decrypt the ciphertext before it reencrypts it using the destination CMK.</p> <p>This parameter is required only when the ciphertext was encrypted under an asymmetric CMK. Otherwise, AWS KMS uses the metadata that it adds to the ciphertext blob to determine which CMK was used to encrypt the ciphertext. However, you can use this parameter to ensure that a particular CMK (of any kind) is used to decrypt the ciphertext before it is reencrypted.</p> <p>If you specify a <code>KeyId</code> value, the decrypt part of the <code>ReEncrypt</code> operation succeeds only if the specified CMK was used to encrypt the ciphertext.</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "SourceKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_key_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReEncryptResponse {
    /// <p>The reencrypted data. When you use the HTTP API or the AWS CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p>
    #[serde(rename = "CiphertextBlob")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext_blob: Option<bytes::Bytes>,
    /// <p>The encryption algorithm that was used to reencrypt the data.</p>
    #[serde(rename = "DestinationEncryptionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_encryption_algorithm: Option<String>,
    /// <p>Unique identifier of the CMK used to reencrypt the data.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The encryption algorithm that was used to decrypt the ciphertext before it was reencrypted.</p>
    #[serde(rename = "SourceEncryptionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_encryption_algorithm: Option<String>,
    /// <p>Unique identifier of the CMK used to originally encrypt the data.</p>
    #[serde(rename = "SourceKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_key_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RetireGrantRequest {
    /// <p><p>Unique identifier of the grant to retire. The grant ID is returned in the response to a <code>CreateGrant</code> operation.</p> <ul> <li> <p>Grant ID Example - 0123456789012345678901234567890123456789012345678901234567890123</p> </li> </ul></p>
    #[serde(rename = "GrantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<String>,
    /// <p>Token that identifies the grant to be retired.</p>
    #[serde(rename = "GrantToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the CMK associated with the grant. </p> <p>For example: <code>arn:aws:kms:us-east-2:444455556666:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RevokeGrantRequest {
    /// <p>Identifier of the grant to be revoked.</p>
    #[serde(rename = "GrantId")]
    pub grant_id: String,
    /// <p>A unique identifier for the customer master key associated with the grant.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK. To specify a CMK in a different AWS account, you must use the key ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ScheduleKeyDeletionRequest {
    /// <p>The unique identifier of the customer master key (CMK) to delete.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>The waiting period, specified in number of days. After the waiting period ends, AWS KMS deletes the customer master key (CMK).</p> <p>This value is optional. If you include a value, it must be between 7 and 30, inclusive. If you do not include a value, it defaults to 30.</p>
    #[serde(rename = "PendingWindowInDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_window_in_days: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ScheduleKeyDeletionResponse {
    /// <p>The date and time after which AWS KMS deletes the customer master key (CMK).</p>
    #[serde(rename = "DeletionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<f64>,
    /// <p>The unique identifier of the customer master key (CMK) for which deletion is scheduled.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SignRequest {
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Identifies an asymmetric CMK. AWS KMS uses the private key in the asymmetric CMK to sign the message. The <code>KeyUsage</code> type of the CMK must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>Specifies the message or message digest to sign. Messages can be 0-4096 bytes. To sign a larger message, provide the message digest.</p> <p>If you provide a message, AWS KMS generates a hash digest of the message and then signs it.</p>
    #[serde(rename = "Message")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub message: bytes::Bytes,
    /// <p>Tells AWS KMS whether the value of the <code>Message</code> parameter is a message or message digest. To indicate a message, enter <code>RAW</code>. To indicate a message digest, enter <code>DIGEST</code>.</p>
    #[serde(rename = "MessageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// <p>Specifies the signing algorithm to use when signing the message. </p> <p>Choose an algorithm that is compatible with the type and size of the specified asymmetric CMK.</p>
    #[serde(rename = "SigningAlgorithm")]
    pub signing_algorithm: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SignResponse {
    /// <p>The Amazon Resource Name (ARN) of the asymmetric CMK that was used to sign the message.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The cryptographic signature that was generated for the message.</p>
    #[serde(rename = "Signature")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<bytes::Bytes>,
    /// <p>The signing algorithm that was used to sign the message.</p>
    #[serde(rename = "SigningAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_algorithm: Option<String>,
}

/// <p>A key-value pair. A tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty (null) strings.</p> <p>For information about the rules that apply to tag keys and tag values, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/allocation-tag-restrictions.html">User-Defined Tag Restrictions</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    #[serde(rename = "TagKey")]
    pub tag_key: String,
    /// <p>The value of the tag.</p>
    #[serde(rename = "TagValue")]
    pub tag_value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>A unique identifier for the CMK you are tagging.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>One or more tags. Each tag consists of a tag key and a tag value.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>A unique identifier for the CMK from which you are removing tags.</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>One or more tag keys. Specify only the tag keys, not the tag values.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAliasRequest {
    /// <p>Identifies the alias that is changing its CMK. This value must begin with <code>alias/</code> followed by the alias name, such as <code>alias/ExampleAlias</code>. You cannot use UpdateAlias to change the alias name.</p>
    #[serde(rename = "AliasName")]
    pub alias_name: String,
    /// <p>Identifies the CMK to associate with the alias. When the update operation completes, the alias will point to this CMK. </p> <p>The CMK must be in the same AWS account and Region as the alias. Also, the new target CMK must be the same type as the current target CMK (both symmetric or both asymmetric) and they must have the same key usage. </p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p> <p>To verify that the alias is mapped to the correct CMK, use <a>ListAliases</a>.</p>
    #[serde(rename = "TargetKeyId")]
    pub target_key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateCustomKeyStoreRequest {
    /// <p>Associates the custom key store with a related AWS CloudHSM cluster. </p> <p>Enter the cluster ID of the cluster that you used to create the custom key store or a cluster that shares a backup history and has the same cluster certificate as the original cluster. You cannot use this parameter to associate a custom key store with an unrelated cluster. In addition, the replacement cluster must <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">fulfill the requirements</a> for a cluster associated with a custom key store. To view the cluster certificate of a cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation.</p>
    #[serde(rename = "CloudHsmClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_hsm_cluster_id: Option<String>,
    /// <p>Identifies the custom key store that you want to update. Enter the ID of the custom key store. To find the ID of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p>
    #[serde(rename = "CustomKeyStoreId")]
    pub custom_key_store_id: String,
    /// <p>Enter the current password of the <code>kmsuser</code> crypto user (CU) in the AWS CloudHSM cluster that is associated with the custom key store.</p> <p>This parameter tells AWS KMS the current password of the <code>kmsuser</code> crypto user (CU). It does not set or change the password of any users in the AWS CloudHSM cluster.</p>
    #[serde(rename = "KeyStorePassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_store_password: Option<String>,
    /// <p>Changes the friendly name of the custom key store to the value that you specify. The custom key store name must be unique in the AWS account.</p>
    #[serde(rename = "NewCustomKeyStoreName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_custom_key_store_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateCustomKeyStoreResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateKeyDescriptionRequest {
    /// <p>New description for the CMK.</p>
    #[serde(rename = "Description")]
    pub description: String,
    /// <p>A unique identifier for the customer master key (CMK).</p> <p>Specify the key ID or the Amazon Resource Name (ARN) of the CMK.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct VerifyRequest {
    /// <p>A list of grant tokens.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#grant_token">Grant Tokens</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    #[serde(rename = "GrantTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_tokens: Option<Vec<String>>,
    /// <p>Identifies the asymmetric CMK that will be used to verify the signature. This must be the same CMK that was used to generate the signature. If you specify a different CMK, the value of the <code>SignatureValid</code> field in the response will be <code>False</code>.</p> <p>To specify a CMK, use its key ID, Amazon Resource Name (ARN), alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a CMK in a different AWS account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul> <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li> <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li> <li> <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code> </p> </li> </ul> <p>To get the key ID and key ARN for a CMK, use <a>ListKeys</a> or <a>DescribeKey</a>. To get the alias name and alias ARN, use <a>ListAliases</a>.</p>
    #[serde(rename = "KeyId")]
    pub key_id: String,
    /// <p>Specifies the message that was signed, or a hash digest of that message. Messages can be 0-4096 bytes. To verify a larger message, provide a hash digest of the message.</p> <p>If the digest of the message specified here is different from the message digest that was signed, the <code>SignatureValid</code> value in the response will be <code>False</code>.</p>
    #[serde(rename = "Message")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub message: bytes::Bytes,
    /// <p>Tells AWS KMS whether the value of the <code>Message</code> parameter is a message or message digest. To indicate a message, enter <code>RAW</code>. To indicate a message digest, enter <code>DIGEST</code>.</p>
    #[serde(rename = "MessageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// <p>The signature that the <code>Sign</code> operation generated.</p>
    #[serde(rename = "Signature")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub signature: bytes::Bytes,
    /// <p>The signing algorithm that was used to sign the message. If you submit a different algorithm, the value of the <code>SignatureValid</code> field in the response will be <code>False</code>.</p>
    #[serde(rename = "SigningAlgorithm")]
    pub signing_algorithm: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VerifyResponse {
    /// <p>The unique identifier for the asymmetric CMK that was used to verify the signature.</p>
    #[serde(rename = "KeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>A Boolean value that indicates whether the signature was verified. A value of True indicates that the <code>Signature</code> was produced by signing the <code>Message</code> with the specified KeyID and <code>SigningAlgorithm.</code> A value of False indicates that the message, the algorithm, or the key changed since the message was signed.</p>
    #[serde(rename = "SignatureValid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_valid: Option<bool>,
    /// <p>The signing algorithm that was used to verify the signature.</p>
    #[serde(rename = "SigningAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_algorithm: Option<String>,
}

/// Errors returned by CancelKeyDeletion
#[derive(Debug, PartialEq)]
pub enum CancelKeyDeletionError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl CancelKeyDeletionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelKeyDeletionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(CancelKeyDeletionError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(CancelKeyDeletionError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(CancelKeyDeletionError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(CancelKeyDeletionError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CancelKeyDeletionError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CancelKeyDeletionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelKeyDeletionError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            CancelKeyDeletionError::InvalidArn(ref cause) => write!(f, "{}", cause),
            CancelKeyDeletionError::KMSInternal(ref cause) => write!(f, "{}", cause),
            CancelKeyDeletionError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            CancelKeyDeletionError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelKeyDeletionError {}
/// Errors returned by ConnectCustomKeyStore
#[derive(Debug, PartialEq)]
pub enum ConnectCustomKeyStoreError {
    /// <p>The request was rejected because the associated AWS CloudHSM cluster did not meet the configuration requirements for a custom key store.</p> <ul> <li> <p>The cluster must be configured with private subnets in at least two different Availability Zones in the Region.</p> </li> <li> <p>The <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">security group for the cluster</a> (cloudhsm-cluster-<i>&lt;cluster-id&gt;</i>-sg) must include inbound rules and outbound rules that allow TCP traffic on ports 2223-2225. The <b>Source</b> in the inbound rules and the <b>Destination</b> in the outbound rules must match the security group ID. These rules are set by default when you create the cluster. Do not delete or change them. To get information about a particular security group, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSecurityGroups.html">DescribeSecurityGroups</a> operation.</p> </li> <li> <p>The cluster must contain at least as many HSMs as the operation requires. To add HSMs, use the AWS CloudHSM <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_CreateHsm.html">CreateHsm</a> operation.</p> <p>For the <a>CreateCustomKeyStore</a>, <a>UpdateCustomKeyStore</a>, and <a>CreateKey</a> operations, the AWS CloudHSM cluster must have at least two active HSMs, each in a different Availability Zone. For the <a>ConnectCustomKeyStore</a> operation, the AWS CloudHSM must contain at least one active HSM.</p> </li> </ul> <p>For information about the requirements for an AWS CloudHSM cluster that is associated with a custom key store, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">Assemble the Prerequisites</a> in the <i>AWS Key Management Service Developer Guide</i>. For information about creating a private subnet for an AWS CloudHSM cluster, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/create-subnets.html">Create a Private Subnet</a> in the <i>AWS CloudHSM User Guide</i>. For information about cluster security groups, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">Configure a Default Security Group</a> in the <i> <i>AWS CloudHSM User Guide</i> </i>. </p>
    CloudHsmClusterInvalidConfiguration(String),
    /// <p>The request was rejected because the AWS CloudHSM cluster that is associated with the custom key store is not active. Initialize and activate the cluster and try the command again. For detailed instructions, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/getting-started.html">Getting Started</a> in the <i>AWS CloudHSM User Guide</i>.</p>
    CloudHsmClusterNotActive(String),
    /// <p><p>The request was rejected because of the <code>ConnectionState</code> of the custom key store. To get the <code>ConnectionState</code> of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>This exception is thrown under the following conditions:</p> <ul> <li> <p>You requested the <a>CreateKey</a> or <a>GenerateRandom</a> operation in a custom key store that is not connected. These operations are valid only when the custom key store <code>ConnectionState</code> is <code>CONNECTED</code>.</p> </li> <li> <p>You requested the <a>UpdateCustomKeyStore</a> or <a>DeleteCustomKeyStore</a> operation on a custom key store that is not disconnected. This operation is valid only when the custom key store <code>ConnectionState</code> is <code>DISCONNECTED</code>.</p> </li> <li> <p>You requested the <a>ConnectCustomKeyStore</a> operation on a custom key store with a <code>ConnectionState</code> of <code>DISCONNECTING</code> or <code>FAILED</code>. This operation is valid for all other <code>ConnectionState</code> values.</p> </li> </ul></p>
    CustomKeyStoreInvalidState(String),
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl ConnectCustomKeyStoreError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ConnectCustomKeyStoreError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmClusterInvalidConfigurationException" => {
                    return RusotoError::Service(
                        ConnectCustomKeyStoreError::CloudHsmClusterInvalidConfiguration(err.msg),
                    )
                }
                "CloudHsmClusterNotActiveException" => {
                    return RusotoError::Service(
                        ConnectCustomKeyStoreError::CloudHsmClusterNotActive(err.msg),
                    )
                }
                "CustomKeyStoreInvalidStateException" => {
                    return RusotoError::Service(
                        ConnectCustomKeyStoreError::CustomKeyStoreInvalidState(err.msg),
                    )
                }
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(
                        ConnectCustomKeyStoreError::CustomKeyStoreNotFound(err.msg),
                    )
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ConnectCustomKeyStoreError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ConnectCustomKeyStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConnectCustomKeyStoreError::CloudHsmClusterInvalidConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            ConnectCustomKeyStoreError::CloudHsmClusterNotActive(ref cause) => {
                write!(f, "{}", cause)
            }
            ConnectCustomKeyStoreError::CustomKeyStoreInvalidState(ref cause) => {
                write!(f, "{}", cause)
            }
            ConnectCustomKeyStoreError::CustomKeyStoreNotFound(ref cause) => write!(f, "{}", cause),
            ConnectCustomKeyStoreError::KMSInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ConnectCustomKeyStoreError {}
/// Errors returned by CreateAlias
#[derive(Debug, PartialEq)]
pub enum CreateAliasError {
    /// <p>The request was rejected because it attempted to create a resource that already exists.</p>
    AlreadyExists(String),
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified alias name is not valid.</p>
    InvalidAliasName(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a limit was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl CreateAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAliasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AlreadyExistsException" => {
                    return RusotoError::Service(CreateAliasError::AlreadyExists(err.msg))
                }
                "DependencyTimeoutException" => {
                    return RusotoError::Service(CreateAliasError::DependencyTimeout(err.msg))
                }
                "InvalidAliasNameException" => {
                    return RusotoError::Service(CreateAliasError::InvalidAliasName(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(CreateAliasError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(CreateAliasError::KMSInvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateAliasError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateAliasError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAliasError::AlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateAliasError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            CreateAliasError::InvalidAliasName(ref cause) => write!(f, "{}", cause),
            CreateAliasError::KMSInternal(ref cause) => write!(f, "{}", cause),
            CreateAliasError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            CreateAliasError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateAliasError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAliasError {}
/// Errors returned by CreateCustomKeyStore
#[derive(Debug, PartialEq)]
pub enum CreateCustomKeyStoreError {
    /// <p>The request was rejected because the specified AWS CloudHSM cluster is already associated with a custom key store or it shares a backup history with a cluster that is associated with a custom key store. Each custom key store must be associated with a different AWS CloudHSM cluster.</p> <p>Clusters that share a backup history have the same cluster certificate. To view the cluster certificate of a cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation.</p>
    CloudHsmClusterInUse(String),
    /// <p>The request was rejected because the associated AWS CloudHSM cluster did not meet the configuration requirements for a custom key store.</p> <ul> <li> <p>The cluster must be configured with private subnets in at least two different Availability Zones in the Region.</p> </li> <li> <p>The <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">security group for the cluster</a> (cloudhsm-cluster-<i>&lt;cluster-id&gt;</i>-sg) must include inbound rules and outbound rules that allow TCP traffic on ports 2223-2225. The <b>Source</b> in the inbound rules and the <b>Destination</b> in the outbound rules must match the security group ID. These rules are set by default when you create the cluster. Do not delete or change them. To get information about a particular security group, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSecurityGroups.html">DescribeSecurityGroups</a> operation.</p> </li> <li> <p>The cluster must contain at least as many HSMs as the operation requires. To add HSMs, use the AWS CloudHSM <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_CreateHsm.html">CreateHsm</a> operation.</p> <p>For the <a>CreateCustomKeyStore</a>, <a>UpdateCustomKeyStore</a>, and <a>CreateKey</a> operations, the AWS CloudHSM cluster must have at least two active HSMs, each in a different Availability Zone. For the <a>ConnectCustomKeyStore</a> operation, the AWS CloudHSM must contain at least one active HSM.</p> </li> </ul> <p>For information about the requirements for an AWS CloudHSM cluster that is associated with a custom key store, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">Assemble the Prerequisites</a> in the <i>AWS Key Management Service Developer Guide</i>. For information about creating a private subnet for an AWS CloudHSM cluster, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/create-subnets.html">Create a Private Subnet</a> in the <i>AWS CloudHSM User Guide</i>. For information about cluster security groups, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">Configure a Default Security Group</a> in the <i> <i>AWS CloudHSM User Guide</i> </i>. </p>
    CloudHsmClusterInvalidConfiguration(String),
    /// <p>The request was rejected because the AWS CloudHSM cluster that is associated with the custom key store is not active. Initialize and activate the cluster and try the command again. For detailed instructions, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/getting-started.html">Getting Started</a> in the <i>AWS CloudHSM User Guide</i>.</p>
    CloudHsmClusterNotActive(String),
    /// <p>The request was rejected because AWS KMS cannot find the AWS CloudHSM cluster with the specified cluster ID. Retry the request with a different cluster ID.</p>
    CloudHsmClusterNotFound(String),
    /// <p>The request was rejected because the specified custom key store name is already assigned to another custom key store in the account. Try again with a custom key store name that is unique in the account.</p>
    CustomKeyStoreNameInUse(String),
    /// <p>The request was rejected because the trust anchor certificate in the request is not the trust anchor certificate for the specified AWS CloudHSM cluster.</p> <p>When you <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/initialize-cluster.html#sign-csr">initialize the cluster</a>, you create the trust anchor certificate and save it in the <code>customerCA.crt</code> file.</p>
    IncorrectTrustAnchor(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl CreateCustomKeyStoreError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCustomKeyStoreError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmClusterInUseException" => {
                    return RusotoError::Service(CreateCustomKeyStoreError::CloudHsmClusterInUse(
                        err.msg,
                    ))
                }
                "CloudHsmClusterInvalidConfigurationException" => {
                    return RusotoError::Service(
                        CreateCustomKeyStoreError::CloudHsmClusterInvalidConfiguration(err.msg),
                    )
                }
                "CloudHsmClusterNotActiveException" => {
                    return RusotoError::Service(
                        CreateCustomKeyStoreError::CloudHsmClusterNotActive(err.msg),
                    )
                }
                "CloudHsmClusterNotFoundException" => {
                    return RusotoError::Service(
                        CreateCustomKeyStoreError::CloudHsmClusterNotFound(err.msg),
                    )
                }
                "CustomKeyStoreNameInUseException" => {
                    return RusotoError::Service(
                        CreateCustomKeyStoreError::CustomKeyStoreNameInUse(err.msg),
                    )
                }
                "IncorrectTrustAnchorException" => {
                    return RusotoError::Service(CreateCustomKeyStoreError::IncorrectTrustAnchor(
                        err.msg,
                    ))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(CreateCustomKeyStoreError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateCustomKeyStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCustomKeyStoreError::CloudHsmClusterInUse(ref cause) => write!(f, "{}", cause),
            CreateCustomKeyStoreError::CloudHsmClusterInvalidConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCustomKeyStoreError::CloudHsmClusterNotActive(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCustomKeyStoreError::CloudHsmClusterNotFound(ref cause) => write!(f, "{}", cause),
            CreateCustomKeyStoreError::CustomKeyStoreNameInUse(ref cause) => write!(f, "{}", cause),
            CreateCustomKeyStoreError::IncorrectTrustAnchor(ref cause) => write!(f, "{}", cause),
            CreateCustomKeyStoreError::KMSInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateCustomKeyStoreError {}
/// Errors returned by CreateGrant
#[derive(Debug, PartialEq)]
pub enum CreateGrantError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a limit was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl CreateGrantError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateGrantError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(CreateGrantError::DependencyTimeout(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(CreateGrantError::Disabled(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(CreateGrantError::InvalidArn(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(CreateGrantError::InvalidGrantToken(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(CreateGrantError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(CreateGrantError::KMSInvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateGrantError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateGrantError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateGrantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateGrantError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            CreateGrantError::Disabled(ref cause) => write!(f, "{}", cause),
            CreateGrantError::InvalidArn(ref cause) => write!(f, "{}", cause),
            CreateGrantError::InvalidGrantToken(ref cause) => write!(f, "{}", cause),
            CreateGrantError::KMSInternal(ref cause) => write!(f, "{}", cause),
            CreateGrantError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            CreateGrantError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateGrantError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateGrantError {}
/// Errors returned by CreateKey
#[derive(Debug, PartialEq)]
pub enum CreateKeyError {
    /// <p>The request was rejected because the associated AWS CloudHSM cluster did not meet the configuration requirements for a custom key store.</p> <ul> <li> <p>The cluster must be configured with private subnets in at least two different Availability Zones in the Region.</p> </li> <li> <p>The <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">security group for the cluster</a> (cloudhsm-cluster-<i>&lt;cluster-id&gt;</i>-sg) must include inbound rules and outbound rules that allow TCP traffic on ports 2223-2225. The <b>Source</b> in the inbound rules and the <b>Destination</b> in the outbound rules must match the security group ID. These rules are set by default when you create the cluster. Do not delete or change them. To get information about a particular security group, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSecurityGroups.html">DescribeSecurityGroups</a> operation.</p> </li> <li> <p>The cluster must contain at least as many HSMs as the operation requires. To add HSMs, use the AWS CloudHSM <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_CreateHsm.html">CreateHsm</a> operation.</p> <p>For the <a>CreateCustomKeyStore</a>, <a>UpdateCustomKeyStore</a>, and <a>CreateKey</a> operations, the AWS CloudHSM cluster must have at least two active HSMs, each in a different Availability Zone. For the <a>ConnectCustomKeyStore</a> operation, the AWS CloudHSM must contain at least one active HSM.</p> </li> </ul> <p>For information about the requirements for an AWS CloudHSM cluster that is associated with a custom key store, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">Assemble the Prerequisites</a> in the <i>AWS Key Management Service Developer Guide</i>. For information about creating a private subnet for an AWS CloudHSM cluster, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/create-subnets.html">Create a Private Subnet</a> in the <i>AWS CloudHSM User Guide</i>. For information about cluster security groups, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">Configure a Default Security Group</a> in the <i> <i>AWS CloudHSM User Guide</i> </i>. </p>
    CloudHsmClusterInvalidConfiguration(String),
    /// <p><p>The request was rejected because of the <code>ConnectionState</code> of the custom key store. To get the <code>ConnectionState</code> of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>This exception is thrown under the following conditions:</p> <ul> <li> <p>You requested the <a>CreateKey</a> or <a>GenerateRandom</a> operation in a custom key store that is not connected. These operations are valid only when the custom key store <code>ConnectionState</code> is <code>CONNECTED</code>.</p> </li> <li> <p>You requested the <a>UpdateCustomKeyStore</a> or <a>DeleteCustomKeyStore</a> operation on a custom key store that is not disconnected. This operation is valid only when the custom key store <code>ConnectionState</code> is <code>DISCONNECTED</code>.</p> </li> <li> <p>You requested the <a>ConnectCustomKeyStore</a> operation on a custom key store with a <code>ConnectionState</code> of <code>DISCONNECTING</code> or <code>FAILED</code>. This operation is valid for all other <code>ConnectionState</code> values.</p> </li> </ul></p>
    CustomKeyStoreInvalidState(String),
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because a limit was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified policy is not syntactically or semantically correct.</p>
    MalformedPolicyDocument(String),
    /// <p>The request was rejected because one or more tags are not valid.</p>
    Tag(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl CreateKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmClusterInvalidConfigurationException" => {
                    return RusotoError::Service(
                        CreateKeyError::CloudHsmClusterInvalidConfiguration(err.msg),
                    )
                }
                "CustomKeyStoreInvalidStateException" => {
                    return RusotoError::Service(CreateKeyError::CustomKeyStoreInvalidState(
                        err.msg,
                    ))
                }
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(CreateKeyError::CustomKeyStoreNotFound(err.msg))
                }
                "DependencyTimeoutException" => {
                    return RusotoError::Service(CreateKeyError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(CreateKeyError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(CreateKeyError::KMSInternal(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateKeyError::LimitExceeded(err.msg))
                }
                "MalformedPolicyDocumentException" => {
                    return RusotoError::Service(CreateKeyError::MalformedPolicyDocument(err.msg))
                }
                "TagException" => return RusotoError::Service(CreateKeyError::Tag(err.msg)),
                "UnsupportedOperationException" => {
                    return RusotoError::Service(CreateKeyError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateKeyError::CloudHsmClusterInvalidConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateKeyError::CustomKeyStoreInvalidState(ref cause) => write!(f, "{}", cause),
            CreateKeyError::CustomKeyStoreNotFound(ref cause) => write!(f, "{}", cause),
            CreateKeyError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            CreateKeyError::InvalidArn(ref cause) => write!(f, "{}", cause),
            CreateKeyError::KMSInternal(ref cause) => write!(f, "{}", cause),
            CreateKeyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateKeyError::MalformedPolicyDocument(ref cause) => write!(f, "{}", cause),
            CreateKeyError::Tag(ref cause) => write!(f, "{}", cause),
            CreateKeyError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateKeyError {}
/// Errors returned by Decrypt
#[derive(Debug, PartialEq)]
pub enum DecryptError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified CMK cannot decrypt the data. The <code>KeyId</code> in a <a>Decrypt</a> request and the <code>SourceKeyId</code> in a <a>ReEncrypt</a> request must identify the same CMK that was used to encrypt the ciphertext.</p>
    IncorrectKey(String),
    /// <p>From the <a>Decrypt</a> or <a>ReEncrypt</a> operation, the request was rejected because the specified ciphertext, or additional authenticated data incorporated into the ciphertext, such as the encryption context, is corrupted, missing, or otherwise invalid.</p> <p>From the <a>ImportKeyMaterial</a> operation, the request was rejected because AWS KMS could not decrypt the encrypted (wrapped) key material. </p>
    InvalidCiphertext(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected for one of the following reasons: </p> <ul> <li> <p>The <code>KeyUsage</code> value of the CMK is incompatible with the API operation.</p> </li> <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the CMK <code>(CustomerMasterKeySpec</code>).</p> </li> </ul> <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To find the encryption or signing algorithms supported for a particular CMK, use the <a>DescribeKey</a> operation.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. You can retry the request.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl DecryptError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DecryptError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(DecryptError::DependencyTimeout(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(DecryptError::Disabled(err.msg))
                }
                "IncorrectKeyException" => {
                    return RusotoError::Service(DecryptError::IncorrectKey(err.msg))
                }
                "InvalidCiphertextException" => {
                    return RusotoError::Service(DecryptError::InvalidCiphertext(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(DecryptError::InvalidGrantToken(err.msg))
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(DecryptError::InvalidKeyUsage(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DecryptError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(DecryptError::KMSInvalidState(err.msg))
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(DecryptError::KeyUnavailable(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DecryptError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DecryptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecryptError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            DecryptError::Disabled(ref cause) => write!(f, "{}", cause),
            DecryptError::IncorrectKey(ref cause) => write!(f, "{}", cause),
            DecryptError::InvalidCiphertext(ref cause) => write!(f, "{}", cause),
            DecryptError::InvalidGrantToken(ref cause) => write!(f, "{}", cause),
            DecryptError::InvalidKeyUsage(ref cause) => write!(f, "{}", cause),
            DecryptError::KMSInternal(ref cause) => write!(f, "{}", cause),
            DecryptError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            DecryptError::KeyUnavailable(ref cause) => write!(f, "{}", cause),
            DecryptError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DecryptError {}
/// Errors returned by DeleteAlias
#[derive(Debug, PartialEq)]
pub enum DeleteAliasError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl DeleteAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAliasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(DeleteAliasError::DependencyTimeout(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DeleteAliasError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(DeleteAliasError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteAliasError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAliasError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            DeleteAliasError::KMSInternal(ref cause) => write!(f, "{}", cause),
            DeleteAliasError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            DeleteAliasError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAliasError {}
/// Errors returned by DeleteCustomKeyStore
#[derive(Debug, PartialEq)]
pub enum DeleteCustomKeyStoreError {
    /// <p>The request was rejected because the custom key store contains AWS KMS customer master keys (CMKs). After verifying that you do not need to use the CMKs, use the <a>ScheduleKeyDeletion</a> operation to delete the CMKs. After they are deleted, you can delete the custom key store.</p>
    CustomKeyStoreHasCMKs(String),
    /// <p><p>The request was rejected because of the <code>ConnectionState</code> of the custom key store. To get the <code>ConnectionState</code> of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>This exception is thrown under the following conditions:</p> <ul> <li> <p>You requested the <a>CreateKey</a> or <a>GenerateRandom</a> operation in a custom key store that is not connected. These operations are valid only when the custom key store <code>ConnectionState</code> is <code>CONNECTED</code>.</p> </li> <li> <p>You requested the <a>UpdateCustomKeyStore</a> or <a>DeleteCustomKeyStore</a> operation on a custom key store that is not disconnected. This operation is valid only when the custom key store <code>ConnectionState</code> is <code>DISCONNECTED</code>.</p> </li> <li> <p>You requested the <a>ConnectCustomKeyStore</a> operation on a custom key store with a <code>ConnectionState</code> of <code>DISCONNECTING</code> or <code>FAILED</code>. This operation is valid for all other <code>ConnectionState</code> values.</p> </li> </ul></p>
    CustomKeyStoreInvalidState(String),
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl DeleteCustomKeyStoreError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCustomKeyStoreError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CustomKeyStoreHasCMKsException" => {
                    return RusotoError::Service(DeleteCustomKeyStoreError::CustomKeyStoreHasCMKs(
                        err.msg,
                    ))
                }
                "CustomKeyStoreInvalidStateException" => {
                    return RusotoError::Service(
                        DeleteCustomKeyStoreError::CustomKeyStoreInvalidState(err.msg),
                    )
                }
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(DeleteCustomKeyStoreError::CustomKeyStoreNotFound(
                        err.msg,
                    ))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DeleteCustomKeyStoreError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteCustomKeyStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCustomKeyStoreError::CustomKeyStoreHasCMKs(ref cause) => write!(f, "{}", cause),
            DeleteCustomKeyStoreError::CustomKeyStoreInvalidState(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteCustomKeyStoreError::CustomKeyStoreNotFound(ref cause) => write!(f, "{}", cause),
            DeleteCustomKeyStoreError::KMSInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteCustomKeyStoreError {}
/// Errors returned by DeleteImportedKeyMaterial
#[derive(Debug, PartialEq)]
pub enum DeleteImportedKeyMaterialError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl DeleteImportedKeyMaterialError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteImportedKeyMaterialError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(DeleteImportedKeyMaterialError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DeleteImportedKeyMaterialError::InvalidArn(
                        err.msg,
                    ))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DeleteImportedKeyMaterialError::KMSInternal(
                        err.msg,
                    ))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(DeleteImportedKeyMaterialError::KMSInvalidState(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteImportedKeyMaterialError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DeleteImportedKeyMaterialError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteImportedKeyMaterialError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteImportedKeyMaterialError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            DeleteImportedKeyMaterialError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DeleteImportedKeyMaterialError::KMSInternal(ref cause) => write!(f, "{}", cause),
            DeleteImportedKeyMaterialError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            DeleteImportedKeyMaterialError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteImportedKeyMaterialError::UnsupportedOperation(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteImportedKeyMaterialError {}
/// Errors returned by DescribeCustomKeyStores
#[derive(Debug, PartialEq)]
pub enum DescribeCustomKeyStoresError {
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl DescribeCustomKeyStoresError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCustomKeyStoresError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(
                        DescribeCustomKeyStoresError::CustomKeyStoreNotFound(err.msg),
                    )
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DescribeCustomKeyStoresError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeCustomKeyStoresError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCustomKeyStoresError::CustomKeyStoreNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeCustomKeyStoresError::KMSInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeCustomKeyStoresError {}
/// Errors returned by DescribeKey
#[derive(Debug, PartialEq)]
pub enum DescribeKeyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl DescribeKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(DescribeKeyError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DescribeKeyError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DescribeKeyError::KMSInternal(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeKeyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeKeyError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            DescribeKeyError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DescribeKeyError::KMSInternal(ref cause) => write!(f, "{}", cause),
            DescribeKeyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeKeyError {}
/// Errors returned by DisableKey
#[derive(Debug, PartialEq)]
pub enum DisableKeyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl DisableKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(DisableKeyError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DisableKeyError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DisableKeyError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(DisableKeyError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DisableKeyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisableKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableKeyError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            DisableKeyError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DisableKeyError::KMSInternal(ref cause) => write!(f, "{}", cause),
            DisableKeyError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            DisableKeyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisableKeyError {}
/// Errors returned by DisableKeyRotation
#[derive(Debug, PartialEq)]
pub enum DisableKeyRotationError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl DisableKeyRotationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableKeyRotationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(DisableKeyRotationError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "DisabledException" => {
                    return RusotoError::Service(DisableKeyRotationError::Disabled(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(DisableKeyRotationError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DisableKeyRotationError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(DisableKeyRotationError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DisableKeyRotationError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DisableKeyRotationError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisableKeyRotationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisableKeyRotationError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            DisableKeyRotationError::Disabled(ref cause) => write!(f, "{}", cause),
            DisableKeyRotationError::InvalidArn(ref cause) => write!(f, "{}", cause),
            DisableKeyRotationError::KMSInternal(ref cause) => write!(f, "{}", cause),
            DisableKeyRotationError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            DisableKeyRotationError::NotFound(ref cause) => write!(f, "{}", cause),
            DisableKeyRotationError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisableKeyRotationError {}
/// Errors returned by DisconnectCustomKeyStore
#[derive(Debug, PartialEq)]
pub enum DisconnectCustomKeyStoreError {
    /// <p><p>The request was rejected because of the <code>ConnectionState</code> of the custom key store. To get the <code>ConnectionState</code> of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>This exception is thrown under the following conditions:</p> <ul> <li> <p>You requested the <a>CreateKey</a> or <a>GenerateRandom</a> operation in a custom key store that is not connected. These operations are valid only when the custom key store <code>ConnectionState</code> is <code>CONNECTED</code>.</p> </li> <li> <p>You requested the <a>UpdateCustomKeyStore</a> or <a>DeleteCustomKeyStore</a> operation on a custom key store that is not disconnected. This operation is valid only when the custom key store <code>ConnectionState</code> is <code>DISCONNECTED</code>.</p> </li> <li> <p>You requested the <a>ConnectCustomKeyStore</a> operation on a custom key store with a <code>ConnectionState</code> of <code>DISCONNECTING</code> or <code>FAILED</code>. This operation is valid for all other <code>ConnectionState</code> values.</p> </li> </ul></p>
    CustomKeyStoreInvalidState(String),
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl DisconnectCustomKeyStoreError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisconnectCustomKeyStoreError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CustomKeyStoreInvalidStateException" => {
                    return RusotoError::Service(
                        DisconnectCustomKeyStoreError::CustomKeyStoreInvalidState(err.msg),
                    )
                }
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(
                        DisconnectCustomKeyStoreError::CustomKeyStoreNotFound(err.msg),
                    )
                }
                "KMSInternalException" => {
                    return RusotoError::Service(DisconnectCustomKeyStoreError::KMSInternal(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DisconnectCustomKeyStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisconnectCustomKeyStoreError::CustomKeyStoreInvalidState(ref cause) => {
                write!(f, "{}", cause)
            }
            DisconnectCustomKeyStoreError::CustomKeyStoreNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DisconnectCustomKeyStoreError::KMSInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisconnectCustomKeyStoreError {}
/// Errors returned by EnableKey
#[derive(Debug, PartialEq)]
pub enum EnableKeyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a limit was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl EnableKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(EnableKeyError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(EnableKeyError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(EnableKeyError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(EnableKeyError::KMSInvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(EnableKeyError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(EnableKeyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for EnableKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableKeyError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            EnableKeyError::InvalidArn(ref cause) => write!(f, "{}", cause),
            EnableKeyError::KMSInternal(ref cause) => write!(f, "{}", cause),
            EnableKeyError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            EnableKeyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            EnableKeyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnableKeyError {}
/// Errors returned by EnableKeyRotation
#[derive(Debug, PartialEq)]
pub enum EnableKeyRotationError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl EnableKeyRotationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableKeyRotationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(EnableKeyRotationError::DependencyTimeout(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(EnableKeyRotationError::Disabled(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(EnableKeyRotationError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(EnableKeyRotationError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(EnableKeyRotationError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(EnableKeyRotationError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(EnableKeyRotationError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for EnableKeyRotationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EnableKeyRotationError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            EnableKeyRotationError::Disabled(ref cause) => write!(f, "{}", cause),
            EnableKeyRotationError::InvalidArn(ref cause) => write!(f, "{}", cause),
            EnableKeyRotationError::KMSInternal(ref cause) => write!(f, "{}", cause),
            EnableKeyRotationError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            EnableKeyRotationError::NotFound(ref cause) => write!(f, "{}", cause),
            EnableKeyRotationError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EnableKeyRotationError {}
/// Errors returned by Encrypt
#[derive(Debug, PartialEq)]
pub enum EncryptError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected for one of the following reasons: </p> <ul> <li> <p>The <code>KeyUsage</code> value of the CMK is incompatible with the API operation.</p> </li> <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the CMK <code>(CustomerMasterKeySpec</code>).</p> </li> </ul> <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To find the encryption or signing algorithms supported for a particular CMK, use the <a>DescribeKey</a> operation.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. You can retry the request.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl EncryptError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EncryptError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(EncryptError::DependencyTimeout(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(EncryptError::Disabled(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(EncryptError::InvalidGrantToken(err.msg))
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(EncryptError::InvalidKeyUsage(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(EncryptError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(EncryptError::KMSInvalidState(err.msg))
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(EncryptError::KeyUnavailable(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(EncryptError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for EncryptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EncryptError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            EncryptError::Disabled(ref cause) => write!(f, "{}", cause),
            EncryptError::InvalidGrantToken(ref cause) => write!(f, "{}", cause),
            EncryptError::InvalidKeyUsage(ref cause) => write!(f, "{}", cause),
            EncryptError::KMSInternal(ref cause) => write!(f, "{}", cause),
            EncryptError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            EncryptError::KeyUnavailable(ref cause) => write!(f, "{}", cause),
            EncryptError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for EncryptError {}
/// Errors returned by GenerateDataKey
#[derive(Debug, PartialEq)]
pub enum GenerateDataKeyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected for one of the following reasons: </p> <ul> <li> <p>The <code>KeyUsage</code> value of the CMK is incompatible with the API operation.</p> </li> <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the CMK <code>(CustomerMasterKeySpec</code>).</p> </li> </ul> <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To find the encryption or signing algorithms supported for a particular CMK, use the <a>DescribeKey</a> operation.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. You can retry the request.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl GenerateDataKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GenerateDataKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(GenerateDataKeyError::DependencyTimeout(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(GenerateDataKeyError::Disabled(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(GenerateDataKeyError::InvalidGrantToken(err.msg))
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(GenerateDataKeyError::InvalidKeyUsage(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GenerateDataKeyError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(GenerateDataKeyError::KMSInvalidState(err.msg))
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(GenerateDataKeyError::KeyUnavailable(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GenerateDataKeyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GenerateDataKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenerateDataKeyError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyError::Disabled(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyError::InvalidGrantToken(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyError::InvalidKeyUsage(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyError::KMSInternal(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyError::KeyUnavailable(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GenerateDataKeyError {}
/// Errors returned by GenerateDataKeyPair
#[derive(Debug, PartialEq)]
pub enum GenerateDataKeyPairError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected for one of the following reasons: </p> <ul> <li> <p>The <code>KeyUsage</code> value of the CMK is incompatible with the API operation.</p> </li> <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the CMK <code>(CustomerMasterKeySpec</code>).</p> </li> </ul> <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To find the encryption or signing algorithms supported for a particular CMK, use the <a>DescribeKey</a> operation.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. You can retry the request.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl GenerateDataKeyPairError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GenerateDataKeyPairError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(GenerateDataKeyPairError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "DisabledException" => {
                    return RusotoError::Service(GenerateDataKeyPairError::Disabled(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(GenerateDataKeyPairError::InvalidGrantToken(
                        err.msg,
                    ))
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(GenerateDataKeyPairError::InvalidKeyUsage(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GenerateDataKeyPairError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(GenerateDataKeyPairError::KMSInvalidState(err.msg))
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(GenerateDataKeyPairError::KeyUnavailable(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GenerateDataKeyPairError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GenerateDataKeyPairError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenerateDataKeyPairError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyPairError::Disabled(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyPairError::InvalidGrantToken(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyPairError::InvalidKeyUsage(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyPairError::KMSInternal(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyPairError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyPairError::KeyUnavailable(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyPairError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GenerateDataKeyPairError {}
/// Errors returned by GenerateDataKeyPairWithoutPlaintext
#[derive(Debug, PartialEq)]
pub enum GenerateDataKeyPairWithoutPlaintextError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected for one of the following reasons: </p> <ul> <li> <p>The <code>KeyUsage</code> value of the CMK is incompatible with the API operation.</p> </li> <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the CMK <code>(CustomerMasterKeySpec</code>).</p> </li> </ul> <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To find the encryption or signing algorithms supported for a particular CMK, use the <a>DescribeKey</a> operation.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. You can retry the request.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl GenerateDataKeyPairWithoutPlaintextError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GenerateDataKeyPairWithoutPlaintextError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(
                        GenerateDataKeyPairWithoutPlaintextError::DependencyTimeout(err.msg),
                    )
                }
                "DisabledException" => {
                    return RusotoError::Service(
                        GenerateDataKeyPairWithoutPlaintextError::Disabled(err.msg),
                    )
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(
                        GenerateDataKeyPairWithoutPlaintextError::InvalidGrantToken(err.msg),
                    )
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(
                        GenerateDataKeyPairWithoutPlaintextError::InvalidKeyUsage(err.msg),
                    )
                }
                "KMSInternalException" => {
                    return RusotoError::Service(
                        GenerateDataKeyPairWithoutPlaintextError::KMSInternal(err.msg),
                    )
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(
                        GenerateDataKeyPairWithoutPlaintextError::KMSInvalidState(err.msg),
                    )
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(
                        GenerateDataKeyPairWithoutPlaintextError::KeyUnavailable(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        GenerateDataKeyPairWithoutPlaintextError::NotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GenerateDataKeyPairWithoutPlaintextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenerateDataKeyPairWithoutPlaintextError::DependencyTimeout(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyPairWithoutPlaintextError::Disabled(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyPairWithoutPlaintextError::InvalidGrantToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyPairWithoutPlaintextError::InvalidKeyUsage(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyPairWithoutPlaintextError::KMSInternal(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyPairWithoutPlaintextError::KMSInvalidState(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyPairWithoutPlaintextError::KeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyPairWithoutPlaintextError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GenerateDataKeyPairWithoutPlaintextError {}
/// Errors returned by GenerateDataKeyWithoutPlaintext
#[derive(Debug, PartialEq)]
pub enum GenerateDataKeyWithoutPlaintextError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected for one of the following reasons: </p> <ul> <li> <p>The <code>KeyUsage</code> value of the CMK is incompatible with the API operation.</p> </li> <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the CMK <code>(CustomerMasterKeySpec</code>).</p> </li> </ul> <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To find the encryption or signing algorithms supported for a particular CMK, use the <a>DescribeKey</a> operation.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. You can retry the request.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl GenerateDataKeyWithoutPlaintextError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GenerateDataKeyWithoutPlaintextError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(
                        GenerateDataKeyWithoutPlaintextError::DependencyTimeout(err.msg),
                    )
                }
                "DisabledException" => {
                    return RusotoError::Service(GenerateDataKeyWithoutPlaintextError::Disabled(
                        err.msg,
                    ))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(
                        GenerateDataKeyWithoutPlaintextError::InvalidGrantToken(err.msg),
                    )
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(
                        GenerateDataKeyWithoutPlaintextError::InvalidKeyUsage(err.msg),
                    )
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GenerateDataKeyWithoutPlaintextError::KMSInternal(
                        err.msg,
                    ))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(
                        GenerateDataKeyWithoutPlaintextError::KMSInvalidState(err.msg),
                    )
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(
                        GenerateDataKeyWithoutPlaintextError::KeyUnavailable(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(GenerateDataKeyWithoutPlaintextError::NotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GenerateDataKeyWithoutPlaintextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenerateDataKeyWithoutPlaintextError::DependencyTimeout(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyWithoutPlaintextError::Disabled(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyWithoutPlaintextError::InvalidGrantToken(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyWithoutPlaintextError::InvalidKeyUsage(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyWithoutPlaintextError::KMSInternal(ref cause) => write!(f, "{}", cause),
            GenerateDataKeyWithoutPlaintextError::KMSInvalidState(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyWithoutPlaintextError::KeyUnavailable(ref cause) => {
                write!(f, "{}", cause)
            }
            GenerateDataKeyWithoutPlaintextError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GenerateDataKeyWithoutPlaintextError {}
/// Errors returned by GenerateRandom
#[derive(Debug, PartialEq)]
pub enum GenerateRandomError {
    /// <p><p>The request was rejected because of the <code>ConnectionState</code> of the custom key store. To get the <code>ConnectionState</code> of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>This exception is thrown under the following conditions:</p> <ul> <li> <p>You requested the <a>CreateKey</a> or <a>GenerateRandom</a> operation in a custom key store that is not connected. These operations are valid only when the custom key store <code>ConnectionState</code> is <code>CONNECTED</code>.</p> </li> <li> <p>You requested the <a>UpdateCustomKeyStore</a> or <a>DeleteCustomKeyStore</a> operation on a custom key store that is not disconnected. This operation is valid only when the custom key store <code>ConnectionState</code> is <code>DISCONNECTED</code>.</p> </li> <li> <p>You requested the <a>ConnectCustomKeyStore</a> operation on a custom key store with a <code>ConnectionState</code> of <code>DISCONNECTING</code> or <code>FAILED</code>. This operation is valid for all other <code>ConnectionState</code> values.</p> </li> </ul></p>
    CustomKeyStoreInvalidState(String),
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl GenerateRandomError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GenerateRandomError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CustomKeyStoreInvalidStateException" => {
                    return RusotoError::Service(GenerateRandomError::CustomKeyStoreInvalidState(
                        err.msg,
                    ))
                }
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(GenerateRandomError::CustomKeyStoreNotFound(
                        err.msg,
                    ))
                }
                "DependencyTimeoutException" => {
                    return RusotoError::Service(GenerateRandomError::DependencyTimeout(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GenerateRandomError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GenerateRandomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenerateRandomError::CustomKeyStoreInvalidState(ref cause) => write!(f, "{}", cause),
            GenerateRandomError::CustomKeyStoreNotFound(ref cause) => write!(f, "{}", cause),
            GenerateRandomError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            GenerateRandomError::KMSInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GenerateRandomError {}
/// Errors returned by GetKeyPolicy
#[derive(Debug, PartialEq)]
pub enum GetKeyPolicyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl GetKeyPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetKeyPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(GetKeyPolicyError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(GetKeyPolicyError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GetKeyPolicyError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(GetKeyPolicyError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetKeyPolicyError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetKeyPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetKeyPolicyError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            GetKeyPolicyError::InvalidArn(ref cause) => write!(f, "{}", cause),
            GetKeyPolicyError::KMSInternal(ref cause) => write!(f, "{}", cause),
            GetKeyPolicyError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            GetKeyPolicyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetKeyPolicyError {}
/// Errors returned by GetKeyRotationStatus
#[derive(Debug, PartialEq)]
pub enum GetKeyRotationStatusError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl GetKeyRotationStatusError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetKeyRotationStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(GetKeyRotationStatusError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(GetKeyRotationStatusError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GetKeyRotationStatusError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(GetKeyRotationStatusError::KMSInvalidState(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetKeyRotationStatusError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(GetKeyRotationStatusError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetKeyRotationStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetKeyRotationStatusError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            GetKeyRotationStatusError::InvalidArn(ref cause) => write!(f, "{}", cause),
            GetKeyRotationStatusError::KMSInternal(ref cause) => write!(f, "{}", cause),
            GetKeyRotationStatusError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            GetKeyRotationStatusError::NotFound(ref cause) => write!(f, "{}", cause),
            GetKeyRotationStatusError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetKeyRotationStatusError {}
/// Errors returned by GetParametersForImport
#[derive(Debug, PartialEq)]
pub enum GetParametersForImportError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl GetParametersForImportError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetParametersForImportError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(GetParametersForImportError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(GetParametersForImportError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GetParametersForImportError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(GetParametersForImportError::KMSInvalidState(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetParametersForImportError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(GetParametersForImportError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetParametersForImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetParametersForImportError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            GetParametersForImportError::InvalidArn(ref cause) => write!(f, "{}", cause),
            GetParametersForImportError::KMSInternal(ref cause) => write!(f, "{}", cause),
            GetParametersForImportError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            GetParametersForImportError::NotFound(ref cause) => write!(f, "{}", cause),
            GetParametersForImportError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetParametersForImportError {}
/// Errors returned by GetPublicKey
#[derive(Debug, PartialEq)]
pub enum GetPublicKeyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected for one of the following reasons: </p> <ul> <li> <p>The <code>KeyUsage</code> value of the CMK is incompatible with the API operation.</p> </li> <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the CMK <code>(CustomerMasterKeySpec</code>).</p> </li> </ul> <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To find the encryption or signing algorithms supported for a particular CMK, use the <a>DescribeKey</a> operation.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. You can retry the request.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl GetPublicKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPublicKeyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(GetPublicKeyError::DependencyTimeout(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(GetPublicKeyError::Disabled(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(GetPublicKeyError::InvalidArn(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(GetPublicKeyError::InvalidGrantToken(err.msg))
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(GetPublicKeyError::InvalidKeyUsage(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(GetPublicKeyError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(GetPublicKeyError::KMSInvalidState(err.msg))
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(GetPublicKeyError::KeyUnavailable(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetPublicKeyError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(GetPublicKeyError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetPublicKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPublicKeyError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            GetPublicKeyError::Disabled(ref cause) => write!(f, "{}", cause),
            GetPublicKeyError::InvalidArn(ref cause) => write!(f, "{}", cause),
            GetPublicKeyError::InvalidGrantToken(ref cause) => write!(f, "{}", cause),
            GetPublicKeyError::InvalidKeyUsage(ref cause) => write!(f, "{}", cause),
            GetPublicKeyError::KMSInternal(ref cause) => write!(f, "{}", cause),
            GetPublicKeyError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            GetPublicKeyError::KeyUnavailable(ref cause) => write!(f, "{}", cause),
            GetPublicKeyError::NotFound(ref cause) => write!(f, "{}", cause),
            GetPublicKeyError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPublicKeyError {}
/// Errors returned by ImportKeyMaterial
#[derive(Debug, PartialEq)]
pub enum ImportKeyMaterialError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified import token is expired. Use <a>GetParametersForImport</a> to get a new import token and public key, use the new public key to encrypt the key material, and then try the request again.</p>
    ExpiredImportToken(String),
    /// <p>The request was rejected because the key material in the request is, expired, invalid, or is not the same key material that was previously imported into this customer master key (CMK).</p>
    IncorrectKeyMaterial(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>From the <a>Decrypt</a> or <a>ReEncrypt</a> operation, the request was rejected because the specified ciphertext, or additional authenticated data incorporated into the ciphertext, such as the encryption context, is corrupted, missing, or otherwise invalid.</p> <p>From the <a>ImportKeyMaterial</a> operation, the request was rejected because AWS KMS could not decrypt the encrypted (wrapped) key material. </p>
    InvalidCiphertext(String),
    /// <p>The request was rejected because the provided import token is invalid or is associated with a different customer master key (CMK).</p>
    InvalidImportToken(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl ImportKeyMaterialError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportKeyMaterialError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ImportKeyMaterialError::DependencyTimeout(err.msg))
                }
                "ExpiredImportTokenException" => {
                    return RusotoError::Service(ImportKeyMaterialError::ExpiredImportToken(
                        err.msg,
                    ))
                }
                "IncorrectKeyMaterialException" => {
                    return RusotoError::Service(ImportKeyMaterialError::IncorrectKeyMaterial(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ImportKeyMaterialError::InvalidArn(err.msg))
                }
                "InvalidCiphertextException" => {
                    return RusotoError::Service(ImportKeyMaterialError::InvalidCiphertext(err.msg))
                }
                "InvalidImportTokenException" => {
                    return RusotoError::Service(ImportKeyMaterialError::InvalidImportToken(
                        err.msg,
                    ))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ImportKeyMaterialError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(ImportKeyMaterialError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ImportKeyMaterialError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(ImportKeyMaterialError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ImportKeyMaterialError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ImportKeyMaterialError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            ImportKeyMaterialError::ExpiredImportToken(ref cause) => write!(f, "{}", cause),
            ImportKeyMaterialError::IncorrectKeyMaterial(ref cause) => write!(f, "{}", cause),
            ImportKeyMaterialError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ImportKeyMaterialError::InvalidCiphertext(ref cause) => write!(f, "{}", cause),
            ImportKeyMaterialError::InvalidImportToken(ref cause) => write!(f, "{}", cause),
            ImportKeyMaterialError::KMSInternal(ref cause) => write!(f, "{}", cause),
            ImportKeyMaterialError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            ImportKeyMaterialError::NotFound(ref cause) => write!(f, "{}", cause),
            ImportKeyMaterialError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ImportKeyMaterialError {}
/// Errors returned by ListAliases
#[derive(Debug, PartialEq)]
pub enum ListAliasesError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ListAliasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAliasesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ListAliasesError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListAliasesError::InvalidArn(err.msg))
                }
                "InvalidMarkerException" => {
                    return RusotoError::Service(ListAliasesError::InvalidMarker(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ListAliasesError::KMSInternal(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListAliasesError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAliasesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAliasesError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            ListAliasesError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListAliasesError::InvalidMarker(ref cause) => write!(f, "{}", cause),
            ListAliasesError::KMSInternal(ref cause) => write!(f, "{}", cause),
            ListAliasesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAliasesError {}
/// Errors returned by ListGrants
#[derive(Debug, PartialEq)]
pub enum ListGrantsError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ListGrantsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListGrantsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ListGrantsError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListGrantsError::InvalidArn(err.msg))
                }
                "InvalidMarkerException" => {
                    return RusotoError::Service(ListGrantsError::InvalidMarker(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ListGrantsError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(ListGrantsError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListGrantsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListGrantsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListGrantsError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            ListGrantsError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListGrantsError::InvalidMarker(ref cause) => write!(f, "{}", cause),
            ListGrantsError::KMSInternal(ref cause) => write!(f, "{}", cause),
            ListGrantsError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            ListGrantsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListGrantsError {}
/// Errors returned by ListKeyPolicies
#[derive(Debug, PartialEq)]
pub enum ListKeyPoliciesError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ListKeyPoliciesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListKeyPoliciesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ListKeyPoliciesError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListKeyPoliciesError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ListKeyPoliciesError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(ListKeyPoliciesError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListKeyPoliciesError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListKeyPoliciesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListKeyPoliciesError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            ListKeyPoliciesError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListKeyPoliciesError::KMSInternal(ref cause) => write!(f, "{}", cause),
            ListKeyPoliciesError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            ListKeyPoliciesError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListKeyPoliciesError {}
/// Errors returned by ListKeys
#[derive(Debug, PartialEq)]
pub enum ListKeysError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl ListKeysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListKeysError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ListKeysError::DependencyTimeout(err.msg))
                }
                "InvalidMarkerException" => {
                    return RusotoError::Service(ListKeysError::InvalidMarker(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ListKeysError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListKeysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListKeysError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            ListKeysError::InvalidMarker(ref cause) => write!(f, "{}", cause),
            ListKeysError::KMSInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListKeysError {}
/// Errors returned by ListResourceTags
#[derive(Debug, PartialEq)]
pub enum ListResourceTagsError {
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ListResourceTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListResourceTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(ListResourceTagsError::InvalidArn(err.msg))
                }
                "InvalidMarkerException" => {
                    return RusotoError::Service(ListResourceTagsError::InvalidMarker(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ListResourceTagsError::KMSInternal(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListResourceTagsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListResourceTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListResourceTagsError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListResourceTagsError::InvalidMarker(ref cause) => write!(f, "{}", cause),
            ListResourceTagsError::KMSInternal(ref cause) => write!(f, "{}", cause),
            ListResourceTagsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListResourceTagsError {}
/// Errors returned by ListRetirableGrants
#[derive(Debug, PartialEq)]
pub enum ListRetirableGrantsError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the marker that specifies where pagination should next begin is not valid.</p>
    InvalidMarker(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ListRetirableGrantsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRetirableGrantsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ListRetirableGrantsError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ListRetirableGrantsError::InvalidArn(err.msg))
                }
                "InvalidMarkerException" => {
                    return RusotoError::Service(ListRetirableGrantsError::InvalidMarker(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ListRetirableGrantsError::KMSInternal(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListRetirableGrantsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListRetirableGrantsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRetirableGrantsError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            ListRetirableGrantsError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ListRetirableGrantsError::InvalidMarker(ref cause) => write!(f, "{}", cause),
            ListRetirableGrantsError::KMSInternal(ref cause) => write!(f, "{}", cause),
            ListRetirableGrantsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRetirableGrantsError {}
/// Errors returned by PutKeyPolicy
#[derive(Debug, PartialEq)]
pub enum PutKeyPolicyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a limit was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified policy is not syntactically or semantically correct.</p>
    MalformedPolicyDocument(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation.</p>
    UnsupportedOperation(String),
}

impl PutKeyPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutKeyPolicyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(PutKeyPolicyError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(PutKeyPolicyError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(PutKeyPolicyError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(PutKeyPolicyError::KMSInvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutKeyPolicyError::LimitExceeded(err.msg))
                }
                "MalformedPolicyDocumentException" => {
                    return RusotoError::Service(PutKeyPolicyError::MalformedPolicyDocument(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutKeyPolicyError::NotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(PutKeyPolicyError::UnsupportedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutKeyPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutKeyPolicyError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            PutKeyPolicyError::InvalidArn(ref cause) => write!(f, "{}", cause),
            PutKeyPolicyError::KMSInternal(ref cause) => write!(f, "{}", cause),
            PutKeyPolicyError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            PutKeyPolicyError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            PutKeyPolicyError::MalformedPolicyDocument(ref cause) => write!(f, "{}", cause),
            PutKeyPolicyError::NotFound(ref cause) => write!(f, "{}", cause),
            PutKeyPolicyError::UnsupportedOperation(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutKeyPolicyError {}
/// Errors returned by ReEncrypt
#[derive(Debug, PartialEq)]
pub enum ReEncryptError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified CMK cannot decrypt the data. The <code>KeyId</code> in a <a>Decrypt</a> request and the <code>SourceKeyId</code> in a <a>ReEncrypt</a> request must identify the same CMK that was used to encrypt the ciphertext.</p>
    IncorrectKey(String),
    /// <p>From the <a>Decrypt</a> or <a>ReEncrypt</a> operation, the request was rejected because the specified ciphertext, or additional authenticated data incorporated into the ciphertext, such as the encryption context, is corrupted, missing, or otherwise invalid.</p> <p>From the <a>ImportKeyMaterial</a> operation, the request was rejected because AWS KMS could not decrypt the encrypted (wrapped) key material. </p>
    InvalidCiphertext(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected for one of the following reasons: </p> <ul> <li> <p>The <code>KeyUsage</code> value of the CMK is incompatible with the API operation.</p> </li> <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the CMK <code>(CustomerMasterKeySpec</code>).</p> </li> </ul> <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To find the encryption or signing algorithms supported for a particular CMK, use the <a>DescribeKey</a> operation.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified CMK was not available. You can retry the request.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ReEncryptError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ReEncryptError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ReEncryptError::DependencyTimeout(err.msg))
                }
                "DisabledException" => {
                    return RusotoError::Service(ReEncryptError::Disabled(err.msg))
                }
                "IncorrectKeyException" => {
                    return RusotoError::Service(ReEncryptError::IncorrectKey(err.msg))
                }
                "InvalidCiphertextException" => {
                    return RusotoError::Service(ReEncryptError::InvalidCiphertext(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(ReEncryptError::InvalidGrantToken(err.msg))
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(ReEncryptError::InvalidKeyUsage(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ReEncryptError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(ReEncryptError::KMSInvalidState(err.msg))
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(ReEncryptError::KeyUnavailable(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ReEncryptError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ReEncryptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReEncryptError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            ReEncryptError::Disabled(ref cause) => write!(f, "{}", cause),
            ReEncryptError::IncorrectKey(ref cause) => write!(f, "{}", cause),
            ReEncryptError::InvalidCiphertext(ref cause) => write!(f, "{}", cause),
            ReEncryptError::InvalidGrantToken(ref cause) => write!(f, "{}", cause),
            ReEncryptError::InvalidKeyUsage(ref cause) => write!(f, "{}", cause),
            ReEncryptError::KMSInternal(ref cause) => write!(f, "{}", cause),
            ReEncryptError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            ReEncryptError::KeyUnavailable(ref cause) => write!(f, "{}", cause),
            ReEncryptError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ReEncryptError {}
/// Errors returned by RetireGrant
#[derive(Debug, PartialEq)]
pub enum RetireGrantError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the specified <code>GrantId</code> is not valid.</p>
    InvalidGrantId(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl RetireGrantError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RetireGrantError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(RetireGrantError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(RetireGrantError::InvalidArn(err.msg))
                }
                "InvalidGrantIdException" => {
                    return RusotoError::Service(RetireGrantError::InvalidGrantId(err.msg))
                }
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(RetireGrantError::InvalidGrantToken(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(RetireGrantError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(RetireGrantError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RetireGrantError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RetireGrantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RetireGrantError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            RetireGrantError::InvalidArn(ref cause) => write!(f, "{}", cause),
            RetireGrantError::InvalidGrantId(ref cause) => write!(f, "{}", cause),
            RetireGrantError::InvalidGrantToken(ref cause) => write!(f, "{}", cause),
            RetireGrantError::KMSInternal(ref cause) => write!(f, "{}", cause),
            RetireGrantError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            RetireGrantError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RetireGrantError {}
/// Errors returned by RevokeGrant
#[derive(Debug, PartialEq)]
pub enum RevokeGrantError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because the specified <code>GrantId</code> is not valid.</p>
    InvalidGrantId(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl RevokeGrantError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RevokeGrantError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(RevokeGrantError::DependencyTimeout(err.msg))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(RevokeGrantError::InvalidArn(err.msg))
                }
                "InvalidGrantIdException" => {
                    return RusotoError::Service(RevokeGrantError::InvalidGrantId(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(RevokeGrantError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(RevokeGrantError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RevokeGrantError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RevokeGrantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RevokeGrantError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            RevokeGrantError::InvalidArn(ref cause) => write!(f, "{}", cause),
            RevokeGrantError::InvalidGrantId(ref cause) => write!(f, "{}", cause),
            RevokeGrantError::KMSInternal(ref cause) => write!(f, "{}", cause),
            RevokeGrantError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            RevokeGrantError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RevokeGrantError {}
/// Errors returned by ScheduleKeyDeletion
#[derive(Debug, PartialEq)]
pub enum ScheduleKeyDeletionError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl ScheduleKeyDeletionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ScheduleKeyDeletionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(ScheduleKeyDeletionError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(ScheduleKeyDeletionError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(ScheduleKeyDeletionError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(ScheduleKeyDeletionError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ScheduleKeyDeletionError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ScheduleKeyDeletionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScheduleKeyDeletionError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            ScheduleKeyDeletionError::InvalidArn(ref cause) => write!(f, "{}", cause),
            ScheduleKeyDeletionError::KMSInternal(ref cause) => write!(f, "{}", cause),
            ScheduleKeyDeletionError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            ScheduleKeyDeletionError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ScheduleKeyDeletionError {}
/// Errors returned by Sign
#[derive(Debug, PartialEq)]
pub enum SignError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected for one of the following reasons: </p> <ul> <li> <p>The <code>KeyUsage</code> value of the CMK is incompatible with the API operation.</p> </li> <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the CMK <code>(CustomerMasterKeySpec</code>).</p> </li> </ul> <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To find the encryption or signing algorithms supported for a particular CMK, use the <a>DescribeKey</a> operation.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the specified CMK was not available. You can retry the request.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl SignError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SignError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(SignError::DependencyTimeout(err.msg))
                }
                "DisabledException" => return RusotoError::Service(SignError::Disabled(err.msg)),
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(SignError::InvalidGrantToken(err.msg))
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(SignError::InvalidKeyUsage(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(SignError::KMSInternal(err.msg))
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(SignError::KeyUnavailable(err.msg))
                }
                "NotFoundException" => return RusotoError::Service(SignError::NotFound(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SignError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SignError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            SignError::Disabled(ref cause) => write!(f, "{}", cause),
            SignError::InvalidGrantToken(ref cause) => write!(f, "{}", cause),
            SignError::InvalidKeyUsage(ref cause) => write!(f, "{}", cause),
            SignError::KMSInternal(ref cause) => write!(f, "{}", cause),
            SignError::KeyUnavailable(ref cause) => write!(f, "{}", cause),
            SignError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SignError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because a limit was exceeded. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html">Limits</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because one or more tags are not valid.</p>
    Tag(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(TagResourceError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(TagResourceError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(TagResourceError::KMSInvalidState(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(TagResourceError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
                }
                "TagException" => return RusotoError::Service(TagResourceError::Tag(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::InvalidArn(ref cause) => write!(f, "{}", cause),
            TagResourceError::KMSInternal(ref cause) => write!(f, "{}", cause),
            TagResourceError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            TagResourceError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            TagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::Tag(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
    /// <p>The request was rejected because one or more tags are not valid.</p>
    Tag(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArnException" => {
                    return RusotoError::Service(UntagResourceError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(UntagResourceError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(UntagResourceError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
                }
                "TagException" => return RusotoError::Service(UntagResourceError::Tag(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::InvalidArn(ref cause) => write!(f, "{}", cause),
            UntagResourceError::KMSInternal(ref cause) => write!(f, "{}", cause),
            UntagResourceError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Tag(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateAlias
#[derive(Debug, PartialEq)]
pub enum UpdateAliasError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl UpdateAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAliasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(UpdateAliasError::DependencyTimeout(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(UpdateAliasError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(UpdateAliasError::KMSInvalidState(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateAliasError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAliasError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            UpdateAliasError::KMSInternal(ref cause) => write!(f, "{}", cause),
            UpdateAliasError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            UpdateAliasError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAliasError {}
/// Errors returned by UpdateCustomKeyStore
#[derive(Debug, PartialEq)]
pub enum UpdateCustomKeyStoreError {
    /// <p>The request was rejected because the associated AWS CloudHSM cluster did not meet the configuration requirements for a custom key store.</p> <ul> <li> <p>The cluster must be configured with private subnets in at least two different Availability Zones in the Region.</p> </li> <li> <p>The <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">security group for the cluster</a> (cloudhsm-cluster-<i>&lt;cluster-id&gt;</i>-sg) must include inbound rules and outbound rules that allow TCP traffic on ports 2223-2225. The <b>Source</b> in the inbound rules and the <b>Destination</b> in the outbound rules must match the security group ID. These rules are set by default when you create the cluster. Do not delete or change them. To get information about a particular security group, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSecurityGroups.html">DescribeSecurityGroups</a> operation.</p> </li> <li> <p>The cluster must contain at least as many HSMs as the operation requires. To add HSMs, use the AWS CloudHSM <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_CreateHsm.html">CreateHsm</a> operation.</p> <p>For the <a>CreateCustomKeyStore</a>, <a>UpdateCustomKeyStore</a>, and <a>CreateKey</a> operations, the AWS CloudHSM cluster must have at least two active HSMs, each in a different Availability Zone. For the <a>ConnectCustomKeyStore</a> operation, the AWS CloudHSM must contain at least one active HSM.</p> </li> </ul> <p>For information about the requirements for an AWS CloudHSM cluster that is associated with a custom key store, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">Assemble the Prerequisites</a> in the <i>AWS Key Management Service Developer Guide</i>. For information about creating a private subnet for an AWS CloudHSM cluster, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/create-subnets.html">Create a Private Subnet</a> in the <i>AWS CloudHSM User Guide</i>. For information about cluster security groups, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">Configure a Default Security Group</a> in the <i> <i>AWS CloudHSM User Guide</i> </i>. </p>
    CloudHsmClusterInvalidConfiguration(String),
    /// <p>The request was rejected because the AWS CloudHSM cluster that is associated with the custom key store is not active. Initialize and activate the cluster and try the command again. For detailed instructions, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/getting-started.html">Getting Started</a> in the <i>AWS CloudHSM User Guide</i>.</p>
    CloudHsmClusterNotActive(String),
    /// <p>The request was rejected because AWS KMS cannot find the AWS CloudHSM cluster with the specified cluster ID. Retry the request with a different cluster ID.</p>
    CloudHsmClusterNotFound(String),
    /// <p>The request was rejected because the specified AWS CloudHSM cluster has a different cluster certificate than the original cluster. You cannot use the operation to specify an unrelated cluster.</p> <p>Specify a cluster that shares a backup history with the original cluster. This includes clusters that were created from a backup of the current cluster, and clusters that were created from the same backup that produced the current cluster.</p> <p>Clusters that share a backup history have the same cluster certificate. To view the cluster certificate of a cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation.</p>
    CloudHsmClusterNotRelated(String),
    /// <p><p>The request was rejected because of the <code>ConnectionState</code> of the custom key store. To get the <code>ConnectionState</code> of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>This exception is thrown under the following conditions:</p> <ul> <li> <p>You requested the <a>CreateKey</a> or <a>GenerateRandom</a> operation in a custom key store that is not connected. These operations are valid only when the custom key store <code>ConnectionState</code> is <code>CONNECTED</code>.</p> </li> <li> <p>You requested the <a>UpdateCustomKeyStore</a> or <a>DeleteCustomKeyStore</a> operation on a custom key store that is not disconnected. This operation is valid only when the custom key store <code>ConnectionState</code> is <code>DISCONNECTED</code>.</p> </li> <li> <p>You requested the <a>ConnectCustomKeyStore</a> operation on a custom key store with a <code>ConnectionState</code> of <code>DISCONNECTING</code> or <code>FAILED</code>. This operation is valid for all other <code>ConnectionState</code> values.</p> </li> </ul></p>
    CustomKeyStoreInvalidState(String),
    /// <p>The request was rejected because the specified custom key store name is already assigned to another custom key store in the account. Try again with a custom key store name that is unique in the account.</p>
    CustomKeyStoreNameInUse(String),
    /// <p>The request was rejected because AWS KMS cannot find a custom key store with the specified key store name or ID.</p>
    CustomKeyStoreNotFound(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
}

impl UpdateCustomKeyStoreError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateCustomKeyStoreError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmClusterInvalidConfigurationException" => {
                    return RusotoError::Service(
                        UpdateCustomKeyStoreError::CloudHsmClusterInvalidConfiguration(err.msg),
                    )
                }
                "CloudHsmClusterNotActiveException" => {
                    return RusotoError::Service(
                        UpdateCustomKeyStoreError::CloudHsmClusterNotActive(err.msg),
                    )
                }
                "CloudHsmClusterNotFoundException" => {
                    return RusotoError::Service(
                        UpdateCustomKeyStoreError::CloudHsmClusterNotFound(err.msg),
                    )
                }
                "CloudHsmClusterNotRelatedException" => {
                    return RusotoError::Service(
                        UpdateCustomKeyStoreError::CloudHsmClusterNotRelated(err.msg),
                    )
                }
                "CustomKeyStoreInvalidStateException" => {
                    return RusotoError::Service(
                        UpdateCustomKeyStoreError::CustomKeyStoreInvalidState(err.msg),
                    )
                }
                "CustomKeyStoreNameInUseException" => {
                    return RusotoError::Service(
                        UpdateCustomKeyStoreError::CustomKeyStoreNameInUse(err.msg),
                    )
                }
                "CustomKeyStoreNotFoundException" => {
                    return RusotoError::Service(UpdateCustomKeyStoreError::CustomKeyStoreNotFound(
                        err.msg,
                    ))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(UpdateCustomKeyStoreError::KMSInternal(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateCustomKeyStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateCustomKeyStoreError::CloudHsmClusterInvalidConfiguration(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCustomKeyStoreError::CloudHsmClusterNotActive(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCustomKeyStoreError::CloudHsmClusterNotFound(ref cause) => write!(f, "{}", cause),
            UpdateCustomKeyStoreError::CloudHsmClusterNotRelated(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCustomKeyStoreError::CustomKeyStoreInvalidState(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCustomKeyStoreError::CustomKeyStoreNameInUse(ref cause) => write!(f, "{}", cause),
            UpdateCustomKeyStoreError::CustomKeyStoreNotFound(ref cause) => write!(f, "{}", cause),
            UpdateCustomKeyStoreError::KMSInternal(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateCustomKeyStoreError {}
/// Errors returned by UpdateKeyDescription
#[derive(Debug, PartialEq)]
pub enum UpdateKeyDescriptionError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because a specified ARN, or an ARN in a key policy, is not valid.</p>
    InvalidArn(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the state of the specified resource is not valid for this request.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p>
    KMSInvalidState(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl UpdateKeyDescriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateKeyDescriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(UpdateKeyDescriptionError::DependencyTimeout(
                        err.msg,
                    ))
                }
                "InvalidArnException" => {
                    return RusotoError::Service(UpdateKeyDescriptionError::InvalidArn(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(UpdateKeyDescriptionError::KMSInternal(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(UpdateKeyDescriptionError::KMSInvalidState(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateKeyDescriptionError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateKeyDescriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateKeyDescriptionError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            UpdateKeyDescriptionError::InvalidArn(ref cause) => write!(f, "{}", cause),
            UpdateKeyDescriptionError::KMSInternal(ref cause) => write!(f, "{}", cause),
            UpdateKeyDescriptionError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            UpdateKeyDescriptionError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateKeyDescriptionError {}
/// Errors returned by Verify
#[derive(Debug, PartialEq)]
pub enum VerifyError {
    /// <p>The system timed out while trying to fulfill the request. The request can be retried.</p>
    DependencyTimeout(String),
    /// <p>The request was rejected because the specified CMK is not enabled.</p>
    Disabled(String),
    /// <p>The request was rejected because the specified grant token is not valid.</p>
    InvalidGrantToken(String),
    /// <p>The request was rejected for one of the following reasons: </p> <ul> <li> <p>The <code>KeyUsage</code> value of the CMK is incompatible with the API operation.</p> </li> <li> <p>The encryption algorithm or signing algorithm specified for the operation is incompatible with the type of key material in the CMK <code>(CustomerMasterKeySpec</code>).</p> </li> </ul> <p>For encrypting, decrypting, re-encrypting, and generating data keys, the <code>KeyUsage</code> must be <code>ENCRYPT_DECRYPT</code>. For signing and verifying, the <code>KeyUsage</code> must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation.</p> <p>To find the encryption or signing algorithms supported for a particular CMK, use the <a>DescribeKey</a> operation.</p>
    InvalidKeyUsage(String),
    /// <p>The request was rejected because an internal exception occurred. The request can be retried.</p>
    KMSInternal(String),
    /// <p>The request was rejected because the specified CMK was not available. You can retry the request.</p>
    KeyUnavailable(String),
    /// <p>The request was rejected because the specified entity or resource could not be found.</p>
    NotFound(String),
}

impl VerifyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<VerifyError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "DependencyTimeoutException" => {
                    return RusotoError::Service(VerifyError::DependencyTimeout(err.msg))
                }
                "DisabledException" => return RusotoError::Service(VerifyError::Disabled(err.msg)),
                "InvalidGrantTokenException" => {
                    return RusotoError::Service(VerifyError::InvalidGrantToken(err.msg))
                }
                "InvalidKeyUsageException" => {
                    return RusotoError::Service(VerifyError::InvalidKeyUsage(err.msg))
                }
                "KMSInternalException" => {
                    return RusotoError::Service(VerifyError::KMSInternal(err.msg))
                }
                "KeyUnavailableException" => {
                    return RusotoError::Service(VerifyError::KeyUnavailable(err.msg))
                }
                "NotFoundException" => return RusotoError::Service(VerifyError::NotFound(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for VerifyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VerifyError::DependencyTimeout(ref cause) => write!(f, "{}", cause),
            VerifyError::Disabled(ref cause) => write!(f, "{}", cause),
            VerifyError::InvalidGrantToken(ref cause) => write!(f, "{}", cause),
            VerifyError::InvalidKeyUsage(ref cause) => write!(f, "{}", cause),
            VerifyError::KMSInternal(ref cause) => write!(f, "{}", cause),
            VerifyError::KeyUnavailable(ref cause) => write!(f, "{}", cause),
            VerifyError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for VerifyError {}
/// Trait representing the capabilities of the KMS API. KMS clients implement this trait.
pub trait Kms {
    /// <p>Cancels the deletion of a customer master key (CMK). When this operation succeeds, the key state of the CMK is <code>Disabled</code>. To enable the CMK, use <a>EnableKey</a>. You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about scheduling and canceling deletion of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html">Deleting Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn cancel_key_deletion(
        &self,
        input: CancelKeyDeletionRequest,
    ) -> RusotoFuture<CancelKeyDeletionResponse, CancelKeyDeletionError>;

    /// <p>Connects or reconnects a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> to its associated AWS CloudHSM cluster.</p> <p>The custom key store must be connected before you can create customer master keys (CMKs) in the key store or use the CMKs it contains. You can disconnect and reconnect a custom key store at any time.</p> <p>To connect a custom key store, its associated AWS CloudHSM cluster must have at least one active HSM. To get the number of active HSMs in a cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation. To add HSMs to the cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_CreateHsm.html">CreateHsm</a> operation.</p> <p>The connection process can take an extended amount of time to complete; up to 20 minutes. This operation starts the connection process, but it does not wait for it to complete. When it succeeds, this operation quickly returns an HTTP 200 response and a JSON object with no properties. However, this response does not indicate that the custom key store is connected. To get the connection state of the custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>During the connection process, AWS KMS finds the AWS CloudHSM cluster that is associated with the custom key store, creates the connection infrastructure, connects to the cluster, logs into the AWS CloudHSM client as the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-concepts.html#concept-kmsuser"> <code>kmsuser</code> crypto user</a> (CU), and rotates its password.</p> <p>The <code>ConnectCustomKeyStore</code> operation might fail for various reasons. To find the reason, use the <a>DescribeCustomKeyStores</a> operation and see the <code>ConnectionErrorCode</code> in the response. For help interpreting the <code>ConnectionErrorCode</code>, see <a>CustomKeyStoresListEntry</a>.</p> <p>To fix the failure, use the <a>DisconnectCustomKeyStore</a> operation to disconnect the custom key store, correct the error, use the <a>UpdateCustomKeyStore</a> operation if necessary, and then use <code>ConnectCustomKeyStore</code> again.</p> <p>If you are having trouble connecting or disconnecting a custom key store, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting a Custom Key Store</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn connect_custom_key_store(
        &self,
        input: ConnectCustomKeyStoreRequest,
    ) -> RusotoFuture<ConnectCustomKeyStoreResponse, ConnectCustomKeyStoreError>;

    /// <p>Creates a display name for a customer managed customer master key (CMK). You can use an alias to identify a CMK in cryptographic operations, such as <a>Encrypt</a> and <a>GenerateDataKey</a>. You can change the CMK associated with the alias at any time.</p> <p>Aliases are easier to remember than key IDs. They can also help to simplify your applications. For example, if you use an alias in your code, you can change the CMK your code uses by associating a given alias with a different CMK. </p> <p>To run the same code in multiple AWS regions, use an alias in your code, such as <code>alias/ApplicationKey</code>. Then, in each AWS Region, create an <code>alias/ApplicationKey</code> alias that is associated with a CMK in that Region. When you run your code, it uses the <code>alias/ApplicationKey</code> CMK for that AWS Region without any Region-specific code.</p> <p>This operation does not return a response. To get the alias that you created, use the <a>ListAliases</a> operation.</p> <p>To use aliases successfully, be aware of the following information.</p> <ul> <li> <p>Each alias points to only one CMK at a time, although a single CMK can have multiple aliases. The alias and its associated CMK must be in the same AWS account and Region. </p> </li> <li> <p>You can associate an alias with any customer managed CMK in the same AWS account and Region. However, you do not have permission to associate an alias with an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">AWS managed CMK</a> or an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-owned-cmk">AWS owned CMK</a>. </p> </li> <li> <p>To change the CMK associated with an alias, use the <a>UpdateAlias</a> operation. The current CMK and the new CMK must be the same type (both symmetric or both asymmetric) and they must have the same key usage (<code>ENCRYPT_DECRYPT</code> or <code>SIGN_VERIFY</code>). This restriction prevents cryptographic errors in code that uses aliases.</p> </li> <li> <p>The alias name must begin with <code>alias/</code> followed by a name, such as <code>alias/ExampleAlias</code>. It can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). The alias name cannot begin with <code>alias/aws/</code>. The <code>alias/aws/</code> prefix is reserved for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">AWS managed CMKs</a>. </p> </li> <li> <p>The alias name must be unique within an AWS Region. However, you can use the same alias name in multiple Regions of the same AWS account. Each instance of the alias is associated with a CMK in its Region.</p> </li> <li> <p>After you create an alias, you cannot change its alias name. However, you can use the <a>DeleteAlias</a> operation to delete the alias and then create a new alias with the desired name.</p> </li> <li> <p>You can use an alias name or alias ARN to identify a CMK in AWS KMS cryptographic operations and in the <a>DescribeKey</a> operation. However, you cannot use alias names or alias ARNs in API operations that manage CMKs, such as <a>DisableKey</a> or <a>GetKeyPolicy</a>. For information about the valid CMK identifiers for each AWS KMS API operation, see the descriptions of the <code>KeyId</code> parameter in the API operation documentation.</p> </li> </ul> <p>Because an alias is not a property of a CMK, you can delete and change the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases and alias ARNs of CMKs in each AWS account and Region, use the <a>ListAliases</a> operation.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn create_alias(&self, input: CreateAliasRequest) -> RusotoFuture<(), CreateAliasError>;

    /// <p>Creates a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> that is associated with an <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/clusters.html">AWS CloudHSM cluster</a> that you own and manage.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p> <p>Before you create the custom key store, you must assemble the required elements, including an AWS CloudHSM cluster that fulfills the requirements for a custom key store. For details about the required elements, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">Assemble the Prerequisites</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>When the operation completes successfully, it returns the ID of the new custom key store. Before you can use your new custom key store, you need to use the <a>ConnectCustomKeyStore</a> operation to connect the new key store to its AWS CloudHSM cluster. Even if you are not going to use your custom key store immediately, you might want to connect it to verify that all settings are correct and then disconnect it until you are ready to use it.</p> <p>For help with failures, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting a Custom Key Store</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn create_custom_key_store(
        &self,
        input: CreateCustomKeyStoreRequest,
    ) -> RusotoFuture<CreateCustomKeyStoreResponse, CreateCustomKeyStoreError>;

    /// <p>Adds a grant to a customer master key (CMK). The grant allows the grantee principal to use the CMK when the conditions specified in the grant are met. When setting permissions, grants are an alternative to key policies. </p> <p>To create a grant that allows a cryptographic operation only when the request includes a particular <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">encryption context</a>, use the <code>Constraints</code> parameter. For details, see <a>GrantConstraints</a>.</p> <p>You can create grants on symmetric and asymmetric CMKs. However, if the grant allows an operation that the CMK does not support, <code>CreateGrant</code> fails with a <code>ValidationException</code>. </p> <ul> <li> <p>Grants for symmetric CMKs cannot allow operations that are not supported for symmetric CMKs, including <a>Sign</a>, <a>Verify</a>, and <a>GetPublicKey</a>. (There are limited exceptions to this rule for legacy operations, but you should not create a grant for an operation that AWS KMS does not support.)</p> </li> <li> <p>Grants for asymmetric CMKs cannot allow operations that are not supported for asymmetric CMKs, including operations that <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_GenerateDataKey">generate data keys</a> or <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_GenerateDataKeyPair">data key pairs</a>, or operations related to <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic key rotation</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or CMKs in <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key stores</a>.</p> </li> <li> <p>Grants for asymmetric CMKs with a <code>KeyUsage</code> of <code>ENCRYPT_DECRYPT</code> cannot allow the <a>Sign</a> or <a>Verify</a> operations. Grants for asymmetric CMKs with a <code>KeyUsage</code> of <code>SIGN_VERIFY</code> cannot allow the <a>Encrypt</a> or <a>Decrypt</a> operations.</p> </li> <li> <p>Grants for asymmetric CMKs cannot include an encryption context grant constraint. An encryption context is not supported on asymmetric CMKs.</p> </li> </ul> <p>For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter. For more information about grants, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html">Grants</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn create_grant(
        &self,
        input: CreateGrantRequest,
    ) -> RusotoFuture<CreateGrantResponse, CreateGrantError>;

    /// <p><p>Creates a unique customer managed <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master-keys">customer master key</a> (CMK) in your AWS account and Region. You cannot use this operation to create a CMK in a different AWS account.</p> <p>You can use the <code>CreateKey</code> operation to create symmetric or asymmetric CMKs.</p> <ul> <li> <p> <b>Symmetric CMKs</b> contain a 256-bit symmetric key that never leaves AWS KMS unencrypted. To use the CMK, you must call AWS KMS. You can use a symmetric CMK to encrypt and decrypt small amounts of data, but they are typically used to generate <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#data-keys">data keys</a> or data key pairs. For details, see <a>GenerateDataKey</a> and <a>GenerateDataKeyPair</a>.</p> </li> <li> <p> <b>Asymmetric CMKs</b> can contain an RSA key pair or an Elliptic Curve (ECC) key pair. The private key in an asymmetric CMK never leaves AWS KMS unencrypted. However, you can use the <a>GetPublicKey</a> operation to download the public key so it can be used outside of AWS KMS. CMKs with RSA key pairs can be used to encrypt or decrypt data or sign and verify messages (but not both). CMKs with ECC key pairs can be used only to sign and verify messages.</p> </li> </ul> <p>For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>To create different types of CMKs, use the following guidance:</p> <dl> <dt>Asymmetric CMKs</dt> <dd> <p>To create an asymmetric CMK, use the <code>CustomerMasterKeySpec</code> parameter to specify the type of key material in the CMK. Then, use the <code>KeyUsage</code> parameter to determine whether the CMK will be used to encrypt and decrypt or sign and verify. You can&#39;t change these properties after the CMK is created.</p> <p> </p> </dd> <dt>Symmetric CMKs</dt> <dd> <p>When creating a symmetric CMK, you don&#39;t need to specify the <code>CustomerMasterKeySpec</code> or <code>KeyUsage</code> parameters. The default value for <code>CustomerMasterKeySpec</code>, <code>SYMMETRIC<em>DEFAULT</code>, and the default value for <code>KeyUsage</code>, <code>ENCRYPT</em>DECRYPT</code>, are the only valid values for symmetric CMKs. </p> <p> </p> </dd> <dt>Imported Key Material</dt> <dd> <p>To import your own key material, begin by creating a symmetric CMK with no key material. To do this, use the <code>Origin</code> parameter of <code>CreateKey</code> with a value of <code>EXTERNAL</code>. Next, use <a>GetParametersForImport</a> operation to get a public key and import token, and use the public key to encrypt your key material. Then, use <a>ImportKeyMaterial</a> with your import token to import the key material. For step-by-step instructions, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>. You cannot import the key material into an asymmetric CMK.</p> <p> </p> </dd> <dt>Custom Key Stores</dt> <dd> <p>To create a symmetric CMK in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, use the <code>CustomKeyStoreId</code> parameter to specify the custom key store. You must also use the <code>Origin</code> parameter with a value of <code>AWS_CLOUDHSM</code>. The AWS CloudHSM cluster that is associated with the custom key store must have at least two active HSMs in different Availability Zones in the AWS Region. </p> <p>You cannot create an asymmetric CMK in a custom key store. For information about custom key stores in AWS KMS see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Using Custom Key Stores</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> </dd> </dl></p>
    fn create_key(
        &self,
        input: CreateKeyRequest,
    ) -> RusotoFuture<CreateKeyResponse, CreateKeyError>;

    /// <p>Decrypts ciphertext that was encrypted by a AWS KMS customer master key (CMK) using any of the following operations:</p> <ul> <li> <p> <a>Encrypt</a> </p> </li> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyPair</a> </p> </li> <li> <p> <a>GenerateDataKeyWithoutPlaintext</a> </p> </li> <li> <p> <a>GenerateDataKeyPairWithoutPlaintext</a> </p> </li> </ul> <p>You can use this operation to decrypt ciphertext that was encrypted under a symmetric or asymmetric CMK. When the CMK is asymmetric, you must specify the CMK and the encryption algorithm that was used to encrypt the ciphertext. For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The Decrypt operation also decrypts ciphertext that was encrypted outside of AWS KMS by the public key in an AWS KMS asymmetric CMK. However, it cannot decrypt ciphertext produced by other libraries, such as the <a href="https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/">AWS Encryption SDK</a> or <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingClientSideEncryption.html">Amazon S3 client-side encryption</a>. These libraries return a ciphertext format that is incompatible with AWS KMS.</p> <p>If the ciphertext was encrypted under a symmetric CMK, you do not need to specify the CMK or the encryption algorithm. AWS KMS can get this information from metadata that it adds to the symmetric ciphertext blob. However, if you prefer, you can specify the <code>KeyId</code> to ensure that a particular CMK is used to decrypt the ciphertext. If you specify a different CMK than the one used to encrypt the ciphertext, the <code>Decrypt</code> operation fails.</p> <p>Whenever possible, use key policies to give users permission to call the Decrypt operation on a particular CMK, instead of using IAM policies. Otherwise, you might create an IAM user policy that gives the user Decrypt permission on all CMKs. This user could decrypt ciphertext that was encrypted by CMKs in other accounts if the key policy for the cross-account CMK permits it. If you must use an IAM policy for <code>Decrypt</code> permissions, limit the user to particular CMKs or particular trusted accounts.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn decrypt(&self, input: DecryptRequest) -> RusotoFuture<DecryptResponse, DecryptError>;

    /// <p>Deletes the specified alias. You cannot perform this operation on an alias in a different AWS account. </p> <p>Because an alias is not a property of a CMK, you can delete and change the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases of all CMKs, use the <a>ListAliases</a> operation. </p> <p>Each CMK can have multiple aliases. To change the alias of a CMK, use <a>DeleteAlias</a> to delete the current alias and <a>CreateAlias</a> to create a new alias. To associate an existing alias with a different customer master key (CMK), call <a>UpdateAlias</a>.</p>
    fn delete_alias(&self, input: DeleteAliasRequest) -> RusotoFuture<(), DeleteAliasError>;

    /// <p>Deletes a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. This operation does not delete the AWS CloudHSM cluster that is associated with the custom key store, or affect any users or keys in the cluster.</p> <p>The custom key store that you delete cannot contain any AWS KMS <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">customer master keys (CMKs)</a>. Before deleting the key store, verify that you will never need to use any of the CMKs in the key store for any cryptographic operations. Then, use <a>ScheduleKeyDeletion</a> to delete the AWS KMS customer master keys (CMKs) from the key store. When the scheduled waiting period expires, the <code>ScheduleKeyDeletion</code> operation deletes the CMKs. Then it makes a best effort to delete the key material from the associated cluster. However, you might need to manually <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#fix-keystore-orphaned-key">delete the orphaned key material</a> from the cluster and its backups.</p> <p>After all CMKs are deleted from AWS KMS, use <a>DisconnectCustomKeyStore</a> to disconnect the key store from AWS KMS. Then, you can delete the custom key store.</p> <p>Instead of deleting the custom key store, consider using <a>DisconnectCustomKeyStore</a> to disconnect it from AWS KMS. While the key store is disconnected, you cannot create or use the CMKs in the key store. But, you do not need to delete CMKs and you can reconnect a disconnected custom key store at any time.</p> <p>If the operation succeeds, it returns a JSON object with no properties.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p>
    fn delete_custom_key_store(
        &self,
        input: DeleteCustomKeyStoreRequest,
    ) -> RusotoFuture<DeleteCustomKeyStoreResponse, DeleteCustomKeyStoreError>;

    /// <p>Deletes key material that you previously imported. This operation makes the specified customer master key (CMK) unusable. For more information about importing key material into AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>. You cannot perform this operation on a CMK in a different AWS account.</p> <p>When the specified CMK is in the <code>PendingDeletion</code> state, this operation does not change the CMK's state. Otherwise, it changes the CMK's state to <code>PendingImport</code>.</p> <p>After you delete key material, you can use <a>ImportKeyMaterial</a> to reimport the same key material into the CMK.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn delete_imported_key_material(
        &self,
        input: DeleteImportedKeyMaterialRequest,
    ) -> RusotoFuture<(), DeleteImportedKeyMaterialError>;

    /// <p>Gets information about <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key stores</a> in the account and region.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p> <p>By default, this operation returns information about all custom key stores in the account and region. To get only information about a particular custom key store, use either the <code>CustomKeyStoreName</code> or <code>CustomKeyStoreId</code> parameter (but not both).</p> <p>To determine whether the custom key store is connected to its AWS CloudHSM cluster, use the <code>ConnectionState</code> element in the response. If an attempt to connect the custom key store failed, the <code>ConnectionState</code> value is <code>FAILED</code> and the <code>ConnectionErrorCode</code> element in the response indicates the cause of the failure. For help interpreting the <code>ConnectionErrorCode</code>, see <a>CustomKeyStoresListEntry</a>.</p> <p>Custom key stores have a <code>DISCONNECTED</code> connection state if the key store has never been connected or you use the <a>DisconnectCustomKeyStore</a> operation to disconnect it. If your custom key store state is <code>CONNECTED</code> but you are having trouble using it, make sure that its associated AWS CloudHSM cluster is active and contains the minimum number of HSMs required for the operation, if any.</p> <p> For help repairing your custom key store, see the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting Custom Key Stores</a> topic in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn describe_custom_key_stores(
        &self,
        input: DescribeCustomKeyStoresRequest,
    ) -> RusotoFuture<DescribeCustomKeyStoresResponse, DescribeCustomKeyStoresError>;

    /// <p>Provides detailed information about a customer master key (CMK). You can run <code>DescribeKey</code> on a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-cmk">customer managed CMK</a> or an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">AWS managed CMK</a>.</p> <p>This detailed information includes the key ARN, creation date (and deletion date, if applicable), the key state, and the origin and expiration date (if any) of the key material. For CMKs in custom key stores, it includes information about the custom key store, such as the key store ID and the AWS CloudHSM cluster ID. It includes fields, like <code>KeySpec</code>, that help you distinguish symmetric from asymmetric CMKs. It also provides information that is particularly important to asymmetric CMKs, such as the key usage (encryption or signing) and the encryption algorithms or signing algorithms that the CMK supports.</p> <p> <code>DescribeKey</code> does not return the following information:</p> <ul> <li> <p>Aliases associated with the CMK. To get this information, use <a>ListAliases</a>.</p> </li> <li> <p>Whether automatic key rotation is enabled on the CMK. To get this information, use <a>GetKeyRotationStatus</a>. Also, some key states prevent a CMK from being automatically rotated. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#rotate-keys-how-it-works">How Automatic Key Rotation Works</a> in <i>AWS Key Management Service Developer Guide</i>.</p> </li> <li> <p>Tags on the CMK. To get this information, use <a>ListResourceTags</a>.</p> </li> <li> <p>Key policies and grants on the CMK. To get this information, use <a>GetKeyPolicy</a> and <a>ListGrants</a>.</p> </li> </ul> <p>If you call the <code>DescribeKey</code> operation on a <i>predefined AWS alias</i>, that is, an AWS alias with no key ID, AWS KMS creates an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">AWS managed CMK</a>. Then, it associates the alias with the new CMK, and returns the <code>KeyId</code> and <code>Arn</code> of the new CMK in the response.</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the KeyId parameter.</p>
    fn describe_key(
        &self,
        input: DescribeKeyRequest,
    ) -> RusotoFuture<DescribeKeyResponse, DescribeKeyError>;

    /// <p>Sets the state of a customer master key (CMK) to disabled, thereby preventing its use for cryptographic operations. You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects the Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn disable_key(&self, input: DisableKeyRequest) -> RusotoFuture<(), DisableKeyError>;

    /// <p>Disables <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic rotation of the key material</a> for the specified symmetric customer master key (CMK).</p> <p> You cannot enable automatic rotation of asymmetric CMKs, CMKs with imported key material, or CMKs in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. You cannot perform this operation on a CMK in a different AWS account.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn disable_key_rotation(
        &self,
        input: DisableKeyRotationRequest,
    ) -> RusotoFuture<(), DisableKeyRotationError>;

    /// <p>Disconnects the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> from its associated AWS CloudHSM cluster. While a custom key store is disconnected, you can manage the custom key store and its customer master keys (CMKs), but you cannot create or use CMKs in the custom key store. You can reconnect the custom key store at any time.</p> <note> <p>While a custom key store is disconnected, all attempts to create customer master keys (CMKs) in the custom key store or to use existing CMKs in cryptographic operations will fail. This action can prevent users from storing and accessing sensitive data.</p> </note> <p/> <p>To find the connection state of a custom key store, use the <a>DescribeCustomKeyStores</a> operation. To reconnect a custom key store, use the <a>ConnectCustomKeyStore</a> operation.</p> <p>If the operation succeeds, it returns a JSON object with no properties.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p>
    fn disconnect_custom_key_store(
        &self,
        input: DisconnectCustomKeyStoreRequest,
    ) -> RusotoFuture<DisconnectCustomKeyStoreResponse, DisconnectCustomKeyStoreError>;

    /// <p>Sets the key state of a customer master key (CMK) to enabled. This allows you to use the CMK for cryptographic operations. You cannot perform this operation on a CMK in a different AWS account.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn enable_key(&self, input: EnableKeyRequest) -> RusotoFuture<(), EnableKeyError>;

    /// <p>Enables <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic rotation of the key material</a> for the specified symmetric customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>You cannot enable automatic rotation of asymmetric CMKs, CMKs with imported key material, or CMKs in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn enable_key_rotation(
        &self,
        input: EnableKeyRotationRequest,
    ) -> RusotoFuture<(), EnableKeyRotationError>;

    /// <p>Encrypts plaintext into ciphertext by using a customer master key (CMK). The <code>Encrypt</code> operation has two primary use cases:</p> <ul> <li> <p>You can encrypt small amounts of arbitrary data, such as a personal identifier or database password, or other sensitive information. </p> </li> <li> <p>You can use the <code>Encrypt</code> operation to move encrypted data from one AWS region to another. In the first region, generate a data key and use the plaintext key to encrypt the data. Then, in the new region, call the <code>Encrypt</code> method on same plaintext data key. Now, you can safely move the encrypted data and encrypted data key to the new region, and decrypt in the new region when necessary.</p> </li> </ul> <p>You don't need to use the <code>Encrypt</code> operation to encrypt a data key. The <a>GenerateDataKey</a> and <a>GenerateDataKeyPair</a> operations return a plaintext data key and an encrypted copy of that data key.</p> <p>When you encrypt data, you must specify a symmetric or asymmetric CMK to use in the encryption operation. The CMK must have a <code>KeyUsage</code> value of <code>ENCRYPT_DECRYPT.</code> To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation. </p> <p>If you use a symmetric CMK, you can use an encryption context to add additional security to your encryption operation. If you specify an <code>EncryptionContext</code> when encrypting data, you must specify the same encryption context (a case-sensitive exact match) when decrypting the data. Otherwise, the request to decrypt fails with an <code>InvalidCiphertextException</code>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>If you specify an asymmetric CMK, you must also specify the encryption algorithm. The algorithm must be compatible with the CMK type.</p> <important> <p>When you use an asymmetric CMK to encrypt or reencrypt data, be sure to record the CMK and encryption algorithm that you choose. You will be required to provide the same CMK and encryption algorithm when you decrypt the data. If the CMK and algorithm do not match the values used to encrypt the data, the decrypt operation fails.</p> <p>You are not required to supply the CMK ID and encryption algorithm when you decrypt with symmetric CMKs because AWS KMS stores this information in the ciphertext blob. AWS KMS cannot store metadata in ciphertext generated with asymmetric keys. The standard format for asymmetric key ciphertext does not include configurable fields.</p> </important> <p>The maximum size of the data that you can encrypt varies with the type of CMK and the encryption algorithm that you choose.</p> <ul> <li> <p>Symmetric CMKs</p> <ul> <li> <p> <code>SYMMETRIC_DEFAULT</code>: 4096 bytes</p> </li> </ul> </li> <li> <p> <code>RSA_2048</code> </p> <ul> <li> <p> <code>RSAES_OAEP_SHA_1</code>: 214 bytes</p> </li> <li> <p> <code>RSAES_OAEP_SHA_256</code>: 190 bytes</p> </li> </ul> </li> <li> <p> <code>RSA_3072</code> </p> <ul> <li> <p> <code>RSAES_OAEP_SHA_1</code>: 342 bytes</p> </li> <li> <p> <code>RSAES_OAEP_SHA_256</code>: 318 bytes</p> </li> </ul> </li> <li> <p> <code>RSA_4096</code> </p> <ul> <li> <p> <code>RSAES_OAEP_SHA_1</code>: 470 bytes</p> </li> <li> <p> <code>RSAES_OAEP_SHA_256</code>: 446 bytes</p> </li> </ul> </li> </ul> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the KeyId parameter.</p>
    fn encrypt(&self, input: EncryptRequest) -> RusotoFuture<EncryptResponse, EncryptError>;

    /// <p><p>Generates a unique symmetric data key. This operation returns a plaintext copy of the data key and a copy that is encrypted under a customer master key (CMK) that you specify. You can use the plaintext key to encrypt your data outside of AWS KMS and store the encrypted data key with the encrypted data.</p> <p> <code>GenerateDataKey</code> returns a unique data key for each request. The bytes in the key are not related to the caller or CMK that is used to encrypt the data key.</p> <p>To generate a data key, specify the symmetric CMK that will be used to encrypt the data key. You cannot use an asymmetric CMK to generate data keys.</p> <p>You must also specify the length of the data key. Use either the <code>KeySpec</code> or <code>NumberOfBytes</code> parameters (but not both). For 128-bit and 256-bit data keys, use the <code>KeySpec</code> parameter. </p> <p>If the operation succeeds, the plaintext copy of the data key is in the <code>Plaintext</code> field of the response, and the encrypted copy of the data key in the <code>CiphertextBlob</code> field.</p> <p>To get only an encrypted copy of the data key, use <a>GenerateDataKeyWithoutPlaintext</a>. To generate an asymmetric data key pair, use the <a>GenerateDataKeyPair</a> or <a>GenerateDataKeyPairWithoutPlaintext</a> operation. To get a cryptographically secure random byte string, use <a>GenerateRandom</a>.</p> <p>You can use the optional encryption context to add additional security to the encryption operation. If you specify an <code>EncryptionContext</code>, you must specify the same encryption context (a case-sensitive exact match) when decrypting the encrypted data key. Otherwise, the request to decrypt fails with an InvalidCiphertextException. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>We recommend that you use the following pattern to encrypt data locally in your application:</p> <ol> <li> <p>Use the <code>GenerateDataKey</code> operation to get a data encryption key.</p> </li> <li> <p>Use the plaintext data key (returned in the <code>Plaintext</code> field of the response) to encrypt data locally, then erase the plaintext data key from memory.</p> </li> <li> <p>Store the encrypted data key (returned in the <code>CiphertextBlob</code> field of the response) alongside the locally encrypted data.</p> </li> </ol> <p>To decrypt data locally:</p> <ol> <li> <p>Use the <a>Decrypt</a> operation to decrypt the encrypted data key. The operation returns a plaintext copy of the data key.</p> </li> <li> <p>Use the plaintext data key to decrypt data locally, then erase the plaintext data key from memory.</p> </li> </ol></p>
    fn generate_data_key(
        &self,
        input: GenerateDataKeyRequest,
    ) -> RusotoFuture<GenerateDataKeyResponse, GenerateDataKeyError>;

    /// <p>Generates a unique asymmetric data key pair. The <code>GenerateDataKeyPair</code> operation returns a plaintext public key, a plaintext private key, and a copy of the private key that is encrypted under the symmetric CMK you specify. You can use the data key pair to perform asymmetric cryptography outside of AWS KMS.</p> <p> <code>GenerateDataKeyPair</code> returns a unique data key pair for each request. The bytes in the keys are not related to the caller or the CMK that is used to encrypt the private key.</p> <p>You can use the public key that <code>GenerateDataKeyPair</code> returns to encrypt data or verify a signature outside of AWS KMS. Then, store the encrypted private key with the data. When you are ready to decrypt data or sign a message, you can use the <a>Decrypt</a> operation to decrypt the encrypted private key.</p> <p>To generate a data key pair, you must specify a symmetric customer master key (CMK) to encrypt the private key in a data key pair. You cannot use an asymmetric CMK. To get the type of your CMK, use the <a>DescribeKey</a> operation.</p> <p>If you are using the data key pair to encrypt data, or for any operation where you don't immediately need a private key, consider using the <a>GenerateDataKeyPairWithoutPlaintext</a> operation. <code>GenerateDataKeyPairWithoutPlaintext</code> returns a plaintext public key and an encrypted private key, but omits the plaintext private key that you need only to decrypt ciphertext or sign a message. Later, when you need to decrypt the data or sign a message, use the <a>Decrypt</a> operation to decrypt the encrypted private key in the data key pair.</p> <p>You can use the optional encryption context to add additional security to the encryption operation. If you specify an <code>EncryptionContext</code>, you must specify the same encryption context (a case-sensitive exact match) when decrypting the encrypted data key. Otherwise, the request to decrypt fails with an InvalidCiphertextException. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn generate_data_key_pair(
        &self,
        input: GenerateDataKeyPairRequest,
    ) -> RusotoFuture<GenerateDataKeyPairResponse, GenerateDataKeyPairError>;

    /// <p>Generates a unique asymmetric data key pair. The <code>GenerateDataKeyPairWithoutPlaintext</code> operation returns a plaintext public key and a copy of the private key that is encrypted under the symmetric CMK you specify. Unlike <a>GenerateDataKeyPair</a>, this operation does not return a plaintext private key. </p> <p>To generate a data key pair, you must specify a symmetric customer master key (CMK) to encrypt the private key in the data key pair. You cannot use an asymmetric CMK. To get the type of your CMK, use the <code>KeySpec</code> field in the <a>DescribeKey</a> response.</p> <p>You can use the public key that <code>GenerateDataKeyPairWithoutPlaintext</code> returns to encrypt data or verify a signature outside of AWS KMS. Then, store the encrypted private key with the data. When you are ready to decrypt data or sign a message, you can use the <a>Decrypt</a> operation to decrypt the encrypted private key.</p> <p> <code>GenerateDataKeyPairWithoutPlaintext</code> returns a unique data key pair for each request. The bytes in the key are not related to the caller or CMK that is used to encrypt the private key.</p> <p>You can use the optional encryption context to add additional security to the encryption operation. If you specify an <code>EncryptionContext</code>, you must specify the same encryption context (a case-sensitive exact match) when decrypting the encrypted data key. Otherwise, the request to decrypt fails with an InvalidCiphertextException. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn generate_data_key_pair_without_plaintext(
        &self,
        input: GenerateDataKeyPairWithoutPlaintextRequest,
    ) -> RusotoFuture<
        GenerateDataKeyPairWithoutPlaintextResponse,
        GenerateDataKeyPairWithoutPlaintextError,
    >;

    /// <p>Generates a unique symmetric data key. This operation returns a data key that is encrypted under a customer master key (CMK) that you specify. To request an asymmetric data key pair, use the <a>GenerateDataKeyPair</a> or <a>GenerateDataKeyPairWithoutPlaintext</a> operations.</p> <p> <code>GenerateDataKeyWithoutPlaintext</code> is identical to the <a>GenerateDataKey</a> operation except that returns only the encrypted copy of the data key. This operation is useful for systems that need to encrypt data at some point, but not immediately. When you need to encrypt the data, you call the <a>Decrypt</a> operation on the encrypted copy of the key. </p> <p>It's also useful in distributed systems with different levels of trust. For example, you might store encrypted data in containers. One component of your system creates new containers and stores an encrypted data key with each container. Then, a different component puts the data into the containers. That component first decrypts the data key, uses the plaintext data key to encrypt data, puts the encrypted data into the container, and then destroys the plaintext data key. In this system, the component that creates the containers never sees the plaintext data key.</p> <p> <code>GenerateDataKeyWithoutPlaintext</code> returns a unique data key for each request. The bytes in the keys are not related to the caller or CMK that is used to encrypt the private key.</p> <p>To generate a data key, you must specify the symmetric customer master key (CMK) that is used to encrypt the data key. You cannot use an asymmetric CMK to generate a data key. To get the type of your CMK, use the <code>KeySpec</code> field in the <a>DescribeKey</a> response. You must also specify the length of the data key using either the <code>KeySpec</code> or <code>NumberOfBytes</code> field (but not both). For common key lengths (128-bit and 256-bit symmetric keys), use the <code>KeySpec</code> parameter. </p> <p>If the operation succeeds, you will find the plaintext copy of the data key in the <code>Plaintext</code> field of the response, and the encrypted copy of the data key in the <code>CiphertextBlob</code> field.</p> <p>You can use the optional encryption context to add additional security to the encryption operation. If you specify an <code>EncryptionContext</code>, you must specify the same encryption context (a case-sensitive exact match) when decrypting the encrypted data key. Otherwise, the request to decrypt fails with an InvalidCiphertextException. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn generate_data_key_without_plaintext(
        &self,
        input: GenerateDataKeyWithoutPlaintextRequest,
    ) -> RusotoFuture<GenerateDataKeyWithoutPlaintextResponse, GenerateDataKeyWithoutPlaintextError>;

    /// <p>Returns a random byte string that is cryptographically secure.</p> <p>By default, the random byte string is generated in AWS KMS. To generate the byte string in the AWS CloudHSM cluster that is associated with a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, specify the custom key store ID.</p> <p>For more information about entropy and random number generation, see the <a href="https://d0.awsstatic.com/whitepapers/KMS-Cryptographic-Details.pdf">AWS Key Management Service Cryptographic Details</a> whitepaper.</p>
    fn generate_random(
        &self,
        input: GenerateRandomRequest,
    ) -> RusotoFuture<GenerateRandomResponse, GenerateRandomError>;

    /// <p>Gets a key policy attached to the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p>
    fn get_key_policy(
        &self,
        input: GetKeyPolicyRequest,
    ) -> RusotoFuture<GetKeyPolicyResponse, GetKeyPolicyError>;

    /// <p>Gets a Boolean value that indicates whether <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic rotation of the key material</a> is enabled for the specified customer master key (CMK).</p> <p>You cannot enable automatic rotation of asymmetric CMKs, CMKs with imported key material, or CMKs in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. The key rotation status for these CMKs is always <code>false</code>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <ul> <li> <p>Disabled: The key rotation status does not change when you disable a CMK. However, while the CMK is disabled, AWS KMS does not rotate the backing key.</p> </li> <li> <p>Pending deletion: While a CMK is pending deletion, its key rotation status is <code>false</code> and AWS KMS does not rotate the backing key. If you cancel the deletion, the original key rotation status is restored.</p> </li> </ul> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter.</p>
    fn get_key_rotation_status(
        &self,
        input: GetKeyRotationStatusRequest,
    ) -> RusotoFuture<GetKeyRotationStatusResponse, GetKeyRotationStatusError>;

    /// <p>Returns the items you need to import key material into a symmetric, customer managed customer master key (CMK). For more information about importing key material into AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>This operation returns a public key and an import token. Use the public key to encrypt the symmetric key material. Store the import token to send with a subsequent <a>ImportKeyMaterial</a> request.</p> <p>You must specify the key ID of the symmetric CMK into which you will import key material. This CMK's <code>Origin</code> must be <code>EXTERNAL</code>. You must also specify the wrapping algorithm and type of wrapping key (public key) that you will use to encrypt the key material. You cannot perform this operation on an asymmetric CMK or on any CMK in a different AWS account.</p> <p>To import key material, you must use the public key and import token from the same response. These items are valid for 24 hours. The expiration date and time appear in the <code>GetParametersForImport</code> response. You cannot use an expired token in an <a>ImportKeyMaterial</a> request. If your key and token expire, send another <code>GetParametersForImport</code> request.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn get_parameters_for_import(
        &self,
        input: GetParametersForImportRequest,
    ) -> RusotoFuture<GetParametersForImportResponse, GetParametersForImportError>;

    /// <p>Returns the public key of an asymmetric CMK. Unlike the private key of a asymmetric CMK, which never leaves AWS KMS unencrypted, callers with <code>kms:GetPublicKey</code> permission can download the public key of an asymmetric CMK. You can share the public key to allow others to encrypt messages and verify signatures outside of AWS KMS. For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>You do not need to download the public key. Instead, you can use the public key within AWS KMS by calling the <a>Encrypt</a>, <a>ReEncrypt</a>, or <a>Verify</a> operations with the identifier of an asymmetric CMK. When you use the public key within AWS KMS, you benefit from the authentication, authorization, and logging that are part of every AWS KMS operation. You also reduce of risk of encrypting data that cannot be decrypted. These features are not effective outside of AWS KMS. For details, see <a href="kms/latest/developerguide/get-public-key.html#get-public-key-considerations">Special Considerations for Downloading Public Keys</a>.</p> <p>To help you use the public key safely outside of AWS KMS, <code>GetPublicKey</code> returns important information about the public key in the response, including:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_GetPublicKey.html#KMS-GetPublicKey-response-CustomerMasterKeySpec">CustomerMasterKeySpec</a>: The type of key material in the public key, such as <code>RSA_4096</code> or <code>ECC_NIST_P521</code>.</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_GetPublicKey.html#KMS-GetPublicKey-response-KeyUsage">KeyUsage</a>: Whether the key is used for encryption or signing.</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_GetPublicKey.html#KMS-GetPublicKey-response-EncryptionAlgorithms">EncryptionAlgorithms</a> or <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_GetPublicKey.html#KMS-GetPublicKey-response-SigningAlgorithms">SigningAlgorithms</a>: A list of the encryption algorithms or the signing algorithms for the key.</p> </li> </ul> <p>Although AWS KMS cannot enforce these restrictions on external operations, it is crucial that you use this information to prevent the public key from being used improperly. For example, you can prevent a public signing key from being used encrypt data, or prevent a public key from being used with an encryption algorithm that is not supported by AWS KMS. You can also avoid errors, such as using the wrong signing algorithm in a verification operation.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn get_public_key(
        &self,
        input: GetPublicKeyRequest,
    ) -> RusotoFuture<GetPublicKeyResponse, GetPublicKeyError>;

    /// <p>Imports key material into an existing symmetric AWS KMS customer master key (CMK) that was created without key material. After you successfully import key material into a CMK, you can <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html#reimport-key-material">reimport the same key material</a> into that CMK, but you cannot import different key material.</p> <p>You cannot perform this operation on an asymmetric CMK or on any CMK in a different AWS account. For more information about creating CMKs with no key material and then importing key material, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>Before using this operation, call <a>GetParametersForImport</a>. Its response includes a public key and an import token. Use the public key to encrypt the key material. Then, submit the import token from the same <code>GetParametersForImport</code> response.</p> <p>When calling this operation, you must specify the following values:</p> <ul> <li> <p>The key ID or key ARN of a CMK with no key material. Its <code>Origin</code> must be <code>EXTERNAL</code>.</p> <p>To create a CMK with no key material, call <a>CreateKey</a> and set the value of its <code>Origin</code> parameter to <code>EXTERNAL</code>. To get the <code>Origin</code> of a CMK, call <a>DescribeKey</a>.)</p> </li> <li> <p>The encrypted key material. To get the public key to encrypt the key material, call <a>GetParametersForImport</a>.</p> </li> <li> <p>The import token that <a>GetParametersForImport</a> returned. You must use a public key and token from the same <code>GetParametersForImport</code> response.</p> </li> <li> <p>Whether the key material expires and if so, when. If you set an expiration date, AWS KMS deletes the key material from the CMK on the specified date, and the CMK becomes unusable. To use the CMK again, you must reimport the same key material. The only way to change an expiration date is by reimporting the same key material and specifying a new expiration date. </p> </li> </ul> <p>When this operation is successful, the key state of the CMK changes from <code>PendingImport</code> to <code>Enabled</code>, and you can use the CMK.</p> <p>If this operation fails, use the exception to help determine the problem. If the error is related to the key material, the import token, or wrapping key, use <a>GetParametersForImport</a> to get a new public key and import token for the CMK and repeat the import procedure. For help, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html#importing-keys-overview">How To Import Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn import_key_material(
        &self,
        input: ImportKeyMaterialRequest,
    ) -> RusotoFuture<ImportKeyMaterialResponse, ImportKeyMaterialError>;

    /// <p>Gets a list of aliases in the caller's AWS account and region. You cannot list aliases in other accounts. For more information about aliases, see <a>CreateAlias</a>.</p> <p>By default, the ListAliases command returns all aliases in the account and region. To get only the aliases that point to a particular customer master key (CMK), use the <code>KeyId</code> parameter.</p> <p>The <code>ListAliases</code> response can include aliases that you created and associated with your customer managed CMKs, and aliases that AWS created and associated with AWS managed CMKs in your account. You can recognize AWS aliases because their names have the format <code>aws/&lt;service-name&gt;</code>, such as <code>aws/dynamodb</code>.</p> <p>The response might also include aliases that have no <code>TargetKeyId</code> field. These are predefined aliases that AWS has created but has not yet associated with a CMK. Aliases that AWS creates in your account, including predefined aliases, do not count against your <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html#aliases-limit">AWS KMS aliases limit</a>.</p>
    fn list_aliases(
        &self,
        input: ListAliasesRequest,
    ) -> RusotoFuture<ListAliasesResponse, ListAliasesError>;

    /// <p>Gets a list of all grants for the specified customer master key (CMK).</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter.</p>
    fn list_grants(
        &self,
        input: ListGrantsRequest,
    ) -> RusotoFuture<ListGrantsResponse, ListGrantsError>;

    /// <p>Gets the names of the key policies that are attached to a customer master key (CMK). This operation is designed to get policy names that you can use in a <a>GetKeyPolicy</a> operation. However, the only valid policy name is <code>default</code>. You cannot perform this operation on a CMK in a different AWS account.</p>
    fn list_key_policies(
        &self,
        input: ListKeyPoliciesRequest,
    ) -> RusotoFuture<ListKeyPoliciesResponse, ListKeyPoliciesError>;

    /// <p>Gets a list of all customer master keys (CMKs) in the caller's AWS account and Region.</p>
    fn list_keys(&self, input: ListKeysRequest) -> RusotoFuture<ListKeysResponse, ListKeysError>;

    /// <p>Returns a list of all tags for the specified customer master key (CMK).</p> <p>You cannot perform this operation on a CMK in a different AWS account.</p>
    fn list_resource_tags(
        &self,
        input: ListResourceTagsRequest,
    ) -> RusotoFuture<ListResourceTagsResponse, ListResourceTagsError>;

    /// <p>Returns a list of all grants for which the grant's <code>RetiringPrincipal</code> matches the one specified.</p> <p>A typical use is to list all grants that you are able to retire. To retire a grant, use <a>RetireGrant</a>.</p>
    fn list_retirable_grants(
        &self,
        input: ListRetirableGrantsRequest,
    ) -> RusotoFuture<ListGrantsResponse, ListRetirableGrantsError>;

    /// <p>Attaches a key policy to the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about key policies, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Key Policies</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn put_key_policy(&self, input: PutKeyPolicyRequest) -> RusotoFuture<(), PutKeyPolicyError>;

    /// <p>Decrypts ciphertext and then reencrypts it entirely within AWS KMS. You can use this operation to change the customer master key (CMK) under which data is encrypted, such as when you <a href="kms/latest/developerguide/rotate-keys.html#rotate-keys-manually">manually rotate</a> a CMK or change the CMK that protects a ciphertext. You can also use it to reencrypt ciphertext under the same CMK, such as to change the encryption context of a ciphertext. </p> <p>The <code>ReEncrypt</code> operation can decrypt ciphertext that was encrypted by using an AWS KMS CMK in an AWS KMS operation, such as <a>Encrypt</a> or <a>GenerateDataKey</a>. It can also decrypt ciphertext that was encrypted by using the public key of an asymmetric CMK outside of AWS KMS. However, it cannot decrypt ciphertext produced by other libraries, such as the <a href="https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/">AWS Encryption SDK</a> or <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingClientSideEncryption.html">Amazon S3 client-side encryption</a>. These libraries return a ciphertext format that is incompatible with AWS KMS.</p> <p>When you use the <code>ReEncrypt</code> operation, you need to provide information for the decrypt operation and the subsequent encrypt operation.</p> <ul> <li> <p>If your ciphertext was encrypted under an asymmetric CMK, you must identify the <i>source CMK</i>, that is, the CMK that encrypted the ciphertext. You must also supply the encryption algorithm that was used. This information is required to decrypt the data.</p> </li> <li> <p>It is optional, but you can specify a source CMK even when the ciphertext was encrypted under a symmetric CMK. This ensures that the ciphertext is decrypted only by using a particular CMK. If the CMK that you specify cannot decrypt the ciphertext, the <code>ReEncrypt</code> operation fails.</p> </li> <li> <p>To reencrypt the data, you must specify the <i>destination CMK</i>, that is, the CMK that re-encrypts the data after it is decrypted. You can select a symmetric or asymmetric CMK. If the destination CMK is an asymmetric CMK, you must also provide the encryption algorithm. The algorithm that you choose must be compatible with the CMK.</p> <important> <p>When you use an asymmetric CMK to encrypt or reencrypt data, be sure to record the CMK and encryption algorithm that you choose. You will be required to provide the same CMK and encryption algorithm when you decrypt the data. If the CMK and algorithm do not match the values used to encrypt the data, the decrypt operation fails.</p> <p>You are not required to supply the CMK ID and encryption algorithm when you decrypt with symmetric CMKs because AWS KMS stores this information in the ciphertext blob. AWS KMS cannot store metadata in ciphertext generated with asymmetric keys. The standard format for asymmetric key ciphertext does not include configurable fields.</p> </important> </li> </ul> <p>Unlike other AWS KMS API operations, <code>ReEncrypt</code> callers must have two permissions:</p> <ul> <li> <p> <code>kms:EncryptFrom</code> permission on the source CMK</p> </li> <li> <p> <code>kms:EncryptTo</code> permission on the destination CMK</p> </li> </ul> <p>To permit reencryption from</p> <p> or to a CMK, include the <code>"kms:ReEncrypt*"</code> permission in your <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">key policy</a>. This permission is automatically included in the key policy when you use the console to create a CMK. But you must include it manually when you create a CMK programmatically or when you use the <a>PutKeyPolicy</a> operation set a key policy.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn re_encrypt(
        &self,
        input: ReEncryptRequest,
    ) -> RusotoFuture<ReEncryptResponse, ReEncryptError>;

    /// <p>Retires a grant. To clean up, you can retire a grant when you're done using it. You should revoke a grant when you intend to actively deny operations that depend on it. The following are permitted to call this API:</p> <ul> <li> <p>The AWS account (root user) under which the grant was created</p> </li> <li> <p>The <code>RetiringPrincipal</code>, if present in the grant</p> </li> <li> <p>The <code>GranteePrincipal</code>, if <code>RetireGrant</code> is an operation specified in the grant</p> </li> </ul> <p>You must identify the grant to retire by its grant token or by a combination of the grant ID and the Amazon Resource Name (ARN) of the customer master key (CMK). A grant token is a unique variable-length base64-encoded string. A grant ID is a 64 character unique identifier of a grant. The <a>CreateGrant</a> operation returns both.</p>
    fn retire_grant(&self, input: RetireGrantRequest) -> RusotoFuture<(), RetireGrantError>;

    /// <p>Revokes the specified grant for the specified customer master key (CMK). You can revoke a grant to actively deny operations that depend on it.</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter.</p>
    fn revoke_grant(&self, input: RevokeGrantRequest) -> RusotoFuture<(), RevokeGrantError>;

    /// <p>Schedules the deletion of a customer master key (CMK). You may provide a waiting period, specified in days, before deletion occurs. If you do not provide a waiting period, the default period of 30 days is used. When this operation is successful, the key state of the CMK changes to <code>PendingDeletion</code>. Before the waiting period ends, you can use <a>CancelKeyDeletion</a> to cancel the deletion of the CMK. After the waiting period ends, AWS KMS deletes the CMK and all AWS KMS data associated with it, including all aliases that refer to it.</p> <important> <p>Deleting a CMK is a destructive and potentially dangerous operation. When a CMK is deleted, all data that was encrypted under the CMK is unrecoverable. To prevent the use of a CMK without deleting it, use <a>DisableKey</a>.</p> </important> <p>If you schedule deletion of a CMK from a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, when the waiting period expires, <code>ScheduleKeyDeletion</code> deletes the CMK from AWS KMS. Then AWS KMS makes a best effort to delete the key material from the associated AWS CloudHSM cluster. However, you might need to manually <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#fix-keystore-orphaned-key">delete the orphaned key material</a> from the cluster and its backups.</p> <p>You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about scheduling a CMK for deletion, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html">Deleting Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn schedule_key_deletion(
        &self,
        input: ScheduleKeyDeletionRequest,
    ) -> RusotoFuture<ScheduleKeyDeletionResponse, ScheduleKeyDeletionError>;

    /// <p>Creates a <a href="https://en.wikipedia.org/wiki/Digital_signature">digital signature</a> for a message or message digest by using the private key in an asymmetric CMK. To verify the signature, use the <a>Verify</a> operation, or use the public key in the same asymmetric CMK outside of AWS KMS. For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>Digital signatures are generated and verified by using asymmetric key pair, such as an RSA or ECC pair that is represented by an asymmetric customer master key (CMK). The key owner (or an authorized user) uses their private key to sign a message. Anyone with the public key can verify that the message was signed with that particular private key and that the message hasn't changed since it was signed. </p> <p>To use the <code>Sign</code> operation, provide the following information:</p> <ul> <li> <p>Use the <code>KeyId</code> parameter to identify an asymmetric CMK with a <code>KeyUsage</code> value of <code>SIGN_VERIFY</code>. To get the <code>KeyUsage</code> value of a CMK, use the <a>DescribeKey</a> operation. The caller must have <code>kms:Sign</code> permission on the CMK.</p> </li> <li> <p>Use the <code>Message</code> parameter to specify the message or message digest to sign. You can submit messages of up to 4096 bytes. To sign a larger message, generate a hash digest of the message, and then provide the hash digest in the <code>Message</code> parameter. To indicate whether the message is a full message or a digest, use the <code>MessageType</code> parameter.</p> </li> <li> <p>Choose a signing algorithm that is compatible with the CMK. </p> </li> </ul> <important> <p>When signing a message, be sure to record the CMK and the signing algorithm. This information is required to verify the signature.</p> </important> <p>To verify the signature that this operation generates, use the <a>Verify</a> operation. Or use the <a>GetPublicKey</a> operation to download the public key and then use the public key to verify the signature outside of AWS KMS. </p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn sign(&self, input: SignRequest) -> RusotoFuture<SignResponse, SignError>;

    /// <p>Adds or edits tags for a customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>Each tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty (null) strings.</p> <p>You can only use a tag key once for each CMK. If you use the tag key again, AWS KMS replaces the current tag value with the specified value.</p> <p>For information about the rules that apply to tag keys and tag values, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/allocation-tag-restrictions.html">User-Defined Tag Restrictions</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError>;

    /// <p>Removes the specified tags from the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>To remove a tag, specify the tag key. To change the tag value of an existing tag key, use <a>TagResource</a>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError>;

    /// <p>Associates an existing AWS KMS alias with a different customer master key (CMK). Each alias is associated with only one CMK at a time, although a CMK can have multiple aliases. The alias and the CMK must be in the same AWS account and region. You cannot perform this operation on an alias in a different AWS account. </p> <p>The current and new CMK must be the same type (both symmetric or both asymmetric), and they must have the same key usage (<code>ENCRYPT_DECRYPT</code> or <code>SIGN_VERIFY</code>). This restriction prevents errors in code that uses aliases. If you must assign an alias to a different type of CMK, use <a>DeleteAlias</a> to delete the old alias and <a>CreateAlias</a> to create a new alias.</p> <p>You cannot use <code>UpdateAlias</code> to change an alias name. To change an alias name, use <a>DeleteAlias</a> to delete the old alias and <a>CreateAlias</a> to create a new alias.</p> <p>Because an alias is not a property of a CMK, you can create, update, and delete the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases of all CMKs in the account, use the <a>ListAliases</a> operation. </p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn update_alias(&self, input: UpdateAliasRequest) -> RusotoFuture<(), UpdateAliasError>;

    /// <p>Changes the properties of a custom key store. Use the <code>CustomKeyStoreId</code> parameter to identify the custom key store you want to edit. Use the remaining parameters to change the properties of the custom key store.</p> <p>You can only update a custom key store that is disconnected. To disconnect the custom key store, use <a>DisconnectCustomKeyStore</a>. To reconnect the custom key store after the update completes, use <a>ConnectCustomKeyStore</a>. To find the connection state of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>Use the parameters of <code>UpdateCustomKeyStore</code> to edit your keystore settings.</p> <ul> <li> <p>Use the <b>NewCustomKeyStoreName</b> parameter to change the friendly name of the custom key store to the value that you specify.</p> <p> </p> </li> <li> <p>Use the <b>KeyStorePassword</b> parameter tell AWS KMS the current password of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-concepts.html#concept-kmsuser"> <code>kmsuser</code> crypto user (CU)</a> in the associated AWS CloudHSM cluster. You can use this parameter to <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#fix-keystore-password">fix connection failures</a> that occur when AWS KMS cannot log into the associated cluster because the <code>kmsuser</code> password has changed. This value does not change the password in the AWS CloudHSM cluster.</p> <p> </p> </li> <li> <p>Use the <b>CloudHsmClusterId</b> parameter to associate the custom key store with a different, but related, AWS CloudHSM cluster. You can use this parameter to repair a custom key store if its AWS CloudHSM cluster becomes corrupted or is deleted, or when you need to create or restore a cluster from a backup. </p> </li> </ul> <p>If the operation succeeds, it returns a JSON object with no properties.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p>
    fn update_custom_key_store(
        &self,
        input: UpdateCustomKeyStoreRequest,
    ) -> RusotoFuture<UpdateCustomKeyStoreResponse, UpdateCustomKeyStoreError>;

    /// <p>Updates the description of a customer master key (CMK). To see the description of a CMK, use <a>DescribeKey</a>. </p> <p>You cannot perform this operation on a CMK in a different AWS account.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn update_key_description(
        &self,
        input: UpdateKeyDescriptionRequest,
    ) -> RusotoFuture<(), UpdateKeyDescriptionError>;

    /// <p>Verifies a digital signature that was generated by the <a>Sign</a> operation. This operation requires an asymmetric CMK with a <code>KeyUsage</code> value of <code>SIGN_VERIFY</code>.</p> <p/> <p>Verification confirms that an authorized user signed the message with the specified key and signing algorithm, and the message hasn't changed since it was signed. A digital signature is generated by using the private key in an asymmetric CMK. The signature is verified by using the public key in the same asymmetric CMK. For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>To verify a digital signature, you can use the <code>Verify</code> operation. Specify the same asymmetric CMK that was used by the <code>Sign</code> operation to generate the digital signature.</p> <p>You can also verify the digital signature by using the public key of the CMK outside of AWS KMS. Use the <a>GetPublicKey</a> operation to download the public key in the asymmetric CMK and then use the public key to verify the signature outside of AWS KMS.</p> <p>The advantage of using the <code>Verify</code> operation is that it is performed within AWS KMS. As a result, it's easy to call, the operation is performed within the FIPS boundary, it is logged in AWS CloudTrail, and you can use key policy and IAM policy to determine who is authorized to use the CMK to verify signatures.</p> <important> <p>The result of the <code>Verify</code> operation, which is represented by its HTTP status code, does not indicate whether the signature verification succeeded or failed. To determine whether the signature was verified, see the <code>SignatureValid</code> field in the response. </p> </important> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn verify(&self, input: VerifyRequest) -> RusotoFuture<VerifyResponse, VerifyError>;
}
/// A client for the KMS API.
#[derive(Clone)]
pub struct KmsClient {
    client: Client,
    region: region::Region,
}

impl KmsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> KmsClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> KmsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        Self::new_with_client(
            Client::new_with(credentials_provider, request_dispatcher),
            region,
        )
    }

    pub fn new_with_client(client: Client, region: region::Region) -> KmsClient {
        KmsClient { client, region }
    }
}

impl fmt::Debug for KmsClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KmsClient")
            .field("region", &self.region)
            .finish()
    }
}

impl Kms for KmsClient {
    /// <p>Cancels the deletion of a customer master key (CMK). When this operation succeeds, the key state of the CMK is <code>Disabled</code>. To enable the CMK, use <a>EnableKey</a>. You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about scheduling and canceling deletion of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html">Deleting Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn cancel_key_deletion(
        &self,
        input: CancelKeyDeletionRequest,
    ) -> RusotoFuture<CancelKeyDeletionResponse, CancelKeyDeletionError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.CancelKeyDeletion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CancelKeyDeletionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CancelKeyDeletionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Connects or reconnects a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> to its associated AWS CloudHSM cluster.</p> <p>The custom key store must be connected before you can create customer master keys (CMKs) in the key store or use the CMKs it contains. You can disconnect and reconnect a custom key store at any time.</p> <p>To connect a custom key store, its associated AWS CloudHSM cluster must have at least one active HSM. To get the number of active HSMs in a cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation. To add HSMs to the cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_CreateHsm.html">CreateHsm</a> operation.</p> <p>The connection process can take an extended amount of time to complete; up to 20 minutes. This operation starts the connection process, but it does not wait for it to complete. When it succeeds, this operation quickly returns an HTTP 200 response and a JSON object with no properties. However, this response does not indicate that the custom key store is connected. To get the connection state of the custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>During the connection process, AWS KMS finds the AWS CloudHSM cluster that is associated with the custom key store, creates the connection infrastructure, connects to the cluster, logs into the AWS CloudHSM client as the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-concepts.html#concept-kmsuser"> <code>kmsuser</code> crypto user</a> (CU), and rotates its password.</p> <p>The <code>ConnectCustomKeyStore</code> operation might fail for various reasons. To find the reason, use the <a>DescribeCustomKeyStores</a> operation and see the <code>ConnectionErrorCode</code> in the response. For help interpreting the <code>ConnectionErrorCode</code>, see <a>CustomKeyStoresListEntry</a>.</p> <p>To fix the failure, use the <a>DisconnectCustomKeyStore</a> operation to disconnect the custom key store, correct the error, use the <a>UpdateCustomKeyStore</a> operation if necessary, and then use <code>ConnectCustomKeyStore</code> again.</p> <p>If you are having trouble connecting or disconnecting a custom key store, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting a Custom Key Store</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn connect_custom_key_store(
        &self,
        input: ConnectCustomKeyStoreRequest,
    ) -> RusotoFuture<ConnectCustomKeyStoreResponse, ConnectCustomKeyStoreError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ConnectCustomKeyStore");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ConnectCustomKeyStoreResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ConnectCustomKeyStoreError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a display name for a customer managed customer master key (CMK). You can use an alias to identify a CMK in cryptographic operations, such as <a>Encrypt</a> and <a>GenerateDataKey</a>. You can change the CMK associated with the alias at any time.</p> <p>Aliases are easier to remember than key IDs. They can also help to simplify your applications. For example, if you use an alias in your code, you can change the CMK your code uses by associating a given alias with a different CMK. </p> <p>To run the same code in multiple AWS regions, use an alias in your code, such as <code>alias/ApplicationKey</code>. Then, in each AWS Region, create an <code>alias/ApplicationKey</code> alias that is associated with a CMK in that Region. When you run your code, it uses the <code>alias/ApplicationKey</code> CMK for that AWS Region without any Region-specific code.</p> <p>This operation does not return a response. To get the alias that you created, use the <a>ListAliases</a> operation.</p> <p>To use aliases successfully, be aware of the following information.</p> <ul> <li> <p>Each alias points to only one CMK at a time, although a single CMK can have multiple aliases. The alias and its associated CMK must be in the same AWS account and Region. </p> </li> <li> <p>You can associate an alias with any customer managed CMK in the same AWS account and Region. However, you do not have permission to associate an alias with an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">AWS managed CMK</a> or an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-owned-cmk">AWS owned CMK</a>. </p> </li> <li> <p>To change the CMK associated with an alias, use the <a>UpdateAlias</a> operation. The current CMK and the new CMK must be the same type (both symmetric or both asymmetric) and they must have the same key usage (<code>ENCRYPT_DECRYPT</code> or <code>SIGN_VERIFY</code>). This restriction prevents cryptographic errors in code that uses aliases.</p> </li> <li> <p>The alias name must begin with <code>alias/</code> followed by a name, such as <code>alias/ExampleAlias</code>. It can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). The alias name cannot begin with <code>alias/aws/</code>. The <code>alias/aws/</code> prefix is reserved for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">AWS managed CMKs</a>. </p> </li> <li> <p>The alias name must be unique within an AWS Region. However, you can use the same alias name in multiple Regions of the same AWS account. Each instance of the alias is associated with a CMK in its Region.</p> </li> <li> <p>After you create an alias, you cannot change its alias name. However, you can use the <a>DeleteAlias</a> operation to delete the alias and then create a new alias with the desired name.</p> </li> <li> <p>You can use an alias name or alias ARN to identify a CMK in AWS KMS cryptographic operations and in the <a>DescribeKey</a> operation. However, you cannot use alias names or alias ARNs in API operations that manage CMKs, such as <a>DisableKey</a> or <a>GetKeyPolicy</a>. For information about the valid CMK identifiers for each AWS KMS API operation, see the descriptions of the <code>KeyId</code> parameter in the API operation documentation.</p> </li> </ul> <p>Because an alias is not a property of a CMK, you can delete and change the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases and alias ARNs of CMKs in each AWS account and Region, use the <a>ListAliases</a> operation.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn create_alias(&self, input: CreateAliasRequest) -> RusotoFuture<(), CreateAliasError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.CreateAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateAliasError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> that is associated with an <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/clusters.html">AWS CloudHSM cluster</a> that you own and manage.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p> <p>Before you create the custom key store, you must assemble the required elements, including an AWS CloudHSM cluster that fulfills the requirements for a custom key store. For details about the required elements, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">Assemble the Prerequisites</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>When the operation completes successfully, it returns the ID of the new custom key store. Before you can use your new custom key store, you need to use the <a>ConnectCustomKeyStore</a> operation to connect the new key store to its AWS CloudHSM cluster. Even if you are not going to use your custom key store immediately, you might want to connect it to verify that all settings are correct and then disconnect it until you are ready to use it.</p> <p>For help with failures, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting a Custom Key Store</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn create_custom_key_store(
        &self,
        input: CreateCustomKeyStoreRequest,
    ) -> RusotoFuture<CreateCustomKeyStoreResponse, CreateCustomKeyStoreError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.CreateCustomKeyStore");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateCustomKeyStoreResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateCustomKeyStoreError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Adds a grant to a customer master key (CMK). The grant allows the grantee principal to use the CMK when the conditions specified in the grant are met. When setting permissions, grants are an alternative to key policies. </p> <p>To create a grant that allows a cryptographic operation only when the request includes a particular <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">encryption context</a>, use the <code>Constraints</code> parameter. For details, see <a>GrantConstraints</a>.</p> <p>You can create grants on symmetric and asymmetric CMKs. However, if the grant allows an operation that the CMK does not support, <code>CreateGrant</code> fails with a <code>ValidationException</code>. </p> <ul> <li> <p>Grants for symmetric CMKs cannot allow operations that are not supported for symmetric CMKs, including <a>Sign</a>, <a>Verify</a>, and <a>GetPublicKey</a>. (There are limited exceptions to this rule for legacy operations, but you should not create a grant for an operation that AWS KMS does not support.)</p> </li> <li> <p>Grants for asymmetric CMKs cannot allow operations that are not supported for asymmetric CMKs, including operations that <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_GenerateDataKey">generate data keys</a> or <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_GenerateDataKeyPair">data key pairs</a>, or operations related to <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic key rotation</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or CMKs in <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key stores</a>.</p> </li> <li> <p>Grants for asymmetric CMKs with a <code>KeyUsage</code> of <code>ENCRYPT_DECRYPT</code> cannot allow the <a>Sign</a> or <a>Verify</a> operations. Grants for asymmetric CMKs with a <code>KeyUsage</code> of <code>SIGN_VERIFY</code> cannot allow the <a>Encrypt</a> or <a>Decrypt</a> operations.</p> </li> <li> <p>Grants for asymmetric CMKs cannot include an encryption context grant constraint. An encryption context is not supported on asymmetric CMKs.</p> </li> </ul> <p>For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter. For more information about grants, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html">Grants</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn create_grant(
        &self,
        input: CreateGrantRequest,
    ) -> RusotoFuture<CreateGrantResponse, CreateGrantError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.CreateGrant");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateGrantResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateGrantError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Creates a unique customer managed <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master-keys">customer master key</a> (CMK) in your AWS account and Region. You cannot use this operation to create a CMK in a different AWS account.</p> <p>You can use the <code>CreateKey</code> operation to create symmetric or asymmetric CMKs.</p> <ul> <li> <p> <b>Symmetric CMKs</b> contain a 256-bit symmetric key that never leaves AWS KMS unencrypted. To use the CMK, you must call AWS KMS. You can use a symmetric CMK to encrypt and decrypt small amounts of data, but they are typically used to generate <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#data-keys">data keys</a> or data key pairs. For details, see <a>GenerateDataKey</a> and <a>GenerateDataKeyPair</a>.</p> </li> <li> <p> <b>Asymmetric CMKs</b> can contain an RSA key pair or an Elliptic Curve (ECC) key pair. The private key in an asymmetric CMK never leaves AWS KMS unencrypted. However, you can use the <a>GetPublicKey</a> operation to download the public key so it can be used outside of AWS KMS. CMKs with RSA key pairs can be used to encrypt or decrypt data or sign and verify messages (but not both). CMKs with ECC key pairs can be used only to sign and verify messages.</p> </li> </ul> <p>For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>To create different types of CMKs, use the following guidance:</p> <dl> <dt>Asymmetric CMKs</dt> <dd> <p>To create an asymmetric CMK, use the <code>CustomerMasterKeySpec</code> parameter to specify the type of key material in the CMK. Then, use the <code>KeyUsage</code> parameter to determine whether the CMK will be used to encrypt and decrypt or sign and verify. You can&#39;t change these properties after the CMK is created.</p> <p> </p> </dd> <dt>Symmetric CMKs</dt> <dd> <p>When creating a symmetric CMK, you don&#39;t need to specify the <code>CustomerMasterKeySpec</code> or <code>KeyUsage</code> parameters. The default value for <code>CustomerMasterKeySpec</code>, <code>SYMMETRIC<em>DEFAULT</code>, and the default value for <code>KeyUsage</code>, <code>ENCRYPT</em>DECRYPT</code>, are the only valid values for symmetric CMKs. </p> <p> </p> </dd> <dt>Imported Key Material</dt> <dd> <p>To import your own key material, begin by creating a symmetric CMK with no key material. To do this, use the <code>Origin</code> parameter of <code>CreateKey</code> with a value of <code>EXTERNAL</code>. Next, use <a>GetParametersForImport</a> operation to get a public key and import token, and use the public key to encrypt your key material. Then, use <a>ImportKeyMaterial</a> with your import token to import the key material. For step-by-step instructions, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>. You cannot import the key material into an asymmetric CMK.</p> <p> </p> </dd> <dt>Custom Key Stores</dt> <dd> <p>To create a symmetric CMK in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, use the <code>CustomKeyStoreId</code> parameter to specify the custom key store. You must also use the <code>Origin</code> parameter with a value of <code>AWS_CLOUDHSM</code>. The AWS CloudHSM cluster that is associated with the custom key store must have at least two active HSMs in different Availability Zones in the AWS Region. </p> <p>You cannot create an asymmetric CMK in a custom key store. For information about custom key stores in AWS KMS see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Using Custom Key Stores</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> </dd> </dl></p>
    fn create_key(
        &self,
        input: CreateKeyRequest,
    ) -> RusotoFuture<CreateKeyResponse, CreateKeyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.CreateKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateKeyResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateKeyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Decrypts ciphertext that was encrypted by a AWS KMS customer master key (CMK) using any of the following operations:</p> <ul> <li> <p> <a>Encrypt</a> </p> </li> <li> <p> <a>GenerateDataKey</a> </p> </li> <li> <p> <a>GenerateDataKeyPair</a> </p> </li> <li> <p> <a>GenerateDataKeyWithoutPlaintext</a> </p> </li> <li> <p> <a>GenerateDataKeyPairWithoutPlaintext</a> </p> </li> </ul> <p>You can use this operation to decrypt ciphertext that was encrypted under a symmetric or asymmetric CMK. When the CMK is asymmetric, you must specify the CMK and the encryption algorithm that was used to encrypt the ciphertext. For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The Decrypt operation also decrypts ciphertext that was encrypted outside of AWS KMS by the public key in an AWS KMS asymmetric CMK. However, it cannot decrypt ciphertext produced by other libraries, such as the <a href="https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/">AWS Encryption SDK</a> or <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingClientSideEncryption.html">Amazon S3 client-side encryption</a>. These libraries return a ciphertext format that is incompatible with AWS KMS.</p> <p>If the ciphertext was encrypted under a symmetric CMK, you do not need to specify the CMK or the encryption algorithm. AWS KMS can get this information from metadata that it adds to the symmetric ciphertext blob. However, if you prefer, you can specify the <code>KeyId</code> to ensure that a particular CMK is used to decrypt the ciphertext. If you specify a different CMK than the one used to encrypt the ciphertext, the <code>Decrypt</code> operation fails.</p> <p>Whenever possible, use key policies to give users permission to call the Decrypt operation on a particular CMK, instead of using IAM policies. Otherwise, you might create an IAM user policy that gives the user Decrypt permission on all CMKs. This user could decrypt ciphertext that was encrypted by CMKs in other accounts if the key policy for the cross-account CMK permits it. If you must use an IAM policy for <code>Decrypt</code> permissions, limit the user to particular CMKs or particular trusted accounts.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn decrypt(&self, input: DecryptRequest) -> RusotoFuture<DecryptResponse, DecryptError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.Decrypt");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<DecryptResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DecryptError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified alias. You cannot perform this operation on an alias in a different AWS account. </p> <p>Because an alias is not a property of a CMK, you can delete and change the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases of all CMKs, use the <a>ListAliases</a> operation. </p> <p>Each CMK can have multiple aliases. To change the alias of a CMK, use <a>DeleteAlias</a> to delete the current alias and <a>CreateAlias</a> to create a new alias. To associate an existing alias with a different customer master key (CMK), call <a>UpdateAlias</a>.</p>
    fn delete_alias(&self, input: DeleteAliasRequest) -> RusotoFuture<(), DeleteAliasError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DeleteAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteAliasError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. This operation does not delete the AWS CloudHSM cluster that is associated with the custom key store, or affect any users or keys in the cluster.</p> <p>The custom key store that you delete cannot contain any AWS KMS <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">customer master keys (CMKs)</a>. Before deleting the key store, verify that you will never need to use any of the CMKs in the key store for any cryptographic operations. Then, use <a>ScheduleKeyDeletion</a> to delete the AWS KMS customer master keys (CMKs) from the key store. When the scheduled waiting period expires, the <code>ScheduleKeyDeletion</code> operation deletes the CMKs. Then it makes a best effort to delete the key material from the associated cluster. However, you might need to manually <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#fix-keystore-orphaned-key">delete the orphaned key material</a> from the cluster and its backups.</p> <p>After all CMKs are deleted from AWS KMS, use <a>DisconnectCustomKeyStore</a> to disconnect the key store from AWS KMS. Then, you can delete the custom key store.</p> <p>Instead of deleting the custom key store, consider using <a>DisconnectCustomKeyStore</a> to disconnect it from AWS KMS. While the key store is disconnected, you cannot create or use the CMKs in the key store. But, you do not need to delete CMKs and you can reconnect a disconnected custom key store at any time.</p> <p>If the operation succeeds, it returns a JSON object with no properties.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p>
    fn delete_custom_key_store(
        &self,
        input: DeleteCustomKeyStoreRequest,
    ) -> RusotoFuture<DeleteCustomKeyStoreResponse, DeleteCustomKeyStoreError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DeleteCustomKeyStore");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteCustomKeyStoreResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteCustomKeyStoreError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes key material that you previously imported. This operation makes the specified customer master key (CMK) unusable. For more information about importing key material into AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>. You cannot perform this operation on a CMK in a different AWS account.</p> <p>When the specified CMK is in the <code>PendingDeletion</code> state, this operation does not change the CMK's state. Otherwise, it changes the CMK's state to <code>PendingImport</code>.</p> <p>After you delete key material, you can use <a>ImportKeyMaterial</a> to reimport the same key material into the CMK.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn delete_imported_key_material(
        &self,
        input: DeleteImportedKeyMaterialRequest,
    ) -> RusotoFuture<(), DeleteImportedKeyMaterialError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DeleteImportedKeyMaterial");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteImportedKeyMaterialError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets information about <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key stores</a> in the account and region.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p> <p>By default, this operation returns information about all custom key stores in the account and region. To get only information about a particular custom key store, use either the <code>CustomKeyStoreName</code> or <code>CustomKeyStoreId</code> parameter (but not both).</p> <p>To determine whether the custom key store is connected to its AWS CloudHSM cluster, use the <code>ConnectionState</code> element in the response. If an attempt to connect the custom key store failed, the <code>ConnectionState</code> value is <code>FAILED</code> and the <code>ConnectionErrorCode</code> element in the response indicates the cause of the failure. For help interpreting the <code>ConnectionErrorCode</code>, see <a>CustomKeyStoresListEntry</a>.</p> <p>Custom key stores have a <code>DISCONNECTED</code> connection state if the key store has never been connected or you use the <a>DisconnectCustomKeyStore</a> operation to disconnect it. If your custom key store state is <code>CONNECTED</code> but you are having trouble using it, make sure that its associated AWS CloudHSM cluster is active and contains the minimum number of HSMs required for the operation, if any.</p> <p> For help repairing your custom key store, see the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html">Troubleshooting Custom Key Stores</a> topic in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn describe_custom_key_stores(
        &self,
        input: DescribeCustomKeyStoresRequest,
    ) -> RusotoFuture<DescribeCustomKeyStoresResponse, DescribeCustomKeyStoresError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DescribeCustomKeyStores");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeCustomKeyStoresResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeCustomKeyStoresError::from_response(response))
                }))
            }
        })
    }

    /// <p>Provides detailed information about a customer master key (CMK). You can run <code>DescribeKey</code> on a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-cmk">customer managed CMK</a> or an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">AWS managed CMK</a>.</p> <p>This detailed information includes the key ARN, creation date (and deletion date, if applicable), the key state, and the origin and expiration date (if any) of the key material. For CMKs in custom key stores, it includes information about the custom key store, such as the key store ID and the AWS CloudHSM cluster ID. It includes fields, like <code>KeySpec</code>, that help you distinguish symmetric from asymmetric CMKs. It also provides information that is particularly important to asymmetric CMKs, such as the key usage (encryption or signing) and the encryption algorithms or signing algorithms that the CMK supports.</p> <p> <code>DescribeKey</code> does not return the following information:</p> <ul> <li> <p>Aliases associated with the CMK. To get this information, use <a>ListAliases</a>.</p> </li> <li> <p>Whether automatic key rotation is enabled on the CMK. To get this information, use <a>GetKeyRotationStatus</a>. Also, some key states prevent a CMK from being automatically rotated. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#rotate-keys-how-it-works">How Automatic Key Rotation Works</a> in <i>AWS Key Management Service Developer Guide</i>.</p> </li> <li> <p>Tags on the CMK. To get this information, use <a>ListResourceTags</a>.</p> </li> <li> <p>Key policies and grants on the CMK. To get this information, use <a>GetKeyPolicy</a> and <a>ListGrants</a>.</p> </li> </ul> <p>If you call the <code>DescribeKey</code> operation on a <i>predefined AWS alias</i>, that is, an AWS alias with no key ID, AWS KMS creates an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">AWS managed CMK</a>. Then, it associates the alias with the new CMK, and returns the <code>KeyId</code> and <code>Arn</code> of the new CMK in the response.</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the KeyId parameter.</p>
    fn describe_key(
        &self,
        input: DescribeKeyRequest,
    ) -> RusotoFuture<DescribeKeyResponse, DescribeKeyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DescribeKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeKeyResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeKeyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Sets the state of a customer master key (CMK) to disabled, thereby preventing its use for cryptographic operations. You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about how key state affects the use of a CMK, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects the Use of a Customer Master Key</a> in the <i> <i>AWS Key Management Service Developer Guide</i> </i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn disable_key(&self, input: DisableKeyRequest) -> RusotoFuture<(), DisableKeyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DisableKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DisableKeyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Disables <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic rotation of the key material</a> for the specified symmetric customer master key (CMK).</p> <p> You cannot enable automatic rotation of asymmetric CMKs, CMKs with imported key material, or CMKs in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. You cannot perform this operation on a CMK in a different AWS account.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn disable_key_rotation(
        &self,
        input: DisableKeyRotationRequest,
    ) -> RusotoFuture<(), DisableKeyRotationError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DisableKeyRotation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DisableKeyRotationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Disconnects the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a> from its associated AWS CloudHSM cluster. While a custom key store is disconnected, you can manage the custom key store and its customer master keys (CMKs), but you cannot create or use CMKs in the custom key store. You can reconnect the custom key store at any time.</p> <note> <p>While a custom key store is disconnected, all attempts to create customer master keys (CMKs) in the custom key store or to use existing CMKs in cryptographic operations will fail. This action can prevent users from storing and accessing sensitive data.</p> </note> <p/> <p>To find the connection state of a custom key store, use the <a>DescribeCustomKeyStores</a> operation. To reconnect a custom key store, use the <a>ConnectCustomKeyStore</a> operation.</p> <p>If the operation succeeds, it returns a JSON object with no properties.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p>
    fn disconnect_custom_key_store(
        &self,
        input: DisconnectCustomKeyStoreRequest,
    ) -> RusotoFuture<DisconnectCustomKeyStoreResponse, DisconnectCustomKeyStoreError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.DisconnectCustomKeyStore");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisconnectCustomKeyStoreResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisconnectCustomKeyStoreError::from_response(response))
                }))
            }
        })
    }

    /// <p>Sets the key state of a customer master key (CMK) to enabled. This allows you to use the CMK for cryptographic operations. You cannot perform this operation on a CMK in a different AWS account.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn enable_key(&self, input: EnableKeyRequest) -> RusotoFuture<(), EnableKeyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.EnableKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(EnableKeyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Enables <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic rotation of the key material</a> for the specified symmetric customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>You cannot enable automatic rotation of asymmetric CMKs, CMKs with imported key material, or CMKs in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn enable_key_rotation(
        &self,
        input: EnableKeyRotationRequest,
    ) -> RusotoFuture<(), EnableKeyRotationError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.EnableKeyRotation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(EnableKeyRotationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Encrypts plaintext into ciphertext by using a customer master key (CMK). The <code>Encrypt</code> operation has two primary use cases:</p> <ul> <li> <p>You can encrypt small amounts of arbitrary data, such as a personal identifier or database password, or other sensitive information. </p> </li> <li> <p>You can use the <code>Encrypt</code> operation to move encrypted data from one AWS region to another. In the first region, generate a data key and use the plaintext key to encrypt the data. Then, in the new region, call the <code>Encrypt</code> method on same plaintext data key. Now, you can safely move the encrypted data and encrypted data key to the new region, and decrypt in the new region when necessary.</p> </li> </ul> <p>You don't need to use the <code>Encrypt</code> operation to encrypt a data key. The <a>GenerateDataKey</a> and <a>GenerateDataKeyPair</a> operations return a plaintext data key and an encrypted copy of that data key.</p> <p>When you encrypt data, you must specify a symmetric or asymmetric CMK to use in the encryption operation. The CMK must have a <code>KeyUsage</code> value of <code>ENCRYPT_DECRYPT.</code> To find the <code>KeyUsage</code> of a CMK, use the <a>DescribeKey</a> operation. </p> <p>If you use a symmetric CMK, you can use an encryption context to add additional security to your encryption operation. If you specify an <code>EncryptionContext</code> when encrypting data, you must specify the same encryption context (a case-sensitive exact match) when decrypting the data. Otherwise, the request to decrypt fails with an <code>InvalidCiphertextException</code>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>If you specify an asymmetric CMK, you must also specify the encryption algorithm. The algorithm must be compatible with the CMK type.</p> <important> <p>When you use an asymmetric CMK to encrypt or reencrypt data, be sure to record the CMK and encryption algorithm that you choose. You will be required to provide the same CMK and encryption algorithm when you decrypt the data. If the CMK and algorithm do not match the values used to encrypt the data, the decrypt operation fails.</p> <p>You are not required to supply the CMK ID and encryption algorithm when you decrypt with symmetric CMKs because AWS KMS stores this information in the ciphertext blob. AWS KMS cannot store metadata in ciphertext generated with asymmetric keys. The standard format for asymmetric key ciphertext does not include configurable fields.</p> </important> <p>The maximum size of the data that you can encrypt varies with the type of CMK and the encryption algorithm that you choose.</p> <ul> <li> <p>Symmetric CMKs</p> <ul> <li> <p> <code>SYMMETRIC_DEFAULT</code>: 4096 bytes</p> </li> </ul> </li> <li> <p> <code>RSA_2048</code> </p> <ul> <li> <p> <code>RSAES_OAEP_SHA_1</code>: 214 bytes</p> </li> <li> <p> <code>RSAES_OAEP_SHA_256</code>: 190 bytes</p> </li> </ul> </li> <li> <p> <code>RSA_3072</code> </p> <ul> <li> <p> <code>RSAES_OAEP_SHA_1</code>: 342 bytes</p> </li> <li> <p> <code>RSAES_OAEP_SHA_256</code>: 318 bytes</p> </li> </ul> </li> <li> <p> <code>RSA_4096</code> </p> <ul> <li> <p> <code>RSAES_OAEP_SHA_1</code>: 470 bytes</p> </li> <li> <p> <code>RSAES_OAEP_SHA_256</code>: 446 bytes</p> </li> </ul> </li> </ul> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN or alias ARN in the value of the KeyId parameter.</p>
    fn encrypt(&self, input: EncryptRequest) -> RusotoFuture<EncryptResponse, EncryptError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.Encrypt");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<EncryptResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(EncryptError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Generates a unique symmetric data key. This operation returns a plaintext copy of the data key and a copy that is encrypted under a customer master key (CMK) that you specify. You can use the plaintext key to encrypt your data outside of AWS KMS and store the encrypted data key with the encrypted data.</p> <p> <code>GenerateDataKey</code> returns a unique data key for each request. The bytes in the key are not related to the caller or CMK that is used to encrypt the data key.</p> <p>To generate a data key, specify the symmetric CMK that will be used to encrypt the data key. You cannot use an asymmetric CMK to generate data keys.</p> <p>You must also specify the length of the data key. Use either the <code>KeySpec</code> or <code>NumberOfBytes</code> parameters (but not both). For 128-bit and 256-bit data keys, use the <code>KeySpec</code> parameter. </p> <p>If the operation succeeds, the plaintext copy of the data key is in the <code>Plaintext</code> field of the response, and the encrypted copy of the data key in the <code>CiphertextBlob</code> field.</p> <p>To get only an encrypted copy of the data key, use <a>GenerateDataKeyWithoutPlaintext</a>. To generate an asymmetric data key pair, use the <a>GenerateDataKeyPair</a> or <a>GenerateDataKeyPairWithoutPlaintext</a> operation. To get a cryptographically secure random byte string, use <a>GenerateRandom</a>.</p> <p>You can use the optional encryption context to add additional security to the encryption operation. If you specify an <code>EncryptionContext</code>, you must specify the same encryption context (a case-sensitive exact match) when decrypting the encrypted data key. Otherwise, the request to decrypt fails with an InvalidCiphertextException. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>We recommend that you use the following pattern to encrypt data locally in your application:</p> <ol> <li> <p>Use the <code>GenerateDataKey</code> operation to get a data encryption key.</p> </li> <li> <p>Use the plaintext data key (returned in the <code>Plaintext</code> field of the response) to encrypt data locally, then erase the plaintext data key from memory.</p> </li> <li> <p>Store the encrypted data key (returned in the <code>CiphertextBlob</code> field of the response) alongside the locally encrypted data.</p> </li> </ol> <p>To decrypt data locally:</p> <ol> <li> <p>Use the <a>Decrypt</a> operation to decrypt the encrypted data key. The operation returns a plaintext copy of the data key.</p> </li> <li> <p>Use the plaintext data key to decrypt data locally, then erase the plaintext data key from memory.</p> </li> </ol></p>
    fn generate_data_key(
        &self,
        input: GenerateDataKeyRequest,
    ) -> RusotoFuture<GenerateDataKeyResponse, GenerateDataKeyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GenerateDataKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GenerateDataKeyResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GenerateDataKeyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Generates a unique asymmetric data key pair. The <code>GenerateDataKeyPair</code> operation returns a plaintext public key, a plaintext private key, and a copy of the private key that is encrypted under the symmetric CMK you specify. You can use the data key pair to perform asymmetric cryptography outside of AWS KMS.</p> <p> <code>GenerateDataKeyPair</code> returns a unique data key pair for each request. The bytes in the keys are not related to the caller or the CMK that is used to encrypt the private key.</p> <p>You can use the public key that <code>GenerateDataKeyPair</code> returns to encrypt data or verify a signature outside of AWS KMS. Then, store the encrypted private key with the data. When you are ready to decrypt data or sign a message, you can use the <a>Decrypt</a> operation to decrypt the encrypted private key.</p> <p>To generate a data key pair, you must specify a symmetric customer master key (CMK) to encrypt the private key in a data key pair. You cannot use an asymmetric CMK. To get the type of your CMK, use the <a>DescribeKey</a> operation.</p> <p>If you are using the data key pair to encrypt data, or for any operation where you don't immediately need a private key, consider using the <a>GenerateDataKeyPairWithoutPlaintext</a> operation. <code>GenerateDataKeyPairWithoutPlaintext</code> returns a plaintext public key and an encrypted private key, but omits the plaintext private key that you need only to decrypt ciphertext or sign a message. Later, when you need to decrypt the data or sign a message, use the <a>Decrypt</a> operation to decrypt the encrypted private key in the data key pair.</p> <p>You can use the optional encryption context to add additional security to the encryption operation. If you specify an <code>EncryptionContext</code>, you must specify the same encryption context (a case-sensitive exact match) when decrypting the encrypted data key. Otherwise, the request to decrypt fails with an InvalidCiphertextException. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn generate_data_key_pair(
        &self,
        input: GenerateDataKeyPairRequest,
    ) -> RusotoFuture<GenerateDataKeyPairResponse, GenerateDataKeyPairError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GenerateDataKeyPair");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GenerateDataKeyPairResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GenerateDataKeyPairError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Generates a unique asymmetric data key pair. The <code>GenerateDataKeyPairWithoutPlaintext</code> operation returns a plaintext public key and a copy of the private key that is encrypted under the symmetric CMK you specify. Unlike <a>GenerateDataKeyPair</a>, this operation does not return a plaintext private key. </p> <p>To generate a data key pair, you must specify a symmetric customer master key (CMK) to encrypt the private key in the data key pair. You cannot use an asymmetric CMK. To get the type of your CMK, use the <code>KeySpec</code> field in the <a>DescribeKey</a> response.</p> <p>You can use the public key that <code>GenerateDataKeyPairWithoutPlaintext</code> returns to encrypt data or verify a signature outside of AWS KMS. Then, store the encrypted private key with the data. When you are ready to decrypt data or sign a message, you can use the <a>Decrypt</a> operation to decrypt the encrypted private key.</p> <p> <code>GenerateDataKeyPairWithoutPlaintext</code> returns a unique data key pair for each request. The bytes in the key are not related to the caller or CMK that is used to encrypt the private key.</p> <p>You can use the optional encryption context to add additional security to the encryption operation. If you specify an <code>EncryptionContext</code>, you must specify the same encryption context (a case-sensitive exact match) when decrypting the encrypted data key. Otherwise, the request to decrypt fails with an InvalidCiphertextException. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn generate_data_key_pair_without_plaintext(
        &self,
        input: GenerateDataKeyPairWithoutPlaintextRequest,
    ) -> RusotoFuture<
        GenerateDataKeyPairWithoutPlaintextResponse,
        GenerateDataKeyPairWithoutPlaintextError,
    > {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "TrentService.GenerateDataKeyPairWithoutPlaintext",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GenerateDataKeyPairWithoutPlaintextResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GenerateDataKeyPairWithoutPlaintextError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Generates a unique symmetric data key. This operation returns a data key that is encrypted under a customer master key (CMK) that you specify. To request an asymmetric data key pair, use the <a>GenerateDataKeyPair</a> or <a>GenerateDataKeyPairWithoutPlaintext</a> operations.</p> <p> <code>GenerateDataKeyWithoutPlaintext</code> is identical to the <a>GenerateDataKey</a> operation except that returns only the encrypted copy of the data key. This operation is useful for systems that need to encrypt data at some point, but not immediately. When you need to encrypt the data, you call the <a>Decrypt</a> operation on the encrypted copy of the key. </p> <p>It's also useful in distributed systems with different levels of trust. For example, you might store encrypted data in containers. One component of your system creates new containers and stores an encrypted data key with each container. Then, a different component puts the data into the containers. That component first decrypts the data key, uses the plaintext data key to encrypt data, puts the encrypted data into the container, and then destroys the plaintext data key. In this system, the component that creates the containers never sees the plaintext data key.</p> <p> <code>GenerateDataKeyWithoutPlaintext</code> returns a unique data key for each request. The bytes in the keys are not related to the caller or CMK that is used to encrypt the private key.</p> <p>To generate a data key, you must specify the symmetric customer master key (CMK) that is used to encrypt the data key. You cannot use an asymmetric CMK to generate a data key. To get the type of your CMK, use the <code>KeySpec</code> field in the <a>DescribeKey</a> response. You must also specify the length of the data key using either the <code>KeySpec</code> or <code>NumberOfBytes</code> field (but not both). For common key lengths (128-bit and 256-bit symmetric keys), use the <code>KeySpec</code> parameter. </p> <p>If the operation succeeds, you will find the plaintext copy of the data key in the <code>Plaintext</code> field of the response, and the encrypted copy of the data key in the <code>CiphertextBlob</code> field.</p> <p>You can use the optional encryption context to add additional security to the encryption operation. If you specify an <code>EncryptionContext</code>, you must specify the same encryption context (a case-sensitive exact match) when decrypting the encrypted data key. Otherwise, the request to decrypt fails with an InvalidCiphertextException. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context">Encryption Context</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn generate_data_key_without_plaintext(
        &self,
        input: GenerateDataKeyWithoutPlaintextRequest,
    ) -> RusotoFuture<GenerateDataKeyWithoutPlaintextResponse, GenerateDataKeyWithoutPlaintextError>
    {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "TrentService.GenerateDataKeyWithoutPlaintext",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GenerateDataKeyWithoutPlaintextResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GenerateDataKeyWithoutPlaintextError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns a random byte string that is cryptographically secure.</p> <p>By default, the random byte string is generated in AWS KMS. To generate the byte string in the AWS CloudHSM cluster that is associated with a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, specify the custom key store ID.</p> <p>For more information about entropy and random number generation, see the <a href="https://d0.awsstatic.com/whitepapers/KMS-Cryptographic-Details.pdf">AWS Key Management Service Cryptographic Details</a> whitepaper.</p>
    fn generate_random(
        &self,
        input: GenerateRandomRequest,
    ) -> RusotoFuture<GenerateRandomResponse, GenerateRandomError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GenerateRandom");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GenerateRandomResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GenerateRandomError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a key policy attached to the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p>
    fn get_key_policy(
        &self,
        input: GetKeyPolicyRequest,
    ) -> RusotoFuture<GetKeyPolicyResponse, GetKeyPolicyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GetKeyPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetKeyPolicyResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetKeyPolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a Boolean value that indicates whether <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic rotation of the key material</a> is enabled for the specified customer master key (CMK).</p> <p>You cannot enable automatic rotation of asymmetric CMKs, CMKs with imported key material, or CMKs in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. The key rotation status for these CMKs is always <code>false</code>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <ul> <li> <p>Disabled: The key rotation status does not change when you disable a CMK. However, while the CMK is disabled, AWS KMS does not rotate the backing key.</p> </li> <li> <p>Pending deletion: While a CMK is pending deletion, its key rotation status is <code>false</code> and AWS KMS does not rotate the backing key. If you cancel the deletion, the original key rotation status is restored.</p> </li> </ul> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter.</p>
    fn get_key_rotation_status(
        &self,
        input: GetKeyRotationStatusRequest,
    ) -> RusotoFuture<GetKeyRotationStatusResponse, GetKeyRotationStatusError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GetKeyRotationStatus");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetKeyRotationStatusResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetKeyRotationStatusError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns the items you need to import key material into a symmetric, customer managed customer master key (CMK). For more information about importing key material into AWS KMS, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>This operation returns a public key and an import token. Use the public key to encrypt the symmetric key material. Store the import token to send with a subsequent <a>ImportKeyMaterial</a> request.</p> <p>You must specify the key ID of the symmetric CMK into which you will import key material. This CMK's <code>Origin</code> must be <code>EXTERNAL</code>. You must also specify the wrapping algorithm and type of wrapping key (public key) that you will use to encrypt the key material. You cannot perform this operation on an asymmetric CMK or on any CMK in a different AWS account.</p> <p>To import key material, you must use the public key and import token from the same response. These items are valid for 24 hours. The expiration date and time appear in the <code>GetParametersForImport</code> response. You cannot use an expired token in an <a>ImportKeyMaterial</a> request. If your key and token expire, send another <code>GetParametersForImport</code> request.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn get_parameters_for_import(
        &self,
        input: GetParametersForImportRequest,
    ) -> RusotoFuture<GetParametersForImportResponse, GetParametersForImportError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GetParametersForImport");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetParametersForImportResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetParametersForImportError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns the public key of an asymmetric CMK. Unlike the private key of a asymmetric CMK, which never leaves AWS KMS unencrypted, callers with <code>kms:GetPublicKey</code> permission can download the public key of an asymmetric CMK. You can share the public key to allow others to encrypt messages and verify signatures outside of AWS KMS. For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>You do not need to download the public key. Instead, you can use the public key within AWS KMS by calling the <a>Encrypt</a>, <a>ReEncrypt</a>, or <a>Verify</a> operations with the identifier of an asymmetric CMK. When you use the public key within AWS KMS, you benefit from the authentication, authorization, and logging that are part of every AWS KMS operation. You also reduce of risk of encrypting data that cannot be decrypted. These features are not effective outside of AWS KMS. For details, see <a href="kms/latest/developerguide/get-public-key.html#get-public-key-considerations">Special Considerations for Downloading Public Keys</a>.</p> <p>To help you use the public key safely outside of AWS KMS, <code>GetPublicKey</code> returns important information about the public key in the response, including:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_GetPublicKey.html#KMS-GetPublicKey-response-CustomerMasterKeySpec">CustomerMasterKeySpec</a>: The type of key material in the public key, such as <code>RSA_4096</code> or <code>ECC_NIST_P521</code>.</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_GetPublicKey.html#KMS-GetPublicKey-response-KeyUsage">KeyUsage</a>: Whether the key is used for encryption or signing.</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_GetPublicKey.html#KMS-GetPublicKey-response-EncryptionAlgorithms">EncryptionAlgorithms</a> or <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_GetPublicKey.html#KMS-GetPublicKey-response-SigningAlgorithms">SigningAlgorithms</a>: A list of the encryption algorithms or the signing algorithms for the key.</p> </li> </ul> <p>Although AWS KMS cannot enforce these restrictions on external operations, it is crucial that you use this information to prevent the public key from being used improperly. For example, you can prevent a public signing key from being used encrypt data, or prevent a public key from being used with an encryption algorithm that is not supported by AWS KMS. You can also avoid errors, such as using the wrong signing algorithm in a verification operation.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn get_public_key(
        &self,
        input: GetPublicKeyRequest,
    ) -> RusotoFuture<GetPublicKeyResponse, GetPublicKeyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.GetPublicKey");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetPublicKeyResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetPublicKeyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Imports key material into an existing symmetric AWS KMS customer master key (CMK) that was created without key material. After you successfully import key material into a CMK, you can <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html#reimport-key-material">reimport the same key material</a> into that CMK, but you cannot import different key material.</p> <p>You cannot perform this operation on an asymmetric CMK or on any CMK in a different AWS account. For more information about creating CMKs with no key material and then importing key material, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">Importing Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>Before using this operation, call <a>GetParametersForImport</a>. Its response includes a public key and an import token. Use the public key to encrypt the key material. Then, submit the import token from the same <code>GetParametersForImport</code> response.</p> <p>When calling this operation, you must specify the following values:</p> <ul> <li> <p>The key ID or key ARN of a CMK with no key material. Its <code>Origin</code> must be <code>EXTERNAL</code>.</p> <p>To create a CMK with no key material, call <a>CreateKey</a> and set the value of its <code>Origin</code> parameter to <code>EXTERNAL</code>. To get the <code>Origin</code> of a CMK, call <a>DescribeKey</a>.)</p> </li> <li> <p>The encrypted key material. To get the public key to encrypt the key material, call <a>GetParametersForImport</a>.</p> </li> <li> <p>The import token that <a>GetParametersForImport</a> returned. You must use a public key and token from the same <code>GetParametersForImport</code> response.</p> </li> <li> <p>Whether the key material expires and if so, when. If you set an expiration date, AWS KMS deletes the key material from the CMK on the specified date, and the CMK becomes unusable. To use the CMK again, you must reimport the same key material. The only way to change an expiration date is by reimporting the same key material and specifying a new expiration date. </p> </li> </ul> <p>When this operation is successful, the key state of the CMK changes from <code>PendingImport</code> to <code>Enabled</code>, and you can use the CMK.</p> <p>If this operation fails, use the exception to help determine the problem. If the error is related to the key material, the import token, or wrapping key, use <a>GetParametersForImport</a> to get a new public key and import token for the CMK and repeat the import procedure. For help, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html#importing-keys-overview">How To Import Key Material</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn import_key_material(
        &self,
        input: ImportKeyMaterialRequest,
    ) -> RusotoFuture<ImportKeyMaterialResponse, ImportKeyMaterialError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ImportKeyMaterial");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ImportKeyMaterialResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ImportKeyMaterialError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a list of aliases in the caller's AWS account and region. You cannot list aliases in other accounts. For more information about aliases, see <a>CreateAlias</a>.</p> <p>By default, the ListAliases command returns all aliases in the account and region. To get only the aliases that point to a particular customer master key (CMK), use the <code>KeyId</code> parameter.</p> <p>The <code>ListAliases</code> response can include aliases that you created and associated with your customer managed CMKs, and aliases that AWS created and associated with AWS managed CMKs in your account. You can recognize AWS aliases because their names have the format <code>aws/&lt;service-name&gt;</code>, such as <code>aws/dynamodb</code>.</p> <p>The response might also include aliases that have no <code>TargetKeyId</code> field. These are predefined aliases that AWS has created but has not yet associated with a CMK. Aliases that AWS creates in your account, including predefined aliases, do not count against your <a href="https://docs.aws.amazon.com/kms/latest/developerguide/limits.html#aliases-limit">AWS KMS aliases limit</a>.</p>
    fn list_aliases(
        &self,
        input: ListAliasesRequest,
    ) -> RusotoFuture<ListAliasesResponse, ListAliasesError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListAliases");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListAliasesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListAliasesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a list of all grants for the specified customer master key (CMK).</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter.</p>
    fn list_grants(
        &self,
        input: ListGrantsRequest,
    ) -> RusotoFuture<ListGrantsResponse, ListGrantsError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListGrants");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListGrantsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListGrantsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the names of the key policies that are attached to a customer master key (CMK). This operation is designed to get policy names that you can use in a <a>GetKeyPolicy</a> operation. However, the only valid policy name is <code>default</code>. You cannot perform this operation on a CMK in a different AWS account.</p>
    fn list_key_policies(
        &self,
        input: ListKeyPoliciesRequest,
    ) -> RusotoFuture<ListKeyPoliciesResponse, ListKeyPoliciesError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListKeyPolicies");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListKeyPoliciesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListKeyPoliciesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a list of all customer master keys (CMKs) in the caller's AWS account and Region.</p>
    fn list_keys(&self, input: ListKeysRequest) -> RusotoFuture<ListKeysResponse, ListKeysError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListKeys");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListKeysResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListKeysError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of all tags for the specified customer master key (CMK).</p> <p>You cannot perform this operation on a CMK in a different AWS account.</p>
    fn list_resource_tags(
        &self,
        input: ListResourceTagsRequest,
    ) -> RusotoFuture<ListResourceTagsResponse, ListResourceTagsError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListResourceTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListResourceTagsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListResourceTagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of all grants for which the grant's <code>RetiringPrincipal</code> matches the one specified.</p> <p>A typical use is to list all grants that you are able to retire. To retire a grant, use <a>RetireGrant</a>.</p>
    fn list_retirable_grants(
        &self,
        input: ListRetirableGrantsRequest,
    ) -> RusotoFuture<ListGrantsResponse, ListRetirableGrantsError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ListRetirableGrants");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListGrantsResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListRetirableGrantsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Attaches a key policy to the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about key policies, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Key Policies</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn put_key_policy(&self, input: PutKeyPolicyRequest) -> RusotoFuture<(), PutKeyPolicyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.PutKeyPolicy");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutKeyPolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Decrypts ciphertext and then reencrypts it entirely within AWS KMS. You can use this operation to change the customer master key (CMK) under which data is encrypted, such as when you <a href="kms/latest/developerguide/rotate-keys.html#rotate-keys-manually">manually rotate</a> a CMK or change the CMK that protects a ciphertext. You can also use it to reencrypt ciphertext under the same CMK, such as to change the encryption context of a ciphertext. </p> <p>The <code>ReEncrypt</code> operation can decrypt ciphertext that was encrypted by using an AWS KMS CMK in an AWS KMS operation, such as <a>Encrypt</a> or <a>GenerateDataKey</a>. It can also decrypt ciphertext that was encrypted by using the public key of an asymmetric CMK outside of AWS KMS. However, it cannot decrypt ciphertext produced by other libraries, such as the <a href="https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/">AWS Encryption SDK</a> or <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingClientSideEncryption.html">Amazon S3 client-side encryption</a>. These libraries return a ciphertext format that is incompatible with AWS KMS.</p> <p>When you use the <code>ReEncrypt</code> operation, you need to provide information for the decrypt operation and the subsequent encrypt operation.</p> <ul> <li> <p>If your ciphertext was encrypted under an asymmetric CMK, you must identify the <i>source CMK</i>, that is, the CMK that encrypted the ciphertext. You must also supply the encryption algorithm that was used. This information is required to decrypt the data.</p> </li> <li> <p>It is optional, but you can specify a source CMK even when the ciphertext was encrypted under a symmetric CMK. This ensures that the ciphertext is decrypted only by using a particular CMK. If the CMK that you specify cannot decrypt the ciphertext, the <code>ReEncrypt</code> operation fails.</p> </li> <li> <p>To reencrypt the data, you must specify the <i>destination CMK</i>, that is, the CMK that re-encrypts the data after it is decrypted. You can select a symmetric or asymmetric CMK. If the destination CMK is an asymmetric CMK, you must also provide the encryption algorithm. The algorithm that you choose must be compatible with the CMK.</p> <important> <p>When you use an asymmetric CMK to encrypt or reencrypt data, be sure to record the CMK and encryption algorithm that you choose. You will be required to provide the same CMK and encryption algorithm when you decrypt the data. If the CMK and algorithm do not match the values used to encrypt the data, the decrypt operation fails.</p> <p>You are not required to supply the CMK ID and encryption algorithm when you decrypt with symmetric CMKs because AWS KMS stores this information in the ciphertext blob. AWS KMS cannot store metadata in ciphertext generated with asymmetric keys. The standard format for asymmetric key ciphertext does not include configurable fields.</p> </important> </li> </ul> <p>Unlike other AWS KMS API operations, <code>ReEncrypt</code> callers must have two permissions:</p> <ul> <li> <p> <code>kms:EncryptFrom</code> permission on the source CMK</p> </li> <li> <p> <code>kms:EncryptTo</code> permission on the destination CMK</p> </li> </ul> <p>To permit reencryption from</p> <p> or to a CMK, include the <code>"kms:ReEncrypt*"</code> permission in your <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">key policy</a>. This permission is automatically included in the key policy when you use the console to create a CMK. But you must include it manually when you create a CMK programmatically or when you use the <a>PutKeyPolicy</a> operation set a key policy.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn re_encrypt(
        &self,
        input: ReEncryptRequest,
    ) -> RusotoFuture<ReEncryptResponse, ReEncryptError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ReEncrypt");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ReEncryptResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ReEncryptError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retires a grant. To clean up, you can retire a grant when you're done using it. You should revoke a grant when you intend to actively deny operations that depend on it. The following are permitted to call this API:</p> <ul> <li> <p>The AWS account (root user) under which the grant was created</p> </li> <li> <p>The <code>RetiringPrincipal</code>, if present in the grant</p> </li> <li> <p>The <code>GranteePrincipal</code>, if <code>RetireGrant</code> is an operation specified in the grant</p> </li> </ul> <p>You must identify the grant to retire by its grant token or by a combination of the grant ID and the Amazon Resource Name (ARN) of the customer master key (CMK). A grant token is a unique variable-length base64-encoded string. A grant ID is a 64 character unique identifier of a grant. The <a>CreateGrant</a> operation returns both.</p>
    fn retire_grant(&self, input: RetireGrantRequest) -> RusotoFuture<(), RetireGrantError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.RetireGrant");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RetireGrantError::from_response(response))),
                )
            }
        })
    }

    /// <p>Revokes the specified grant for the specified customer master key (CMK). You can revoke a grant to actively deny operations that depend on it.</p> <p>To perform this operation on a CMK in a different AWS account, specify the key ARN in the value of the <code>KeyId</code> parameter.</p>
    fn revoke_grant(&self, input: RevokeGrantRequest) -> RusotoFuture<(), RevokeGrantError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.RevokeGrant");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RevokeGrantError::from_response(response))),
                )
            }
        })
    }

    /// <p>Schedules the deletion of a customer master key (CMK). You may provide a waiting period, specified in days, before deletion occurs. If you do not provide a waiting period, the default period of 30 days is used. When this operation is successful, the key state of the CMK changes to <code>PendingDeletion</code>. Before the waiting period ends, you can use <a>CancelKeyDeletion</a> to cancel the deletion of the CMK. After the waiting period ends, AWS KMS deletes the CMK and all AWS KMS data associated with it, including all aliases that refer to it.</p> <important> <p>Deleting a CMK is a destructive and potentially dangerous operation. When a CMK is deleted, all data that was encrypted under the CMK is unrecoverable. To prevent the use of a CMK without deleting it, use <a>DisableKey</a>.</p> </important> <p>If you schedule deletion of a CMK from a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>, when the waiting period expires, <code>ScheduleKeyDeletion</code> deletes the CMK from AWS KMS. Then AWS KMS makes a best effort to delete the key material from the associated AWS CloudHSM cluster. However, you might need to manually <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#fix-keystore-orphaned-key">delete the orphaned key material</a> from the cluster and its backups.</p> <p>You cannot perform this operation on a CMK in a different AWS account.</p> <p>For more information about scheduling a CMK for deletion, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/deleting-keys.html">Deleting Customer Master Keys</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn schedule_key_deletion(
        &self,
        input: ScheduleKeyDeletionRequest,
    ) -> RusotoFuture<ScheduleKeyDeletionResponse, ScheduleKeyDeletionError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.ScheduleKeyDeletion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ScheduleKeyDeletionResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ScheduleKeyDeletionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a <a href="https://en.wikipedia.org/wiki/Digital_signature">digital signature</a> for a message or message digest by using the private key in an asymmetric CMK. To verify the signature, use the <a>Verify</a> operation, or use the public key in the same asymmetric CMK outside of AWS KMS. For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>Digital signatures are generated and verified by using asymmetric key pair, such as an RSA or ECC pair that is represented by an asymmetric customer master key (CMK). The key owner (or an authorized user) uses their private key to sign a message. Anyone with the public key can verify that the message was signed with that particular private key and that the message hasn't changed since it was signed. </p> <p>To use the <code>Sign</code> operation, provide the following information:</p> <ul> <li> <p>Use the <code>KeyId</code> parameter to identify an asymmetric CMK with a <code>KeyUsage</code> value of <code>SIGN_VERIFY</code>. To get the <code>KeyUsage</code> value of a CMK, use the <a>DescribeKey</a> operation. The caller must have <code>kms:Sign</code> permission on the CMK.</p> </li> <li> <p>Use the <code>Message</code> parameter to specify the message or message digest to sign. You can submit messages of up to 4096 bytes. To sign a larger message, generate a hash digest of the message, and then provide the hash digest in the <code>Message</code> parameter. To indicate whether the message is a full message or a digest, use the <code>MessageType</code> parameter.</p> </li> <li> <p>Choose a signing algorithm that is compatible with the CMK. </p> </li> </ul> <important> <p>When signing a message, be sure to record the CMK and the signing algorithm. This information is required to verify the signature.</p> </important> <p>To verify the signature that this operation generates, use the <a>Verify</a> operation. Or use the <a>GetPublicKey</a> operation to download the public key and then use the public key to verify the signature outside of AWS KMS. </p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn sign(&self, input: SignRequest) -> RusotoFuture<SignResponse, SignError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.Sign");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<SignResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SignError::from_response(response))),
                )
            }
        })
    }

    /// <p>Adds or edits tags for a customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>Each tag consists of a tag key and a tag value. Tag keys and tag values are both required, but tag values can be empty (null) strings.</p> <p>You can only use a tag key once for each CMK. If you use the tag key again, AWS KMS replaces the current tag value with the specified value.</p> <p>For information about the rules that apply to tag keys and tag values, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/allocation-tag-restrictions.html">User-Defined Tag Restrictions</a> in the <i>AWS Billing and Cost Management User Guide</i>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes the specified tags from the specified customer master key (CMK). You cannot perform this operation on a CMK in a different AWS account.</p> <p>To remove a tag, specify the tag key. To change the tag value of an existing tag key, use <a>TagResource</a>.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Associates an existing AWS KMS alias with a different customer master key (CMK). Each alias is associated with only one CMK at a time, although a CMK can have multiple aliases. The alias and the CMK must be in the same AWS account and region. You cannot perform this operation on an alias in a different AWS account. </p> <p>The current and new CMK must be the same type (both symmetric or both asymmetric), and they must have the same key usage (<code>ENCRYPT_DECRYPT</code> or <code>SIGN_VERIFY</code>). This restriction prevents errors in code that uses aliases. If you must assign an alias to a different type of CMK, use <a>DeleteAlias</a> to delete the old alias and <a>CreateAlias</a> to create a new alias.</p> <p>You cannot use <code>UpdateAlias</code> to change an alias name. To change an alias name, use <a>DeleteAlias</a> to delete the old alias and <a>CreateAlias</a> to create a new alias.</p> <p>Because an alias is not a property of a CMK, you can create, update, and delete the aliases of a CMK without affecting the CMK. Also, aliases do not appear in the response from the <a>DescribeKey</a> operation. To get the aliases of all CMKs in the account, use the <a>ListAliases</a> operation. </p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn update_alias(&self, input: UpdateAliasRequest) -> RusotoFuture<(), UpdateAliasError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.UpdateAlias");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateAliasError::from_response(response))),
                )
            }
        })
    }

    /// <p>Changes the properties of a custom key store. Use the <code>CustomKeyStoreId</code> parameter to identify the custom key store you want to edit. Use the remaining parameters to change the properties of the custom key store.</p> <p>You can only update a custom key store that is disconnected. To disconnect the custom key store, use <a>DisconnectCustomKeyStore</a>. To reconnect the custom key store after the update completes, use <a>ConnectCustomKeyStore</a>. To find the connection state of a custom key store, use the <a>DescribeCustomKeyStores</a> operation.</p> <p>Use the parameters of <code>UpdateCustomKeyStore</code> to edit your keystore settings.</p> <ul> <li> <p>Use the <b>NewCustomKeyStoreName</b> parameter to change the friendly name of the custom key store to the value that you specify.</p> <p> </p> </li> <li> <p>Use the <b>KeyStorePassword</b> parameter tell AWS KMS the current password of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-store-concepts.html#concept-kmsuser"> <code>kmsuser</code> crypto user (CU)</a> in the associated AWS CloudHSM cluster. You can use this parameter to <a href="https://docs.aws.amazon.com/kms/latest/developerguide/fix-keystore.html#fix-keystore-password">fix connection failures</a> that occur when AWS KMS cannot log into the associated cluster because the <code>kmsuser</code> password has changed. This value does not change the password in the AWS CloudHSM cluster.</p> <p> </p> </li> <li> <p>Use the <b>CloudHsmClusterId</b> parameter to associate the custom key store with a different, but related, AWS CloudHSM cluster. You can use this parameter to repair a custom key store if its AWS CloudHSM cluster becomes corrupted or is deleted, or when you need to create or restore a cluster from a backup. </p> </li> </ul> <p>If the operation succeeds, it returns a JSON object with no properties.</p> <p>This operation is part of the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">Custom Key Store feature</a> feature in AWS KMS, which combines the convenience and extensive integration of AWS KMS with the isolation and control of a single-tenant key store.</p>
    fn update_custom_key_store(
        &self,
        input: UpdateCustomKeyStoreRequest,
    ) -> RusotoFuture<UpdateCustomKeyStoreResponse, UpdateCustomKeyStoreError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.UpdateCustomKeyStore");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateCustomKeyStoreResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateCustomKeyStoreError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates the description of a customer master key (CMK). To see the description of a CMK, use <a>DescribeKey</a>. </p> <p>You cannot perform this operation on a CMK in a different AWS account.</p> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn update_key_description(
        &self,
        input: UpdateKeyDescriptionRequest,
    ) -> RusotoFuture<(), UpdateKeyDescriptionError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.UpdateKeyDescription");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateKeyDescriptionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Verifies a digital signature that was generated by the <a>Sign</a> operation. This operation requires an asymmetric CMK with a <code>KeyUsage</code> value of <code>SIGN_VERIFY</code>.</p> <p/> <p>Verification confirms that an authorized user signed the message with the specified key and signing algorithm, and the message hasn't changed since it was signed. A digital signature is generated by using the private key in an asymmetric CMK. The signature is verified by using the public key in the same asymmetric CMK. For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Using Symmetric and Asymmetric CMKs</a> in the <i>AWS Key Management Service Developer Guide</i>.</p> <p>To verify a digital signature, you can use the <code>Verify</code> operation. Specify the same asymmetric CMK that was used by the <code>Sign</code> operation to generate the digital signature.</p> <p>You can also verify the digital signature by using the public key of the CMK outside of AWS KMS. Use the <a>GetPublicKey</a> operation to download the public key in the asymmetric CMK and then use the public key to verify the signature outside of AWS KMS.</p> <p>The advantage of using the <code>Verify</code> operation is that it is performed within AWS KMS. As a result, it's easy to call, the operation is performed within the FIPS boundary, it is logged in AWS CloudTrail, and you can use key policy and IAM policy to determine who is authorized to use the CMK to verify signatures.</p> <important> <p>The result of the <code>Verify</code> operation, which is represented by its HTTP status code, does not indicate whether the signature verification succeeded or failed. To determine whether the signature was verified, see the <code>SignatureValid</code> field in the response. </p> </important> <p>The CMK that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">How Key State Affects Use of a Customer Master Key</a> in the <i>AWS Key Management Service Developer Guide</i>.</p>
    fn verify(&self, input: VerifyRequest) -> RusotoFuture<VerifyResponse, VerifyError> {
        let mut request = SignedRequest::new("POST", "kms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "TrentService.Verify");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<VerifyResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(VerifyError::from_response(response))),
                )
            }
        })
    }
}
