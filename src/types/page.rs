use crate::{impl_base_query_params, QueryParams};
use bigdecimal::BigDecimal;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Paging {
  pub before: Option<String>,
  pub after: Option<String>,
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

  #[allow(clippy::unnecessary_wraps)]
  pub(crate) fn build(&self) -> std::result::Result<QueryParams, crate::error::ParamError> {
    Ok(Vec::clone(&self.params))
  }
}

#[derive(Debug, Default)]
pub struct PagingAddressRequestBuilder {
  params: QueryParams,
  base: BasePageParams,
}

impl_base_query_params!(PagingAddressRequestBuilder);

#[derive(Debug, Default)]
pub struct PagingVaultRequestBuilder {
  params: QueryParams,
  base: BasePageParams,
}

impl_base_query_params!(PagingVaultRequestBuilder);

impl PagingAddressRequestBuilder {
  pub fn before(&mut self, t: &str) -> &mut Self {
    self.params.push(("before".to_owned(), String::from(t)));
    self
  }

  pub fn after(&mut self, t: &str) -> &mut Self {
    self.params.push(("before".to_owned(), String::from(t)));
    self
  }
}

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

  pub fn before(&mut self, t: &str) -> &mut Self {
    self.params.push(("before".to_owned(), String::from(t)));
    self
  }

  pub fn after(&mut self, t: &str) -> &mut Self {
    if !t.is_empty() {
      self.params.push(("after".to_owned(), String::from(t)));
    }
    self
  }
}
