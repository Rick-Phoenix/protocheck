use quote::{quote, ToTokens};

use super::{
  protovalidate::{Ignore, RepeatedRules},
  FieldData, GeneratedCodeKind, ProtoType, ValidatorCallTemplate,
};
use crate::{
  rules::{cel_rules::get_cel_rules, core::get_field_rules},
  Span2,
};

pub fn get_repeated_rules(
  field_data: &FieldData,
  repeated_rules: &RepeatedRules,
) -> Result<ValidatorCallTemplate, Box<dyn std::error::Error>> {
  let mut templates: Vec<ValidatorCallTemplate> = Vec::new();
  let mut items_templates: Vec<ValidatorCallTemplate> = Vec::new();

  let mut min_items: Option<u64> = None;
  let mut max_items: Option<u64> = None;

  if repeated_rules.min_items() > 0 {
    let rule_val = repeated_rules.min_items();
    min_items = Some(rule_val);
    templates.push(ValidatorCallTemplate {
      validator_path: Some(quote! { protocheck::validators::repeated::min_items }),
      target_value_tokens: Some(rule_val.to_token_stream()),
      field_data: field_data.clone(),
      kind: GeneratedCodeKind::FieldRule,
    });
  }

  if repeated_rules.max_items() > 0 {
    let rule_val = repeated_rules.max_items();
    max_items = Some(rule_val);
    templates.push(ValidatorCallTemplate {
      validator_path: Some(quote! { protocheck::validators::repeated::max_items }),
      target_value_tokens: Some(rule_val.to_token_stream()),
      field_data: field_data.clone(),
      kind: GeneratedCodeKind::FieldRule,
    });
  }

  if min_items.is_some() && max_items.is_some() && min_items.unwrap() > max_items.unwrap() {
    return Err(Box::new(syn::Error::new(
      Span2::call_site(),
      "repeated.min_items cannot be larger than repeated.max_items",
    )));
  }

  let mut unique_values = false;
  let float_values = matches!(field_data.proto_type, ProtoType::Float)
    || matches!(field_data.proto_type, ProtoType::Double);

  if repeated_rules.unique() {
    if matches!(field_data.proto_type, ProtoType::Message) {
      return Err(Box::new(syn::Error::new(
        Span2::call_site(),
        "repeated.unique only works for scalar fields",
      )));
    }

    unique_values = true;
  }

  if repeated_rules.items.is_some() {
    let items_rules_descriptor = repeated_rules.items.clone().unwrap();
    let ignore = items_rules_descriptor.ignore();
    if !matches!(ignore, Ignore::Always) {
      let mut items_field_data = field_data.clone();
      items_field_data.ignore = ignore;
      items_field_data.is_repeated = false;
      items_field_data.is_repeated_item = true;

      let rules_for_single_item = get_field_rules(&items_field_data, &items_rules_descriptor)?;

      items_templates.extend(rules_for_single_item);

      if !items_rules_descriptor.cel.is_empty() {
        let cel_rules = get_cel_rules(&items_field_data, items_rules_descriptor.cel, false)?;
        items_templates.extend(cel_rules);
      }
    }
  }

  Ok(ValidatorCallTemplate {
    validator_path: None,
    target_value_tokens: None,
    field_data: field_data.clone(),
    kind: GeneratedCodeKind::RepeatedValidationLoop {
      vec_level_rules: templates,
      items_rules: items_templates,
      unique_values,
      float_values,
    },
  })
}
