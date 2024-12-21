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

/// NodeControls : Configure special node requirements.  For routing
/// transactions to a custom node please set the `type` to `NODE_ROUTER` and the
/// `tag` to the pre-configured tag value. For MEV protection, set only the
/// `type` property to `MEV` (`tag` is not required at this stage)  * Note: This
/// is a premium feature that should be enabled in your workspace.   Please
/// contract your Customer Success Manager/Fireblocks Support for more info.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeControls {
    /// `NODE_ROUTER` - used for transaction routing to a custom node `MEV` -
    /// used for transaction routing to a MEV protection provider
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Should be used when type is `NODE_ROUTER` only
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl NodeControls {
    /// Configure special node requirements.  For routing transactions to a
    /// custom node please set the `type` to `NODE_ROUTER` and the `tag` to the
    /// pre-configured tag value. For MEV protection, set only the `type`
    /// property to `MEV` (`tag` is not required at this stage)  * Note: This is
    /// a premium feature that should be enabled in your workspace.   Please
    /// contract your Customer Success Manager/Fireblocks Support for more info.
    pub fn new() -> NodeControls {
        NodeControls {
            r#type: None,
            tag: None,
        }
    }
}
/// `NODE_ROUTER` - used for transaction routing to a custom node `MEV` - used
/// for transaction routing to a MEV protection provider
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "MEV")]
    Mev,
    #[serde(rename = "NODE_ROUTER")]
    NodeRouter,
}

impl Default for Type {
    fn default() -> Type {
        Self::Mev
    }
}
