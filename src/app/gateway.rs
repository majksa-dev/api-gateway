use async_trait::async_trait;
use log::{debug, warn};
use pingora::http::{RequestHeader, ResponseHeader, StatusCode};
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

use pingora::proxy::{ProxyHttp, Session};
use pingora::upstreams::peer::HttpPeer;
use pingora::{Error, ErrorType, Result};

use crate::env::Env;
use crate::http::headers::GetHeader;

use super::context::{ContextError, CTX};
use super::request::RequestHandler;
use super::{config::Apps, context::Context, proxy::AppProxy};

#[derive(Debug)]
pub enum GatewayError {
    ReadConfigFile(std::io::Error),
    ParseConfigFile(serde_json::Error),
    ContextCreation(ContextError),
}

pub type GatewayResult<T> = std::result::Result<T, GatewayError>;

impl std::fmt::Display for GatewayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GatewayError::ReadConfigFile(e) => write!(f, "Could not read config file: {}", e),
            GatewayError::ParseConfigFile(e) => write!(f, "Could not parse config file: {}", e),
            GatewayError::ContextCreation(e) => write!(f, "Could not create context: {}", e),
        }
    }
}

pub struct Gateway {
    apps: HashMap<String, Arc<AppProxy>>,
    ctx: Arc<Context>,
}

impl Gateway {
    pub fn new(env: &Env) -> GatewayResult<Self> {
        Ok(Gateway {
            apps: fs::read_to_string(&env.config)
                .map_err(GatewayError::ReadConfigFile)
                .and_then(|config| Apps::new(config).map_err(GatewayError::ParseConfigFile))?
                .apps
                .into_iter()
                .filter_map(|(name, app)| {
                    AppProxy::new(app)
                        .map(|app_proxy| (name, app_proxy))
                        .map_err(|e| warn!("Could not create proxy for {}", e))
                        .ok()
                })
                .map(|(name, app)| (name, Arc::new(app)))
                .collect(),
            ctx: Context::new(env).map_err(GatewayError::ContextCreation)?,
        })
    }

    fn get_proxy(&self, req: &RequestHeader) -> Result<Arc<AppProxy>> {
        self.apps
            .get(&req.get_header("Host")?)
            .cloned()
            .ok_or(Error::new(ErrorType::ConnectProxyFailure))
    }
}

#[async_trait]
impl ProxyHttp for Gateway {
    type CTX = CTX;

    fn new_ctx(&self) -> Self::CTX {
        CTX::new(&self.ctx)
    }

    async fn request_filter(&self, session: &mut Session, ctx: &mut Self::CTX) -> Result<bool> {
        let response = match self.get_proxy(session.req_header()) {
            Ok(app) => {
                debug!("Selected app: {}", app.config.name);
                ctx.select_app(&app);
                app.handle_request(session, ctx).await
            }
            Err(e) => {
                warn!("Could not get proxy: {}", e);
                Some(StatusCode::BAD_GATEWAY)
            }
        };
        match response {
            Some(status) => {
                session.respond_error(status.as_u16()).await;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        Ok(ctx.app().upstream_peer().await)
    }

    async fn upstream_request_filter(
        &self,
        session: &mut Session,
        upstream_request: &mut RequestHeader,
        ctx: &mut Self::CTX,
    ) -> Result<()> {
        ctx.app()
            .modify_request(session, upstream_request, ctx)
            .await
    }

    async fn response_filter(
        &self,
        session: &mut Session,
        upstream_response: &mut ResponseHeader,
        ctx: &mut Self::CTX,
    ) -> Result<()>
    where
        Self::CTX: Send + Sync,
    {
        upstream_response
            .insert_header("Server", "ApiGateway")
            .unwrap();
        upstream_response.remove_header("alt-svc");
        ctx.app()
            .modify_response(session, upstream_response, ctx)
            .await
    }
}
