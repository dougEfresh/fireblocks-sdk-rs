# WalletQuoteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**protocol** | [**models::SwapProviderProtocolsEnum**](SwapProviderProtocolsEnum.md) |  | 
**input_amount** | **String** | The amount of tokens the swapper will provide | 
**input_asset** | **String** | The id of the asset the swapper will provide | 
**slippage_tolerance** | **f64** | The slippage tolerance is a percentage. The slippage tolerance is the maximum amount the price can change between the time the transaction is submitted and the time it is executed | 
**output_min_amount** | **String** | The minimum amount of tokens the swapper will receive | 
**output_max_amount** | **String** | Maximum amount of tokens that the swapper will receive | 
**output_asset** | **String** | The id of the asset the swapper will receive | 
**additional_data** | [**models::ProviderAdditionalData**](ProviderAdditionalData.md) |  | 
**provider_quote_id** | [**uuid::Uuid**](uuid::Uuid.md) | An identifier that uniquely identifies the received quote | 
**expired_at** | **String** | When was the received `providerQuoteId` is expired (ISO Date time). | 
**required_actions** | [**Vec<models::SwapRequiredActionsEnum>**](SwapRequiredActionsEnum.md) | The required actions for completing a swap operation | 
**estimated_fees** | [**models::QuoteFee**](QuoteFee.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


