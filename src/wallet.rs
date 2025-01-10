use {
    crate::{
        assets::Asset,
        models::{
            TransferPeerPathType,
            UnmanagedExternalWallet,
            UnmanagedWallet,
            WalletAsset,
            WalletAssetExternal,
        },
        WalletType,
    },
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct WalletContainer {
    pub id: String,
    pub name: String,
    pub assets: Vec<WalletAssetExternal>,
    pub customer_ref_id: Option<String>,
}

impl From<UnmanagedWallet> for WalletContainer {
    fn from(value: UnmanagedWallet) -> Self {
        Self {
            id: value.id.unwrap_or_default(),
            name: value.name.unwrap_or_default(),
            customer_ref_id: value.customer_ref_id,
            assets: value
                .assets
                .unwrap_or_default()
                .into_iter()
                .map(|w| WalletAssetExternal {
                    id: w.id.unwrap_or_default(),
                    locked_amount: w.locked_amount,
                    address: w.address.unwrap_or_default(),
                    tag: w.tag,
                    activation_time: w.activation_time,
                    status: w.status.unwrap_or_default(),
                })
                .collect(),
        }
    }
}

impl From<UnmanagedExternalWallet> for WalletContainer {
    fn from(value: UnmanagedExternalWallet) -> Self {
        Self {
            id: value.id,
            name: value.name,
            customer_ref_id: value.customer_ref_id,
            assets: value.assets,
        }
    }
}
