#![doc = include_str!("../README.md")]
use {
    chrono::{DateTime, Utc},
    std::fmt::Display,
};
mod assets;
mod client;
pub mod error;
pub(crate) mod jwt;
mod log;
#[cfg(feature = "page")]
mod paged_client;
mod wallet;
#[cfg(feature = "page")]
pub use paged_client::{PagedClient, TransactionStream, VaultStream};
pub use {
    crate::error::*,
    apis::{
        configuration::{ApiKey, Configuration},
        ApiClient,
    },
    assets::{
        Asset,
        ASSET_BTC,
        ASSET_BTC_TEST,
        ASSET_DOGE,
        ASSET_DOGE_TEST,
        ASSET_ETH,
        ASSET_ETH_TEST,
        ASSET_SOL,
        ASSET_SOL_TEST,
    },
    client::{Client, ClientBuilder},
    wallet::WalletContainer,
};

pub const FIREBLOCKS_API: &str = "https://api.fireblocks.io/v1";
pub const FIREBLOCKS_SANDBOX_API: &str = "https://sandbox-api.fireblocks.io/v1";
pub type Epoch = DateTime<Utc>;
pub type Result<T> = std::result::Result<T, FireblocksError>;
pub type QueryParams = Vec<(String, String)>;

#[derive(Debug, Clone, Copy)]
pub enum WalletType {
    Internal,
    External,
    Contract,
}

impl Display for WalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match &self {
            Self::Internal => "Internal",
            Self::External => "External",
            Self::Contract => "Contract",
        })
    }
}

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
pub mod apis;
#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
pub mod models;
