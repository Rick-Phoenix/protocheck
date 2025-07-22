use proto_types::FieldData;

use proto_types::{
  buf::validate::{FieldPath, FieldPathElement, Violation},
  google::protobuf::field_descriptor_proto::Type as ProtoTypes,
};

pub fn max_len(
  field_data: FieldData,
  value: Option<&str>,
  max_len: usize,
) -> Result<(), Violation> {
  let check = if value.is_some() {
    let unwrapped_val = value.unwrap();
    unwrapped_val.chars().count() < max_len
  } else {
    return Ok(());
  };

  let plural_suffix = if max_len > 1 {
    format!("s")
  } else {
    format!("")
  };

  if !check {
    let mut elements = field_data.parent_elements.to_vec();
    let current_elem = FieldPathElement {
      field_type: Some(ProtoTypes::String.into()),
      field_name: Some(field_data.name.clone()),
      key_type: field_data.key_type.map(|t| t as i32),
      value_type: field_data.value_type.map(|t| t as i32),
      field_number: Some(field_data.tag as i32),
      subscript: field_data.subscript,
    };
    elements.push(current_elem);
    let violation = Violation {
      rule_id: Some("string.max_len".to_string()),
      message: Some(format!(
        "{} cannot be longer than {} character{}",
        field_data.name.clone(),
        max_len,
        plural_suffix
      )),
      for_key: Some(field_data.for_key),
      field: Some(FieldPath { elements: elements }),
      rule: Some(FieldPath {
        elements: vec![
          FieldPathElement {
            field_name: Some("string".to_string()),
            field_number: Some(14),
            field_type: Some(ProtoTypes::Message as i32),
            subscript: None,
            key_type: None,
            value_type: None,
          },
          FieldPathElement {
            field_name: Some("max_len".to_string()),
            field_number: Some(3),
            field_type: Some(ProtoTypes::Uint64 as i32),
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
