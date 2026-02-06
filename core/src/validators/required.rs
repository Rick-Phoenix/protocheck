use std::sync::LazyLock;

use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPathElement, Violation},
  validators::static_data::base_violations::create_violation,
  ProtoType,
};

pub fn required(field_context: &FieldContext) -> Violation {
  create_violation(
    field_context,
    &REQUIRED_VIOLATION,
    "field.required",
    "is required",
  )
}

static REQUIRED_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![FieldPathElement {
    field_type: Some(ProtoType::Bool as i32),
    field_name: Some("required".to_string()),
    field_number: Some(25),
    key_type: None,
    value_type: None,
    subscript: None,
  }]
});
