# \OffExchangesApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**off_exchange_add_post**](OffExchangesApi.md#off_exchange_add_post) | **POST** /off_exchange/add | add collateral
[**off_exchange_collateral_accounts_main_exchange_account_id_get**](OffExchangesApi.md#off_exchange_collateral_accounts_main_exchange_account_id_get) | **GET** /off_exchange/collateral_accounts/{mainExchangeAccountId} | Find a specific collateral exchange account
[**off_exchange_remove_post**](OffExchangesApi.md#off_exchange_remove_post) | **POST** /off_exchange/remove | remove collateral
[**off_exchange_settlements_trader_post**](OffExchangesApi.md#off_exchange_settlements_trader_post) | **POST** /off_exchange/settlements/trader | create settlement for a trader
[**off_exchange_settlements_transactions_get**](OffExchangesApi.md#off_exchange_settlements_transactions_get) | **GET** /off_exchange/settlements/transactions | get settlements transactions from exchange



## off_exchange_add_post

> models::CreateTransactionResponse off_exchange_add_post(add_collateral_request_body)
add collateral

add collateral, create deposit request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_collateral_request_body** | Option<[**AddCollateralRequestBody**](AddCollateralRequestBody.md)> |  |  |

### Return type

[**models::CreateTransactionResponse**](CreateTransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## off_exchange_collateral_accounts_main_exchange_account_id_get

> models::ExchangeAccount off_exchange_collateral_accounts_main_exchange_account_id_get(main_exchange_account_id)
Find a specific collateral exchange account

Returns a collateral account by mainExchangeAccountId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**main_exchange_account_id** | **String** | The id of the main exchange account for which the requested collateral account is associated with | [required] |

### Return type

[**models::ExchangeAccount**](ExchangeAccount.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## off_exchange_remove_post

> models::CreateTransactionResponse off_exchange_remove_post(remove_collateral_request_body)
remove collateral

remove collateral, create withdraw request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remove_collateral_request_body** | Option<[**RemoveCollateralRequestBody**](RemoveCollateralRequestBody.md)> |  |  |

### Return type

[**models::CreateTransactionResponse**](CreateTransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## off_exchange_settlements_trader_post

> models::SettlementResponse off_exchange_settlements_trader_post(settlement_request_body)
create settlement for a trader

create settlement for a trader

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**settlement_request_body** | Option<[**SettlementRequestBody**](SettlementRequestBody.md)> |  |  |

### Return type

[**models::SettlementResponse**](SettlementResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## off_exchange_settlements_transactions_get

> models::SettlementResponse off_exchange_settlements_transactions_get(main_exchange_account_id)
get settlements transactions from exchange

get settlements transactions from exchange

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**main_exchange_account_id** | **String** |  | [required] |

### Return type

[**models::SettlementResponse**](SettlementResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

