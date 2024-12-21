# VaultAsset

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**total** | **String** | The total wallet balance.   Total = available + pending + lockedAmount + frozen  - In EOS this value includes the network balance, self staking and pending refund.   - For all other coins it is the balance as it appears on the blockchain.  | 
**balance** | Option<**String**> | Deprecated - replaced by \"total\" | [optional]
**available** | **String** | Funds available for transfer. Equals: \"total\" minus \"lockedAmount\" minus \"frozen\" minus \"pending\"  | 
**pending** | **String** | The cumulative balance of all transactions pending to be cleared | 
**frozen** | **String** | The cumulative frozen balance | 
**locked_amount** | **String** | Funds in outgoing transactions that are not yet published to the network | 
**staked** | Option<**String**> | Deprecated | [optional]
**total_staked_cpu** | Option<**f64**> | Deprecated | [optional]
**total_staked_network** | Option<**String**> | Deprecated | [optional]
**self_staked_cpu** | Option<**String**> | Deprecated | [optional]
**self_staked_network** | Option<**String**> | Deprecated | [optional]
**pending_refund_cpu** | Option<**String**> | Deprecated | [optional]
**pending_refund_network** | Option<**String**> | Deprecated | [optional]
**block_height** | Option<**String**> | The height (number) of the block of the balance | [optional]
**block_hash** | Option<**String**> | The hash of the block of the balance | [optional]
**rewards_info** | Option<[**models::RewardsInfo**](RewardsInfo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


