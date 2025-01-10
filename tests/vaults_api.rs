mod setup;
use {
    apis::vaults_api::*,
    fireblocks_sdk::*,
    setup::{config, Config},
    std::time::Duration,
};

#[rstest::rstest]
#[tokio::test]
async fn get_vault_account(config: Config) -> anyhow::Result<()> {
    let c = config.client();
    let id = String::from("0");
    let params = GetVaultAccountParams::builder()
        .vault_account_id(id.clone())
        .build();
    let result = c.vaults_api().get_vault_account(params).await?;
    assert_eq!(result.id, id);

    Ok(())
}

#[rstest::rstest]
#[tokio::test]
async fn vault_id(config: Config) -> anyhow::Result<()> {
    let c = config.client();
    c.vault("0").await?;
    Ok(())
}

#[rstest::rstest]
#[tokio::test]
async fn vault_addresses(config: Config) -> anyhow::Result<()> {
    let c = config.client();
    c.addresses("0", ASSET_SOL_TEST).await?;
    Ok(())
}

#[rstest::rstest]
#[tokio::test]
async fn get_paged_vault_accounts(config: Config) -> anyhow::Result<()> {
    let c = config.client();
    let params = GetPagedVaultAccountsParams::builder().limit(10.0).build();
    let result = c.vaults_api().get_paged_vault_accounts(params).await?;
    assert!(!result.accounts.is_empty());
    Ok(())
}

#[rstest::rstest]
#[tokio::test]
async fn test_vault_names(config: Config) -> anyhow::Result<()> {
    let c = config.client();
    let params = GetPagedVaultAccountsParams::builder()
        .name_prefix("Default".to_owned())
        .build();
    let results = c.vaults_api().get_paged_vault_accounts(params).await?;
    assert!(!results.accounts.is_empty());
    assert_eq!(results.accounts[0].name, "Default");

    let params = GetPagedVaultAccountsParams::builder()
        .name_suffix("Default".to_owned())
        .build();
    let results = c.vaults_api().get_paged_vault_accounts(params).await?;
    assert!(!results.accounts.is_empty());
    assert_eq!(results.accounts[0].name, "Default");
    Ok(())
}

#[rstest::rstest]
#[tokio::test]
async fn vault_create(config: Config) -> anyhow::Result<()> {
    if !config.create_vault() {
        return Ok(());
    }
    let c = config.client();

    let params = models::CreateVaultAccountRequest {
        name: Some(setup::dummy_name(None)),
        hidden_on_ui: Some(true),
        customer_ref_id: None,
        auto_fuel: None,
        vault_type: None,
        auto_assign: None,
    };
    let _vault_account = c.create_vault(params).await?;
    //tokio::time::sleep(Duration::from_secs(1)).await;
    //c.create_asset(&vault_account.id, ASSET_ETH_TEST).await?;
    Ok(())
}
