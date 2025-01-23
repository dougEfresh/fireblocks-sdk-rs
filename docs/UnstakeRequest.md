# UnstakeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | id of position to unstake | 
**fee** | Option<**String**> | Represents the fee for a transaction, which can be specified as a percentage value. Only one of fee/feeLevel is required. | [optional]
**fee_level** | Option<[**models::FeeLevel**](FeeLevel.md)> |  | [optional]
**tx_note** | Option<**String**> | The note to associate with the transactions. | [optional]
**amount** | Option<**String**> | The number of tokens to unstake.  This optional field is applicable only for liquid staking and allows for a partial unstake of the position.  If not provided, the entire position will be unstaked by default. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


