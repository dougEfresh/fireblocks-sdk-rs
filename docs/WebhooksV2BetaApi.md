# \WebhooksV2BetaApi

All URIs are relative to *https://api.fireblocks.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_webhook**](WebhooksV2BetaApi.md#create_webhook) | **POST** /webhooks | Create a new webhook
[**delete_webhook**](WebhooksV2BetaApi.md#delete_webhook) | **DELETE** /webhooks/{webhookId} | Delete a webhook
[**get_notification**](WebhooksV2BetaApi.md#get_notification) | **GET** /webhooks/{webhookId}/notifications/{notificationId} | Get notification by id
[**get_notifications**](WebhooksV2BetaApi.md#get_notifications) | **GET** /webhooks/{webhookId}/notifications | Get all notifications by webhook id
[**get_webhook**](WebhooksV2BetaApi.md#get_webhook) | **GET** /webhooks/{webhookId} | Get webhook by id
[**get_webhooks**](WebhooksV2BetaApi.md#get_webhooks) | **GET** /webhooks | Get all webhooks
[**resend_notification_by_id**](WebhooksV2BetaApi.md#resend_notification_by_id) | **POST** /webhooks/{webhookId}/notifications/{notificationId}/resend | Resend notification by id
[**resend_notifications_by_resource_id**](WebhooksV2BetaApi.md#resend_notifications_by_resource_id) | **POST** /webhooks/{webhookId}/notifications/resend_by_resource | Resend notifications by resource Id
[**update_webhook**](WebhooksV2BetaApi.md#update_webhook) | **PATCH** /webhooks/{webhookId} | Update webhook



## create_webhook

> models::Webhook create_webhook(create_webhook_request, idempotency_key)
Create a new webhook

Creates a new webhook, which will be triggered on the specified events  **Note:**  - This endpoint is currently in beta and might be subject to changes.  - This endpoint requires Admin privilege or above due to the sensitivity of actions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_webhook_request** | [**CreateWebhookRequest**](CreateWebhookRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

[**models::Webhook**](Webhook.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhook

> models::Webhook delete_webhook(webhook_id)
Delete a webhook

Delete a webhook by its id  **Note:**  - This endpoint is currently in beta and might be subject to changes.  - This endpoint requires Admin privilege or above due to the sensitivity of actions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **uuid::Uuid** | The unique identifier of the webhook | [required] |

### Return type

[**models::Webhook**](Webhook.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification

> models::NotificationWithData get_notification(webhook_id, notification_id, include_data)
Get notification by id

Get notification by id **Note:** These endpoints are currently in beta and might be subject to changes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook to fetch | [required] |
**notification_id** | **String** | The ID of the notification to fetch | [required] |
**include_data** | Option<**bool**> | Include the data of the notification |  |

### Return type

[**models::NotificationWithData**](NotificationWithData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications

> models::NotificationPaginatedResponse get_notifications(webhook_id, order, page_cursor, page_size, created_start_date, created_end_date, statuses, event_types, resource_id)
Get all notifications by webhook id

Get all notifications by webhook id (paginated) **Note:** These endpoints are currently in beta and might be subject to changes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **uuid::Uuid** |  | [required] |
**order** | Option<**String**> | ASC / DESC ordering (default DESC) |  |[default to DESC]
**page_cursor** | Option<**String**> | Cursor of the required page |  |
**page_size** | Option<**f64**> | Maximum number of items in the page |  |[default to 100]
**created_start_date** | Option<**String**> | sort by start date |  |
**created_end_date** | Option<**String**> | sort by end date |  |
**statuses** | Option<[**Vec<models::NotificationStatus>**](models::NotificationStatus.md)> | Filter by Notification statues |  |
**event_types** | Option<[**Vec<models::WebhookEvent>**](models::WebhookEvent.md)> | Filter by Notification eventTypes |  |
**resource_id** | Option<**String**> | Filter by resourceId |  |

### Return type

[**models::NotificationPaginatedResponse**](NotificationPaginatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook

> models::Webhook get_webhook(webhook_id)
Get webhook by id

Retrieve a webhook by its id **Note:** These endpoints are currently in beta and might be subject to changes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **uuid::Uuid** | The unique identifier of the webhook | [required] |

### Return type

[**models::Webhook**](Webhook.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhooks

> models::WebhookPaginatedResponse get_webhooks(order, page_cursor, page_size)
Get all webhooks

Get all webhooks (paginated) **Note:** These endpoints are currently in beta and might be subject to changes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order** | Option<**String**> | ASC / DESC ordering (default DESC) |  |[default to DESC]
**page_cursor** | Option<**String**> | Cursor of the required page |  |
**page_size** | Option<**f64**> | Maximum number of items in the page |  |[default to 10]

### Return type

[**models::WebhookPaginatedResponse**](WebhookPaginatedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_notification_by_id

> resend_notification_by_id(webhook_id, notification_id, idempotency_key)
Resend notification by id

Resend notification by ID **Note:** These endpoints are currently in beta and might be subject to changes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook | [required] |
**notification_id** | **String** | The ID of the notification | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_notifications_by_resource_id

> resend_notifications_by_resource_id(webhook_id, resend_notifications_by_resource_id_request, idempotency_key)
Resend notifications by resource Id

Resend notifications by resource Id **Note:** These endpoints are currently in beta and might be subject to changes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook | [required] |
**resend_notifications_by_resource_id_request** | [**ResendNotificationsByResourceIdRequest**](ResendNotificationsByResourceIdRequest.md) |  | [required] |
**idempotency_key** | Option<**String**> | A unique identifier for the request. If the request is sent multiple times with the same idempotency key, the server will return the same response as the first request. The idempotency key is valid for 24 hours. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook

> models::Webhook update_webhook(webhook_id, update_webhook_request)
Update webhook

Update a webhook by its id  **Note:**  - This endpoint is currently in beta and might be subject to changes.  - This endpoint requires Admin privilege or above due to the sensitivity of actions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **uuid::Uuid** | The unique identifier of the webhook | [required] |
**update_webhook_request** | [**UpdateWebhookRequest**](UpdateWebhookRequest.md) |  | [required] |

### Return type

[**models::Webhook**](Webhook.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

