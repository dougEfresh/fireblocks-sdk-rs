use {
    fireblocks_sdk::Client,
    std::{sync::Once, time::Duration},
    tracing_subscriber::{fmt::format::FmtSpan, EnvFilter},
};

static INIT: Once = Once::new();

#[allow(clippy::unwrap_used, clippy::missing_panics_doc)]
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
            tracing::debug!("no .env file");
        }
    });
}

#[rstest::fixture]
//#[once]
pub fn config() -> Config {
    setup();
    Config::new()
}

pub struct Config {
    client: Client,
    create_tx: bool,
    create_vault: bool,
    create_webhook: bool,
}

impl Config {
    fn new() -> Self {
        let create_tx = std::env::var("FIREBLOCKS_CREATE_TX").ok().is_some();
        let create_vault = std::env::var("FIREBLOCKS_CREATE_VAULT").ok().is_some();
        let create_webhook = std::env::var("FIREBLOCKS_CREATE_WEBHOOK").ok().is_some();
        let api_key: String =
            std::env::var("FIREBLOCKS_API_KEY").expect("FIREBLOCKS_API_KEY is not set");
        let key: String = std::env::var("FIREBLOCKS_SECRET").expect("FIREBLOCKS_SECRET is not set");
        let rsa_pem = key.as_bytes().to_vec();
        let client = fireblocks_sdk::ClientBuilder::new(&api_key, &rsa_pem)
            .use_sandbox()
            .with_sandbox()
            .with_user_agent("fireblocks-rs-sdk-test")
            .with_timeout(Duration::from_secs(15))
            .build()
            .expect("failed to configure client. Is .env configured properly?");

        Self {
            client,
            create_tx,
            create_vault,
            create_webhook,
        }
    }
    #[allow(dead_code)]
    pub const fn create_vault(&self) -> bool {
        self.create_vault
    }

    #[allow(dead_code)]
    pub const fn create_webhook(&self) -> bool {
        self.create_webhook
    }

    #[allow(dead_code)]
    pub const fn create_tx(&self) -> bool {
        self.create_tx
    }

    #[allow(dead_code)]
    pub fn client(&self) -> Client {
        self.client.clone()
    }
}

#[allow(dead_code)]
pub fn dummy_name(extra: Option<&str>) -> String {
    let name = format!("z-test-{}", chrono::Utc::now().timestamp_millis());
    format!("{}{}", name, extra.unwrap_or_default())
}
