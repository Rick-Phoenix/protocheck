use super::CelRule;
use super::CelRuleValue;
use crate::buf::validate;
use crate::buf::validate::BytesRules;
use regex::Regex;

pub fn get_bytes_rules(
  bytes_rules: &BytesRules,
) -> Result<Vec<CelRule>, Box<dyn std::error::Error>> {
  let mut rules: Vec<CelRule> = Vec::new();

  if bytes_rules.r#const.is_some() {
    let const_value = bytes_rules.r#const.clone().unwrap();

    rules.push(CelRule {
      id: "bytes.const".to_string(),
      message: "".to_string(),
      expression: "this != getField(rules, 'const') ? 'value must be %x'.format([getField(rules, 'const')]) : ''".to_string(),
      value: CelRuleValue::Bytes(const_value),
    });
  }

  if bytes_rules.len.is_some() {
    let len_value = bytes_rules.len.unwrap();

    rules.push(CelRule {
      id: "bytes.len".to_string(),
      message: "".to_string(),
      expression:
        "uint(this.size()) != rules.len ? 'value length must be %s bytes'.format([rules.len]) : ''"
          .to_string(),
      value: CelRuleValue::U64(len_value),
    });
  }

  if bytes_rules.max_len.is_some() {
    let max_len_value = bytes_rules.max_len.unwrap();

    rules.push(CelRule {
      id: "bytes.max_len".to_string(),
      message: "".to_string(),
      expression: "uint(this.size()) > rules.max_len ? 'value must be at most %s bytes'.format([rules.max_len]) : ''".to_string(),
      value: CelRuleValue::U64(max_len_value),
    });
  }

  if bytes_rules.min_len.is_some() {
    let min_len_value = bytes_rules.min_len.unwrap();

    rules.push(CelRule {
      id: "bytes.min_len".to_string(),
      message: "".to_string(),
      expression: "uint(this.size()) < rules.min_len ? 'value length must be at least %s bytes'.format([rules.min_len]) : ''".to_string(),
      value: CelRuleValue::U64(min_len_value),
    });
  }

  if bytes_rules.pattern.is_some() {
    let pattern_value = bytes_rules.pattern.clone().unwrap();

    let compiled_regex = Regex::new(&pattern_value)?;
    rules.push(CelRule {
      id: "bytes.pattern".to_string(),
      message: "".to_string(),
      expression: "!string(this).matches(rules.pattern) ? 'value must match regex pattern `%s`'.format([rules.pattern]) : ''".to_string(),
      value: CelRuleValue::Regex(Box::new(compiled_regex)),
    });
  }

  if bytes_rules.prefix.is_some() {
    let prefix_value = bytes_rules.prefix.clone().unwrap();

    rules.push(CelRule {
      id: "bytes.prefix".to_string(),
      message: "".to_string(),
      expression: "!this.startsWith(rules.prefix) ? 'value does not have prefix %x'.format([rules.prefix]) : ''".to_string(),
      value: CelRuleValue::Bytes(prefix_value),
    });
  }

  if bytes_rules.suffix.is_some() {
    let suffix_value = bytes_rules.suffix.clone().unwrap();

    rules.push(CelRule {
      id: "bytes.suffix".to_string(),
      message: "".to_string(),
      expression:
        "!this.endsWith(rules.suffix) ? 'value does not have suffix %x'.format([rules.suffix]) : ''"
          .to_string(),
      value: CelRuleValue::Bytes(suffix_value),
    });
  }

  if bytes_rules.contains.is_some() {
    let contains_value = bytes_rules.contains.clone().unwrap();

    rules.push(CelRule {
      id: "bytes.contains".to_string(),
      message: "".to_string(),
      expression:
        "!this.contains(rules.contains) ? 'value does not contain %x'.format([rules.contains]) : ''"
          .to_string(),
      value: CelRuleValue::Bytes(contains_value),
    });
  }

  if bytes_rules.r#in.len() > 0 {
    let in_value = bytes_rules.r#in.clone();

    rules.push(CelRule {
      id: "bytes.in".to_string(),
      message: "".to_string(),
      expression: "getField(rules, 'in').size() > 0 && !(this in getField(rules, 'in')) ? 'value must be in list %s'.format([getField(rules, 'in')]) : ''".to_string(),
      value: CelRuleValue::RepeatedBytes(in_value),
    });
  }

  if bytes_rules.not_in.len() > 0 {
    let not_in_value = bytes_rules.not_in.clone();

    rules.push(CelRule {
      id: "bytes.not_in".to_string(),
      message: "".to_string(),
      expression:
        "this in rules.not_in ? 'value must not be in list %s'.format([rules.not_in]) : ''"
          .to_string(),
      value: CelRuleValue::RepeatedBytes(not_in_value),
    });
  }

  if let Some(well_known) = bytes_rules.well_known {
    let mut get_well_known =
      |wk: &str, message: &str, expression: &str| -> Result<(), Box<dyn std::error::Error>> {
        rules.push(CelRule {
          id: format!("bytes.{}", wk).to_string(),
          message: message.to_string(),
          expression: expression.to_string(),
          value: CelRuleValue::Bool(true),
        });

        Ok(())
      };
    match well_known {
      validate::bytes_rules::WellKnown::Ip(_) => {
        get_well_known(
          "ip",
          "value must be a valid IP address",
          "!rules.ip || this.size() == 0 || this.size() == 4 || this.size() == 16",
        )?;
      }
      validate::bytes_rules::WellKnown::Ipv4(_) => {
        get_well_known(
          "ipv4",
          "value must be a valid IPv4 address",
          "!rules.ipv4 || this.size() == 0 || this.size() == 4",
        )?;
      }
      validate::bytes_rules::WellKnown::Ipv6(_) => {
        get_well_known(
          "ipv6",
          "value must be a valid IPv6 address",
          "!rules.ipv6 || this.size() == 0 || this.size() == 16",
        )?;
      }
    };
  }

  Ok(rules)
}
