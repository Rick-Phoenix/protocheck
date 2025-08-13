mod base;
pub use base::DurationError;

mod duration_impls;

mod formatting;

/// Structs for duration units such as Seconds and Minutes.
pub mod data {
  pub use super::{duration_data::*, duration_units::*};
}

mod duration_data;
mod duration_units;
