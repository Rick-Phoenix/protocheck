use proto_types::{protovalidate::violations_data::timestamp_violations::*, Duration, Timestamp};

use super::*;

pub fn within(
  field_context: &FieldContext,
  value: Timestamp,
  time_range: Duration,
  error_message: &str,
) -> Result<(), Violation> {
  let check = value.is_within_range_from_now(time_range);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &TIMESTAMP_WITHIN_VIOLATION,
      error_message,
    ))
  }
}

pub fn lt_now(field_context: &FieldContext, value: Timestamp) -> Result<(), Violation> {
  let check = value.is_past();

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &TIMESTAMP_LT_NOW_VIOLATION,
      "must be in the past",
    ))
  }
}

pub fn gt_now(field_context: &FieldContext, value: Timestamp) -> Result<(), Violation> {
  let check = value.is_future();

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &TIMESTAMP_GT_NOW_VIOLATION,
      "must be in the future",
    ))
  }
}
