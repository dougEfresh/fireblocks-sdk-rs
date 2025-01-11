# SmartTransferTicket

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique id of Smart Transfer ticket | 
**r#type** | **String** | Kind of Smart Transfer. Can be either `ASYNC` or `ATOMIC` | 
**direction** | Option<**String**> | Direction of Smart Transfer. | [optional]
**status** | **String** | Current status of Smart Transfer ticket | 
**terms** | Option<[**Vec<models::SmartTransferTicketTerm>**](SmartTransferTicketTerm.md)> | Ticket terms (legs) | [optional]
**expires_in** | Option<**f64**> | Number of hours for expiration.This data is valid only it ticket not in DRAFT state and it will be used to calculate expiresAt value | [optional]
**expires_at** | Option<**String**> | Date and time at which the ticket will expire if no funding is performed. | [optional]
**submitted_at** | Option<**String**> | Date and time when ticket is submitted. | [optional]
**expired_at** | Option<**String**> | Date and time when ticket is expired. | [optional]
**canceled_at** | Option<**String**> | Date and time when ticket is canceled. | [optional]
**fulfilled_at** | Option<**String**> | Date and time when ticket is fulfilled. | [optional]
**external_ref_id** | Option<**String**> | External Ref ID for Smart Transfer ticket. | [optional]
**note** | Option<**String**> | Note | [optional]
**created_by_network_id** | **String** | ID of network profile that created ticket | 
**created_by_network_id_name** | **String** | Name of network profile that created ticket | 
**canceled_by_network_id_name** | Option<**String**> | Name of network profile that canceled ticket | [optional]
**created_at** | **String** | Date and time at which the ticket is created. | 
**updated_at** | **String** | Date and time of last ticket update. | 
**canceled_by_me** | Option<**bool**> |  | [optional]
**created_by_me** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


