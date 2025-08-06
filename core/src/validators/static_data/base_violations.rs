use std::sync::LazyLock;

use proto_types::protovalidate::{FieldPath, Violation};

use crate::{
  field_data::{FieldContext, FieldKind},
  protovalidate::FieldPathElement,
  ProtoType,
};

static MAP_KEY_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
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
      field_name: Some("keys".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ]
});

static MAP_VALUE_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
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
      field_name: Some("values".to_string()),
      field_number: Some(5),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ]
});

static REPEATED_ITEM_VIOLATION: LazyLock<Vec<FieldPathElement>> = LazyLock::new(|| {
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
      field_name: Some("items".to_string()),
      field_number: Some(4),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
  ]
});

pub fn get_violation_elements(field_context: &FieldContext) -> Vec<FieldPathElement> {
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

pub fn get_base_violations_path(field_kind: FieldKind) -> Vec<FieldPathElement> {
  let mut violations_path = vec![];

  if field_kind.is_repeated_item() {
    violations_path.extend(REPEATED_ITEM_VIOLATION.clone());
  } else if field_kind.is_map_key() {
    violations_path.extend(MAP_KEY_VIOLATION.clone());
  } else if field_kind.is_map_value() {
    violations_path.extend(MAP_VALUE_VIOLATION.clone());
  }

  violations_path
}

pub fn create_violation(
  field_context: &FieldContext,
  violation_path: &'static [FieldPathElement],
  rule_id: &str,
  error_message: &str,
) -> Violation {
  let elements = get_violation_elements(field_context);

  let mut rule_elements = get_base_violations_path(field_context.field_kind);

  rule_elements.extend(violation_path.to_vec());

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
