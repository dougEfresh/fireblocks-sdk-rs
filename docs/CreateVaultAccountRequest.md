# CreateVaultAccountRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Vault Account name | [optional]
**hidden_on_ui** | Option<**bool**> | Optional - if true, the created account and all related transactions will not be shown on Fireblocks console | [optional]
**customer_ref_id** | Option<**String**> | Optional - Sets a customer reference ID for AML integrations | [optional]
**auto_fuel** | Option<**bool**> | Optional - Sets the autoFuel property of the vault account for the Fireblocks Gas Station | [optional]
**vault_type** | Option<**String**> | Type of vault account. The default type will be set to MPC.<br/>  If the workspace does not support the selected type, it will return an error. | [optional][default to Mpc]
**auto_assign** | Option<**bool**> | Applicable only when the vault account type is KEY_LINK. For MPC, this parameter will be ignored.<br/> If set to true and there are available keys, random keys will be assigned to the newly created vault account.<br/> If set to true and there are no available keys to be assigned, it will return an error.<br/> If set to false, the vault account will be created without any keys. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


