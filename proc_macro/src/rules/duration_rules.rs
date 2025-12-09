use crate::*;

pub fn get_duration_rules(
  validation_data: &ValidationData,
  rules: &DurationRules,
) -> Result<TokenStream2, Error> {
  let mut tokens = TokenStream2::new();

  let field_span = validation_data.field_span;
  let field_name = validation_data.full_name;

  if let Some(const_rule) = rules.const_rule() {
    validation_data.get_const_validator(&mut tokens, const_rule);

    return Ok(tokens);
  }

  let comparable_rules = rules
    .comparable_rules()
    .validate()
    .map_err(|e| get_field_error(field_name, field_span, e))?;

  if comparable_rules.less_than.is_some() || comparable_rules.greater_than.is_some() {
    validation_data.get_comparable_validator(&mut tokens, &comparable_rules);
  }

  let lists_rules = rules
    .list_rules()
    .map_err(|e| get_field_error(field_name, field_span, &e))?;

  if !lists_rules.is_empty() {
    validation_data.get_list_validators(lists_rules, &mut tokens);
  }

  Ok(tokens)
}
