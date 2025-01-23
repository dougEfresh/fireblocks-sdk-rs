# \PaymentsPayoutApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_payout**](PaymentsPayoutApi.md#create_payout) | **POST** /payments/payout | Create a payout instruction set
[**execute_payout_action**](PaymentsPayoutApi.md#execute_payout_action) | **POST** /payments/payout/{payoutId}/actions/execute | Execute a payout instruction set
[**get_payout**](PaymentsPayoutApi.md#get_payout) | **GET** /payments/payout/{payoutId} | Get the status of a payout instruction set



## create_payout

> models::PayoutResponse create_payout(idempotency_key, create_payout_request)
Create a payout instruction set

**Note:** The reference content in this section documents the Payments Engine endpoint. The Payments Engine endpoints include APIs available only for customers with Payments Engine enabled on their accounts. </br> </br>These endpoints are currently in beta and might be subject to changes.</br> </br>If you want to learn more about Fireblocks Payments Engine, please contact your Fireblocks Customer Success Manager or email CSM@fireblocks.com. </br> </br> <b u>Create a payout instruction set.</b> </u></br> A payout instruction set is a set of instructions for distributing payments from a single payment account to a list of payee accounts. </br> The instruction set defines: </br> <ul> <li>the payment account and its account type (vault, exchange, or fiat). </li> <li>the account type (vault account, exchange account, whitelisted address, network connection, fiat account, or merchant account), the amount, and the asset of payment for each payee account.</li> </ul> Learn more about Fireblocks Payments - Payouts in the following [guide](https://developers.fireblocks.com/docs/create-payouts). </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |
**create_payout_request** | Option<[**CreatePayoutRequest**](CreatePayoutRequest.md)> |  |  |

### Return type

[**models::PayoutResponse**](PayoutResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## execute_payout_action

> models::DispatchPayoutResponse execute_payout_action(payout_id, idempotency_key)
Execute a payout instruction set

**Note:** The reference content in this section documents the Payments Engine endpoint. The Payments Engine endpoints include APIs available only for customers with Payments Engine enabled on their accounts. </br> </br>These endpoints are currently in beta and might be subject to changes.</br> </br>If you want to learn more about Fireblocks Payments Engine, please contact your Fireblocks Customer Success Manager or email CSM@fireblocks.com. </br> </br><b u>Execute a payout instruction set.</b> </u> </br> </br>The instruction set will be verified and executed.</br> <b><u>Source locking</br></b> </u> If you are executing a payout instruction set from a payment account with an already active payout the active payout will complete before the new payout instruction set can be executed. </br> You cannot execute the same payout instruction set more than once. </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payout_id** | **String** | the payout id received from the creation of the payout instruction set | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::DispatchPayoutResponse**](DispatchPayoutResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payout

> models::PayoutResponse get_payout(payout_id)
Get the status of a payout instruction set

**Note:** The reference content in this section documents the Payments Engine endpoint. The Payments Engine endpoints include APIs available only for customers with Payments Engine enabled on their accounts. </br> </br>These endpoints are currently in beta and might be subject to changes.</br> </br>If you want to learn more about Fireblocks Payments Engine, please contact your Fireblocks Customer Success Manager or email CSM@fireblocks.com. </br> </br>Endpoint Permission: Admin, Non-Signing Admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payout_id** | **String** | the payout id received from the creation of the payout instruction set | [required] |

### Return type

[**models::PayoutResponse**](PayoutResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

