mod setup;
use {
    fireblocks_sdk::{models::TransactionStatus, WalletContainer, WalletType, ASSET_SOL_TEST},
    setup::{config, Config},
};

fn transfer_client(config: &Config) -> Option<fireblocks_sdk::Client> {
    if !config.is_ok() {
        return None;
    }
    let c = config.client();

    if !std::env::var("FIREBLOCKS_CREATE_TX").is_ok() {
        tracing::info!("skipping transfers test");
        return None;
    }

    return Some(c);
}

#[rstest::rstest]
#[tokio::test]
async fn test_transfer_internal(config: &Config) -> anyhow::Result<()> {
    let c = transfer_client(config);
    if c.is_none() {
        return Ok(());
    }
    let c = c.unwrap();
    let w: WalletContainer = c
        .wallet_name(WalletType::Internal, "test-whitelist")
        .await?
        .ok_or_else(|| {
            anyhow::format_err!(
                "internal wallet 'test-whitelist' is not found, please create and add SOL_TEST \
                 address"
            )
        })?;
    let response = c
        .vault_whitelist_transfer("0", ASSET_SOL_TEST, "0.00001", WalletType::Internal, &w.id)
        .await?;

    assert_eq!(response.status, TransactionStatus::Submitted);
    Ok(())
}
