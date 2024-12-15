# TokenOwnershipResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The Fireblocks NFT asset id | 
**token_id** | **String** | Token id within the contract/collection | 
**standard** | **String** | ERC721 / ERC1155 | 
**metadata_uri** | Option<**String**> | URL of the original token JSON metadata | [optional]
**cached_metadata_uri** | Option<**String**> | URL of the cached token JSON metadata | [optional]
**media** | [**Vec<models::MediaEntityResponse>**](MediaEntityResponse.md) | Media items extracted from metadata JSON | 
**spam** | Option<[**models::SpamOwnershipResponse**](SpamOwnershipResponse.md)> | Owned Token's Spam status | [optional]
**collection** | Option<[**models::TokenCollectionResponse**](TokenCollectionResponse.md)> | Parent collection information | [optional]
**balance** | **String** |  | 
**vault_account_id** | **String** |  | 
**ownership_start_time** | **f64** |  | 
**ownership_last_update_time** | **f64** |  | 
**blockchain_descriptor** | **String** |  | 
**description** | **String** |  | 
**name** | **String** |  | 
**ncw_id** | Option<**String**> |  | [optional]
**ncw_account_id** | Option<**String**> |  | [optional]
**status** | **String** | Owned Token's status | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


