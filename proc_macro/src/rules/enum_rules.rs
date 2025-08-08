use std::collections::HashSet;

use proc_macro2::{Ident, Span, TokenStream};
use prost_reflect::EnumDescriptor;
use proto_types::protovalidate_impls::ContainingRules;
use quote::{quote, ToTokens};
use syn::Error;

use super::protovalidate::EnumRules;
use crate::{rules::core::hashset_to_tokens, validation_data::ValidationData};

pub fn get_enum_rules(
  field_type_ident: String,
  enum_desc: &EnumDescriptor,
  validation_data: &ValidationData,
  enum_rules: &EnumRules,
  static_defs: &mut Vec<TokenStream>,
) -> Result<TokenStream, Error> {
  let mut tokens = TokenStream::new();

  let enum_name = enum_desc.name();
  let error_prefix = format!("Error for field {}:", validation_data.full_name);

  let field_span = validation_data.field_span;
  let field_context_ident = validation_data.field_context_ident();
  let value_ident = validation_data.value_ident();

  if let Some(const_val) = enum_rules.r#const {
    let validator_tokens = validation_data.get_constant_validator(&const_val.to_token_stream());

    tokens.extend(validator_tokens);

    return Ok(tokens);
  }

  if enum_rules.defined_only() {
    let enum_ident_tokens: TokenStream = field_type_ident.parse().unwrap_or(quote! { compile_error!(format!("Failed to parse enum ident {} into tokens for enum {}", field_type_ident, enum_name)) });

    let violations_ident = &validation_data.violations_ident;
    let field_context_ident = &validation_data.field_context_ident();
    let field_context_tokens = validation_data.field_context_tokens();
    let value_ident = validation_data.value_ident();

    let validator_tokens = quote! {
      if !#enum_ident_tokens::try_from(#value_ident).is_ok() {
        #field_context_tokens
        #violations_ident.push(protocheck::validators::enums::defined_only(&#field_context_ident, #enum_name));
      }
    };

    tokens.extend(validator_tokens);
  }

  let ContainingRules {
    in_list,
    not_in_list,
  } = enum_rules.containing_rules(field_span, &error_prefix)?;

  if !in_list.is_empty() {
    let enum_values: HashSet<i32> = enum_desc.values().map(|e| e.number()).collect();
    for n in enum_rules.r#in.iter() {
      let mut invalid_numbers: Vec<i32> = Vec::new();
      if !enum_values.contains(n) {
        invalid_numbers.push(*n);
      }
      if !invalid_numbers.is_empty() {
        return Err(syn::Error::new(
          field_span,
          format!(
            "{} enum_rules.in contains values that are not in the {} enum: {:?}",
            error_prefix, enum_name, invalid_numbers
          ),
        ));
      }
    }

    let in_list_ident = Ident::new(
      &format!("__{}_IN_LIST", validation_data.static_full_name()),
      Span::call_site(),
    );
    let type_tokens = quote! { i32 };
    let hashset_tokens = hashset_to_tokens(in_list, &type_tokens);

    static_defs.push(quote! {
      static #in_list_ident: ::std::sync::LazyLock<std::collections::HashSet<#type_tokens>> = ::std::sync::LazyLock::new(||{
        #hashset_tokens
      });
    });

    let validator_expression_tokens = quote! {
      protocheck::validators::containing::in_list(&#field_context_ident, #value_ident, &#in_list_ident)
    };

    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);
    tokens.extend(validator_tokens);
  }

  if !not_in_list.is_empty() {
    let not_in_list_ident = Ident::new(
      &format!("__{}_NOT_IN_LIST", validation_data.static_full_name()),
      Span::call_site(),
    );

    let type_tokens = quote! { i32 };
    let hashset_tokens = hashset_to_tokens(not_in_list, &type_tokens);

    static_defs.push(quote! {
      static #not_in_list_ident: ::std::sync::LazyLock<::std::collections::HashSet<#type_tokens>> = ::std::sync::LazyLock::new(||{
        #hashset_tokens
      });
    });

    let validator_expression_tokens = quote! {
      protocheck::validators::containing::not_in_list(&#field_context_ident, #value_ident, &#not_in_list_ident)
    };

    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);
    tokens.extend(validator_tokens);
  }

  Ok(tokens)
}
