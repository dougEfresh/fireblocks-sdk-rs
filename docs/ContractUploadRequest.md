# ContractUploadRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the contract template | 
**description** | **String** | A short description of the contract template | 
**long_description** | Option<**String**> | A full description of the contract template. May contain   to break the lines | [optional]
**bytecode** | **String** | The compiled artifact of this smart contract. Used for deployment of this contract template | 
**sourcecode** | Option<**String**> | The source code of the contract. Optional. | [optional]
**r#type** | Option<**String**> | The type of the contract template | [optional]
**docs** | Option<[**models::ContractDoc**](ContractDoc.md)> | A `natspec` compliant documentation json. Can be retrieved from the output json after compilation | [optional]
**abi** | [**Vec<Vec<models::AbiFunction>>**](Vec.md) |  | 
**attributes** | Option<[**models::ContractAttributes**](ContractAttributes.md)> | The attributes related to this contract template. It will be displayed in the tokenization page | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


