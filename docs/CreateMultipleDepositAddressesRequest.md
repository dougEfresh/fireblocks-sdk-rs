# CreateMultipleDepositAddressesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vault_account_id** | **i32** | Existing Vault account ID to add deposit addresses to | 
**asset_id** | **String** | asset ID | 
**count** | **i32** | Count of deposit addresses to issue | 
**descriptions** | Option<**Vec<String>**> | Desctiptions of the newly created addresses | [optional]
**vault_account_to_copy_desc_from** | Option<**i32**> | Existing Vault Account ID to copy deposit addresses descriptions from in case no descriptions were provided | [optional]
**vault_account_to_copy_desc_from_index** | Option<**i32**> | Existing length within the vault account to copy deposit addresses descriptions from | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


