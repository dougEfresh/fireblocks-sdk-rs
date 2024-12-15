# \ExchangeAccountsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**exchange_accounts_exchange_account_id_asset_id_get**](ExchangeAccountsApi.md#exchange_accounts_exchange_account_id_asset_id_get) | **GET** /exchange_accounts/{exchangeAccountId}/{assetId} | Find an asset for an exchange account
[**exchange_accounts_exchange_account_id_convert_post**](ExchangeAccountsApi.md#exchange_accounts_exchange_account_id_convert_post) | **POST** /exchange_accounts/{exchangeAccountId}/convert | Convert exchange account funds from the source asset to the destination asset. Coinbase (USD to USDC, USDC to USD) and Bitso (MXN to USD) are supported conversions.
[**exchange_accounts_exchange_account_id_get**](ExchangeAccountsApi.md#exchange_accounts_exchange_account_id_get) | **GET** /exchange_accounts/{exchangeAccountId} | Find a specific exchange account
[**exchange_accounts_exchange_account_id_internal_transfer_post**](ExchangeAccountsApi.md#exchange_accounts_exchange_account_id_internal_transfer_post) | **POST** /exchange_accounts/{exchangeAccountId}/internal_transfer | Internal tranfer for exchange accounts
[**exchange_accounts_get**](ExchangeAccountsApi.md#exchange_accounts_get) | **GET** /exchange_accounts | List exchange accounts
[**exchange_accounts_paged_get**](ExchangeAccountsApi.md#exchange_accounts_paged_get) | **GET** /exchange_accounts/paged | List exchange accounts (paged)



## exchange_accounts_exchange_account_id_asset_id_get

> models::ExchangeAsset exchange_accounts_exchange_account_id_asset_id_get(exchange_account_id, asset_id)
Find an asset for an exchange account

Returns an asset for an exchange account.

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
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exchange_accounts_exchange_account_id_convert_post

> exchange_accounts_exchange_account_id_convert_post(exchange_account_id, exchange_accounts_exchange_account_id_convert_post_request)
Convert exchange account funds from the source asset to the destination asset. Coinbase (USD to USDC, USDC to USD) and Bitso (MXN to USD) are supported conversions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exchange_account_id** | **String** | The ID of the exchange account. Please make sure the exchange supports conversions. To find the ID of your exchange account, use GET/exchange_accounts. | [required] |
**exchange_accounts_exchange_account_id_convert_post_request** | Option<[**ExchangeAccountsExchangeAccountIdConvertPostRequest**](ExchangeAccountsExchangeAccountIdConvertPostRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exchange_accounts_exchange_account_id_get

> models::ExchangeAccount exchange_accounts_exchange_account_id_get(exchange_account_id)
Find a specific exchange account

Returns an exchange account by ID.

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
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exchange_accounts_exchange_account_id_internal_transfer_post

> exchange_accounts_exchange_account_id_internal_transfer_post(exchange_account_id, exchange_accounts_exchange_account_id_internal_transfer_post_request)
Internal tranfer for exchange accounts

Transfers funds between trading accounts under the same exchange account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exchange_account_id** | **String** | The ID of the exchange account to return | [required] |
**exchange_accounts_exchange_account_id_internal_transfer_post_request** | Option<[**ExchangeAccountsExchangeAccountIdInternalTransferPostRequest**](ExchangeAccountsExchangeAccountIdInternalTransferPostRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exchange_accounts_get

> Vec<models::ExchangeAccount> exchange_accounts_get()
List exchange accounts

Returns all exchange accounts. </br> **Note:** This endpoint will be deprecated soon. Please use the `/exchange_accounts/paged` endpoint instead. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ExchangeAccount>**](ExchangeAccount.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exchange_accounts_paged_get

> Vec<models::ExchangeAccountsPaged> exchange_accounts_paged_get(limit, before, after)
List exchange accounts (paged)

Returns a paginated list of exchange accounts.

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
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

