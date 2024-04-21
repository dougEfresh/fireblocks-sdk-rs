use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub struct ParseError;

impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("")
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EthNetwork {
  Main,
  Test,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Network {
  Main,
  Test,
}

#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Asset {
  BTC(Network),
  SOL(Network),
  ETH(EthNetwork),
  SolToken(Network, String),
  Unknown(String),
}

pub const ASSET_BTC: Asset = Asset::BTC(Network::Main);
pub const ASSET_BTC_TEST: Asset = Asset::BTC(Network::Test);
pub const ASSET_SOL: Asset = Asset::SOL(Network::Main);
pub const ASSET_SOL_TEST: Asset = Asset::SOL(Network::Test);

impl AsRef<str> for Asset {

  #[allow(clippy::match_same_arms)]
  fn as_ref(&self) -> &str {
    match self {
      Self::BTC(Network::Main) => "BTC",
      Self::BTC(Network::Test) => "BTC_TEST",
      Self::SOL(Network::Main) => "SOL",
      Self::SOL(Network::Test) => "SOL_TEST",
      Self::ETH(EthNetwork::Main) => "ETH",
      Self::ETH(EthNetwork::Test) => "ETH_TEST_6",
      Self::SolToken(Network::Main, ref token) => token,
      Self::SolToken(Network::Test, ref token) => token,
      Self::Unknown(ref s) => s,
    }
  }
}

impl Borrow<str> for Asset {
  fn borrow(&self) -> &str {
    self.as_ref()
  }
}

impl Display for Asset {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_ref())
  }
}

impl FromStr for Asset {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_uppercase().as_str() {
      "BTC" => Ok(ASSET_BTC),
      "BTC_TEST" => Ok(ASSET_BTC_TEST),
      "SOL" => Ok(ASSET_SOL),
      "SOL_TEST" => Ok(ASSET_SOL_TEST),
      _ => Ok(Self::Unknown(String::from(s))),
    }
  }
}

/*
impl Into<&str> for Asset {
  fn into(self) -> &'static str {
    ""
  }
}
 */

impl From<Asset> for String {
  fn from(value: Asset) -> Self {
    let a: &str = value.as_ref();
    Self::from(a)
  }
}

#[cfg(test)]
mod tests {
  use crate::assets::Asset;
  use crate::ASSET_BTC;
  use std::str::FromStr;

  #[test]
  fn asset_from_string() -> color_eyre::Result<()> {
    let a = Asset::from_str("BTC")?;
    assert_eq!(a, ASSET_BTC);
    Ok(())
  }
}

/*
impl Into<&'static str> for Asset {
  fn into(self) -> &'static str {
    match self {
      Asset::BTC(n) => {
        match n {
          Network::MAIN => {
            "BTC"
          }
          Network::TEST => {
            "BTC_TEST"
          }
        }
      },
      Asset::ETH(n) => {
        match n {
          EthNetwork::MAIN => {
            "ETH"
          }
          EthNetwork::TEST => {
            "ETH_TEST6"
          }
        }
      },
      Asset::SOL(n) => {
        match n {
          Network::MAIN => {
            "SOL"
          }
          Network::TEST => {
            "SOL_TEST"
          }
        }
      }
      Asset::SolToken(n, t) => {
        match n {
          Network::MAIN => {
            format!("SOL_{}", t.to_uppercase())
          }
          Network::TEST => {
            format!("SOL_TEST_{}", t.to_uppercase())
          }
        }
      }
    }
  }
}

 */
