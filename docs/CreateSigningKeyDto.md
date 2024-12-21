# CreateSigningKeyDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signing_device_key_id** | **String** | The ID, name or label of the key specified on the customer's signing device. | 
**signed_cert_pem** | **String** | The signed certificate that includes the public key PEM of the signing key, signed by a validation key. | 
**agent_user_id** | **String** | Id of user to which this key belongs | 
**proof_of_ownership** | Option<[**models::CreateSigningKeyDtoProofOfOwnership**](CreateSigningKeyDto_proofOfOwnership.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


