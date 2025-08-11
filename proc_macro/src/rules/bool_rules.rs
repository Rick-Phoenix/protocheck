use proc_macro2::TokenStream;
use proto_types::protovalidate::BoolRules;
use syn::Error;

use crate::validation_data::ValidationData;

pub fn get_bool_rules(
  validation_data: &ValidationData,
  rules: &BoolRules,
) -> Result<TokenStream, Error> {
  let mut tokens = TokenStream::new();

  if let Some(const_rule) = rules.const_rule() {
    validation_data.get_const_validator(&mut tokens, const_rule);
  }

  Ok(tokens)
}
