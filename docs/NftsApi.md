# \NftsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_nft**](NftsApi.md#get_nft) | **GET** /nfts/tokens/{id} | List token data by ID
[**get_nfts**](NftsApi.md#get_nfts) | **GET** /nfts/tokens | List tokens by IDs
[**get_ownership_tokens**](NftsApi.md#get_ownership_tokens) | **GET** /nfts/ownership/tokens | List all owned tokens (paginated)
[**list_owned_collections**](NftsApi.md#list_owned_collections) | **GET** /nfts/ownership/collections | List owned collections (paginated)
[**list_owned_tokens**](NftsApi.md#list_owned_tokens) | **GET** /nfts/ownership/assets | List all distinct owned tokens (paginated)
[**refresh_nft_metadata**](NftsApi.md#refresh_nft_metadata) | **PUT** /nfts/tokens/{id} | Refresh token metadata
[**update_ownership_tokens**](NftsApi.md#update_ownership_tokens) | **PUT** /nfts/ownership/tokens | Refresh vault account tokens
[**update_token_ownership_status**](NftsApi.md#update_token_ownership_status) | **PUT** /nfts/ownership/tokens/{id}/status | Update token ownership status
[**update_tokens_ownership_spam**](NftsApi.md#update_tokens_ownership_spam) | **PUT** /nfts/ownership/tokens/spam | Update tokens ownership spam property
[**update_tokens_ownership_status**](NftsApi.md#update_tokens_ownership_status) | **PUT** /nfts/ownership/tokens/status | Update tokens ownership status



## get_nft

> models::TokenResponse get_nft(id)
List token data by ID

Returns the requested token data. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | NFT ID | [required] |

### Return type

[**models::TokenResponse**](TokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_nfts

> models::GetNftsResponse get_nfts(ids, page_cursor, page_size, sort, order)
List tokens by IDs

Returns the requested tokens data. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | A comma separated list of NFT IDs. Up to 100 are allowed in a single request. | [required] |
**page_cursor** | Option<**String**> | Page cursor to fetch |  |
**page_size** | Option<**f64**> | Items per page (max 100) |  |
**sort** | Option<[**Vec<String>**](String.md)> | Sort by param, it can be one param or a list of params separated by comma |  |
**order** | Option<**String**> | Order direction, it can be `ASC` for ascending or `DESC` for descending |  |[default to ASC]

### Return type

[**models::GetNftsResponse**](GetNFTsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ownership_tokens

> models::GetOwnershipTokensResponse get_ownership_tokens(blockchain_descriptor, vault_account_ids, ncw_id, ncw_account_ids, wallet_type, ids, collection_ids, page_cursor, page_size, sort, order, status, search, spam)
List all owned tokens (paginated)

Returns all tokens and their data in your workspace.  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain_descriptor** | Option<**String**> | Blockchain descriptor filter |  |
**vault_account_ids** | Option<**String**> | A comma separated list of Vault Account IDs. Up to 100 are allowed in a single request.  This field will be ignored when walletType=END_USER_WALLET or ncwId is provided. |  |
**ncw_id** | Option<**String**> | Non-Custodial Wallet ID |  |
**ncw_account_ids** | Option<**String**> | A comma separated list of Non-Custodial account IDs. Up to 100 are allowed in a single request. This field will be ignored when walletType=VAULT_ACCOUNT or ncwId is not provided. |  |
**wallet_type** | Option<**String**> | Wallet type, it can be `VAULT_ACCOUNT` or `END_USER_WALLET` |  |[default to VAULT_ACCOUNT]
**ids** | Option<**String**> | A comma separated list of NFT IDs. Up to 100 are allowed in a single request. |  |
**collection_ids** | Option<**String**> | A comma separated list of collection IDs. Up to 100 are allowed in a single request. |  |
**page_cursor** | Option<**String**> | Page cursor to fetch |  |
**page_size** | Option<**f64**> | Items per page (max 100) |  |
**sort** | Option<[**Vec<String>**](String.md)> | Sort by param, it can be one param or a list of params separated by comma |  |
**order** | Option<**String**> | Order direction, it can be `ASC` for ascending or `DESC` for descending |  |[default to ASC]
**status** | Option<**String**> | Token ownership status |  |[default to LISTED]
**search** | Option<**String**> | Search owned tokens and their collections. Possible criteria for search:  token name and id within the contract/collection, collection name, blockchain descriptor and name. |  |
**spam** | Option<**String**> | Token ownership spam status. |  |

### Return type

[**models::GetOwnershipTokensResponse**](GetOwnershipTokensResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_owned_collections

> models::ListOwnedCollectionsResponse list_owned_collections(ncw_id, wallet_type, search, page_cursor, page_size, sort, order, status)
List owned collections (paginated)

Returns all collections in your workspace. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ncw_id** | Option<**String**> | Tenant's Non-Custodial Wallet ID |  |
**wallet_type** | Option<**String**> | Wallet type, it can be `VAULT_ACCOUNT` or `END_USER_WALLET` |  |[default to VAULT_ACCOUNT]
**search** | Option<**String**> | Search owned collections. Possible criteria for search: collection name, collection contract address. |  |
**page_cursor** | Option<**String**> | Page cursor to fetch |  |
**page_size** | Option<**f64**> | Items per page (max 100) |  |
**sort** | Option<[**Vec<String>**](String.md)> | Sort by param, it can be one param or a list of params separated by comma |  |
**order** | Option<**String**> | Order direction, it can be `ASC` for ascending or `DESC` for descending |  |[default to ASC]
**status** | Option<**String**> | Token ownership status |  |[default to LISTED]

### Return type

[**models::ListOwnedCollectionsResponse**](ListOwnedCollectionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_owned_tokens

> models::ListOwnedTokensResponse list_owned_tokens(ncw_id, wallet_type, page_cursor, page_size, sort, order, status, search, spam)
List all distinct owned tokens (paginated)

Returns all owned distinct tokens (for your workspace) and their data in your workspace. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ncw_id** | Option<**String**> | Non-Custodial Wallet ID |  |
**wallet_type** | Option<**String**> | Wallet type, it can be `VAULT_ACCOUNT` or `END_USER_WALLET` |  |[default to VAULT_ACCOUNT]
**page_cursor** | Option<**String**> | Page cursor to fetch |  |
**page_size** | Option<**f64**> | Items per page (max 100) |  |
**sort** | Option<[**Vec<String>**](String.md)> | Sort by param, it can be one param or a list of params separated by comma |  |
**order** | Option<**String**> | Order direction, it can be `ASC` for ascending or `DESC` for descending |  |[default to ASC]
**status** | Option<**String**> | Token ownership status |  |[default to LISTED]
**search** | Option<**String**> | Search owned tokens by token name |  |
**spam** | Option<**String**> | Token ownership spam status. |  |

### Return type

[**models::ListOwnedTokensResponse**](ListOwnedTokensResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_nft_metadata

> refresh_nft_metadata(id, idempotency_key)
Refresh token metadata

Updates the latest token metadata. Learn more about Fireblocks NFT management in the following [guide](https://developers.fireblocks.com/reference/deploy-an-nft-collection).  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | NFT ID | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ownership_tokens

> update_ownership_tokens(blockchain_descriptor, vault_account_id, idempotency_key)
Refresh vault account tokens

Updates all tokens and balances per blockchain and vault account. Learn more about Fireblocks NFT management in the following [guide](https://developers.fireblocks.com/reference/deploy-an-nft-collection).  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain_descriptor** | **String** | Blockchain descriptor filter | [required] |
**vault_account_id** | **String** | Vault account filter | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_token_ownership_status

> update_token_ownership_status(id, update_token_ownership_status_dto, idempotency_key)
Update token ownership status

Updates token status for a workspace, in all workspace vaults.  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | NFT ID | [required] |
**update_token_ownership_status_dto** | [**UpdateTokenOwnershipStatusDto**](UpdateTokenOwnershipStatusDto.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tokens_ownership_spam

> update_tokens_ownership_spam(token_ownership_spam_update_payload, idempotency_key)
Update tokens ownership spam property

Updates tokens spam property for a workspace's token ownerships, in all vault accounts.  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_ownership_spam_update_payload** | [**Vec<models::TokenOwnershipSpamUpdatePayload>**](TokenOwnershipSpamUpdatePayload.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tokens_ownership_status

> update_tokens_ownership_status(token_ownership_status_update_payload, idempotency_key)
Update tokens ownership status

Updates tokens status for a workspace, in all vault accounts.  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_ownership_status_update_payload** | [**Vec<models::TokenOwnershipStatusUpdatePayload>**](TokenOwnershipStatusUpdatePayload.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

