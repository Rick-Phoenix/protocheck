#![allow(clippy::all, dead_code, unused)]
use crate::validator::buf::validate::field_path_element::Subscript;
use buf::validate::{
  field_rules, FieldPath, FieldPathElement, FieldRules, Ignore, MessageRules, OneofRules,
  PredefinedRules, Rule, Violation,
};
use bytes::Bytes;
use proc_macro2::TokenStream as TokenStream2;
use prost_reflect::{
  prost::Message, DescriptorPool, ExtensionDescriptor, MessageDescriptor, Value,
};

use syn::DeriveInput;

use proc_macro::TokenStream;

use std::collections::HashMap;
use std::sync::LazyLock;

use google::protobuf::{Duration, Timestamp};
use regex::Regex;

pub mod any_rules;
pub mod bool_rules;
pub mod bytes_rules;
pub mod duration_rules;
pub mod enum_rules;
pub mod map_rules;
pub mod numeric_rules;
pub mod repeated_rules;
pub mod string_rules;
pub mod timestamp_rules;

#[derive(Debug, Clone)]
pub struct CelRule {
  pub id: String,
  pub message: String,
  pub expression: String,
  pub value: CelRuleValue,
}

#[derive(Debug, Clone)]
pub enum CelRuleValue {
  Bool(bool),
  U64(u64),
  I32(i32),
  I64(i64),
  F32(f32),
  F64(f64),
  String(String),
  Regex(Box<Regex>),
  Bytes(Vec<u8>),
  RepeatedString(Vec<String>),
  RepeatedBytes(Vec<Vec<u8>>),
  RepeatedU64(Vec<u64>),
  RepeatedI64(Vec<i64>),
  RepeatedI32(Vec<i32>),
  RepeatedF32(Vec<f32>),
  RepeatedF64(Vec<f64>),
  RepeatedDuration(Vec<Duration>),
  Duration(Duration),
  Timestamp(Timestamp),
  Unspecified,
}

pub struct FieldValidationRules {
  pub cel_rules: Vec<Rule>,
  pub rules: Vec<CelRule>,
  pub required: bool,
}

static COMMON_RULES: LazyLock<HashMap<String, (String, String)>> = LazyLock::new(|| {
  let mut rules: HashMap<String, (String, String)> = HashMap::new();

  rules.insert("in".to_string(), ("!(this in getField(rules, 'in')) ? 'value must be in list %s'.format([getField(rules, 'in')]) : ''".to_string(), "".to_string()));
  rules.insert(
    "not_in".to_string(),
    (
      "this in rules.not_in ? 'value must not be in list %s'.format([rules.not_in]) : ''"
        .to_string(),
      "".to_string(),
    ),
  );
  rules.insert("const".to_string(), ("this != getField(rules, 'const') ? 'value must equal %s'.format([getField(rules, 'const')]) : ''".to_string(), "".to_string()));

  rules
});

use google::protobuf::field_descriptor_proto::Type as ProtoTypes;

