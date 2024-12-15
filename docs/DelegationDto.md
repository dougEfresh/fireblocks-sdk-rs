# DelegationDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The unique identifier of the staking position | 
**vault_account_id** | **String** | The source vault account to stake from | 
**validator_name** | **String** | The destination validator address name | 
**provider_name** | **String** | The destination validator provider name | 
**chain_descriptor** | **String** | The protocol identifier (e.g. \"ETH\"/ \"SOL\") to use | 
**amount** | **String** | Amount of tokens to stake, measured in the staked asset unit. | 
**rewards_amount** | **String** | The amount staked in the position, measured in the staked asset unit. | 
**date_created** | **String** | When was the request made (ISO Date). | 
**status** | **String** | The current status. | 
**related_transactions** | [**Vec<models::RelatedTransactionDto>**](RelatedTransactionDto.md) | An array of transaction objects related to this position. Each object includes a 'txId' representing the transaction ID and a 'completed' boolean indicating if the transaction was completed. | 
**validator_address** | **String** | The destination address of the staking transaction. | 
**provider_id** | **String** | The unique identifier of the staking provider | 
**available_actions** | **Vec<String>** | An array of available actions that can be performed. for example, actions like \"unstake\" or \"withdraw\". | 
**in_progress** | **bool** | Indicates whether there is an ongoing action for this position (true if ongoing, false if not). | 
**in_progress_tx_id** | Option<**String**> | The transaction ID of the ongoing request | [optional]
**blockchain_position_info** | [**models::SolanaBlockchainDataDto**](SolanaBlockchainDataDto.md) | Additional fields per blockchain - can be empty or missing if not initialized or no additional info exists. The type depends on the chainDescriptor value. For Solana (SOL), stake account address. For Ethereum (ETH), an empty object is returned as no specific data is available. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


