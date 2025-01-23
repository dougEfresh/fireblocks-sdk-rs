# \OffExchangeApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_off_exchange**](OffExchangeApi.md#add_off_exchange) | **POST** /off_exchange/add | Add Collateral
[**get_off_exchange_collateral_accounts**](OffExchangeApi.md#get_off_exchange_collateral_accounts) | **GET** /off_exchange/collateral_accounts/{mainExchangeAccountId} | Find a specific collateral exchange account
[**get_off_exchange_settlement_transactions**](OffExchangeApi.md#get_off_exchange_settlement_transactions) | **GET** /off_exchange/settlements/transactions | Get Settlements Transactions
[**remove_off_exchange**](OffExchangeApi.md#remove_off_exchange) | **POST** /off_exchange/remove | Remove Collateral
[**settle_off_exchange_trades**](OffExchangeApi.md#settle_off_exchange_trades) | **POST** /off_exchange/settlements/trader | Create Settlement for a Trader



## add_off_exchange

> models::CreateTransactionResponse add_off_exchange(idempotency_key, add_collateral_request_body)
Add Collateral

Add collateral and create deposit request. Learn more about Fireblocks Off Exchange in the following [guide](https://developers.fireblocks.com/docs/off-exchange). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

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

Returns a collateral account by mainExchangeAccountId. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

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
Get Settlements Transactions

Get settlements transactions from exchange. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

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
Remove Collateral

Remove collateral and create withdraw request. </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

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
Create Settlement for a Trader

Create settlement for a trader. Learn more about Fireblocks Off Exchange in the following [guide](https://developers.fireblocks.com/docs/off-exchange). </br>Endpoint Permission: Admin, Non-Signing Admin, Signer, Approver, Editor.

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

