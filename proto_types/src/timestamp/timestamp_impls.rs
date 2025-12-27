use std::time::SystemTime;

use crate::{Duration, Timestamp};

#[cfg(not(feature = "chrono"))]
impl crate::Timestamp {
  /// Returns the timestamp in YYYY-MM-DD format.
  /// The same method, with the `chrono` feature, allows for custom formatting.
  pub fn format(&self) -> String {
    self.to_string()
  }
}

#[cfg(feature = "chrono")]
mod chrono {
  use chrono::Utc;

  use crate::{Timestamp, timestamp::TimestampError};

  impl Timestamp {
    /// Converts this timestamp into a [`chrono::DateTime<Utc>`] struct and calls .format on it with the string argument being given.
    pub fn format(&self, string: &str) -> Result<String, TimestampError> {
      let chrono_timestamp: chrono::DateTime<Utc> = (*self).try_into()?;

      Ok(chrono_timestamp.format(string).to_string())
    }

    /// Converts this [`Timestamp`] instance to chrono::[`DateTime`](::chrono::DateTime) with [`chrono::Utc`].
    pub fn as_datetime_utc(&self) -> Result<chrono::DateTime<Utc>, TimestampError> {
      (*self).try_into()
    }
  }
}

impl Timestamp {
  /// Returns the current timestamp.
  #[must_use]
  pub fn now() -> Self {
    SystemTime::now().into()
  }

  /// Creates a new instance.
  #[must_use]
  pub const fn new(seconds: i64, nanos: i32) -> Self {
    Self { seconds, nanos }
  }

  /// Checks whether the Timestamp instance is within the indicated range (positive or negative) from now.
  #[must_use]
  pub fn is_within_range_from_now(&self, range: Duration) -> bool {
    let now = Self::now();

    (now + range) >= *self && (now - range) <= *self
  }

  /// Checks whether the Timestamp instance is within the indicated range in the future.
  #[must_use]
  pub fn is_within_future_range(&self, range: Duration) -> bool {
    (Self::now() + range) >= *self
  }

  /// Checks whether the Timestamp instance is within the indicated range in the past.
  #[must_use]
  pub fn is_within_past_range(&self, range: Duration) -> bool {
    (Self::now() - range) <= *self
  }

  /// Returns `true` if the timestamp is in the future.
  #[must_use]
  pub fn is_future(&self) -> bool {
    *self > Self::now()
  }

  /// Returns `true` if the timestamp is in the past.
  #[must_use]
  pub fn is_past(&self) -> bool {
    *self < Self::now()
  }
}
