mod setup;
use {
    apis::vaults_api::*,
    fireblocks_sdk::*,
    setup::{config, Config},
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
