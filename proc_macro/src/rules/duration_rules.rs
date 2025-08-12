use proc_macro2::TokenStream;
use proto_types::protovalidate::{ContainingRules, DurationRules};
use syn::Error;

use crate::{
  rules::core::{get_field_error, invalid_lists_error},
  validation_data::{ListRule, ValidationData},
};

pub fn get_duration_rules(
  validation_data: &ValidationData,
  rules: &DurationRules,
  static_defs: &mut TokenStream,
) -> Result<TokenStream, Error> {
  let mut tokens = TokenStream::new();

  let field_span = validation_data.field_span;
  let field_name = validation_data.full_name;

  if let Some(const_rule) = rules.const_rule() {
    validation_data.get_const_validator(&mut tokens, const_rule);

    return Ok(tokens);
  }

  let comparable_rules = rules
    .comparable_rules()
    .map_err(|e| get_field_error(field_name, field_span, e))?;

  if comparable_rules.less_than.is_some() || comparable_rules.greater_than.is_some() {
    validation_data.get_comparable_validator(&mut tokens, &comparable_rules);
  }

  let ContainingRules {
    in_list_rule,
    not_in_list_rule,
  } = rules
    .containing_rules(validation_data.full_name)
    .map_err(|invalid_items| invalid_lists_error(field_span, field_name, &invalid_items))?;

  if let Some(in_list) = in_list_rule {
    validation_data.get_list_validator(ListRule::In, &mut tokens, in_list, static_defs);
  }

  if let Some(not_in_list) = not_in_list_rule {
    validation_data.get_list_validator(ListRule::NotIn, &mut tokens, not_in_list, static_defs);
  }

  Ok(tokens)
}
