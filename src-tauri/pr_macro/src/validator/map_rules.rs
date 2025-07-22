use crate::validator::core::convert_kind_to_proto_type;
use crate::validator::core::get_field_rules;
use proc_macro2::{Ident, Span, TokenStream};
use prost_reflect::{FieldDescriptor, Kind};
use proto_types::buf::validate::Ignore;
use proto_types::buf::validate::MapRules;
use proto_types::google::protobuf::field_descriptor_proto::Type as ProtoType;
use proto_types::FieldData;
use proto_types::GeneratedCodeKind;
use proto_types::ValidatorCallTemplate;
use quote::{quote, ToTokens};

pub fn get_map_rules(
  map_field_desc: &FieldDescriptor,
  map_rules: &MapRules,
  ignore: Ignore,
) -> Result<ValidatorCallTemplate, Box<dyn std::error::Error>> {
  let mut map_level_rules_templates: Vec<ValidatorCallTemplate> = Vec::new();
  let mut key_rules_templates: Vec<ValidatorCallTemplate> = Vec::new();
  let mut value_rules_templates: Vec<ValidatorCallTemplate> = Vec::new();

  let (key_desc, value_desc) = if let Kind::Message(map_entry_message_desc) = map_field_desc.kind()
  {
    (
      map_entry_message_desc.get_field_by_name("key"),
      map_entry_message_desc.get_field_by_name("value"),
    )
  } else {
    return Err(
      format!(
        "Map field {} has no associated map entry message descriptor.",
        map_field_desc.name()
      )
      .into(),
    );
  };

  let (key_desc, value_desc) = (
    key_desc.ok_or("Map entry missing 'key' field descriptor")?,
    value_desc.ok_or("Map entry missing 'value' field descriptor")?,
  );

  let key_proto_type = convert_kind_to_proto_type(key_desc.kind());
  let value_proto_type = convert_kind_to_proto_type(value_desc.kind());

  let map_field_data = FieldData {
    rust_name: map_field_desc.name().to_string(),
    proto_name: map_field_desc.name().to_string(),
    proto_type: ProtoType::Message,
    tag: map_field_desc.number(),
    is_required: false,
    is_map: true,
    is_repeated: false,
    is_optional: false,
    is_for_key: false,
    key_type: Some(key_proto_type),
    value_type: Some(value_proto_type),
    ignore: ignore,
  };

  if map_rules.min_pairs.is_some() {
    let min_pairs_value = map_rules.min_pairs.unwrap() as usize;
    map_level_rules_templates.push(ValidatorCallTemplate {
      validator_path: Some(quote! { macro_impl::validators::map::min_pairs }),
      target_value_tokens: Some(min_pairs_value.into_token_stream()),
      kind: GeneratedCodeKind::FieldRule,
      field_data: map_field_data.clone(),
    });
  }

  if map_rules.keys.is_some() {
    let key_rules_descriptor = map_rules.keys.clone().unwrap();
    let ignore = key_rules_descriptor.ignore();
    if !matches!(ignore, Ignore::Always) {
      let is_required = key_rules_descriptor.required();

      let mut key_field_data = map_field_data.clone();
      key_field_data.is_required = is_required;
      key_field_data.is_for_key = true;
      key_field_data.ignore = ignore;

      let generated_key_templates = get_field_rules(key_field_data, &key_rules_descriptor)?;
      key_rules_templates.extend(generated_key_templates);
    }
  }

  let mut value_is_message = false;
  if let Kind::Message(_) = value_desc.kind() {
    value_is_message = true;
  }

  // Change it for well known types
  if map_rules.values.is_some() && !value_is_message {
    let value_rules_descriptor = map_rules.values.clone().unwrap();
    let ignore = value_rules_descriptor.ignore();
    if !matches!(ignore, Ignore::Always) {
      let is_required = value_rules_descriptor.required();

      let mut value_field_data = map_field_data.clone();
      value_field_data.is_required = is_required;

      value_field_data.ignore = ignore;

      let generated_value_templates = get_field_rules(value_field_data, &value_rules_descriptor)?;
      value_rules_templates.extend(generated_value_templates);
    }
  }

  Ok(ValidatorCallTemplate {
    validator_path: None,
    target_value_tokens: None,
    field_data: map_field_data,
    kind: GeneratedCodeKind::MapValidationLoop {
      map_level_rules: map_level_rules_templates,
      key_rules: key_rules_templates,
      value_rules: value_rules_templates,
      value_is_message,
    },
  })
}
