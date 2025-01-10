mod setup;
use {
    fireblocks_sdk::ASSET_BTC_TEST,
    setup::{config, Config},
};

#[rstest::rstest]
#[tokio::test]
#[allow(clippy::unwrap_used)]
async fn test_supported_assets(config: Config) -> anyhow::Result<()> {
    let c = config.client();
    let assets = c.supported_assets().await?;
    assert!(!assets.is_empty());
    let found = assets.iter().find(|a| a.id == ASSET_BTC_TEST.to_string());
    assert!(found.is_some());
    Ok(())
}
