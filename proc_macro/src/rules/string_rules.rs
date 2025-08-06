use proc_macro2::TokenStream;
use prost_reflect::FieldDescriptor;
use proto_types::{
  protovalidate::string_rules::WellKnown,
  protovalidate_impls::{ContainingRules, LengthRules},
};
use quote::{format_ident, quote, ToTokens};
use regex::Regex;
use syn::Error;

use super::protovalidate::StringRules;
use crate::validation_data::ValidationData;

pub fn get_string_rules(
  static_defs: &mut Vec<TokenStream>,
  field_desc: &FieldDescriptor,
  validation_data: &ValidationData,
  rules: &StringRules,
) -> Result<TokenStream, Error> {
  let mut tokens = TokenStream::new();

  let field_span = validation_data.field_span;
  let error_prefix = format!("Error for field {}:", validation_data.full_name);

  if let Some(const_val) = &rules.r#const {
    let validator_tokens = validation_data.get_constant_validator(&const_val.to_token_stream());

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

  let LengthRules {
    len: len_bytes,
    min_len: min_bytes,
    max_len: max_bytes,
  } = rules.bytes_length_rules(field_span, &error_prefix)?;

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
      static #static_regex_ident: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(#pattern).unwrap()
      });
    });

    let validator_expression_tokens = quote! {
      protocheck::validators::strings::pattern(&#field_context_ident, #value_ident, &#static_regex_ident)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if !in_list.is_empty() {
    let validator_expression_tokens = quote! {
      protocheck::validators::containing::string_in_list(&#field_context_ident, #value_ident, &[ #(#in_list),* ])
    };

    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);
    tokens.extend(validator_tokens);
  }

  if !not_in_list.is_empty() {
    let validator_expression_tokens = quote! {
      protocheck::validators::containing::string_not_in_list(&#field_context_ident, #value_ident, &[ #(#not_in_list),*])
    };

    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);
    tokens.extend(validator_tokens);
  }

  if let Some(len_value) = len {
    let validator_expression_tokens = quote! {
      protocheck::validators::strings::len(&#field_context_ident, #value_ident, #len_value)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(min_len_value) = min_len {
    let validator_expression_tokens = quote! {
      protocheck::validators::strings::min_len(&#field_context_ident, #value_ident, #min_len_value)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(max_len_value) = max_len {
    let validator_expression_tokens = quote! {
      protocheck::validators::strings::max_len(&#field_context_ident, #value_ident, #max_len_value)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(len_bytes_val) = len_bytes {
    let validator_expression_tokens = quote! {
      protocheck::validators::strings::len_bytes(&#field_context_ident, #value_ident, #len_bytes_val)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(min_bytes_val) = min_bytes {
    let validator_expression_tokens = quote! {
      protocheck::validators::strings::min_bytes(&#field_context_ident, #value_ident, #min_bytes_val)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(max_bytes_val) = max_bytes {
    let validator_expression_tokens = quote! {
      protocheck::validators::strings::max_bytes(&#field_context_ident, #value_ident, #max_bytes_val)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(ref contains_val) = rules.contains {
    let validator_expression_tokens = quote! {
      protocheck::validators::strings::contains(&#field_context_ident, #value_ident, #contains_val)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(ref not_contains_val) = rules.not_contains {
    let validator_expression_tokens = quote! {
      protocheck::validators::strings::not_contains(&#field_context_ident, #value_ident, #not_contains_val)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(ref prefix_val) = rules.prefix {
    let validator_expression_tokens = quote! {
      protocheck::validators::strings::prefix(&#field_context_ident, #value_ident, #prefix_val)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(ref suffix_val) = rules.suffix {
    let validator_expression_tokens = quote! {
      protocheck::validators::strings::suffix(&#field_context_ident, #value_ident, #suffix_val)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  if let Some(well_known_kind) = rules.well_known {
    let mut is_strict: Option<bool> = None;
    let validator_path = match well_known_kind {
      WellKnown::Email(enabled) => enabled.then_some(quote! { email }),
      WellKnown::Hostname(enabled) => enabled.then_some(quote! { hostname }),
      WellKnown::Ip(enabled) => enabled.then_some(quote! { ip }),
      WellKnown::Ipv4(enabled) => enabled.then_some(quote! { ipv4 }),
      WellKnown::Ipv6(enabled) => enabled.then_some(quote! { ipv6 }),
      WellKnown::Uri(enabled) => enabled.then_some(quote! { uri }),
      WellKnown::UriRef(enabled) => enabled.then_some(quote! { uri_ref }),
      WellKnown::Address(enabled) => enabled.then_some(quote! { address }),
      WellKnown::Uuid(enabled) => enabled.then_some(quote! { uuid }),
      WellKnown::Tuuid(enabled) => enabled.then_some(quote! { tuuid }),
      WellKnown::IpWithPrefixlen(enabled) => enabled.then_some(quote! { ip_with_prefix_len }),
      WellKnown::Ipv4WithPrefixlen(enabled) => enabled.then_some(quote! { ipv4_with_prefix_len }),
      WellKnown::Ipv6WithPrefixlen(enabled) => enabled.then_some(quote! { ipv6_with_prefix_len }),
      WellKnown::IpPrefix(enabled) => enabled.then_some(quote! { ip_prefix }),
      WellKnown::Ipv4Prefix(enabled) => enabled.then_some(quote! { ipv4_prefix }),
      WellKnown::Ipv6Prefix(enabled) => enabled.then_some(quote! { ip6_prefix }),
      WellKnown::HostAndPort(enabled) => enabled.then_some(quote! { host_and_port }),
      WellKnown::WellKnownRegex(well_known_regex) => {
        is_strict = Some(rules.strict());

        match well_known_regex {
          1 => Some(quote! { header_name }),
          2 => Some(quote! { header_value }),
          _ => None,
        }
      }
    };

    if let Some(validator_func) = validator_path {
      let strict_arg = is_strict.map(|bool| quote! { , #bool });
      let validator_expression_tokens = quote! {
        protocheck::validators::strings::#validator_func(&#field_context_ident, #value_ident #strict_arg)
      };
      let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

      tokens.extend(validator_tokens);
    }
  }

  Ok(tokens)
}
