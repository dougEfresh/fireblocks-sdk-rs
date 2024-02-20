use std::{
  fmt::Debug,
  time::{SystemTime, UNIX_EPOCH},
};

use jsonwebtoken::{errors as jwterrors, Algorithm, EncodingKey, Header};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

const EXPIRY: u64 = 55;

#[derive(Clone)]
pub struct Signer {
  key: EncodingKey,
  api_key: String,
}

impl Signer {
  pub fn new(key: EncodingKey, api_key: &str) -> Self {
    Self { key, api_key: api_key.to_string() }
  }

  #[tracing::instrument(skip(self))]
  pub fn sign<S: Serialize + Debug>(&self, path: &str, body: S) -> Result<String, JwtError> {
    let header = Header::new(Algorithm::RS256);
    let claims = Claims::new(path, &self.api_key, body)?;
    let msg = jsonwebtoken::encode(&header, &claims, &self.key)?;
    // debug!("signed message {}", msg);
    Ok(msg)
  }

  pub fn api_key(&self) -> &str {
    &self.api_key
  }
}

#[derive(Debug, Deserialize, Serialize)]
/// JWT Claims as specified in [signing](https://docs.fireblocks.com/api/#signing-a-request)
struct Claims<'a> {
  /// The URI part of the request (e.g., /v1/transactions)
  uri: &'a str,
  /// Constantly increasing number. Usually, a timestamp can be used.
  nonce: u64,
  /// The time at which the JWT was issued, in seconds since Epoch.
  iat: u64,
  /// The expiration time on and after which the JWT must not be accepted for processing, in seconds since Epoch. Must be less than iat+30sec.
  exp: u64,
  /// The API key
  sub: &'a str,
  #[serde(rename = "bodyHash")]
  /// Hex-encoded SHA-256 hash of the raw HTTP request body.
  body_hash: String,
}

#[derive(Debug, Error)]
pub enum JwtError {
  #[error("Could not serialize JWT body: {0}")]
  Json(#[from] serde_json::Error),
  #[error("Could not create JWT time: {0}")]
  Time(#[from] std::time::SystemTimeError),
  #[error(transparent)]
  Jwt(#[from] jwterrors::Error),
  #[error(transparent)]
  TryFrom(#[from] std::num::TryFromIntError),
}

trait HexString {
  fn to_hex_string(&self) -> String;
}

impl HexString for Vec<u8> {
  #[allow(clippy::format_collect)]
  fn to_hex_string(&self) -> String {
    self.iter().map(|byte| format!("{byte:02x}")).collect()
  }
}

impl<'a> Claims<'a> {
  fn new<S: Serialize>(uri: &'a str, sub: &'a str, body: S) -> Result<Self, JwtError> {
    // use millisecond precision to ensure that it's not reused
    let now = u64::try_from(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis())?;
    let mut rng = rand::thread_rng();
    let nonce = rng.gen::<u64>();
    let now = now / 1000;

    let body_hash = {
      let mut digest = Sha256::new();
      digest.update(serde_json::to_vec(&body)?);
      digest.finalize().to_vec()
    };

    Ok(Self { uri, sub, body_hash: body_hash.to_hex_string(), nonce, iat: now, exp: now + EXPIRY })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn claim_new() {
    Claims::new("/api", "api-key", "body").expect("failed creating claim");
  }
}
