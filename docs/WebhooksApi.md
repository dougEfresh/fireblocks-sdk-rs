# \WebhooksApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**resend_transaction_webhooks**](WebhooksApi.md#resend_transaction_webhooks) | **POST** /webhooks/resend/{txId} | Resend webhooks for a transaction by ID
[**resend_webhooks**](WebhooksApi.md#resend_webhooks) | **POST** /webhooks/resend | Resend failed webhooks



## resend_transaction_webhooks

> models::ResendWebhooksByTransactionIdResponse resend_transaction_webhooks(tx_id, resend_transaction_webhooks_request, idempotency_key)
Resend webhooks for a transaction by ID

Resends webhook notifications for a transaction by its unique identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** | The ID of the transaction for webhooks | [required] |
**resend_transaction_webhooks_request** | [**ResendTransactionWebhooksRequest**](ResendTransactionWebhooksRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::ResendWebhooksByTransactionIdResponse**](ResendWebhooksByTransactionIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_webhooks

> models::ResendWebhooksResponse resend_webhooks(idempotency_key)
Resend failed webhooks

Resends all failed webhook notifications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::ResendWebhooksResponse**](ResendWebhooksResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

