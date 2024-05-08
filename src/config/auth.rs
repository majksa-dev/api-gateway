use gateway::cors;
use serde::Deserialize;

use crate::utils::time::Frequency;

#[derive(Debug, Deserialize, Clone)]
pub struct Auth {
    pub token: String,
    pub origins: Vec<String>,
    pub quota: Option<Quota>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Quota {
    pub total: Option<Frequency>,
    pub user: Option<Frequency>,
}

impl From<Auth> for cors::Auth {
    fn from(value: Auth) -> Self {
        Self::new(value.token, value.origins)
    }
}
