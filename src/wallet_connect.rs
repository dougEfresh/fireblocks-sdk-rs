use crate::types::connect::{PagedWalletConnectResponse, WalletApprove, WalletConnectRequest, WalletConnectResponse};
use crate::Client;

impl Client {
  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn wallet_connections(&self) -> crate::Result<PagedWalletConnectResponse> {
    let u = self.build_url("connections")?.0;
    self.get(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn wallet_connect(&self, request: &WalletConnectRequest) -> crate::Result<WalletConnectResponse> {
    let u = self.build_url("connections/wc")?.0;
    self.post(u, Some(request)).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn wallet_connection_delete(&self, id: &str) -> crate::Result<()> {
    let u = self.build_url(&format!("connections/wc/{id}"))?.0;
    self.delete(u).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn wallet_connection_approve(&self, id: &str, approve: bool) -> crate::Result<()> {
    let u = self.build_url(&format!("connections/wc/{id}"))?.0;
    self.put(u, Some(&WalletApprove { approve })).await
  }
}
