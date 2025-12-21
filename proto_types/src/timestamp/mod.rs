// Most of the content of this file is taken from (prost-types)[https://github.com/tokio-rs/prost/blob/master/prost-types/src], licensed under the Apache-2.0 license.
// Modifications have been applied to make casting operations more explicit.

#[cfg(feature = "serde")]
mod serde;

mod timestamp_conversions;
mod timestamp_impls;
mod timestamp_operations;

use super::*;
use crate::{
  Timestamp,
  constants::{NANOS_PER_SECOND, PACKAGE_PREFIX},
};

impl Timestamp {
  /// Normalizes the timestamp to a canonical format.
  ///
  /// Based on [`google::protobuf::util::CreateNormalized`][1].
  ///
  /// [1]: https://github.com/google/protobuf/blob/v3.3.2/src/google/protobuf/util/time_util.cc#L59-L77
  pub fn normalize(&mut self) {
    // Make sure nanos is in the range.
    if self.nanos <= -NANOS_PER_SECOND || self.nanos >= NANOS_PER_SECOND {
      if let Some(seconds) = self
        .seconds
        .checked_add(i64::from(self.nanos / NANOS_PER_SECOND))
      {
        self.seconds = seconds;

        self.nanos %= NANOS_PER_SECOND;
      } else if self.nanos < 0 {
        // Negative overflow! Set to the earliest normal value.

        self.seconds = i64::MIN;

        self.nanos = 0;
      } else {
        // Positive overflow! Set to the latest normal value.

        self.seconds = i64::MAX;

        self.nanos = 999_999_999;
      }
    }

    // For Timestamp nanos should be in the range [0, 999999999].

    if self.nanos < 0 {
      if let Some(seconds) = self.seconds.checked_sub(1) {
        self.seconds = seconds;

        self.nanos += NANOS_PER_SECOND;
      } else {
        // Negative overflow! Set to the earliest normal value.

        debug_assert_eq!(self.seconds, i64::MIN);

        self.nanos = 0;
      }
    }

    // TODO: should this be checked?

    // debug_assert!(self.seconds >= -62_135_596_800 && self.seconds <= 253_402_300_799,

    //               "invalid timestamp: {:?}", self);
  }

  /// Normalizes the timestamp to a canonical format, returning the original value if it cannot be
  /// normalized.
  ///
  /// Normalization is based on [`google::protobuf::util::CreateNormalized`][1].
  ///
  /// [1]: https://github.com/google/protobuf/blob/v3.3.2/src/google/protobuf/util/time_util.cc#L59-L77
  pub fn try_normalize(mut self) -> Result<Self, Self> {
    let before = self;

    self.normalize();

    // If the seconds value has changed, and is either i64::MIN or i64::MAX, then the timestamp

    // normalization overflowed.

    if (self.seconds == i64::MAX || self.seconds == i64::MIN) && self.seconds != before.seconds {
      Err(before)
    } else {
      Ok(self)
    }
  }

  /// Return a normalized copy of the timestamp to a canonical format.
  ///
  /// Based on [`google::protobuf::util::CreateNormalized`][1].
  ///
  /// [1]: https://github.com/google/protobuf/blob/v3.3.2/src/google/protobuf/util/time_util.cc#L59-L77
  #[must_use]
  pub fn normalized(&self) -> Self {
    let mut result = *self;

    result.normalize();

    result
  }

  /// Creates a new `Timestamp` at the start of the provided UTC date.
  pub fn date(year: i64, month: u8, day: u8) -> Result<Self, TimestampError> {
    Self::date_time_nanos(year, month, day, 0, 0, 0, 0)
  }

  /// Creates a new `Timestamp` instance with the provided UTC date and time.
  pub fn date_time(
    year: i64,

    month: u8,

    day: u8,

    hour: u8,

    minute: u8,

    second: u8,
  ) -> Result<Self, TimestampError> {
    Self::date_time_nanos(year, month, day, hour, minute, second, 0)
  }

