# \DAppConnectionsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create**](DAppConnectionsApi.md#create) | **POST** /connections/wc | Create a new dApp connection.
[**get**](DAppConnectionsApi.md#get) | **GET** /connections | List all open dApp connections
[**remove**](DAppConnectionsApi.md#remove) | **DELETE** /connections/wc/{id} | Remove an existing dApp connection
[**submit**](DAppConnectionsApi.md#submit) | **PUT** /connections/wc/{id} | Respond to a pending dApp connection request



## create

> models::CreateConnectionResponse create(create_connection_request, idempotency_key)
Create a new dApp connection.

Initiate a new dApp connection.  * Note: After this succeeds, make a request to `PUT /v1/connections/wc/{id}` (below) to approve or reject the new Web3 connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_connection_request** | [**CreateConnectionRequest**](CreateConnectionRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::CreateConnectionResponse**](CreateConnectionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get

> models::GetConnectionsResponse get(order, filter, sort, page_size, next)
List all open dApp connections

List all open dApp connections.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order** | Option<**String**> | List order (ascending or descending) |  |[default to ASC]
**filter** | Option<[**GetFilterParameter**](.md)> | Parsed filter object |  |
**sort** | Option<**String**> | Property to sort Web3 connections by. |  |[default to createdAt]
**page_size** | Option<**f64**> | Amount of results to return in the next page. |  |[default to 10]
**next** | Option<**String**> | Cursor to the next page |  |

### Return type

[**models::GetConnectionsResponse**](GetConnectionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove

> remove(id)
Remove an existing dApp connection

Remove an existing dApp connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the existing Web3 connection to remove. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit

> submit(id, respond_to_connection_request, idempotency_key)
Respond to a pending dApp connection request

Submit a response to *approve* or *reject* an initiated dApp connection.  * Note: This call is used to complete your `POST /v1/connections/wc/` request.  After this succeeds, your new dApp connection is created and functioning. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the initiated Web3 connection to approve. | [required] |
**respond_to_connection_request** | [**RespondToConnectionRequest**](RespondToConnectionRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

