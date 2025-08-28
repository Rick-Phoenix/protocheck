use std::{
  cmp::{Ord, Ordering, PartialOrd},
  convert::TryFrom,
};

use ::prost::alloc::string::String;
use thiserror::Error;

use crate::common::Date;

/// Errors that can occur during Date operations and conversions.
#[derive(Debug, PartialEq, Eq, Error)]
pub enum DateError {
  #[error("Date has an invalid component (year, month, or day out of range)")]
  InvalidDateComponent,
  #[error("Date conversion error: {0}")]
  ConversionError(String),
}

impl Date {
  /// Creates a new `Date` instance with validation.
  /// Allows `year: 0`, `month: 0`, `day: 0` as special cases described in the proto spec.
  /// Returns an error if any component is out of range or date is invalid (e.g., February 30th).
  pub fn new(year: i32, month: i32, day: i32) -> Result<Self, DateError> {
    if !(0..=9999).contains(&year) {
      return Err(DateError::InvalidDateComponent);
    }

    if !(0..=12).contains(&month) {
      return Err(DateError::InvalidDateComponent);
    }

    if !(0..=31).contains(&day) {
      return Err(DateError::InvalidDateComponent);
    }

    if year == 0 && month == 0 {
      return Err(DateError::InvalidDateComponent);
    }

    if day != 0 && month == 0 {
      return Err(DateError::InvalidDateComponent);
    }

    Ok(Date { year, month, day })
  }

  /// Checks if this `Date` instance represents a valid date according to its constraints.
  pub fn is_valid(&self) -> bool {
    Self::new(self.year, self.month, self.day).is_ok()
  }

  /// Returns `true` if the `Date` has a specific year (i.e., `year` is not 0).
  pub fn has_year(&self) -> bool {
    self.year != 0
  }

  /// Returns `true` if the `Date` has a specific month (i.e., `month` is not 0).
  pub fn has_month(&self) -> bool {
    self.month != 0
  }

  /// Returns `true` if the `Date` has a specific day (i.e., `day` is not 0).
  pub fn has_day(&self) -> bool {
    self.day != 0
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
    Date {
      year: naive_date.year(),
      month: naive_date.month() as i32,
      day: naive_date.day() as i32,
    }
  }
}
