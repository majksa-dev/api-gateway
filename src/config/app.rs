use gateway::cors;
use serde::Deserialize;

use super::{auth::Auth, endpoint::Endpoint, method::Method, upstream::Upstream};

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfigRaw {
    pub upstream: Upstream,
    pub auth: Vec<Auth>,
    pub methods: Option<Vec<Method>>,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub name: String,
    pub upstream: Upstream,
    pub auth: Vec<Auth>,
    pub methods: Vec<http::Method>,
    pub endpoints: Vec<Endpoint>,
}

impl AppConfig {
    pub fn from_raw(data: AppConfigRaw, name: String) -> Self {
        AppConfig {
            name,
            upstream: data.upstream,
            methods: data
                .methods
                .unwrap_or_default()
                .into_iter()
                .map(http::Method::from)
                .collect(),
            auth: data.auth,
            endpoints: data.endpoints,
        }
    }
}

impl From<AppConfig> for cors::AppConfig {
    fn from(value: AppConfig) -> Self {
        let endpoints = value
            .endpoints
            .iter()
            .cloned()
            .map(|endpoint| (endpoint.id.clone(), endpoint.into()))
            .collect();
        Self::new(value.into(), endpoints)
    }
}

impl From<AppConfig> for cors::ConfigRules {
    fn from(value: AppConfig) -> Self {
        Self::new(vec![], value.auth.into_iter().map(Auth::into).collect())
    }
}
