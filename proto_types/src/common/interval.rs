use std::cmp::Ordering;

use thiserror::Error;

use crate::{common::Interval, constants::NANOS_PER_SECOND, Duration, Timestamp};

/// Errors that can occur during the creation, conversion or validation of an [`Interval`].
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum IntervalError {
  #[error("Start and end time must be both defined or undefined")]
  InvalidPairing,
  #[error("Interval contains an invalid Timestamp")]
  InvalidTimestamp,
  #[error("Interval's end_time is before its start_time")]
  EndTimeBeforeStartTime,
  #[error("Interval arithmetic resulted in a value outside its representable range")]
  OutOfRange,
  #[error("Interval conversion error: {0}")]
  ConversionError(String),
}

fn validate_interval(
  start: Option<Timestamp>,
  end: Option<Timestamp>,
) -> Result<(), IntervalError> {
  if end < start {
    Err(IntervalError::EndTimeBeforeStartTime)
  } else if !((start.is_some() && end.is_some()) || start.is_none() && end.is_none()) {
    Err(IntervalError::InvalidPairing)
  } else {
    Ok(())
  }
}

impl Interval {
  /// Creates a new [`Interval`] instance with validation.
  /// `end_time` must not be before `start_time`, and they must be both either set or unset.
  pub fn new(
    start_time: Option<Timestamp>,
    end_time: Option<Timestamp>,
  ) -> Result<Self, IntervalError> {
    validate_interval(start_time, end_time)?;

    Ok(Interval {
      start_time,
      end_time,
    })
  }

  /// Creates an [`Interval`] going from now to the `end_time` specified.
  /// The present moment is calculated using the SystemTime.
  pub fn from_now_to(end_time: Timestamp) -> Self {
    Self {
      start_time: Some(Timestamp::now()),
      end_time: Some(end_time),
    }
  }

  /// Creates a new [`Interval`] going from the specified `start_time` to the present moment.
  /// The present moment is calculated using the SystemTime.
  pub fn from_start_to_now(start_time: Timestamp) -> Self {
    Self {
      start_time: Some(start_time),
      end_time: Some(Timestamp::now()),
    }
  }

  /// Checks that `end_time` is not before `start_time`. And that start and `end_time` are both either unspecified or specified at the same time.
  pub fn is_valid(&self) -> bool {
    validate_interval(self.start_time, self.end_time).is_ok()
  }

  /// Returns `true` if the `Interval` is empty (`start_time` equals `end_time`).
  pub fn is_empty(&self) -> bool {
    self
      .start_time
      .as_ref()
      .zip(self.end_time.as_ref())
      .map_or_else(|| false, |(start, end)| start == end)
  }

  /// Returns `true` if the `Interval` is unspecified (no `start_time` and no `end_time`)
  pub fn is_unspecified(&self) -> bool {
    self.start_time.is_none() && self.end_time.is_none()
  }
}

impl TryFrom<Interval> for Duration {
  type Error = IntervalError;
  fn try_from(value: Interval) -> Result<Self, Self::Error> {
    let result = value.start_time.zip(value.end_time).map(|(start, end)| {
      let mut seconds_diff = end.seconds - start.seconds;
      let mut nanos_diff = end.nanos - start.nanos;

      if nanos_diff < 0 {
        seconds_diff -= 1;
        nanos_diff += NANOS_PER_SECOND;
      } else if nanos_diff >= NANOS_PER_SECOND {
        seconds_diff += 1;
        nanos_diff -= NANOS_PER_SECOND;
      }

      Duration {
        seconds: seconds_diff,
        nanos: nanos_diff,
      }
    });

    result.ok_or(IntervalError::ConversionError(
      "Cannot convert to Duration due to missing start or end time".to_string(),
    ))
  }
}

impl PartialOrd for Interval {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if !self.is_valid() || !other.is_valid() {
      return None;
    }

    if self.is_unspecified() {
      if other.is_unspecified() {
        Some(Ordering::Equal)
      } else {
        Some(Ordering::Less)
      }
    } else if other.is_unspecified() {
      Some(Ordering::Greater)
    } else {
      let self_as_duration: Result<Duration, IntervalError> = (*self).try_into();
      let other_as_duration: Result<Duration, IntervalError> = (*other).try_into();

      if self_as_duration.is_ok() && other_as_duration.is_err() {
        Some(Ordering::Greater)
      } else if self_as_duration.is_err() && other_as_duration.is_ok() {
        Some(Ordering::Less)
      } else if self_as_duration.is_err() && other_as_duration.is_err() {
        Some(Ordering::Equal)
      } else {
        self_as_duration
          .unwrap()
          .partial_cmp(&other_as_duration.unwrap())
      }
    }
  }
}
