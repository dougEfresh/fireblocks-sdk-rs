use crate::types::WalletContainer;
use crate::Client;
use crate::Result;

impl Client {
  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn internal_wallets(&self) -> Result<Vec<WalletContainer>> {
    let u = self.build_url("internal_wallets")?.0;
    self.get(u).await
  }
}
