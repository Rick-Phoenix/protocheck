#![allow(clippy::all, dead_code, unused)]
use crate::validator::buf::validate::field_path_element::Subscript;
use buf::validate::{
  field_rules, FieldPath, FieldPathElement, FieldRules, Ignore, MessageRules, OneofRules,
  PredefinedRules, Rule, Violation,
};
use bytes::Bytes;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use prost_reflect::{
  prost::Message, DescriptorPool, ExtensionDescriptor, Kind, MessageDescriptor, Value,
};
use proto_types::FieldData;

use syn::DeriveInput;

use proc_macro::TokenStream;

use std::collections::HashMap;
use std::sync::LazyLock;

use google::protobuf::{Duration, Timestamp};
use regex::Regex;

pub mod any_rules;
pub mod bool_rules;
pub mod bytes_rules;
pub mod core;
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
