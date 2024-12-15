# \BlockchainsAssetsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**estimate_network_fee_get**](BlockchainsAssetsApi.md#estimate_network_fee_get) | **GET** /estimate_network_fee | Estimate the required fee for an asset
[**register_new_asset**](BlockchainsAssetsApi.md#register_new_asset) | **POST** /assets | Register an asset
[**supported_assets_get**](BlockchainsAssetsApi.md#supported_assets_get) | **GET** /supported_assets | List all assets and assets types supported by Fireblocks
[**transactions_validate_address_asset_id_address_get**](BlockchainsAssetsApi.md#transactions_validate_address_asset_id_address_get) | **GET** /transactions/validate_address/{assetId}/{address} | Validate destination address



## estimate_network_fee_get

> models::EstimatedNetworkFeeResponse estimate_network_fee_get(asset_id)
Estimate the required fee for an asset

Gets the estimated required fee for an asset. For UTXO based assets, the response will contain the suggested fee per byte, for ETH/ETC based assets, the suggested gas price, and for XRP/XLM, the transaction fee.

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
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_new_asset

> models::AssetResponse register_new_asset(register_new_asset_request)
Register an asset

Register a new asset to a workspace and return the newly created asset's details. Currently supported chains are: - EVM based chains - Stellar - Algorand - TRON - NEAR - Solana 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**register_new_asset_request** | Option<[**RegisterNewAssetRequest**](RegisterNewAssetRequest.md)> |  |  |

### Return type

[**models::AssetResponse**](AssetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## supported_assets_get

> Vec<models::AssetTypeResponse> supported_assets_get()
List all assets and assets types supported by Fireblocks

Returns all assets and assets types supported by Fireblocks.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::AssetTypeResponse>**](AssetTypeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_validate_address_asset_id_address_get

> models::ValidateAddressResponse transactions_validate_address_asset_id_address_get(asset_id, address)
Validate destination address

Checks if an address is valid (for XRP, DOT, XLM, and EOS).

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
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

