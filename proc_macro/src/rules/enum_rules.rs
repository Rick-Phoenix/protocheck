use std::collections::HashSet;

use proc_macro2::TokenStream;
use prost_reflect::EnumDescriptor;
use proto_types::protovalidate::ContainingRules;
use quote::quote;
use syn::Error;

use super::protovalidate::EnumRules;
use crate::{
  rules::core::{get_field_error, invalid_lists_error},
  validation_data::{ListRule, ValidationData},
};

pub fn get_enum_rules(
  enum_ident_str: String,
  enum_desc: &EnumDescriptor,
  validation_data: &ValidationData,
  rules: &EnumRules,
  static_defs: &mut TokenStream,
) -> Result<TokenStream, Error> {
  let mut tokens = TokenStream::new();

  let enum_name = enum_desc.name();

  let field_span = validation_data.field_span;
  let field_name = validation_data.full_name;

  if let Some(const_rule) = rules.const_rule() {
    validation_data.get_const_validator(&mut tokens, const_rule);

    return Ok(tokens);
  }

  if rules.defined_only() {
    let enum_ident_tokens: TokenStream = enum_ident_str.parse().unwrap_or(quote! { compile_error!(format!("Failed to parse enum ident {} into tokens for enum {} in field {}", field_type_ident, enum_name, field_name)) });

    let violations_ident = &validation_data.violations_ident;
    let field_context_ident = &validation_data.field_context_ident();
    let value_ident = validation_data.value_ident();

    let error_message = format!("must be a defined value of '{}'", enum_name);

    let validator_tokens = quote! {
      if !#enum_ident_tokens::try_from(#value_ident).is_ok() {
        #violations_ident.push(::protocheck::validators::enums::defined_only(&#field_context_ident, #error_message));
      }
    };

    tokens.extend(validator_tokens);
  }

  let ContainingRules {
    in_list_rule,
    not_in_list_rule,
  } = rules
    .containing_rules(&validation_data.static_full_name())
    .map_err(|invalid_items| invalid_lists_error(field_span, field_name, &invalid_items))?;

  if let Some(in_list) = in_list_rule {
    let enum_values: HashSet<i32> = enum_desc.values().map(|e| e.number()).collect();
    let mut invalid_numbers: Vec<i32> = Vec::new();

    for n in rules.r#in.iter() {
      if !enum_values.contains(n) {
        invalid_numbers.push(*n);
      }
    }

    if !invalid_numbers.is_empty() {
      return Err(get_field_error(
        field_name,
        field_span,
        &format!(
          "enum_rules.in contains values that are not in the {} enum: {:?}",
          enum_name, invalid_numbers
        ),
      ));
    }

    validation_data.get_list_validator(ListRule::In, &mut tokens, in_list, static_defs);
  }

  if let Some(not_in_list) = not_in_list_rule {
    validation_data.get_list_validator(ListRule::NotIn, &mut tokens, not_in_list, static_defs);
  }

  Ok(tokens)
}
