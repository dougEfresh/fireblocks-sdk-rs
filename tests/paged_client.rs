mod setup;
use {
    chrono::{TimeZone, Utc},
    fireblocks_sdk::*,
    setup::{Config, config},
    std::{sync::Arc, time::Duration},
    tokio_stream::StreamExt,
};

async fn transaction_stream(mut ts: TransactionStream) -> anyhow::Result<()> {
    let mut counter = 0;
    let mut after = Utc
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
                // tracing::info!("id={}", last.id)
                #[allow(clippy::cast_possible_truncation)]
                let created = created as i64;
                assert!(after < created);
                after = created;
            }
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    Ok(())
}

#[rstest::rstest]
#[tokio::test]
//#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
async fn get_paged_vault_accounts(config: Config) -> anyhow::Result<()> {
    let c = config.client();
    let pc = PagedClient::new(Arc::new(c.clone()));
    let mut vs = pc.vaults(50);

    while let Ok(Some(_)) = vs.try_next().await {
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    Ok(())
}

#[rstest::rstest]
#[tokio::test]
async fn test_paged_transactions(config: Config) -> anyhow::Result<()> {
    let c = config.client();
    let pc = PagedClient::new(Arc::new(c.clone()));
    let ts = pc.transactions_from_source(0, 100, None);
    transaction_stream(ts).await?;
    let ts = pc.transactions_from_destination(0, 100, None);
    transaction_stream(ts).await
}
