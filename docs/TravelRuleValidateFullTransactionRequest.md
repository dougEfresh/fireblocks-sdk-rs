# TravelRuleValidateFullTransactionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_asset** | **String** | The asset involved in the transaction | 
**transaction_amount** | **String** | The amount of the transaction | 
**originator_did** | **String** | The DID of the transaction originator | 
**beneficiary_did** | **String** | The DID of the transaction beneficiary | 
**originator_vas_pdid** | **String** | The VASP ID of the transaction originator | 
**beneficiary_vas_pdid** | **String** | The VASP ID of the transaction beneficiary | 
**beneficiary_vas_pname** | **String** | The name of the VASP acting as the beneficiary | 
**transaction_blockchain_info** | [**models::TravelRuleTransactionBlockchainInfo**](TravelRuleTransactionBlockchainInfo.md) | Information about the blockchain transaction | 
**originator** | [**models::TravelRulePiiIvms**](TravelRulePiiIVMS.md) | Information about the originator of the transaction | 
**beneficiary** | [**models::TravelRulePiiIvms**](TravelRulePiiIVMS.md) | Information about the beneficiary of the transaction | 
**encrypted** | **String** | Encrypted data related to the transaction | 
**protocol** | **String** | The protocol used to perform the travel rule | 
**notification_email** | **String** | The email address where a notification should be sent upon completion of the travel rule | 
**skip_beneficiary_data_validation** | **bool** | Whether to skip validation of beneficiary data | 
**travel_rule_behavior** | **bool** | Whether to check if the transaction is a TRAVEL_RULE in the beneficiary VASP's jurisdiction | 
**originator_proof** | [**models::TravelRuleOwnershipProof**](TravelRuleOwnershipProof.md) | Ownership proof related to the originator of the transaction | 
**beneficiary_proof** | [**models::TravelRuleOwnershipProof**](TravelRuleOwnershipProof.md) | Ownership proof related to the beneficiary of the transaction | 
**pii** | [**models::TravelRulePiiIvms**](TravelRulePiiIVMS.md) | Personal identifiable information related to the transaction | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


