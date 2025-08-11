use proc_macro2::TokenStream;
use prost_reflect::FieldDescriptor;
use proto_types::protovalidate::{string_rules::WellKnown, ContainingRules, LengthRules};
use quote::{format_ident, quote};
use regex::Regex;
use syn::Error;

use super::protovalidate::StringRules;
use crate::{
  rules::core::invalid_lists_error,
  validation_data::{ListRule, ValidationData},
};

pub fn get_string_rules(
  static_defs: &mut TokenStream,
  field_desc: &FieldDescriptor,
  validation_data: &ValidationData,
  rules: &StringRules,
) -> Result<TokenStream, Error> {
  let mut tokens = TokenStream::new();

  let field_span = validation_data.field_span;
  let error_prefix = format!("Error for field {}:", validation_data.full_name);

  if let Some(const_rule) = rules.const_rule() {
    validation_data.get_const_validator(&mut tokens, const_rule);

    return Ok(tokens);
  }

  let ContainingRules {
    in_list_rule,
    not_in_list_rule,
  } = rules
    .containing_rules(validation_data.full_name)
    .map_err(|invalid_items| invalid_lists_error(field_span, &error_prefix, &invalid_items))?;

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

    let static_regex_ident = format_ident!(
      "__{}_REGEX",
      field_desc.full_name().replace(".", "_").to_uppercase()
    );

    static_defs.extend(quote! {
      static #static_regex_ident: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(#pattern).unwrap()
      });
    });

    let error_message = format!("must match the following regex: `{}`", pattern);

    let validator_expression_tokens = quote! {
      protocheck::validators::strings::pattern(&#field_context_ident, &#value_ident, &#static_regex_ident, #error_message)
    };

    validation_data.get_validator_tokens(&mut tokens, &validator_expression_tokens);
  }

  if let Some(in_list) = in_list_rule {
    validation_data.get_list_validator(ListRule::In, &mut tokens, in_list, static_defs);
  }

  if let Some(not_in_list) = not_in_list_rule {
    validation_data.get_list_validator(ListRule::NotIn, &mut tokens, not_in_list, static_defs);
  }

  if let Some(len_value) = len {
    let plural_prefix = if len_value != 1 { "s" } else { "" };

    let error_message = format!(
      "must be exactly {} character{} long",
      len_value, plural_prefix
    );

    let validator_expression_tokens = quote! {
      protocheck::validators::strings::len(&#field_context_ident, &#value_ident, #len_value, #error_message)
    };

    validation_data.get_validator_tokens(&mut tokens, &validator_expression_tokens);
  }

  if let Some(min_len_value) = min_len {
    let plural_prefix = if min_len_value != 1 { "s" } else { "" };

    let error_message = format!(
      "cannot be shorter than {} character{}",
      min_len_value, plural_prefix
    );

    let validator_expression_tokens = quote! {
      protocheck::validators::strings::min_len(&#field_context_ident, &#value_ident, #min_len_value, #error_message)
    };

    validation_data.get_validator_tokens(&mut tokens, &validator_expression_tokens);
  }

  if let Some(max_len_value) = max_len {
    let plural_prefix = if max_len_value != 1 { "s" } else { "" };

    let error_message = format!(
      "cannot be longer than {} character{}",
      max_len_value, plural_prefix
    );

    let validator_expression_tokens = quote! {
      protocheck::validators::strings::max_len(&#field_context_ident, &#value_ident, #max_len_value, #error_message)
    };

    validation_data.get_validator_tokens(&mut tokens, &validator_expression_tokens);
  }

  if let Some(len_bytes_val) = len_bytes {
    let plural_prefix = if len_bytes_val != 1 { "s" } else { "" };

    let error_message = format!(
      "must be exactly {} byte{} long",
      len_bytes_val, plural_prefix
    );

    let validator_expression_tokens = quote! {
      protocheck::validators::strings::len_bytes(&#field_context_ident, &#value_ident, #len_bytes_val, #error_message)
    };

    validation_data.get_validator_tokens(&mut tokens, &validator_expression_tokens);
  }

  if let Some(min_bytes_val) = min_bytes {
    let plural_prefix = if min_bytes_val != 1 { "s" } else { "" };

    let error_message = format!(
      "cannot be shorter than {} byte{}",
      min_bytes_val, plural_prefix
    );

    let validator_expression_tokens = quote! {
      protocheck::validators::strings::min_bytes(&#field_context_ident, &#value_ident, #min_bytes_val, #error_message)
    };
    validation_data.get_validator_tokens(&mut tokens, &validator_expression_tokens);
  }

  if let Some(max_bytes_val) = max_bytes {
    let plural_prefix = if max_bytes_val != 1 { "s" } else { "" };

    let error_message = format!(
      "cannot be longer than {} byte{}",
      max_bytes_val, plural_prefix
    );

    let validator_expression_tokens = quote! {
      protocheck::validators::strings::max_bytes(&#field_context_ident, &#value_ident, #max_bytes_val, #error_message)
    };

    validation_data.get_validator_tokens(&mut tokens, &validator_expression_tokens);
  }

  if let Some(ref contains_val) = rules.contains {
    let error_message = format!("must contain the following substring: '{}'", contains_val);

    let validator_expression_tokens = quote! {
      protocheck::validators::strings::contains(&#field_context_ident, &#value_ident, #contains_val, #error_message)
    };

    validation_data.get_validator_tokens(&mut tokens, &validator_expression_tokens);
  }

  if let Some(ref not_contains_val) = rules.not_contains {
    let error_message = format!(
      "cannot contain the following substring: '{}'",
      not_contains_val
    );

    let validator_expression_tokens = quote! {
      protocheck::validators::strings::not_contains(&#field_context_ident, &#value_ident, #not_contains_val, #error_message)
    };

    validation_data.get_validator_tokens(&mut tokens, &validator_expression_tokens);
  }

  if let Some(ref prefix_val) = rules.prefix {
    let error_message = format!("must start with '{}'", prefix_val);

    let validator_expression_tokens = quote! {
      protocheck::validators::strings::prefix(&#field_context_ident, &#value_ident, #prefix_val, #error_message)
    };

    validation_data.get_validator_tokens(&mut tokens, &validator_expression_tokens);
  }

  if let Some(ref suffix_val) = rules.suffix {
    let error_message = format!("must end with '{}'", suffix_val);

    let validator_expression_tokens = quote! {
      protocheck::validators::strings::suffix(&#field_context_ident, &#value_ident, #suffix_val, #error_message)
    };

    validation_data.get_validator_tokens(&mut tokens, &validator_expression_tokens);
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
      WellKnown::IpWithPrefixlen(enabled) => enabled.then_some(quote! { ip_with_prefixlen }),
      WellKnown::Ipv4WithPrefixlen(enabled) => enabled.then_some(quote! { ipv4_with_prefixlen }),
      WellKnown::Ipv6WithPrefixlen(enabled) => enabled.then_some(quote! { ipv6_with_prefixlen }),
      WellKnown::IpPrefix(enabled) => enabled.then_some(quote! { ip_prefix }),
      WellKnown::Ipv4Prefix(enabled) => enabled.then_some(quote! { ipv4_prefix }),
      WellKnown::Ipv6Prefix(enabled) => enabled.then_some(quote! { ipv6_prefix }),
      WellKnown::HostAndPort(enabled) => enabled.then_some(quote! { host_and_port }),
      WellKnown::WellKnownRegex(well_known_regex) => {
        if let Some(val) = rules.strict {
          is_strict = Some(val)
        } else {
          is_strict = Some(true)
        };

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
        protocheck::validators::strings::#validator_func(&#field_context_ident, &#value_ident #strict_arg)
      };

      validation_data.get_validator_tokens(&mut tokens, &validator_expression_tokens);
    }
  }

  Ok(tokens)
}
