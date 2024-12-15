# AssetWallet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vault_id** | Option<**String**> | ID of the vault account. You can [get the vault account by this ID](https://developers.fireblocks.com/reference/get_vault-accounts-vaultaccountid) to retrieve vault properties such as its name, auto fueling, hidden on UI or customer reference ID. | [optional]
**asset_id** | Option<**String**> | ID of the asset. You can get more information about this asset by using the [supported assets API](https://developers.fireblocks.com/reference/get_supported-assets) | [optional]
**available** | Option<**String**> | Available balance, available to use in a transaction. | [optional]
**total** | Option<**String**> | Total balance at the asset wallet, as seen at the blockchain explorers. This includes balance available, and any kind of unavailable balance such as locked, frozen, or others. | [optional]
**pending** | Option<**String**> | Pending balance. | [optional]
**staked** | Option<**String**> | Staked balance. | [optional]
**frozen** | Option<**String**> | Funds frozen due to the anti-money laundering policy at this workspace. | [optional]
**locked_amount** | Option<**String**> | Locked balance. | [optional]
**block_height** | Option<**String**> | The height (number) of the block of the balance. Can by empty. | [optional]
**block_hash** | Option<**String**> | The hash of the block of the balance. Can by empty. | [optional]
**creation_timestamp** | Option<**String**> | Unix timestamp of the time the asset wallet was created. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


