use std::time::SystemTime;

use crate::{Duration, Timestamp};

#[cfg(feature = "chrono")]
mod chrono {
  use chrono::Utc;

  use crate::{Timestamp, TimestampError};

  impl Timestamp {
    pub fn format(&self, string: &str) -> Result<String, TimestampError> {
      let chrono_timestamp: chrono::DateTime<Utc> = (*self).try_into()?;

      Ok(chrono_timestamp.format(string).to_string())
    }

    pub fn as_datetime_utc(&self) -> Result<chrono::DateTime<Utc>, TimestampError> {
      (*self).try_into()
    }
  }
}

impl Timestamp {
  pub fn now() -> Self {
    SystemTime::now().into()
  }

  pub fn new(seconds: i64, nanos: i32) -> Self {
    Timestamp { seconds, nanos }
  }

  pub fn is_within_range_from_now(&self, range: Duration) -> bool {
    (Timestamp::now() + range) >= *self && (Timestamp::now() - range) <= *self
  }

  pub fn is_within_future_range(&self, range: Duration) -> bool {
    (Timestamp::now() + range) >= *self
  }

  pub fn is_within_past_range(&self, range: Duration) -> bool {
    (Timestamp::now() - range) <= *self
  }

  pub fn is_future(&self) -> bool {
    *self > Self::now()
  }

  pub fn is_past(&self) -> bool {
    *self < Self::now()
  }
}
