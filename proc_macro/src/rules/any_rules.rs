use proc_macro2::TokenStream;
use proto_types::{protovalidate::AnyRules, protovalidate_impls::ContainingRules};
use quote::quote;
use syn::Error;

use crate::validation_data::ValidationData;

pub fn get_any_rules(
  validation_data: &ValidationData,
  rules: &AnyRules,
) -> Result<TokenStream, Error> {
  let mut tokens = TokenStream::new();

  let field_span = validation_data.field_span;

  let error_prefix = format!("Error for field {}:", &validation_data.proto_name);

  let ContainingRules {
    in_list,
    not_in_list,
  } = rules.containing_rules(field_span, &error_prefix)?;

  let field_context_ident = &validation_data.field_context_ident();
  let value_ident = validation_data.value_ident();

  if !in_list.is_empty() {
    let validator_expression_tokens = quote! {
          protocheck::validators::containing::any_in_list(&#field_context_ident, #value_ident, &[ #(#in_list),* ])
    };

    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);
    tokens.extend(validator_tokens);
  }

  if !not_in_list.is_empty() {
    let validator_expression_tokens = quote! {
          protocheck::validators::containing::any_not_in_list(&#field_context_ident, #value_ident, &[ #(#not_in_list),* ])
    };

    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);
    tokens.extend(validator_tokens);
  }

  Ok(tokens)
}
