use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HookResponse {
  pub message_count: u32,
}
