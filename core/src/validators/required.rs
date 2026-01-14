use super::*;
use crate::protovalidate::REQUIRED_VIOLATION;

#[must_use]
pub fn required(field_context: &FieldContext, parent_elements: &[FieldPathElement]) -> Violation {
  create_violation(
    field_context,
    REQUIRED_VIOLATION,
    "is required",
    parent_elements,
  )
}
