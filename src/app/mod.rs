use crate::config::apps::Apps;
use crate::env::Env;
use anyhow::{Context, Result};
use bb8_redis::{bb8::Pool, RedisConnectionManager};
use gateway::{
    self, auth, cache, cors, http::HeaderMapExt, rate_limit, tcp, ParamRouterBuilder, Request,
    Server,
};
use http::header;
use std::{net::IpAddr, path::Path};
use tokio::fs;

async fn create_redis(connection: String) -> Result<Pool<RedisConnectionManager>> {
    let manager = RedisConnectionManager::new(connection)
        .with_context(|| "Failed to create Redis connection manager")?;
    Pool::builder()
        .build(manager)
        .await
        .with_context(|| "Failed to create Redis connection pool")
}

async fn load_config(config_path: impl AsRef<Path>) -> Result<Apps> {
    let config_data = fs::read_to_string(config_path)
        .await
        .with_context(|| "Failed to read config file")?;
    Apps::new(config_data).with_context(|| "Failed to parse config file")
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
    let mut builder = gateway::builder(
        tcp::Builder::build(
            config
                .apps
                .iter()
                .map(|(name, app)| {
                    (
                        name.clone(),
                        tcp::config::Connection::new(app.upstream.to_string()),
                    )
                })
                .collect(),
        ),
        peer_key_from_host(),
    )
    .with_app_port(env.port.unwrap_or(80))
    .with_health_check_port(env.healthcheck_port.unwrap_or(9000))
    .with_host(env.host.unwrap_or(IpAddr::from([127, 0, 0, 1])))
    .register_middleware(1, cors::Builder::build((&config).into()))
    .register_middleware(
        2,
        rate_limit::Builder::build(
            (&config).into(),
            rate_limit::datastore::RedisDatastore::new(redis_rate_limiter),
        ),
    )
    .register_middleware(3, auth::basic::Builder::build((&config).into()))
    .register_middleware(4, auth::jwt::Builder::build((&config).into()))
    .register_middleware(5, auth::endpoint::Builder::build((&config).into()))
    .register_middleware(
        6,
        cache::Builder::build(
            (&config).into(),
            cache::datastore::RedisDatastore::new(redis_cache),
        ),
    );
    for (peer, config) in config.apps.into_iter() {
        builder = builder.register_peer(
            peer,
            config
                .endpoints
                .into_iter()
                .map(|endpoint| (endpoint.method.into(), endpoint.path, endpoint.id))
                .collect::<ParamRouterBuilder>(),
        );
    }
    builder.build().await
}
