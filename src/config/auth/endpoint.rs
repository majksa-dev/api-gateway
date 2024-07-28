use std::collections::HashMap;

use gateway::auth;
use serde::Deserialize;
use url::Url;

use crate::config::{app::AppConfig, endpoint::Endpoint};

#[derive(Debug, Deserialize, Clone)]
pub struct App {
    pub rules: Vec<Auth>,
    #[serde(default)]
    pub roles: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Auth {
    pub url: Url,
    #[serde(default)]
    pub claims: Vec<Claim>,
    pub roles_claim: Option<RolesClaims>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Claim {
    pub claim: String,
    pub header: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RolesClaims {
    pub claim: String,
    pub mapping: Option<String>,
}

impl From<&AppConfig>
    for Option<(
        auth::endpoint::config::App,
        HashMap<String, auth::endpoint::config::Endpoint>,
    )>
{
    fn from(value: &AppConfig) -> Self {
        Some((
            value.endpoint_auth.as_ref()?.into(),
            value
                .endpoints
                .iter()
                .filter_map(|endpoint| {
                    Option::<_>::from(endpoint).map(|app| (endpoint.id.clone(), app))
                })
                .collect(),
        ))
    }
}

impl From<&App> for auth::endpoint::config::App {
    fn from(value: &App) -> Self {
        Self::new(
            value.rules.iter().map(Into::into).collect(),
            if value.roles.is_empty() {
                None
            } else {
                Some(value.roles.clone())
            },
        )
    }
}

impl From<&Auth> for auth::endpoint::config::Auth {
    fn from(value: &Auth) -> Self {
        Self::new(
            value.url.clone(),
            value.claims.iter().map(Into::into).collect(),
            value.roles_claim.as_ref().map(Into::into),
        )
    }
}

impl From<&Endpoint> for Option<auth::endpoint::config::Endpoint> {
    fn from(value: &Endpoint) -> Self {
        value
            .roles
            .clone()
            .map(auth::endpoint::config::Endpoint::new)
    }
}

impl From<&Claim> for auth::endpoint::config::Claim {
    fn from(value: &Claim) -> Self {
        Self {
            claim: value.claim.clone(),
            header: value.header.clone(),
        }
    }
}

impl From<&RolesClaims> for auth::endpoint::config::RolesClaims {
    fn from(value: &RolesClaims) -> Self {
        Self {
            claim: value.claim.clone(),
            inner_mapping: value.mapping.clone(),
        }
    }
}
