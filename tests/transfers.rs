mod setup;
use {
    fireblocks_sdk::{
        ASSET_SOL_TEST,
        Client,
        TransactionStatus,
        WalletContainer,
        WalletType,
        models::CreateTransactionResponse,
    },
    setup::{Config, config},
    std::time::Duration,
    tracing::info,
};

fn transfer_client(config: &Config) -> Option<fireblocks_sdk::Client> {
    let c = config.client();
    if !config.create_tx() {
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
        .vault_whitelist_transfer("0", ASSET_SOL_TEST, "0.00001", wallet_type, &w.id, None)
        .await?;

    let status: TransactionStatus = response
        .status
        .clone()
        .unwrap_or_else(|| String::from("unknown"))
        .into();
    assert_eq!(status, TransactionStatus::Submitted);
    Ok(response)
}

#[rstest::rstest]
#[tokio::test]
async fn test_transfer_internal(config: Config) -> anyhow::Result<()> {
    let c = transfer_client(&config);
    if c.is_none() {
        return Ok(());
    }
    transfer_whitelist(c.expect("client should be defined"), WalletType::Internal).await?;
    Ok(())
}

#[rstest::rstest]
#[tokio::test]
async fn test_transfer_external(config: Config) -> anyhow::Result<()> {
    let c = transfer_client(&config);
    if c.is_none() {
        return Ok(());
    }
    transfer_whitelist(c.expect("client should be defined"), WalletType::External).await?;
    Ok(())
}

#[rstest::rstest]
#[tokio::test]
async fn test_transfer_poll(config: Config) -> anyhow::Result<()> {
    let c = transfer_client(&config);
    if c.is_none() {
        return Ok(());
    }
    let c = c.expect("client should be defined");
    let resp = transfer_whitelist(c.clone(), WalletType::External).await?;
    assert!(resp.id.is_some());
    let id = resp.id.unwrap_or_default();
    c.poll_transaction(&id, Duration::from_secs(120), Duration::from_secs(5), |t| {
        info!("transaction status {} ({})", t.status, t.id);
    })
    .await?;
    Ok(())
}
