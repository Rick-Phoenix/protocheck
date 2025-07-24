use proto_types::FieldContext;

use proto_types::{
  buf::validate::{FieldPath, FieldPathElement, Violation},
  google::protobuf::field_descriptor_proto::Type as ProtoTypes,
};

pub fn required(field_context: FieldContext) -> Violation {
  let elements = field_context.parent_elements.to_vec();
  let violation = Violation {
    rule_id: Some("oneof.required".to_string()),
    message: Some(format!(
      "at least one value for `{}` is required",
      field_context.field_data.proto_name.clone(),
    )),
    for_key: None,
    field: Some(FieldPath { elements: elements }),
    rule: Some(FieldPath {
      elements: vec![FieldPathElement {
        field_name: Some("required".to_string()),
        field_number: Some(1),
        field_type: Some(ProtoTypes::Bool as i32),
        subscript: None,
        key_type: None,
        value_type: None,
      }],
    }),
  };
  violation
}
