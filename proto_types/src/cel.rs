use std::{collections::HashMap, convert::Infallible};

use cel::{objects::Key as CelKey, Value as CelValue};
use thiserror::Error;

use crate::{duration::DurationError, timestamp::TimestampError, Any, Empty, FieldMask};

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum CelConversionError {
  #[error("{0}")]
  DurationError(#[from] DurationError),

  #[error("{0}")]
  TimestampError(#[from] TimestampError),
}

impl From<Infallible> for CelConversionError {
  fn from(infallible: Infallible) -> Self {
    match infallible {}
  }
}

impl From<Any> for CelValue {
  fn from(value: Any) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();
    cel_map.insert("type_url".into(), CelValue::String(value.type_url.into()));
    cel_map.insert("value".into(), CelValue::Bytes(value.value.into()));

    CelValue::Map(cel_map.into())
  }
}

#[cfg(feature = "chrono")]
mod chrono {
  use cel::Value as CelValue;
  use chrono::{DateTime, FixedOffset};

  use crate::{cel::CelConversionError, Duration, Timestamp};

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
}

impl From<FieldMask> for CelValue {
  fn from(value: FieldMask) -> Self {
    let paths = value.paths;

    let mut cel_vals: Vec<CelValue> = Vec::new();
    for path in paths {
      cel_vals.push(CelValue::String(path.into()));
    }

    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();
    cel_map.insert("paths".into(), CelValue::List(cel_vals.into()));

    CelValue::Map(cel_map.into())
  }
}

impl From<&FieldMask> for CelValue {
  fn from(value: &FieldMask) -> Self {
    let paths = &value.paths;

    let mut cel_vals: Vec<CelValue> = Vec::new();
    for path in paths {
      cel_vals.push(CelValue::String(path.clone().into()));
    }

    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();
    cel_map.insert("paths".into(), CelValue::List(cel_vals.into()));

    CelValue::Map(cel_map.into())
  }
}

impl From<Empty> for CelValue {
  fn from(_: Empty) -> Self {
    CelValue::Map(HashMap::<CelKey, CelValue>::new().into())
  }
}
