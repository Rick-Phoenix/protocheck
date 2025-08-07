use std::{collections::HashSet, hash::Hash, sync::LazyLock};

use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPathElement, Violation},
  validators::static_data::base_violations::create_violation,
  ProtoType,
};

pub fn min_items(
  field_context: &FieldContext,
  value: usize,
  min_items: u64,
) -> Result<(), Violation> {
  let check = value >= min_items as usize;

  if check {
    Ok(())
  } else {
    let plural_suffix = if min_items > 1 { "s" } else { "" };
    let error_message = format!("requires at least {} item{}", min_items, plural_suffix);

    Err(create_violation(
      field_context,
      &REPEATED_MIN_ITEMS_VIOLATION,
      "repeated.min_items",
      &error_message,
    ))
  }
}

pub fn max_items(
  field_context: &FieldContext,
  value: usize,
  max_items: u64,
) -> Result<(), Violation> {
  let check = value <= max_items as usize;

  if check {
    Ok(())
  } else {
    let plural_suffix = if max_items > 1 { "s" } else { "" };
    let error_message = format!("cannot have more than {} item{}", max_items, plural_suffix);

    Err(create_violation(
      field_context,
      &REPEATED_MAX_ITEMS_VIOLATION,
      "repeated.max_items",
      &error_message,
    ))
  }
}

pub fn unique<T>(
  field_context: &FieldContext,
  value: T,
  processed_values: &mut HashSet<T>,
) -> Result<(), Violation>
where
  T: Eq + Hash + Clone + ToString,
{
  let check = processed_values.insert(value.clone());

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &REPEATED_UNIQUE_VIOLATION,
      "repeated.unique",
      "must contain unique values",
    ))
  }
}

pub fn unique_f64(
  field_context: &FieldContext,
  value: f64,
  processed_values: &mut HashSet<u64>,
) -> Result<(), Violation>
where
{
  let bits = value.to_bits();
  let check = processed_values.insert(bits);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &REPEATED_UNIQUE_VIOLATION,
      "repeated.unique",
      "must contain unique values",
    ))
  }
}

pub fn unique_f32(
  field_context: &FieldContext,
  value: f32,
  processed_values: &mut HashSet<u32>,
) -> Result<(), Violation>
where
{
  let bits = value.to_bits();
  let check = processed_values.insert(bits);

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &REPEATED_UNIQUE_VIOLATION,
      "repeated.unique",
      "must contain unique values",
    ))
  }
}

static REPEATED_MIN_ITEMS_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("repeated".to_string()),
      field_number: Some(18),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("min_items".to_string()),
      field_number: Some(1),
      field_type: Some(ProtoType::Uint64 as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

static REPEATED_MAX_ITEMS_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("repeated".to_string()),
      field_number: Some(18),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("max_items".to_string()),
      field_number: Some(2),
      field_type: Some(ProtoType::Uint64 as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});

static REPEATED_UNIQUE_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
  vec![
    FieldPathElement {
      field_name: Some("repeated".to_string()),
      field_number: Some(18),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("unique".to_string()),
      field_number: Some(3),
      field_type: Some(ProtoType::Uint64 as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]
});
