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

/// TransactionOperation : * `TRANSFER` - The default value for an operation. Transfers funds from one account to another. UTXO blockchains allow multi-input and multi-output transfers. All other blockchains allow transfers with one source address and one destination address. * `MINT` - Mints new tokens. Supported for Stellar, Ripple and EVM-based blockchains. * `BURN` - Burns tokens. Supported for Stellar, Ripple and EVM-based blockchains. * `CONTRACT_CALL` - Calls a smart contract method for web3 operations on any EVM blockchain. The Fireblocks [development libraries](https://developers.fireblocks.com/docs/ethereum-development#convenience-libraries) are recommended for building contract call transactions. * `TYPED_MESSAGE` - An off-chain message in either Ethereum Personal Message or EIP712 format. Use it to sign specific readable messages that are not actual transactions. [Learn more about typed messages](https://developers.fireblocks.com/docs/typed-message-signing-1). * `RAW` - An off-chain message with no predefined format. Use it to sign any message with your private key, including protocols such as blockchains and custom transaction types that are not natively supported by Fireblocks. [Learn more about raw signing transactions.](https://developers.fireblocks.com/docs/raw-signing)
/// * `TRANSFER` - The default value for an operation. Transfers funds from one account to another. UTXO blockchains allow multi-input and multi-output transfers. All other blockchains allow transfers with one source address and one destination address. * `MINT` - Mints new tokens. Supported for Stellar, Ripple and EVM-based blockchains. * `BURN` - Burns tokens. Supported for Stellar, Ripple and EVM-based blockchains. * `CONTRACT_CALL` - Calls a smart contract method for web3 operations on any EVM blockchain. The Fireblocks [development libraries](https://developers.fireblocks.com/docs/ethereum-development#convenience-libraries) are recommended for building contract call transactions. * `TYPED_MESSAGE` - An off-chain message in either Ethereum Personal Message or EIP712 format. Use it to sign specific readable messages that are not actual transactions. [Learn more about typed messages](https://developers.fireblocks.com/docs/typed-message-signing-1). * `RAW` - An off-chain message with no predefined format. Use it to sign any message with your private key, including protocols such as blockchains and custom transaction types that are not natively supported by Fireblocks. [Learn more about raw signing transactions.](https://developers.fireblocks.com/docs/raw-signing)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionOperation {
    #[serde(rename = "TRANSFER")]
    Transfer,
    #[serde(rename = "BURN")]
    Burn,
    #[serde(rename = "CONTRACT_CALL")]
    ContractCall,
    #[serde(rename = "MINT")]
    Mint,
    #[serde(rename = "RAW")]
    Raw,
    #[serde(rename = "TYPED_MESSAGE")]
    TypedMessage,
}

impl std::fmt::Display for TransactionOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Transfer => write!(f, "TRANSFER"),
            Self::Burn => write!(f, "BURN"),
            Self::ContractCall => write!(f, "CONTRACT_CALL"),
            Self::Mint => write!(f, "MINT"),
            Self::Raw => write!(f, "RAW"),
            Self::TypedMessage => write!(f, "TYPED_MESSAGE"),
        }
    }
}

impl Default for TransactionOperation {
    fn default() -> TransactionOperation {
        Self::Transfer
    }
}