static DUMMY_VIOLATION: LazyLock<Violation> = LazyLock::new(|| {
  let violation = Violation {
    rule_id: Some("example".to_string()),
    message: Some(format!("example")),
    for_key: Some(false),
    field: Some(FieldPath {
      elements: vec![FieldPathElement {
        field_type: Some(ProtoTypes::String.into()),
        field_name: Some("example".to_string()),
        key_type: None,
        value_type: None,
        field_number: Some(1),
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
  violation
});

pub fn get_field_rules(
  field_name: &str,
  field_tag: u32,
  field_rules: &FieldRules,
) -> Result<Vec<TokenStream2>, Box<dyn std::error::Error>> {
  if let Some(rules_type) = field_rules.r#type.clone() {
    match rules_type {
      field_rules::Type::String(string_rules) => {
        string_rules::get_string_rules(field_name, field_tag, &string_rules)
      }
      // field_rules::Type::Int64(int64_rules) => numeric_rules::get_int64_rules(&int64_rules),
      // field_rules::Type::Int32(int32_rules) => numeric_rules::get_int32_rules(&int32_rules),
      // field_rules::Type::Bytes(bytes_rules) => bytes_rules::get_bytes_rules(&bytes_rules),
      // field_rules::Type::Bool(bool_rules) => bool_rules::get_bool_rules(&bool_rules),
      // field_rules::Type::Enum(enum_rules) => enum_rules::get_enum_rules(&enum_rules),
      // field_rules::Type::Repeated(repeated_rules) => {
      //   repeated_rules::get_repeated_rules(&repeated_rules)
      // }
      // field_rules::Type::Map(map_rules) => map_rules::get_map_rules(&map_rules),
      // field_rules::Type::Any(any_rules) => any_rules::get_any_rules(&any_rules),
      // field_rules::Type::Duration(dur_rules) => duration_rules::get_duration_rules(&dur_rules),
      // field_rules::Type::Timestamp(time_rules) => timestamp_rules::get_timestamp_rules(&time_rules),
      _ => Ok(Vec::new()),
    }
  } else {
    Ok(Vec::new())
  }
}

#[derive(Debug)]
struct CustomError(String);

impl std::fmt::Display for CustomError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl std::error::Error for CustomError {}

fn get_rule(
  predefined_descriptor: &ExtensionDescriptor,
  rules_descriptor: &MessageDescriptor,
  rule_category: &str,
  rule_name: &str,
) -> Result<(String, String), Box<dyn std::error::Error>> {
  let rule_field_descriptor = rules_descriptor
    .get_field_by_name(rule_name)
    .ok_or(format!(
      "rule descriptor for rule `{}` not found",
      rule_name
    ))?;

  let rule_options = rule_field_descriptor.options();

  if let Value::Message(predefined_dynamic_msg) =
    rule_options.get_extension(&predefined_descriptor).as_ref()
  {
    let predefined_rules =
      PredefinedRules::decode(predefined_dynamic_msg.encode_to_vec().as_slice())?;

    for rule in predefined_rules.cel {
      if rule.id() == format!("{}.{}", rule_category, rule_name) {
        return Ok((rule.expression().to_string(), rule.message().to_string()));
      }
    }
  };

  return Err(Box::new(CustomError(format!(
    "rule {}.{} not found",
    rule_category, rule_name
  ))));
}

mod buf {
  pub mod validate {
    include!(concat!(env!("OUT_DIR"), "/buf.validate.rs"));
  }
}
mod google {
  pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
  }
}

pub(crate) struct ValidationData {
  pub field: String,
  pub tokens: TokenStream,
}

pub fn extract_validators(input_tokens: DeriveInput) -> Result<Vec<TokenStream2>, syn::Error> {
  let mut validation_data: Vec<TokenStream2> = Vec::new();
  let range = input_tokens.ident;
  let struct_name = range.to_string();
  let descriptor_set_bytes =
    Bytes::from(std::fs::read(std::env::var("PROTO_DESCRIPTOR_SET").unwrap()).unwrap());
  let pool = DescriptorPool::decode(descriptor_set_bytes).unwrap();

  let user_desc = pool
    .get_message_by_name("myapp.v1.User")
    .ok_or(syn::Error::new_spanned(range, "User message not found"))?;

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
    let field_name = field_desc.name();
    // println!("\nField: {}", field_name);

    let is_repeated = field_desc.is_list();
    let is_map = field_desc.is_map();

    let is_optional = field_desc.supports_presence();

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

      if field_rules.cel.len() > 0 {
        let cel_rules = field_rules.cel.clone();
      }

      let rules = get_field_rules(field_name, field_desc.number(), &field_rules).unwrap();
      validation_data.extend(rules);
      // println!("Rules: {:#?}", rules);
    }
  }

  Ok(validation_data)
}
