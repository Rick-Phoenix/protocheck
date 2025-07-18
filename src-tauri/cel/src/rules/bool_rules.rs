
use super::CelRule;

use crate::{ rules::CelRuleValue};
use crate::buf::validate::{ BoolRules};

pub fn get_bool_rules(
  bool_rules: BoolRules,
) -> Result<Vec<CelRule>, Box<dyn std::error::Error>> {
  let mut rules: Vec<CelRule> = Vec::new();

  if bool_rules.r#const.is_some() {
    let const_val = bool_rules.r#const();
    rules.push(CelRule {    
      id: "bool.const".to_string(),
      expression: "this != getField(rules, 'const') ? 'value must equal %s'.format([getField(rules, 'const')]) : ''".to_string(),
      message: "".to_string(), 
      value: CelRuleValue::Bool(const_val), 
    })
  }

  Ok(rules)
}

