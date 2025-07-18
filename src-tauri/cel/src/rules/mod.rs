use std::collections::HashMap;

use regex::Regex;

use crate::buf::validate;
use crate::buf::validate::field_rules;
use crate::buf::validate::Int32Rules;
use crate::buf::validate::Int64Rules;
use crate::buf::validate::PredefinedRules;
use prost::Message;
use prost_reflect::{DescriptorPool, ExtensionDescriptor, MessageDescriptor, Value};

pub mod bool_rules;
pub mod bytes_rules;
pub mod numeric_rules;
pub mod string_rules;

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
  Unspecified,
}

pub enum NumericRules {
  Int64(Int64Rules),
  Int32(Int32Rules),
}

lazy_static! {
  static ref COMMON_RULES: HashMap<String, (String, String)> = {
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
  };
}

pub fn get_field_rules(
  pool: &DescriptorPool,
  rules_type: &validate::field_rules::Type,
) -> Result<Vec<CelRule>, Box<dyn std::error::Error>> {
  match rules_type {
    field_rules::Type::String(string_rules) => string_rules::get_string_rules(string_rules),
    field_rules::Type::Int64(int64_rules) => numeric_rules::get_numeric_rules::<i64>(int64_rules),
    _ => Ok(Vec::new()),
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
