# Webhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The id of the webhook | [optional]
**url** | Option<**String**> | The url of the webhook where notifications will be sent. Must be a valid URL and https. | [optional]
**description** | Option<**String**> | description of the webhook of what it is used for | [optional]
**events** | Option<[**Vec<models::WebhookEvent>**](WebhookEvent.md)> | The events that the webhook will be subscribed to | [optional]
**status** | Option<**String**> | The status of the webhook | [optional]
**created_at** | Option<**String**> | The date and time the webhook was created | [optional]
**updated_at** | Option<**String**> | The date and time the webhook was last updated | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


