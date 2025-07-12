# \TokenizationApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**burn_collection_token**](TokenizationApi.md#burn_collection_token) | **POST** /tokenization/collections/{id}/tokens/burn | Burn tokens
[**create_new_collection**](TokenizationApi.md#create_new_collection) | **POST** /tokenization/collections | Create a new collection
[**fetch_collection_token_details**](TokenizationApi.md#fetch_collection_token_details) | **GET** /tokenization/collections/{id}/tokens/{tokenId} | Get collection token details
[**get_collection_by_id**](TokenizationApi.md#get_collection_by_id) | **GET** /tokenization/collections/{id} | Get a collection by id
[**get_linked_collections**](TokenizationApi.md#get_linked_collections) | **GET** /tokenization/collections | Get collections
[**get_linked_token**](TokenizationApi.md#get_linked_token) | **GET** /tokenization/tokens/{id} | Return a linked token
[**get_linked_tokens**](TokenizationApi.md#get_linked_tokens) | **GET** /tokenization/tokens | List all linked tokens
[**issue_new_token**](TokenizationApi.md#issue_new_token) | **POST** /tokenization/tokens | Issue a new token
[**link**](TokenizationApi.md#link) | **POST** /tokenization/tokens/link | Link a contract
[**mint_collection_token**](TokenizationApi.md#mint_collection_token) | **POST** /tokenization/collections/{id}/tokens/mint | Mint tokens
[**unlink**](TokenizationApi.md#unlink) | **DELETE** /tokenization/tokens/{id} | Unlink a token
[**unlink_collection**](TokenizationApi.md#unlink_collection) | **DELETE** /tokenization/collections/{id} | Delete a collection link



## burn_collection_token

> models::CollectionBurnResponseDto burn_collection_token(id, collection_burn_request_dto, idempotency_key)
Burn tokens

Burn tokens in a collection. </br>Endpoint Permission: Owner, Admin, Non-Signing Admin, Signer, and Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The collection link id | [required] |
**collection_burn_request_dto** | [**CollectionBurnRequestDto**](CollectionBurnRequestDto.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::CollectionBurnResponseDto**](CollectionBurnResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_new_collection

> models::CollectionLinkDto create_new_collection(collection_deploy_request_dto, idempotency_key)
Create a new collection

Create a new collection and link it as a token. </br>Endpoint Permission: Owner, Admin, Non-Signing Admin, Signer, and Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_deploy_request_dto** | [**CollectionDeployRequestDto**](CollectionDeployRequestDto.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::CollectionLinkDto**](CollectionLinkDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_collection_token_details

> models::CollectionLinkDto fetch_collection_token_details(id, token_id)
Get collection token details

Get collection token details by id. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The collection link id | [required] |
**token_id** | **String** | The tokenId as it appears on the blockchain | [required] |

### Return type

[**models::CollectionLinkDto**](CollectionLinkDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection_by_id

> models::CollectionLinkDto get_collection_by_id(id)
Get a collection by id

Get a collection by id. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The token link id | [required] |

### Return type

[**models::CollectionLinkDto**](CollectionLinkDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linked_collections

> models::GetLinkedCollectionsPaginatedResponse get_linked_collections(page_cursor, page_size, status)
Get collections

Get collections (paginated). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor, Viewer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_cursor** | Option<**String**> | Page cursor to get the next page, for example - \"MjAyMy0xMi0xMyAyMDozNjowOC4zMDI=:MTEwMA==\" |  |
**page_size** | Option<**f64**> | Number of items per page (max 100), requesting more then 100 will return 100 items |  |[default to 100]
**status** | Option<[**serde_json::Value**](.md)> | A comma separated list of statuses to filter. Default is \"COMPLETED\" |  |

### Return type

[**models::GetLinkedCollectionsPaginatedResponse**](GetLinkedCollectionsPaginatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linked_token

> models::TokenLinkDto get_linked_token(id)
Return a linked token

Return a linked token, with its status and metadata.  </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The token link id | [required] |

### Return type

[**models::TokenLinkDto**](TokenLinkDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_linked_tokens

> models::TokensPaginatedResponse get_linked_tokens(page_cursor, page_size, status)
List all linked tokens

Return all linked tokens (paginated).  </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_cursor** | Option<**String**> | Page cursor to get the next page |  |
**page_size** | Option<**f64**> | Number of items per page, requesting more then max will return max items |  |
**status** | Option<[**serde_json::Value**](.md)> | A comma separated list of statuses to filter. Default is \"COMPLETED\" |  |

### Return type

[**models::TokensPaginatedResponse**](TokensPaginatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_new_token

> models::TokenLinkDto issue_new_token(create_token_request_dto, idempotency_key)
Issue a new token

Facilitates the creation of a new token, supporting both EVM-based and Stellar/Ripple platforms. For EVM, it deploys the corresponding contract template to the blockchain and links the token to the workspace. For Stellar/Ripple, it links a newly created token directly to the workspace without deploying a contract. Returns the token link with status \"PENDING\" until the token is deployed or \"SUCCESS\" if no deployment is needed. </br>Endpoint Permission: Owner, Admin, Non-Signing Admin, Signer, and Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_token_request_dto** | [**CreateTokenRequestDto**](CreateTokenRequestDto.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::TokenLinkDto**](TokenLinkDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link

> models::TokenLinkDto link(token_link_request_dto, idempotency_key)
Link a contract

Link a contract. </br>Endpoint Permission: Owner, Admin, Non-Signing Admin, and Signer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_link_request_dto** | [**TokenLinkRequestDto**](TokenLinkRequestDto.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::TokenLinkDto**](TokenLinkDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mint_collection_token

> models::CollectionMintResponseDto mint_collection_token(id, collection_mint_request_dto, idempotency_key)
Mint tokens

Mint tokens and upload metadata. </br>Endpoint Permission: Owner, Admin, Non-Signing Admin, Signer, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The collection link id | [required] |
**collection_mint_request_dto** | [**CollectionMintRequestDto**](CollectionMintRequestDto.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::CollectionMintResponseDto**](CollectionMintResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink

> unlink(id)
Unlink a token

Unlink a token. The token will be unlinked from the workspace. The token will not be deleted on chain nor the refId, only the link to the workspace will be removed. </br>Endpoint Permission: Owner, Admin, Non-Signing Admin, Signer, and Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The token link id | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_collection

> unlink_collection(id)
Delete a collection link

Delete a collection link. </br>Endpoint Permission: Owner, Admin, Non-Signing Admin, Signer, and Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The token link id | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