  /// Creates a new `Timestamp` instance with the provided UTC date and time.
  pub fn date_time_nanos(
    year: i64,

    month: u8,

    day: u8,

    hour: u8,

    minute: u8,

    second: u8,

    nanos: u32,
  ) -> Result<Self, TimestampError> {
    let date_time = datetime_internal::DateTime {
      year,

      month,

      day,

      hour,

      minute,

      second,

      nanos,
    };

    Self::try_from(date_time)
  }
}

impl Name for Timestamp {
  const PACKAGE: &'static str = PACKAGE_PREFIX;

  const NAME: &'static str = "Timestamp";

  fn type_url() -> String {
    type_url_for::<Self>()
  }
}

impl From<std::time::SystemTime> for Timestamp {
  fn from(system_time: std::time::SystemTime) -> Self {
    let (seconds, nanos) = match system_time.duration_since(std::time::UNIX_EPOCH) {
      Ok(duration) => {
        let seconds = i64::try_from(duration.as_secs()).unwrap_or_default();

        // SAFETY: Safe due to the standard library's implementation
        (seconds, duration.subsec_nanos().cast_signed())
      }

      Err(error) => {
        let duration = error.duration();

        let seconds = i64::try_from(duration.as_secs()).unwrap_or_default();

        // SAFETY: Safe due to the standard library's implementation
        let nanos = duration.subsec_nanos().cast_signed();

        if nanos == 0 {
          (-seconds, 0)
        } else {
          (-seconds - 1, 1_000_000_000 - nanos)
        }
      }
    };

    Self { seconds, nanos }
  }
}

/// A timestamp handling error.
#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum TimestampError {
  /// Indicates that a [`Timestamp`] could not be converted to
  /// [`SystemTime`][std::time::SystemTime] because it is out of range.
  ///
  /// The range of times that can be represented by `SystemTime` depends on the platform. All
  /// `Timestamp`s are likely representable on 64-bit Unix-like platforms, but other platforms,
  /// such as Windows and 32-bit Linux, may not be able to represent the full range of
  /// `Timestamp`s.
  OutOfSystemRange(Timestamp),
  /// An error indicating failure to parse a timestamp in RFC-3339 format.
  ParseFailure,
  /// Indicates an error when constructing a timestamp due to invalid date or time data.
  InvalidDateTime,
}

impl fmt::Display for TimestampError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::OutOfSystemRange(timestamp) => {
        write!(
          f,
          "{timestamp} is not representable as a `SystemTime` because it is out of range"
        )
      }

      Self::ParseFailure => {
        write!(f, "failed to parse RFC-3339 formatted timestamp")
      }

      Self::InvalidDateTime => {
        write!(f, "invalid date or time")
      }
    }
  }
}

impl std::error::Error for TimestampError {}

impl TryFrom<Timestamp> for std::time::SystemTime {
  type Error = TimestampError;

  fn try_from(mut timestamp: Timestamp) -> Result<Self, Self::Error> {
    let orig_timestamp = timestamp;

    timestamp.normalize();

    let system_time = if timestamp.seconds >= 0 {
      std::time::UNIX_EPOCH.checked_add(time::Duration::from_secs(
        timestamp
          .seconds
          .try_into()
          .map_err(|_| TimestampError::OutOfSystemRange(timestamp))?,
      ))
    } else {
      std::time::UNIX_EPOCH.checked_sub(time::Duration::from_secs(
        timestamp
          .seconds
          .checked_neg()
          .and_then(|s| s.try_into().ok())
          .ok_or(TimestampError::OutOfSystemRange(timestamp))?,
      ))
    };

    let system_time = system_time
      .map(|time| {
        let nanos = u64::try_from(timestamp.nanos)
          .map_err(|_| TimestampError::OutOfSystemRange(timestamp))?;

        time
          .checked_add(std::time::Duration::from_nanos(nanos))
          .ok_or(TimestampError::OutOfSystemRange(timestamp))
      })
      .transpose()?;

    system_time.ok_or(TimestampError::OutOfSystemRange(orig_timestamp))
  }
}

impl FromStr for Timestamp {
  type Err = TimestampError;

  fn from_str(s: &str) -> Result<Self, TimestampError> {
    datetime_internal::parse_timestamp(s).ok_or(TimestampError::ParseFailure)
  }
}

impl fmt::Display for Timestamp {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    datetime_internal::DateTime::from(*self).fmt(f)
  }
}
