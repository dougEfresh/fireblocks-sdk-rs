mod setup;
use {apis::vaults_api::*, fireblocks_sdk::*, setup::CLIENT, tracing::info};

#[tokio::test]
async fn get_vault_account() -> anyhow::Result<()> {
    setup::setup();
    let c = CLIENT.get();
    if c.is_none() {
        return Ok(());
    }
    let c = c.unwrap();
    let id = String::from("0");
    let params = GetVaultAccountParams {
        vault_account_id: id.clone(),
    };
    let result = c.vaults_api().get_vault_account(params).await?;
    assert_eq!(result.id, id);

    Ok(())
}

#[tokio::test]
async fn get_paged_vault_accounts() -> anyhow::Result<()> {
    setup::setup();
    let c = CLIENT.get();
    if c.is_none() {
        return Ok(());
    }
    let c = c.unwrap();
    let params = GetPagedVaultAccountsParams::builder().limit(10.0).build();
    let result = c.vaults_api().get_paged_vault_accounts(params).await?;
    assert!(result.accounts.is_some());
    assert!(!result.accounts.unwrap().is_empty());
    Ok(())
}
