use crate::client::Client;
use crate::{
  types::{
    asset::SupportedAsset,
    staking::{StakingPosition, StakingPositionsSummary},
  },
  Result,
};

impl Client {
  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn supported_assets(&self) -> Result<Vec<SupportedAsset>> {
    let u = self.build_url("supported_assets")?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn staking_chains(&self) -> Result<Vec<String>> {
    let u = self.build_url("staking/chains")?.0;
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
