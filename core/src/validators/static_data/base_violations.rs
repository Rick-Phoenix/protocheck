use proto_types::protovalidate::{FieldPath, Violation, ViolationData};

use crate::{
  field_data::{FieldContext, FieldKind},
  protovalidate::{
    FieldPathElement, MAP_KEY_VIOLATION, MAP_VALUE_VIOLATION, REPEATED_ITEM_VIOLATION,
  },
};

macro_rules! create_violation {
  ($proto_type:ident, $check:ident, $field_context:ident, $violation_name:ident, $error_message:expr ) => {
    paste! {
      if $check {
        Ok(())
      } else {
        Err(create_violation(
          $field_context,
          &[< $proto_type:upper _ $violation_name:upper _ VIOLATION >],
          $error_message,
        ))
      }
    }
  };
}

pub(crate) fn create_violation(
  field_context: &FieldContext,
  violation_data: &ViolationData,
  error_message: &str,
) -> Violation {
  let elements = get_violation_elements(field_context);

  let mut rule_elements = get_base_violations_path(field_context.field_kind);

  rule_elements.extend(violation_data.elements.to_vec());

  Violation {
    rule_id: Some(violation_data.name.to_string()),
    message: Some(error_message.to_string()),
    for_key: field_context.field_kind.is_map_key().then_some(true),
    field: Some(FieldPath { elements }),
    rule: Some(FieldPath {
      elements: rule_elements,
    }),
  }
}

pub(crate) fn create_violation_with_custom_id(
  rule_id: &str,
  field_context: &FieldContext,
  violation_data: &ViolationData,
  error_message: &str,
) -> Violation {
  let elements = get_violation_elements(field_context);

  let mut rule_elements = get_base_violations_path(field_context.field_kind);

  rule_elements.extend(violation_data.elements.to_vec());

  Violation {
    rule_id: Some(rule_id.to_string()),
    message: Some(error_message.to_string()),
    for_key: field_context.field_kind.is_map_key().then_some(true),
    field: Some(FieldPath { elements }),
    rule: Some(FieldPath {
      elements: rule_elements,
    }),
  }
}

pub(crate) fn get_violation_elements(field_context: &FieldContext) -> Vec<FieldPathElement> {
  let mut elements = field_context.parent_elements.to_vec();
  let current_elem = FieldPathElement {
    field_type: Some(field_context.field_kind.inner_type().into()),
    field_name: Some(field_context.proto_name.to_string()),
    key_type: field_context.key_type.map(|t| t as i32),
    value_type: field_context.value_type.map(|t| t as i32),
    field_number: Some(field_context.tag as i32),
    subscript: field_context.subscript.clone(),
  };

  elements.push(current_elem);

  elements
}

pub(crate) fn get_base_violations_path(field_kind: FieldKind) -> Vec<FieldPathElement> {
  let mut violations_path = vec![];

  if field_kind.is_repeated_item() {
    violations_path.extend(REPEATED_ITEM_VIOLATION.elements.to_vec());
  } else if field_kind.is_map_key() {
    violations_path.extend(MAP_KEY_VIOLATION.elements.to_vec());
  } else if field_kind.is_map_value() {
    violations_path.extend(MAP_VALUE_VIOLATION.elements.to_vec());
  }

  violations_path
}
