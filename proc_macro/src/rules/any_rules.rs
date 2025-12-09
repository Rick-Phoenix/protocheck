use crate::*;

pub fn get_any_rules(
  validation_data: &ValidationData,
  rules: &AnyRules,
) -> Result<TokenStream2, Error> {
  let mut tokens = TokenStream2::new();

  let field_span = validation_data.field_span;
  let field_name = validation_data.full_name;

  let lists_rules = rules
    .list_rules()
    .map_err(|e| get_field_error(field_name, field_span, &e))?;

  if !lists_rules.is_empty() {
    validation_data.get_list_validators(lists_rules, &mut tokens);
  }

  Ok(tokens)
}
