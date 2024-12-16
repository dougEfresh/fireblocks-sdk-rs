mod setup;

#[tokio::test]
async fn vaults_api() -> anyhow::Result<()> {
    setup::setup();
    Ok(())
}
