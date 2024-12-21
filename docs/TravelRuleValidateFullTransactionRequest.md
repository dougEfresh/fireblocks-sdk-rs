# TravelRuleValidateFullTransactionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_asset** | Option<**String**> | The asset involved in the transaction | [optional]
**transaction_amount** | Option<**String**> | The amount of the transaction | [optional]
**originator_did** | Option<**String**> | The DID of the transaction originator | [optional]
**beneficiary_did** | Option<**String**> | The DID of the transaction beneficiary | [optional]
**originator_vas_pdid** | Option<**String**> | The VASP ID of the transaction originator | [optional]
**beneficiary_vas_pdid** | Option<**String**> | The VASP ID of the transaction beneficiary | [optional]
**beneficiary_vas_pname** | Option<**String**> | The name of the VASP acting as the beneficiary | [optional]
**transaction_blockchain_info** | Option<[**models::TravelRuleTransactionBlockchainInfo**](TravelRuleTransactionBlockchainInfo.md)> | Information about the blockchain transaction | [optional]
**originator** | [**models::TravelRulePiiIvms**](TravelRulePiiIVMS.md) | Information about the originator of the transaction | 
**beneficiary** | [**models::TravelRulePiiIvms**](TravelRulePiiIVMS.md) | Information about the beneficiary of the transaction | 
**encrypted** | Option<**String**> | Encrypted data related to the transaction | [optional]
**protocol** | Option<**String**> | The protocol used to perform the travel rule | [optional]
**notification_email** | Option<**String**> | The email address where a notification should be sent upon completion of the travel rule | [optional]
**skip_beneficiary_data_validation** | Option<**bool**> | Whether to skip validation of beneficiary data | [optional]
**travel_rule_behavior** | Option<**bool**> | Whether to check if the transaction is a TRAVEL_RULE in the beneficiary VASP's jurisdiction | [optional]
**originator_proof** | Option<[**models::TravelRuleOwnershipProof**](TravelRuleOwnershipProof.md)> | Ownership proof related to the originator of the transaction | [optional]
**beneficiary_proof** | Option<[**models::TravelRuleOwnershipProof**](TravelRuleOwnershipProof.md)> | Ownership proof related to the beneficiary of the transaction | [optional]
**pii** | Option<[**models::TravelRulePiiIvms**](TravelRulePiiIVMS.md)> | Personal identifiable information related to the transaction | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


