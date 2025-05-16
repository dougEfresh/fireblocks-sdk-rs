# TravelRuleValidateFullTransactionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**originator_vas_pdid** | Option<**String**> | The Decentralized Identifier (DID) of the exchange (VASP) that is sending the virtual assets. This identifier is unique to the exchange and is generated when the exchange's account is  created in the Notabene network. | [optional]
**beneficiary_vas_pdid** | Option<**String**> | The Decentralized Identifier (DID) of the exchange (VASP) that is receiving the virtual assets. This identifier is unique to the exchange and is generated when the exchange's account is  created in the Notabene network. | [optional]
**transaction_asset** | Option<**String**> | Transaction asset symbol (e.g., BTC, ETH, USDC).  By using the `notation` query string, users can select the type of asset notation  - `fireblocks`: Converts asset symbols to Fireblocks notation.  - `notabene`: Retains the original Notabene asset symbol format.  | [optional]
**transaction_amount** | Option<**String**> | Transaction amount in the transaction asset. For example, if the asset is BTC, the amount  is the value in BTC units.  By using the `notation` query string, users can select the type of amount notation - `fireblocks`: Converts the amount to Fireblocks notation (e.g., adjusted for decimals). - `notabene`: Retains the original Notabene amount format.  | [optional]
**originator_vas_pname** | Option<**String**> | The name of the VASP acting as the transaction originator. | [optional]
**beneficiary_vas_pname** | Option<**String**> | The name of the VASP acting as the transaction beneficiary. | [optional]
**transaction_blockchain_info** | Option<[**models::TravelRuleTransactionBlockchainInfo**](TravelRuleTransactionBlockchainInfo.md)> | Information about the blockchain transaction. | [optional]
**originator** | [**models::TravelRuleValidatePiiIvms**](TravelRuleValidatePiiIVMS.md) | Information about the originator of the transaction. | 
**beneficiary** | [**models::TravelRuleValidatePiiIvms**](TravelRuleValidatePiiIVMS.md) | Information about the beneficiary of the transaction. | 
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
**notification_email** | Option<**String**> | The email address where a notification should be sent upon completion of the travel rule | [optional]
**pii** | Option<[**models::TravelRulePiiIvms**](TravelRulePiiIVMS.md)> | Personal identifiable information related to the transaction | [optional]
**pii_url** | Option<**String**> | The URL of the personal identifiable information related to the transaction | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


