use std::sync::LazyLock;

use proto_types::{protovalidate::Violation, Duration, Timestamp};

use crate::{
  field_data::FieldContext,
  validators::{static_data::base_violations::create_violation, FieldPathElement},
  ProtoType,
};

pub fn within(
  field_context: &FieldContext,
  value: Timestamp,
  time_range: Duration,
  error_message: &'static str,
) -> Result<(), Violation> {
  let check = value.is_within_range_from_now(time_range);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &TIMESTAMP_WITHIN_VIOLATION,
      "timestamp.within",
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
      "timestamp.lt_now",
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
      "timestamp.gt_now",
      "must be in the future",
    ))
  }
}

static TIMESTAMP_WITHIN_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("timestamp".to_string()),
      field_number: Some(22),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("within".to_string()),
      field_number: Some(9),
      field_type: Some(ProtoType::Message as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

static TIMESTAMP_GT_NOW_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("timestamp".to_string()),
      field_number: Some(22),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("gt_now".to_string()),
      field_number: Some(8),
      field_type: Some(ProtoType::Bool as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

static TIMESTAMP_LT_NOW_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("timestamp".to_string()),
      field_number: Some(22),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("lt_now".to_string()),
      field_number: Some(7),
      field_type: Some(ProtoType::Bool as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});
