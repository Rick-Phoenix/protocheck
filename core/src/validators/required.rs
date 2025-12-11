use crate::{
  field_data::FieldContext,
  protovalidate::{Violation, REQUIRED_VIOLATION},
  validators::static_data::base_violations::create_violation,
};

pub fn required(field_context: &FieldContext) -> Violation {
  create_violation(field_context, &REQUIRED_VIOLATION, "is required")
}
