use crate::{
  field_data::FieldContext,
  protovalidate::{FieldPath, FieldPathElement, Violation},
  validators::static_data::base_violations::get_base_violations_path,
  ProtoType,
};

pub fn required(field_context: &FieldContext) -> Violation {
  let mut elements = field_context.parent_elements.to_vec();

  let current_elem = FieldPathElement {
    field_type: Some(field_context.field_data.proto_type as i32),
    field_name: Some(field_context.field_data.proto_name.clone()),
    field_number: Some(field_context.field_data.tag as i32),
    key_type: field_context.field_data.key_type.map(|t| t as i32),
    value_type: field_context.field_data.value_type.map(|t| t as i32),
    subscript: field_context.subscript.clone(),
  };
  elements.push(current_elem);

  let mut violations_path = get_base_violations_path(&field_context.field_data.kind);

  let required_violation_path = FieldPathElement {
    field_type: Some(ProtoType::Bool as i32),
    field_name: Some("required".to_string()),
    field_number: Some(25),
    key_type: None,
    value_type: None,
    subscript: None,
  };
  violations_path.push(required_violation_path);

  Violation {
    message: Some(format!(
      "{} is required",
      field_context.field_data.proto_name
    )),
    rule_id: Some("field.required".to_string()),
    rule: Some(FieldPath {
      elements: violations_path,
    }),
    field: Some(FieldPath { elements }),
    for_key: Some(field_context.field_data.kind.is_map_key()),
  }
}
