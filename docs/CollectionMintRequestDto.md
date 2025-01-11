# CollectionMintRequestDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vault_account_id** | **String** | The id of the vault account that initiates the mint function. | 
**to** | **String** | The EVM address to mint to  | 
**token_id** | **String** | The token id, recommended to have numerical format and in sequential order | 
**amount** | Option<**String**> | For ERC721, amount is optional or should always be 1 and for ERC1155, amount should be 1 or greater | [optional]
**metadata_uri** | Option<**String**> | URL of metadata uploaded, skip uploading to IPFS if this field is provided with any value | [optional]
**metadata** | Option<[**models::CollectionTokenMetadataDto**](CollectionTokenMetadataDto.md)> | Metadata to upload | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


