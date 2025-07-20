use crate::validators::buf::validate::Violations;

pub mod buf {
  pub mod validate {
    include!(concat!(env!("OUT_DIR"), "/buf.validate.rs"));
  }
}
mod google {
  pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
  }
}

pub trait WithValidator {
  fn validate(&self) -> Result<(), Violations>;
}

pub struct FieldData {
  pub name: String,
  pub tag: i32,
  pub is_repeated: bool,
  pub is_map: bool,
}

pub mod strings {
  use crate::validators::{
    buf::validate::{field_path_element::Subscript, FieldPath, FieldPathElement, Violation},
    google::protobuf::field_descriptor_proto::Type as ProtoTypes,
  };

  pub fn max_len(
    field_name: String,
    field_tag: u32,
    value: &String,
    max_len: usize,
  ) -> Result<(), Violation> {
    let check = value.chars().count() < max_len;
    if !check {
      let violation = Violation {
        rule_id: Some("string.max_len".to_string()),
        message: Some(format!(
          "{} cannot be longer than {} characters",
          field_name, max_len
        )),
        for_key: Some(false),
        field: Some(FieldPath {
          elements: vec![FieldPathElement {
            field_type: Some(ProtoTypes::String.into()),
            field_name: Some(field_name),
            key_type: None,
            value_type: None,
            field_number: Some(field_tag as i32),
            subscript: None,
          }],
        }),
        rule: Some(FieldPath {
          elements: vec![FieldPathElement {
            key_type: Some(0),
            field_type: Some(0),
            value_type: Some(0),
            field_name: Some("".to_string()),
            field_number: Some(1),
            subscript: Some(Subscript::BoolKey(true)),
          }],
        }),
      };
      return Err(violation);
    };
    Ok(())
  }
}
