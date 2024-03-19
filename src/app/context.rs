use std::sync::Arc;

use log::error;
use pingora::http::StatusCode;

use crate::env::Env;

use super::{
    endpoint::Endpoint,
    proxy::AppProxy,
    rate_limit::{RateLimit, RateLimiter},
    request::ToStatusCode,
};

pub struct Context {
    redis: redis::Client,
}

#[derive(Default)]
pub struct ConnectionContext {
    pub redis: Option<redis::Connection>,
    pub app: Option<Arc<AppProxy>>,
    pub endpoint: Option<Arc<Endpoint>>,
    pub rate_limit: Option<RateLimit>,
    pub rate_limiter: Option<Arc<RateLimiter>>,
}

pub struct CTX(Arc<Context>, ConnectionContext);

#[derive(Debug)]
pub enum ContextError {
    RedisConnection(redis::RedisError),
}

impl std::fmt::Display for ContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextError::RedisConnection(e) => write!(f, "Could not connect to redis: {}", e),
        }
    }
}

pub type ContextResult<T> = std::result::Result<T, ContextError>;

impl<T> ToStatusCode<T> for ContextResult<T> {
    fn to_status_code(self) -> Result<T, StatusCode> {
        self.map_err(|e| {
            error!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
    }
}

impl Context {
    pub fn new(env: &Env) -> ContextResult<Arc<Context>> {
        Ok(Arc::new(Context {
            redis: redis::Client::open(env.redis.as_str())
                .map_err(ContextError::RedisConnection)?,
        }))
    }
}

impl ConnectionContext {
    fn new() -> Self {
        ConnectionContext::default()
    }
}

impl CTX {
    pub fn new(ctx: &Arc<Context>) -> Self {
        CTX(ctx.clone(), ConnectionContext::new())
    }

    pub fn select_app(&mut self, app: &Arc<AppProxy>) {
        self.1.app = Some(app.clone());
    }

    pub fn select_endpoint(&mut self, endpoint: &Arc<Endpoint>) {
        self.1.endpoint = Some(endpoint.clone());
    }

    pub fn connect(&mut self) -> ContextResult<()> {
        self.1.redis = Some(
            self.0
                .redis
                .get_connection()
                .map_err(ContextError::RedisConnection)?,
        );
        Ok(())
    }

    pub fn set_rate_limit(&mut self, rate_limit: RateLimit) {
        self.1.rate_limit = Some(rate_limit);
    }

    pub fn set_rate_limiter(&mut self, rate_limiter: &Arc<RateLimiter>) {
        self.1.rate_limiter = Some(rate_limiter.clone());
    }

    pub fn redis(&mut self) -> &mut redis::Connection {
        self.1.redis.as_mut().unwrap()
    }

    pub fn app(&self) -> Arc<AppProxy> {
        self.1.app.as_ref().cloned().unwrap()
    }

    pub fn endpoint(&self) -> Arc<Endpoint> {
        self.1.endpoint.as_ref().cloned().unwrap()
    }

    pub fn rate_limit(&self) -> Option<&RateLimit> {
        self.1.rate_limit.as_ref()
    }

    pub fn rate_limiter(&self) -> Option<Arc<RateLimiter>> {
        self.1.rate_limiter.clone()
    }
}
