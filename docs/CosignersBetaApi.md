# \CosignersBetaApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_cosigner**](CosignersBetaApi.md#add_cosigner) | **POST** /cosigners | Add cosigner
[**get_api_key**](CosignersBetaApi.md#get_api_key) | **GET** /cosigners/{cosignerId}/api_keys/{apiKeyId} | Get API key
[**get_api_keys**](CosignersBetaApi.md#get_api_keys) | **GET** /cosigners/{cosignerId}/api_keys | Get all API keys
[**get_cosigner**](CosignersBetaApi.md#get_cosigner) | **GET** /cosigners/{cosignerId} | Get cosigner
[**get_cosigners**](CosignersBetaApi.md#get_cosigners) | **GET** /cosigners | Get all cosigners
[**get_request_status**](CosignersBetaApi.md#get_request_status) | **GET** /cosigners/{cosignerId}/api_keys/{apiKeyId}/{requestId} | Get request status
[**pair_api_key**](CosignersBetaApi.md#pair_api_key) | **PUT** /cosigners/{cosignerId}/api_keys/{apiKeyId} | Pair API key
[**rename_cosigner**](CosignersBetaApi.md#rename_cosigner) | **PATCH** /cosigners/{cosignerId} | Rename cosigner
[**unpair_api_key**](CosignersBetaApi.md#unpair_api_key) | **DELETE** /cosigners/{cosignerId}/api_keys/{apiKeyId} | Unpair API key
[**update_callback_handler**](CosignersBetaApi.md#update_callback_handler) | **PATCH** /cosigners/{cosignerId}/api_keys/{apiKeyId} | Update API key callback handler



## add_cosigner

> models::AddCosignerResponse add_cosigner(add_cosigner_request, idempotency_key)
Add cosigner

Add a new cosigner. The cosigner will be pending pairing until the API key is manually paired </br>Endpoint Permission: Admin and Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_cosigner_request** | [**AddCosignerRequest**](AddCosignerRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::AddCosignerResponse**](AddCosignerResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_key

> models::ApiKey get_api_key(cosigner_id, api_key_id)
Get API key

Get an API key by ID. **Note:** These endpoints are currently in beta and might be subject to changes. </br>Endpoint Permission: Admin and Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cosigner_id** | **uuid::Uuid** | The unique identifier of the cosigner | [required] |
**api_key_id** | **String** | The unique identifier of the API key | [required] |

### Return type

[**models::ApiKey**](ApiKey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_keys

> models::ApiKeysPaginatedResponse get_api_keys(cosigner_id, order, page_cursor, page_size)
Get all API keys

Get all cosigner paired API keys (paginated). **Note:** These endpoints are currently in beta and might be subject to changes. </br>Endpoint Permission: Admin and Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cosigner_id** | **uuid::Uuid** | The unique identifier of the cosigner | [required] |
**order** | Option<**String**> | ASC / DESC ordering (default DESC) |  |[default to DESC]
**page_cursor** | Option<**String**> | Cursor of the required page |  |
**page_size** | Option<**f64**> | Maximum number of items in the page |  |[default to 10]

### Return type

[**models::ApiKeysPaginatedResponse**](ApiKeysPaginatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cosigner

> models::Cosigner get_cosigner(cosigner_id)
Get cosigner

Get a cosigner by ID. **Note:** These endpoints are currently in beta and might be subject to changes. </br>Endpoint Permission: Admin and Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cosigner_id** | **uuid::Uuid** | The unique identifier of the cosigner | [required] |

### Return type

[**models::Cosigner**](Cosigner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cosigners

> models::CosignersPaginatedResponse get_cosigners(order, page_cursor, page_size)
Get all cosigners

Get all workspace cosigners (paginated). **Note:** These endpoints are currently in beta and might be subject to changes. </br>Endpoint Permission: Admin and Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order** | Option<**String**> | ASC / DESC ordering (default DESC) |  |[default to DESC]
**page_cursor** | Option<**String**> | Cursor of the required page |  |
**page_size** | Option<**f64**> | Maximum number of items in the page |  |[default to 10]

### Return type

[**models::CosignersPaginatedResponse**](CosignersPaginatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_request_status

> models::Status get_request_status(cosigner_id, api_key_id, request_id)
Get request status

Get the status of an asynchronous request </br>Endpoint Permission: Admin and Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cosigner_id** | **uuid::Uuid** | The unique identifier of the cosigner | [required] |
**api_key_id** | **String** | The unique identifier of the API key | [required] |
**request_id** | **String** |  | [required] |

### Return type

[**models::Status**](Status.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pair_api_key

> models::PairApiKeyResponse pair_api_key(cosigner_id, api_key_id, pair_api_key_request, idempotency_key)
Pair API key

Pair an API key to a cosigner </br>Endpoint Permission: Admin and Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cosigner_id** | **uuid::Uuid** | The unique identifier of the cosigner | [required] |
**api_key_id** | **String** | The unique identifier of the API key | [required] |
**pair_api_key_request** | [**PairApiKeyRequest**](PairApiKeyRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::PairApiKeyResponse**](PairApiKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rename_cosigner

> models::Cosigner rename_cosigner(cosigner_id, rename_cosigner)
Rename cosigner

Rename a cosigner by ID. **Note:** These endpoints are currently in beta and might be subject to changes. </br>Endpoint Permission: Admin and Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cosigner_id** | **uuid::Uuid** | The unique identifier of the cosigner | [required] |
**rename_cosigner** | [**RenameCosigner**](RenameCosigner.md) |  | [required] |

### Return type

[**models::Cosigner**](Cosigner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unpair_api_key

> models::ApiKey unpair_api_key(cosigner_id, api_key_id)
Unpair API key

Unpair an API key from a cosigner </br>Endpoint Permission: Admin and Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cosigner_id** | **uuid::Uuid** | The unique identifier of the cosigner | [required] |
**api_key_id** | **String** | The unique identifier of the API key | [required] |

### Return type

[**models::ApiKey**](ApiKey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_callback_handler

> models::UpdateCallbackHandlerResponse update_callback_handler(cosigner_id, api_key_id, update_callback_handler_request)
Update API key callback handler

Update the callback handler of an API key </br>Endpoint Permission: Admin and Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cosigner_id** | **uuid::Uuid** | The unique identifier of the cosigner | [required] |
**api_key_id** | **String** | The unique identifier of the API key | [required] |
**update_callback_handler_request** | [**UpdateCallbackHandlerRequest**](UpdateCallbackHandlerRequest.md) |  | [required] |

### Return type

[**models::UpdateCallbackHandlerResponse**](UpdateCallbackHandlerResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

