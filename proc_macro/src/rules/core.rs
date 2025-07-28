use proc_macro2::Span;
use prost_reflect::{FieldDescriptor, Kind};
use syn::Error;

use super::{field_rules::Type as RulesType, FieldData, ProtoType, ValidatorCallTemplate};
use crate::rules::{
  enum_rules::get_enum_rules, map_rules::get_map_rules, repeated_rules::get_repeated_rules,
  string_rules::get_string_rules,
};

pub fn get_field_rules(
  field_rust_enum: Option<String>,
  field_span: Span,
  field_desc: &FieldDescriptor,
  field_data: &FieldData,
  field_rules: &RulesType,
) -> Result<Vec<ValidatorCallTemplate>, Error> {
  let mut rules_agg: Vec<ValidatorCallTemplate> = Vec::new();
  let mut error: Option<Error> = None;

  let field_name = &field_data.proto_name;
  let field_kind = &field_desc.kind();

  // Check if repeated or map rules are being used on fields directly

  match field_rules {
    RulesType::Repeated(repeated_rules) => {
      if !field_data.is_repeated {
        error = Some(Error::new(
          field_span,
          format!(
            "Cannot use repeated rules for non repeated field {}",
            field_name
          ),
        ));
      } else {
        let rules = get_repeated_rules(
          field_rust_enum,
          field_desc,
          field_span,
          field_data,
          repeated_rules,
        )?;
        rules_agg.push(rules);
      }
    }
    RulesType::Map(map_rules) => {
      if !field_data.is_map {
        error = Some(Error::new(
          field_span,
          format!("Cannot use map rules for non map field {}", field_name),
        ));
      } else {
        let rules = get_map_rules(
          field_rust_enum,
          field_span,
          field_desc,
          field_data,
          map_rules,
        )?;
        rules_agg.push(rules);
      }
    }
    RulesType::Enum(enum_rules) => {
      if field_rust_enum.is_none() {
        error = Some(Error::new(
          field_span,
          format!(
            "Could not find the rust path to the generated enum for field {}",
            field_name
          ),
        ));
      } else if let Kind::Enum(enum_descriptor) = &field_kind {
        let rules = get_enum_rules(
          field_rust_enum.unwrap(),
          field_span,
          enum_descriptor,
          field_data,
          enum_rules,
        )?;
        rules_agg.extend(rules);
      } else {
        error = Some(Error::new(
          field_span,
          format!(
            "Could not find enum descriptor for enum field {}",
            field_name
          ),
        ))
      }
    }
    RulesType::String(string_rules) => {
      if !matches!(&field_kind, Kind::String) {
        error = Some(field_mismatch_error(
          field_name, "string", field_kind, field_span,
        ))
      } else {
        let rules = get_string_rules(field_span, field_data, string_rules)?;
        rules_agg.extend(rules);
      }
    }
    // RulesType::String(string_rules) => string_rules::get_string_rules(field_data, &string_rules),
    _ => {}
  };

  if let Some(err) = error {
    return Err(err);
  }

  Ok(rules_agg)
}

pub fn field_mismatch_error(
  field_name: &str,
  rule_type: &str,
  field_type: &Kind,
  span: Span,
) -> Error {
  Error::new(
    span,
    format!(
      "Wrong rule type for field {}. Rule type: {}, Field Type: {:#?}",
      field_name, rule_type, field_type
    ),
  )
}

pub fn convert_kind_to_proto_type(kind: &Kind) -> ProtoType {
  match kind {
    Kind::Double => ProtoType::Double,
    Kind::Float => ProtoType::Float,
    Kind::Int32 => ProtoType::Int32,
    Kind::Int64 => ProtoType::Int64,
    Kind::Uint32 => ProtoType::Uint32,
    Kind::Uint64 => ProtoType::Uint64,
    Kind::Sint32 => ProtoType::Sint32,
    Kind::Sint64 => ProtoType::Sint64,
    Kind::Fixed32 => ProtoType::Fixed32,
    Kind::Fixed64 => ProtoType::Fixed64,
    Kind::Sfixed32 => ProtoType::Sfixed32,
    Kind::Sfixed64 => ProtoType::Sfixed64,
    Kind::Bool => ProtoType::Bool,
    Kind::String => ProtoType::String,
    Kind::Bytes => ProtoType::Bytes,
    Kind::Message(_) => ProtoType::Message,
    Kind::Enum(_) => ProtoType::Enum,
  }
}
