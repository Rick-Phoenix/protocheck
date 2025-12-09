use crate::*;

pub fn get_repeated_rules(
  validation_data: &ValidationData,
  validation_tokens: &mut TokenStream2,
  field_rust_enum: Option<String>,
  field_desc: &FieldDescriptor,
  field_rules: &FieldRules,
) -> Result<(), Error> {
  let mut vec_level_rules: TokenStream2 = TokenStream2::new();
  let mut items_rules: TokenStream2 = TokenStream2::new();
  let mut items_validation_data: Option<ValidationData> = None;

  let field_span = validation_data.field_span;
  let field_name = validation_data.full_name;

  let item_is_message = field_is_message(&field_desc.kind());

  let mut ignore_items_validators = false;

  if !field_rules.cel.is_empty() {
    vec_level_rules.extend(get_cel_rules_checked(
      &CelRuleTemplateTarget::Field {
        field_desc,
        validation_data,
        field_span: validation_data.field_span,
      },
      &field_rules.cel,
    )?);
  }

  if let Some(RulesType::Repeated(ref repeated_rules)) = field_rules.r#type {
    if repeated_rules.unique() {
      if !supports_unique(validation_data.field_kind.inner_type()) {
        return Err(get_field_error(
          field_name,
          field_span,
          "repeated.unique only works for scalar fields",
        ));
      }

      let items_validation_data =
        items_validation_data.get_or_insert_with(|| validation_data.to_repeated_item(field_desc));

      let field_context_ident = items_validation_data.field_context_ident();
      let value_ident = items_validation_data.value_ident();
      let violations_ident = items_validation_data.violations_ident;

      let vec_ident = validation_data.value_ident();

      let is_float = matches!(
        validation_data.field_kind.inner_type(),
        FieldType::Float | FieldType::Double
      );

      let ordered_floats_enabled = cfg!(feature = "ordered-float");

      let lookup_tokens = if is_float && !ordered_floats_enabled {
        quote! { ::protocheck::validators::repeated::UniqueLookup::Vec(vec![]) }
      } else {
        quote! {
          if #vec_ident.len() < 16 {
            ::protocheck::validators::repeated::UniqueLookup::Vec(vec![])
          } else {
            ::protocheck::validators::repeated::UniqueLookup::Set(::std::collections::HashSet::new())
          }
        }
      };

      vec_level_rules.extend(quote! {
        let mut processed_values = #lookup_tokens;
        let mut found_not_unique_items = false;
      });

      items_rules.extend(quote! {
        if !found_not_unique_items {
          match ::protocheck::validators::repeated::unique(&#field_context_ident, #value_ident, &mut processed_values) {
            Ok(_) => {},
            Err(v) => {
              found_not_unique_items = true;
              #violations_ident.push(v);
            }
          };
        }
      });
    }

    let length_rules = repeated_rules
      .length_rules()
      .map_err(|e| get_field_error(field_name, field_span, &e))?;

    if length_rules.has_rule() {
      validation_data.get_length_validator(&mut vec_level_rules, length_rules);
    }

    if let Some(items_rules_descriptor) = repeated_rules.items.as_ref() {
      let ignore = items_rules_descriptor.ignore();

      if matches!(ignore, Ignore::Always) {
        ignore_items_validators = true
      } else {
        let repeated_items_validation_data =
          items_validation_data.get_or_insert_with(|| validation_data.to_repeated_item(field_desc));

        if let Some(ref rules_type) = items_rules_descriptor.r#type
          && !item_is_message {
            let items_rules_tokens = get_field_rules(
              field_rust_enum,
              field_desc,
              repeated_items_validation_data,
              rules_type,
            )?;

            items_rules.extend(items_rules_tokens);
          }

        if !items_rules_descriptor.cel.is_empty() {
          let cel_rules = get_cel_rules_checked(
            &CelRuleTemplateTarget::Field {
              field_desc,
              validation_data: repeated_items_validation_data,
              field_span: validation_data.field_span,
            },
            &items_rules_descriptor.cel,
          )?;
          items_rules.extend(cel_rules);
        }
      }
    }
  }

  if item_is_message && !ignore_items_validators {
    validation_data.get_message_field_validator_tokens(
      &mut items_rules,
      FieldKind::RepeatedItem(FieldType::Message),
    );
  }

  validation_data.aggregate_vec_rules(
    validation_tokens,
    &RepeatedValidator {
      vec_level_rules,
      items_rules,
    },
  );

  Ok(())
}

fn supports_unique(field_type: FieldType) -> bool {
  !matches!(field_type, FieldType::Message | FieldType::Any)
}
