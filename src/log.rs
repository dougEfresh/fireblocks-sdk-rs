use async_trait::async_trait;
use http::Extensions;
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next, Result};
use tracing::info;
use tracing::Level;

pub struct LoggingMiddleware;

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        let span = tracing::span!(
            Level::INFO,
            "request",
            method = req.method().to_string(),
            path = req.url().path().to_string(),
            params = req.url().query().unwrap_or_default(),
            requestId = tracing::field::Empty
        );
        let _enter = span.enter();
        tracing::debug!("request started {:?}", req);
        let res = next.run(req, extensions).await;
        if let Ok(response) = &res {
            let request_id = response
                .headers()
                .get("x-request-id")
                .and_then(|value| value.to_str().ok())
                .unwrap_or_default()
                .to_string();
            span.record("requestId", &request_id);
        }
        res
    }
}
