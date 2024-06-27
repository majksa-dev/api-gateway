use gateway::auth;
use serde::Deserialize;
use url::Url;

use crate::config::app::AppConfig;

#[derive(Debug, Deserialize, Clone)]
pub struct Auth {
    pub keys_url: Url,
    pub claims: Vec<Claim>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Claim {
    pub claim: String,
    pub header: String,
}

impl From<&AppConfig> for auth::jwt::config::App {
    fn from(value: &AppConfig) -> Self {
        match &value.jwt {
            Some(rules) => Self::new(rules.iter().map(auth::jwt::config::Auth::from).collect()),
            None => Self::new(vec![]),
        }
    }
}

impl From<&Auth> for auth::jwt::config::Auth {
    fn from(value: &Auth) -> Self {
        Self::new(
            value.keys_url.clone(),
            value
                .claims
                .iter()
                .map(|claim| auth::jwt::config::Claim {
                    claim: claim.claim.clone(),
                    header: claim.header.clone(),
                })
                .collect(),
        )
    }
}
