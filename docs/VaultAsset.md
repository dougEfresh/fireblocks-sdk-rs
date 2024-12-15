# VaultAsset

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**total** | Option<**String**> | The total wallet balance. In EOS this value includes the network balance, self staking and pending refund. For all other coins it is the balance as it appears on the blockchain. | [optional]
**balance** | Option<**String**> | Deprecated - replaced by \"total\" | [optional]
**available** | Option<**String**> | Funds available for transfer. Equals the blockchain balance minus any locked amounts | [optional]
**pending** | Option<**String**> | The cumulative balance of all transactions pending to be cleared | [optional]
**frozen** | Option<**String**> | The cumulative frozen balance | [optional]
**locked_amount** | Option<**String**> | Funds in outgoing transactions that are not yet published to the network | [optional]
**staked** | Option<**String**> | Staked balance | [optional]
**total_staked_cpu** | Option<**f64**> | Deprecated | [optional]
**total_staked_network** | Option<**String**> | Deprecated | [optional]
**self_staked_cpu** | Option<**String**> | Deprecated | [optional]
**self_staked_network** | Option<**String**> | Deprecated | [optional]
**pending_refund_cpu** | Option<**String**> | Deprecated | [optional]
**pending_refund_network** | Option<**String**> | Deprecated | [optional]
**block_height** | Option<**String**> |  | [optional]
**block_hash** | Option<**String**> |  | [optional]
**rewards_info** | Option<[**models::RewardsInfo**](RewardsInfo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


