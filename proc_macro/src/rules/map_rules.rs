use proc_macro2::TokenStream;
use prost_reflect::{FieldDescriptor, Kind};
use proto_types::{protovalidate::FieldRules, FieldType};
use protocheck_core::field_data::FieldKind;
use quote::quote;
use syn::Error;

#[cfg(not(feature = "cel"))]
use super::get_cel_rules;
use super::{field_rules::Type as RulesType, Ignore};
#[cfg(feature = "cel")]
use crate::rules::cel_rules::get_cel_rules;
use crate::{
  cel_rule_template::CelRuleTemplateTarget,
  extract_validators::field_is_message,
  rules::core::{convert_kind_to_proto_type, get_field_rules, get_field_type},
  validation_data::{MapValidator, ValidationData},
};

pub fn get_map_rules(
  validation_data: ValidationData,
  validation_tokens: &mut TokenStream,
  static_defs: &mut TokenStream,
  field_rust_enum: Option<String>,
  map_field_desc: &FieldDescriptor,
  field_rules: &FieldRules,
) -> Result<(), Error> {
  let mut map_level_rules = TokenStream::new();
  let mut keys_rules = TokenStream::new();
  let mut values_rules = TokenStream::new();

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

  map_validation_data.map_keys_type = Some(key_proto_type);
  map_validation_data.map_values_type = Some(value_proto_type);

  let value_is_message = field_is_message(&value_desc.kind());

  let mut ignore_values_validators = false;

  if !field_rules.cel.is_empty() {
    map_level_rules.extend(get_cel_rules(
      &CelRuleTemplateTarget::Field {
        field_desc: map_field_desc,
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

      let plural_suffix = if min_pairs_value != 1 { "s" } else { "" };

      let error_message = format!(
        "must contain at least {:?} key-value pair{}",
        min_pairs_value, plural_suffix
      );

      let validator_expression_tokens = quote! {
        protocheck::validators::maps::min_pairs(&#field_context_ident, #value_ident.len(), #min_pairs_value, #error_message)
      };
      map_validation_data.get_validator_tokens(&mut map_level_rules, &validator_expression_tokens);
    }

    if let Some(max_pairs_value) = map_rules.max_pairs {
      max_pairs = Some(max_pairs_value);

      let plural_suffix = if max_pairs_value != 1 { "s" } else { "" };

      let error_message = format!(
        "cannot contain more than {:?} key-value pair{}",
        max_pairs_value, plural_suffix
      );

      let validator_expression_tokens = quote! {
        protocheck::validators::maps::max_pairs(&#field_context_ident, #value_ident.len(), #max_pairs_value, #error_message)
      };
      map_validation_data.get_validator_tokens(&mut map_level_rules, &validator_expression_tokens);
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

    if let Some(keys_rules_descriptor) = map_rules.keys.as_ref() {
      let ignore = keys_rules_descriptor.ignore();

      if !matches!(ignore, Ignore::Always) {
        let keys_validation_data = map_validation_data.to_map_key(key_proto_type.into());

        if let Some(ref rules) = keys_rules_descriptor.r#type {
          let key_validators_tokens = get_field_rules(
            static_defs,
            field_rust_enum.clone(),
            &key_desc,
            &keys_validation_data,
            rules,
          )?;
          keys_rules.extend(key_validators_tokens);
        }

        if !keys_rules_descriptor.cel.is_empty() {
          let cel_rules = get_cel_rules(
            &CelRuleTemplateTarget::Field {
              validation_data: &keys_validation_data,
              field_desc: &key_desc,
            },
            &keys_rules_descriptor.cel,
            static_defs,
          )?;
          keys_rules.extend(cel_rules);
        }
      }
    }

    if let Some(values_rules_descriptor) = map_rules.values.as_ref() {
      let ignore = values_rules_descriptor.ignore();

      if matches!(ignore, Ignore::Always) {
        ignore_values_validators = true;
      } else {
        let values_validation_data = map_validation_data.to_map_value(get_field_type(&value_desc));

        if let Some(ref rules) = values_rules_descriptor.r#type
          && !value_is_message {
            let value_validators_tokens = get_field_rules(
              static_defs,
              field_rust_enum,
              &value_desc,
              &values_validation_data,
              rules,
            )?;
            values_rules.extend(value_validators_tokens);
          }

        if !values_rules_descriptor.cel.is_empty() {
          let cel_rules = get_cel_rules(
            &CelRuleTemplateTarget::Field {
              validation_data: &values_validation_data,
              field_desc: &value_desc,
            },
            &values_rules_descriptor.cel,
            static_defs,
          )?;
          values_rules.extend(cel_rules);
        }
      }
    }
  }

  if value_is_message && !ignore_values_validators {
    map_validation_data.get_message_field_validator_tokens(
      &mut values_rules,
      FieldKind::MapValue(FieldType::Message),
    );
  }

  if map_level_rules.is_empty() && keys_rules.is_empty() && values_rules.is_empty() {
    Ok(())
  } else {
    map_validation_data.aggregate_map_rules(
      validation_tokens,
      &MapValidator {
        map_level_rules,
        keys_rules,
        values_rules,
      },
    );
    Ok(())
  }
}
