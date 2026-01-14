use super::*;
use crate::protovalidate::violations_data::{MAP_MAX_PAIRS_VIOLATION, MAP_MIN_PAIRS_VIOLATION};

pub fn min_pairs<K, V>(
  field_context: &FieldContext,
  parent_elements: &[FieldPathElement],
  value: &HashMap<K, V>,
  min_pairs: u64,
  error_message: &str,
) -> Result<(), Violation> {
  #[allow(clippy::cast_possible_truncation)]
  let is_valid = value.len() >= min_pairs as usize;

  if is_valid {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      MAP_MIN_PAIRS_VIOLATION,
      error_message,
      parent_elements,
    ))
  }
}

pub fn max_pairs<K, V>(
  field_context: &FieldContext,
  parent_elements: &[FieldPathElement],
  value: &HashMap<K, V>,
  max_pairs: u64,
  error_message: &str,
) -> Result<(), Violation> {
  #[allow(clippy::cast_possible_truncation)]
  let is_valid = value.len() <= max_pairs as usize;

  if is_valid {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      MAP_MAX_PAIRS_VIOLATION,
      error_message,
      parent_elements,
    ))
  }
}
