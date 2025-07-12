# \SwapBetaApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**approve_terms_of_service**](SwapBetaApi.md#approve_terms_of_service) | **POST** /swap/providers/{providerId}/approve_terms | Approve terms of service
[**create_quote**](SwapBetaApi.md#create_quote) | **POST** /swap/providers/{providerId}/quote | Create a quote
[**create_swap_operation**](SwapBetaApi.md#create_swap_operation) | **POST** /swap/operations | Create swap operation
[**get_swap_operation_by_id**](SwapBetaApi.md#get_swap_operation_by_id) | **GET** /swap/operations/{operationId} | Get operation details
[**get_swap_operations**](SwapBetaApi.md#get_swap_operations) | **GET** /swap/operations | Get Operations list
[**get_swap_providers**](SwapBetaApi.md#get_swap_providers) | **GET** /swap/providers | Get Providers List



## approve_terms_of_service

> models::SwapProvider approve_terms_of_service(provider_id, idempotency_key)
Approve terms of service

Approve the terms of service for a swap provider. Some providers require this approval before performing a swap action for the first time.  Note: These endpoints are currently in beta and might be subject to changes.  If you want to participate and learn more about the Fireblocks TAP, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com.  Endpoint Permission: Owner, Admin, Non-Signing Admin, Signer, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_id** | **String** |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::SwapProvider**](SwapProvider.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_quote

> models::QuoteResponse create_quote(provider_id, quote_request, idempotency_key)
Create a quote

Create a quote from specific swap provider.  Note: These endpoints are currently in beta and might be subject to changes.  If you want to participate and learn more about the Fireblocks TAP, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com.  Endpoint Permission: Owner, Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provider_id** | **String** |  | [required] |
**quote_request** | [**QuoteRequest**](QuoteRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::QuoteResponse**](QuoteResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_swap_operation

> models::SwapOperation create_swap_operation(swap_operation_request, idempotency_key)
Create swap operation

Create swap operation based on a provided quote Id  Note: These endpoints are currently in beta and might be subject to changes.  If you want to participate and learn more about the Fireblocks TAP, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com.  Endpoint Permission: Owner, Admin, Non-Signing Admin, Signer, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**swap_operation_request** | [**SwapOperationRequest**](SwapOperationRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::SwapOperation**](SwapOperation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_swap_operation_by_id

> models::SwapOperation get_swap_operation_by_id(operation_id)
Get operation details

Get swap operation Details by ID.  Note:These endpoints are currently in beta and might be subject to changes.  If you want to participate and learn more about the Fireblocks Swap, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com.  Endpoint Permission: Owner, Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**operation_id** | **String** |  | [required] |

### Return type

[**models::SwapOperation**](SwapOperation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_swap_operations

> models::SwapOperationsPaginatedResponse get_swap_operations(page_cursor, page_size)
Get Operations list

Return a list of swap operations for the specific workspace The operations are sorted by creation date in descending order, meaning the most recent operation appears first.  Note:These endpoints are currently in beta and might be subject to changes.  If you want to participate and learn more about the Fireblocks TAP, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com.  Endpoint Permission: Owner, Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_cursor** | Option<**String**> | Cursor of the required page |  |
**page_size** | Option<**f64**> | Number of items in a page, Acceptable values are 1-100. Maximum value is 100 |  |[default to 10]

### Return type

[**models::SwapOperationsPaginatedResponse**](SwapOperationsPaginatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_swap_providers

> models::SwapProvidersPaginatedResponse get_swap_providers(page_cursor, page_size)
Get Providers List

Return a list of all supported swap providers.  Note: These endpoints are currently in beta and might be subject to changes.  If you want to participate and learn more about the Fireblocks TAP, please contact your Fireblocks Customer Success Manager or send an email to CSM@fireblocks.com.  Endpoint Permission: Owner, Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_cursor** | Option<**String**> | Cursor of the required page |  |
**page_size** | Option<**f64**> | Number of items in a page, Acceptable values are 1-100. Maximum value is 100 |  |[default to 10]

### Return type

[**models::SwapProvidersPaginatedResponse**](SwapProvidersPaginatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

