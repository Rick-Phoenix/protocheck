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
    let now = Self::now();
    let max = now + range;

    *self <= max && *self >= now
  }

  /// Checks whether the Timestamp instance is within the indicated range in the past.
  #[must_use]
  pub fn is_within_past_range(&self, range: Duration) -> bool {
    let now = Self::now();
    let min = now - range;

    *self >= min && *self <= now
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

#[cfg(test)]
mod tests {
  use super::*;

  fn offset_seconds(base: &Timestamp, s: i64) -> Timestamp {
    Timestamp {
      seconds: base.seconds + s,
      nanos: base.nanos,
    }
  }

  #[test]
  fn test_is_future() {
    let now = Timestamp::now();

    let future_point = offset_seconds(&now, 5);
    let past_point = offset_seconds(&now, -5);

    assert!(future_point.is_future(), "T + 5s should be in the future");
    assert!(
      !past_point.is_future(),
      "T - 5s should NOT be in the future"
    );
  }

  #[test]
  fn test_is_past() {
    let now = Timestamp::now();

    let future_point = offset_seconds(&now, 5);
    let past_point = offset_seconds(&now, -5);

    assert!(past_point.is_past(), "T - 5s should be in the past");
    assert!(!future_point.is_past(), "T + 5s should NOT be in the past");
  }

  #[test]
  fn test_is_within_future_range() {
    let now = Timestamp::now();
    let range = Duration::new(10, 0);

    let inside = offset_seconds(&now, 5);
    assert!(
      inside.is_within_future_range(range),
      "5s is within 10s range"
    );

    let outside_far = offset_seconds(&now, 15);
    assert!(
      !outside_far.is_within_future_range(range),
      "15s is outside 10s range"
    );

    let outside_past = offset_seconds(&now, -1);
    assert!(
      !outside_past.is_within_future_range(range),
      "Past value is not in future range"
    );
  }

  #[test]
  fn test_is_within_past_range() {
    let now = Timestamp::now();
    let range = Duration::new(10, 0);

    let inside = offset_seconds(&now, -5);
    assert!(
      inside.is_within_past_range(range),
      "-5s is within 10s past range"
    );

    let outside_old = offset_seconds(&now, -15);
    assert!(
      !outside_old.is_within_past_range(range),
      "-15s is too old for 10s range"
    );

    let outside_future = offset_seconds(&now, 1);
    assert!(
      !outside_future.is_within_past_range(range),
      "Future value is not in past range"
    );
  }
}
