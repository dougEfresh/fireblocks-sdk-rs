# SwapOperation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The id of the swap operation | 
**account_id** | **String** | The id of the vault account or account id | 
**provider_id** | **String** | The ID of the provider | 
**category** | [**models::ProviderCategoryEnum**](ProviderCategoryEnum.md) |  | 
**protocol** | [**models::SwapProviderProtocolsEnum**](SwapProviderProtocolsEnum.md) |  | 
**status** | **String** | **CREATED** – The swap request has been created but not yet started. **PENDING_USER_ACTION** – Awaiting a user action (e.g. signature or approval). **PENDING_PROVIDER_ACTION** – Awaiting the provider to process the request. **PROCESSING** – The swap is actively being executed on‐chain. **COMPLETED** – The swap has finished successfully. **CANCELED** – The swap was cancelled by user or provider before completion. **FAILED** – The swap attempted but encountered an error. | 
**input_amount** | **String** | The amount of tokens the swapper will provide | 
**input_asset** | **String** | The id of the asset the swapper will provide | 
**slippage_tolerance** | **f64** | The slippage tolerance is a percentage. The slippage tolerance is the maximum amount the price can change between the time the transaction is submitted and the time it is executed | 
**output_min_amount** | **String** | The minimum amount of tokens the swapper will receive | 
**output_max_amount** | **String** | Maximum amount of tokens that the swapper will receive | 
**output_asset** | **String** | The id of the asset the swapper will receive | 
**output_final_amount** | Option<**String**> | Final amount of tokens that the swapper will receive | [optional]
**required_actions** | [**Vec<models::SwapRequiredAction>**](SwapRequiredAction.md) | The required actions for the swap, including the type of action, the status of the action, and the transaction id | 
**error** | Option<[**models::SwapFlowError**](SwapFlowError.md)> |  | [optional]
**created_at** | **String** | The creation time of the swap operation (ISO Date time). | 
**updated_at** | **String** | The last update time of the swap operation (ISO Date time). | 
**created_by** | [**uuid::Uuid**](uuid::Uuid.md) | Fireblocks user id that issued the swap | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


