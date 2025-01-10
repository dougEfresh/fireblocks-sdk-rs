use {
    fireblocks_sdk::Client,
    std::{
        sync::{Once, OnceLock},
        time::Duration,
    },
    tracing::warn,
    tracing_subscriber::{fmt::format::FmtSpan, EnvFilter},
};

static INIT: Once = Once::new();
pub static CLIENT: OnceLock<Client> = OnceLock::new();

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
            .with_sandbox()
            .with_user_agent("fireblocks-rs-sdk-test")
            .with_timeout(Duration::from_secs(15))
            .build()
        {
            let _ = CLIENT.set(c);
        }
    });
}

#[rstest::fixture]
#[once]
pub fn config() -> Config {
    setup();
    Config::new()
}

pub struct Config {
    client: Option<Client>,
    #[allow(dead_code)]
    create_tx: bool,
}

impl Config {
    fn new() -> Self {
        let create_tx = std::env::var("FIREBLOCKS_CREATE_TX").ok().is_some();
        Self {
            client: CLIENT.get().cloned(),
            create_tx,
        }
    }

    #[allow(dead_code)]
    pub const fn is_ok(&self) -> bool {
        self.client.is_some()
    }

    #[allow(clippy::unwrap_used, dead_code, clippy::missing_panics_doc)]
    pub fn client(&self) -> Client {
        self.client.as_ref().unwrap().clone()
    }
}

#[allow(dead_code)]
pub fn dummy_name(extra: Option<&str>) -> String {
    let name = format!("z-test-{}", chrono::Utc::now().timestamp_millis());
    format!("{}{}", name, extra.unwrap_or_default())
}
