mod setup;
use {
    apis::transactions_api::*,
    fireblocks_sdk::*,
    setup::{Config, config},
};

#[rstest::rstest]
#[tokio::test]
async fn get_transactions(config: Config) -> anyhow::Result<()> {
    let c = config.client();
    let params = GetTransactionsParams::builder()
        .limit(10)
        .after("1649203261000".to_string())
        .order_by("createdAt".to_owned())
        .sort("ASC".to_string())
        .status(models::TransactionStatus::Completed)
        .source_id("0".to_string())
        .build();
    let result = c.transactions_api().get_transactions(params).await?;
    assert!(!result.is_empty());
    Ok(())
}
