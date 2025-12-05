use crate::*;

pub fn get_any_rules(
  validation_data: &ValidationData,
  rules: &AnyRules,
  static_defs: &mut TokenStream2,
) -> Result<TokenStream2, Error> {
  let mut tokens = TokenStream2::new();

  let field_span = validation_data.field_span;
  let field_name = validation_data.full_name;

  let ContainingRules {
    in_list_rule,
    not_in_list_rule,
  } = rules
    .containing_rules(&validation_data.static_full_name())
    .map_err(|invalid_items| invalid_lists_error(field_span, field_name, &invalid_items))?;

  if let Some(in_list) = in_list_rule {
    validation_data.get_list_validator(ListRule::In, &mut tokens, in_list, static_defs);
  }

  if let Some(not_in_list) = not_in_list_rule {
    validation_data.get_list_validator(ListRule::NotIn, &mut tokens, not_in_list, static_defs);
  }

  Ok(tokens)
}
