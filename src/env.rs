use std::net::IpAddr;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Env {
    pub port: Option<u16>,
    pub healthcheck_port: Option<u16>,
    pub host: Option<IpAddr>,
    pub config_file: String,
    pub redis_cache_url: String,
    pub redis_rate_limiter_url: String,
}

impl Env {
    pub fn new() -> Result<Self, envy::Error> {
        envy::from_env()
    }
}
