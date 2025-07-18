use crate::rules::Int32Rules;
use std::collections::HashMap;

use super::CelRule;

use crate::buf::validate::int32_rules;
use crate::buf::validate::int64_rules;
use crate::{buf::validate::Int64Rules, rules::CelRuleValue};

lazy_static! {
  static ref NUMERIC_RULES: HashMap<String, (String, String)> = {
    let mut rules_map: HashMap<String, (String, String)> = HashMap::new();

    rules_map.insert("const".to_string(), ("this != getField(rules, 'const') ? 'value must equal %s'.format([getField(rules, 'const')]) : ''".to_string(), "".to_string()));
    rules_map.insert("lt".to_string(), ("!has(rules.gte) && !has(rules.gt) && this >= rules.lt ? 'value must be less than %s'.format([rules.lt]) : ''".to_string(), "".to_string()));
    rules_map.insert("lte".to_string(), ("!has(rules.gte) && !has(rules.gt) && this > rules.lte ? 'value must be less than or equal to %s'.format([rules.lte]) : ''".to_string(), "".to_string()));
    rules_map.insert("gt".to_string(), ("!has(rules.lt) && !has(rules.lte) && this <= rules.gt ? 'value must be greater than %s'.format([rules.gt]) : ''".to_string(), "".to_string()));
    rules_map.insert("gte".to_string(), ("!has(rules.lt) && !has(rules.lte) && this < rules.gte ? 'value must be greater than or equal to %s'.format([rules.gte]) : ''".to_string(), "".to_string()));

    rules_map
  };
}

pub fn get_int64_rules(
  int64_rules: &Int64Rules,
) -> Result<Vec<CelRule>, Box<dyn std::error::Error>> {
  let mut rules: Vec<CelRule> = Vec::new();

  if int64_rules.r#const.is_some() {
    let const_val = int64_rules.r#const();
    let (expression, message) = super::COMMON_RULES.get("const").unwrap();
    rules.push(CelRule {
      id: format!("int64.const"),
      expression: expression.to_string(),
      message: message.to_string(),
      value: CelRuleValue::I64(const_val),
    })
  }

  if int64_rules.less_than.is_some() {
    match int64_rules.less_than.unwrap() {
      int64_rules::LessThan::Lt(val) => {
        let (expression, message) = NUMERIC_RULES.get("lt").unwrap();
        rules.push(CelRule {
          id: "int64.lt".to_string(),
          expression: expression.to_string(),
          message: message.to_string(),
          value: CelRuleValue::I64(val),
        })
      }
      int64_rules::LessThan::Lte(val) => {
        let (expression, message) = NUMERIC_RULES.get("lte").unwrap();
        rules.push(CelRule {
          id: "int64.lte".to_string(),
          expression: expression.to_string(),
          message: message.to_string(),
          value: CelRuleValue::I64(val),
        })
      }
    }
  }

  if int64_rules.greater_than.is_some() {
    match int64_rules.greater_than.unwrap() {
      int64_rules::GreaterThan::Gt(val) => {
        let (expression, message) = NUMERIC_RULES.get("gt").unwrap();
        rules.push(CelRule {
          id: "int64.gt".to_string(),
          expression: expression.to_string(),
          message: message.to_string(),
          value: CelRuleValue::I64(val),
        })
      }
      int64_rules::GreaterThan::Gte(val) => {
        let (expression, message) = NUMERIC_RULES.get("gte").unwrap();
        rules.push(CelRule {
          id: "int64.gte".to_string(),
          expression: expression.to_string(),
          message: message.to_string(),
          value: CelRuleValue::I64(val),
        })
      }
    }

    if int64_rules.r#in.len() > 0 {
      let value = CelRuleValue::RepeatedI64(int64_rules.r#in.clone());
      let (expression, message) = super::COMMON_RULES.get("in").unwrap();
      rules.push(CelRule {
        id: "int64.in".to_string(),
        expression: expression.to_string(),
        message: message.to_string(),
        value,
      })
    }

    if int64_rules.not_in.len() > 0 {
      let value = CelRuleValue::RepeatedI64(int64_rules.not_in.clone());
      let (expression, message) = super::COMMON_RULES.get("not_in").unwrap();
      rules.push(CelRule {
        id: "int64.not_in".to_string(),
        expression: expression.to_string(),
        message: message.to_string(),
        value,
      })
    }
  }

  Ok(rules)
}

pub fn get_int32_rules(
  int32_rules: &Int32Rules,
) -> Result<Vec<CelRule>, Box<dyn std::error::Error>> {
  let mut rules: Vec<CelRule> = Vec::new();

  if int32_rules.r#const.is_some() {
    let const_val = int32_rules.r#const();
    let (expression, message) = super::COMMON_RULES.get("const").unwrap();
    rules.push(CelRule {
      id: format!("int32.const"),
      expression: expression.to_string(),
      message: message.to_string(),
      value: CelRuleValue::I32(const_val),
    })
  }

  if int32_rules.less_than.is_some() {
    match int32_rules.less_than.unwrap() {
      int32_rules::LessThan::Lt(val) => {
        let (expression, message) = NUMERIC_RULES.get("lt").unwrap();
        rules.push(CelRule {
          id: "int32.lt".to_string(),
          expression: expression.to_string(),
          message: message.to_string(),
          value: CelRuleValue::I32(val),
        })
      }
      int32_rules::LessThan::Lte(val) => {
        let (expression, message) = NUMERIC_RULES.get("lte").unwrap();
        rules.push(CelRule {
          id: "int32.lte".to_string(),
          expression: expression.to_string(),
          message: message.to_string(),
          value: CelRuleValue::I32(val),
        })
      }
    }
  }

  if int32_rules.greater_than.is_some() {
    match int32_rules.greater_than.unwrap() {
      int32_rules::GreaterThan::Gt(val) => {
        let (expression, message) = NUMERIC_RULES.get("gt").unwrap();
        rules.push(CelRule {
          id: "int32.gt".to_string(),
          expression: expression.to_string(),
          message: message.to_string(),
          value: CelRuleValue::I32(val),
        })
      }
      int32_rules::GreaterThan::Gte(val) => {
        let (expression, message) = NUMERIC_RULES.get("gte").unwrap();
        rules.push(CelRule {
          id: "int32.gte".to_string(),
          expression: expression.to_string(),
          message: message.to_string(),
          value: CelRuleValue::I32(val),
        })
      }
    }

    if int32_rules.r#in.len() > 0 {
      let value = CelRuleValue::RepeatedI32(int32_rules.r#in.clone());
      let (expression, message) = super::COMMON_RULES.get("in").unwrap();
      rules.push(CelRule {
        id: "int32.in".to_string(),
        expression: expression.to_string(),
        message: message.to_string(),
        value,
      })
    }

    if int32_rules.not_in.len() > 0 {
      let value = CelRuleValue::RepeatedI32(int32_rules.not_in.clone());
      let (expression, message) = super::COMMON_RULES.get("not_in").unwrap();
      rules.push(CelRule {
        id: "int32.not_in".to_string(),
        expression: expression.to_string(),
        message: message.to_string(),
        value,
      })
    }
  }

  Ok(rules)
}
