use std::collections::HashMap;

use gateway::{auth, cache, cors, rate_limit};
use serde::Deserialize;

use super::{
    auth::{basic, jwt, Auth},
    endpoint::Endpoint,
    quota::Quota,
    upstream::Upstream,
};

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfigRaw {
    pub upstream: Upstream,
    pub auth: Option<Vec<Auth>>,
    pub endpoints: Vec<Endpoint>,
    pub quota: Option<Quota>,
    pub jwt: Option<Vec<jwt::Auth>>,
    pub basic: Option<basic::Auth>,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub name: String,
    pub upstream: Upstream,
    pub auth: Option<Vec<Auth>>,
    pub endpoints: Vec<Endpoint>,
    pub quota: Option<Quota>,
    pub jwt: Option<Vec<jwt::Auth>>,
    pub basic: Option<basic::Auth>,
}

impl AppConfig {
    pub fn from_raw(data: AppConfigRaw, name: String) -> Self {
        AppConfig {
            name,
            upstream: data.upstream,
            auth: data.auth,
            endpoints: data.endpoints,
            quota: data.quota,
            jwt: data.jwt,
            basic: data.basic,
        }
    }
}

impl From<&AppConfig> for cors::config::AppConfig {
    fn from(value: &AppConfig) -> Self {
        Self::new(
            value
                .auth
                .clone()
                .unwrap_or_default()
                .iter()
                .map(cors::config::Auth::from)
                .collect(),
        )
    }
}

impl From<&AppConfig>
    for (
        rate_limit::config::Rules,
        HashMap<String, rate_limit::config::Rules>,
    )
{
    fn from(value: &AppConfig) -> Self {
        (
            rate_limit::config::Rules::new(
                value.quota.as_ref().map(rate_limit::config::Quota::from),
                value
                    .auth
                    .clone()
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|auth| {
                        auth.quota
                            .as_ref()
                            .map(rate_limit::config::Quota::from)
                            .map(|quota| (auth.token, quota))
                    })
                    .collect(),
            ),
            value
                .endpoints
                .iter()
                .map(|endpoint| {
                    (
                        endpoint.id.clone(),
                        rate_limit::config::Rules::new(
                            endpoint.quota.as_ref().map(rate_limit::config::Quota::from),
                            endpoint
                                .auth
                                .clone()
                                .unwrap_or_default()
                                .into_iter()
                                .filter_map(|auth| {
                                    auth.quota
                                        .as_ref()
                                        .map(rate_limit::config::Quota::from)
                                        .map(|quota| (auth.token, quota))
                                })
                                .collect(),
                        ),
                    )
                })
                .collect(),
        )
    }
}

impl From<&AppConfig> for HashMap<String, cache::config::Endpoint> {
    fn from(value: &AppConfig) -> Self {
        value
            .endpoints
            .iter()
            .filter_map(|endpoint| {
                endpoint
                    .cache
                    .as_ref()
                    .map(|cache| (endpoint.id.clone(), cache::config::Endpoint::from(cache)))
            })
            .collect()
    }
}

impl From<&AppConfig> for Option<auth::basic::config::Auth> {
    fn from(value: &AppConfig) -> Self {
        Some(auth::basic::config::Auth::from(value.basic.as_ref()?))
    }
}
