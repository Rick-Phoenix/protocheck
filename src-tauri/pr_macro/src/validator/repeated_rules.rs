use super::CelRule;
use super::CelRuleValue;
use crate::validator::buf::validate::RepeatedRules;
use crate::validator::get_field_rules;

pub fn get_repeated_rules(
  repeated_rules: &RepeatedRules,
) -> Result<Vec<CelRule>, Box<dyn std::error::Error>> {
  let mut rules: Vec<CelRule> = Vec::new();

  if repeated_rules.min_items.is_some() {
    let min_items_value = repeated_rules.min_items.unwrap();

    rules.push(CelRule {
      id: "repeated.min_items".to_string(),
      message: "".to_string(),
      expression: "uint(this.size()) < rules.min_items ? 'value must contain at least %d item(s)'.format([rules.min_items]) : ''".to_string(),
      value: CelRuleValue::U64(min_items_value),
    });
  }

  if repeated_rules.max_items.is_some() {
    let max_items_value = repeated_rules.max_items.unwrap();

    rules.push(CelRule {
      id: "repeated.max_items".to_string(),
      message: "".to_string(),
      expression: "uint(this.size()) > rules.max_items ? 'value must contain no more than %s item(s)'.format([rules.max_items]) : ''".to_string(),
      value: CelRuleValue::U64(max_items_value),
    });
  }

  if repeated_rules.unique.is_some() {
    rules.push(CelRule {
      id: "repeated.unique".to_string(),
      message: "".to_string(),
      expression: "!rules.unique || this.unique()".to_string(),
      value: CelRuleValue::Bool(true),
    });
  }

  if repeated_rules.items.is_some() {
    let items_rules_descriptor = repeated_rules.items.clone().unwrap();
    let items_rules: Vec<CelRule> = get_field_rules(&items_rules_descriptor)?
      .into_iter()
      .map(|mut rule| {
        rule.id = format!("repeated.items.{}", rule.id);
        rule
      })
      .collect();
    rules.extend(items_rules);
  }

  Ok(rules)
}
