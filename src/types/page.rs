use crate::{impl_base_query_params, Epoch, QueryParams};
use bigdecimal::BigDecimal;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Paging {
  pub before: Option<String>,
  pub after: Option<String>,
}

impl Paging {
  fn epoch(before: &Epoch) -> String {
    format!("{}", before.timestamp_millis())
  }

  pub fn set_before(&mut self, before: &Epoch) {
    self.before = Some(Self::epoch(before));
  }

  pub fn set_after(&mut self, after: &Epoch) {
    self.after = Some(Self::epoch(after));
  }
}

#[derive(Debug, Default)]
pub struct BasePageParams {
  params: QueryParams,
}

impl BasePageParams {
  pub(crate) const fn new() -> Self {
    Self { params: Vec::new() }
  }

  pub(crate) fn limit(&mut self, limit: u16) -> &mut Self {
    self.params.push(("limit".to_owned(), format!("{limit}")));
    self
  }

  pub(crate) fn before(&mut self, t: &Epoch) -> &mut Self {
    self.add_instant("before", t)
  }

  pub(crate) fn after(&mut self, t: &Epoch) -> &mut Self {
    self.add_instant("after", t)
  }

  fn add_instant(&mut self, param: &str, t: &Epoch) -> &mut Self {
    self.params.push((param.to_owned(), Self::epoch(t)));
    self
  }
  fn epoch(before: &Epoch) -> String {
    format!("{}", before.timestamp_millis())
  }

  #[allow(clippy::unnecessary_wraps)]
  pub(crate) fn build(&self) -> std::result::Result<QueryParams, crate::error::ParamError> {
    Ok(Vec::clone(&self.params))
  }
}

#[derive(Debug, Default)]
pub struct PagingAddressRequestBuilder {
  params: QueryParams, // this is ignored
  base: BasePageParams,
}

impl_base_query_params!(PagingAddressRequestBuilder);

#[derive(Debug, Default)]
pub struct PagingVaultRequestBuilder {
  params: QueryParams,
  base: BasePageParams,
}

impl_base_query_params!(PagingVaultRequestBuilder);

impl PagingVaultRequestBuilder {
  pub fn min_threshold(&mut self, min: &BigDecimal) -> &mut Self {
    self.params.push(("minAmountThreshold".to_owned(), min.to_string()));
    self
  }

  pub fn name_prefix(&mut self, n: &str) -> &mut Self {
    self.params.push(("namePrefix".to_owned(), String::from(n)));
    self
  }

  pub fn name_suffix(&mut self, n: &str) -> &mut Self {
    self.params.push(("nameSuffix".to_owned(), String::from(n)));
    self
  }
}
