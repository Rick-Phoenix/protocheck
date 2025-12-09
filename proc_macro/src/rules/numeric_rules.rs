use crate::*;

pub fn get_numeric_rules<HashableType, T>(
  validation_data: &ValidationData,
  rules: &T,
) -> Result<TokenStream2, Error>
where
  HashableType: Copy + ToTokens + Eq + PartialOrd + Hash + Display,
  T: NumericRules<HashableType>
    + RuleWithConst<T::Unit>
    + RulesWithComparables<T::Unit>
    + RuleWithLists<HashableType>,
  <T as NumericRules<HashableType>>::Unit: ComparableError + Copy,
{
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

pub trait NumericRules<HashableType>
where
  HashableType: ToTokens + Eq + PartialOrd + Hash,
{
  type Unit: ToTokens + PartialEq + PartialOrd + Debug + Display;

  fn finite(&self) -> bool;
}

impl NumericRules<FloatWrapper> for FloatRules {
  type Unit = f32;

  fn finite(&self) -> bool {
    self.finite()
  }
}

impl NumericRules<FloatWrapper> for DoubleRules {
  type Unit = f64;

  fn finite(&self) -> bool {
    self.finite()
  }
}

impl NumericRules<i64> for Int64Rules {
  type Unit = i64;

  fn finite(&self) -> bool {
    false
  }
}

impl NumericRules<i64> for SInt64Rules {
  type Unit = i64;

  fn finite(&self) -> bool {
    false
  }
}

impl NumericRules<i64> for SFixed64Rules {
  type Unit = i64;

  fn finite(&self) -> bool {
    false
  }
}

impl NumericRules<i32> for Int32Rules {
  type Unit = i32;

  fn finite(&self) -> bool {
    false
  }
}

impl NumericRules<i32> for SInt32Rules {
  type Unit = i32;

  fn finite(&self) -> bool {
    false
  }
}

impl NumericRules<i32> for SFixed32Rules {
  type Unit = i32;

  fn finite(&self) -> bool {
    false
  }
}

impl NumericRules<u64> for UInt64Rules {
  type Unit = u64;

  fn finite(&self) -> bool {
    false
  }
}

impl NumericRules<u64> for Fixed64Rules {
  type Unit = u64;

  fn finite(&self) -> bool {
    false
  }
}

impl NumericRules<u32> for UInt32Rules {
  type Unit = u32;

  fn finite(&self) -> bool {
    false
  }
}

impl NumericRules<u32> for Fixed32Rules {
  type Unit = u32;

  fn finite(&self) -> bool {
    false
  }
}
