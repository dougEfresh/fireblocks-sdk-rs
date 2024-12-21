# \CosignersBetaApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_api_key**](CosignersBetaApi.md#get_api_key) | **GET** /cosigners/{cosignerId}/api_keys/{apiKeyId} | Get API key
[**get_api_keys**](CosignersBetaApi.md#get_api_keys) | **GET** /cosigners/{cosignerId}/api_keys | Get all API keys
[**get_cosigner**](CosignersBetaApi.md#get_cosigner) | **GET** /cosigners/{cosignerId} | Get cosigner
[**get_cosigners**](CosignersBetaApi.md#get_cosigners) | **GET** /cosigners | Get all cosigners
[**rename_cosigner**](CosignersBetaApi.md#rename_cosigner) | **PATCH** /cosigners/{cosignerId} | Rename cosigner



## get_api_key

> models::ApiKey get_api_key(cosigner_id, api_key_id)
Get API key

Get an API key by ID **Note:** These endpoints are currently in beta and might be subject to changes. 

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

Get all cosigner paired API keys (paginated) **Note:** These endpoints are currently in beta and might be subject to changes. 

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

Get a cosigner by ID **Note:** These endpoints are currently in beta and might be subject to changes. 

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

Get all workspace cosigners (paginated) **Note:** These endpoints are currently in beta and might be subject to changes. 

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


## rename_cosigner

> models::Cosigner rename_cosigner(cosigner_id, rename_cosigner)
Rename cosigner

Rename a cosigner by ID **Note:** These endpoints are currently in beta and might be subject to changes. 

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

