use std::collections::HashSet;

use proc_macro2::{Ident, Span, TokenStream};
use prost_reflect::EnumDescriptor;
use proto_types::protovalidate::ContainingRules;
use quote::quote;
use syn::Error;

use super::protovalidate::EnumRules;
use crate::{rules::core::hashset_to_tokens, validation_data::ValidationData};

pub fn get_enum_rules(
  enum_ident_str: String,
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
    let error_message = format!("has to be equal to {:?}", const_val);

    let validator_tokens = validation_data.get_const_validator("enum", const_val, &error_message);

    tokens.extend(validator_tokens);

    return Ok(tokens);
  }

  if enum_rules.defined_only() {
    let enum_ident_tokens: TokenStream = enum_ident_str.parse().unwrap_or(quote! { compile_error!(format!("Failed to parse enum ident {} into tokens for enum {}", field_type_ident, enum_name)) });

    let violations_ident = &validation_data.violations_ident;
    let field_context_ident = &validation_data.field_context_ident();
    let value_ident = validation_data.value_ident();

    let error_message = format!("must be a defined value of {}", enum_name);

    let validator_tokens = quote! {
      if !#enum_ident_tokens::try_from(#value_ident).is_ok() {
        #violations_ident.push(protocheck::validators::enums::defined_only(&#field_context_ident, #error_message));
      }
    };

    tokens.extend(validator_tokens);
  }

  let ContainingRules {
    in_list,
    not_in_list,
  } = enum_rules.containing_rules(field_span, &error_prefix)?;

  if let Some((in_list, in_list_str)) = in_list {
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
    let error_message = format!("must be one of these values: [ {} ]", in_list_str);
    let hashset_tokens = hashset_to_tokens(in_list, &type_tokens);

    static_defs.push(quote! {
      static #in_list_ident: ::std::sync::LazyLock<std::collections::HashSet<#type_tokens>> = ::std::sync::LazyLock::new(||{
        #hashset_tokens
      });
    });

    let validator_expression_tokens = quote! {
      protocheck::validators::containing::enum_in_list(&#field_context_ident, #value_ident, &#in_list_ident, #error_message)
    };

    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);
    tokens.extend(validator_tokens);
  }

  if let Some((not_in_list, not_in_list_str)) = not_in_list {
    let not_in_list_ident = Ident::new(
      &format!("__{}_NOT_IN_LIST", validation_data.static_full_name()),
      Span::call_site(),
    );

    let type_tokens = quote! { i32 };
    let error_message = format!("cannot be one of these values: [ {} ]", not_in_list_str);
    let hashset_tokens = hashset_to_tokens(not_in_list, &type_tokens);

    static_defs.push(quote! {
      static #not_in_list_ident: ::std::sync::LazyLock<::std::collections::HashSet<#type_tokens>> = ::std::sync::LazyLock::new(||{
        #hashset_tokens
      });
    });

    let validator_expression_tokens = quote! {
      protocheck::validators::containing::enum_not_in_list(&#field_context_ident, #value_ident, &#not_in_list_ident, #error_message)
    };

    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);
    tokens.extend(validator_tokens);
  }

  Ok(tokens)
}
