# \BlockchainsAssetsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**estimate_network_fee**](BlockchainsAssetsApi.md#estimate_network_fee) | **GET** /estimate_network_fee | Estimate the required fee for an asset
[**get_supported_assets**](BlockchainsAssetsApi.md#get_supported_assets) | **GET** /supported_assets | List all asset types supported by Fireblocks
[**register_new_asset**](BlockchainsAssetsApi.md#register_new_asset) | **POST** /assets | Register an asset
[**validate_address**](BlockchainsAssetsApi.md#validate_address) | **GET** /transactions/validate_address/{assetId}/{address} | Validate destination address



## estimate_network_fee

> models::EstimatedNetworkFeeResponse estimate_network_fee(asset_id)
Estimate the required fee for an asset

Gets the estimated required fee for an asset. Fireblocks fetches, calculates and caches the result every 30 seconds. Customers should query this API while taking the caching interval into consideration.  - For UTXO based assets, the response will contain the suggested fee per byte - For ETH (and all EVM) based assets, the suggested gas price - For XRP/XLM, the transaction fee 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** | The asset for which to estimate the fee | [required] |

### Return type

[**models::EstimatedNetworkFeeResponse**](EstimatedNetworkFeeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_supported_assets

> Vec<models::AssetTypeResponse> get_supported_assets()
List all asset types supported by Fireblocks

Returns all asset types supported by Fireblocks.   The response includes all assets supported by Fireblocks globally in addition to assets added to the specific workspace. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::AssetTypeResponse>**](AssetTypeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_new_asset

> models::AssetResponse register_new_asset(idempotency_key, register_new_asset_request)
Register an asset

Register a new asset to a workspace and return the newly created asset's details. Currently supported chains are: - EVM based chains - Stellar - Algorand - TRON - NEAR - Solana 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**register_new_asset_request** | Option<[**RegisterNewAssetRequest**](RegisterNewAssetRequest.md)> |  |  |

### Return type

[**models::AssetResponse**](AssetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_address

> models::ValidateAddressResponse validate_address(asset_id, address)
Validate destination address

Checks if an address is valid and active (for XRP, DOT, XLM, and EOS).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** | The asset of the address | [required] |
**address** | **String** | The address to validate | [required] |

### Return type

[**models::ValidateAddressResponse**](ValidateAddressResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

