#![allow(clippy::all, dead_code, unused)]
use crate::validator::map_rules::get_map_rules;
use crate::validator::{map_rules, repeated_rules, string_rules};
use bytes::Bytes;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use prost_reflect::prost_types::Type;
use prost_reflect::FieldDescriptor;
use prost_reflect::{
  prost::Message, DescriptorPool, ExtensionDescriptor, Kind, MessageDescriptor, Value,
};
use proto_types::buf::validate::MapRules;
use proto_types::buf::validate::{
  field_path_element::Subscript, field_rules, FieldPath, FieldPathElement, FieldRules, Ignore,
  MessageRules, OneofRules, PredefinedRules, Rule, Violation,
};
use proto_types::google::protobuf::field_descriptor_proto::Type as ProtoType;
use proto_types::GeneratedCodeKind;
use proto_types::{FieldData, ValidatorCallTemplate};
use quote::{quote, ToTokens};

use syn::token::Continue;
use syn::DeriveInput;

use proc_macro::TokenStream;

use std::collections::HashMap;
use std::sync::LazyLock;

use proto_types::google::protobuf::{Duration, Timestamp};
use regex::Regex;

pub fn get_field_rules(
  field_data: FieldData,
  field_rules: &FieldRules,
) -> Result<Vec<ValidatorCallTemplate>, Box<dyn std::error::Error>> {
  if let Some(rules_type) = field_rules.r#type.clone() {
    match rules_type {
      field_rules::Type::String(string_rules) => {
        string_rules::get_string_rules(field_data, &string_rules)
      }
      // field_rules::Type::Int64(int64_rules) => numeric_rules::get_int64_rules(&int64_rules),
      // field_rules::Type::Int32(int32_rules) => numeric_rules::get_int32_rules(&int32_rules),
      // field_rules::Type::Bytes(bytes_rules) => bytes_rules::get_bytes_rules(&bytes_rules),
      // field_rules::Type::Bool(bool_rules) => bool_rules::get_bool_rules(&bool_rules),
      // field_rules::Type::Enum(enum_rules) => enum_rules::get_enum_rules(&enum_rules),
      field_rules::Type::Repeated(repeated_rules) => {
        repeated_rules::get_repeated_rules(field_data, &repeated_rules)
      }
      // field_rules::Type::Map(map_rules) => map_rules::get_map_rules(field_data, &map_rules),
      // field_rules::Type::Any(any_rules) => any_rules::get_any_rules(&any_rules),
      // field_rules::Type::Duration(dur_rules) => duration_rules::get_duration_rules(&dur_rules),
      // field_rules::Type::Timestamp(time_rules) => timestamp_rules::get_timestamp_rules(&time_rules),
      _ => Ok(Vec::new()),
    }
  } else {
    Ok(Vec::new())
  }
}

