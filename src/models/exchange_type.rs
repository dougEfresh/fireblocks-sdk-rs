/*
 * Fireblocks API
 *
 * Fireblocks provides a suite of applications to manage digital asset operations and a complete development platform to build your business on the blockchain.  - Visit our website for more information: [Fireblocks Website](https://fireblocks.com) - Visit our developer docs: [Fireblocks DevPortal](https://developers.fireblocks.com) 
 *
 * The version of the OpenAPI document: 1.8.0
 * Contact: developers@fireblocks.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ExchangeType {
    #[serde(rename = "BINANCE")]
    Binance,
    #[serde(rename = "BINANCEUS")]
    Binanceus,
    #[serde(rename = "BITFINEX")]
    Bitfinex,
    #[serde(rename = "BITHUMB")]
    Bithumb,
    #[serde(rename = "BITMEX")]
    Bitmex,
    #[serde(rename = "BITSO")]
    Bitso,
    #[serde(rename = "BITSTAMP")]
    Bitstamp,
    #[serde(rename = "BITTREX")]
    Bittrex,
    #[serde(rename = "CIRCLE")]
    Circle,
    #[serde(rename = "COINBASEPRO")]
    Coinbasepro,
    #[serde(rename = "COINMETRO")]
    Coinmetro,
    #[serde(rename = "COINSPRO")]
    Coinspro,
    #[serde(rename = "CRYPTOCOM")]
    Cryptocom,
    #[serde(rename = "DERIBIT")]
    Deribit,
    #[serde(rename = "FTX")]
    Ftx,
    #[serde(rename = "FIXUS")]
    Fixus,
    #[serde(rename = "GEMINI")]
    Gemini,
    #[serde(rename = "HITBTC")]
    Hitbtc,
    #[serde(rename = "HUOBI")]
    Huobi,
    #[serde(rename = "KORBIT")]
    Korbit,
    #[serde(rename = "KRAKEN")]
    Kraken,
    #[serde(rename = "LIQUID")]
    Liquid,
    #[serde(rename = "POLONIEX")]
    Poloniex,
    #[serde(rename = "OKCOIN")]
    Okcoin,
    #[serde(rename = "OKEX")]
    Okex,
    #[serde(rename = "SEEDCX")]
    Seedcx,

}

impl std::fmt::Display for ExchangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Binance => write!(f, "BINANCE"),
            Self::Binanceus => write!(f, "BINANCEUS"),
            Self::Bitfinex => write!(f, "BITFINEX"),
            Self::Bithumb => write!(f, "BITHUMB"),
            Self::Bitmex => write!(f, "BITMEX"),
            Self::Bitso => write!(f, "BITSO"),
            Self::Bitstamp => write!(f, "BITSTAMP"),
            Self::Bittrex => write!(f, "BITTREX"),
            Self::Circle => write!(f, "CIRCLE"),
            Self::Coinbasepro => write!(f, "COINBASEPRO"),
            Self::Coinmetro => write!(f, "COINMETRO"),
            Self::Coinspro => write!(f, "COINSPRO"),
            Self::Cryptocom => write!(f, "CRYPTOCOM"),
            Self::Deribit => write!(f, "DERIBIT"),
            Self::Ftx => write!(f, "FTX"),
            Self::Fixus => write!(f, "FIXUS"),
            Self::Gemini => write!(f, "GEMINI"),
            Self::Hitbtc => write!(f, "HITBTC"),
            Self::Huobi => write!(f, "HUOBI"),
            Self::Korbit => write!(f, "KORBIT"),
            Self::Kraken => write!(f, "KRAKEN"),
            Self::Liquid => write!(f, "LIQUID"),
            Self::Poloniex => write!(f, "POLONIEX"),
            Self::Okcoin => write!(f, "OKCOIN"),
            Self::Okex => write!(f, "OKEX"),
            Self::Seedcx => write!(f, "SEEDCX"),
        }
    }
}

impl Default for ExchangeType {
    fn default() -> ExchangeType {
        Self::Binance
    }
}

