use crate::protovalidate::{FieldPathElement, Violation, Violations};

pub trait ProtoValidator {
  fn validate(&self) -> Result<(), Violations>;
  fn nested_validate(
    &self,
    parent_messages: &mut Vec<FieldPathElement>,
    violations: &mut Vec<Violation>,
  );
}

pub mod cel;
pub mod common;
pub mod comparables;
pub mod constants;
pub mod containing;
pub mod enums;
pub mod floats;
pub mod maps;
pub mod oneofs;
pub mod repeated;

#[allow(unused, dead_code)]
pub mod static_data;
pub mod strings;
