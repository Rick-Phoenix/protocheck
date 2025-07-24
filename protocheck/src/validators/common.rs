use std::sync::LazyLock;

use crate::{
  field_data::FieldData,
  protovalidate::{FieldPath, FieldPathElement, Violation},
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

pub fn get_base_violations_path(
  is_repeated_item: bool,
  is_map_key: bool,
  is_map_value: bool,
) -> Vec<FieldPathElement> {
  let mut violations_path = vec![];

  if is_repeated_item {
    violations_path.extend(REPEATED_ITEM_VIOLATION.clone());
  } else if is_map_key {
    violations_path.extend(MAP_KEY_VIOLATION.clone());
  } else if is_map_value {
    violations_path.extend(MAP_VALUE_VIOLATION.clone());
  }

  violations_path
}
