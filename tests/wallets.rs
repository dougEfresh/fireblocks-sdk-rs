mod setup;
use {
    fireblocks_sdk::{
        apis::{
            whitelisted_external_wallets_api::GetExternalWalletParams,
            whitelisted_internal_wallets_api::GetInternalWalletParams,
        },
        WalletType,
    },
    setup::{config, dummy_name, Config},
};

#[rstest::rstest]
#[tokio::test]
async fn test_wallet_whitelist_external(config: &Config) -> anyhow::Result<()> {
    if !config.is_ok() {
        return Ok(());
    }
    let c = config.client();
    let name = dummy_name(Some("ext"));
    let id = c
        .wallet_create(fireblocks_sdk::WalletType::External, &name)
        .await?;
    assert!(!id.is_empty());
    let api = c.wallet_external_api();
    let w = api
        .get_external_wallet(GetExternalWalletParams {
            wallet_id: id.clone(),
        })
        .await?;
    assert!(!w.id.is_empty());
    assert_eq!(&w.id, &id);
    let found = c
        .wallets(WalletType::External)
        .await?
        .into_iter()
        .find(|w| w.id == id);
    assert!(found.is_some());
    c.wallet_delete(WalletType::External, &id).await?;
    Ok(())
}

#[rstest::rstest]
#[tokio::test]
async fn test_wallet_whitelist_internal(config: &Config) -> anyhow::Result<()> {
    if !config.is_ok() {
        return Ok(());
    }
    let c = config.client();
    let name = dummy_name(Some("int"));
    let id = c.wallet_create(WalletType::Internal, &name).await?;
    assert!(!id.is_empty());

    let w = c
        .wallet_internal_api()
        .get_internal_wallet(GetInternalWalletParams {
            wallet_id: id.clone(),
        })
        .await?;
    assert!(w.id.is_some());
    assert_eq!(&w.id.unwrap_or_default(), &id);
    let found = c
        .wallets(WalletType::Internal)
        .await?
        .into_iter()
        .find(|w| w.id == id);
    assert!(found.is_some());

    c.wallet_delete(WalletType::Internal, &id).await?;
    Ok(())
}
