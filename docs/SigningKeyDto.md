# SigningKeyDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key_id** | **String** | External signing key id set by Fireblocks. | 
**signing_device_key_id** | **String** | The ID, name or label of the key specified on the customer's signing device. | 
**public_key_pem** | **String** | PEM encoded public key | 
**algorithm** | **String** | Algorithm and curve used for the signature. Can be: ECDSA_SECP256K1 or EDDSA_ED25519 | 
**enabled** | **bool** | True if the signing key is enabled | 
**vault_account_id** | Option<**f64**> | Id of the vault account which this key is linked to | 
**agent_user_id** | **String** | Id of user that represent agent servers that can sign with the key | 
**created_at** | **f64** | Creation date (timestamp) in milliseconds. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


