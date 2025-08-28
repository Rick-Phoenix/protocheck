use std::cmp::Ordering;

use thiserror::Error;

use crate::{
  common::TimeOfDay,
  constants::{NANOS_PER_HOUR, NANOS_PER_MINUTE, NANOS_PER_SECOND},
};

/// Errors that can occur during the creation, conversion or validation of a [`TimeOfDay`].
#[derive(Debug, Error)]
pub enum TimeOfDayError {
  #[error("Hours out of valid range (0-23)")]
  InvalidHours,
  #[error("Minutes out of valid range (0-59)")]
  InvalidMinutes,
  #[error("Seconds out of valid range (0-59)")]
  InvalidSeconds,
  #[error("Nanoseconds out of valid range (0-999,999,999)")]
  InvalidNanos,
  #[error("Arithmetic resulted in a time outside the 24-hour range")]
  OutOfRange,
}

#[cfg(feature = "chrono")]
impl From<chrono::NaiveTime> for TimeOfDay {
  fn from(value: chrono::NaiveTime) -> Self {
    use chrono::Timelike;

    TimeOfDay {
      hours: value.hour() as i32,
      minutes: value.minute() as i32,
      seconds: value.second() as i32,
      nanos: value.nanosecond() as i32,
    }
  }
}

impl PartialOrd for TimeOfDay {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self
      .nanos_since_midnight()
      .partial_cmp(&other.nanos_since_midnight())
  }
}

fn validate_time_of_day(
  hours: i32,
  minutes: i32,
  seconds: i32,
  nanos: i32,
) -> Result<(), TimeOfDayError> {
  if !((0..=23).contains(&hours)) {
    return Err(TimeOfDayError::InvalidHours);
  }
  if !((0..=59).contains(&minutes)) {
    return Err(TimeOfDayError::InvalidMinutes);
  }
  if !((0..=59).contains(&seconds)) {
    return Err(TimeOfDayError::InvalidSeconds);
  }
  if !((0..=999_999_999).contains(&nanos)) {
    return Err(TimeOfDayError::InvalidNanos);
  }

  Ok(())
}

impl TimeOfDay {
  /// Returns the total amount of nanoseconds since midnight for this instance.
  pub fn nanos_since_midnight(&self) -> i64 {
    self.hours as i64 * NANOS_PER_HOUR
      + self.minutes as i64 * NANOS_PER_MINUTE
      + self.seconds as i64 * NANOS_PER_SECOND as i64
      + self.nanos as i64
  }

  /// Creates a new [`TimeOfDay`] instance with validation.
  pub fn new(hours: i32, minutes: i32, seconds: i32, nanos: i32) -> Result<Self, TimeOfDayError> {
    validate_time_of_day(hours, minutes, seconds, nanos)?;

    Ok(TimeOfDay {
      hours,
      minutes,
      seconds,
      nanos,
    })
  }

  /// Checks if this [`TimeOfDay`] instance represents a valid time.
  pub fn is_valid(&self) -> bool {
    validate_time_of_day(self.hours, self.minutes, self.seconds, self.nanos).is_ok()
  }
}
