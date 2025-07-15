# MergeStakeAccountsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_id** | [**uuid::Uuid**](uuid::Uuid.md) | Id of the source position to merge from | 
**destination_id** | [**uuid::Uuid**](uuid::Uuid.md) | Id of the destination position to merge into | 
**fee** | Option<**String**> | Represents the fee for a transaction, which can be specified as a percentage value. Only one of fee/feeLevel is required. | [optional]
**fee_level** | Option<[**models::FeeLevel**](FeeLevel.md)> |  | [optional]
**tx_note** | Option<**String**> | The note to associate with the transactions. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


