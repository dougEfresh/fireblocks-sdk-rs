# DelegationSummaryDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | [**Vec<models::AmountAndChainDescriptor>**](amountAndChainDescriptor.md) | An array of objects containing chain descriptors and associated amounts, representing active positions. | 
**inactive** | [**Vec<models::AmountAndChainDescriptor>**](amountAndChainDescriptor.md) | An array of objects containing chain descriptors and associated amounts, representing inactive positions. | 
**rewards_amount** | [**Vec<models::AmountAndChainDescriptor>**](amountAndChainDescriptor.md) | An array of objects containing chain descriptors and associated amounts, representing rewards positions. | 
**total_staked** | [**Vec<models::AmountAndChainDescriptor>**](amountAndChainDescriptor.md) | An array of objects with chain descriptors and total staked amounts, representing the combined staked totals of active and inactive positions. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


