use proc_macro2::TokenStream;
use prost_reflect::FieldDescriptor;
use proto_types::protovalidate::FieldRules;
use quote::quote;
use syn::Error;

use super::{field_rules::Type as RulesType, protovalidate::Ignore};
use crate::{
  cel_rule_template::CelRuleTemplateTarget,
  extract_validators::{field_is_boxed, field_is_message},
  rules::{cel_rules::get_cel_rules, core::get_field_rules},
  validation_data::{RepeatedValidator, ValidationData},
};

pub fn get_repeated_rules(
  validation_data: &ValidationData,
  static_defs: &mut Vec<TokenStream>,
  field_rust_enum: Option<String>,
  field_desc: &FieldDescriptor,
  field_rules: &FieldRules,
) -> Result<TokenStream, Error> {
  let mut vec_level_rules: TokenStream = TokenStream::new();
  let mut items_rules: TokenStream = TokenStream::new();
  let mut items_validation_data: Option<ValidationData> = None;

  let field_span = validation_data.field_span;

  let item_is_message = field_is_message(&field_desc.kind());

  let error_prefix = format!("Error for field {}:", validation_data.full_name);

  let mut unique_values = false;
  let mut ignore_items_validators = false;

  if !field_rules.cel.is_empty() {
    vec_level_rules.extend(get_cel_rules(
      &CelRuleTemplateTarget::Field {
        field_desc,
        is_boxed: false,
        validation_data,
      },
      &field_rules.cel,
      static_defs,
    )?);
  }

  if let Some(RulesType::Repeated(ref repeated_rules)) = field_rules.r#type {
    if repeated_rules.unique() {
      if !validation_data.field_kind.is_scalar() {
        return Err(syn::Error::new(
          field_span,
          format!(
            "{} repeated.unique only works for scalar fields",
            error_prefix
          ),
        ));
      }

      unique_values = true;
      items_validation_data = Some(validation_data.to_repeated_item(field_desc));
    }

    let value_ident = validation_data.value_ident();
    let field_context_ident = &validation_data.field_context_ident();

    let mut min_items: Option<u64> = None;
    let mut max_items: Option<u64> = None;

    if repeated_rules.min_items() > 0 {
      let rule_val = repeated_rules.min_items();
      min_items = Some(rule_val);

      let validator_expression_tokens = quote! {
        protocheck::validators::repeated::min_items(&#field_context_ident, #value_ident.len(), #rule_val)
      };
      let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

      vec_level_rules.extend(validator_tokens);
    }

    if repeated_rules.max_items() > 0 {
      let rule_val = repeated_rules.max_items();
      max_items = Some(rule_val);

      let validator_expression_tokens = quote! {
        protocheck::validators::repeated::max_items(&#field_context_ident, #value_ident.len(), #rule_val)
      };
      let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

      vec_level_rules.extend(validator_tokens);
    }

    if min_items.is_some() && max_items.is_some() && min_items.unwrap() > max_items.unwrap() {
      return Err(syn::Error::new(
        field_span,
        format!(
          "{} repeated.min_items cannot be larger than repeated.max_items",
          error_prefix
        ),
      ));
    }

    if let Some(items_rules_descriptor) = repeated_rules.items.as_ref() {
      let ignore = items_rules_descriptor.ignore();

      if matches!(ignore, Ignore::Always) {
        ignore_items_validators = true
      } else {
        let repeated_items_validation_data =
          items_validation_data.get_or_insert_with(|| validation_data.to_repeated_item(field_desc));

        if let Some(ref rules_type) = items_rules_descriptor.r#type
          && !item_is_message {
            let items_rules_tokens = get_field_rules(
              static_defs,
              field_rust_enum,
              field_desc,
              repeated_items_validation_data,
              rules_type,
            )?;

            items_rules.extend(items_rules_tokens);
          }

        if !items_rules_descriptor.cel.is_empty() {
          let cel_rules = get_cel_rules(
            &CelRuleTemplateTarget::Field {
              field_desc,
              validation_data: repeated_items_validation_data,
              is_boxed: field_is_boxed(field_desc, field_desc.parent_message()),
            },
            &items_rules_descriptor.cel,
            static_defs,
          )?;
          items_rules.extend(cel_rules);
        }
      }
    }
  }

  if item_is_message && !ignore_items_validators {
    let repeated_message_validation_data =
      items_validation_data.get_or_insert_with(|| validation_data.to_repeated_item(field_desc));

    let validator_tokens = repeated_message_validation_data.get_message_field_validator_tokens();

    items_rules.extend(validator_tokens);
  }

  let items_context_tokens =
    items_validation_data.map_or(TokenStream::new(), |data| data.field_context_tokens());

  Ok(validation_data.aggregate_vec_rules(&RepeatedValidator {
    vec_level_rules,
    items_rules,
    unique_values,
    items_context_tokens,
  }))
}
