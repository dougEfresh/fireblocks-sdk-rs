mod setup;
use {
    fireblocks_sdk::{
        models::{CreateTransactionResponse, TransactionStatus},
        Client,
        WalletContainer,
        WalletType,
        ASSET_SOL_TEST,
    },
    setup::{config, Config},
    std::time::Duration,
    tracing::info,
};

fn transfer_client(config: &Config) -> Option<fireblocks_sdk::Client> {
    if !config.is_ok() {
        return None;
    }
    let c = config.client();

    if std::env::var("FIREBLOCKS_CREATE_TX").is_err() {
        tracing::info!("skipping transfers test");
        return None;
    }

    Some(c)
}

async fn transfer_whitelist(
    c: Client,
    wallet_type: WalletType,
) -> anyhow::Result<CreateTransactionResponse> {
    let wallet_name = match wallet_type {
        WalletType::Internal => "test-whitelist",
        WalletType::External => "test-ext-whitelist",
        WalletType::Contract => "test-contract-whitelist",
    };
    let w: WalletContainer = c
        .wallet_by_name(wallet_type, wallet_name)
        .await?
        .ok_or_else(|| {
            anyhow::format_err!(
                "{} wallet '{}' is not found, please create and add SOL_TEST address",
                wallet_type,
                wallet_name
            )
        })?;
    let response = c
        .vault_whitelist_transfer("0", ASSET_SOL_TEST, "0.00001", WalletType::Internal, &w.id)
        .await?;

    assert_eq!(response.status, TransactionStatus::Submitted);
    Ok(response)
}

#[rstest::rstest]
#[tokio::test]
async fn test_transfer_internal(config: &Config) -> anyhow::Result<()> {
    let c = transfer_client(config);
    if c.is_none() {
        return Ok(());
    }
    transfer_whitelist(c.expect("client should be defined"), WalletType::Internal).await?;
    Ok(())
}

#[rstest::rstest]
#[tokio::test]
async fn test_transfer_external(config: &Config) -> anyhow::Result<()> {
    let c = transfer_client(config);
    if c.is_none() {
        return Ok(());
    }
    transfer_whitelist(c.expect("client should be defined"), WalletType::External).await?;
    Ok(())
}

#[rstest::rstest]
#[tokio::test]
async fn test_transfer_poll(config: &Config) -> anyhow::Result<()> {
    let c = transfer_client(config);
    if c.is_none() {
        return Ok(());
    }
    let c = c.expect("client should be defined");
    let resp = transfer_whitelist(c.clone(), WalletType::External).await?;
    c.poll_transaction(
        &resp.id,
        Duration::from_secs(120),
        Duration::from_secs(5),
        |t| info!("transaction status {} ({})", t.status, t.id),
    )
    .await?;
    Ok(())
}
