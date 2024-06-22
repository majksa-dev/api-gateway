use gateway::rate_limit;
use serde::Deserialize;

use super::time::Frequency;

#[derive(Debug, Clone, Deserialize)]
pub struct Quota {
    pub total: Frequency,
    pub user: Option<Frequency>,
}

impl From<Quota> for rate_limit::Quota {
    fn from(value: Quota) -> Self {
        Self {
            total: value.total.into(),
            user: value.user.map(Frequency::into),
        }
    }
}
