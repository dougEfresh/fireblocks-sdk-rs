# VaultAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Vault Account unique identifier | 
**name** | **String** | Vault Account name | 
**assets** | [**Vec<models::VaultAsset>**](VaultAsset.md) | An array of vault assets | 
**hidden_on_ui** | **bool** | Whether the Vault Account is visible in the UI or not | 
**customer_ref_id** | Option<**String**> | Customer reference ID for AML integrations | [optional]
**auto_fuel** | Option<**bool**> | Whether the Vault Account is monitored by the Fireblocks Gas Station or not | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


