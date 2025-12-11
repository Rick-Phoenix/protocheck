use super::*;
use crate::protovalidate::REQUIRED_VIOLATION;

pub fn required(field_context: &FieldContext) -> Violation {
  create_violation(field_context, &REQUIRED_VIOLATION, "is required")
}
