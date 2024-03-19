use std::{net::ToSocketAddrs, sync::Arc};

use log::error;
use pingora::{
    http::{RequestHeader, ResponseHeader, StatusCode},
    proxy::Session,
    upstreams::peer::HttpPeer,
    Error, ErrorType, Result,
};

use crate::http::cors::CheckCors;

use super::{config::AppConfig, context::CTX, endpoint::Endpoint, request::RequestHandler};

#[derive(Debug, Clone)]
pub struct AppProxy {
    pub config: AppConfig,
    pub endpoints: Vec<Arc<Endpoint>>,
    pub peer: Box<HttpPeer>,
}

impl AppProxy {
    pub fn new(config: AppConfig) -> Result<Self> {
        Ok(AppProxy {
            peer: Box::new(HttpPeer::new(
                format!("{}:{}", config.upstream.host, config.upstream.port)
                    .to_socket_addrs()
                    .map_err(|e| {
                        error!("Could not resolve upstream address: {}", e);
                        Error::new(ErrorType::ConnectProxyFailure)
                    })?
                    .next()
                    .ok_or_else(|| Error::new(ErrorType::ConnectProxyFailure))?,
                config.upstream.tls,
                config.upstream.host.clone(),
            )),
            endpoints: config
                .endpoints
                .iter()
                .map(|endpoint| Endpoint::new(endpoint))
                .map(Arc::new)
                .collect(),
            config,
        })
    }

    pub async fn upstream_peer(&self) -> Box<HttpPeer> {
        self.peer.clone()
    }

    pub fn get_endpoint(&self, req: &RequestHeader) -> Option<Arc<Endpoint>> {
        self.endpoints
            .iter()
            .find(|endpoint| endpoint.is_match(req))
            .cloned()
    }
}

impl RequestHandler for AppProxy {
    async fn handle_request(&self, session: &mut Session, ctx: &mut CTX) -> Option<StatusCode> {
        let status = match self.get_endpoint(session.req_header()) {
            Some(endpoint) => {
                ctx.select_endpoint(&endpoint);
                endpoint.handle_request(session, ctx).await
            }
            None => Some(StatusCode::NOT_FOUND),
        };
        if status.is_some() {
            return status;
        }
        if !session.check_cors(
            &self
                .config
                .auth
                .iter()
                .map(|auth| (&auth.token, &auth.origins))
                .collect(),
        ) {
            return Some(StatusCode::UNAUTHORIZED);
        }
        None
    }

    async fn modify_request(
        &self,
        session: &mut Session,
        request: &mut RequestHeader,
        ctx: &mut CTX,
    ) -> Result<()> {
        request.insert_header(
            "Host",
            format!(
                "{}:{}",
                self.config.upstream.host, self.config.upstream.port
            ),
        )?;
        self.get_endpoint(session.req_header())
            .ok_or(Error::new(ErrorType::ConnectProxyFailure))?
            .modify_request(session, request, ctx)
            .await
    }

    async fn modify_response(
        &self,
        session: &mut Session,
        response: &mut ResponseHeader,
        ctx: &mut CTX,
    ) -> Result<()> {
        self.get_endpoint(session.req_header())
            .ok_or(Error::new(ErrorType::ConnectProxyFailure))?
            .modify_response(session, response, ctx)
            .await
    }
}
