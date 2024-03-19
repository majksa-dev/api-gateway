use std::collections::HashMap;

use regex::Regex;
use serde::{Deserialize, Deserializer};

use crate::utils::time::Frequency;

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

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfigRaw {
    pub upstream: Upstream,
    pub auth: Vec<Auth>,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub name: String,
    pub upstream: Upstream,
    pub auth: Vec<Auth>,
    pub endpoints: Vec<Endpoint>,
}

impl AppConfig {
    pub fn from_raw(data: AppConfigRaw, name: String) -> Self {
        AppConfig {
            name,
            upstream: data.upstream,
            auth: data.auth,
            endpoints: data.endpoints,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Upstream {
    pub host: String,
    pub port: u16,
    pub tls: bool,
}

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

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub path: Regex,
    pub id: String,
    pub method: pingora::http::Method,
    pub headers: Vec<String>,
    pub rate_limit: Option<Frequency>,
    pub websocket: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub enum Method {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "OPTIONS")]
    Options,
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "CONNECT")]
    Connect,
    #[serde(rename = "TRACE")]
    Trace,
}

impl Method {
    pub fn to_pingora(&self) -> pingora::http::Method {
        match self {
            Method::Get => pingora::http::Method::GET,
            Method::Post => pingora::http::Method::POST,
            Method::Put => pingora::http::Method::PUT,
            Method::Delete => pingora::http::Method::DELETE,
            Method::Patch => pingora::http::Method::PATCH,
            Method::Options => pingora::http::Method::OPTIONS,
            Method::Head => pingora::http::Method::HEAD,
            Method::Connect => pingora::http::Method::CONNECT,
            Method::Trace => pingora::http::Method::TRACE,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct EndpointRaw {
    path: String,
    id: String,
    method: Method,
    headers: Option<Vec<String>>,
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
            method: e.method.to_pingora(),
            headers: e.headers.unwrap_or_else(Vec::new),
            rate_limit: e.rate_limit,
            websocket: e.websocket.unwrap_or(false),
        })
    }
}
