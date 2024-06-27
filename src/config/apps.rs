use gateway::{auth, cache, cors, rate_limit};
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

impl From<&Apps> for cors::Builder {
    fn from(value: &Apps) -> Self {
        value
            .apps
            .iter()
            .map(|(name, app)| (name.clone(), app.into()))
            .collect()
    }
}

impl From<&Apps> for rate_limit::Builder {
    fn from(value: &Apps) -> Self {
        value
            .apps
            .iter()
            .map(|(name, app)| (name.clone(), app.into()))
            .collect()
    }
}

impl From<&Apps> for cache::Builder {
    fn from(value: &Apps) -> Self {
        value
            .apps
            .iter()
            .map(|(name, app)| {
                (
                    name.clone(),
                    HashMap::<String, cache::config::Endpoint>::from(app),
                )
            })
            .collect()
    }
}

impl From<&Apps> for auth::basic::Builder {
    fn from(value: &Apps) -> Self {
        value
            .apps
            .iter()
            .filter_map(|(name, app)| {
                Into::<Option<auth::basic::config::Auth>>::into(app).map(|app| (name.clone(), app))
            })
            .collect()
    }
}

impl From<&Apps> for auth::jwt::Builder {
    fn from(value: &Apps) -> Self {
        value
            .apps
            .iter()
            .map(|(name, app)| (name.clone(), app.into()))
            .collect()
    }
}
