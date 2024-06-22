use std::collections::HashMap;

use super::quota::Quota;
use gateway::cors;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Auth {
    pub token: String,
    pub origins: Vec<String>,
    pub quota: Option<Quota>,
    pub endpoints: Option<HashMap<String, Quota>>,
}

impl From<Auth> for cors::Auth {
    fn from(value: Auth) -> Self {
        Self::new(value.token, value.origins)
    }
}
