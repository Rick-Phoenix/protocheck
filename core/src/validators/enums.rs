use std::sync::LazyLock;

use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPathElement, Violation},
  validators::static_data::base_violations::create_violation,
  ProtoType,
};

pub fn defined_only(field_context: &FieldContext, enum_name: &str) -> Violation {
  create_violation(
    field_context,
    &ENUM_DEFINED_ONLY_VIOLATION,
    "enum.defined_only",
    &format!("must be a defined value of `{enum_name}`"),
  )
}

static ENUM_DEFINED_ONLY_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("enum".to_string()),
      field_number: Some(16),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("defined_only".to_string()),
      field_number: Some(2),
      field_type: Some(ProtoType::Bool as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});
