use std::fmt::Write;

use proc_macro2::{Ident, Span, TokenStream};
use proto_types::{
  protovalidate::{bytes_rules::WellKnown, BytesRules},
  protovalidate_impls::{ContainingRules, LengthRules},
};
use quote::{format_ident, quote, ToTokens};
use regex::Regex;
use syn::{Error, LitByteStr};

use crate::{rules::core::byte_lit_hashset_to_tokens, validation_data::ValidationData};

pub fn get_bytes_rules(
  validation_data: &ValidationData,
  rules: &BytesRules,
  static_defs: &mut Vec<TokenStream>,
) -> Result<TokenStream, Error> {
  let mut tokens = TokenStream::new();

  let field_span = validation_data.field_span;
  let error_prefix = format!("Error for field {}:", validation_data.full_name);

  if let Some(const_val) = &rules.r#const {
    let const_val_tokens = LitByteStr::new(const_val, Span::call_site());

    let error_message = format!("must be equal to {}", format_bytes(const_val));

    let validator_tokens =
      validation_data.get_constant_validator(&const_val_tokens.to_token_stream(), &error_message);

    tokens.extend(validator_tokens);

    return Ok(tokens);
  }

  let ContainingRules {
    in_list,
    not_in_list,
  } = rules.containing_rules(field_span, &error_prefix)?;

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

    let static_regex_ident = format_ident!("__{}_REGEX", validation_data.static_full_name());
    static_defs.push(quote! {
      static #static_regex_ident: ::std::sync::LazyLock<regex::Regex> = ::std::sync::LazyLock::new(|| {
        ::regex::Regex::new(#pattern).unwrap()
      });
    });

    let error_message = format!("must match the following regex: `{}`", pattern);

    let validator_expression_tokens = quote! {
      ::protocheck::validators::bytes::pattern(&#field_context_ident, &#value_ident, &#static_regex_ident, #error_message)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some((in_list, in_list_str)) = in_list {
    let in_list_ident = Ident::new(
      &format!("__{}_IN_LIST", validation_data.static_full_name()),
      Span::call_site(),
    );
    let type_tokens = quote! { ::bytes::Bytes };
    let error_message = format!("must be one of these values: [ {} ]", in_list_str);
    let hashset_tokens = byte_lit_hashset_to_tokens(in_list, &type_tokens);

    static_defs.push(quote! {
      static #in_list_ident: ::std::sync::LazyLock<std::collections::HashSet<#type_tokens>> = ::std::sync::LazyLock::new(||{
        #hashset_tokens
      });
    });

    let validator_expression_tokens = quote! {
      protocheck::validators::containing::bytes_in_list(&#field_context_ident, &#value_ident, &#in_list_ident, #error_message)
    };

    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);
    tokens.extend(validator_tokens);
  }

  if let Some((not_in_list, not_in_list_str)) = not_in_list {
    let not_in_list_ident = Ident::new(
      &format!("__{}_NOT_IN_LIST", validation_data.static_full_name()),
      Span::call_site(),
    );
    let type_tokens = quote! { ::bytes::Bytes };
    let error_message = format!("cannot be one of these values: [ {} ]", not_in_list_str);
    let hashset_tokens = byte_lit_hashset_to_tokens(not_in_list, &type_tokens);

    static_defs.push(quote! {
      static #not_in_list_ident: ::std::sync::LazyLock<std::collections::HashSet<#type_tokens>> = ::std::sync::LazyLock::new(||{
        #hashset_tokens
      });
    });

    let validator_expression_tokens = quote! {
      protocheck::validators::containing::bytes_not_in_list(&#field_context_ident, &#value_ident, &#not_in_list_ident, #error_message)
    };

    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);
    tokens.extend(validator_tokens);
  }

  if let Some(len_value) = len {
    let plural_prefix = if len_value != 1 { "s" } else { "" };
    let error_message = format!("must be exactly {} byte{} long", len_value, plural_prefix);
    let validator_expression_tokens = quote! {
          protocheck::validators::bytes::len(&#field_context_ident, &#value_ident, #len_value, #error_message)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(min_len_value) = min_len {
    let plural_prefix = if min_len_value != 1 { "s" } else { "" };
    let error_message = format!(
      "cannot be shorter than {} byte{}",
      min_len_value, plural_prefix
    );

    let validator_expression_tokens = quote! {
          protocheck::validators::bytes::min_len(&#field_context_ident, &#value_ident, #min_len_value, #error_message)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(max_len_value) = max_len {
    let plural_prefix = if max_len_value != 1 { "s" } else { "" };
    let error_message = format!(
      "cannot be shorter than {} byte{}",
      max_len_value, plural_prefix
    );

    let validator_expression_tokens = quote! {
          protocheck::validators::bytes::max_len(&#field_context_ident, &#value_ident, #max_len_value, #error_message)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(ref contains_val) = rules.contains {
    let contains_val_tokens = LitByteStr::new(contains_val, Span::call_site()).to_token_stream();
    let error_message = format!("must contain {}", format_bytes(contains_val));

    let validator_expression_tokens = quote! {
      protocheck::validators::bytes::contains(&#field_context_ident, &#value_ident, #contains_val_tokens, #error_message)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(ref prefix) = rules.prefix {
    let prefix_tokens = LitByteStr::new(prefix, Span::call_site()).to_token_stream();
    let error_message = format!("must start with {}", format_bytes(prefix));

    let validator_expression_tokens = quote! {
      protocheck::validators::bytes::prefix(&#field_context_ident, &#value_ident, #prefix_tokens, #error_message)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(ref suffix) = rules.suffix {
    let suffix_tokens = LitByteStr::new(suffix, Span::call_site()).to_token_stream();
    let error_message = format!("must end with {}", format_bytes(suffix));

    let validator_expression_tokens = quote! {
      protocheck::validators::bytes::suffix(&#field_context_ident, &#value_ident, #suffix_tokens, #error_message)
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
      protocheck::validators::bytes::#validator_path(&#field_context_ident, &#value_ident)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  Ok(tokens)
}

pub(crate) fn format_bytes(bytes: &[u8]) -> String {
  let mut s = String::with_capacity(bytes.len() * 2);
  s.push('\'');

  for &byte in bytes.iter() {
    match byte {
      b'\n' => s.push_str("\\n"),
      b'\r' => s.push_str("\\r"),
      b'\t' => s.push_str("\\t"),
      b'\\' => s.push_str("\\\\"),
      b'"' => s.push_str("\\\""),

      32..=126 => s.push(byte as char),

      _ => {
        write!(s, "\\x{:02x}", byte).unwrap();
      }
    }
  }

  s.push('\'');
  s
}
