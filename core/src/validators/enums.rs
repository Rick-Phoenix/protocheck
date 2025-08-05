use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPath, FieldPathElement, Violation},
  validators::static_data::base_violations::get_base_violations_path,
  ProtoType,
};

pub fn defined_only(field_context: &FieldContext, enum_name: &str) -> Violation {
  let mut elements = field_context.parent_elements.to_vec();
  let current_elem = FieldPathElement {
    field_type: Some(ProtoType::Enum as i32),
    field_name: Some(field_context.proto_name.to_string()),
    key_type: field_context.key_type.map(|t| t as i32),
    value_type: field_context.value_type.map(|t| t as i32),
    field_number: Some(field_context.tag as i32),
    subscript: field_context.subscript.clone(),
  };
  elements.push(current_elem);

  let mut violations_path = get_base_violations_path(field_context.field_kind);

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
      field_context.proto_name.clone(),
      enum_name,
    )),
    field: Some(FieldPath { elements }),
    rule: Some(FieldPath {
      elements: violations_path,
    }),
    for_key: None,
  }
}
