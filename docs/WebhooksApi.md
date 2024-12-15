# \WebhooksApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**webhooks_resend_post**](WebhooksApi.md#webhooks_resend_post) | **POST** /webhooks/resend | Resend failed webhooks
[**webhooks_resend_tx_id_post**](WebhooksApi.md#webhooks_resend_tx_id_post) | **POST** /webhooks/resend/{txId} | Resend failed webhooks by Transaction ID



## webhooks_resend_post

> models::ResendWebhooksResponse webhooks_resend_post()
Resend failed webhooks

Resends all failed webhook notifications.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ResendWebhooksResponse**](ResendWebhooksResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhooks_resend_tx_id_post

> webhooks_resend_tx_id_post(tx_id, webhooks_resend_tx_id_post_request)
Resend failed webhooks by Transaction ID

Resends failed webhook notifications for a specific transaction identified by Fireblocks Transaction ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_id** | **String** | The Fireblocks Transaction ID | [required] |
**webhooks_resend_tx_id_post_request** | [**WebhooksResendTxIdPostRequest**](WebhooksResendTxIdPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

