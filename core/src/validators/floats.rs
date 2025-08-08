use std::sync::LazyLock;

use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPathElement, Violation},
  validators::static_data::base_violations::create_violation,
  ProtoType,
};

pub fn f32_is_finite(field_context: &FieldContext, value: f32) -> Result<(), Violation> {
  let check = !value.is_nan();

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &FLOAT_FINITE_VIOLATION,
      "float.finite",
      "must be a finite number",
    ))
  }
}

pub fn f64_is_finite(field_context: &FieldContext, value: f64) -> Result<(), Violation> {
  let check = !value.is_nan();

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &DOUBLE_FINITE_VIOLATION,
      "double.finite",
      "must be a finite number",
    ))
  }
}

static FLOAT_FINITE_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("float".to_string()),
      field_number: Some(1),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("finite".to_string()),
      field_number: Some(8),
      field_type: Some(ProtoType::Bool as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

static DOUBLE_FINITE_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("double".to_string()),
      field_number: Some(2),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("finite".to_string()),
      field_number: Some(8),
      field_type: Some(ProtoType::Bool as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});
