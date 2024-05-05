use bigdecimal::BigDecimal;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Fee {
  pub network_fee: Option<BigDecimal>,
  pub gas_price: Option<BigDecimal>,
  pub fee_per_byte: Option<BigDecimal>,
  pub base_fee: Option<BigDecimal>,
  pub priority_fee: Option<BigDecimal>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EstimateFee {
  pub low: Fee,
  pub medium: Fee,
  pub high: Fee,
}
