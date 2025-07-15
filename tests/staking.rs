mod setup;
use {
    fireblocks_sdk::{
        apis::{Api, staking_api::GetChainInfoParams},
        models::ChainDescriptor,
    },
    setup::{Config, config},
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

    for chain in [ChainDescriptor::Eth, ChainDescriptor::Sol] {
        let params = GetChainInfoParams::builder()
            .chain_descriptor(chain)
            .build();
        stake_api.get_chain_info(params).await?;
    }
    Ok(())
}
