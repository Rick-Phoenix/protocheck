use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum DecimalError {
  #[error("Invalid decimal format: {0}")]
  InvalidFormat(String),
}

#[cfg(feature = "decimal")]
mod decimal_impl {
  use std::str::FromStr;

  use rust_decimal::Decimal as RustDecimal;

  use crate::common::{decimal::DecimalError, Decimal};

  impl TryFrom<Decimal> for RustDecimal {
    type Error = DecimalError;
    fn try_from(value: Decimal) -> Result<Self, Self::Error> {
      RustDecimal::from_str(&value.value).map_err(|e| DecimalError::InvalidFormat(e.to_string()))
    }
  }

  impl From<RustDecimal> for Decimal {
    fn from(value: RustDecimal) -> Self {
      Decimal {
        value: value.to_string(),
      }
    }
  }
}
