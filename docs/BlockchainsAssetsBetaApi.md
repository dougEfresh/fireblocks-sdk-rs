# \BlockchainsAssetsBetaApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_asset_by_id**](BlockchainsAssetsBetaApi.md#get_asset_by_id) | **GET** /assets/{id} | Get an asset by ID
[**get_blockchain_by_id**](BlockchainsAssetsBetaApi.md#get_blockchain_by_id) | **GET** /blockchains/{id} | Get a Blockchain by ID
[**list_assets**](BlockchainsAssetsBetaApi.md#list_assets) | **GET** /assets | List assets
[**list_blockchains**](BlockchainsAssetsBetaApi.md#list_blockchains) | **GET** /blockchains | List blockchains



## get_asset_by_id

> models::AssetResponseBeta get_asset_by_id(id, idempotency_key)
Get an asset by ID

Returns an asset by ID or legacyID.</br>  **Note**: - This endpoint is now in Beta, disabled for general availability at this time. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID or legacyId of the asset | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::AssetResponseBeta**](AssetResponseBeta.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockchain_by_id

> models::BlockchainResponse get_blockchain_by_id(id)
Get a Blockchain by ID

Returns a blockchain by ID or legacyID.</br>  **Note**: - This endpoint is now in Beta, disabled for general availability at this time. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID or legacyId of the blockchain | [required] |

### Return type

[**models::BlockchainResponse**](BlockchainResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_assets

> models::ListAssetsResponse list_assets(blockchain_id, asset_class, symbol, scope, deprecated, page_cursor, page_size, idempotency_key)
List assets

Returns all asset type supported by Fireblocks.</br>  **Note**: - This endpoint is now in Beta, disabled for general availability at this time. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain_id** | Option<**String**> | Blockchain id of the assets |  |
**asset_class** | Option<[**AssetClassBeta**](.md)> | Assets class |  |
**symbol** | Option<**String**> | Assets onchain symbol |  |
**scope** | Option<**String**> | Scope of the assets |  |
**deprecated** | Option<**bool**> | Are assets deprecated |  |
**page_cursor** | Option<**String**> | Next page cursor to fetch |  |
**page_size** | Option<**f64**> | Items per page |  |[default to 500]
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::ListAssetsResponse**](ListAssetsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_blockchains

> models::ListBlockchainsResponse list_blockchains(protocol, deprecated, test, page_cursor, page_size)
List blockchains

Returns all blockchains supported by Fireblocks.</br>  **Note**: - This endpoint is now in Beta, disabled for general availability at this time. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | Option<**String**> | Blockchain protocol |  |
**deprecated** | Option<**bool**> | Is blockchain deprecated |  |
**test** | Option<**bool**> | Is test blockchain |  |
**page_cursor** | Option<**String**> | Page cursor to fetch |  |
**page_size** | Option<**f64**> | Items per page (max 500) |  |[default to 500]

### Return type

[**models::ListBlockchainsResponse**](ListBlockchainsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

