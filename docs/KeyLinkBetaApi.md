# \KeyLinkBetaApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_signing_key**](KeyLinkBetaApi.md#create_signing_key) | **POST** /key_link/signing_keys | Add a new signing key
[**create_validation_key**](KeyLinkBetaApi.md#create_validation_key) | **POST** /key_link/validation_keys | Add a new validation key
[**disable_validation_key**](KeyLinkBetaApi.md#disable_validation_key) | **PATCH** /key_link/validation_keys/{keyId} | Disables a validation key
[**get_signing_key**](KeyLinkBetaApi.md#get_signing_key) | **GET** /key_link/signing_keys/{keyId} | Get a signing key by `keyId`
[**get_signing_keys_list**](KeyLinkBetaApi.md#get_signing_keys_list) | **GET** /key_link/signing_keys | Get list of signing keys
[**get_validation_key**](KeyLinkBetaApi.md#get_validation_key) | **GET** /key_link/validation_keys/{keyId} | Get a validation key by `keyId`
[**get_validation_keys_list**](KeyLinkBetaApi.md#get_validation_keys_list) | **GET** /key_link/validation_keys | Get list of registered validation keys
[**set_agent_id**](KeyLinkBetaApi.md#set_agent_id) | **PATCH** /key_link/signing_keys/{keyId}/agent_user_id | Set agent user id
[**update_signing_key**](KeyLinkBetaApi.md#update_signing_key) | **PATCH** /key_link/signing_keys/{keyId} | Modify the signing keyId



## create_signing_key

> models::SigningKeyDto create_signing_key(create_signing_key_dto, idempotency_key)
Add a new signing key

Adds a new signing key to the workspace. The added key will be linked to the specific Fireblocks agent user ID. The same user will receive the proof of ownership message to be signed, and upon successful proof, the key will become enabled. Please note that this endpoint is available only for Key Link enabled workspaces. **Note:**  This endpoint is currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks Key Link, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. </br>Endpoint Permission: Admin and Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_signing_key_dto** | [**CreateSigningKeyDto**](CreateSigningKeyDto.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::SigningKeyDto**](SigningKeyDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_validation_key

> models::CreateValidationKeyResponseDto create_validation_key(create_validation_key_dto, idempotency_key)
Add a new validation key

Adds a new validation key used to validate signing keys. The new validation key will undergo an approval process by the workspace quorum. Please note that this endpoint is available only for Key Link enabled workspaces. **Note:**  This endpoint is currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks Key Link, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. </br>Endpoint Permission: Admin and Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_validation_key_dto** | [**CreateValidationKeyDto**](CreateValidationKeyDto.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::CreateValidationKeyResponseDto**](CreateValidationKeyResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_validation_key

> models::ValidationKeyDto disable_validation_key(key_id, modify_validation_key_dto)
Disables a validation key

Allows disabling validation key even if it has not expired yet. It is not allowed to enable the validation key back. Another key has to be used for future validations. Please note that this endpoint is available only for Key Link enabled workspaces. **Note:**  This endpoint is currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks Key Link, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. </br>Endpoint Permission: Admin and Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The unique identifier for the validation key provided by Fireblocks | [required] |
**modify_validation_key_dto** | [**ModifyValidationKeyDto**](ModifyValidationKeyDto.md) |  | [required] |

### Return type

[**models::ValidationKeyDto**](ValidationKeyDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_signing_key

> models::SigningKeyDto get_signing_key(key_id)
Get a signing key by `keyId`

Returns a signing key if it exists, identified by the specified Fireblocks provided `keyId`. Please note that this endpoint is available only for Key Link enabled workspaces. **Note:**  This endpoint is currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks Key Link, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. </br>Endpoint Permission: Admin and Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The unique identifier for the signing key provided by Fireblocks | [required] |

### Return type

[**models::SigningKeyDto**](SigningKeyDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_signing_keys_list

> models::GetSigningKeyResponseDto get_signing_keys_list(page_cursor, page_size, sort_by, order, vault_account_id, agent_user_id, algorithm, enabled, available)
Get list of signing keys

Returns the list of signing keys in the workspace Please note that this endpoint is available only for Key Link enabled workspaces. **Note:**  This endpoint is currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks Key Link, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. </br>Endpoint Permission: Admin and Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_cursor** | Option<**String**> | Cursor to the next page |  |
**page_size** | Option<**f64**> | Amount of results to return in the next page |  |[default to 10]
**sort_by** | Option<**String**> | Field(s) to use for sorting |  |[default to createdAt]
**order** | Option<**String**> | Is the order ascending or descending |  |[default to ASC]
**vault_account_id** | Option<**f64**> | Return keys assigned to a specific vault |  |
**agent_user_id** | Option<**String**> | Return keys associated with a specific agent user |  |
**algorithm** | Option<**String**> | Return only keys with a specific algorithm |  |
**enabled** | Option<**bool**> | Return keys that have been proof of ownership |  |
**available** | Option<**bool**> | Return keys that are proof of ownership but not assigned. Available filter can be used only when vaultAccountId and enabled filters are not set |  |

### Return type

[**models::GetSigningKeyResponseDto**](GetSigningKeyResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_validation_key

> models::ValidationKeyDto get_validation_key(key_id)
Get a validation key by `keyId`

Returns a validation key if it exists, identified by the specified `keyId`. Please note that this endpoint is available only for Key Link enabled workspaces. **Note:**  This endpoint is currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks Key Link, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. </br>Endpoint Permission: Admin and Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** |  | [required] |

### Return type

[**models::ValidationKeyDto**](ValidationKeyDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_validation_keys_list

> models::GetValidationKeyResponseDto get_validation_keys_list(page_cursor, page_size, sort_by, order)
Get list of registered validation keys

Returns the list of validation keys in the workspace Please note that this endpoint is available only for Key Link enabled workspaces. **Note:**  This endpoint is currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks Key Link, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. </br>Endpoint Permission: Admin and Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_cursor** | Option<**String**> | Cursor to the next page |  |
**page_size** | Option<**f64**> | Amount of results to return in the next page |  |[default to 10]
**sort_by** | Option<**String**> | Field(s) to use for sorting |  |[default to createdAt]
**order** | Option<**String**> | Is the order ascending or descending |  |[default to ASC]

### Return type

[**models::GetValidationKeyResponseDto**](GetValidationKeyResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_agent_id

> set_agent_id(key_id, modify_signing_key_agent_id_dto)
Set agent user id

Can modify existing signing key id if the key is not enabled. The change done in background and will be visible once applied. If key is already enabled (after proof of ownership) the user cannot be changed. Please note that this endpoint is available only for Key Link enabled workspaces. **Note:**  This endpoint is currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks Key Link, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. </br>Endpoint Permission: Admin and Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The unique identifier for the signing key provided by Fireblocks | [required] |
**modify_signing_key_agent_id_dto** | [**ModifySigningKeyAgentIdDto**](ModifySigningKeyAgentIdDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_signing_key

> models::SigningKeyDto update_signing_key(key_id, modify_signing_key_dto)
Modify the signing keyId

Allows assigning the signing key to a vault account, if it hasn't been assigned to any other vault accounts yet. Please note that this endpoint is available only for Key Link enabled workspaces. **Note:**  This endpoint is currently in beta and might be subject to changes. If you want to participate and learn more about the Fireblocks Key Link, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com. </br>Endpoint Permission: Admin and Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **String** | The unique identifier for the signing key provided by Fireblocks | [required] |
**modify_signing_key_dto** | [**ModifySigningKeyDto**](ModifySigningKeyDto.md) |  | [required] |

### Return type

[**models::SigningKeyDto**](SigningKeyDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

