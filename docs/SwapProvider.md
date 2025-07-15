# SwapProvider

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the provider | 
**name** | **String** | Name of the provider | 
**protocols** | [**Vec<models::SwapProviderProtocolsEnum>**](SwapProviderProtocolsEnum.md) | List of supported protocols. Protocols are specific per provider | 
**category** | [**models::ProviderCategoryEnum**](ProviderCategoryEnum.md) |  | 
**is_terms_approval_required** | **bool** | Indicates whether the terms of service are required for the provider. if `true`, the user must approve the terms of service before using the provider. otherwise, `termsOfServiceUrl` and `isTermsOfServiceApproved` are not shown under the provider data. | 
**terms_of_service_url** | Option<**String**> | URL to the terms of service | [optional]
**is_terms_of_service_approved** | Option<**bool**> | Indicates whether the terms of service are approved by the user | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


