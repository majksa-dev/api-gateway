use std::sync::Arc;

use log::error;
use pingora::{
    http::{Method, RequestHeader, ResponseHeader, StatusCode},
    proxy::Session,
    Result,
};

use crate::{
    http::{
        cors::{ApplyCors, Preflight},
        headers::GetHeader,
    },
    utils::time::Frequency,
};

use super::{
    config,
    context::CTX,
    rate_limit::RateLimiter,
    request::{RequestHandler, ToStatusCode},
};

#[derive(Debug, Clone)]
pub struct Endpoint(config::Endpoint);
impl Endpoint {
    pub fn new(config: &config::Endpoint) -> Self {
        Endpoint(config.clone())
    }

    pub fn is_match(&self, req: &RequestHeader) -> bool {
        (req.method == Method::OPTIONS || self.0.method == req.method)
            && self.0.path.is_match(req.uri.path())
    }

    pub fn id(&self) -> &String {
        &self.0.id
    }

    pub fn rate_limit(&self) -> Option<&Frequency> {
        self.0.rate_limit.as_ref()
    }
}

impl RequestHandler for Endpoint {
    async fn handle_request(&self, session: &mut Session, ctx: &mut CTX) -> Option<StatusCode> {
        match session.preflight(&self.0.headers).await {
            Ok(false) => {}
            Ok(true) => return Some(StatusCode::OK),
            Err(e) => {
                error!("Error when handling preflight: {}", e);
                return Some(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
        if let Err(status) = ctx.connect().to_status_code() {
            return Some(status);
        }

        if let Some(frequency) = self.rate_limit() {
            let rate_limiter = Arc::new(RateLimiter::new(frequency.clone()));
            ctx.set_rate_limiter(&rate_limiter);
            if let Some(status) = rate_limiter.handle_request(session, ctx).await {
                return Some(status);
            }
        }
        None
    }

    async fn modify_request(
        &self,
        session: &mut Session,
        request: &mut RequestHeader,
        ctx: &mut CTX,
    ) -> Result<()> {
        if let Some(rate_limiter) = ctx.rate_limiter() {
            rate_limiter.modify_request(session, request, ctx).await?;
        }
        if self.0.websocket {
            let upgrade = session.get_header("Upgrade");
            request.insert_header(
                "Connection",
                if upgrade.is_ok() { "upgrade" } else { "close" },
            )?;
            request.insert_header("Upgrade", upgrade.unwrap_or(String::new()))?;
        }
        Ok(())
    }

    async fn modify_response(
        &self,
        session: &mut Session,
        response: &mut ResponseHeader,
        ctx: &mut CTX,
    ) -> Result<()> {
        let origin = session.get_header("Origin")?;
        response.apply_cors(&origin, &session.req_header().method, &self.0.headers)?;
        if let Some(rate_limiter) = ctx.rate_limiter() {
            rate_limiter.modify_response(session, response, ctx).await?;
        }
        Ok(())
    }
}
