mod base;
mod format_helpers;
mod serde;

pub use base::DurationError;

use crate::Duration;

impl Duration {
  pub fn new(seconds: i64, nanos: i32) -> Self {
    let mut instance = Duration { seconds, nanos };
    instance.normalize();
    instance
  }
}
