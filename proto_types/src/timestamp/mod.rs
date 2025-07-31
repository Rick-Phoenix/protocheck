mod impls;

use chrono::{DateTime, Utc};
pub use impls::TimestampError;

use crate::{Duration, Timestamp};

impl Timestamp {
  pub fn as_datetime_utc(&self) -> Option<DateTime<Utc>> {
    (*self).try_into().ok()
  }

  pub fn now() -> Self {
    Utc::now().into()
  }

  pub fn is_within_range_from_now(&self, range: &Duration) -> bool {
    (*self + range) <= Self::now()
  }
}
