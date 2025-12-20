use crate::{
  ProtoType,
  protovalidate::{FieldPath, FieldPathElement, Violation},
};

#[must_use]
pub fn required(name: &str, parent_elements: &[FieldPathElement]) -> Violation {
  let mut elements = parent_elements.to_vec();

  elements.push(FieldPathElement {
    field_name: Some(name.to_string()),
    field_number: None,
    field_type: None,
    subscript: None,
    key_type: None,
    value_type: None,
  });

  Violation {
    rule_id: Some("oneof.required".to_string()),
    message: Some("at least one value is required".to_string()),
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
