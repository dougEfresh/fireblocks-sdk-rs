# \FiatAccountsApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fiat_accounts_account_id_deposit_from_linked_dda_post**](FiatAccountsApi.md#fiat_accounts_account_id_deposit_from_linked_dda_post) | **POST** /fiat_accounts/{accountId}/deposit_from_linked_dda | Deposit funds from DDA
[**fiat_accounts_account_id_get**](FiatAccountsApi.md#fiat_accounts_account_id_get) | **GET** /fiat_accounts/{accountId} | Find a specific fiat account
[**fiat_accounts_account_id_redeem_to_linked_dda_post**](FiatAccountsApi.md#fiat_accounts_account_id_redeem_to_linked_dda_post) | **POST** /fiat_accounts/{accountId}/redeem_to_linked_dda | Redeem funds to DDA
[**fiat_accounts_get**](FiatAccountsApi.md#fiat_accounts_get) | **GET** /fiat_accounts | List fiat accounts



## fiat_accounts_account_id_deposit_from_linked_dda_post

> fiat_accounts_account_id_deposit_from_linked_dda_post(account_id, fiat_accounts_account_id_redeem_to_linked_dda_post_request)
Deposit funds from DDA

Deposits funds from the linked DDA.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The ID of the fiat account to use | [required] |
**fiat_accounts_account_id_redeem_to_linked_dda_post_request** | Option<[**FiatAccountsAccountIdRedeemToLinkedDdaPostRequest**](FiatAccountsAccountIdRedeemToLinkedDdaPostRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fiat_accounts_account_id_get

> models::FiatAccount fiat_accounts_account_id_get(account_id)
Find a specific fiat account

Returns a fiat account by ID.

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
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fiat_accounts_account_id_redeem_to_linked_dda_post

> fiat_accounts_account_id_redeem_to_linked_dda_post(account_id, fiat_accounts_account_id_redeem_to_linked_dda_post_request)
Redeem funds to DDA

Redeems funds to the linked DDA.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The ID of the fiat account to use | [required] |
**fiat_accounts_account_id_redeem_to_linked_dda_post_request** | Option<[**FiatAccountsAccountIdRedeemToLinkedDdaPostRequest**](FiatAccountsAccountIdRedeemToLinkedDdaPostRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fiat_accounts_get

> Vec<models::FiatAccount> fiat_accounts_get()
List fiat accounts

Returns all fiat accounts.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::FiatAccount>**](FiatAccount.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

