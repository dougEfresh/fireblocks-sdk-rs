use {
    async_trait::async_trait,
    bytes::Bytes,
    http::Extensions,
    jsonwebtoken::{errors as jwterrors, Algorithm, EncodingKey, Header},
    rand::Rng,
    reqwest::{Body, Response},
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

    pub fn sign(&self, path: &str, body: Vec<u8>) -> Result<String, JwtError> {
        //tracing::debug!("signing path:'{}' hasBody:{}", path, body.is_some());
        let header = Header::new(Algorithm::RS256);
        let claims = Claims::new(path, &self.api_key, &body);
        let msg = jsonwebtoken::encode(&header, &claims, &self.key)?;
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
    fn new(uri: &'a str, sub: &'a str, body: &[u8]) -> Self {
        // use millisecond precision to ensure that it's not reused
        let now = u64::try_from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis(),
        )
        .unwrap_or_default();
        let mut rng = rand::thread_rng();
        let nonce = rng.gen::<u64>();
        let now = now / 1000;

        let body_hash = {
            let mut digest = Sha256::new();
            digest.update(body);
            digest.finalize().to_vec()
        };

        Self {
            uri,
            sub,
            body_hash: body_hash.to_hex_string(),
            nonce,
            iat: now,
            exp: now + EXPIRY,
        }
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
}
impl JwtSigningMiddleware {
    async fn buffer_and_get_body_bytes(req: &mut reqwest::Request) -> Vec<u8> {
        // Extract the existing body (if any)
        let body_opt = req.body_mut().take();

        // If there's no body set, return empty buffer
        let Some(body) = body_opt else {
            return Vec::new();
        };

        // Convert it into bytes (this will buffer the entire body in memory)
        let body_bytes = body.as_bytes().unwrap_or_default().to_vec();

        // Re-attach a copy of the bytes to the request body
        req.body_mut().replace(Body::from(body_bytes.clone()));
        body_bytes
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
        let body_bytes = Self::buffer_and_get_body_bytes(&mut req).await;
        let jwt = self.signer.sign(&path, body_bytes).map_err(|e| {
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

        next.run(req, extensions).await
    }
}
