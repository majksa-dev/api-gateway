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

impl TimeUnit {
    pub fn _to_seconds(&self, amount: u32) -> u32 {
        match self {
            TimeUnit::Seconds => amount,
            TimeUnit::Minutes => amount * 60,
            TimeUnit::Hours => amount * 3600,
            TimeUnit::Days => amount * 86400,
        }
    }

    pub fn _from_seconds(&self, seconds: u32) -> u32 {
        match self {
            TimeUnit::Seconds => seconds,
            TimeUnit::Minutes => seconds / 60,
            TimeUnit::Hours => seconds / 3600,
            TimeUnit::Days => seconds / 86400,
        }
    }

    pub fn _convert(&self, amount: u32, unit: TimeUnit) -> u32 {
        unit._from_seconds(self._to_seconds(amount))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Time {
    pub amount: u32,
    pub unit: TimeUnit,
}

impl Time {
    pub fn _to_seconds(&self) -> u32 {
        self.unit._to_seconds(self.amount)
    }

    pub fn _from_seconds(seconds: u32) -> Self {
        Time {
            amount: seconds,
            unit: TimeUnit::Seconds,
        }
    }

    pub fn _convert(&self, unit: TimeUnit) -> Time {
        Time {
            amount: self.unit._convert(self.amount, unit),
            unit,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Frequency {
    pub amount: u32,
    pub interval: Time,
}
