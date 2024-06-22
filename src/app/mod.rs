use std::{
    net::{IpAddr, SocketAddr},
    path::Path,
};

use crate::config::apps::Apps;
use ::gateway::{builder, Server, TcpOrigin};
use bb8_redis::{bb8::Pool, RedisConnectionManager};
use essentials::error;
use gateway::{cache, cors, http::HeaderMapExt, rate_limit, ParamRouter, Request};
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
        req.header(header::HOST)
            .and_then(|host| host.to_str().ok())
            .map(|host| host.to_string())
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
        .register_middleware(1, cors::Middleware::new(config.clone().into()))
        .register_middleware(
            2,
            rate_limit::Middleware::new(
                config.clone().into(),
                rate_limit::RedisDatastore::new(redis_rate_limiter),
            ),
        )
        .register_middleware(
            3,
            cache::Middleware::new(
                config.clone().into(),
                cache::RedisDatastore::new(redis_cache),
            ),
        );
    for (peer, config) in config.apps.clone().into_iter() {
        let mut router = ParamRouter::new();
        for endpoint in config.endpoints.iter() {
            router = router.add_route(
                endpoint.method.clone().into(),
                endpoint.path.clone(),
                endpoint.id.clone(),
            );
        }
        builder = builder.register_peer(peer, router);
    }
    Ok(builder.build())
}
