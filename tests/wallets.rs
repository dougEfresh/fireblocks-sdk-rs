mod setup;
use fireblocks_sdk::apis::whitelisted_external_wallets_api::GetExternalWalletParams;
use fireblocks_sdk::apis::whitelisted_internal_wallets_api::GetInternalWalletParams;
use setup::{config, dummy_name, Config};

#[rstest::rstest]
#[tokio::test]
async fn test_wallet_whitelist_external(config: &Config) -> anyhow::Result<()> {
    if !config.is_ok() {
        return Ok(());
    }
    let c = config.client();
    let name = dummy_name();
    let id = c
        .wallet_create(fireblocks_sdk::WalletType::External, &name)
        .await?;
    assert!(!id.is_empty());
    let w = c
        .wallet_external_api()
        .get_external_wallet(GetExternalWalletParams {
            wallet_id: id.clone(),
        })
        .await?;
    assert!(!w.id.is_empty());
    assert_eq!(&w.id, &id);
    c.wallet_delete(fireblocks_sdk::WalletType::External, &id)
        .await?;
    Ok(())
}

#[rstest::rstest]
#[tokio::test]
async fn test_wallet_whitelist_internal(config: &Config) -> anyhow::Result<()> {
    if !config.is_ok() {
        return Ok(());
    }
    let c = config.client();
    let name = dummy_name();
    let id = c
        .wallet_create(fireblocks_sdk::WalletType::Internal, &name)
        .await?;
    assert!(!id.is_empty());

    let w = c
        .wallet_internal_api()
        .get_internal_wallet(GetInternalWalletParams {
            wallet_id: id.clone(),
        })
        .await?;
    assert!(w.id.is_some());
    assert_eq!(&w.id.unwrap_or_default(), &id);
    c.wallet_delete(fireblocks_sdk::WalletType::Internal, &id)
        .await?;
    Ok(())
}
