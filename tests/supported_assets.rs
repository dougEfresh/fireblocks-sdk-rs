mod setup;
use {
    fireblocks_sdk::{apis::Api, ASSET_BTC_TEST},
    setup::{config, Config},
};

#[rstest::rstest]
#[tokio::test]
#[allow(clippy::unwrap_used)]
async fn test_supported_assets(config: &Config) -> anyhow::Result<()> {
    if !config.is_ok() {
        return Ok(());
    }
    let c = config.client();
    let assets = c
        .apis()
        .blockchains_assets_api()
        .get_supported_assets()
        .await?;
    assert!(!assets.is_empty());
    let found = assets.iter().find(|a| a.id == ASSET_BTC_TEST.to_string());
    assert!(found.is_some());
    Ok(())
}
