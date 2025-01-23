# UpdateWebhookRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | Option<**String**> | The url of the webhook where notifications will be sent. URL must be valid, unique and https. | [optional]
**description** | Option<**String**> | description of the webhook of what it is used for.should not contain special characters. | [optional]
**events** | Option<[**Vec<models::WebhookEvent>**](WebhookEvent.md)> | The events that the webhook will be subscribed to | [optional]
**enabled** | Option<**bool**> | The status of the webhook | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


