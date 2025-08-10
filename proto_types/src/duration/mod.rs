mod base;
pub use base::DurationError;

mod duration_impls;

mod formatting;

pub mod data {
  pub use super::{duration_data::*, duration_units::*};
}

mod duration_data;
mod duration_units;
