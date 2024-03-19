use std::{env, num::ParseIntError};

#[derive(Debug, Clone)]
pub struct Env {
    pub port: u16,
    pub health_check_port: u16,
    pub config: String,
    pub redis: String,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidPort(ParseIntError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidPort(e) => write!(f, "Invalid port: {}", e),
        }
    }
}

impl Env {
    pub fn new() -> Result<Self> {
        Ok(Env {
            port: env::var("PORT")
                .unwrap_or_else(|_| "80".to_string())
                .parse()
                .map_err(Error::InvalidPort)?,
            health_check_port: env::var("HEALTHCHECK_PORT")
                .unwrap_or_else(|_| "9000".to_string())
                .parse()
                .map_err(Error::InvalidPort)?,
            config: env::var("CONFIG_FILE")
                .unwrap_or_else(|_| "/etc/gateway/config.json".to_string()),
            redis: env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string()),
        })
    }
}
