use gateway::auth;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Auth {
    pub credentials: Vec<Credential>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Credential {
    pub username: String,
    pub password: String,
}

impl From<&Auth> for auth::basic::config::Auth {
    fn from(value: &Auth) -> Self {
        Self::new(
            value
                .credentials
                .iter()
                .map(|credential| auth::basic::config::Credential {
                    username: credential.username.clone(),
                    password: credential.password.clone(),
                })
                .collect(),
        )
    }
}
