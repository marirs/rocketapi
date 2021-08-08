use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{str::FromStr, time::Duration};
use strum::{Display, EnumString};
use throttle::Throttle;

#[derive(thiserror::Error, Debug)]
pub enum RateTimeError {
    #[error("Invalid rate time unit")]
    InvalidUnit(#[from] strum::ParseError),
    #[error("Invalid format: expected 'usize/<sec|min|hour|day>'")]
    InvalidFormat,
    #[error(transparent)]
    ParseError(#[from] std::num::ParseIntError),
}

#[derive(Debug, EnumString, Display, Clone)]
#[strum(serialize_all = "snake_case", ascii_case_insensitive)]
pub enum RateUnit {
    Sec,
    Min,
    Hour,
    Day,
}

#[derive(Debug, Clone, derive_more::Display)]
#[display(fmt = "{}/{}", frequency, unit)]
pub struct RateTime {
    frequency: usize,
    unit: RateUnit,
}

impl Serialize for RateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for RateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::from_str(String::deserialize(deserializer)?.as_str())
            .map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}

impl From<RateTime> for Throttle {
    fn from(rate: RateTime) -> Self {
        let duration = match rate.unit {
            RateUnit::Sec => Duration::from_secs(1),
            RateUnit::Min => Duration::from_secs(60),
            RateUnit::Hour => Duration::from_secs(3600),
            RateUnit::Day => Duration::from_secs(86400),
        };

        Self::new(duration, rate.frequency)
    }
}

impl FromStr for RateTime {
    type Err = RateTimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.split('/').collect::<Vec<_>>()[..] {
            [frequency, unit] => Ok(Self {
                frequency: frequency.parse::<usize>()?,
                unit: RateUnit::from_str(unit)?,
            }),
            _ => Err(RateTimeError::InvalidFormat),
        }
    }
}
