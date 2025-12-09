use crate::*;

pub fn get_timestamp_rules(
  validation_data: &ValidationData,
  rules: &TimestampRules,
) -> Result<TokenStream2, Error> {
  let mut tokens = TokenStream2::new();

  let field_span = validation_data.field_span;
  let field_name = validation_data.full_name;

  let field_context_ident = &validation_data.field_context_ident();
  let value_ident = validation_data.value_ident();

  if let Some(const_rule) = rules.const_rule() {
    validation_data.get_const_validator(&mut tokens, const_rule);

    return Ok(tokens);
  }

  if let Some(within_val) = rules.within {
    let error_message = format!("must be within {} from now", within_val);

    let validator_expression_tokens = quote! {
      ::protocheck::validators::timestamps::within(&#field_context_ident, #value_ident, #within_val, #error_message)
    };
    validation_data.get_validator_tokens(&mut tokens, &validator_expression_tokens);
  }

  let comparable_rules = rules
    .comparable_rules()
    .validate()
    .map_err(|e| get_field_error(field_name, field_span, e))?;

  if comparable_rules.less_than.is_some() || comparable_rules.greater_than.is_some() {
    validation_data.get_comparable_validator(&mut tokens, &comparable_rules);
  }

  if let Some(timestamp_rules::LessThan::LtNow(true)) = rules.less_than {
    if comparable_rules.less_than.is_some() {
      return Err(get_field_error(
        field_name,
        field_span,
        "lt_now and lt/lte cannot be used together",
      ));
    }

    let validator_expression_tokens = quote! {
      ::protocheck::validators::timestamps::lt_now(&#field_context_ident, #value_ident)
    };
    validation_data.get_validator_tokens(&mut tokens, &validator_expression_tokens);
  }

  if let Some(timestamp_rules::GreaterThan::GtNow(true)) = rules.greater_than {
    if comparable_rules.greater_than.is_some() {
      return Err(get_field_error(
        field_name,
        field_span,
        "gt_now and gt/gte cannot be used together",
      ));
    }

    let validator_expression_tokens = quote! {
      ::protocheck::validators::timestamps::gt_now(&#field_context_ident, #value_ident)
    };
    validation_data.get_validator_tokens(&mut tokens, &validator_expression_tokens);
  }

  Ok(tokens)
}
