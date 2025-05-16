# ResendNotificationsByResourceIdRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**resource_id** | [**uuid::Uuid**](uuid::Uuid.md) | The resource id to resend notifications for | 
**exclude_statuses** | Option<[**Vec<models::NotificationStatus>**](NotificationStatus.md)> | (optional) List of notification statuses to exclude from the resend operation     - Empty array means all statuses will be included     - If you want to exclude some statuses, you can use the following example: [ IN_PROGRESS, FAILED ]     - Default if missing, means all statuses other than \"COMPLETED\" will be included  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


