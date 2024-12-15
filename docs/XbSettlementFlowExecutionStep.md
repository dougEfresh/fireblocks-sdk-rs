# XbSettlementFlowExecutionStep

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | A unique id for the step execution | 
**account_id** | **String** |  | 
**status** | [**models::XbSettlementFlowExecutionStepStatus**](XBSettlementFlowExecutionStepStatus.md) |  | 
**input_amount** | [**models::XbSettlementAsset**](XBSettlementAsset.md) |  | 
**output_amount** | Option<[**models::XbSettlementAsset**](XBSettlementAsset.md)> |  | [optional]
**fee** | Option<[**models::XbSettlementAsset**](XBSettlementAsset.md)> |  | [optional]
**started_at** | Option<**f64**> | The step execution start time in epoch format. | [optional]
**completed_at** | Option<**f64**> | The step execution end time in epoch format. | [optional]
**is_sign_required** | **bool** | Whether or not signing is required for executing the step. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


