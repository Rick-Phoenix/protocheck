mod base;
mod format_helpers;
mod serde;

pub use base::DurationError;
use cel_interpreter::Value as CelValue;

use crate::Duration;

impl TryFrom<Duration> for CelValue {
  type Error = DurationError;

  fn try_from(value: Duration) -> Result<Self, Self::Error> {
    let chrono_dur: chrono::Duration = value.try_into()?;

    Ok(CelValue::Duration(chrono_dur))
  }
}

impl Duration {
  pub fn new(seconds: i64, nanos: i32) -> Self {
    let mut instance = Duration { seconds, nanos };
    instance.normalize();
    instance
  }
}
