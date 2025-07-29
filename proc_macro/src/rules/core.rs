use proc_macro2::Span;
use prost_reflect::{FieldDescriptor, Kind};
use syn::Error;

use super::{field_rules::Type as RulesType, FieldData, ProtoType, ValidatorTemplate};
use crate::rules::{enum_rules::get_enum_rules, string_rules::get_string_rules};

pub fn get_field_rules(
  field_rust_enum: Option<String>,
  field_span: Span,
  field_desc: &FieldDescriptor,
  field_data: &FieldData,
  field_rules: &RulesType,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut rules_agg: Vec<ValidatorTemplate> = Vec::new();
  let mut error: Option<Error> = None;

  let field_name = &field_data.proto_name;
  let field_kind = &field_desc.kind();

  let error_prefix = format!("Error for field {}:", field_name);

  match field_rules {
    RulesType::Enum(enum_rules) => {
      if let Kind::Enum(enum_descriptor) = &field_kind {
        match field_rust_enum {
          Some(enum_ident) => {
            let rules = get_enum_rules(
              enum_ident,
              field_span,
              enum_descriptor,
              field_data,
              enum_rules,
            )?;
            rules_agg.extend(rules);
          }
          None => {
            error = Some(Error::new(
              field_span,
              format!("{} could not find enum field ident", error_prefix),
            ))
          }
        };
      } else {
        error = Some(Error::new(
          field_span,
          format!("{} could not find enum descriptor", error_prefix),
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
