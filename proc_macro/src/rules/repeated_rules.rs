use proc_macro2::TokenStream;
use prost_reflect::{FieldDescriptor, Kind};
use quote::quote;
use syn::Error;

use super::{
  field_rules::Type as RulesType, protovalidate::Ignore, ValidatorKind, ValidatorTemplate,
};
use crate::{
  cel_rule_template::CelRuleTemplateTarget,
  extract_validators::field_is_boxed,
  rules::{cel_rules::get_cel_rules, core::get_field_rules},
  validation_data::ValidationData,
  validator_template::FieldValidator,
};

pub fn get_repeated_rules(
  validation_data: &ValidationData,
  static_defs: &mut Vec<TokenStream>,
  field_rust_enum: Option<String>,
  field_desc: &FieldDescriptor,
  field_rules: Option<&RulesType>,
) -> Result<Option<ValidatorTemplate>, Error> {
  let mut vec_level_rules: Vec<ValidatorTemplate> = Vec::new();
  let mut items_rules: Vec<ValidatorTemplate> = Vec::new();

  let field_span = validation_data.field_span;

  let mut item_is_message = false;
  if let Kind::Message(item_desc) = field_desc.kind() {
    if !item_desc.full_name().starts_with("google.protobuf") {
      item_is_message = true;
    }
  }

  let error_prefix = format!("Error for field {}:", validation_data.full_name);

  let mut unique_values = false;
  let float_values = validation_data.field_kind.inner_type().is_float();
  let mut ignore_items_validators = false;

  if let Some(RulesType::Repeated(repeated_rules)) = field_rules {
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
    }

    let value_ident = validation_data.value_ident();
    let field_context_ident = &validation_data.field_context_ident;

    let mut min_items: Option<u64> = None;
    let mut max_items: Option<u64> = None;

    if repeated_rules.min_items() > 0 {
      let rule_val = repeated_rules.min_items();
      min_items = Some(rule_val);

      let validator_expression_tokens = quote! {
        protocheck::validators::repeated::min_items(&#field_context_ident, #value_ident, #rule_val)
      };
      let validator_tokens = validation_data.get_validator_tokens(validator_expression_tokens);

      vec_level_rules.push(ValidatorTemplate {
        kind: ValidatorKind::PureTokens(validator_tokens),
      });
    }

    if repeated_rules.max_items() > 0 {
      let rule_val = repeated_rules.max_items();
      max_items = Some(rule_val);

      let validator_expression_tokens = quote! {
        protocheck::validators::repeated::max_items(&#field_context_ident, #value_ident, #rule_val)
      };
      let validator_tokens = validation_data.get_validator_tokens(validator_expression_tokens);

      vec_level_rules.push(ValidatorTemplate {
        kind: ValidatorKind::PureTokens(validator_tokens),
      });
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
        if let Some(ref rules_type) = items_rules_descriptor.r#type {
          if !item_is_message {
            let rules_for_single_item = get_field_rules(
              static_defs,
              field_rust_enum,
              field_desc,
              validation_data,
              rules_type,
            )?;

            items_rules.extend(rules_for_single_item);
          }
        }

        if !items_rules_descriptor.cel.is_empty() {
          let cel_rules = get_cel_rules(
            &CelRuleTemplateTarget::Field {
              field_desc,
              validation_data: validation_data,
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
    let validator_tokens = validation_data.get_message_field_validator_tokens();

    let message_items_validator = ValidatorTemplate {
      kind: ValidatorKind::PureTokens(validator_tokens),
    };

    items_rules.push(message_items_validator);
  }

  if vec_level_rules.is_empty() && items_rules.is_empty() {
    Ok(None)
  } else {
    Ok(Some(ValidatorTemplate {
      kind: ValidatorKind::Field {
        validation_data: Box::new(validation_data.clone()),
        field_validator: FieldValidator::Repeated {
          vec_level_rules,
          items_rules,
          unique_values,
          float_values,
        },
      },
    }))
  }
}
