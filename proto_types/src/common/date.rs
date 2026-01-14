use core::{
  cmp::{Ord, Ordering, PartialOrd},
  fmt::Display,
};

use thiserror::Error;

use crate::{String, ToString, common::Date};

/// Errors that can occur during the creation, conversion or validation of a [`Date`].
#[derive(Debug, Error, PartialEq, Eq, Clone)]
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

impl Display for Date {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self.kind() {
      DateKind::Full => write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day),
      DateKind::YearAndMonth => write!(f, "{:04}-{:02}", self.year, self.month),
      DateKind::YearOnly => write!(f, "{:04}", self.year),
      DateKind::MonthAndDay => write!(f, "{:02}-{:02}", self.month, self.day),
    }
  }
}

/// The kind of combinations that a [`Date`] can contain.
#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum DateKind {
  /// A full date, with non-zero year, month, and day values
  Full,
  /// A year on its own, with zero month and day values
  YearOnly,
  /// A year and month value, with a zero day, such as a credit card expiration
  YearAndMonth,
  /// A month and day value, with a zero year, such as an anniversary
  MonthAndDay,
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
  /// Creates a new [`Date`] instance with validation.
  /// Allows `year: 0`, `month: 0`, `day: 0` as special cases described in the proto spec.
  /// Returns an error if any component is out of range or date is invalid (e.g., February 30th).
  pub fn new(year: i32, month: i32, day: i32) -> Result<Self, DateError> {
    validate_date(year, month, day)?;

    Ok(Self { year, month, day })
  }

  /// Returns the kind of values combination for this [`Date`]
  #[must_use]
  pub const fn kind(&self) -> DateKind {
    if self.year != 0 && self.month == 0 && self.day == 0 {
      DateKind::YearOnly
    } else if self.year != 0 && self.month != 0 && self.day == 0 {
      DateKind::YearAndMonth
    } else if self.year == 0 && self.month != 0 && self.day != 0 {
      DateKind::MonthAndDay
    } else {
      DateKind::Full
    }
  }

  /// Checks if this [`Date`] instance represents a valid date according to its constraints.
  #[must_use]
  pub fn is_valid(&self) -> bool {
    validate_date(self.year, self.month, self.day).is_ok()
  }

  #[must_use]
  pub const fn has_year(&self) -> bool {
    self.year != 0
  }

  /// Returns `true` if this [`Date`] only indicates a year.
  #[must_use]
  pub const fn is_year_only(&self) -> bool {
    self.year != 0 && (self.month == 0 && self.day == 0)
  }

  /// Returns `true` if this [`Date`] only indicates a year and a month (i.e. for a credit card expiration date).
  #[must_use]
  pub const fn is_year_and_month(&self) -> bool {
    self.year != 0 && self.month != 0 && self.day == 0
  }

  /// Returns `true` if this [`Date`] only indicates a month and a day, with no specific year.
  #[must_use]
  pub const fn is_month_and_day(&self) -> bool {
    self.year == 0 && self.month != 0 && self.day != 0
  }
}

impl PartialOrd for Date {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if !(self.is_valid() && other.is_valid()) {
      return None;
    }

    let self_kind = self.kind();
    let other_kind = other.kind();

    if self_kind != other_kind {
      return None;
    }

    Some(
      self
        .year
        .cmp(&other.year)
        .then_with(|| self.month.cmp(&other.month))
        .then_with(|| self.day.cmp(&other.day)),
    )
  }
}

#[cfg(feature = "chrono")]
mod chrono {
  use chrono::Utc;

  use super::validate_date;
  use crate::{Date, ToString, date::DateError, format};

  impl Date {
    /// Converts this [`Date`] to [`chrono::NaiveDate`]. It fails if the year, month or day are set to zero.
    pub fn to_naive_date(self) -> Result<::chrono::NaiveDate, DateError> {
      self.try_into()
    }

    /// Returns the current date.
    #[must_use]
    pub fn today() -> Self {
      Utc::now().naive_utc().date().into()
    }
  }

  impl TryFrom<crate::Date> for chrono::NaiveDate {
    type Error = DateError;

    fn try_from(date: Date) -> Result<Self, Self::Error> {
      if date.year == 0 || date.month == 0 || date.day == 0 {
        return Err(DateError::ConversionError(
          "Cannot convert Date with year=0, month=0, or day=0 to NaiveDate".to_string(),
        ));
      }

      validate_date(date.year, date.month, date.day)?;

      // Safe castings after validation
      Self::from_ymd_opt(
        date.year,
        date.month.cast_unsigned(),
        date.day.cast_unsigned(),
      )
      .ok_or_else(|| {
        DateError::ConversionError(format!(
          "Invalid date components for NaiveDate: Y:{}, M:{}, D:{}",
          date.year, date.month, date.day
        ))
      })
    }
  }

  impl From<chrono::NaiveDate> for Date {
    fn from(naive_date: chrono::NaiveDate) -> Self {
      use chrono::Datelike;
      // Casting is safe due to chrono's costructor API
      Self {
        year: naive_date.year(),
        month: naive_date.month().cast_signed(),
        day: naive_date.day().cast_signed(),
      }
    }
  }
}
