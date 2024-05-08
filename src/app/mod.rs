use std::{
    net::{IpAddr, SocketAddr},
    path::Path,
};

use crate::config::{app::AppConfig, apps::Apps};
use ::gateway::{builder, Server, TcpOrigin};
use bb8_redis::{bb8::Pool, RedisConnectionManager};
use essentials::error;
use gateway::{cors, Request};
use http::header;
use tokio::fs;

use crate::env::Env;

type Result<T> = std::result::Result<T, String>;

async fn create_redis(connection: String) -> Result<Pool<RedisConnectionManager>> {
    let manager = RedisConnectionManager::new(connection)
        .map_err(|e| format!("Failed to create Redis connection manager: {}", e))?;
    Pool::builder()
        .build(manager)
        .await
        .map_err(|e| format!("Failed to create Redis connection pool: {}", e))
}

async fn load_config(config_path: impl AsRef<Path>) -> Result<Apps> {
    let config_data = fs::read_to_string(config_path)
        .await
        .map_err(|e| format!("Failed to read config file: {}", e))?;
    Apps::new(config_data).map_err(|e| format!("Failed to parse config file: {}", e))
}

fn create_origin(config: &Apps) -> TcpOrigin {
    TcpOrigin::new(
        config
            .apps
            .iter()
            .filter_map(|(name, app)| {
                Some((
                    name.clone(),
                    Box::new(
                        TryInto::<SocketAddr>::try_into(&app.upstream)
                            .map_err(|e| error!("Failed to create upstream: {}", e))
                            .ok()?,
                    ),
                ))
            })
            .collect(),
    )
}

fn peer_key_from_host() -> impl Fn(&Request) -> Option<String> + Send + Sync + 'static {
    |req: &Request| {
        req.headers
            .get(header::HOST)
            .and_then(|host| host.to_str().ok())
            .map(|host| host.to_string())
    }
}

fn endpoint_key_from_host(
    config: AppConfig,
) -> impl Fn(&Request) -> Option<String> + Send + Sync + 'static {
    let endpoints = config
        .endpoints
        .into_iter()
        .map(|endpoint| (endpoint.id, endpoint.path))
        .collect::<Vec<_>>();
    move |req: &Request| {
        endpoints
            .iter()
            .find(|(_, path)| path.is_match(&req.path))
            .map(|(id, _)| id.clone())
    }
}

pub async fn build(env: Env) -> Result<Server> {
    let redis_cache = create_redis(env.redis_cache_url).await?;
    let redis_rate_limiter = create_redis(env.redis_rate_limiter_url).await?;
    let config = load_config(env.config_file).await?;
    let mut builder = builder(create_origin(&config), peer_key_from_host())
        .with_app_port(env.port.unwrap_or(80))
        .with_health_check_port(env.healthcheck_port.unwrap_or(9000))
        .with_host(env.host.unwrap_or(IpAddr::from([127, 0, 0, 1])))
        .register_middleware(1, cors::Middleware(config.clone().into()));
    for (peer, config) in config.apps.clone().into_iter() {
        builder = builder.register_peer(peer, endpoint_key_from_host(config));
    }
    Ok(builder.build())
}
