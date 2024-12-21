# WriteCallFunctionDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vault_account_id** | **String** | The vault account id this contract was deploy from | 
**abi_function** | [**Vec<models::WriteAbiFunction>**](WriteAbiFunction.md) | The abi of the read function you wish to call | 
**amount** | Option<**String**> | Amount in base asset. Being used in payable functions | [optional]
**fee_level** | Option<**String**> | Fee level for the write function transaction. interchangeable with the 'fee' field | [optional]
**fee** | Option<**String**> | Max fee amount for the write function transaction. interchangeable with the 'feeLevel' field | [optional]
**note** | Option<**String**> | Custom note, not sent to the blockchain, that describes the transaction at your Fireblocks workspace | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


