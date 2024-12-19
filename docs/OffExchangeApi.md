# \OffExchangeApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_off_exchange**](OffExchangeApi.md#add_off_exchange) | **POST** /off_exchange/add | add collateral
[**get_off_exchange_collateral_accounts**](OffExchangeApi.md#get_off_exchange_collateral_accounts) | **GET** /off_exchange/collateral_accounts/{mainExchangeAccountId} | Find a specific collateral exchange account
[**get_off_exchange_settlement_transactions**](OffExchangeApi.md#get_off_exchange_settlement_transactions) | **GET** /off_exchange/settlements/transactions | get settlements transactions from exchange
[**remove_off_exchange**](OffExchangeApi.md#remove_off_exchange) | **POST** /off_exchange/remove | remove collateral
[**settle_off_exchange_trades**](OffExchangeApi.md#settle_off_exchange_trades) | **POST** /off_exchange/settlements/trader | create settlement for a trader



## add_off_exchange

> models::CreateTransactionResponse add_off_exchange(idempotency_key, add_collateral_request_body)
add collateral

add collateral, create deposit request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**add_collateral_request_body** | Option<[**AddCollateralRequestBody**](AddCollateralRequestBody.md)> |  |  |

### Return type

[**models::CreateTransactionResponse**](CreateTransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_off_exchange_collateral_accounts

> models::ExchangeAccount get_off_exchange_collateral_accounts(main_exchange_account_id)
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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_off_exchange_settlement_transactions

> models::ExchangeSettlementTransactionsResponse get_off_exchange_settlement_transactions(main_exchange_account_id)
get settlements transactions from exchange

get settlements transactions from exchange

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**main_exchange_account_id** | **String** |  | [required] |

### Return type

[**models::ExchangeSettlementTransactionsResponse**](ExchangeSettlementTransactionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_off_exchange

> models::CreateTransactionResponse remove_off_exchange(idempotency_key, remove_collateral_request_body)
remove collateral

remove collateral, create withdraw request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**remove_collateral_request_body** | Option<[**RemoveCollateralRequestBody**](RemoveCollateralRequestBody.md)> |  |  |

### Return type

[**models::CreateTransactionResponse**](CreateTransactionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## settle_off_exchange_trades

> models::SettlementResponse settle_off_exchange_trades(idempotency_key, settlement_request_body)
create settlement for a trader

create settlement for a trader

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**settlement_request_body** | Option<[**SettlementRequestBody**](SettlementRequestBody.md)> |  |  |

### Return type

[**models::SettlementResponse**](SettlementResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

