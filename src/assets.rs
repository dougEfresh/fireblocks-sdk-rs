/// A collection of common asset symbols
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

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
  Dodge(Network),
  ETH(EthNetwork),
  Unknown(String),
}

pub const ASSET_BTC: Asset = Asset::BTC(Network::Main);
pub const ASSET_BTC_TEST: Asset = Asset::BTC(Network::Test);
pub const ASSET_SOL: Asset = Asset::SOL(Network::Main);
pub const ASSET_SOL_TEST: Asset = Asset::SOL(Network::Test);
pub const ASSET_ETH: Asset = Asset::ETH(EthNetwork::Main);
pub const ASSET_ETH_TEST: Asset = Asset::ETH(EthNetwork::Test);
pub const ASSET_DOGE: Asset = Asset::Dodge(Network::Main);
pub const ASSET_DOGE_TEST: Asset = Asset::Dodge(Network::Test);

impl AsRef<str> for Asset {
  #[allow(clippy::match_same_arms)]
  fn as_ref(&self) -> &str {
    match self {
      Self::BTC(Network::Main) => "BTC",
      Self::BTC(Network::Test) => "BTC_TEST",
      Self::Dodge(Network::Main) => "DOGE",
      Self::Dodge(Network::Test) => "DOGE_TEST",
      Self::SOL(Network::Main) => "SOL",
      Self::SOL(Network::Test) => "SOL_TEST",
      Self::ETH(EthNetwork::Main) => "ETH",
      Self::ETH(EthNetwork::Test) => "ETH_TEST6",
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

/// Convert a String to an [`Asset`]
/// Note: This method will never fail
impl FromStr for Asset {
  type Err = std::string::ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_uppercase().as_str() {
      "BTC" => Ok(ASSET_BTC),
      "BTC_TEST" => Ok(ASSET_BTC_TEST),
      "SOL" => Ok(ASSET_SOL),
      "SOL_TEST" => Ok(ASSET_SOL_TEST),
      "DOGE" => Ok(ASSET_DOGE),
      "DOGE_TEST" => Ok(ASSET_DOGE_TEST),
      "ETH" => Ok(ASSET_ETH),
      "ETH_TEST6" => Ok(ASSET_ETH_TEST),
      _ => Ok(Self::Unknown(String::from(s))),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::assets::{Asset, ASSET_DOGE, ASSET_DOGE_TEST, ASSET_ETH, ASSET_ETH_TEST};
  use crate::{ASSET_BTC, ASSET_BTC_TEST, ASSET_SOL, ASSET_SOL_TEST};
  use std::str::FromStr;

  #[test]
  fn asset_from_string() -> color_eyre::Result<()> {
    let a = Asset::from_str("BTC")?;
    assert_eq!(a, ASSET_BTC);

    let a = Asset::from_str("BTC_TEST")?;
    assert_eq!(a, ASSET_BTC_TEST);

    let a = Asset::from_str("SOL")?;
    assert_eq!(a, ASSET_SOL);

    let a = Asset::from_str("SOL_TEST")?;
    assert_eq!(a, ASSET_SOL_TEST);

    let a = Asset::from_str("DOGE")?;
    assert_eq!(a, ASSET_DOGE);

    let a = Asset::from_str("DOGE_TEST")?;
    assert_eq!(a, ASSET_DOGE_TEST);

    let a = Asset::from_str("ETH")?;
    assert_eq!(a, ASSET_ETH);

    let a = Asset::from_str("ETH_TEST6")?;
    assert_eq!(a, ASSET_ETH_TEST);

    let a = Asset::from_str("UNKNOWN")?;
    assert_eq!(a, Asset::Unknown("UNKNOWN".to_string()));
    Ok(())
  }
}
