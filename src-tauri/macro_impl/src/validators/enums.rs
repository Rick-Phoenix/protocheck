use proto_types::FieldContext;

use proto_types::{
  buf::validate::{FieldPath, FieldPathElement, Violation},
  google::protobuf::field_descriptor_proto::Type as ProtoTypes,
};

pub fn defined_only(
  field_context: FieldContext,
  value: Option<&i32>,
  valid_values: &'static std::collections::HashSet<i32>,
) -> Result<(), Violation> {
  let check = if let Some(unwrapped_val) = value {
    valid_values.contains(unwrapped_val)
  } else {
    return Ok(());
  };

  if !check {
    let mut elements = field_context.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(ProtoTypes::Enum.into()),
      field_name: Some(field_context.field_data.proto_name.clone()),
      key_type: field_context.field_data.key_type.map(|t| t as i32),
      value_type: field_context.field_data.value_type.map(|t| t as i32),
      field_number: Some(field_context.field_data.tag as i32),
      subscript: field_context.subscript,
    };
    elements.push(current_elem);

    let violation = Violation {
      rule_id: Some("enum.defined_only".to_string()),
      message: Some(format!(
        "{} must be a defined value",
        field_context.field_data.proto_name.clone(),
      )),
      for_key: Some(field_context.field_data.is_for_key),
      field: Some(FieldPath { elements: elements }),
      rule: Some(FieldPath {
        elements: vec![
          FieldPathElement {
            field_name: Some("enum".to_string()),
            field_number: Some(15),
            field_type: Some(ProtoTypes::Message as i32),
            subscript: None,
            key_type: None,
            value_type: None,
          },
          FieldPathElement {
            field_name: Some("defined_only".to_string()),
            field_number: Some(1),
            field_type: Some(ProtoTypes::Bool as i32),
            key_type: None,
            value_type: None,
            subscript: None,
          },
        ],
      }),
    };
    return Err(violation);
  };
  Ok(())
}
