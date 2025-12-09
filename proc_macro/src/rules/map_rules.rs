use crate::*;

pub fn get_map_rules(
  map_validation_data: &mut ValidationData,
  validation_tokens: &mut TokenStream2,
  field_rust_enum: Option<String>,
  map_field_desc: &FieldDescriptor,
  field_rules: &FieldRules,
) -> Result<(), Error> {
  let mut map_level_rules = TokenStream2::new();
  let mut keys_rules = TokenStream2::new();
  let mut values_rules = TokenStream2::new();

  let field_span = map_validation_data.field_span;
  let field_name = map_validation_data.full_name;

  let (key_desc, value_desc) =
    if let ProstReflectKind::Message(map_entry_message_desc) = map_field_desc.kind() {
      (
        map_entry_message_desc.get_field_by_name("key"),
        map_entry_message_desc.get_field_by_name("value"),
      )
    } else {
      return Err(get_field_error(
        field_name,
        field_span,
        "map field has no associated map entry message descriptor",
      ));
    };

  let (key_desc, value_desc) = (
    key_desc.ok_or(get_field_error(
      field_name,
      field_span,
      "map entry missing 'key' field descriptor",
    ))?,
    value_desc.ok_or(get_field_error(
      field_name,
      field_span,
      "map entry missing 'value' field descriptor",
    ))?,
  );

  let key_proto_type = convert_kind_to_proto_type(key_desc.kind());
  let value_proto_type = convert_kind_to_proto_type(value_desc.kind());

  map_validation_data.map_keys_type = Some(key_proto_type);
  map_validation_data.map_values_type = Some(value_proto_type);

  let value_is_message = field_is_message(&value_desc.kind());

  let mut ignore_values_validators = false;

  if !field_rules.cel.is_empty() {
    map_level_rules.extend(get_cel_rules_checked(
      &CelRuleTemplateTarget::Field {
        field_desc: map_field_desc,
        validation_data: map_validation_data,
        field_span: map_validation_data.field_span,
      },
      &field_rules.cel,
    )?);
  }

  if let Some(RulesType::Map(map_rules)) = field_rules.r#type.as_ref() {
    let length_rules = map_rules
      .length_rules()
      .map_err(|e| get_field_error(field_name, field_span, &e))?;

    if length_rules.has_rule() {
      map_validation_data.get_length_validator(&mut map_level_rules, length_rules);
    }

    if let Some(keys_rules_descriptor) = map_rules.keys.as_ref() {
      let ignore = keys_rules_descriptor.ignore();

      if !matches!(ignore, Ignore::Always) {
        let keys_validation_data = map_validation_data.to_map_key(key_proto_type.into());

        if let Some(ref rules) = keys_rules_descriptor.r#type {
          let key_validators_tokens = get_field_rules(
            field_rust_enum.clone(),
            &key_desc,
            &keys_validation_data,
            rules,
          )?;
          keys_rules.extend(key_validators_tokens);
        }

        if !keys_rules_descriptor.cel.is_empty() {
          let cel_rules = get_cel_rules_checked(
            &CelRuleTemplateTarget::Field {
              validation_data: &keys_validation_data,
              field_desc: &key_desc,
              field_span: map_validation_data.field_span,
            },
            &keys_rules_descriptor.cel,
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
              field_rust_enum,
              &value_desc,
              &values_validation_data,
              rules,
            )?;
            values_rules.extend(value_validators_tokens);
          }

        if !values_rules_descriptor.cel.is_empty() {
          let cel_rules = get_cel_rules_checked(
            &CelRuleTemplateTarget::Field {
              validation_data: &values_validation_data,
              field_desc: &value_desc,
              field_span: map_validation_data.field_span,
            },
            &values_rules_descriptor.cel,
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
