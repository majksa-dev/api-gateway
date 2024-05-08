use serde::Deserialize;
use std::net::{AddrParseError, SocketAddr};

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

impl TryFrom<&Upstream> for SocketAddr {
    type Error = AddrParseError;

    fn try_from(value: &Upstream) -> Result<Self, Self::Error> {
        Ok(SocketAddr::new(value.host.parse()?, value.port))
    }
}
