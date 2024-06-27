use super::time::Time;
use gateway::cache;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Cache {
    pub expires_in: Time,
    pub vary_headers: Option<Vec<String>>,
}

impl From<&Cache> for cache::config::Endpoint {
    fn from(value: &Cache) -> Self {
        Self::new(
            value.expires_in.clone().into(),
            value.vary_headers.clone().unwrap_or_default(),
        )
    }
}
