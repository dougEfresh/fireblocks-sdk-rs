# ParameterWithValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the parameter as it appears in the ABI | 
**description** | Option<**String**> | A description of the parameter, fetched from the devdoc of this contract | [optional]
**internal_type** | Option<**String**> | The  internal type of the parameter as it appears in the ABI | [optional]
**r#type** | **String** | The type of the parameter as it appears in the ABI | 
**components** | Option<[**Vec<models::Parameter>**](Parameter.md)> |  | [optional]
**value** | Option<**String**> | The value of the parameter. can also be ParameterWithValue | [optional]
**function_value** | Option<[**models::LeanAbiFunction**](LeanAbiFunction.md)> | The function value of this param (if has one). If this is set, the `value` shouldn`t be. Used for proxies | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


