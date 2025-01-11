use {
    fireblocks_sdk::{apis::vaults_api::GetVaultAccountParams, ClientBuilder},
    std::time::Duration,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = std::env::var("FIREBLOCKS_API_KEY")?;
    let secret = std::env::var("FIREBLOCKS_SECRET")?;
    let client = ClientBuilder::new(&api_key, &secret.into_bytes())
        .with_timeout(Duration::from_secs(10))
        .with_connect_timeout(Duration::from_secs(5))
        .build()?;

    let id = String::from("0");
    let params = GetVaultAccountParams {
        vault_account_id: id.clone(),
    };

    let vault_account = client.vaults_api().get_vault_account(params).await?;
    println!("vault account: {vault_account:#?}");
    Ok(())
}
