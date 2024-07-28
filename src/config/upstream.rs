use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Deserialize, Clone)]
pub struct Upstream {
    pub host: String,
    #[serde(default = "Upstream::default_port")]
    pub port: u16,
    #[serde(default = "Upstream::default_tls")]
    pub tls: bool,
}

impl Upstream {
    pub fn default_port() -> u16 {
        80
    }

    pub fn default_tls() -> bool {
        false
    }
}

impl Display for Upstream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}
