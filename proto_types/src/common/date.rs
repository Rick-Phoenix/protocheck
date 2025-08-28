use std::{
  cmp::{Ord, Ordering, PartialOrd},
  convert::TryFrom,
};

use ::prost::alloc::string::String;
use thiserror::Error;

use crate::common::Date;

/// Errors that can occur during the creation, conversion or validation of a [`Date`].
#[derive(Debug, PartialEq, Eq, Error)]
pub enum DateError {
  #[error("{0}")]
  InvalidYear(String),
  #[error("{0}")]
  InvalidMonth(String),
  #[error("{0}")]
  InvalidDay(String),
  #[error("Date conversion error: {0}")]
  ConversionError(String),
}

fn validate_date(year: i32, month: i32, day: i32) -> Result<(), DateError> {
  if !(0..=9999).contains(&year) {
    return Err(DateError::InvalidYear(
      "Invalid year value (must be within 0 (to indicate a date without a specific year) and 9999)"
        .to_string(),
    ));
  }

  if !(0..=12).contains(&month) {
    return Err(DateError::InvalidMonth(
      "Invalid month value (must be within 0 (if only the year is specified) and 12)".to_string(),
    ));
  }

  if !(0..=31).contains(&day) {
    return Err(DateError::InvalidDay(
      "Invalid day value (must be within 0 (if only the year is specified) and 31)".to_string(),
    ));
  }

  if year == 0 {
    if month == 0 {
      return Err(DateError::InvalidMonth(
        "The month cannot be set to 0 if the year is also set to 0".to_string(),
      ));
    }

    if day == 0 {
      return Err(DateError::InvalidDay(
        "The day cannot be set to 0 if the year is also set to 0".to_string(),
      ));
    }
  } else if month == 0 {
    return Err(DateError::InvalidMonth(
      "The month cannot be set to 0 if the year is non-zero".to_string(),
    ));
  }

  Ok(())
}

impl Date {
  /// Creates a new `Date` instance with validation.
  /// Allows `year: 0`, `month: 0`, `day: 0` as special cases described in the proto spec.
  /// Returns an error if any component is out of range or date is invalid (e.g., February 30th).
  pub fn new(year: i32, month: i32, day: i32) -> Result<Self, DateError> {
    validate_date(year, month, day)?;

    Ok(Date { year, month, day })
  }

  /// Checks if this [`Date`] instance represents a valid date according to its constraints.
  pub fn is_valid(&self) -> bool {
    validate_date(self.year, self.month, self.day).is_ok()
  }

  pub fn has_year(&self) -> bool {
    self.year != 0
  }

  /// Returns `true` if this [`Date`] only indicates a year.
  pub fn is_year_only(&self) -> bool {
    self.year != 0 && (self.month == 0 && self.day == 0)
  }

  /// Returns `true` if this [`Date`] only indicates a year and a month (i.e. for a credit card expiration date).
  pub fn is_year_and_month(&self) -> bool {
    self.year != 0 && self.month != 0 && self.day == 0
  }

  /// Returns `true` if this [`Date`] only indicates a month and a day, with no specific year.
  pub fn is_month_and_day(&self) -> bool {
    self.year == 0 && self.month != 0 && self.day != 0
  }
}

impl PartialOrd for Date {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Date {
  fn cmp(&self, other: &Self) -> Ordering {
    self
      .year
      .cmp(&other.year)
      .then_with(|| self.month.cmp(&other.month))
      .then_with(|| self.day.cmp(&other.day))
  }
}

#[cfg(feature = "chrono")]
impl TryFrom<Date> for chrono::NaiveDate {
  type Error = DateError;

  fn try_from(date: Date) -> Result<Self, Self::Error> {
    if date.year == 0 || date.month == 0 || date.day == 0 {
      return Err(DateError::ConversionError(
        "Cannot convert Date with year=0, month=0, or day=0 to NaiveDate".to_string(),
      ));
    }

    validate_date(date.year, date.month, date.day)?;

    // Safe castings after validation
    chrono::NaiveDate::from_ymd_opt(date.year, date.month as u32, date.day as u32).ok_or_else(
      || {
        DateError::ConversionError(format!(
          "Invalid date components for NaiveDate: Y:{}, M:{}, D:{}",
          date.year, date.month, date.day
        ))
      },
    )
  }
}

#[cfg(feature = "chrono")]
impl From<chrono::NaiveDate> for Date {
  fn from(naive_date: chrono::NaiveDate) -> Self {
    use chrono::Datelike;
    // Casting is safe due to chrono's costructor API
    Date {
      year: naive_date.year(),
      month: naive_date.month() as i32,
      day: naive_date.day() as i32,
    }
  }
}
