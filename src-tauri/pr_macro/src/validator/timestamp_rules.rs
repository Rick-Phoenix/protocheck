// use crate::validator::buf::validate::{timestamp_rules, TimestampRules};
// use crate::validator::{CelRule, CelRuleValue};
//
// pub fn get_timestamp_rules(
//   time_rules: &TimestampRules,
// ) -> Result<Vec<CelRule>, Box<dyn std::error::Error>> {
//   let mut rules: Vec<CelRule> = Vec::new();
//
//   if time_rules.r#const.is_some() {
//     let const_val = time_rules.r#const.unwrap();
//
//     let (expression, message) = super::COMMON_RULES.get("const").unwrap();
//     rules.push(CelRule {
//       id: "timestamp.const".to_string(),
//       message: message.to_string(),
//       expression: expression.to_string(),
//       value: CelRuleValue::Timestamp(const_val),
//     });
//   }
//
//   if time_rules.less_than.is_some() {
//     match time_rules.less_than.unwrap() {
//       timestamp_rules::LessThan::Lt(lt_val) => {
//         rules.push(CelRule {
//           id: "timestamp.lt".to_string(),
//           message: "".to_string(),
//           expression: "!has(rules.gte) && !has(rules.gt) && this >= rules.lt ? 'value must be less than %s'.format([rules.lt]) : ''".to_string(),
//           value: CelRuleValue::Timestamp(lt_val)
//         });
//       }
//       timestamp_rules::LessThan::Lte(lte_val) => {
//         rules.push(CelRule{
//           id: "timestamp.lte".to_string(),
//           message: "".to_string(),
//           expression: "!has(rules.gte) && !has(rules.gt) && this > rules.lte ? 'value must be less than or equal to %s'.format([rules.lte]) : ''".to_string(),
//           value: CelRuleValue::Timestamp(lte_val),
//         })
//       }
//       timestamp_rules::LessThan::LtNow(ltnow_val) => {
//         rules.push(CelRule{
//           id: "timestamp.lt_now".to_string(),
//           message: "".to_string(),
//           expression: "(rules.lt_now && this > now) ? 'value must be less than now' : ''".to_string(),
//           value: CelRuleValue::Bool(ltnow_val),
//         })
//       }
//     }
//   }
//
//   if time_rules.greater_than.is_some() {
//     match time_rules.greater_than.unwrap() {
//       timestamp_rules::GreaterThan::Gt(gt_val) => {
//         rules.push(CelRule {
//           id: "timestamp.gt".to_string(),
//           message: "".to_string(),
//           expression: "!has(rules.lt) && !has(rules.lte) && this <= rules.gt ? 'value must be greater than %s'.format([rules.gt]) : ''".to_string(),
//           value: CelRuleValue::Timestamp(gt_val)
//         });
//       }
//       timestamp_rules::GreaterThan::Gte(gte_val) => {
//         rules.push(CelRule{
//           id: "timestamp.gte".to_string(),
//           message: "".to_string(),
//           expression: "!has(rules.lt) && !has(rules.lte) && this < rules.gte ? 'value must be greater than or equal to %s'.format([rules.gte]) : ''".to_string(),
//           value: CelRuleValue::Timestamp(gte_val),
//         })
//       }
//       timestamp_rules::GreaterThan::GtNow(gt_now_val) => {
//         rules.push(CelRule{
//           id: "timestamp.gt_now".to_string(),
//           message: "".to_string(),
//           expression: "(rules.gt_now && this < now) ? 'value must be greater than now' : ''".to_string(),
//           value: CelRuleValue::Bool(gt_now_val),
//         })
//       }
//     }
//   }
//
//   if time_rules.within.is_some() {
//     let within_val = time_rules.within.unwrap();
//
//     rules.push(CelRule {
//       id: "timestamp.within".to_string(),
//       message: "".to_string(),
//       expression: "this < now-rules.within || this > now+rules.within ? 'value must be within %s of now'.format([rules.within]) : ''".to_string(),
//       value: CelRuleValue::Duration(within_val),
//     });
//   }
//
//   Ok(rules)
// }
