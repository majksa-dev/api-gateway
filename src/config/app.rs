use gateway::{cache, cors, rate_limit};
use serde::Deserialize;

use super::{auth::Auth, endpoint::Endpoint, quota::Quota, upstream::Upstream};

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfigRaw {
    pub upstream: Upstream,
    pub auth: Vec<Auth>,
    pub endpoints: Vec<Endpoint>,
    pub quota: Option<Quota>,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub name: String,
    pub upstream: Upstream,
    pub auth: Vec<Auth>,
    pub endpoints: Vec<Endpoint>,
    pub quota: Option<Quota>,
}

impl AppConfig {
    pub fn from_raw(data: AppConfigRaw, name: String) -> Self {
        AppConfig {
            name,
            upstream: data.upstream,
            auth: data.auth,
            endpoints: data.endpoints,
            quota: data.quota,
        }
    }
}

impl From<AppConfig> for cors::AppConfig {
    fn from(value: AppConfig) -> Self {
        Self::new(value.auth.into_iter().map(Auth::into).collect())
    }
}

impl From<AppConfig> for rate_limit::AppConfig {
    fn from(value: AppConfig) -> Self {
        Self::new(
            rate_limit::Rules {
                quota: value.quota.map(Quota::into),
                endpoints: value
                    .endpoints
                    .into_iter()
                    .filter_map(|endpoint| {
                        if let Some(quota) = endpoint.quota {
                            Some((endpoint.id, quota.into()))
                        } else {
                            None
                        }
                    })
                    .collect(),
            },
            value
                .auth
                .into_iter()
                .map(|auth| {
                    (
                        auth.token,
                        rate_limit::Rules::new(
                            auth.quota.map(Quota::into),
                            auth.endpoints
                                .unwrap_or_default()
                                .into_iter()
                                .map(|(id, quota)| (id, quota.into()))
                                .collect(),
                        ),
                    )
                })
                .collect(),
        )
    }
}

impl From<AppConfig> for cache::AppConfig {
    fn from(value: AppConfig) -> Self {
        Self::new(
            value
                .endpoints
                .into_iter()
                .filter_map(|endpoint| {
                    if let Some(cache) = endpoint.cache {
                        Some((endpoint.id, cache.into()))
                    } else {
                        None
                    }
                })
                .collect(),
        )
    }
}
