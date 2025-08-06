use proc_macro2::{Span, TokenStream};
use prost_reflect::FieldDescriptor;
use proto_types::{
  protovalidate::{bytes_rules::WellKnown, BytesRules},
  protovalidate_impls::LengthRules,
};
use quote::{format_ident, quote, ToTokens};
use regex::Regex;
use syn::{Error, LitByteStr};

use crate::validation_data::ValidationData;

pub fn get_bytes_rules(
  static_defs: &mut Vec<TokenStream>,
  field_desc: &FieldDescriptor,
  validation_data: &ValidationData,
  rules: &BytesRules,
) -> Result<TokenStream, Error> {
  let mut tokens = TokenStream::new();

  let field_span = validation_data.field_span;
  let error_prefix = format!("Error for field {}:", validation_data.full_name);

  if let Some(const_val) = &rules.r#const {
    let const_val_tokens = LitByteStr::new(const_val, Span::call_site());

    let validator_tokens =
      validation_data.get_constant_validator(&const_val_tokens.to_token_stream());

    tokens.extend(validator_tokens);

    return Ok(tokens);
  }

  let (in_list, not_in_list) = rules.containing_rules(field_span, &error_prefix)?;

  let LengthRules {
    len,
    min_len,
    max_len,
  } = rules.length_rules(field_span, &error_prefix)?;

  let field_context_ident = &validation_data.field_context_ident();
  let value_ident = validation_data.value_ident();

  if let Some(ref pattern) = rules.pattern {
    Regex::new(pattern).map_err(|e| {
      Error::new(
        field_span,
        format!("{} invalid regex pattern: {}", error_prefix, e),
      )
    })?;

    let static_regex_ident = format_ident!("__{}_REGEX", field_desc.full_name());
    static_defs.push(quote! {
      static #static_regex_ident: std::sync::LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(#pattern).unwrap()
      });
    });

    let validator_expression_tokens = quote! {
      protocheck::validators::bytes::pattern(&#field_context_ident, #value_ident, #static_regex_ident )
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(in_list_tokens) = in_list {
    let validator_tokens = validation_data.get_not_in_list_validator(&in_list_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(not_in_list_tokens) = not_in_list {
    let validator_tokens = validation_data.get_not_in_list_validator(&not_in_list_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(len_value) = len {
    let validator_expression_tokens = quote! {
          protocheck::validators::bytes::len(&#field_context_ident, #value_ident, #len_value)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(min_len_value) = min_len {
    let validator_expression_tokens = quote! {
          protocheck::validators::bytes::min_len(&#field_context_ident, #value_ident, #min_len_value)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(max_len_value) = max_len {
    let validator_expression_tokens = quote! {
          protocheck::validators::bytes::max_len(&#field_context_ident, #value_ident, #max_len_value)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(ref contains_val) = rules.contains {
    let contains_val_tokens = LitByteStr::new(contains_val, Span::call_site()).to_token_stream();

    let validator_expression_tokens = quote! {
      protocheck::validators::bytes::contains(&#field_context_ident, #value_ident, #contains_val_tokens)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(ref prefix) = rules.prefix {
    let prefix_tokens = LitByteStr::new(prefix, Span::call_site()).to_token_stream();

    let validator_expression_tokens = quote! {
      protocheck::validators::bytes::prefix(&#field_context_ident, #value_ident, #prefix_tokens)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(ref suffix) = rules.suffix {
    let suffix_tokens = LitByteStr::new(suffix, Span::call_site()).to_token_stream();

    let validator_expression_tokens = quote! {
      protocheck::validators::bytes::suffix(&#field_context_ident, #value_ident, #suffix_tokens)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
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

    let validator_expression_tokens = quote! {
      protocheck::validators::bytes::#validator_path(&#field_context_ident, #value_ident)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  Ok(tokens)
}
