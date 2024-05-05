use crate::api::Success;
use crate::types::hooks::HookResponse;
use crate::Client;
use crate::Result;
use serde_derive::{Deserialize, Serialize};

impl Client {
  /// Resends all failed webhook notifications.
  ///
  /// See
  /// * [resendWebhooks](https://docs.fireblocks.com/api/swagger-ui/#/Webhooks/resendWebhooks)
  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn hooks_resend(&self) -> Result<HookResponse> {
    let u = self.build_url("webhooks/resend")?.0;
    self.post::<HookResponse, ()>(u, None).await
  }

  /// Resend a specific transaction by txId
  ///
  /// See
  /// * [resendTransactionWebhooks](https://docs.fireblocks.com/api/swagger-ui/#/Webhooks/resendTransactionWebhooks)
  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn hooks_resend_tx(&self, tx_id: &str, created: bool, updated: bool) -> Result<Success> {
    #[derive(Debug, Deserialize, Serialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct HookTransaction {
      resend_created: bool,
      resend_status_updated: bool,
    }
    let request = HookTransaction { resend_created: created, resend_status_updated: updated };
    let u = self.build_url(format!("webhooks/resend/{tx_id}"))?.0;
    self.post::<Success, HookTransaction>(u, Some(request).as_ref()).await
  }
}
