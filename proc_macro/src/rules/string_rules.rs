use quote::{quote, ToTokens};
use syn::{Error, Ident};

use super::{protovalidate::StringRules, FieldData, GeneratedCodeKind, ValidatorCallTemplate};
use crate::Span2;

pub fn get_string_rules(
  field_span: Span2,
  field_data: &FieldData,
  string_rules: &StringRules,
) -> Result<Vec<ValidatorCallTemplate>, Error> {
  let mut templates: Vec<ValidatorCallTemplate> = Vec::new();

  let mut min_len: Option<usize> = None;
  let mut max_len: Option<usize> = None;
  let mut len: Option<usize> = None;

  if string_rules.len.is_some() {
    let len_value = string_rules.len.unwrap() as usize;
    len = Some(len_value);

    templates.push(ValidatorCallTemplate {
      validator_path: Some(quote! { protocheck::validators::strings::len }),
      target_value_tokens: Some(len_value.into_token_stream()),
      field_data: field_data.clone(),
      kind: GeneratedCodeKind::FieldRule,
    });
  }

  if string_rules.min_len.is_some() {
    let min_len_value = string_rules.min_len.unwrap() as usize;
    min_len = Some(min_len_value);

    templates.push(ValidatorCallTemplate {
      validator_path: Some(quote! { protocheck::validators::strings::min_len }),
      target_value_tokens: Some(min_len_value.into_token_stream()),
      field_data: field_data.clone(),
      kind: GeneratedCodeKind::FieldRule,
    });
  }

  if string_rules.max_len.is_some() {
    let max_len_value = string_rules.max_len.unwrap() as usize;
    max_len = Some(max_len_value);

    templates.push(ValidatorCallTemplate {
      validator_path: Some(quote! { protocheck::validators::strings::max_len }),
      target_value_tokens: Some(max_len_value.into_token_stream()),
      field_data: field_data.clone(),
      kind: GeneratedCodeKind::FieldRule,
    });
  }

  if len.is_some() && (min_len.is_some() || max_len.is_some()) {
    return Err(syn::Error::new(
      field_span,
      "string.len cannot be used with string.max_len or string.min_len",
    ));
  }

  if min_len.is_some() && max_len.is_some() && min_len.unwrap() > max_len.unwrap() {
    return Err(syn::Error::new(
      field_span,
      "string.min_len cannot be larger than string.max_len",
    ));
  }

  Ok(templates)
}
