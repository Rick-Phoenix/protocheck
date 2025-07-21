// use super::CelRule;
// use crate::validator::CelRuleValue;
//
// use crate::validator::buf::validate::BoolRules;
//
// pub fn get_bool_rules(bool_rules: &BoolRules) -> Result<Vec<CelRule>, Box<dyn std::error::Error>> {
//   let mut rules: Vec<CelRule> = Vec::new();
//
//   if bool_rules.r#const.is_some() {
//     let const_val = bool_rules.r#const();
//     let (expression, message) = super::COMMON_RULES.get("const").unwrap();
//     rules.push(CelRule {
//       id: "bool.const".to_string(),
//       expression: expression.to_string(),
//       message: message.to_string(),
//       value: CelRuleValue::Bool(const_val),
//     })
//   }
//
//   Ok(rules)
// }
