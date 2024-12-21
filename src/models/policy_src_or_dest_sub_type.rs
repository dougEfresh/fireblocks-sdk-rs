// Fireblocks API
//
// Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com)
//
// The version of the OpenAPI document: 1.8.0
// Contact: developers@fireblocks.com
// Generated by: https://openapi-generator.tech

use {
    crate::models,
    serde::{Deserialize, Serialize},
};

/// PolicySrcOrDestSubType : * EXTERNAL - A whitelisted wallet assigned as
/// external is typically used for addresses managed by your clients and
/// counterparties * INTERNAL - A whitelisted wallet assigned as internal, is
/// typically used for addresses that you control outside of your Fireblocks
/// workspace * CONTRACT - A whitelisted wallet assigned as contract is for
/// identifying and managing external smart contracts * EXCHANGETEST - Exchanges
/// which operate only on testnet assets * \"*\" - All subtypes
/// * EXTERNAL - A whitelisted wallet assigned as external is typically used for
///   addresses managed by your clients and counterparties * INTERNAL - A
///   whitelisted wallet assigned as internal, is typically used for addresses
///   that you control outside of your Fireblocks workspace * CONTRACT - A
///   whitelisted wallet assigned as contract is for identifying and managing
///   external smart contracts * EXCHANGETEST - Exchanges which operate only on
///   testnet assets * \"*\" - All subtypes
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PolicySrcOrDestSubType {
    #[serde(rename = "EXTERNAL")]
    External,
    #[serde(rename = "INTERNAL")]
    Internal,
    #[serde(rename = "CONTRACT")]
    Contract,
    #[serde(rename = "EXCHANGETEST")]
    Exchangetest,
    #[serde(rename = "*")]
    Star,
}

impl std::fmt::Display for PolicySrcOrDestSubType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::External => write!(f, "EXTERNAL"),
            Self::Internal => write!(f, "INTERNAL"),
            Self::Contract => write!(f, "CONTRACT"),
            Self::Exchangetest => write!(f, "EXCHANGETEST"),
            Self::Star => write!(f, "*"),
        }
    }
}

impl Default for PolicySrcOrDestSubType {
    fn default() -> PolicySrcOrDestSubType {
        Self::External
    }
}
