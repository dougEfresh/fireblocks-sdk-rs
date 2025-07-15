# QuoteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The id of the vault account or account id | [optional]
**input_amount** | **String** | The amount of tokens the swapper will provide, positive number, can be a decimal. | 
**input_asset** | **String** | The id of the asset the swapper will provide | 
**output_asset** | **String** | The id of the asset the swapper will receive | 
**slippage_tolerance** | **f64** | The slippage tolerance is a percentage. The slippage tolerance is the maximum amount the price can change between the time the transaction is submitted and the time it is executed | 
**protocol** | [**models::SwapProviderProtocolsEnum**](SwapProviderProtocolsEnum.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


