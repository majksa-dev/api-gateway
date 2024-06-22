use gateway::{cache, cors, rate_limit};
use serde::Deserialize;
use std::collections::HashMap;

use super::app::{AppConfig, AppConfigRaw};

#[derive(Debug, Clone)]
pub struct Apps {
    pub apps: HashMap<String, AppConfig>,
}

impl Apps {
    pub fn new(data: String) -> Result<Self, serde_json::Error> {
        Ok(Self::from_raw(AppsRaw::new(data)?))
    }
    pub fn from_raw(data: AppsRaw) -> Self {
        let mut apps = HashMap::new();
        for (name, config) in data.apps {
            apps.insert(name.clone(), AppConfig::from_raw(config, name));
        }
        Apps { apps }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppsRaw {
    pub apps: HashMap<String, AppConfigRaw>,
}

impl AppsRaw {
    pub fn new(data: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&data)
    }
}

impl From<Apps> for cors::Config {
    fn from(value: Apps) -> Self {
        Self {
            config: value
                .apps
                .into_iter()
                .map(|(name, app)| (name, app.into()))
                .collect(),
        }
    }
}

impl From<Apps> for rate_limit::Config {
    fn from(value: Apps) -> Self {
        Self::new(
            value
                .apps
                .into_iter()
                .map(|(name, app)| (name, app.into()))
                .collect(),
        )
    }
}

impl From<Apps> for cache::Config {
    fn from(value: Apps) -> Self {
        Self::new(
            value
                .apps
                .into_iter()
                .map(|(name, app)| (name, app.into()))
                .collect(),
        )
    }
}
