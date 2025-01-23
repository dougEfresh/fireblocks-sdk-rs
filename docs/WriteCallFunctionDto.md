# WriteCallFunctionDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vault_account_id** | **String** | The vault account id this contract was deploy from | 
**abi_function** | [**models::WriteAbiFunction**](WriteAbiFunction.md) |  | 
**amount** | Option<**String**> | Amount in base asset. Being used in payable functions | [optional]
**fee_level** | Option<**String**> | Fee level for the write function transaction. interchangeable with the 'fee' field | [optional]
**fee** | Option<**String**> | Max fee amount for the write function transaction. interchangeable with the 'feeLevel' field | [optional]
**note** | Option<**String**> | Custom note, not sent to the blockchain, that describes the transaction at your Fireblocks workspace | [optional]
**use_gasless** | Option<**bool**> | Indicates whether the token should be created in a gasless manner, utilizing the ERC-2771 standard. When set to true, the transaction will be relayed by a designated relayer. The workspace must be configured to use Fireblocks gasless relay. | [optional]
**external_id** | Option<**String**> | External id that can be used to identify the transaction in your system. The unique identifier of the transaction outside of Fireblocks with max length of 255 characters | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


