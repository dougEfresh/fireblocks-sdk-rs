# \FiatAccountsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**deposit_funds_from_linked_dda**](FiatAccountsApi.md#deposit_funds_from_linked_dda) | **POST** /fiat_accounts/{accountId}/deposit_from_linked_dda | Deposit funds from DDA
[**get_fiat_account**](FiatAccountsApi.md#get_fiat_account) | **GET** /fiat_accounts/{accountId} | Find a specific fiat account
[**get_fiat_accounts**](FiatAccountsApi.md#get_fiat_accounts) | **GET** /fiat_accounts | List fiat accounts
[**redeem_funds_to_linked_dda**](FiatAccountsApi.md#redeem_funds_to_linked_dda) | **POST** /fiat_accounts/{accountId}/redeem_to_linked_dda | Redeem funds to DDA



## deposit_funds_from_linked_dda

> models::DepositFundsFromLinkedDdaResponse deposit_funds_from_linked_dda(account_id, idempotency_key, funds)
Deposit funds from DDA

Deposits funds from the linked DDA. Learn more about Fireblocks FIAT Connectivity in the following [guide](https://developers.fireblocks.com/docs/connect-to-exchanges-and-fiat-providers). </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The ID of the fiat account to use | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**funds** | Option<[**Funds**](Funds.md)> |  |  |

### Return type

[**models::DepositFundsFromLinkedDdaResponse**](DepositFundsFromLinkedDDAResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fiat_account

> models::FiatAccount get_fiat_account(account_id)
Find a specific fiat account

Returns a fiat account by ID. </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The ID of the fiat account to return | [required] |

### Return type

[**models::FiatAccount**](FiatAccount.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fiat_accounts

> Vec<models::FiatAccount> get_fiat_accounts()
List fiat accounts

Returns all fiat accounts. </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::FiatAccount>**](FiatAccount.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## redeem_funds_to_linked_dda

> models::RedeemFundsToLinkedDdaResponse redeem_funds_to_linked_dda(account_id, idempotency_key, funds)
Redeem funds to DDA

Redeems funds to the linked DDA. Learn more about Fireblocks FIAT Connectivity in the following [guide](https://developers.fireblocks.com/docs/connect-to-exchanges-and-fiat-providers). </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The ID of the fiat account to use | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**funds** | Option<[**Funds**](Funds.md)> |  |  |

### Return type

[**models::RedeemFundsToLinkedDdaResponse**](RedeemFundsToLinkedDDAResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

