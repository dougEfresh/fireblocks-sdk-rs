# TravelRuleCreateTransactionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**originator_vas_pdid** | Option<**String**> | The Decentralized Identifier (DID) of the exchange (VASP) that is sending the virtual assets. This identifier is unique to the exchange and is generated when the exchange's account is  created in the Notabene network. | [optional]
**beneficiary_vas_pdid** | Option<**String**> | The Decentralized Identifier (DID) of the exchange (VASP) that is receiving the virtual assets. This identifier is unique to the exchange and is generated when the exchange's account is  created in the Notabene network. | [optional]
**originator_vas_pname** | Option<**String**> | The name of the VASP acting as the transaction originator. | [optional]
**beneficiary_vas_pname** | Option<**String**> | The name of the VASP acting as the transaction beneficiary. | [optional]
**beneficiary_vas_pwebsite** | Option<**String**> | The website of the VASP acting as the transaction beneficiary. | [optional]
**transaction_blockchain_info** | Option<[**models::TravelRuleTransactionBlockchainInfo**](TravelRuleTransactionBlockchainInfo.md)> | Information about the blockchain transaction. | [optional]
**originator** | [**models::TravelRulePiiIvms**](TravelRulePiiIVMS.md) | Information about the originator of the transaction. | 
**beneficiary** | [**models::TravelRulePiiIvms**](TravelRulePiiIVMS.md) | Information about the beneficiary of the transaction. | 
**encrypted** | Option<**String**> | Encrypted data related to the transaction. | [optional]
**protocol** | Option<**String**> | The protocol used to perform the travel rule. | [optional]
**skip_beneficiary_data_validation** | Option<**bool**> | Whether to skip validation of beneficiary data. | [optional]
**travel_rule_behavior** | Option<**bool**> | Whether to check if the transaction complies with the travel rule in the beneficiary VASP's jurisdiction. | [optional]
**originator_ref** | Option<**String**> | A reference ID related to the originator of the transaction. | [optional]
**beneficiary_ref** | Option<**String**> | A reference ID related to the beneficiary of the transaction. | [optional]
**travel_rule_behavior_ref** | Option<**String**> | A reference ID related to the travel rule behavior. | [optional]
**originator_proof** | Option<[**models::TravelRuleOwnershipProof**](TravelRuleOwnershipProof.md)> | Ownership proof related to the originator of the transaction. | [optional]
**beneficiary_proof** | Option<[**models::TravelRuleOwnershipProof**](TravelRuleOwnershipProof.md)> | Ownership proof related to the beneficiary of the transaction. | [optional]
**beneficiary_did** | Option<**String**> | The Decentralized Identifier (DID) of the person at the receiving exchange (VASP).  This identifier is generated when the customer is registered in the Notabene network,  or automatically created based on the `beneficiaryRef`.  - If neither `beneficiaryRef` nor `beneficiaryDid` is provided in the `txCreate` payload,    a new random DID is generated for every transaction. | [optional]
**originator_did** | Option<**String**> | The Decentralized Identifier (DID) of the person at the exchange (VASP) who is requesting the withdrawal. This identifier is generated when the customer is registered in the Notabene network or automatically created based on the `originatorRef`.  - If neither `originatorRef` nor `originatorDid` is provided in the `txCreate` payload,    a new random DID is generated for every transaction. | [optional]
**is_non_custodial** | Option<**bool**> | Indicates if the transaction involves a non-custodial wallet. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


