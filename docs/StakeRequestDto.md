# StakeRequestDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vault_account_id** | **String** | The source vault account to stake from | 
**provider_id** | **String** | The ID of the provider | 
**stake_amount** | **String** | Amount of tokens to stake | 
**tx_note** | Option<**String**> | The note to associate with the stake transactions. | [optional]
**fee** | Option<**String**> | Represents the fee for a transaction, which can be specified as a percentage value. Only one of fee/feeLevel is required. | [optional]
**fee_level** | Option<**String**> | Represents the fee level for a transaction, which can be set as slow, medium, or fast. Only one of fee/feeLevel is required. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


