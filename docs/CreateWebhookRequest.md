# CreateWebhookRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | The url of the webhook where notifications will be sent. URL must be valid, unique and https. | 
**description** | **String** | description of the webhook. should not contain special characters. | 
**events** | [**Vec<models::WebhookEvent>**](WebhookEvent.md) | event types the webhook will subscribe to | 
**enabled** | Option<**bool**> | The status of the webhook. If false, the webhook will not receive notifications. | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


