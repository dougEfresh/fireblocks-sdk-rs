# UnsignedMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pre_hash** | Option<[**models::PreHash**](PreHash.md)> |  | [optional]
**content** | **String** | Content to sign on. - EIP-191: Requires a 32 byte-long string for ECDSA (hash of the actual message to sign) or any length for EdDSA, as prehashing is not required. - EIP-712: Requires an object specifying the structured data format, including `types`, `domain`, `primaryType`, and `message`.  | 
**bip44address_index** | Option<**i32**> | BIP44 address index | [optional]
**bip44change** | Option<**f64**> | BIP44 change index | [optional]
**derivation_path** | Option<**Vec<f64>**> | BIP44 full derivation path | [optional]
**r#type** | Option<**String**> | Typed Message Signing - message type.  - EIP191 & EIP712: for ETH and all EVM based assets typed message signing - TIP191: For Tron (TRX) typed message signing - BTC_MESSAGE: For Bitcoin (BTC) typed message signing  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


