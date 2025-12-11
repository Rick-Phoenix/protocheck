use super::*;
use crate::protovalidate::violations_data::{DOUBLE_FINITE_VIOLATION, FLOAT_FINITE_VIOLATION};

pub fn float_is_finite(field_context: &FieldContext, value: f32) -> Result<(), Violation> {
  let check = !value.is_nan();

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &FLOAT_FINITE_VIOLATION,
      "must be a finite number",
    ))
  }
}

pub fn double_is_finite(field_context: &FieldContext, value: f64) -> Result<(), Violation> {
  let check = !value.is_nan();

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &DOUBLE_FINITE_VIOLATION,
      "must be a finite number",
    ))
  }
}
