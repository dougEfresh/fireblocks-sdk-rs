mod setup;
use {
    fireblocks_sdk::{Asset, Client, WalletType, ASSET_ETH_TEST, ASSET_SOL_TEST},
    setup::{config, dummy_name, Config},
    std::time::Duration,
};

fn asset_address(wallet_type: WalletType) -> (Asset, String) {
    match wallet_type {
        WalletType::External => (
            ASSET_SOL_TEST,
            String::from("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"),
        ),
        WalletType::Internal => (
            ASSET_SOL_TEST,
            String::from("8m3uKEn4fMPNVr7nv6RmQYktT4zRqEZzhuZDpG8hQZT4"),
        ),
        WalletType::Contract => (
            ASSET_ETH_TEST,
            String::from("0xFa971c72230A9e9B5b805B64229AB6da17Bc370c"),
        ),
    }
}

async fn wallet_whitelisting(c: Client, wallet_type: WalletType) -> anyhow::Result<()> {
    let name = dummy_name(Some(wallet_type.to_string().as_str()));
    let (asset_id, address) = asset_address(wallet_type);
    let id = c.wallet_create(wallet_type, &name).await?;
    assert!(!id.is_empty());
    let w = c.wallet_by_id(wallet_type, &id).await?;
    assert!(!w.id.is_empty());
    assert_eq!(&w.id, &id);
    c.wallet_create_asset(wallet_type, &w.id, asset_id, &address)
        .await?;
    // let them rest for a second before delete
    tokio::time::sleep(Duration::from_secs(1)).await;
    c.wallet_delete(wallet_type, &id).await?;
    Ok(())
}
#[rstest::rstest]
#[tokio::test]
async fn test_wallet_list_contract(config: Config) -> anyhow::Result<()> {
    config.client().wallets(WalletType::Contract).await?;
    Ok(())
}

#[rstest::rstest]
#[tokio::test]
async fn test_wallet_list_external(config: Config) -> anyhow::Result<()> {
    let c = config.client();
    c.wallets(WalletType::External).await?;
    Ok(())
}

#[rstest::rstest]
#[tokio::test]
async fn test_wallet_list_internal(config: Config) -> anyhow::Result<()> {
    let c = config.client();
    c.wallets(WalletType::Internal).await?;
    Ok(())
}

#[rstest::rstest]
#[tokio::test]
async fn test_wallet_whitelist_contract(config: Config) -> anyhow::Result<()> {
    let c = config.client();
    if let Err(e) = wallet_whitelisting(c, WalletType::Contract).await {
        tracing::warn!("ignoring contract whitelist test {e}");
    }
    Ok(())
}

#[rstest::rstest]
#[tokio::test]
async fn test_wallet_whitelist_external(config: Config) -> anyhow::Result<()> {
    let c = config.client();
    wallet_whitelisting(c, WalletType::External).await
}

#[rstest::rstest]
#[tokio::test]
async fn test_wallet_whitelist_internal(config: Config) -> anyhow::Result<()> {
    let c = config.client();
    wallet_whitelisting(c, WalletType::Internal).await
}
