use crate::{
  field_data::{FieldContext, FieldData},
  protovalidate::{FieldPath, FieldPathElement, Violation},
  validators::common::get_base_violations_path,
  ProtoType,
};

pub fn enum_contains<E>(value: i32) -> bool
where
  E: std::convert::TryFrom<i32>,
  E: std::fmt::Debug + Sized,
{
  E::try_from(value).is_ok()
}

pub fn defined_only(field_context: FieldContext, enum_name: &str) -> Violation {
  let mut elements = field_context.parent_elements.to_vec();
  let current_elem = FieldPathElement {
    field_type: Some(ProtoType::Enum.into()),
    field_name: Some(field_context.field_data.proto_name.clone()),
    key_type: field_context.field_data.key_type.map(|t| t as i32),
    value_type: field_context.field_data.value_type.map(|t| t as i32),
    field_number: Some(field_context.field_data.tag as i32),
    subscript: field_context.subscript,
  };

  elements.push(current_elem);

  let FieldData {
    is_repeated_item,
    is_map_key,
    is_map_value,
    ..
  } = field_context.field_data;

  let mut violations_path = get_base_violations_path(is_repeated_item, is_map_key, is_map_value);

  violations_path.extend(vec![
    FieldPathElement {
      field_name: Some("enum".to_string()),
      field_number: Some(16),
      field_type: Some(ProtoType::Message as i32),
      subscript: None,
      key_type: None,
      value_type: None,
    },
    FieldPathElement {
      field_name: Some("defined_only".to_string()),
      field_number: Some(2),
      field_type: Some(ProtoType::Bool as i32),
      key_type: None,
      value_type: None,
      subscript: None,
    },
  ]);

  Violation {
    rule_id: Some("enum.defined_only".to_string()),
    message: Some(format!(
      "field {} must be a defined value of {}",
      field_context.field_data.proto_name.clone(),
      enum_name,
    )),
    for_key: Some(field_context.field_data.is_map_key),
    field: Some(FieldPath { elements }),
    rule: Some(FieldPath {
      elements: violations_path,
    }),
  }
}
