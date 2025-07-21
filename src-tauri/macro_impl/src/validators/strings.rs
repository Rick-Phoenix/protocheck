use proto_types::FieldData;

use crate::validators::{
  buf::validate::{field_path_element::Subscript, FieldPath, FieldPathElement, Violation},
  google::protobuf::field_descriptor_proto::Type as ProtoTypes,
};

pub fn max_len(
  index: usize,
  field_data: FieldData,
  value: &str,
  max_len: usize,
) -> Result<(), Violation> {
  let check = value.chars().count() < max_len;
  let plural_suffix = if max_len > 1 {
    format!("s")
  } else {
    format!("")
  };

  println!("INDEX ====================== {}", index);

  if !check {
    let violation = Violation {
      rule_id: Some("string.max_len".to_string()),
      message: Some(format!(
        "{} cannot be longer than {} character{}",
        field_data.name, max_len, plural_suffix
      )),
      for_key: Some(false),
      field: Some(FieldPath {
        elements: vec![FieldPathElement {
          field_type: Some(ProtoTypes::String.into()),
          field_name: Some(field_data.name),
          key_type: None,
          value_type: None,
          field_number: Some(field_data.tag as i32),
          subscript: None,
        }],
      }),
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
