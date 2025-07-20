use super::CelRule;

use crate::validator::buf::validate::EnumRules;
use crate::validator::CelRuleValue;

pub fn get_enum_rules(enum_rules: &EnumRules) -> Result<Vec<CelRule>, Box<dyn std::error::Error>> {
  let mut rules: Vec<CelRule> = Vec::new();

  if enum_rules.r#const.is_some() {
    let const_val = enum_rules.r#const();
    let (expression, message) = super::COMMON_RULES.get("const").unwrap();
    rules.push(CelRule {
      id: "enum.const".to_string(),
      expression: expression.to_string(),
      message: message.to_string(),
      value: CelRuleValue::I32(const_val),
    })
  }

  if enum_rules.defined_only.is_some() {
    rules.push(CelRule {
      id: "enum.defined_only".to_string(),
      message: "".to_string(),
      expression: "".to_string(),
      value: CelRuleValue::Bool(true),
    });
  }

  if enum_rules.r#in.len() > 0 {
    let in_value = enum_rules.r#in.clone();

    let (expression, message) = super::COMMON_RULES.get("in").unwrap();
    rules.push(CelRule {
      id: "enum.in".to_string(),
      message: message.to_string(),
      expression: expression.to_string(),
      value: CelRuleValue::RepeatedI32(in_value),
    });
  }

  if enum_rules.not_in.len() > 0 {
    let not_in_value = enum_rules.not_in.clone();

    let (expression, message) = super::COMMON_RULES.get("not_in").unwrap();
    rules.push(CelRule {
      id: "enum.not_in".to_string(),
      message: message.to_string(),
      expression: expression.to_string(),
      value: CelRuleValue::RepeatedI32(not_in_value),
    });
  }

  Ok(rules)
}
