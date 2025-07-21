use proc_macro2::{Ident, Span, TokenStream};
use proto_types::FieldData;
use proto_types::ValidatorCallTemplate;
use quote::quote;

use super::CelRule;
use super::CelRuleValue;
use crate::validator::core::get_field_rules;
use proto_types::buf::validate::RepeatedRules;

pub fn get_repeated_rules(
  field_data: FieldData,
  repeated_rules: &RepeatedRules,
) -> Result<Vec<ValidatorCallTemplate>, Box<dyn std::error::Error>> {
  let mut templates: Vec<ValidatorCallTemplate> = Vec::new();

  if repeated_rules.items.is_some() {
    let items_rules_descriptor = repeated_rules.items.clone().unwrap();

    let mut item_field_data = field_data.clone();
    item_field_data.is_repeated = false;

    let rules_for_single_item = get_field_rules(item_field_data, &items_rules_descriptor)?;

    templates.extend(rules_for_single_item);
  }

  Ok(templates)
}
