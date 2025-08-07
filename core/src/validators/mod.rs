use crate::protovalidate::{FieldPathElement, Violations};

pub trait ProtoValidator {
  fn validate(&self) -> Result<(), Violations>;
}

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

#[allow(unused, dead_code)]
pub mod static_data;
pub mod strings;
