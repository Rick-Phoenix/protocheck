use super::*;
use crate::protovalidate::{MAP_KEY_VIOLATION, MAP_VALUE_VIOLATION, REPEATED_ITEM_VIOLATION};

fn create_violation_core(
  custom_rule_id: Option<&str>,
  field_context: &FieldContext,
  violation_data: &ViolationData,
  error_message: &str,
) -> Violation {
  let mut field_elements = field_context.parent_elements.to_vec();

  let current_elem = FieldPathElement {
    field_type: Some(field_context.field_kind.inner_type().into()),
    field_name: Some(field_context.proto_name.to_string()),
    key_type: field_context.key_type.map(|t| t as i32),
    value_type: field_context.value_type.map(|t| t as i32),
    field_number: Some(field_context.tag),
    subscript: field_context.subscript.clone(),
  };

  field_elements.push(current_elem);

  let mut rule_elements: Vec<FieldPathElement> = Vec::new();

  match &field_context.field_kind {
    FieldKind::MapKey(_) => rule_elements.extend(MAP_KEY_VIOLATION.elements.to_vec()),
    FieldKind::MapValue(_) => rule_elements.extend(MAP_VALUE_VIOLATION.elements.to_vec()),
    FieldKind::RepeatedItem(_) => rule_elements.extend(REPEATED_ITEM_VIOLATION.elements.to_vec()),
    _ => {}
  };

  rule_elements.extend(violation_data.elements.to_vec());

  Violation {
    rule_id: Some(
      custom_rule_id.map_or_else(|| violation_data.name.to_string(), |id| id.to_string()),
    ),
    message: Some(error_message.to_string()),
    for_key: field_context
      .field_kind
      .is_map_key()
      .then_some(true),
    field: Some(FieldPath {
      elements: field_elements,
    }),
    rule: Some(FieldPath {
      elements: rule_elements,
    }),
  }
}

pub(crate) fn create_violation(
  field_context: &FieldContext,
  violation_data: &ViolationData,
  error_message: &str,
) -> Violation {
  create_violation_core(None, field_context, violation_data, error_message)
}

pub(crate) fn create_violation_with_custom_id(
  rule_id: &str,
  field_context: &FieldContext,
  violation_data: &ViolationData,
  error_message: &str,
) -> Violation {
  create_violation_core(Some(rule_id), field_context, violation_data, error_message)
}
