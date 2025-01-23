# NotificationWithData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**created_at** | **String** | The creation date of the notification | 
**updated_at** | **String** | The date when the notification was updated | 
**status** | [**models::NotificationStatus**](NotificationStatus.md) |  | 
**event_type** | [**models::WebhookEvent**](WebhookEvent.md) |  | 
**event_version** | **f64** | The event version which the Notification is listen to | 
**resource_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The resource id of the event which the Notification is listen to | [optional]
**attempts** | **Vec<String>** | The attempts related to Notification | [default to []]
**data** | Option<[**serde_json::Value**](.md)> | notification data | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


