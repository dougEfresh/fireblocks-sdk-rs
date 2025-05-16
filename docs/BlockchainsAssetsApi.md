# \BlockchainsAssetsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**estimate_network_fee**](BlockchainsAssetsApi.md#estimate_network_fee) | **GET** /estimate_network_fee | Estimate the required fee for an asset
[**get_asset**](BlockchainsAssetsApi.md#get_asset) | **GET** /assets/{id} | Get an asset by ID
[**get_blockchain**](BlockchainsAssetsApi.md#get_blockchain) | **GET** /blockchains/{id} | Get a Blockchain by ID
[**get_supported_assets**](BlockchainsAssetsApi.md#get_supported_assets) | **GET** /supported_assets | Legacy - List Assets
[**list_assets**](BlockchainsAssetsApi.md#list_assets) | **GET** /assets | List assets
[**list_blockchains**](BlockchainsAssetsApi.md#list_blockchains) | **GET** /blockchains | List blockchains
[**register_new_asset**](BlockchainsAssetsApi.md#register_new_asset) | **POST** /assets | Register an asset
[**set_asset_price**](BlockchainsAssetsApi.md#set_asset_price) | **POST** /assets/prices/{id} | Set asset price
[**validate_address**](BlockchainsAssetsApi.md#validate_address) | **GET** /transactions/validate_address/{assetId}/{address} | Validate destination address



## estimate_network_fee

> models::EstimatedNetworkFeeResponse estimate_network_fee(asset_id)
Estimate the required fee for an asset

Gets the estimated required fee for an asset. Fireblocks fetches, calculates and caches the result every 30 seconds. Customers should query this API while taking the caching interval into consideration. Notes: - The `networkFee` parameter is the `gasPrice` with a given delta added, multiplied by the gasLimit plus the delta. - The estimation provided depends on the asset type.     - For UTXO-based assets, the response contains the `feePerByte` parameter     - For ETH-based and all EVM based assets, the response will contain `gasPrice` parameter. This is calculated by adding the `baseFee` to the `actualPriority` based on the latest 12 blocks. The response for ETH-based  contains the `baseFee`, `gasPrice`, and `priorityFee` parameters.     - For ADA-based assets, the response will contain the parameter `networkFee` and `feePerByte` parameters.     - For XRP and XLM, the response will contain the transaction fee.     - For other assets, the response will contain the `networkFee` parameter.  Learn more about Fireblocks Fee Management in the following [guide](https://developers.fireblocks.com/reference/estimate-transaction-fee). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

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


## get_asset

> models::Asset get_asset(id, idempotency_key)
Get an asset by ID

Returns an asset by ID or legacyID.</br>   **Note**: - We will continue displaying and supporting the legacy ID (API ID).  Since not all Fireblocks services fully support the new Assets UUID, please use only the legacy ID until further notice. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID or legacyId of the asset | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::Asset**](Asset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockchain

> models::BlockchainResponse get_blockchain(id)
Get a Blockchain by ID

Returns a blockchain by ID or legacyID. 

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


## get_supported_assets

> Vec<models::AssetTypeResponse> get_supported_assets()
Legacy - List Assets

Legacy Endpoint – Retrieves all assets supported by Fireblocks in your workspace without extended information.</br>  **Note**:  - This endpoint will remain available for the foreseeable future and is not deprecated.</br> - The [List assets](https://developers.fireblocks.com/reference/listassets) endpoint provides more detailed asset information and improved performance.</br> - We recommend transitioning to the [List assets](https://developers.fireblocks.com/reference/listassets) endpoint for better results.  </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor. 

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


## list_assets

> models::ListAssetsResponse list_assets(blockchain_id, asset_class, symbol, scope, deprecated, ids, page_cursor, page_size, idempotency_key)
List assets

Retrieves all assets supported by Fireblocks in your workspace, providing extended information and enhanced performance compared to the legacy `supported_assets` endpoint.</br>   **Note**:  - We will continue displaying and supporting the legacy ID (API ID). Since not all Fireblocks services fully support the new Assets UUID, please use only the legacy ID until further notice.</br> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain_id** | Option<**String**> | Blockchain id of the assets |  |
**asset_class** | Option<[**AssetClass**](.md)> | Assets class |  |
**symbol** | Option<**String**> | Assets onchain symbol |  |
**scope** | Option<[**AssetScope**](.md)> | Scope of the assets |  |
**deprecated** | Option<**bool**> | Are assets deprecated |  |
**ids** | Option<[**Vec<String>**](String.md)> | A list of asset IDs (max 100) |  |
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

> models::ListBlockchainsResponse list_blockchains(protocol, deprecated, test, ids, page_cursor, page_size)
List blockchains

Returns all blockchains supported by Fireblocks.</br> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**protocol** | Option<**String**> | Blockchain protocol |  |
**deprecated** | Option<**bool**> | Is blockchain deprecated |  |
**test** | Option<**bool**> | Is test blockchain |  |
**ids** | Option<[**Vec<String>**](String.md)> | A list of blockchain IDs (max 100) |  |
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


## register_new_asset

> models::AssetResponse register_new_asset(idempotency_key, register_new_asset_request)
Register an asset

Register a new asset to a workspace and return the newly created asset's details. Currently supported chains are:    - EVM based chains   - Stellar   - Algorand   - TRON   - Solana  Learn more about Supporting Additional Assets in Fireblocks  in the following [guide](https://developers.fireblocks.com/docs/add-your-tokens-1). </br>Endpoint Permission: Owner, Admin, Non-Signing Admin, NCW Admin, Editor, and Signer. 

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


## set_asset_price

> models::AssetPriceResponse set_asset_price(id, idempotency_key, set_asset_price_request)
Set asset price

Set asset price for the given asset id. Returns the asset price response. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the asset | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**set_asset_price_request** | Option<[**SetAssetPriceRequest**](SetAssetPriceRequest.md)> |  |  |

### Return type

[**models::AssetPriceResponse**](AssetPriceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_address

> models::ValidateAddressResponse validate_address(asset_id, address)
Validate destination address

Checks if an address is valid and active (for XRP, DOT, XLM, and EOS). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

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

