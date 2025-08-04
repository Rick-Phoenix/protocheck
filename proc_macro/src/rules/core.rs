use proc_macro2::{Span, TokenStream};
use prost_reflect::{FieldDescriptor, Kind};
use protocheck_core::field_data::FieldKind;
use syn::Error;

use super::{field_rules::Type as RulesType, ProtoType, ValidatorTemplate};
use crate::{
  rules::{
    any_rules::get_any_rules, bool_rules::get_bool_rules, duration_rules::get_duration_rules,
    enum_rules::get_enum_rules, string_rules::get_string_rules,
    timestamp_rules::get_timestamp_rules,
  },
  validation_data::ValidationData,
};

pub fn get_field_rules(
  static_defs: &mut Vec<TokenStream>,
  field_rust_enum: Option<String>,
  field_desc: &FieldDescriptor,
  validation_data: &ValidationData,
  field_rules: &RulesType,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut rules_agg: Vec<ValidatorTemplate> = Vec::new();
  let mut error: Option<Error> = None;

  let field_name = &validation_data.full_name;
  let field_proto_kind = &field_desc.kind();
  let field_data_kind = &validation_data.field_data.kind;
  let field_span = validation_data.field_span;

  let error_prefix = &format!("Error for field {}:", field_name);

  match field_rules {
    RulesType::Enum(enum_rules) => {
      if let Kind::Enum(enum_descriptor) = &field_proto_kind {
        match field_rust_enum {
          Some(enum_ident) => {
            let rules = get_enum_rules(enum_ident, enum_descriptor, validation_data, enum_rules)?;
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
      if !matches!(&field_proto_kind, Kind::String) {
        error = Some(field_mismatch_error(
          error_prefix,
          "string",
          field_proto_kind,
          field_span,
        ))
      } else {
        let rules = get_string_rules(static_defs, field_desc, validation_data, string_rules)?;
        rules_agg.extend(rules);
      }
    }
    RulesType::Duration(duration_rules) => {
      if !matches!(field_data_kind, FieldKind::Duration) {
        error = Some(field_mismatch_error(
          error_prefix,
          "duration",
          field_proto_kind,
          field_span,
        ))
      } else {
        let rules = get_duration_rules(validation_data, duration_rules)?;
        rules_agg.extend(rules);
      }
    }
    RulesType::Timestamp(timestamp_rules) => {
      if !matches!(&field_data_kind, FieldKind::Timestamp) {
        error = Some(field_mismatch_error(
          error_prefix,
          "timestamp",
          field_proto_kind,
          field_span,
        ))
      } else {
        let rules = get_timestamp_rules(validation_data, timestamp_rules)?;
        rules_agg.extend(rules);
      }
    }
    RulesType::Any(any_rules) => {
      if !matches!(&field_data_kind, FieldKind::Any) {
        error = Some(field_mismatch_error(
          error_prefix,
          "any",
          field_proto_kind,
          field_span,
        ))
      } else {
        let rules = get_any_rules(validation_data, any_rules)?;
        rules_agg.extend(rules);
      }
    }
    RulesType::Bool(bool_rules) => {
      if !matches!(&field_proto_kind, Kind::Bool) {
        error = Some(field_mismatch_error(
          error_prefix,
          "bool",
          field_proto_kind,
          field_span,
        ))
      } else {
        let rules = get_bool_rules(validation_data, bool_rules)?;
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
  error_prefix: &str,
  rule_type: &str,
  field_type: &Kind,
  span: Span,
) -> Error {
  Error::new(
    span,
    format!(
      "{} wrong rule type. Rule type: {}, Field Type: {:#?}",
      error_prefix, rule_type, field_type
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
