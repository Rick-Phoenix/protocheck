use prost_reflect::{FieldDescriptor, Kind};
use quote::{quote, ToTokens};
use syn::Error;

use super::{field_rules::Type as RulesType, FieldData, Ignore, ValidatorKind, ValidatorTemplate};
use crate::{
  rules::{
    cel_rules::{get_cel_rules, CelRuleKind},
    core::{convert_kind_to_proto_type, get_field_rules},
  },
  Span2,
};

pub fn get_map_rules(
  field_rust_enum: Option<String>,
  map_field_span: Span2,
  map_field_desc: &FieldDescriptor,
  map_field_data: &FieldData,
  field_rules: Option<&RulesType>,
) -> Result<Option<ValidatorTemplate>, Error> {
  let mut map_level_rules: Vec<ValidatorTemplate> = Vec::new();
  let mut key_rules: Vec<ValidatorTemplate> = Vec::new();
  let mut value_rules: Vec<ValidatorTemplate> = Vec::new();

  let (key_desc, value_desc) = if let Kind::Message(map_entry_message_desc) = map_field_desc.kind()
  {
    (
      map_entry_message_desc.get_field_by_name("key"),
      map_entry_message_desc.get_field_by_name("value"),
    )
  } else {
    return Err(Error::new(
      map_field_span,
      format!(
        "Map field {} has no associated map entry message descriptor.",
        map_field_desc.name()
      ),
    ));
  };

  let (key_desc, value_desc) = (
    key_desc.ok_or(Error::new(
      map_field_span,
      "Map entry missing 'key' field descriptor",
    ))?,
    value_desc.ok_or(Error::new(
      map_field_span,
      "Map entry missing 'value' field descriptor",
    ))?,
  );

  let key_proto_type = convert_kind_to_proto_type(&key_desc.kind());
  let value_proto_type = convert_kind_to_proto_type(&value_desc.kind());

  let mut map_field_data = map_field_data.clone();
  map_field_data.key_type = Some(key_proto_type);
  map_field_data.value_type = Some(value_proto_type);
  map_field_data.is_repeated = false;

  let mut value_is_message = false;
  if let Kind::Message(value_message_desc) = value_desc.kind() {
    if !value_message_desc
      .full_name()
      .starts_with("google.protobuf")
    {
      value_is_message = true;
    }
  }

  if let Some(RulesType::Map(map_rules)) = field_rules {
    let mut min_pairs: Option<u64> = None;
    let mut max_pairs: Option<u64> = None;

    if let Some(min_pairs_value) = map_rules.min_pairs {
      min_pairs = Some(min_pairs_value);
      map_level_rules.push(ValidatorTemplate {
        kind: ValidatorKind::FieldRule {
          validator_path: quote! { macro_impl::validators::maps::min_pairs },
          target_value_tokens: min_pairs_value.into_token_stream(),
        },
        field_data: map_field_data.clone(),
      });
    }

    if let Some(max_pairs_value) = map_rules.max_pairs {
      max_pairs = Some(max_pairs_value);
      map_level_rules.push(ValidatorTemplate {
        kind: ValidatorKind::FieldRule {
          validator_path: quote! { macro_impl::validators::maps::max_pairs },
          target_value_tokens: max_pairs_value.into_token_stream(),
        },
        field_data: map_field_data.clone(),
      });
    }

    if min_pairs.is_some() && max_pairs.is_some() && min_pairs.unwrap() > max_pairs.unwrap() {
      return Err(syn::Error::new(
        map_field_span,
        "map.min_pairs cannot be larger than map.max_pairs",
      ));
    }

    if let Some(key_rules_descriptor) = map_rules.keys.as_ref() {
      let ignore = key_rules_descriptor.ignore();
      if let Some(ref rules) = key_rules_descriptor.r#type {
        if !matches!(ignore, Ignore::Always) {
          let is_required = key_rules_descriptor.required();

          let mut key_field_data = map_field_data.clone();
          key_field_data.is_required = is_required;
          key_field_data.is_map = false;
          key_field_data.is_map_key = true;
          key_field_data.ignore = ignore;

          let generated_key_templates = get_field_rules(
            field_rust_enum.clone(),
            map_field_span,
            &key_desc,
            &key_field_data,
            rules,
          )?;
          key_rules.extend(generated_key_templates);

          if !key_rules_descriptor.cel.is_empty() {
            let cel_rules = get_cel_rules(
              &CelRuleKind::Field(&key_desc),
              &key_field_data,
              &key_rules_descriptor.cel,
            )?;
            key_rules.extend(cel_rules);
          }
        }
      }
    }

    if let Some(value_rules_descriptor) = map_rules.values.as_ref() {
      let ignore = value_rules_descriptor.ignore();
      if let Some(ref rules) = value_rules_descriptor.r#type {
        if !matches!(ignore, Ignore::Always) {
          let is_required = value_rules_descriptor.required();

          let mut value_field_data = map_field_data.clone();
          value_field_data.is_required = is_required;
          value_field_data.is_map = false;
          value_field_data.is_map_value = true;
          value_field_data.ignore = ignore;

          if !value_rules_descriptor.cel.is_empty() {
            let cel_rules = get_cel_rules(
              &CelRuleKind::Field(&value_desc),
              &value_field_data,
              &value_rules_descriptor.cel,
            )?;
            value_rules.extend(cel_rules);
          }

          if !value_is_message {
            let generated_value_templates = get_field_rules(
              field_rust_enum,
              map_field_span,
              &value_desc,
              &value_field_data,
              rules,
            )?;
            value_rules.extend(generated_value_templates);
          }
        }
      }
    }
  }

  if value_is_message {
    let mut value_field_data = map_field_data.clone();
    value_field_data.is_map = false;
    value_field_data.is_map_value = true;
    let value_message_rules = ValidatorTemplate {
      field_data: value_field_data,
      kind: ValidatorKind::MessageField,
    };

    value_rules.push(value_message_rules);
  }

  if map_level_rules.is_empty() && key_rules.is_empty() && value_rules.is_empty() {
    Ok(None)
  } else {
    Ok(Some(ValidatorTemplate {
      field_data: map_field_data.to_owned(),
      kind: ValidatorKind::MapValidationLoop {
        map_level_rules,
        key_rules,
        value_rules,
      },
    }))
  }
}
