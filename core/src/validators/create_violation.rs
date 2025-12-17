use super::*;
use crate::protovalidate::{MAP_KEY_VIOLATION, MAP_VALUE_VIOLATION, REPEATED_ITEM_VIOLATION};

pub(crate) fn create_violation_core(
  custom_rule_id: Option<&str>,
  field_context: Option<&FieldContext>,
  parent_elements: &[FieldPathElement],
  violation_data: &ViolationData,
  error_message: &str,
) -> Violation {
  let mut field_elements: Option<Vec<FieldPathElement>> = None;
  let mut rule_elements: Vec<FieldPathElement> = Vec::new();
  let mut is_for_key = false;

  // In case of a top level message with CEL violations applied to the message
  // as a whole, there would be no field path
  if let Some(field_context) = field_context {
    let elements = field_elements.get_or_insert_default();

    elements.extend(parent_elements.iter().cloned());

    let current_elem = FieldPathElement {
      field_type: Some(field_context.field_type as i32),
      field_name: Some(field_context.proto_name.to_string()),
      key_type: field_context.key_type.map(|t| t as i32),
      value_type: field_context.value_type.map(|t| t as i32),
      field_number: Some(field_context.tag),
      subscript: field_context.subscript.clone(),
    };

    elements.push(current_elem);

    match &field_context.field_kind {
      FieldKind::MapKey => {
        is_for_key = true;
        rule_elements.extend(MAP_KEY_VIOLATION.elements.to_vec());
      }
      FieldKind::MapValue => rule_elements.extend(MAP_VALUE_VIOLATION.elements.to_vec()),
      FieldKind::RepeatedItem => rule_elements.extend(REPEATED_ITEM_VIOLATION.elements.to_vec()),
      _ => {}
    };
  }

  rule_elements.extend(violation_data.elements.to_vec());

  Violation {
    rule_id: Some(
      custom_rule_id.map_or_else(|| violation_data.name.to_string(), |id| id.to_string()),
    ),
    message: Some(error_message.to_string()),
    for_key: Some(is_for_key),
    field: field_elements.map(|elements| FieldPath { elements }),
    rule: Some(FieldPath {
      elements: rule_elements,
    }),
  }
}

pub(crate) fn create_violation(
  field_context: &FieldContext,
  violation_data: &ViolationData,
  error_message: &str,
  parent_elements: &[FieldPathElement],
) -> Violation {
  create_violation_core(
    None,
    Some(field_context),
    parent_elements,
    violation_data,
    error_message,
  )
}

pub(crate) fn create_violation_with_custom_id(
  rule_id: &str,
  field_context: &FieldContext,
  violation_data: &ViolationData,
  error_message: &str,
  parent_elements: &[FieldPathElement],
) -> Violation {
  create_violation_core(
    Some(rule_id),
    Some(field_context),
    parent_elements,
    violation_data,
    error_message,
  )
}
