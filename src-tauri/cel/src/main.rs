use bytes::Bytes;
use prost::Message;
use prost_reflect::{DescriptorPool, ExtensionDescriptor, MessageDescriptor, Value};
use regex::Regex;

use crate::buf::validate::{FieldRules, PredefinedRules};

mod myapp {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/myapp.v1.rs"));
  }
}
mod buf {
  pub mod validate {
    include!(concat!(env!("OUT_DIR"), "/buf.validate.rs"));
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let descriptor_set_bytes = Bytes::from(std::fs::read(std::env::var("PROTO_DESCRIPTOR_SET")?)?);
  let pool = DescriptorPool::decode(descriptor_set_bytes)?;

  let user_desc = pool
    .get_message_by_name("myapp.v1.User")
    .ok_or("User message not found")?;

  let field_ext_descriptor = pool
    .get_extension_by_name("buf.validate.field")
    .ok_or("buf.validate.field extension not found in descriptor pool")?;

  println!("--- User Message Validation Rules ---");

  for field_desc in user_desc.fields() {
    let field_name = field_desc.name();
    println!("\nField: {}", field_name);

    let field_options = field_desc.options();

    let field_rules_descriptor = field_options.get_extension(&field_ext_descriptor);

    if let Value::Message(field_rules_msg) = field_rules_descriptor.as_ref() {
      let field_rules = FieldRules::decode(field_rules_msg.encode_to_vec().as_slice())?;

      let is_required = field_rules.required();

      if let Some(rules_type) = field_rules.r#type {
        let rules = get_field_rules(&pool, &rules_type);
        println!("Rules: {:#?}", rules);
      }
    }
  }

  Ok(())
}

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

#[derive(Debug, Clone)]
struct CelRule {
  id: String,
  message: String,
  expression: String,
  value: CelRuleValue,
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
  RepeatedU64(Vec<u64>),
  RepeatedI32(Vec<i32>),
  RepeatedF32(Vec<f32>),
  RepeatedF64(Vec<f64>),
  Unspecified, // Or a better default/error handling for when a value isn't set.
}

fn get_field_rules(
  pool: &DescriptorPool,
  rules_type: &buf::validate::field_rules::Type,
) -> Result<Vec<CelRule>, Box<dyn std::error::Error>> {
  let mut rules: Vec<CelRule> = Vec::new();
  let predefined_descriptor = pool
    .get_extension_by_name("buf.validate.predefined")
    .ok_or("buf.validate.predefined not found")?;
  match rules_type {
    buf::validate::field_rules::Type::String(string_rules) => {
      let rule_category = "string";
      let string_rules_desc = pool
        .get_message_by_name("buf.validate.StringRules")
        .ok_or("StringRules message not found")?;

      if string_rules.pattern.is_some() {
        let pattern_value = string_rules.pattern.clone().unwrap();

        let (expression, message) = get_rule(
          &predefined_descriptor,
          &string_rules_desc,
          rule_category,
          "pattern",
        )?;

        let compiled_regex = Regex::new(&pattern_value)?;
        rules.push(CelRule {
          id: "string.pattern".to_string(),
          message: message.to_string(),
          expression: expression.to_string(),
          value: CelRuleValue::Regex(Box::new(compiled_regex)),
        });
      }

      if string_rules.len.is_some() {
        let len_value = string_rules.len.unwrap();

        let (expression, message) = get_rule(
          &predefined_descriptor,
          &string_rules_desc,
          rule_category,
          "len",
        )?;
        rules.push(CelRule {
          id: "string.len".to_string(),
          message: message.to_string(),
          expression: expression.to_string(),
          value: CelRuleValue::U64(len_value),
        });
      }

      if string_rules.max_len.is_some() {
        let max_len_value = string_rules.max_len.unwrap();

        let (expression, message) = get_rule(
          &predefined_descriptor,
          &string_rules_desc,
          rule_category,
          "max_len",
        )?;
        rules.push(CelRule {
          id: "string.max_len".to_string(),
          message: message.to_string(),
          expression: expression.to_string(),
          value: CelRuleValue::U64(max_len_value),
        });
      }

      if string_rules.min_len.is_some() {
        let min_len_value = string_rules.min_len.unwrap();

        let (expression, message) = get_rule(
          &predefined_descriptor,
          &string_rules_desc,
          rule_category,
          "min_len",
        )?;
        rules.push(CelRule {
          id: "string.min_len".to_string(),
          message: message.to_string(),
          expression: expression.to_string(),
          value: CelRuleValue::U64(min_len_value),
        });
      }

      if string_rules.len_bytes.is_some() {
        let len_bytes_value = string_rules.len_bytes.unwrap();

        let (expression, message) = get_rule(
          &predefined_descriptor,
          &string_rules_desc,
          rule_category,
          "len_bytes",
        )?;
        rules.push(CelRule {
          id: "string.len_bytes".to_string(),
          message: message.to_string(),
          expression: expression.to_string(),
          value: CelRuleValue::U64(len_bytes_value),
        });
      }

      if string_rules.min_bytes.is_some() {
        let min_bytes_value = string_rules.min_bytes.unwrap();

        let (expression, message) = get_rule(
          &predefined_descriptor,
          &string_rules_desc,
          rule_category,
          "min_bytes",
        )?;
        rules.push(CelRule {
          id: "string.min_bytes".to_string(),
          message: message.to_string(),
          expression: expression.to_string(),
          value: CelRuleValue::U64(min_bytes_value),
        });
      }

      if string_rules.max_bytes.is_some() {
        let max_bytes_value = string_rules.max_bytes.unwrap();

        let (expression, message) = get_rule(
          &predefined_descriptor,
          &string_rules_desc,
          rule_category,
          "max_bytes",
        )?;
        rules.push(CelRule {
          id: "string.max_bytes".to_string(),
          message: message.to_string(),
          expression: expression.to_string(),
          value: CelRuleValue::U64(max_bytes_value),
        });
      }

      if string_rules.r#const.is_some() {
        let const_value = string_rules.r#const.clone().unwrap();

        let (expression, message) = get_rule(
          &predefined_descriptor,
          &string_rules_desc,
          rule_category,
          "const",
        )?;
        rules.push(CelRule {
          id: "string.const".to_string(),
          message: message.to_string(),
          expression: expression.to_string(),
          value: CelRuleValue::String(const_value),
        });
      }
    }
    _ => {}
  };

  Ok(rules)
}

#[derive(Debug)]
struct CustomError(String);

impl std::fmt::Display for CustomError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl std::error::Error for CustomError {}
