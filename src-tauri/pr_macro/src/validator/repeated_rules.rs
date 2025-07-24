use proc_macro2::{Ident, Span, TokenStream};
use proto_types::buf::validate::Ignore;
use proto_types::FieldData;
use proto_types::ValidatorCallTemplate;
use quote::quote;

use super::CelRule;
use super::CelRuleValue;
use crate::validator::cel_rules::get_cel_rules;
use crate::validator::core::get_field_rules;
use proto_types::buf::validate::RepeatedRules;

pub fn get_repeated_rules(
  field_data: &FieldData,
  repeated_rules: &RepeatedRules,
) -> Result<Vec<ValidatorCallTemplate>, Box<dyn std::error::Error>> {
  let mut templates: Vec<ValidatorCallTemplate> = Vec::new();

  if repeated_rules.items.is_some() {
    let items_rules_descriptor = repeated_rules.items.clone().unwrap();
    let ignore = items_rules_descriptor.ignore();
    if !matches!(ignore, Ignore::Always) {
      let mut items_field_data = field_data.clone();
      items_field_data.ignore = ignore;

      let rules_for_single_item = get_field_rules(&items_field_data, &items_rules_descriptor)?;

      templates.extend(rules_for_single_item);

      if items_rules_descriptor.cel.len() > 0 {
        let cel_rules = get_cel_rules(&items_field_data, items_rules_descriptor.cel, false)?;
        templates.extend(cel_rules);
      }
    }
  }

  Ok(templates)
}
