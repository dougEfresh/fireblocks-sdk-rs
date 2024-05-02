use crate::client::Client;
use crate::{
  types::{
    asset::SupportedAsset,
    staking::{StakingChainInfo, StakingPosition, StakingPositionsSummary},
  },
  Asset, Result,
};
use std::fmt::{Debug, Display};

impl Client {
  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn supported_assets(&self) -> Result<Vec<SupportedAsset>> {
    let u = self.build_url("supported_assets")?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn staking_chains(&self) -> Result<Vec<Asset>> {
    let u = self.build_url("staking/chains")?.0;
    self.get(u).await
  }

  /// Get info about your stake
  ///
  /// See
  ///
  /// * [`StakingChainInfo`]
  /// * [getChainInfo](https://docs.fireblocks.com/api/swagger-ui/#/Staking%20(Beta)/getChainInfo)
  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn staking_chain_info<T>(&self, chain: T) -> Result<StakingChainInfo>
  where
    T: AsRef<str> + Display + Debug,
  {
    let u = self.build_url(format!("staking/chains/{chain}/chainInfo"))?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn staking_positions(&self) -> Result<Vec<StakingPosition>> {
    let u = self.build_url("staking/positions")?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn staking_positions_summary(&self) -> Result<StakingPositionsSummary> {
    let u = self.build_url("staking/positions/summary")?.0;
    self.get(u).await
  }
}
