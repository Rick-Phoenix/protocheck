use crate::protovalidate::{FieldPathElement, Violations};

pub trait ProtoValidator {
  fn validate(&self) -> Result<(), Violations>;
}

#[macro_use]
pub mod static_data;

#[cfg(feature = "bytes")]
pub mod bytes;
#[cfg(feature = "cel")]
pub mod cel;
pub mod comparables;
pub mod constants;
pub mod containing;
pub mod enums;
pub mod floats;
pub mod maps;
pub mod oneofs;
pub mod repeated;
pub mod required;
pub mod strings;
pub mod timestamps;
mod well_known_strings;
