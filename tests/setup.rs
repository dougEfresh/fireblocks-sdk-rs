use std::sync::{Arc, Once, OnceLock};
use std::time::Duration;
use tokio::time;
use tokio_stream::StreamExt;
use tracing::warn;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;

static INIT: Once = Once::new();
pub static CLIENT: OnceLock<fireblocks_sdk::Client> = OnceLock::new();

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
        let key: Option<String> = std::env::var("FIREBLOCKS_SECRET").ok();
        if api_key.is_none() || key.is_none() {
            return;
        }
        let api_key = api_key.unwrap();
        let rsa_pem = key.unwrap().as_bytes().to_vec();
        if let Ok(c) = fireblocks_sdk::ClientBuilder::new(&api_key, &rsa_pem)
            .use_sandbox()
            .build()
        {
            let _ = CLIENT.set(c);
        }
    });
}
