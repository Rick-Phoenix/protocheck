use crate::common::Decimal;

impl Decimal {
  pub fn new(value: String) -> Self {
    Self { value }
  }
}

impl Display for Decimal {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.value)
  }
}

use std::{fmt::Display, str::FromStr};

use rust_decimal::Decimal as RustDecimal;
use thiserror::Error;

/// Errors that can occur during the creation, conversion or validation of a [`Decimal`].
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum DecimalError {
  #[error("Invalid decimal format: {0}")]
  InvalidFormat(String),
}

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
