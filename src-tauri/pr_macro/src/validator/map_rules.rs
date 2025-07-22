use proc_macro2::{Ident, Span, TokenStream};
use proto_types::FieldData;
use proto_types::ValidatorCallTemplate;
use quote::quote;

use super::CelRule;
use super::CelRuleValue;
use crate::validator::core::get_field_rules;
use proto_types::buf::validate::MapRules;

pub fn get_map_rules(
  field_data: FieldData,
  map_rules: &MapRules,
) -> Result<Vec<ValidatorCallTemplate>, Box<dyn std::error::Error>> {
  let mut templates: Vec<ValidatorCallTemplate> = Vec::new();

  if map_rules.keys.is_some() {
    let keys_rules_descriptor = map_rules.keys.clone().unwrap();

    let mut item_field_data = field_data.clone();
    item_field_data.for_key = true;

    let rules_for_keys = get_field_rules(item_field_data, &keys_rules_descriptor)?;

    templates.extend(rules_for_keys);
  }

  if map_rules.values.is_some() {
    let values_rules_descriptor = map_rules.values.clone().unwrap();

    let item_field_data = field_data.clone();

    let rules_for_values = get_field_rules(item_field_data, &values_rules_descriptor)?;

    templates.extend(rules_for_values);
  }

  Ok(templates)
}
