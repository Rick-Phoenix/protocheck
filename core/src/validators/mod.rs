use crate::protovalidate::{FieldPathElement, Violations};

pub trait ProtoValidator {
  fn validate(&self) -> Result<(), Violations>;
}

#[macro_use]
pub mod static_data;

pub mod bytes;
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
pub mod timestamps;

pub mod strings;
