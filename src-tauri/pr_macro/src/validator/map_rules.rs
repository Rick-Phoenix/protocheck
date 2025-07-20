use super::CelRule;
use super::CelRuleValue;
use crate::validator::buf::validate::MapRules;
use crate::validator::get_field_rules;

// pub fn get_map_rules(
//   map_rules: &MapRules,
//   field_name: &str,
//   field_tag: u32,
// ) -> Result<Vec<CelRule>, Box<dyn std::error::Error>> {
//   let mut rules: Vec<CelRule> = Vec::new();
//
//   if map_rules.min_pairs.is_some() {
//     let min_pairs_value = map_rules.min_pairs.unwrap();
//
//     rules.push(CelRule {
//       id: "map.min_pairs".to_string(),
//       message: "".to_string(),
//       expression: "uint(this.size()) < rules.min_pairs ? 'map must be at least %d entries'.format([rules.min_pairs]) : ''".to_string(),
//       value: CelRuleValue::U64(min_pairs_value),
//     });
//   }
//
//   if map_rules.max_pairs.is_some() {
//     let max_pairs_value = map_rules.max_pairs.unwrap();
//
//     rules.push(CelRule {
//       id: "map.max_pairs".to_string(),
//       message: "".to_string(),
//       expression: "uint(this.size()) > rules.max_pairs ? 'map must be at most %d entries'.format([rules.max_pairs]) : ''".to_string(),
//       value: CelRuleValue::U64(max_pairs_value),
//     });
//   }
//
//   if map_rules.keys.is_some() {
//     let keys_rules_descriptor = map_rules.keys.clone().unwrap();
//     let keys_rules: Vec<CelRule> = get_field_rules(field_name, field_tag, &keys_rules_descriptor)?
//       .into_iter()
//       .map(|mut rule| {
//         rule.id = format!("map.keys.{}", rule.id);
//         rule
//       })
//       .collect();
//     rules.extend(keys_rules);
//   }
//
//   if map_rules.values.is_some() {
//     let values_rules_descriptor = map_rules.values.clone().unwrap();
//     let values_rules: Vec<CelRule> =
//       get_field_rules(field_name, field_tag, &values_rules_descriptor)?
//         .into_iter()
//         .map(|mut rule| {
//           rule.id = format!("map.values.{}", rule.id);
//           rule
//         })
//         .collect();
//     rules.extend(values_rules);
//   }
//
//   Ok(rules)
// }
