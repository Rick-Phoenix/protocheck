use super::get_rule;
use super::CelRule;
use super::CelRuleValue;
use crate::buf::validate;
use prost_reflect::DescriptorPool;
use regex::Regex;

use crate::buf::validate::StringRules;

pub fn get_string_rules(
  pool: &DescriptorPool,
  string_rules: &StringRules,
) -> Result<Vec<CelRule>, Box<dyn std::error::Error>> {
  let mut rules: Vec<CelRule> = Vec::new();
  let predefined_descriptor = pool
    .get_extension_by_name("buf.validate.predefined")
    .ok_or("buf.validate.predefined not found")?;
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

  if string_rules.prefix.is_some() {
    let prefix_value = string_rules.prefix.clone().unwrap();

    let (expression, message) = get_rule(
      &predefined_descriptor,
      &string_rules_desc,
      rule_category,
      "prefix",
    )?;
    rules.push(CelRule {
      id: "string.prefix".to_string(),
      message: message.to_string(),
      expression: expression.to_string(),
      value: CelRuleValue::String(prefix_value),
    });
  }

  if string_rules.suffix.is_some() {
    let suffix_value = string_rules.suffix.clone().unwrap();

    let (expression, message) = get_rule(
      &predefined_descriptor,
      &string_rules_desc,
      rule_category,
      "suffix",
    )?;
    rules.push(CelRule {
      id: "string.suffix".to_string(),
      message: message.to_string(),
      expression: expression.to_string(),
      value: CelRuleValue::String(suffix_value),
    });
  }

  if string_rules.contains.is_some() {
    let contains_value = string_rules.contains.clone().unwrap();

    let (expression, message) = get_rule(
      &predefined_descriptor,
      &string_rules_desc,
      rule_category,
      "contains",
    )?;
    rules.push(CelRule {
      id: "string.contains".to_string(),
      message: message.to_string(),
      expression: expression.to_string(),
      value: CelRuleValue::String(contains_value),
    });
  }

  if string_rules.not_contains.is_some() {
    let not_contains_value = string_rules.not_contains.clone().unwrap();

    let (expression, message) = get_rule(
      &predefined_descriptor,
      &string_rules_desc,
      rule_category,
      "not_contains",
    )?;
    rules.push(CelRule {
      id: "string.not_contains".to_string(),
      message: message.to_string(),
      expression: expression.to_string(),
      value: CelRuleValue::String(not_contains_value),
    });
  }

  if string_rules.r#in.len() > 0 {
    let in_value = string_rules.r#in.clone();

    let (expression, message) = get_rule(
      &predefined_descriptor,
      &string_rules_desc,
      rule_category,
      "in",
    )?;
    rules.push(CelRule {
      id: "string.in".to_string(),
      message: message.to_string(),
      expression: expression.to_string(),
      value: CelRuleValue::RepeatedString(in_value),
    });
  }

  if string_rules.not_in.len() > 0 {
    let not_in_value = string_rules.not_in.clone();

    let (expression, message) = get_rule(
      &predefined_descriptor,
      &string_rules_desc,
      rule_category,
      "not_in",
    )?;
    rules.push(CelRule {
      id: "string.not_in".to_string(),
      message: message.to_string(),
      expression: expression.to_string(),
      value: CelRuleValue::RepeatedString(not_in_value),
    });
  }

  if let Some(well_known) = string_rules.well_known {
    let mut get_well_known = |wk| -> Result<(), Box<dyn std::error::Error>> {
      let (expression, message) = get_rule(
        &predefined_descriptor,
        &string_rules_desc,
        rule_category,
        wk,
      )?;

      rules.push(CelRule {
        id: format!("string.{}", wk).to_string(),
        message: message.to_string(),
        expression: expression.to_string(),
        value: CelRuleValue::Bool(true),
      });

      Ok(())
    };
    match well_known {
      validate::string_rules::WellKnown::Email(_) => {
        get_well_known("email")?;
      }
      validate::string_rules::WellKnown::Hostname(_) => {
        get_well_known("hostname")?;
      }
      validate::string_rules::WellKnown::Ip(_) => {
        get_well_known("ip")?;
      }
      validate::string_rules::WellKnown::Ipv4(_) => {
        get_well_known("ipv4")?;
      }
      validate::string_rules::WellKnown::Ipv6(_) => {
        get_well_known("ipv6")?;
      }
      validate::string_rules::WellKnown::Uri(_) => {
        get_well_known("uri")?;
      }
      validate::string_rules::WellKnown::UriRef(_) => {
        get_well_known("uri_ref")?;
      }
      validate::string_rules::WellKnown::Address(_) => {
        get_well_known("address")?;
      }
      validate::string_rules::WellKnown::Uuid(_) => {
        get_well_known("uuid")?;
      }
      validate::string_rules::WellKnown::Tuuid(_) => {
        get_well_known("tuuid")?;
      }
      validate::string_rules::WellKnown::IpWithPrefixlen(_) => {
        get_well_known("ip_with_prefixlen")?;
      }
      validate::string_rules::WellKnown::Ipv4WithPrefixlen(_) => {
        get_well_known("ipv4_with_prefixlen")?;
      }
      validate::string_rules::WellKnown::Ipv6WithPrefixlen(_) => {
        get_well_known("ipv6_with_prefixlen")?;
      }
      validate::string_rules::WellKnown::IpPrefix(_) => {
        get_well_known("ip_prefix")?;
      }
      validate::string_rules::WellKnown::Ipv4Prefix(_) => {
        get_well_known("ipv4_prefix")?;
      }
      validate::string_rules::WellKnown::Ipv6Prefix(_) => {
        get_well_known("ipv6_prefix")?;
      }
      validate::string_rules::WellKnown::HostAndPort(_) => {
        get_well_known("host_and_port")?;
      }
      validate::string_rules::WellKnown::WellKnownRegex(well_known_regex_id) => {
        match well_known_regex_id {
          1 => {
            get_well_known("well_known_regex.header_name")?;
          }
          2 => {
            get_well_known("well_known_regex.header_value")?;
          }
          _ => {}
        }
      }
    };
  }

  Ok(rules)
}
