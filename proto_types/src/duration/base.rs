// Partially taken from (prost-types)[https://github.com/tokio-rs/prost/blob/master/prost-types/src/duration.rs]
use super::super::*;
use crate::constants::{NANOS_PER_SECOND, PACKAGE_PREFIX, TIME_NANOS_MAX};

impl Duration {
  /// Normalizes the duration to a canonical format.
  ///
  /// Based on [`google::protobuf::util::CreateNormalized`][1].
  ///
  /// [1]: https://github.com/google/protobuf/blob/v3.3.2/src/google/protobuf/util/time_util.cc#L79-L100
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
        // Negative overflow! Set to the least normal value.

        self.seconds = i64::MIN;

        self.nanos = -TIME_NANOS_MAX;
      } else {
        // Positive overflow! Set to the greatest normal value.

        self.seconds = i64::MAX;

        self.nanos = TIME_NANOS_MAX;
      }
    }

    // nanos should have the same sign as seconds.

    if self.seconds < 0 && self.nanos > 0 {
      if let Some(seconds) = self.seconds.checked_add(1) {
        self.seconds = seconds;

        self.nanos -= NANOS_PER_SECOND;
      } else {
        // Positive overflow! Set to the greatest normal value.

        debug_assert_eq!(self.seconds, i64::MAX);

        self.nanos = TIME_NANOS_MAX;
      }
    } else if self.seconds > 0 && self.nanos < 0 {
      if let Some(seconds) = self.seconds.checked_sub(1) {
        self.seconds = seconds;

        self.nanos += NANOS_PER_SECOND;
      } else {
        // Negative overflow! Set to the least normal value.

        debug_assert_eq!(self.seconds, i64::MIN);

        self.nanos = -TIME_NANOS_MAX;
      }
    }

    // TODO: should this be checked?

    // debug_assert!(self.seconds >= -315_576_000_000 && self.seconds <= 315_576_000_000,

    //               "invalid duration: {:?}", self);
  }

  /// Returns a normalized copy of the duration to a canonical format.
  ///
  /// Based on [`google::protobuf::util::CreateNormalized`][1].
  ///
  /// [1]: https://github.com/google/protobuf/blob/v3.3.2/src/google/protobuf/util/time_util.cc#L79-L100
  #[must_use]
  pub fn normalized(&self) -> Self {
    let mut result = *self;

    result.normalize();

    result
  }
}

impl Name for Duration {
  const PACKAGE: &'static str = PACKAGE_PREFIX;

  const NAME: &'static str = "Duration";

  fn type_url() -> String {
    type_url_for::<Self>()
  }
}

impl TryFrom<time::Duration> for Duration {
  type Error = DurationError;

  /// Converts a `std::time::Duration` to a `Duration`, failing if the duration is too large.
  fn try_from(duration: time::Duration) -> Result<Self, DurationError> {
    let seconds = i64::try_from(duration.as_secs()).map_err(|_| DurationError::OutOfRange)?;

    let nanos: i32 = duration
      .subsec_nanos()
      .try_into()
      .map_err(|_| DurationError::OutOfRange)?;

    let duration = Self { seconds, nanos };

    Ok(duration.normalized())
  }
}

impl TryFrom<Duration> for time::Duration {
  type Error = DurationError;

  /// Converts a `Duration` to a `std::time::Duration`, failing if the duration is negative.
  fn try_from(mut duration: Duration) -> Result<Self, DurationError> {
    duration.normalize();

    if duration.seconds >= 0 && duration.nanos >= 0 {
      Ok(Self::new(
        duration
          .seconds
          .try_into()
          .map_err(|_| DurationError::OutOfRange)?,
        duration
          .nanos
          .try_into()
          .map_err(|_| DurationError::OutOfRange)?,
      ))
    } else {
      Err(DurationError::NegativeDuration(Self::new(
        (-duration.seconds)
          .try_into()
          .map_err(|_| DurationError::OutOfRange)?,
        (-duration.nanos)
          .try_into()
          .map_err(|_| DurationError::OutOfRange)?,
      )))
    }
  }
}

/// A duration handling error.
#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum DurationError {
  /// Indicates failure to parse a [`Duration`] from a string.
  ///
  /// The [`Duration`] string format is specified in the [Protobuf JSON mapping specification][1].
  ///
  /// [1]: https://developers.google.com/protocol-buffers/docs/proto3#json
  ParseFailure,

  /// Indicates failure to convert a `prost_types::Duration` to a `std::time::Duration` because
  /// the duration is negative. The included `std::time::Duration` matches the magnitude of the
  /// original negative `prost_types::Duration`.
  NegativeDuration(time::Duration),

  /// Indicates failure to convert a `std::time::Duration` to a `prost_types::Duration`.
  ///
  /// Converting a `std::time::Duration` to a `prost_types::Duration` fails if the magnitude
  /// exceeds that representable by `prost_types::Duration`.
  OutOfRange,
}

impl fmt::Display for DurationError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ParseFailure => write!(f, "failed to parse duration"),

      Self::NegativeDuration(duration) => {
        write!(f, "failed to convert negative duration: {duration:?}")
      }

      Self::OutOfRange => {
        write!(f, "failed to convert duration out of range")
      }
    }
  }
}

impl core::error::Error for DurationError {}

impl FromStr for Duration {
  type Err = DurationError;

  fn from_str(s: &str) -> Result<Self, DurationError> {
    datetime_internal::parse_duration(s).ok_or(DurationError::ParseFailure)
  }
}

#[cfg(feature = "chrono")]
mod chrono {

  use crate::{Duration, duration::DurationError};

  impl From<::chrono::TimeDelta> for Duration {
    fn from(value: ::chrono::TimeDelta) -> Self {
      let mut result = Self {
        seconds: value.num_seconds(),

        nanos: value.subsec_nanos(),
      };

      result.normalize();

      result
    }
  }

  impl TryFrom<Duration> for ::chrono::TimeDelta {
    type Error = DurationError;

    fn try_from(mut value: Duration) -> Result<Self, DurationError> {
      value.normalize();

      let seconds = Self::try_seconds(value.seconds).ok_or(DurationError::OutOfRange)?;

      let nanos = Self::nanoseconds(value.nanos.into());

      seconds
        .checked_add(&nanos)
        .ok_or(DurationError::OutOfRange)
    }
  }
}
