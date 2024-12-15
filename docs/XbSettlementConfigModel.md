# XbSettlementConfigModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config_id** | [**uuid::Uuid**](uuid::Uuid.md) | Cross Bodrder configuraion unique id | 
**corridor_id** | [**models::XbSettlementCorridorId**](XBSettlementCorridorId.md) |  | 
**name** | **String** | The name for the cross-border ettlement configuration | 
**steps** | [**models::XbSettlementConfigStepsRecord**](XBSettlementConfigStepsRecord.md) |  | 
**conversion_slippage_basis_points** | **i32** | Slippage configuarion in basis points, the default value is 10%  | [default to 10000]
**created_at** | **f64** | The creation time in epoch format. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


