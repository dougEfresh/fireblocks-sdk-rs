# CreateMultipleAccountsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**count** | **i32** | Count | 
**base_asset_ids** | **Vec<String>** | Array of asset IDs | 
**names** | Option<**Vec<String>**> | Names to assign to vault accounts. if vaultAccountNamesStartingIndex or prefix is used it'll fail | [optional]
**vault_account_names_starting_index** | Option<**i32**> | Copy vault accounts names starting from this index. If names array is used it'll fail | [optional]
**prefix** | Option<**String**> | When copying from existing vault accounts (vaultAccountNamesStartingIndex) then adding a prefix to the names. If names array is used it'll fail | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


