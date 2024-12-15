# ChainInfoResponseDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chain_descriptor** | **String** | The protocol identifier (e.g. \"ETH\"/\"SOL\"). | 
**current_epoch** | **f64** | The current epoch number of the blockchain network. | 
**epoch_elapsed** | **f64** | The percentage of time that has elapsed within the current epoch, represented as a decimal value between 0 and 1. | 
**epoch_duration** | **f64** | The total duration in milliseconds of a single epoch. | 
**additional_info** | [**models::AdditionalInfoDto**](AdditionalInfoDto.md) | Additional information related to the blockchain. This may include extra details about the blockchain network. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


