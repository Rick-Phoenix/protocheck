use super::CelRule;
use super::CelRuleValue;
use crate::rules::validate::AnyRules;

pub fn get_any_rules(any_rules: &AnyRules) -> Result<Vec<CelRule>, Box<dyn std::error::Error>> {
  let mut rules: Vec<CelRule> = Vec::new();

  if any_rules.r#in.len() > 0 {
    let in_values = any_rules.r#in.clone();

    let (expression, message) = super::COMMON_RULES.get("in").unwrap();
    rules.push(CelRule {
      id: "any.in".to_string(),
      message: message.to_string(),
      expression: expression.to_string(),
      value: CelRuleValue::RepeatedString(in_values),
    });
  }

  if any_rules.not_in.len() > 0 {
    let not_in_values = any_rules.not_in.clone();

    let (expression, message) = super::COMMON_RULES.get("not_in").unwrap();
    rules.push(CelRule {
      id: "any.not_in".to_string(),
      message: message.to_string(),
      expression: expression.to_string(),
      value: CelRuleValue::RepeatedString(not_in_values),
    });
  }

  Ok(rules)
}
