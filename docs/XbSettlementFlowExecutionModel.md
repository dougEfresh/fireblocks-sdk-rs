# XbSettlementFlowExecutionModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**flow_id** | **String** | The unique id for the cross-border flow. | 
**config_id** | [**uuid::Uuid**](uuid::Uuid.md) | Cross Bodrder configuraion unique id | 
**input_amount** | [**models::XbSettlementAsset**](XBSettlementAsset.md) |  | 
**output_amount** | [**models::XbSettlementAsset**](XBSettlementAsset.md) |  | 
**total_fee** | [**models::XbSettlementAsset**](XBSettlementAsset.md) |  | 
**initiated_at** | **f64** | The time the cross-border flow executed in epoch format. | 
**initiated_by** | **String** | The id of the user which launched the flow | 
**state** | [**models::XbSettlementFlowExecutionStatus**](XBSettlementFlowExecutionStatus.md) |  | 
**steps** | [**models::XbSettlementFlowStepsExecutionRecord**](XBSettlementFlowStepsExecutionRecord.md) |  | 
**selected_conversion_slippage** | [**models::XbSettlementFlowExecutionModelSelectedConversionSlippage**](XBSettlementFlowExecutionModel_selectedConversionSlippage.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


