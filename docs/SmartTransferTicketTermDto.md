# SmartTransferTicketTermDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique id of Smart Transfer ticket term | 
**ticket_id** | **String** | Unique id of Smart Transfer ticket | 
**asset** | **String** | Asset name | 
**amount** | **f64** | Amount | 
**from_network_id** | **String** | Identifier of the origination Network Profile | 
**from_network_id_name** | **String** | Source network name | 
**to_network_id** | **String** | Identifier of the destination Network Profile | 
**to_network_id_name** | **String** | Destination network name | 
**tx_hash** | Option<**String**> | Blockchain TX hash | 
**fb_tx_id** | Option<**String**> | Fireblocks transaction ID. It is set when the funding transaction is created. | 
**tx_status** | Option<**String**> | Ticket term transaction status | 
**status** | **String** | Ticket term status | 
**created_at** | **String** | Date and time when the term is created. | 
**updated_at** | **String** | Date and time of last term update. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


