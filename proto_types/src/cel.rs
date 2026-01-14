#![allow(clippy::std_instead_of_core)]
use std::{collections::HashMap, convert::Infallible};

use cel::{Value as CelValue, objects::Key as CelKey};
use thiserror::Error;

use crate::{Any, Empty, FieldMask, Vec, duration::DurationError, timestamp::TimestampError};

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
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();
    cel_map.insert("type_url".into(), Self::String(value.type_url.into()));
    cel_map.insert("value".into(), Self::Bytes(value.value.into()));

    Self::Map(cel_map.into())
  }
}

#[cfg(feature = "chrono")]
mod chrono {
  use cel::Value as CelValue;
  use chrono::{DateTime, FixedOffset};

  use crate::{Duration, Timestamp, cel::CelConversionError};

  impl TryFrom<Duration> for CelValue {
    type Error = CelConversionError;

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
      let chrono_dur: chrono::Duration = value
        .try_into()
        .map_err(CelConversionError::from)?;

      Ok(Self::Duration(chrono_dur))
    }
  }

  impl TryFrom<Timestamp> for CelValue {
    type Error = CelConversionError;

    fn try_from(value: Timestamp) -> Result<Self, Self::Error> {
      let chrono_timestamp: DateTime<FixedOffset> = value
        .try_into()
        .map_err(CelConversionError::from)?;
      Ok(Self::Timestamp(chrono_timestamp))
    }
  }
}

impl From<FieldMask> for CelValue {
  fn from(value: FieldMask) -> Self {
    let paths = value.paths;

    let mut cel_vals: Vec<Self> = Vec::new();
    for path in paths {
      cel_vals.push(Self::String(path.into()));
    }

    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();
    cel_map.insert("paths".into(), Self::List(cel_vals.into()));

    Self::Map(cel_map.into())
  }
}

impl From<&FieldMask> for CelValue {
  fn from(value: &FieldMask) -> Self {
    let paths = &value.paths;

    let mut cel_vals: Vec<Self> = Vec::new();
    for path in paths {
      cel_vals.push(Self::String(path.clone().into()));
    }

    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();
    cel_map.insert("paths".into(), Self::List(cel_vals.into()));

    Self::Map(cel_map.into())
  }
}

impl From<Empty> for CelValue {
  fn from(_: Empty) -> Self {
    Self::Map(HashMap::<CelKey, Self>::new().into())
  }
}
