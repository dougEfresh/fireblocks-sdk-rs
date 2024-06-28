use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HookResponse {
  pub messages_count: u32,
}
