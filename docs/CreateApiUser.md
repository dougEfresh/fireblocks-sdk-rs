# CreateApiUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**role** | [**models::ApiUserRole**](APIUserRole.md) |  | 
**name** | **String** | User Name | 
**csr_pem** | **String** | API requests are authenticated by providing in each request:   a. API Key in the `X-API-Key` header   b. Auth header - `Authorization: Bearer <JWT>` while the JWT is signed with an RSA 4096 private key.  When creating a new API Key, you need to generate an RSA 4096 private key and a CSR file.  The CSR file is uploaded to Fireblocks upon the user creation and used later on for signature validation (Auth JWT signature validation).  For more info read the following [article](https://developers.fireblocks.com/docs/manage-api-keys)  | 
**co_signer_setup_type** | Option<**String**> | Required for Signer/Admin API users that planned to be paired with an API Co-Signer Machine.  - SGX_MACHINE: For SGX enabled servers - FIREBLOCKS_CCMT: Fireblocks Communal Co-Signer (for Testnet workspaces only) - NITRO_MACHINE: For AWS Nitro Enclave enabled servers  For more information about Fireblocks Co-Signer setup please read the following [article](https://support.fireblocks.io/hc/en-us/articles/12006018592156-API-Co-Signer-Overview).  | [optional]
**co_signer_setup_is_first_user** | Option<**bool**> | Pass as `true`` if this is the first user on the your Co-Signer machine | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


