use std::sync::{Arc, Once, OnceLock};
use std::time::Duration;
//  use crate::assets::{ASSET_BTC_TEST, ASSET_SOL_TEST};
//  use crate::paged_client::{PagedClient, TransactionStream};
//  use crate::types::*;
//  use crate::{Client, ClientBuilder, ASSET_ETH, ASSET_ETH_TEST, ASSET_SOL};
//  use bigdecimal::BigDecimal;
//  use chrono::{TimeZone, Utc};
//  use color_eyre::eyre::format_err;
use tokio::time;
use tokio_stream::StreamExt;
use tracing::warn;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;

static INIT: Once = Once::new();
static KEYS: OnceLock<(String, String)> = OnceLock::new();

#[allow(clippy::unwrap_used)]
pub fn setup() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_target(true)
            .with_level(true)
            .with_span_events(FmtSpan::CLOSE)
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        let env = dotenvy::dotenv();
        if env.is_err() {
            warn!("no .env file");
        }

        let api_key: Option<String> = std::env::var("FIREBLOCKS_API_KEY").ok();
        let path: Option<String> = std::env::var("FIREBLOCKS_SECRET").ok();
        if api_key.is_none() || path.is_none() {
            return;
        }
        let _ = KEYS.set((api_key.unwrap(), path.unwrap()));
    });
}
