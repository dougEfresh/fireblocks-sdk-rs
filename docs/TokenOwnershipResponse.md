# TokenOwnershipResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The Fireblocks NFT asset id | 
**token_id** | **String** | Token id within the contract/collection | 
**standard** | **String** | Token's standard | 
**metadata_uri** | Option<**String**> | URL of the original token JSON metadata | [optional]
**cached_metadata_uri** | Option<**String**> | URL of the cached token JSON metadata | [optional]
**media** | Option<[**Vec<models::MediaEntityResponse>**](MediaEntityResponse.md)> | Media items extracted from metadata JSON | [optional]
**spam** | Option<[**models::SpamOwnershipResponse**](SpamOwnershipResponse.md)> | Owned Token's Spam status | [optional]
**collection** | Option<[**models::TokenCollectionResponse**](TokenCollectionResponse.md)> | Parent collection information | [optional]
**balance** | **String** | Token's balance | 
**vault_account_id** | Option<**String**> | Fireblocks Vault Account ID | [optional]
**ownership_start_time** | **f64** | Ownership start timestamp (epoch) | 
**ownership_last_update_time** | **f64** | Ownership start last update time (epoch) | 
**blockchain_descriptor** | **String** | Token's blockchain | 
**description** | Option<**String**> | Token's metadata description | [optional]
**name** | Option<**String**> | Token's name | [optional]
**ncw_id** | Option<**String**> | Ownership Non-Custodial Wallet ID | [optional]
**ncw_account_id** | Option<**String**> | Ownership Non-Custodial Wallet's account ID | [optional]
**status** | **String** | Owned Token's status | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


