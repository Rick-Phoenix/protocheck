use super::{CelRule, CelRuleValue};
use proc_macro2::{Ident, Span, TokenStream};
use proto_types::buf::validate;
use proto_types::buf::validate::field_path_element::Subscript;
use proto_types::buf::validate::StringRules;
use proto_types::{FieldData, GeneratedCodeKind, ValidatorCallTemplate};
use quote::{quote, ToTokens};
use regex::Regex;

pub fn get_string_rules(
  field_data: FieldData,
  string_rules: &proto_types::buf::validate::StringRules,
) -> Result<Vec<ValidatorCallTemplate>, Box<dyn std::error::Error>> {
  let mut templates: Vec<ValidatorCallTemplate> = Vec::new();

  if string_rules.max_len.is_some() {
    let max_len_value = string_rules.max_len.unwrap() as usize;

    templates.push(ValidatorCallTemplate {
      validator_path: Some(quote! { macro_impl::validators::strings::max_len }),
      target_value_tokens: Some(max_len_value.into_token_stream()),

      field_rust_ident: field_data.name.clone(),
      field_tag: field_data.tag,
      field_proto_name: field_data.name.clone(),
      field_proto_type: proto_types::google::protobuf::field_descriptor_proto::Type::String,
      field_is_repeated: field_data.is_repeated,
      field_is_map: field_data.is_map,
      field_is_required: field_data.is_required,
      field_is_optional: field_data.is_optional,
      for_key: field_data.for_key,
      key_type: field_data.key_type,
      value_type: field_data.value_type,
      ignore: field_data.ignore,
      kind: GeneratedCodeKind::FieldRule,
    });
  }

  Ok(templates)
}
