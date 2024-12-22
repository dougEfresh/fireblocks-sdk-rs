use {
    async_trait::async_trait,
    bytes::Bytes,
    http::Extensions,
    jsonwebtoken::{errors as jwterrors, Algorithm, EncodingKey, Header},
    rand::Rng,
    reqwest::Response,
    reqwest_middleware::{Middleware, Next},
    serde::{Deserialize, Serialize},
    sha2::{Digest, Sha256},
    std::{
        fmt::Debug,
        time::{SystemTime, UNIX_EPOCH},
    },
    thiserror::Error,
};

const EXPIRY: u64 = 55;

#[derive(Clone)]
pub struct Signer {
    key: EncodingKey,
    api_key: String,
}

impl Signer {
    pub fn new(key: EncodingKey, api_key: &str) -> Self {
        Self {
            key,
            api_key: api_key.to_string(),
        }
    }

    pub fn sign<S: Serialize + Debug>(
        &self,
        path: &str,
        body: Option<S>,
    ) -> Result<String, JwtError> {
        let header = Header::new(Algorithm::RS256);
        let claims = match body {
            Some(b) => Claims::new(path, &self.api_key, b)?,
            None => Claims::new(path, &self.api_key, ())?,
        };
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
    /// The expiration time on and after which the JWT must not be accepted for
    /// processing, in seconds since Epoch. Must be less than iat+30sec.
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

        Ok(Self {
            uri,
            sub,
            body_hash: body_hash.to_hex_string(),
            nonce,
            iat: now,
            exp: now + EXPIRY,
        })
    }
}

#[derive(Clone)]
pub struct JwtSigningMiddleware {
    signer: Signer,
}

impl JwtSigningMiddleware {
    pub const fn new(signer: Signer) -> Self {
        Self { signer }
    }

    fn calculate_body_hash(body: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(body);
        hex::encode(hasher.finalize())
    }
}

#[async_trait]
impl Middleware for JwtSigningMiddleware {
    async fn handle(
        &self,
        mut req: reqwest::Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        let query = req
            .url()
            .query()
            .map_or_else(String::new, |q| format!("?{q}"));
        let path = format!("{}{}", req.url().path(), query);

        let body = req
            .body()
            .and_then(|body| body.as_bytes().map(Self::calculate_body_hash));
        let jwt = body.map_or_else(
            || self.signer.sign::<()>(&path, None),
            |body_hash| self.signer.sign(&path, Some(body_hash)),
        );
        let jwt = jwt.map_err(|e| {
            anyhow::format_err!("failed to sign payload for path {path} error:'{e}'")
        })?;
        // Add the Authorization header
        req.headers_mut().insert(
            "Authorization",
            format!("Bearer {jwt}")
                .parse()
                .expect("failed to parse HttpHeader for jwt"),
        );
        req.headers_mut().insert(
            "X-API-Key",
            self.signer
                .api_key()
                .parse()
                .expect("could not create x-api-key header"),
        );

        // Continue with the request chain
        next.run(req, extensions).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claim_new() -> anyhow::Result<()> {
        Claims::new("/api", "api-key", "body")?;
        Ok(())
    }
}
