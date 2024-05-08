use gateway::cors;
use regex::Regex;
use serde::{Deserialize, Deserializer};

use crate::utils::time::Frequency;

use super::{auth::Auth, method::Method};

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub path: Regex,
    pub id: String,
    pub methods: Vec<http::Method>,
    pub auth: Vec<Auth>,
    pub rate_limit: Option<Frequency>,
    pub websocket: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct EndpointRaw {
    path: String,
    id: String,
    methods: Option<Vec<Method>>,
    auth: Option<Vec<Auth>>,
    #[serde(rename = "rate-limit")]
    rate_limit: Option<Frequency>,
    websocket: Option<bool>,
}

impl<'de> Deserialize<'de> for Endpoint {
    fn deserialize<D>(deserializer: D) -> Result<Endpoint, D::Error>
    where
        D: Deserializer<'de>,
    {
        let e = EndpointRaw::deserialize(deserializer)?;
        Ok(Endpoint {
            path: Regex::new(&e.path).unwrap(),
            id: e.id,
            methods: e
                .methods
                .unwrap_or_default()
                .into_iter()
                .map(http::Method::from)
                .collect(),
            auth: e.auth.unwrap_or_default(),
            rate_limit: e.rate_limit,
            websocket: e.websocket.unwrap_or(false),
        })
    }
}

impl From<Endpoint> for cors::ConfigRules {
    fn from(value: Endpoint) -> Self {
        Self::new(
            value.methods,
            value.auth.into_iter().map(Auth::into).collect(),
        )
    }
}
