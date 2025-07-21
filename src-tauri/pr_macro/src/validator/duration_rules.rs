// use crate::validator::buf::validate::duration_rules;
// use crate::validator::{buf::validate::DurationRules, CelRule, CelRuleValue};
//
// pub fn get_duration_rules(
//   dur_rules: &DurationRules,
// ) -> Result<Vec<CelRule>, Box<dyn std::error::Error>> {
//   let mut rules: Vec<CelRule> = Vec::new();
//
//   if dur_rules.r#const.is_some() {
//     let const_val = dur_rules.r#const.unwrap();
//
//     let (expression, message) = super::COMMON_RULES.get("const").unwrap();
//     rules.push(CelRule {
//       id: "duration.const".to_string(),
//       message: message.to_string(),
//       expression: expression.to_string(),
//       value: CelRuleValue::Duration(const_val),
//     });
//   }
//
//   if dur_rules.less_than.is_some() {
//     match dur_rules.less_than.unwrap() {
//       duration_rules::LessThan::Lt(lt_val) => {
//         rules.push(CelRule {
//           id: "duration.lt".to_string(),
//           message: "".to_string(),
//           expression: "!has(rules.gte) && !has(rules.gt) && this >= rules.lt ? 'value must be less than %s'.format([rules.lt]) : ''".to_string(),
//           value: CelRuleValue::Duration(lt_val)
//         });
//       }
//       duration_rules::LessThan::Lte(lte_val) => {
//         rules.push(CelRule{
//           id: "duration.lte".to_string(),
//           message: "".to_string(),
//           expression: "!has(rules.gte) && !has(rules.gt) && this > rules.lte ? 'value must be less than or equal to %s'.format([rules.lte]) : ''".to_string(),
//           value: CelRuleValue::Duration(lte_val),
//         })
//       }
//     }
//   }
//
//   if dur_rules.greater_than.is_some() {
//     match dur_rules.greater_than.unwrap() {
//       duration_rules::GreaterThan::Gt(gt_val) => {
//         rules.push(CelRule {
//           id: "duration.gt".to_string(),
//           message: "".to_string(),
//           expression: "!has(rules.lt) && !has(rules.lte) && this <= rules.gt ? 'value must be greater than %s'.format([rules.gt]) : ''".to_string(),
//           value: CelRuleValue::Duration(gt_val)
//         });
//       }
//       duration_rules::GreaterThan::Gte(gte_val) => {
//         rules.push(CelRule{
//           id: "duration.gte".to_string(),
//           message: "".to_string(),
//           expression: "!has(rules.lt) && !has(rules.lte) && this < rules.gte ? 'value must be greater than or equal to %s'.format([rules.gte]) : ''".to_string(),
//           value: CelRuleValue::Duration(gte_val),
//         })
//       }
//     }
//   }
//
//   if dur_rules.r#in.len() > 0 {
//     let in_val = dur_rules.r#in.clone();
//
//     let (expression, message) = super::COMMON_RULES.get("in").unwrap();
//     rules.push(CelRule {
//       id: "duration.in".to_string(),
//       message: message.to_string(),
//       expression: expression.to_string(),
//       value: CelRuleValue::RepeatedDuration(in_val),
//     });
//   }
//
//   if dur_rules.not_in.len() > 0 {
//     let not_in_val = dur_rules.not_in.clone();
//
//     let (expression, message) = super::COMMON_RULES.get("not_in").unwrap();
//     rules.push(CelRule {
//       id: "duration.not_in".to_string(),
//       message: message.to_string(),
//       expression: expression.to_string(),
//       value: CelRuleValue::RepeatedDuration(not_in_val),
//     });
//   }
//
//   Ok(rules)
// }
