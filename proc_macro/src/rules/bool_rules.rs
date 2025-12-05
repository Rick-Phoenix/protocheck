use crate::*;

pub fn get_bool_rules(
  validation_data: &ValidationData,
  rules: &BoolRules,
) -> Result<TokenStream2, Error> {
  let mut tokens = TokenStream2::new();

  if let Some(const_rule) = rules.const_rule() {
    validation_data.get_const_validator(&mut tokens, const_rule);
  }

  Ok(tokens)
}
