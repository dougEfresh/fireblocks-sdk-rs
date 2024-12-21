# ContractWithAbiDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**contract_address** | **String** | The address of the contract | 
**base_asset_id** | **String** | The blockchain base assetId | 
**name** | **String** | The name of the contract | 
**abi** | [**Vec<models::AbiFunction>**](AbiFunction.md) | The ABI of the contract | 
**is_proxy** | Option<**bool**> | Whether the contract is a proxy contract | [optional]
**implementation** | Option<**String**> | The implementation contract address | [optional]
**is_public** | **bool** | Whether the contract ABI is public | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


