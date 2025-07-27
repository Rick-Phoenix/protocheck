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
pub mod enums;
pub mod maps;
pub mod oneofs;
pub mod repeated;
pub mod strings;
