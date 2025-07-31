use proc_macro2::TokenStream;
use prost_reflect::{FieldDescriptor, Kind};
use protocheck_core::field_data::FieldKind;
use quote::{quote, ToTokens};
use syn::Error;

use super::{field_rules::Type as RulesType, Ignore, ValidatorKind, ValidatorTemplate};
use crate::{
  cel_rule_template::CelRuleTemplateTarget,
  rules::{
    cel_rules::get_cel_rules,
    core::{convert_kind_to_proto_type, get_field_rules},
  },
  validation_data::ValidationData,
  validator_template::FieldValidator,
};

pub fn get_map_rules(
  validation_data: &ValidationData,
  static_defs: &mut Vec<TokenStream>,
  field_rust_enum: Option<String>,
  map_field_desc: &FieldDescriptor,
  field_rules: Option<&RulesType>,
) -> Result<Option<ValidatorTemplate>, Error> {
  let mut map_level_rules: Vec<ValidatorTemplate> = Vec::new();
  let mut key_rules: Vec<ValidatorTemplate> = Vec::new();
  let mut value_rules: Vec<ValidatorTemplate> = Vec::new();

  let map_field_span = validation_data.field_span;

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

  let mut map_field_data = validation_data.field_data.clone();
  map_field_data.key_type = Some(key_proto_type);
  map_field_data.value_type = Some(value_proto_type);

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
        item_rust_name: map_field_data.rust_name.clone(),
        kind: ValidatorKind::Field {
          validation_data: validation_data.clone(),
          field_validator: FieldValidator::Scalar {
            validator_path: quote! { macro_impl::validators::maps::min_pairs },
            target_value_tokens: min_pairs_value.into_token_stream(),
          },
        },
      });
    }

    if let Some(max_pairs_value) = map_rules.max_pairs {
      max_pairs = Some(max_pairs_value);
      map_level_rules.push(ValidatorTemplate {
        item_rust_name: map_field_data.rust_name.clone(),
        kind: ValidatorKind::Field {
          validation_data: validation_data.clone(),
          field_validator: FieldValidator::Scalar {
            validator_path: quote! { macro_impl::validators::maps::max_pairs },
            target_value_tokens: max_pairs_value.into_token_stream(),
          },
        },
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
          let mut key_validation_data = validation_data.clone();
          key_validation_data.field_data.kind = FieldKind::MapKey;
          key_validation_data.field_data.ignore = ignore;

          let generated_key_templates =
            get_field_rules(field_rust_enum.clone(), &key_desc, validation_data, rules)?;
          key_rules.extend(generated_key_templates);

          if !key_rules_descriptor.cel.is_empty() {
            let cel_rules = get_cel_rules(
              &CelRuleTemplateTarget::Field(key_desc, key_validation_data),
              &key_rules_descriptor.cel,
              static_defs,
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
          let mut values_validation_data = validation_data.clone();
          values_validation_data.field_data.kind = FieldKind::MapValue;
          values_validation_data.field_data.ignore = ignore;

          if !value_is_message {
            let generated_value_templates =
              get_field_rules(field_rust_enum, &value_desc, validation_data, rules)?;
            value_rules.extend(generated_value_templates);
          }

          if !value_rules_descriptor.cel.is_empty() {
            let cel_rules = get_cel_rules(
              &CelRuleTemplateTarget::Field(value_desc, values_validation_data),
              &value_rules_descriptor.cel,
              static_defs,
            )?;
            value_rules.extend(cel_rules);
          }
        }
      }
    }
  }

  if value_is_message {
    let value_message_rules = ValidatorTemplate {
      item_rust_name: map_field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::MessageField,
      },
    };

    value_rules.push(value_message_rules);
  }

  if map_level_rules.is_empty() && key_rules.is_empty() && value_rules.is_empty() {
    Ok(None)
  } else {
    Ok(Some(ValidatorTemplate {
      item_rust_name: map_field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Map {
          map_level_rules,
          key_rules,
          value_rules,
        },
      },
    }))
  }
}
