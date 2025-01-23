# TransactionReceiptResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_hash** | **String** | The block hash | 
**block_number** | **i32** | The block number | 
**contract_address** | Option<**String**> | The address of deployed contract | [optional]
**cumulative_gas_used** | **i32** | The cumulative gas used in the transaction | 
**effective_gas_price** | **i32** | The effective gas price | 
**from** | **String** | Sender address | 
**gas_used** | **i32** | Gas used by the transaction | 
**logs** | [**Vec<models::TxLog>**](TxLog.md) | Array of transaction logs | 
**logs_bloom** | **String** | Logs bloom filter | 
**status** | **i32** | Transaction status (1 for success, 0 for failure) | 
**to** | Option<**String**> | Recipient address | [optional]
**transaction_hash** | **String** | The transaction hash | 
**transaction_index** | **i32** | Transaction index in the block | 
**r#type** | **String** | Type of transaction | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


