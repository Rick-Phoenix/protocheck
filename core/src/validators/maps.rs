use std::sync::LazyLock;

use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPathElement, Violation},
  validators::static_data::base_violations::create_violation,
  ProtoType,
};

pub fn min_pairs(
  field_context: &FieldContext,
  value: usize,
  min_pairs: u64,
) -> Result<(), Violation> {
  let check = value >= min_pairs as usize;

  if check {
    Ok(())
  } else {
    let plural_suffix = if min_pairs > 1 { "s" } else { "" };
    let error_message = format!("requires at least {} pair{}", min_pairs, plural_suffix);
    Err(create_violation(
      field_context,
      &MAP_MIN_PAIRS_VIOLATION,
      "map.min_pairs",
      &error_message,
    ))
  }
}

pub fn max_pairs(
  field_context: &FieldContext,
  value: usize,
  max_pairs: u64,
) -> Result<(), Violation> {
  let check = value <= max_pairs as usize;

  if check {
    Ok(())
  } else {
    let plural_suffix = if max_pairs > 1 { "s" } else { "" };
    let error_message = format!("cannot have more than {} pair{}", max_pairs, plural_suffix);
    Err(create_violation(
      field_context,
      &MAP_MAX_PAIRS_VIOLATION,
      "map.max_pairs",
      &error_message,
    ))
  }
}

static MAP_MIN_PAIRS_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("map".to_string()),
      field_number: Some(19),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("min_pairs".to_string()),
      field_number: Some(1),
      field_type: Some(ProtoType::Uint64 as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

static MAP_MAX_PAIRS_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("map".to_string()),
      field_number: Some(19),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("max_pairs".to_string()),
      field_number: Some(2),
      field_type: Some(ProtoType::Uint64 as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});
