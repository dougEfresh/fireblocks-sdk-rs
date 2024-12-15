# TransactionResponseDestination

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination** | Option<[**models::DestinationTransferPeerPathResponse**](DestinationTransferPeerPathResponse.md)> |  | [optional]
**destination_address** | Option<[**serde_json::Value**](.md)> | Address where the asset was transferred. | [optional]
**destination_address_description** | Option<[**serde_json::Value**](.md)> | Description of the address. | [optional]
**amount** | Option<**String**> | The amount to be sent to this destination. | [optional]
**amount_usd** | Option<**String**> | The USD value of the requested amount. | [optional]
**aml_screening_result** | Option<[**models::AmlScreeningResult**](AmlScreeningResult.md)> |  | [optional]
**customer_ref_id** | Option<[**serde_json::Value**](.md)> | The ID for AML providers to associate the owner of funds with transactions. | [optional]
**authorization_info** | Option<[**models::AuthorizationInfo**](AuthorizationInfo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