pub fn extract_validators(
  input_tokens: DeriveInput,
) -> Result<Vec<ValidatorCallTemplate>, syn::Error> {
  let mut validation_data: Vec<ValidatorCallTemplate> = Vec::new();
  let range = input_tokens.ident;
  let struct_name = range.to_string();

  // println!("{}", struct_name.to_string());

  let descriptor_set_bytes =
    Bytes::from(std::fs::read(std::env::var("PROTO_DESCRIPTOR_SET").unwrap()).unwrap());
  let pool = DescriptorPool::decode(descriptor_set_bytes).unwrap();

  let message_name = if struct_name == "User" {
    "User".to_string()
  } else if struct_name == "Post" {
    "User.Post".to_string()
  } else {
    "".to_string()
  };

  let message = pool.get_message_by_name(format!("myapp.v1.{}", message_name).as_str());

  if !message.is_some() {
    return Ok(validation_data);
  };

  let user_desc = message.unwrap();

  let field_ext_descriptor = pool
    .get_extension_by_name("buf.validate.field")
    .ok_or("buf.validate.field extension not found in descriptor pool")
    .unwrap();

  let message_ext_descriptor = pool
    .get_extension_by_name("buf.validate.message")
    .ok_or("buf.validate.message extension not found in descriptor pool")
    .unwrap();

  let oneof_ext_descriptor = pool
    .get_extension_by_name("buf.validate.oneof")
    .ok_or("buf.validate.oneof extension not found in descriptor pool")
    .unwrap();

  let message_options = user_desc.options();

  let message_rules_descriptor = message_options.get_extension(&message_ext_descriptor);

  if let Value::Message(message_rules_msg) = message_rules_descriptor.as_ref() {
    let message_rules = MessageRules::decode(message_rules_msg.encode_to_vec().as_slice()).unwrap();

    if message_rules.cel.len() > 0 {
      let message_cel_rules = message_rules.cel.clone();
    }

    if message_rules.oneof.len() > 0 {
      let message_oneof_rules = message_rules.oneof;
    }
  }

  for oneof in user_desc.oneofs() {
    if let Value::Message(oneof_rules_msg) = oneof
      .options()
      .get_extension(&oneof_ext_descriptor)
      .as_ref()
    {
      let oneof_rules = OneofRules::decode(oneof_rules_msg.encode_to_vec().as_slice()).unwrap();
      if oneof_rules.required() {
        //
      }
    }
  }

  for field_desc in user_desc.fields() {
    // println!("{}", user_desc.name());

    let field_name = field_desc.name();
    // println!("{}", field_name.to_string());
    let is_repeated = field_desc.is_list();
    let is_map = field_desc.is_map();
    // println!("{}", is_map);
    let is_optional = field_desc.supports_presence();

    let field_rust_ident = field_desc.json_name(); // Or derive from proto name
    let field_tag = field_desc.number();

    // println!("{:#?}", field_desc.kind());

    let field_options = field_desc.options();

    let field_rules_descriptor = field_options.get_extension(&field_ext_descriptor);

    if let Value::Message(field_rules_msg) = field_rules_descriptor.as_ref() {
      let field_rules = FieldRules::decode(field_rules_msg.encode_to_vec().as_slice()).unwrap();
      let mut ignore_val: Option<Ignore> = None;

      if field_rules.ignore.is_some() {
        match field_rules.ignore() {
          Ignore::Always => continue,
          Ignore::IfZeroValue => ignore_val = Some(Ignore::IfZeroValue),
          Ignore::Unspecified => ignore_val = Some(Ignore::Unspecified),
        }
      }

      let is_required = field_rules.required();

      let field_data = FieldData {
        rust_name: field_name.to_string(),
        proto_name: field_name.to_string(),
        proto_type: convert_kind_to_proto_type(field_desc.kind()),
        tag: field_tag,
        is_required,
        is_repeated,
        is_map,
        is_for_key: false,
        key_type: None,
        value_type: None,
        ignore: ignore_val,
        is_optional: is_optional,
      };

      if let Kind::Message(field_message_type) = field_desc.kind() {
        if !is_map {
          if field_desc.name() != "posts" {
            // println!("{}", field_desc.name());
            continue;
          }
          let template = ValidatorCallTemplate {
            validator_path: None,
            target_value_tokens: None,
            field_data,
            kind: GeneratedCodeKind::NestedMessageRecursion,
          };
          // println!("{:#?}", template);
          validation_data.push(template);
          continue;
        }
      }

      if field_rules.cel.len() > 0 {
        let cel_rules = field_rules.cel.clone();
      }

      if let Some(rules_type) = field_rules.r#type.clone() {
        let rules = match rules_type {
          field_rules::Type::Map(map_rules) => {
            vec![get_map_rules(&field_desc, &map_rules, ignore_val).unwrap()]
          }
          _ => get_field_rules(field_data, &field_rules).unwrap(),
        };

        validation_data.extend(rules);
        // println!("Rules: {:#?}", rules);
      }
    }
  }

  Ok(validation_data)
}

pub fn convert_kind_to_proto_type(kind: Kind) -> ProtoType {
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
