use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Metadata {
  pub app_url: String,
  pub app_name: String,
  #[serde(default)]
  pub app_description: String,
  pub app_icon: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct WalletConnection {
  pub id: String,
  #[serde(rename = "userId")]
  pub user: String,
  #[serde(rename = "sessionMetadata")]
  pub metadata: Metadata,
  #[serde(rename = "vaultAccountId")]
  pub vault: i32,
  pub fee_level: String,
  #[serde(rename = "chainIds")]
  pub chains: Vec<String>,
  pub connection_type: String,
  pub connection_method: String,
  pub creation_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct NextPage {
  pub next: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct PagedWalletConnectResponse {
  pub data: Vec<WalletConnection>,
  pub page: Option<NextPage>,
}

#[cfg(test)]
mod test {
  use crate::types::connect::WalletConnection;
  use chrono::Datelike;

  #[test]
  fn test_json_wallet_connections() {
    let data = r#"
        {
      "id": "4e9e7051-f3b2-48e9-8ee6-b12492552657",
      "userId": "string",
      "sessionMetadata": {
        "appUrl": "url",
        "appName": "string",
        "appDescription": "string",
        "appIcon": "string"
      },
      "vaultAccountId": 1,
      "feeLevel": "MEDIUM",
      "chainIds": [
        "ETH",
        "ETH_TEST",
        "SOL"
      ],
      "connectionType": "WalletConnect",
      "connectionMethod": "API",
      "creationDate": "2024-03-22T10:51:03.772Z"
    }
    "#;

    let r: WalletConnection = serde_json::from_str(data).expect("oh no");
    assert_eq!(22, r.creation_date.day());
    assert_eq!("url", r.metadata.app_url);
  }
}
