use std::{fmt::Debug, hash::Hash};

use proc_macro2::TokenStream;
use proto_types::protovalidate::{ContainingRules, NumericRules};
use quote::{quote, ToTokens};
use syn::Error;

use crate::{
  rules::core::invalid_lists_error,
  validation_data::{ListRule, ValidationData},
};

pub fn get_numeric_rules<HashableType, T: NumericRules<HashableType>>(
  validation_data: &ValidationData,
  rules: &T,
  static_defs: &mut TokenStream,
) -> Result<TokenStream, Error>
where
  HashableType: Debug + Copy + ToTokens + Eq + PartialOrd + Hash,
{
  let mut tokens = TokenStream::new();

  let field_span = validation_data.field_span;
  let error_prefix = format!("Error for field {}:", &validation_data.proto_name);

  if let Some(const_rule) = rules.constant() {
    validation_data.get_const_validator(&mut tokens, const_rule);

    return Ok(tokens);
  }

  let comparable_rules = rules.comparable_rules(field_span, &error_prefix)?;

  if comparable_rules.less_than.is_some() || comparable_rules.greater_than.is_some() {
    validation_data.get_comparable_validator(&mut tokens, &comparable_rules);
  }

  let ContainingRules {
    in_list_rule,
    not_in_list_rule,
  } = rules
    .num_containing_rules(validation_data.full_name)
    .map_err(|invalid_items| invalid_lists_error(field_span, &error_prefix, &invalid_items))?;

  if let Some(in_list) = in_list_rule {
    validation_data.get_list_validator(ListRule::In, &mut tokens, in_list, static_defs);
  };

  if let Some(not_in_list) = not_in_list_rule {
    validation_data.get_list_validator(ListRule::NotIn, &mut tokens, not_in_list, static_defs);
  }

  if let Some(func_tokens) = rules.finite() {
    let field_context_ident = &validation_data.field_context_ident();
    let value_ident = validation_data.value_ident();

    let validator_expression_tokens = quote! {
      #func_tokens(&#field_context_ident, #value_ident)
    };
    validation_data.get_validator_tokens(&mut tokens, &validator_expression_tokens);
  }

  Ok(tokens)
}
