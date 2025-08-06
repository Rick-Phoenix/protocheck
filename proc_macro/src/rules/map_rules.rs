use proc_macro2::TokenStream;
use prost_reflect::{FieldDescriptor, Kind};
use proto_types::{protovalidate::FieldRules, FieldType};
use protocheck_core::field_data::FieldKind;
use quote::quote;
use syn::Error;

use super::{field_rules::Type as RulesType, Ignore};
use crate::{
  cel_rule_template::CelRuleTemplateTarget,
  extract_validators::{field_is_boxed, field_is_message},
  rules::{
    cel_rules::get_cel_rules,
    core::{convert_kind_to_proto_type, get_field_rules},
  },
  validation_data::{MapValidator, ValidationData},
};

pub fn get_map_rules(
  validation_data: ValidationData,
  static_defs: &mut Vec<TokenStream>,
  field_rust_enum: Option<String>,
  map_field_desc: &FieldDescriptor,
  field_rules: &FieldRules,
) -> Result<TokenStream, Error> {
  let mut map_level_rules = TokenStream::new();
  let mut key_rules = TokenStream::new();
  let mut value_rules = TokenStream::new();
  let mut key_context_tokens = TokenStream::new();
  let mut value_context_tokens = TokenStream::new();

  let map_field_span = validation_data.field_span;
  let error_prefix = format!("Error for field {}:", validation_data.full_name);

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
        "{} map field has no associated map entry message descriptor.",
        error_prefix
      ),
    ));
  };

  let (key_desc, value_desc) = (
    key_desc.ok_or(Error::new(
      map_field_span,
      format!("{} map entry missing 'key' field descriptor", error_prefix),
    ))?,
    value_desc.ok_or(Error::new(
      map_field_span,
      format!(
        "{} map entry missing 'value' field descriptor",
        error_prefix
      ),
    ))?,
  );

  let key_proto_type = convert_kind_to_proto_type(key_desc.kind());
  let value_proto_type = convert_kind_to_proto_type(value_desc.kind());

  let mut map_validation_data = validation_data;

  map_validation_data.key_type = Some(key_proto_type);
  map_validation_data.value_type = Some(value_proto_type);

  let value_is_message = field_is_message(&value_desc.kind());

  let mut ignore_values_validators = false;

  if !field_rules.cel.is_empty() {
    map_level_rules.extend(get_cel_rules(
      &CelRuleTemplateTarget::Field {
        field_desc: map_field_desc,
        is_boxed: false,
        validation_data: &map_validation_data,
      },
      &field_rules.cel,
      static_defs,
    )?);
  }

  if let Some(RulesType::Map(map_rules)) = field_rules.r#type.as_ref() {
    let mut min_pairs: Option<u64> = None;
    let mut max_pairs: Option<u64> = None;

    let field_context_ident = &map_validation_data.field_context_ident();
    let value_ident = map_validation_data.value_ident();

    if let Some(min_pairs_value) = map_rules.min_pairs {
      min_pairs = Some(min_pairs_value);

      let validator_expression_tokens = quote! {
        protocheck::validators::maps::min_pairs(&#field_context_ident, #value_ident, #min_pairs_value)
      };
      let validator_tokens = map_validation_data.get_validator_tokens(&validator_expression_tokens);

      map_level_rules.extend(validator_tokens);
    }

    if let Some(max_pairs_value) = map_rules.max_pairs {
      max_pairs = Some(max_pairs_value);

      let validator_expression_tokens = quote! {
        protocheck::validators::maps::max_pairs(&#field_context_ident, #value_ident, #max_pairs_value)
      };
      let validator_tokens = map_validation_data.get_validator_tokens(&validator_expression_tokens);

      map_level_rules.extend(validator_tokens);
    }

    if min_pairs.is_some() && max_pairs.is_some() && min_pairs.unwrap() > max_pairs.unwrap() {
      return Err(syn::Error::new(
        map_field_span,
        format!(
          "{} map.min_pairs cannot be larger than map.max_pairs",
          error_prefix
        ),
      ));
    }

    if let Some(key_rules_descriptor) = map_rules.keys.as_ref() {
      let ignore = key_rules_descriptor.ignore();

      if !matches!(ignore, Ignore::Always) {
        let mut key_validation_data = map_validation_data.clone();
        key_validation_data.field_kind = FieldKind::MapKey(key_proto_type.into());
        key_validation_data.ignore = ignore;

        if let Some(ref rules) = key_rules_descriptor.r#type {
          let key_validators_tokens = get_field_rules(
            static_defs,
            field_rust_enum.clone(),
            &key_desc,
            &key_validation_data,
            rules,
          )?;
          key_rules.extend(key_validators_tokens);
        }

        if !key_rules_descriptor.cel.is_empty() {
          let cel_rules = get_cel_rules(
            &CelRuleTemplateTarget::Field {
              validation_data: &key_validation_data,
              field_desc: &key_desc,
              is_boxed: false,
            },
            &key_rules_descriptor.cel,
            static_defs,
          )?;
          key_rules.extend(cel_rules);
        }

        if !key_rules.is_empty() {
          key_context_tokens = key_validation_data.field_context_tokens();
        }
      }
    }

    if let Some(value_rules_descriptor) = map_rules.values.as_ref() {
      let ignore = value_rules_descriptor.ignore();

      if matches!(ignore, Ignore::Always) {
        ignore_values_validators = true;
      } else {
        let mut values_validation_data = map_validation_data.clone();

        values_validation_data.field_kind = FieldKind::MapValue(value_proto_type.into());
        values_validation_data.ignore = ignore;

        if let Some(ref rules) = value_rules_descriptor.r#type {
          if !value_is_message {
            let value_validators_tokens = get_field_rules(
              static_defs,
              field_rust_enum,
              &value_desc,
              &values_validation_data,
              rules,
            )?;
            value_rules.extend(value_validators_tokens);
          }
        }

        if !value_rules_descriptor.cel.is_empty() {
          let cel_rules = get_cel_rules(
            &CelRuleTemplateTarget::Field {
              is_boxed: field_is_boxed(&value_desc, map_field_desc.parent_message()),
              validation_data: &values_validation_data,
              field_desc: &value_desc,
            },
            &value_rules_descriptor.cel,
            static_defs,
          )?;
          value_rules.extend(cel_rules);
        }

        if !value_rules.is_empty() {
          value_context_tokens = values_validation_data.field_context_tokens();
        }
      }
    }
  }

  if value_is_message && !ignore_values_validators {
    let mut validation_data = map_validation_data.clone();
    validation_data.field_kind = FieldKind::MapValue(FieldType::Message);

    let validator_tokens = validation_data.get_message_field_validator_tokens();

    value_rules.extend(validator_tokens);
  }

  if map_level_rules.is_empty() && key_rules.is_empty() && value_rules.is_empty() {
    Ok(map_level_rules)
  } else {
    Ok(map_validation_data.aggregate_map_rules(&MapValidator {
      key_context_tokens,
      value_context_tokens,
      map_level_rules,
      key_rules,
      value_rules,
    }))
  }
}
