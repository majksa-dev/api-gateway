use pingora::{
    http::{RequestHeader, ResponseHeader, StatusCode},
    proxy::Session,
    Result,
};

use crate::{app::request::ToStatusCode, http::headers::GetHeader, utils::time::Frequency};

use super::{context::CTX, request::RequestHandler};

#[derive(Debug, Clone)]
pub struct RateLimit {
    pub limit: u32,
    pub remaining: u32,
    pub reset: u32,
}

#[derive(Debug, Clone)]
pub struct RateLimiter(Frequency);

impl RateLimiter {
    pub fn new(frequency: Frequency) -> Self {
        RateLimiter(frequency)
    }
}

impl RequestHandler for RateLimiter {
    async fn handle_request(&self, session: &mut Session, ctx: &mut CTX) -> Option<StatusCode> {
        let ip = session.req_header().get_header("X-Real-IP");
        if ip.is_err() {
            return None;
        }
        let ip = ip.unwrap();
        let key = format!("{}--{}--{}", ctx.app().config.name, ctx.endpoint().id(), ip);
        let redis = ctx.redis();
        let rate_limit = redis::pipe()
            .atomic()
            .cmd("SET")
            .arg(&key)
            .arg(0)
            .arg("EX")
            .arg(self.0.interval.to_seconds())
            .arg("NX")
            .ignore()
            .cmd("INCR")
            .arg(&key)
            .cmd("TTL")
            .arg(&key)
            .query(redis)
            .and_then(|(count, ttl): (u32, u32)| {
                Ok(RateLimit {
                    limit: self.0.amount,
                    remaining: if count > self.0.amount {
                        0
                    } else {
                        self.0.amount - count
                    },
                    reset: chrono::Utc::now()
                        .timestamp()
                        .checked_add(ttl as i64)
                        .unwrap_or(0) as u32,
                })
            })
            .to_status_code();
        if let Err(status) = rate_limit {
            return Some(status);
        }
        let rate_limit = rate_limit.unwrap();
        let is_rate_limited = rate_limit.remaining == 0;
        ctx.set_rate_limit(rate_limit);
        if is_rate_limited {
            Some(StatusCode::TOO_MANY_REQUESTS)
        } else {
            None
        }
    }

    async fn modify_request(
        &self,
        _session: &mut Session,
        _request: &mut RequestHeader,
        _ctx: &mut CTX,
    ) -> Result<()> {
        Ok(())
    }

    async fn modify_response(
        &self,
        _session: &mut Session,
        response: &mut ResponseHeader,
        ctx: &mut CTX,
    ) -> Result<()> {
        if let Some(rate_limit) = ctx.rate_limit() {
            response.insert_header("X-RateLimit-Limit", rate_limit.limit)?;
            response.insert_header("X-RateLimit-Remaining", rate_limit.remaining)?;
            response.insert_header("X-RateLimit-Reset", rate_limit.reset)?;
        }
        Ok(())
    }
}
