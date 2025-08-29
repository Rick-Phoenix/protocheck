use std::{cmp::Ordering, fmt::Display};

use thiserror::Error;

use crate::{common::TimeOfDay, constants::NANOS_PER_SECOND};

const NANOS_PER_MINUTE: i64 = NANOS_PER_SECOND as i64 * 60;
const NANOS_PER_HOUR: i64 = NANOS_PER_MINUTE * 60;

impl Display for TimeOfDay {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{:02}:{:02}:{:02}",
      self.hours, self.minutes, self.seconds
    )
  }
}

/// Errors that can occur during the creation, conversion or validation of a [`TimeOfDay`].
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum TimeOfDayError {
  #[error("Hours out of valid range (0-23)")]
  InvalidHours,
  #[error("Minutes out of valid range (0-59)")]
  InvalidMinutes,
  #[error("Seconds out of valid range (0-59)")]
  InvalidSeconds,
  #[error("Nanoseconds out of valid range (0-999,999,999)")]
  InvalidNanos,
  #[error("The values for this TimeOfDay are outside of the allowed range")]
  ConversionError,
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

#[cfg(feature = "chrono")]
impl TryFrom<TimeOfDay> for chrono::NaiveTime {
  type Error = TimeOfDayError;
  fn try_from(value: TimeOfDay) -> Result<Self, Self::Error> {
    let hours_u32: u32 = value
      .hours
      .try_into()
      .map_err(|_| TimeOfDayError::InvalidHours)?;
    let minutes_u32: u32 = value
      .minutes
      .try_into()
      .map_err(|_| TimeOfDayError::InvalidMinutes)?;
    let seconds_u32: u32 = value
      .seconds
      .try_into()
      .map_err(|_| TimeOfDayError::InvalidSeconds)?;
    let nanos_u32: u32 = value
      .nanos
      .try_into()
      .map_err(|_| TimeOfDayError::InvalidNanos)?;

    Self::from_hms_nano_opt(hours_u32, minutes_u32, seconds_u32, nanos_u32)
      .ok_or(TimeOfDayError::ConversionError)
  }
}

impl PartialOrd for TimeOfDay {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for TimeOfDay {
  fn cmp(&self, other: &Self) -> Ordering {
    // Directly use the i64 comparison.
    self
      .nanos_since_midnight()
      .cmp(&other.nanos_since_midnight())
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

pub const MIDNIGHT: TimeOfDay = TimeOfDay {
  hours: 0,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const ONE_AM: TimeOfDay = TimeOfDay {
  hours: 1,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const TWO_AM: TimeOfDay = TimeOfDay {
  hours: 2,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const THREE_AM: TimeOfDay = TimeOfDay {
  hours: 3,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const FOUR_AM: TimeOfDay = TimeOfDay {
  hours: 4,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const FIVE_AM: TimeOfDay = TimeOfDay {
  hours: 5,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const SIX_AM: TimeOfDay = TimeOfDay {
  hours: 6,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const SEVEN_AM: TimeOfDay = TimeOfDay {
  hours: 7,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const EIGHT_AM: TimeOfDay = TimeOfDay {
  hours: 8,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const NINE_AM: TimeOfDay = TimeOfDay {
  hours: 9,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const TEN_AM: TimeOfDay = TimeOfDay {
  hours: 10,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const ELEVEN_AM: TimeOfDay = TimeOfDay {
  hours: 11,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const NOON: TimeOfDay = TimeOfDay {
  hours: 12,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const ONE_PM: TimeOfDay = TimeOfDay {
  hours: 13,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const TWO_PM: TimeOfDay = TimeOfDay {
  hours: 14,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const THREE_PM: TimeOfDay = TimeOfDay {
  hours: 15,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const FOUR_PM: TimeOfDay = TimeOfDay {
  hours: 16,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const FIVE_PM: TimeOfDay = TimeOfDay {
  hours: 17,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const SIX_PM: TimeOfDay = TimeOfDay {
  hours: 18,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const SEVEN_PM: TimeOfDay = TimeOfDay {
  hours: 19,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const EIGHT_PM: TimeOfDay = TimeOfDay {
  hours: 20,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const NINE_PM: TimeOfDay = TimeOfDay {
  hours: 21,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const TEN_PM: TimeOfDay = TimeOfDay {
  hours: 22,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
pub const ELEVEN_PM: TimeOfDay = TimeOfDay {
  hours: 23,
  minutes: 0,
  seconds: 0,
  nanos: 0,
};
