pub mod basic;
pub mod jwt;

use super::quota::Quota;
use gateway::cors;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Auth {
    pub token: String,
    pub origins: Vec<String>,
    pub quota: Option<Quota>,
}

impl From<&Auth> for cors::config::Auth {
    fn from(value: &Auth) -> Self {
        Self::new(value.token.clone(), Some(value.origins.clone()))
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct EndpointAuth {
    pub token: String,
    pub origins: Vec<String>,
    pub quota: Option<Quota>,
}
