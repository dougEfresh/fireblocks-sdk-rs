# Provider

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the provider | 
**provider_name** | **String** | Name of the provider | 
**validators** | [**Vec<models::Validator>**](Validator.md) | An array of objects that includes chain descriptors and the corresponding fee percentages for validators supported by the provider | 
**icon_url** | Option<**String**> | URL to the validator's icon | [optional]
**terms_of_service_url** | Option<**String**> | URL to the terms of service | [optional]
**is_terms_of_service_approved** | **bool** | Indicates whether the terms of service are approved | 
**is_private** | Option<**bool**> | Is the provider private, i.e created by the user | [optional]
**is_liquid_staking** | **bool** | Is the provider a liquid staking provider | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


