mod setup;
use {
    fireblocks_sdk::{
        apis::d_app_connections_api::{RemoveParams, SubmitParams},
        models::RespondToConnectionRequest,
    },
    setup::{config, Config},
};

#[rstest::rstest]
#[tokio::test]
async fn test_wallet_connections(config: Config) -> anyhow::Result<()> {
    let c = config.client();
    let dapp = c.wallet_connect_api();
    let params = RemoveParams::builder()
        .id("wallet-connect-id".to_owned())
        .build();
    if let Err(e) = dapp.remove(params).await {
        assert!(e.to_string().contains("Not Found"));
    }
    let params = SubmitParams::builder()
        .id("wallet-connect-id".to_owned())
        .respond_to_connection_request(RespondToConnectionRequest::new(true))
        .build();

    if let Err(e) = dapp.submit(params).await {
        let msg = e.to_string();
        assert!(msg.contains("Not Found") || msg.contains("Unauthorized"));
    }
    Ok(())
}
