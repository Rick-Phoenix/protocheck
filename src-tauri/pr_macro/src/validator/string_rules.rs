use super::{CelRule, CelRuleValue};
use proc_macro2::{Ident, Span, TokenStream};
use proto_types::buf::validate;
use proto_types::buf::validate::field_path_element::Subscript;
use proto_types::buf::validate::StringRules;
use proto_types::{FieldData, GeneratedCodeKind, ValidatorCallTemplate};
use quote::{quote, ToTokens};
use regex::Regex;

pub fn get_string_rules(
  field_data: &FieldData,
  string_rules: &proto_types::buf::validate::StringRules,
) -> Result<Vec<ValidatorCallTemplate>, Box<dyn std::error::Error>> {
  let mut templates: Vec<ValidatorCallTemplate> = Vec::new();

  let mut min_len: Option<usize> = None;
  let mut max_len: Option<usize> = None;
  let mut len: Option<usize> = None;

  if string_rules.len.is_some() {
    let len_value = string_rules.len.unwrap() as usize;
    len = Some(len_value);

    templates.push(ValidatorCallTemplate {
      validator_path: Some(quote! { macro_impl::validators::strings::len }),
      target_value_tokens: Some(len_value.into_token_stream()),
      field_data: field_data.clone(),
      kind: GeneratedCodeKind::FieldRule,
    });
  }

  if string_rules.min_len.is_some() {
    let min_len_value = string_rules.min_len.unwrap() as usize;
    min_len = Some(min_len_value);

    templates.push(ValidatorCallTemplate {
      validator_path: Some(quote! { macro_impl::validators::strings::min_len }),
      target_value_tokens: Some(min_len_value.into_token_stream()),
      field_data: field_data.clone(),
      kind: GeneratedCodeKind::FieldRule,
    });
  }

  if string_rules.max_len.is_some() {
    let max_len_value = string_rules.max_len.unwrap() as usize;
    max_len = Some(max_len_value);

    templates.push(ValidatorCallTemplate {
      validator_path: Some(quote! { macro_impl::validators::strings::max_len }),
      target_value_tokens: Some(max_len_value.into_token_stream()),
      field_data: field_data.clone(),
      kind: GeneratedCodeKind::FieldRule,
    });
  }

  if len.is_some() && (min_len.is_some() || max_len.is_some()) {
    return Err(Box::new(syn::Error::new(
      Span::call_site(),
      "string.len cannot be used with string.max_len or string.min_len",
    )));
  }

  if min_len.is_some() && max_len.is_some() {
    if min_len.unwrap() > max_len.unwrap() {
      return Err(Box::new(syn::Error::new(
        Span::call_site(),
        "string.min_len cannot be larger than string.max_len",
      )));
    }
  }

  Ok(templates)
}
