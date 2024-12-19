/*
 * Fireblocks API
 *
 * Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com) 
 *
 * The version of the OpenAPI document: 1.8.0
 * Contact: developers@fireblocks.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TravelRuleCreateTransactionRequest {
    /// The VASP ID of the transaction originator
    #[serde(rename = "originatorVASPdid", skip_serializing_if = "Option::is_none")]
    pub originator_vas_pdid: Option<String>,
    /// The VASP ID of the transaction beneficiary
    #[serde(rename = "beneficiaryVASPdid", skip_serializing_if = "Option::is_none")]
    pub beneficiary_vas_pdid: Option<String>,
    /// The name of the VASP acting as the beneficiary
    #[serde(rename = "beneficiaryVASPname", skip_serializing_if = "Option::is_none")]
    pub beneficiary_vas_pname: Option<String>,
    /// Information about the blockchain transaction
    #[serde(rename = "transactionBlockchainInfo", skip_serializing_if = "Option::is_none")]
    pub transaction_blockchain_info: Option<models::TravelRuleTransactionBlockchainInfo>,
    /// Information about the originator of the transaction
    #[serde(rename = "originator")]
    pub originator: models::TravelRulePiiIvms,
    /// Information about the beneficiary of the transaction
    #[serde(rename = "beneficiary")]
    pub beneficiary: models::TravelRulePiiIvms,
    /// Encrypted data related to the transaction
    #[serde(rename = "encrypted", skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<String>,
    /// The protocol used to perform the travel rule
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// Whether to skip validation of beneficiary data
    #[serde(rename = "skipBeneficiaryDataValidation", skip_serializing_if = "Option::is_none")]
    pub skip_beneficiary_data_validation: Option<bool>,
    /// Whether to check if the transaction is a TRAVEL_RULE in the beneficiary VASP's jurisdiction
    #[serde(rename = "travelRuleBehavior", skip_serializing_if = "Option::is_none")]
    pub travel_rule_behavior: Option<bool>,
    /// Ownership proof related to the originator of the transaction
    #[serde(rename = "originatorProof", skip_serializing_if = "Option::is_none")]
    pub originator_proof: Option<models::TravelRuleOwnershipProof>,
    /// Ownership proof related to the beneficiary of the transaction
    #[serde(rename = "beneficiaryProof", skip_serializing_if = "Option::is_none")]
    pub beneficiary_proof: Option<models::TravelRuleOwnershipProof>,
    /// Personal identifiable information related to the transaction
    #[serde(rename = "pii", skip_serializing_if = "Option::is_none")]
    pub pii: Option<models::TravelRulePiiIvms>,
}

impl TravelRuleCreateTransactionRequest {
    pub fn new(originator: models::TravelRulePiiIvms, beneficiary: models::TravelRulePiiIvms) -> TravelRuleCreateTransactionRequest {
        TravelRuleCreateTransactionRequest {
            originator_vas_pdid: None,
            beneficiary_vas_pdid: None,
            beneficiary_vas_pname: None,
            transaction_blockchain_info: None,
            originator,
            beneficiary,
            encrypted: None,
            protocol: None,
            skip_beneficiary_data_validation: None,
            travel_rule_behavior: None,
            originator_proof: None,
            beneficiary_proof: None,
            pii: None,
        }
    }
}

