use serde::Deserialize;

#[derive(Debug, Copy, Clone, Deserialize, PartialEq, Eq)]
pub enum TimeUnit {
    #[serde(rename = "seconds")]
    Seconds,
    #[serde(rename = "minutes")]
    Minutes,
    #[serde(rename = "hours")]
    Hours,
    #[serde(rename = "days")]
    Days,
}

impl From<TimeUnit> for gateway::time::TimeUnit {
    fn from(value: TimeUnit) -> Self {
        match value {
            TimeUnit::Seconds => Self::Seconds,
            TimeUnit::Minutes => Self::Minutes,
            TimeUnit::Hours => Self::Hours,
            TimeUnit::Days => Self::Days,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Time {
    pub amount: usize,
    pub unit: TimeUnit,
}

impl From<Time> for gateway::time::Time {
    fn from(value: Time) -> Self {
        Self {
            amount: value.amount,
            unit: value.unit.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Frequency {
    pub amount: usize,
    pub interval: Time,
}

impl From<Frequency> for gateway::time::Frequency {
    fn from(value: Frequency) -> Self {
        Self {
            amount: value.amount,
            interval: value.interval.into(),
        }
    }
}
