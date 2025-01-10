# \ExchangeAccountsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**convert_assets**](ExchangeAccountsApi.md#convert_assets) | **POST** /exchange_accounts/{exchangeAccountId}/convert | Convert exchange account funds from the source asset to the destination asset
[**get_exchange_account**](ExchangeAccountsApi.md#get_exchange_account) | **GET** /exchange_accounts/{exchangeAccountId} | Get a specific exchange account
[**get_exchange_account_asset**](ExchangeAccountsApi.md#get_exchange_account_asset) | **GET** /exchange_accounts/{exchangeAccountId}/{assetId} | Get an asset for an exchange account
[**get_paged_exchange_accounts**](ExchangeAccountsApi.md#get_paged_exchange_accounts) | **GET** /exchange_accounts/paged | List connected exchange accounts
[**internal_transfer**](ExchangeAccountsApi.md#internal_transfer) | **POST** /exchange_accounts/{exchangeAccountId}/internal_transfer | Internal transfer for exchange accounts



## convert_assets

> models::ConvertAssetsResponse convert_assets(exchange_account_id, idempotency_key, convert_assets_request)
Convert exchange account funds from the source asset to the destination asset

Convert exchange account funds from the source asset to the destination asset. Coinbase (USD to USDC, USDC to USD) and Bitso (MXN to USD) are supported conversions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exchange_account_id** | **String** | The ID of the exchange account. Please make sure the exchange supports conversions. To find the ID of your exchange account, use GET/exchange_accounts. | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**convert_assets_request** | Option<[**ConvertAssetsRequest**](ConvertAssetsRequest.md)> |  |  |

### Return type

[**models::ConvertAssetsResponse**](ConvertAssetsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_exchange_account

> models::ExchangeAccount get_exchange_account(exchange_account_id)
Get a specific exchange account

Returns an exchange account by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exchange_account_id** | **String** | The ID of the exchange account to return | [required] |

### Return type

[**models::ExchangeAccount**](ExchangeAccount.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_exchange_account_asset

> models::ExchangeAsset get_exchange_account_asset(exchange_account_id, asset_id)
Get an asset for an exchange account

Returns an asset for an exchange account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exchange_account_id** | **String** | The ID of the exchange account to return | [required] |
**asset_id** | **String** | The ID of the asset to return | [required] |

### Return type

[**models::ExchangeAsset**](ExchangeAsset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_paged_exchange_accounts

> Vec<models::ExchangeAccountsPaged> get_paged_exchange_accounts(limit, before, after)
List connected exchange accounts

Returns a list of the connected exchange accounts in your workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | **f64** | number of exchanges per page | [required] |[default to 3]
**before** | Option<**String**> |  |  |
**after** | Option<**String**> |  |  |

### Return type

[**Vec<models::ExchangeAccountsPaged>**](ExchangeAccountsPaged.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## internal_transfer

> models::InternalTransferResponse internal_transfer(exchange_account_id, idempotency_key, create_internal_transfer_request)
Internal transfer for exchange accounts

Transfers funds between trading accounts under the same exchange account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exchange_account_id** | **String** | The ID of the exchange account to return | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**create_internal_transfer_request** | Option<[**CreateInternalTransferRequest**](CreateInternalTransferRequest.md)> |  |  |

### Return type

[**models::InternalTransferResponse**](InternalTransferResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
