# CreateValidationKeyResponseDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**validation_key** | [**models::ValidationKeyDto**](ValidationKeyDto.md) | Created validation key | 
**admins** | **Vec<String>** | Admins who have to approve the validation key addition | 
**approval_threshold** | **f64** | Minimal number of approvers required. 0 for all | 
**request_id** | **f64** | Approval request id. Can be cancelled | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


