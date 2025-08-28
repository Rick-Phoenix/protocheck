use std::cmp::Ordering;

use thiserror::Error;

use crate::{common::TimeOfDay, constants::*};

#[derive(Debug, Error)]
pub enum TimeOfDayError {
  #[error("Hours out of valid range (0-23)")]
  InvalidHours,
  #[error("Minutes out of valid range (0-59)")]
  InvalidMinutes,
  #[error("Seconds out of valid range (0-59/60)")]
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

fn total_nanos_to_time_of_day(mut total_nanos: i64) -> Result<TimeOfDay, TimeOfDayError> {
  // Ensure the total nanoseconds value is within a single day's range
  if !(0..NANOS_PER_DAY_EXCLUSIVE).contains(&total_nanos) {
    return Err(TimeOfDayError::OutOfRange);
  }

  let hours = (total_nanos / NANOS_PER_HOUR) as i32;
  total_nanos %= NANOS_PER_HOUR;

  let minutes = (total_nanos / NANOS_PER_MINUTE) as i32;
  total_nanos %= NANOS_PER_MINUTE;

  let seconds = (total_nanos / NANOS_PER_SECOND as i64) as i32;
  total_nanos %= NANOS_PER_SECOND as i64;

  let nanos = total_nanos as i32;

  TimeOfDay::new(hours, minutes, seconds, nanos)
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
  /// Allows 24 hours and 60 seconds as special cases described in the proto spec.
  pub fn new(hours: i32, minutes: i32, seconds: i32, nanos: i32) -> Result<Self, TimeOfDayError> {
    if !((0..=24).contains(&hours)) {
      // Proto allows 24:00:00 for closing time scenarios
      return Err(TimeOfDayError::InvalidHours);
    }
    if !((0..=59).contains(&minutes)) {
      return Err(TimeOfDayError::InvalidMinutes);
    }
    if !((0..=60).contains(&seconds)) {
      // Proto allows 60 for leap seconds
      return Err(TimeOfDayError::InvalidSeconds);
    }
    if !((0..=999_999_999).contains(&nanos)) {
      return Err(TimeOfDayError::InvalidNanos);
    }

    // Additional logical validation:
    // If hours is 24, minutes, seconds, and nanos must be 0
    if hours == 24 && (minutes != 0 || seconds != 0 || nanos != 0) {
      return Err(TimeOfDayError::InvalidHours);
    }
    // If seconds is 60, nanos must be 0
    if seconds == 60 && nanos != 0 {
      return Err(TimeOfDayError::InvalidSeconds);
    }

    Ok(TimeOfDay {
      hours,
      minutes,
      seconds,
      nanos,
    })
  }

  /// Checks if this `TimeOfDay` instance represents a valid time.
  pub fn is_valid(&self) -> bool {
    Self::new(self.hours, self.minutes, self.seconds, self.nanos).is_ok()
  }

  /// Attempts to add a specified number of seconds to this `TimeOfDay`.
  /// The result wraps around the 24-hour mark if the new time falls into the next day.
  /// Returns an error if the input seconds are too large to cause an overflow in calculation.
  pub fn try_add_seconds(&self, seconds_to_add: i64) -> Result<Self, TimeOfDayError> {
    let current_total_nanos = self.nanos_since_midnight();
    let nanos_to_add = seconds_to_add
      .checked_mul(NANOS_PER_SECOND as i64)
      .ok_or(TimeOfDayError::OutOfRange)?;

    let mut new_total_nanos = current_total_nanos
      .checked_add(nanos_to_add)
      .ok_or(TimeOfDayError::OutOfRange)?;

    // Wrap around the day if necessary
    while new_total_nanos < 0 {
      new_total_nanos += NANOS_PER_DAY_EXCLUSIVE;
    }
    new_total_nanos %= NANOS_PER_DAY_EXCLUSIVE;

    total_nanos_to_time_of_day(new_total_nanos)
  }

  /// Attempts to subtract a specified number of seconds from this `TimeOfDay`.
  /// The result wraps around the 24-hour mark if the new time falls into the previous day.
  /// Returns an error if the input seconds are too large to cause an overflow in calculation.
  pub fn try_sub_seconds(&self, seconds_to_sub: i64) -> Result<Self, TimeOfDayError> {
    let current_total_nanos = self.nanos_since_midnight();
    let nanos_to_sub = seconds_to_sub
      .checked_mul(NANOS_PER_SECOND as i64)
      .ok_or(TimeOfDayError::OutOfRange)?;

    let mut new_total_nanos = current_total_nanos
      .checked_sub(nanos_to_sub)
      .ok_or(TimeOfDayError::OutOfRange)?;

    // Wrap around the day if necessary
    while new_total_nanos < 0 {
      new_total_nanos += NANOS_PER_DAY_EXCLUSIVE;
    }
    new_total_nanos %= NANOS_PER_DAY_EXCLUSIVE;

    total_nanos_to_time_of_day(new_total_nanos)
  }
}
