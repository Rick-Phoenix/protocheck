use cel_interpreter::Value as CelValue;
use chrono::{DateTime, FixedOffset};
use thiserror::Error;

use crate::{Duration, DurationError, Timestamp, TimestampError};

#[derive(Debug, Error)]
pub enum CelConversionError {
  #[error("{0}")]
  DurationError(#[from] DurationError),

  #[error("{0}")]
  TimestampError(#[from] TimestampError),
}

impl TryFrom<Duration> for CelValue {
  type Error = CelConversionError;

  fn try_from(value: Duration) -> Result<Self, Self::Error> {
    let chrono_dur: chrono::Duration = value.try_into()?;

    Ok(CelValue::Duration(chrono_dur))
  }
}

impl TryFrom<Timestamp> for CelValue {
  type Error = CelConversionError;

  fn try_from(value: Timestamp) -> Result<Self, Self::Error> {
    let chrono_timestamp: DateTime<FixedOffset> = value.try_into()?;
    Ok(CelValue::Timestamp(chrono_timestamp))
  }
}
