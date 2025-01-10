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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionDto {
    /// Id of the connection
    #[serde(rename = "id")]
    pub id: String,
    /// Id of the user that created the connection
    #[serde(rename = "userId")]
    pub user_id: String,
    /// Metadata of the connection (provided by the dapp)
    #[serde(rename = "sessionMetadata")]
    pub session_metadata: models::SessionMetadata,
    /// The vault to connect
    #[serde(rename = "vaultAccountId")]
    pub vault_account_id: f64,
    /// The default fee level
    #[serde(rename = "feeLevel")]
    pub fee_level: FeeLevel,
    /// The chains approved for the connection
    #[serde(rename = "chainIds")]
    pub chain_ids: Vec<String>,
    /// The connection's type
    #[serde(rename = "connectionType")]
    pub connection_type: ConnectionType,
    /// The method through which the connection was established
    #[serde(rename = "connectionMethod")]
    pub connection_method: ConnectionMethod,
    /// Timestamp of the session's creation
    #[serde(rename = "creationDate")]
    pub creation_date: String,
}

impl SessionDto {
    pub fn new(
        id: String,
        user_id: String,
        session_metadata: models::SessionMetadata,
        vault_account_id: f64,
        fee_level: FeeLevel,
        chain_ids: Vec<String>,
        connection_type: ConnectionType,
        connection_method: ConnectionMethod,
        creation_date: String,
    ) -> SessionDto {
        SessionDto {
            id,
            user_id,
            session_metadata,
            vault_account_id,
            fee_level,
            chain_ids,
            connection_type,
            connection_method,
            creation_date,
        }
    }
}
/// The default fee level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FeeLevel {
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "HIGH")]
    High,
}

impl Default for FeeLevel {
    fn default() -> FeeLevel {
        Self::Medium
    }
}
/// The connection's type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConnectionType {
    #[serde(rename = "WalletConnect")]
    WalletConnect,
}

impl Default for ConnectionType {
    fn default() -> ConnectionType {
        Self::WalletConnect
    }
}
/// The method through which the connection was established
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConnectionMethod {
    #[serde(rename = "DESKTOP")]
    Desktop,
    #[serde(rename = "MOBILE")]
    Mobile,
    #[serde(rename = "API")]
    Api,
}

impl Default for ConnectionMethod {
    fn default() -> ConnectionMethod {
        Self::Desktop
    }
}