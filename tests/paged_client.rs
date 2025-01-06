mod setup;
use chrono::{TimeZone, Utc};
use {
    apis::vaults_api::*,
    fireblocks_sdk::*,
    setup::CLIENT,
    std::{sync::Arc, time::Duration},
    tokio_stream::StreamExt,
    tracing::info,
};

async fn transaction_stream(mut ts: TransactionStream) -> anyhow::Result<()> {
    let mut counter = 0;
    let mut after: f64 = Utc
        .with_ymd_and_hms(2022, 4, 6, 0, 1, 1)
        .unwrap()
        .timestamp_millis();

    while let Some(result) = ts.try_next().await? {
        tracing::info!("transactions {}", result.len());
        counter += 1;
        if counter > 5 {
            break;
        }
        if let Some(last) = result.last() {
            if let Some(created) = last.created_at {
                assert!(after < created);
                after = created;
            }
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    Ok(())
}

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

#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
async fn test_paged_transactions() -> anyhow::Result<()> {
    setup::setup();
    let c = CLIENT.get();
    if c.is_none() {
        return Ok(());
    }

    let pc = PagedClient::new(Arc::new(c.unwrap().clone()));
    let ts = pc.transactions_from_source(0, 100, None);
    transaction_stream(ts).await?;
    let ts = pc.transactions_from_destination(0, 100, None);
    transaction_stream(ts).await
}
