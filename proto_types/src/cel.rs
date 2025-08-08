use std::collections::HashMap;

use cel_interpreter::{objects::Key as CelKey, Value as CelValue};
use chrono::{DateTime, FixedOffset};
use thiserror::Error;

use crate::{Any, Duration, DurationError, Timestamp, TimestampError};

#[derive(Debug, Error)]
pub enum CelConversionError {
  #[error("{0}")]
  DurationError(#[from] DurationError),

  #[error("{0}")]
  TimestampError(#[from] TimestampError),
}

impl From<Any> for CelValue {
  fn from(value: Any) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();
    cel_map.insert(
      "type_url".to_string().into(),
      CelValue::String(value.type_url.clone().into()),
    );
    cel_map.insert(
      "value".to_string().into(),
      CelValue::Bytes(value.value.clone().into()),
    );

    CelValue::Map(cel_map.into())
  }
}

impl TryFrom<Duration> for CelValue {
  type Error = CelConversionError;

  fn try_from(value: Duration) -> Result<Self, Self::Error> {
    let chrono_dur: chrono::Duration = value.try_into().map_err(CelConversionError::from)?;

    Ok(CelValue::Duration(chrono_dur))
  }
}

impl TryFrom<Timestamp> for CelValue {
  type Error = CelConversionError;

  fn try_from(value: Timestamp) -> Result<Self, Self::Error> {
    let chrono_timestamp: DateTime<FixedOffset> =
      value.try_into().map_err(CelConversionError::from)?;
    Ok(CelValue::Timestamp(chrono_timestamp))
  }
}
