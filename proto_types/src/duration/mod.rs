mod base;
pub use base::DurationError;

use crate::Duration;

mod duration_impls;

mod formatting;

/// Structs for duration units such as Seconds and Minutes.
pub mod data {
  pub use super::{duration_data::*, duration_units::*};
}

mod duration_data;
mod duration_operations;
mod duration_units;

impl Duration {
  /// Whether the duration is negative or not.
  pub fn is_negative(&self) -> bool {
    self.normalized().seconds < 0
  }

  pub fn new(seconds: i64, nanos: i32) -> Self {
    let mut instance = Duration { seconds, nanos };
    instance.normalize();
    instance
  }
}
