use proc_macro2::TokenStream;
use prost_reflect::{FieldDescriptor, Kind};
use protocheck_core::field_data::FieldKind;
use syn::Error;

use super::{field_rules::Type as RulesType, ProtoType, ValidatorTemplate};
use crate::{
  rules::{
    any_rules::get_any_rules, bool_rules::get_bool_rules, bytes_rules::get_bytes_rules,
    duration_rules::get_duration_rules, enum_rules::get_enum_rules,
    numeric_rules::get_numeric_rules, string_rules::get_string_rules,
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
  let mut error: Option<&str> = None;

  let field_name = &validation_data.full_name;
  let field_proto_kind = &field_desc.kind();
  let field_data_kind = &validation_data.field_kind;
  let field_span = validation_data.field_span;

  let error_prefix = &format!("Error for field {}:", field_name);

  field_rules.matches_type(
    &validation_data.field_data.proto_type,
    field_span,
    error_prefix,
  )?;

  match field_rules {
    RulesType::Float(rules) => {
      let rules = get_numeric_rules(validation_data, rules)?;
      rules_agg.extend(rules);
    }
    RulesType::Double(rules) => {
      let rules = get_numeric_rules(validation_data, rules)?;
      rules_agg.extend(rules);
    }
    RulesType::Int32(rules) => {
      let rules = get_numeric_rules(validation_data, rules)?;
      rules_agg.extend(rules);
    }
    RulesType::Int64(rules) => {
      let rules = get_numeric_rules(validation_data, rules)?;
      rules_agg.extend(rules);
    }
    RulesType::Uint32(rules) => {
      let rules = get_numeric_rules(validation_data, rules)?;
      rules_agg.extend(rules);
    }
    RulesType::Uint64(rules) => {
      let rules = get_numeric_rules(validation_data, rules)?;
      rules_agg.extend(rules);
    }
    RulesType::Sint32(rules) => {
      let rules = get_numeric_rules(validation_data, rules)?;
      rules_agg.extend(rules);
    }
    RulesType::Sint64(rules) => {
      let rules = get_numeric_rules(validation_data, rules)?;
      rules_agg.extend(rules);
    }
    RulesType::Fixed32(rules) => {
      let rules = get_numeric_rules(validation_data, rules)?;
      rules_agg.extend(rules);
    }
    RulesType::Fixed64(rules) => {
      let rules = get_numeric_rules(validation_data, rules)?;
      rules_agg.extend(rules);
    }
    RulesType::Sfixed32(rules) => {
      let rules = get_numeric_rules(validation_data, rules)?;
      rules_agg.extend(rules);
    }
    RulesType::Sfixed64(rules) => {
      let rules = get_numeric_rules(validation_data, rules)?;
      rules_agg.extend(rules);
    }
    RulesType::String(string_rules) => {
      let rules = get_string_rules(static_defs, field_desc, validation_data, string_rules)?;
      rules_agg.extend(rules);
    }
    RulesType::Enum(enum_rules) => {
      if let Kind::Enum(enum_descriptor) = &field_proto_kind {
        match field_rust_enum {
          Some(enum_ident) => {
            let rules = get_enum_rules(enum_ident, enum_descriptor, validation_data, enum_rules)?;
            rules_agg.extend(rules);
          }
          None => error = Some("could not find enum field ident"),
        };
      } else {
        error = Some("could not find enum descriptor")
      }
    }
    RulesType::Duration(duration_rules) => {
      if !matches!(field_data_kind, FieldKind::Duration) {
        error = Some("cannot use duration rules on a field that is not a Duration")
      } else {
        let rules = get_duration_rules(validation_data, duration_rules)?;
        rules_agg.extend(rules);
      }
    }
    RulesType::Timestamp(timestamp_rules) => {
      if !matches!(&field_data_kind, FieldKind::Timestamp) {
        error = Some("cannot use timestamp rules on a field that is not a Timestamp")
      } else {
        let rules = get_timestamp_rules(validation_data, timestamp_rules)?;
        rules_agg.extend(rules);
      }
    }
    RulesType::Any(any_rules) => {
      if !matches!(&field_data_kind, FieldKind::Any) {
        error = Some("cannot use 'any' rules on a field that is not Any")
      } else {
        let rules = get_any_rules(validation_data, any_rules)?;
        rules_agg.extend(rules);
      }
    }
    RulesType::Bool(bool_rules) => {
      let rules = get_bool_rules(validation_data, bool_rules)?;
      rules_agg.extend(rules);
    }
    RulesType::Bytes(bytes_rules) => {
      let rules = get_bytes_rules(static_defs, field_desc, validation_data, bytes_rules)?;
      rules_agg.extend(rules);
    }
    _ => {}
  };

  if let Some(err) = error {
    return Err(Error::new(field_span, format!("{} {}", error_prefix, err)));
  }

  Ok(rules_agg)
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
