use proc_macro2::{Ident, Span, TokenStream};
use proto_types::buf::validate::Ignore;
use proto_types::google::protobuf::field_descriptor_proto;
use proto_types::FieldData;
use proto_types::GeneratedCodeKind;
use proto_types::ValidatorCallTemplate;
use quote::quote;
use quote::ToTokens;

use super::CelRule;
use super::CelRuleValue;
use crate::validator::cel_rules::get_cel_rules;
use crate::validator::core::get_field_rules;
use proto_types::buf::validate::RepeatedRules;

pub fn get_repeated_rules(
  field_data: &FieldData,
  repeated_rules: &RepeatedRules,
) -> Result<ValidatorCallTemplate, Box<dyn std::error::Error>> {
  let mut templates: Vec<ValidatorCallTemplate> = Vec::new();
  let mut items_templates: Vec<ValidatorCallTemplate> = Vec::new();

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

      if items_rules_descriptor.cel.len() > 0 {
        let cel_rules = get_cel_rules(&items_field_data, items_rules_descriptor.cel, false)?;
        items_templates.extend(cel_rules);
      }
    }
  }

  if repeated_rules.min_items() > 0 {
    let rule_val = repeated_rules.min_items();
    templates.push(ValidatorCallTemplate {
      validator_path: Some(quote! { macro_impl::validators::repeated::min_items }),
      target_value_tokens: Some(rule_val.to_token_stream()),
      field_data: field_data.clone(),
      kind: GeneratedCodeKind::FieldRule,
    });
  }

  let mut unique_values = false;

  if repeated_rules.unique() {
    if matches!(field_data.proto_type, field_descriptor_proto::Type::Message) {
      return Err(Box::new(syn::Error::new(
        Span::call_site(),
        "repeated.unique only works for scalar fields",
      )));
    }

    unique_values = true;
  }

  Ok(ValidatorCallTemplate {
    validator_path: None,
    target_value_tokens: None,
    field_data: field_data.clone(),
    kind: GeneratedCodeKind::RepeatedValidationLoop {
      vec_level_rules: templates,
      items_rules: items_templates,
      unique_values,
    },
  })
}
