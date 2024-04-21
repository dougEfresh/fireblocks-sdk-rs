use crate::{impl_base_query_params, Epoch};
use bigdecimal::BigDecimal;

#[derive(Debug, Default)]
pub struct BasePageParams {
  params: Vec<(String, String)>,
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

  pub(crate) fn build(&self) -> Vec<(String, String)> {
    Vec::clone(&self.params)
  }
}

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct PagingVaultRequestBuilder {
  params: Vec<(String, String)>,
  base: BasePageParams,
  // TODO min_threshold: Option<&BigDecimal>,
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
