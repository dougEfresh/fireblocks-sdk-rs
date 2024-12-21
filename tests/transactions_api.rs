mod setup;
use {apis::transactions_api::*, fireblocks_sdk::*, setup::CLIENT, tracing::info};

#[tokio::test]
async fn get_transactions() -> anyhow::Result<()> {
    setup::setup();
    let c = CLIENT.get();
    if c.is_none() {
        return Ok(());
    }
    let c = c.unwrap();
    let params = GetTransactionsParams::builder().limit(10).build();
    let result = c.transactions_api().get_transactions(params).await?;
    assert!(result.len() > 0);
    Ok(())
}
