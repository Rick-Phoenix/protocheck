mod base;
mod conversions;
mod operations;
mod serde;

pub use base::TimestampError;
use chrono::{DateTime, Utc};

use crate::{Duration, Timestamp};

impl Timestamp {
  pub fn as_datetime_utc(&self) -> Result<DateTime<Utc>, TimestampError> {
    (*self).try_into()
  }

  pub fn now() -> Self {
    Utc::now().into()
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
