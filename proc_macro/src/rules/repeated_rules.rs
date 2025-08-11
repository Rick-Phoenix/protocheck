use proc_macro2::TokenStream;
use prost_reflect::FieldDescriptor;
use proto_types::{protovalidate::FieldRules, FieldType};
use protocheck_core::field_data::FieldKind;
use quote::quote;
use syn::Error;

#[cfg(not(feature = "cel"))]
use super::get_cel_rules;
use super::{field_rules::Type as RulesType, protovalidate::Ignore};
#[cfg(feature = "cel")]
use crate::rules::cel_rules::get_cel_rules;
use crate::{
  cel_rule_template::CelRuleTemplateTarget,
  extract_validators::field_is_message,
  rules::core::get_field_rules,
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
      if !validation_data.field_kind.inner_type().is_scalar() {
        return Err(syn::Error::new(
          field_span,
          format!(
            "{} repeated.unique only works for scalar fields",
            error_prefix
          ),
        ));
      }

      let items_validation_data =
        items_validation_data.get_or_insert_with(|| validation_data.to_repeated_item(field_desc));

      let field_context_ident = items_validation_data.field_context_ident();
      let value_ident = items_validation_data.value_ident();
      let violations_ident = items_validation_data.violations_ident;

      vec_level_rules.extend(quote! {
        let mut processed_values = ::std::collections::HashSet::new();
        let mut not_unique = false;
      });

      let func_name = match validation_data.field_kind.inner_type() {
        FieldType::Float => quote! { unique_f32 },
        FieldType::Double => quote! { unique_f64 },
        _ => quote! { unique },
      };

      items_rules.extend(quote! {
        if !not_unique {
          match ::protocheck::validators::repeated::#func_name(&#field_context_ident, #value_ident, &mut processed_values) {
            Ok(_) => {},
            Err(v) => {
              not_unique = true;
              #violations_ident.push(v);
            }
          };
        }
      });
    }

    let value_ident = validation_data.value_ident();
    let field_context_ident = &validation_data.field_context_ident();

    let mut min_items: Option<u64> = None;
    let mut max_items: Option<u64> = None;

    if repeated_rules.min_items() > 0 {
      let rule_val = repeated_rules.min_items();
      min_items = Some(rule_val);

      let plural_suffix = if rule_val != 1 { "s" } else { "" };
      let error_message = format!("must contain at least {} item{}", rule_val, plural_suffix);

      let validator_expression_tokens = quote! {
        protocheck::validators::repeated::min_items(&#field_context_ident, #value_ident.len(), #rule_val, #error_message)
      };
      validation_data.get_validator_tokens(&mut vec_level_rules, &validator_expression_tokens);
    }

    if repeated_rules.max_items() > 0 {
      let rule_val = repeated_rules.max_items();
      max_items = Some(rule_val);

      let plural_suffix = if rule_val != 1 { "s" } else { "" };
      let error_message = format!(
        "cannot contain more than {} item{}",
        rule_val, plural_suffix
      );

      let validator_expression_tokens = quote! {
        protocheck::validators::repeated::max_items(&#field_context_ident, #value_ident.len(), #rule_val, #error_message)
      };
      validation_data.get_validator_tokens(&mut vec_level_rules, &validator_expression_tokens);
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
              is_boxed: false,
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
    let validator_tokens = validation_data
      .get_message_field_validator_tokens(FieldKind::RepeatedItem(FieldType::Message));

    items_rules.extend(validator_tokens);
  }

  Ok(validation_data.aggregate_vec_rules(&RepeatedValidator {
    vec_level_rules,
    items_rules,
  }))
}
