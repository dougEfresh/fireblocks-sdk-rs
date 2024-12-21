mod setup;
use {
    apis::vaults_api::*,
    fireblocks_sdk::*,
    setup::CLIENT,
    std::{sync::Arc, time::Duration},
    tokio_stream::StreamExt,
    tracing::info,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
async fn get_paged_vault_accounts() -> anyhow::Result<()> {
    setup::setup();
    let c = CLIENT.get();
    if c.is_none() {
        return Ok(());
    }
    let pc = PagedClient::new(Arc::new(c.unwrap().clone()));
    let mut vs = pc.vaults(50);

    while let Ok(Some(result)) = vs.try_next().await {
        if let Some(accounts) = result.accounts {
            tracing::info!("accounts {}", accounts.len());
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }
    Ok(())
}
