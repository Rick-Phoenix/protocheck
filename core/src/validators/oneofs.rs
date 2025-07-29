use crate::{
  protovalidate::{FieldPath, FieldPathElement, Violation},
  ProtoType,
};

pub fn required(name: &str, parent_elements: &[FieldPathElement]) -> Violation {
  let elements = parent_elements.to_vec();

  Violation {
    rule_id: Some("oneof.required".to_string()),
    message: Some(format!("at least one value for `{}` is required", name,)),
    for_key: None,
    field: Some(FieldPath { elements }),
    rule: Some(FieldPath {
      elements: vec![FieldPathElement {
        field_name: Some("required".to_string()),
        field_number: Some(1),
        field_type: Some(ProtoType::Bool as i32),
        subscript: None,
        key_type: None,
        value_type: None,
      }],
    }),
  }
}
