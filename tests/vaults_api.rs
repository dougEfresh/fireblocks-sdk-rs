mod setup;
use {
    apis::vaults_api::*,
    fireblocks_sdk::*,
    setup::{config, Config, CLIENT},
    tracing::info,
};

#[rstest::rstest]
#[tokio::test]
async fn get_vault_account(config: &Config) -> anyhow::Result<()> {
    if !config.is_ok() {
        return Ok(());
    }
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
async fn get_paged_vault_accounts(config: &Config) -> anyhow::Result<()> {
    if !config.is_ok() {
        return Ok(());
    }
    let c = config.client();
    let params = GetPagedVaultAccountsParams::builder().limit(10.0).build();
    let result = c.vaults_api().get_paged_vault_accounts(params).await?;
    assert!(!result.accounts.is_empty());
    Ok(())
}

#[rstest::rstest]
#[tokio::test]
async fn test_vault_names(config: &Config) -> anyhow::Result<()> {
    if !config.is_ok() {
        return Ok(());
    }
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
#[test]
fn check_ci(config: &Config) -> anyhow::Result<()> {
    if std::env::var("CI").is_ok() {
        if !config.is_ok() {
            return Err(anyhow::format_err!(
                "client is not configured and you are running in CI"
            ));
        }
    }
    Ok(())
}
