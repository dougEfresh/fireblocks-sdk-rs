# XbSettlementFlowPreviewModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**flow_id** | **String** | The unique id for the cross-border flow. | 
**config_id** | [**uuid::Uuid**](uuid::Uuid.md) | Cross Bodrder configuraion unique id | 
**conversion_rate** | **String** | The conversion rate received from the on-ramp or off-ramp. | 
**input_amount** | [**models::XbSettlementAsset**](XBSettlementAsset.md) |  | 
**estimated_output_amount** | [**models::XbSettlementAsset**](XBSettlementAsset.md) |  | 
**total_estimated_fee** | [**models::XbSettlementAsset**](XBSettlementAsset.md) |  | 
**total_estimated_time** | **f64** | The total *estimated* time for executing the cross-border flow. | 
**steps** | [**models::XbSettlementFlowStepsRecord**](XBSettlementFlowStepsRecord.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


