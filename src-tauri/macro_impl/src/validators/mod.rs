use proto_types::buf::validate::{FieldPathElement, Violation, Violations};

pub trait WithValidator {
  fn validate(&self) -> Result<(), Violations>;
  fn nested_validate(
    &self,
    parent_messages: &mut Vec<FieldPathElement>,
    violations: &mut Vec<Violation>,
  ) -> Result<(), Violations>;
}

pub mod strings;
