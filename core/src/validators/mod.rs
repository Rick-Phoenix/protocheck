use std::{
  collections::{HashMap, HashSet},
  sync::LazyLock,
};

use super::*;
use crate::{
  field_data::*,
  protovalidate::{Violation, ViolationData},
  validators::create_violation::*,
  wrappers::*,
};

pub(crate) mod create_violation;

/// Validators for bytes fields.
pub mod bytes;

/// Cel validation for messages or message fields.
#[cfg(feature = "cel")]
pub mod cel;

/// Validators for types that are comparable (PartialOrd, PartialEq)
pub mod comparables;

/// Validators for constant fields.
pub mod constants;

/// Validators for lists of allowed (or forbidden) values.
pub mod containing;

/// Validators for enum fields.
pub mod enums;

/// Validators for floating point numbers (f32 and f64).
pub mod floats;

/// Validators for protobuf maps.
pub mod maps;

/// Validators for protobuf oneofs.
pub mod oneofs;

/// Validators for repeated fields.
pub mod repeated;

/// Validators for fields that are marked as "required" in the protovalidate annotations.
pub mod required;

/// Validators for strings.
pub mod string;

/// Validators for timestamp fields.
pub mod timestamps;

mod well_known_strings;
