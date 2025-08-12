use proc_macro2::TokenStream;
use proto_types::protovalidate::{string_rules::WellKnown, ContainingRules};
use quote::quote;
use regex::Regex;
use syn::Error;

use super::protovalidate::StringRules;
use crate::{
  rules::core::{get_field_error, invalid_lists_error},
  validation_data::{ListRule, ValidationData},
};

pub fn get_string_rules(
  static_defs: &mut TokenStream,
  validation_data: &ValidationData,
  rules: &StringRules,
) -> Result<TokenStream, Error> {
  let mut tokens = TokenStream::new();

  let field_span = validation_data.field_span;
  let field_name = validation_data.full_name;

  if let Some(const_rule) = rules.const_rule() {
    validation_data.get_const_validator(&mut tokens, const_rule);

    return Ok(tokens);
  }

  let ContainingRules {
    in_list_rule,
    not_in_list_rule,
  } = rules
    .containing_rules(&validation_data.static_full_name())
    .map_err(|invalid_items| invalid_lists_error(field_span, field_name, &invalid_items))?;

  let length_rules = rules
    .length_rules()
    .map_err(|e| get_field_error(field_name, field_span, &e))?;

  if length_rules.has_rule() {
    validation_data.get_length_validator(&mut tokens, length_rules);
  }

  let bytes_length_rules = rules
    .bytes_length_rules()
    .map_err(|e| get_field_error(field_name, field_span, &e))?;

  if bytes_length_rules.has_rule() {
    validation_data.get_length_validator(&mut tokens, bytes_length_rules);
  }

  let substring_rules = rules.substring_rules();

  if substring_rules.has_rule() {
    validation_data.get_substring_validator(&mut tokens, substring_rules);
  }

  if let Some(ref pattern) = rules.pattern {
    Regex::new(pattern).map_err(|e| {
      get_field_error(
        field_name,
        field_span,
        &format!("invalid regex pattern: {}", e),
      )
    })?;

    validation_data.get_regex_validator(&mut tokens, static_defs, pattern);
  }

  if let Some(in_list) = in_list_rule {
    validation_data.get_list_validator(ListRule::In, &mut tokens, in_list, static_defs);
  }

  if let Some(not_in_list) = not_in_list_rule {
    validation_data.get_list_validator(ListRule::NotIn, &mut tokens, not_in_list, static_defs);
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
      let field_context_ident = validation_data.field_context_ident();
      let value_ident = validation_data.value_ident();

      let strict_arg = is_strict.map(|bool| quote! { , #bool });

      let validator_expression_tokens = quote! {
        protocheck::validators::string::#validator_func(&#field_context_ident, &#value_ident #strict_arg)
      };

      validation_data.get_validator_tokens(&mut tokens, &validator_expression_tokens);
    }
  }

  Ok(tokens)
}
