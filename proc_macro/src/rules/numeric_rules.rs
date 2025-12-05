use crate::*;

pub fn get_numeric_rules<HashableType, T: NumericRules<HashableType>>(
  validation_data: &ValidationData,
  rules: &T,
  static_defs: &mut TokenStream2,
) -> Result<TokenStream2, Error>
where
  HashableType: Debug + Copy + ToTokens + Eq + PartialOrd + Hash,
{
  let mut tokens = TokenStream2::new();

  let field_span = validation_data.field_span;
  let field_name = validation_data.full_name;

  if let Some(const_rule) = rules.constant() {
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
    .num_containing_rules(&validation_data.static_full_name())
    .map_err(|invalid_items| invalid_lists_error(field_span, field_name, &invalid_items))?;

  if let Some(in_list) = in_list_rule {
    validation_data.get_list_validator(ListRule::In, &mut tokens, in_list, static_defs);
  };

  if let Some(not_in_list) = not_in_list_rule {
    validation_data.get_list_validator(ListRule::NotIn, &mut tokens, not_in_list, static_defs);
  }

  if rules.finite() {
    let field_context_ident = &validation_data.field_context_ident();
    let value_ident = validation_data.value_ident();
    let func_ident = format_ident!(
      "{}_is_finite",
      validation_data.field_kind.inner_type().name()
    );

    let validator_expression_tokens = quote! {
      ::protocheck::validators::floats::#func_ident(&#field_context_ident, #value_ident)
    };
    validation_data.get_validator_tokens(&mut tokens, &validator_expression_tokens);
  }

  Ok(tokens)
}
