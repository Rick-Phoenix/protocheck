use std::collections::HashMap;

use super::CelRule;

use crate::{buf::validate::Int64Rules, rules::CelRuleValue};
use crate::buf::validate::int64_rules;

lazy_static! {
  static ref NUMERIC_RULES: HashMap<String, (String,String)> = {
    let mut rules_map: HashMap<String, (String,String)> = HashMap::new();

    rules_map.insert("const".to_string(), ("this != getField(rules, 'const') ? 'value must equal %s'.format([getField(rules, 'const')]) : ''".to_string(), "".to_string()));
    rules_map.insert("lt".to_string(), ("!has(rules.gte) && !has(rules.gt) && this >= rules.lt ? 'value must be less than %s'.format([rules.lt]) : ''".to_string(), "".to_string()));
    rules_map.insert("lte".to_string(), ("!has(rules.gte) && !has(rules.gt) && this > rules.lte ? 'value must be less than or equal to %s'.format([rules.lte]) : ''".to_string(), "".to_string()));
    rules_map.insert("gt".to_string(), ("!has(rules.lt) && !has(rules.lte) && this <= rules.gt ? 'value must be greater than %s'.format([rules.gt]) : ''".to_string(), "".to_string()));
    rules_map.insert("gte".to_string(), ("!has(rules.lt) && !has(rules.lte) && this < rules.gte ? 'value must be greater than or equal to %s'.format([rules.gte]) : ''".to_string(), "".to_string()));

    rules_map
  };
}

pub fn get_int_rules(
  numeric_rules: Int64Rules,
) -> Result<Vec<CelRule>, Box<dyn std::error::Error>> {
  let mut rules: Vec<CelRule> = Vec::new();


  if  numeric_rules.r#const.is_some() {
    let const_val = numeric_rules.r#const();
    rules.push(CelRule {    
      id: format!("int64.const"),
      expression: format!("this != getField(rules, 'const') ? 'value must equal %s'.format([getField(rules, 'const')]) : ''"),
      message: format!(""), 
      value: CelRuleValue::I64(const_val), 
    })
  }

  if numeric_rules.less_than.is_some() {
    match numeric_rules.less_than.unwrap() {
      int64_rules::LessThan::Lt(val) => {
        rules.push(CelRule {
          id: "int64.lt".to_string(),
          expression: "!has(rules.gte) && !has(rules.gt) && this >= rules.lt ? 'value must be less than %s'.format([rules.lt]) : ''".to_string(), 
          message: "".to_string(),
          value: CelRuleValue::I64(val),
        })
      } 
      int64_rules::LessThan::Lte(val) => {
        rules.push(CelRule {
          id: "int64.lte".to_string(),
          expression: "!has(rules.gte) && !has(rules.gt) && this > rules.lte ? 'value must be less than or equal to %s'.format([rules.lte]) : ''".to_string(),
          message: "".to_string(),
          value: CelRuleValue::I64(val),
        })

      }
    }
  }

    if numeric_rules.greater_than.is_some() {
    match numeric_rules.greater_than.unwrap() {
      int64_rules::GreaterThan::Gt(val) => {
        rules.push(CelRule {
          id: "int64.gt".to_string(),
          expression: "!has(rules.lt) && !has(rules.lte) && this <= rules.gt ? 'value must be greater than %s'.format([rules.gt]) : ''".to_string(),
          message: "".to_string(),
          value: CelRuleValue::I64(val),
        })
      } 
      int64_rules::GreaterThan::Gte(val) => {
        rules.push(CelRule {
          id: "int64.gte".to_string(),
          expression: "!has(rules.lt) && !has(rules.lte) && this < rules.gte ? 'value must be greater than or equal to %s'.format([rules.gte]) : ''".to_string(),
          message: "".to_string(),
          value: CelRuleValue::I64(val),
        })

      }
    }

    if numeric_rules.r#in.len() > 0 {
      let value = CelRuleValue::RepeatedI64(numeric_rules.r#in);

      rules.push(CelRule {
        id: "int64.in".to_string(),
        expression: "!(this in getField(rules, 'in')) ? 'value must be in list %s'.format([getField(rules, 'in')]) : ''".to_string(),
        message: "".to_string(),
        value,
      })
    }


    if numeric_rules.not_in.len() > 0 {
      let value = CelRuleValue::RepeatedI64(numeric_rules.not_in);

      rules.push(CelRule {
        id: "int64.not_in".to_string(),
        expression: "this in rules.not_in ? 'value must not be in list %s'.format([rules.not_in]) : ''".to_string(),
        message: "".to_string(),
        value,
      })
    }
  }

  Ok(rules)
}
