# CreateTokenRequestDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**blockchain_id** | Option<**String**> | The id of the blockchain the request was initiated on | [optional]
**asset_id** | Option<**String**> | The base asset identifier of the blockchain you want to deploy to | [optional]
**vault_account_id** | **String** | The id of the vault account that initiated the request to issue the token | 
**create_params** | [**models::CreateTokenRequestDtoCreateParams**](CreateTokenRequestDto_createParams.md) |  | 
**display_name** | Option<**String**> |  | [optional]
**use_gasless** | Option<**bool**> | Indicates whether the token should be created in a gasless manner, utilizing the ERC-2771 standard. When set to true, the transaction will be relayed by a designated relayer. The workspace must be configured to use Fireblocks gasless relay. | [optional]
**fee** | Option<**String**> | Max fee amount for the write function transaction. interchangeable with the 'feeLevel' field | [optional]
**fee_level** | Option<**String**> | Fee level for the write function transaction. interchangeable with the 'fee' field | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


