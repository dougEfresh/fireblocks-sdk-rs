# StakeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vault_account_id** | **String** | The source vault account to stake from | 
**provider_id** | [**models::StakingProvider**](StakingProvider.md) |  | 
**stake_amount** | **String** | Amount of tokens to stake | 
**tx_note** | Option<**String**> | The note to associate with the stake transactions. | [optional]
**fee** | Option<**String**> | Represents the fee for a transaction, which can be specified as a percentage value. Only one of fee/feeLevel is required. | [optional]
**fee_level** | Option<[**models::FeeLevel**](FeeLevel.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


