use std::collections::HashSet;

use crate::validator::{
  core::{convert_kind_to_proto_type, get_field_rules},
  enum_rules::{self, get_enum_rules},
  map_rules::{self, get_map_rules},
  pool_loader::DESCRIPTOR_POOL,
  repeated_rules, string_rules,
};
use proc_macro::Span;
use proc_macro2::{Ident, TokenStream as TokenStream2};
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
use proto_types::{FieldData, GeneratedCodeKind, ValidatorCallTemplate};
use quote::{quote, ToTokens};
use syn::token::Continue;
use syn::DeriveInput;

pub fn extract_validators(
  input_tokens: DeriveInput,
) -> Result<Vec<ValidatorCallTemplate>, syn::Error> {
  let mut validation_data: Vec<ValidatorCallTemplate> = Vec::new();
  let range = &input_tokens.ident;
  let struct_name = range.to_string();

  let message_name = if struct_name == "User" {
    "User".to_string()
  } else if struct_name == "Post" {
    "User.Post".to_string()
  } else {
    "".to_string()
  };

  let message = DESCRIPTOR_POOL.get_message_by_name(format!("myapp.v1.{}", message_name).as_str());

  if !message.is_some() {
    return Ok(validation_data);
  };

  let user_desc = message.unwrap();

  let field_ext_descriptor = DESCRIPTOR_POOL
    .get_extension_by_name("buf.validate.field")
    .ok_or("buf.validate.field extension not found in descriptor pool")
    .unwrap();

  let message_ext_descriptor = DESCRIPTOR_POOL
    .get_extension_by_name("buf.validate.message")
    .ok_or("buf.validate.message extension not found in descriptor pool")
    .unwrap();

  let oneof_ext_descriptor = DESCRIPTOR_POOL
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
    println!("{:?}", oneof.name());
    if let Value::Message(oneof_rules_msg) = oneof
      .options()
      .get_extension(&oneof_ext_descriptor)
      .as_ref()
    {
      let oneof_rules = OneofRules::decode(oneof_rules_msg.encode_to_vec().as_slice()).unwrap();
      if oneof_rules.required() {
        let name = oneof.name();
        let field_data = FieldData {
          rust_name: name.to_string(),
          proto_name: name.to_string(),
          tag: 0,
          is_repeated: false,
          is_map: false,
          is_required: false,
          is_optional: false,
          is_for_key: false,
          key_type: None,
          value_type: None,
          proto_type: ProtoType::Bool,
          enum_full_name: None,
          ignore: Ignore::IfZeroValue,
        };
        validation_data.push(ValidatorCallTemplate {
          validator_path: None,
          target_value_tokens: None,
          field_data,
          kind: GeneratedCodeKind::OneofRule,
        });
      }
    }
  }

  // println!("Struct Name: {}", struct_name.to_string());

  for field_desc in user_desc.fields() {
    // println!("{}", user_desc.name());

    let field_name = field_desc.name();
    // println!("{}", field_name.to_string());
    let is_repeated = field_desc.is_list();
    let is_map = field_desc.is_map();
    let is_optional = field_desc.supports_presence();

    let field_rust_ident = field_desc.json_name(); // Or derive from proto name
    let field_tag = field_desc.number();

    let field_options = field_desc.options();

    let field_rules_descriptor = field_options.get_extension(&field_ext_descriptor);

    if let Value::Message(field_rules_msg) = field_rules_descriptor.as_ref() {
      let field_rules = FieldRules::decode(field_rules_msg.encode_to_vec().as_slice()).unwrap();
      let ignore_val = field_rules.ignore();

      if matches!(ignore_val, Ignore::Always) {
        continue;
      }

      let is_required = field_rules.required();

      // println!("Enum Name: {:?}", enum_full_name);

      let mut field_data = FieldData {
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
        enum_full_name: None,
        is_optional,
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
          field_rules::Type::Enum(enum_rules) => match field_desc.kind() {
            Kind::Enum(enum_descriptor) => {
              field_data.enum_full_name = Some(enum_descriptor.full_name().to_string());
              let enum_values: HashSet<i32> =
                enum_descriptor.values().map(|e| e.number()).collect();
              get_enum_rules(field_data, &enum_rules, enum_values).unwrap()
            }
            _ => {
              return Err(syn::Error::new_spanned(
                input_tokens,
                "Found enum rules on a non-enum field.",
              ))
            }
          },
          _ => get_field_rules(field_data, &field_rules).unwrap(),
        };

        validation_data.extend(rules);
        // println!("Rules: {:#?}", rules);
      }
    }
  }

  Ok(validation_data)
}
