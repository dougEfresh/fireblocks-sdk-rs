mod setup;
use {
    fireblocks_sdk::{
        apis::{staking_api::GetChainInfoParams, Api},
        ASSET_ETH,
        ASSET_SOL,
        ASSET_SOL_TEST,
    },
    setup::{config, Config},
};

#[rstest::rstest]
#[tokio::test]
async fn test_staking(config: Config) -> anyhow::Result<()> {
    let c = config.client();
    let api = c.apis();
    let stake_api = api.staking_api();
    let chains = stake_api.get_chains().await?;
    assert!(!chains.is_empty());
    stake_api.get_summary().await?;
    let providers = stake_api.get_providers().await?;
    assert!(!providers.is_empty());
    // for p in providers {
    //    let params = ApproveTermsOfServiceByProviderIdParams::builder()
    //        .provider_id(p.id.clone())
    //        .build();
    //    stake_api
    //        .approve_terms_of_service_by_provider_id(params)
    //        .await?;
    //}

    for chain in [ASSET_SOL, ASSET_SOL_TEST, ASSET_ETH] {
        let params = GetChainInfoParams::builder()
            .chain_descriptor(chain.to_string())
            .build();
        stake_api.get_chain_info(params).await?;
    }
    Ok(())
}
