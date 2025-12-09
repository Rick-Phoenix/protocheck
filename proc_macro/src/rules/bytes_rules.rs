use crate::{bytes_rules::WellKnown, *};

pub fn get_bytes_rules(
  validation_data: &ValidationData,
  rules: &BytesRules,
) -> Result<TokenStream2, Error> {
  let mut tokens = TokenStream2::new();

  let field_span = validation_data.field_span;
  let field_name = validation_data.full_name;

  if let Some(const_rule) = rules.const_rule() {
    validation_data.get_const_validator(&mut tokens, const_rule);

    return Ok(tokens);
  }

  let lists_rules = rules
    .list_rules()
    .map_err(|e| get_field_error(field_name, field_span, &e))?;

  if !lists_rules.is_empty() {
    validation_data.get_list_validators(lists_rules, &mut tokens);
  }

  let length_rules = rules
    .length_rules()
    .map_err(|e| get_field_error(field_name, field_span, &e))?;

  if length_rules.has_rule() {
    validation_data.get_length_validator(&mut tokens, length_rules);
  }

  let substring_rules = rules.substring_rules();

  if substring_rules.has_rule() {
    validation_data.get_substring_validator(&mut tokens, substring_rules);
  }

  if let Some(ref pattern) = rules.pattern {
    Regex::new(pattern).map_err(|e| {
      get_field_error(
        field_name,
        field_span,
        &format!("invalid regex pattern: {e}"),
      )
    })?;

    validation_data.get_regex_validator(&mut tokens, pattern, true);
  }

  if let Some(well_known) = rules.well_known {
    let validator_path = match well_known {
      WellKnown::Ip(enabled) => enabled.then_some(quote! {
        ip
      }),
      WellKnown::Ipv4(enabled) => enabled.then_some(quote! {
        ipv4
      }),
      WellKnown::Ipv6(enabled) => enabled.then_some(quote! {
        ipv6
      }),
    };

    let field_context_ident = validation_data.field_context_ident();
    let value_ident = validation_data.value_ident();

    let validator_expression_tokens = quote! {
      ::protocheck::validators::bytes::#validator_path(&#field_context_ident, &#value_ident)
    };
    validation_data.get_validator_tokens(&mut tokens, &validator_expression_tokens);
  }

  Ok(tokens)
}
