use fireblocks_sdk::{ClientBuilder, PagingVaultRequestBuilder};
use std::time::Duration;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
  let api_key = std::env::var("FIREBLOCKS_API_KEY")?;
  let secret = std::env::var("FIREBLOCKS_SECRET")?;
  let client = ClientBuilder::new(&api_key, &secret.into_bytes())
    .with_timeout(Duration::from_secs(10))
    .with_connect_timeout(Duration::from_secs(5))
    .build()?;
  let params = PagingVaultRequestBuilder::new().limit(10).build()?;
  let (vault_accounts, request_id) = client.vaults(params).await?;
  println!("Got requestId: {request_id}");
  println!("vault accounts: {:#?}", vault_accounts.accounts);
  Ok(())
}
