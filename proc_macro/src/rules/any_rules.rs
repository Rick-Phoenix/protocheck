use proc_macro2::{Ident, Span, TokenStream};
use proto_types::protovalidate::AnyRules;
use quote::quote;
use syn::Error;

use crate::{
  rules::{core::hashset_to_tokens, protovalidate::ContainingRules},
  validation_data::ValidationData,
};

pub fn get_any_rules(
  validation_data: &ValidationData,
  rules: &AnyRules,
  static_defs: &mut Vec<TokenStream>,
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

  if let Some((in_list, in_list_str)) = in_list {
    let in_list_ident = Ident::new(
      &format!("__{}_IN_LIST", validation_data.static_full_name()),
      Span::call_site(),
    );

    let type_tokens = quote! { &'static str };
    let error_message = format!(
      "the type url must be one of these values: [ {} ]",
      in_list_str
    );
    let hashset_tokens = hashset_to_tokens(in_list, &type_tokens);

    static_defs.push(quote! {
      static #in_list_ident: ::std::sync::LazyLock<std::collections::HashSet<&'static str>> = ::std::sync::LazyLock::new(||{
        #hashset_tokens
      });
    });

    let validator_expression_tokens = quote! {
      protocheck::validators::containing::any_in_list(&#field_context_ident, &#value_ident, &#in_list_ident, #error_message)
    };

    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);
    tokens.extend(validator_tokens);
  }

  if let Some((not_in_list, not_in_list_str)) = not_in_list {
    let not_in_list_ident = Ident::new(
      &format!("__{}_NOT_IN_LIST", validation_data.static_full_name()),
      Span::call_site(),
    );

    let type_tokens = quote! { &'static str };
    let error_message = format!(
      "the type url cannot be one of these values: [ {} ]",
      not_in_list_str
    );
    let hashset_tokens = hashset_to_tokens(not_in_list, &type_tokens);

    static_defs.push(quote! {
      static #not_in_list_ident: ::std::sync::LazyLock<std::collections::HashSet<&'static str>> = ::std::sync::LazyLock::new(||{
        #hashset_tokens
      });
    });

    let validator_expression_tokens = quote! {
      protocheck::validators::containing::any_not_in_list(&#field_context_ident, &#value_ident, &#not_in_list_ident, #error_message)
    };

    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);
    tokens.extend(validator_tokens);
  }

  Ok(tokens)
}
