use std::collections::HashSet;

use proc_macro2::{Ident, Span, TokenStream};
use prost_reflect::{FieldDescriptor, Kind as ProstReflectKind};
use proto_types::FieldType;
use quote::{quote, ToTokens};
use syn::Error;

use super::{field_rules::Type as RulesType, ProtoType};
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
) -> Result<TokenStream, Error> {
  let mut rules_tokens = TokenStream::new();
  let mut error: Option<&str> = None;

  let field_name = &validation_data.full_name;
  let field_proto_kind = &field_desc.kind();
  let field_span = validation_data.field_span;

  let error_prefix = &format!("Error for field {}:", field_name);

  field_rules.matches_type(
    validation_data.field_kind.inner_type(),
    field_span,
    error_prefix,
  )?;

  match field_rules {
    RulesType::Float(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Double(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Int32(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Int64(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Uint32(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Uint64(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Sint32(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Sint64(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Fixed32(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Fixed64(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Sfixed32(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Sfixed64(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::String(string_rules) => {
      let rules = get_string_rules(static_defs, field_desc, validation_data, string_rules)?;
      rules_tokens.extend(rules);
    }
    RulesType::Enum(enum_rules) => {
      if let ProstReflectKind::Enum(enum_descriptor) = &field_proto_kind {
        match field_rust_enum {
          Some(enum_ident) => {
            let rules = get_enum_rules(
              enum_ident,
              enum_descriptor,
              validation_data,
              enum_rules,
              static_defs,
            )?;
            rules_tokens.extend(rules);
          }
          None => error = Some("could not find enum field ident"),
        };
      } else {
        error = Some("could not find enum descriptor")
      }
    }
    RulesType::Duration(duration_rules) => {
      let rules = get_duration_rules(validation_data, duration_rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Timestamp(timestamp_rules) => {
      let rules = get_timestamp_rules(validation_data, timestamp_rules)?;
      rules_tokens.extend(rules);
    }
    RulesType::Any(any_rules) => {
      let rules = get_any_rules(validation_data, any_rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Bool(bool_rules) => {
      let rules = get_bool_rules(validation_data, bool_rules)?;
      rules_tokens.extend(rules);
    }
    RulesType::Bytes(bytes_rules) => {
      let rules = get_bytes_rules(validation_data, bytes_rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    _ => {}
  };

  if let Some(err) = error {
    return Err(Error::new(field_span, format!("{} {}", error_prefix, err)));
  }

  Ok(rules_tokens)
}

pub fn get_field_type(field_desc: &FieldDescriptor) -> FieldType {
  match field_desc.kind() {
    ProstReflectKind::Message(message_desc) => match message_desc.full_name() {
      "google.protobuf.Duration" => FieldType::Duration,
      "google.protobuf.Timestamp" => FieldType::Timestamp,
      "google.protobuf.Any" => FieldType::Any,
      _ => FieldType::Message,
    },
    ProstReflectKind::Double => FieldType::Double,
    ProstReflectKind::Float => FieldType::Float,
    ProstReflectKind::Int32 => FieldType::Int32,
    ProstReflectKind::Int64 => FieldType::Int64,
    ProstReflectKind::Uint32 => FieldType::Uint32,
    ProstReflectKind::Uint64 => FieldType::Uint64,
    ProstReflectKind::Sint32 => FieldType::Sint32,
    ProstReflectKind::Sint64 => FieldType::Sint64,
    ProstReflectKind::Fixed32 => FieldType::Fixed32,
    ProstReflectKind::Fixed64 => FieldType::Fixed64,
    ProstReflectKind::Sfixed32 => FieldType::Sfixed32,
    ProstReflectKind::Sfixed64 => FieldType::Sfixed64,
    ProstReflectKind::Bool => FieldType::Bool,
    ProstReflectKind::String => FieldType::String,
    ProstReflectKind::Bytes => FieldType::Bytes,
    ProstReflectKind::Enum(_) => FieldType::Enum,
  }
}

pub fn convert_kind_to_proto_type(kind: ProstReflectKind) -> ProtoType {
  match kind {
    ProstReflectKind::Double => ProtoType::Double,
    ProstReflectKind::Float => ProtoType::Float,
    ProstReflectKind::Int32 => ProtoType::Int32,
    ProstReflectKind::Int64 => ProtoType::Int64,
    ProstReflectKind::Uint32 => ProtoType::Uint32,
    ProstReflectKind::Uint64 => ProtoType::Uint64,
    ProstReflectKind::Sint32 => ProtoType::Sint32,
    ProstReflectKind::Sint64 => ProtoType::Sint64,
    ProstReflectKind::Fixed32 => ProtoType::Fixed32,
    ProstReflectKind::Fixed64 => ProtoType::Fixed64,
    ProstReflectKind::Sfixed32 => ProtoType::Sfixed32,
    ProstReflectKind::Sfixed64 => ProtoType::Sfixed64,
    ProstReflectKind::Bool => ProtoType::Bool,
    ProstReflectKind::String => ProtoType::String,
    ProstReflectKind::Bytes => ProtoType::Bytes,
    ProstReflectKind::Message(_) => ProtoType::Message,
    ProstReflectKind::Enum(_) => ProtoType::Enum,
  }
}

pub fn hashset_to_tokens<T>(hashset: HashSet<T>, type_tokens: &TokenStream) -> TokenStream
where
  T: ToTokens,
{
  let set_ident = Ident::new("set", Span::call_site());
  let mut tokens = quote! {
    let mut #set_ident: ::std::collections::HashSet<#type_tokens> = ::std::collections::HashSet::new();
  };

  for item in hashset {
    tokens.extend(quote! {
      #set_ident.insert(#item);
    });
  }

  tokens.extend(quote! {
    #set_ident
  });

  tokens
}
