use proc_macro2::TokenStream;
use proto_types::protovalidate::BoolRules;
use syn::Error;

use crate::validation_data::ValidationData;

pub fn get_bool_rules(
  validation_data: &ValidationData,
  rules: &BoolRules,
) -> Result<TokenStream, Error> {
  let mut tokens = TokenStream::new();

  if let Some(const_val) = rules.r#const {
    let error_message = format!("has to be equal to {:?}", const_val);

    let validator_tokens = validation_data.get_const_validator("bool", const_val, &error_message);
    tokens.extend(validator_tokens);
  }

  Ok(tokens)
}
