use crate::*;

#[derive(Clone, Debug)]
struct OneofField {
  pub ident: Ident,
  pub enum_path: Option<String>,
  pub span: Span,
}

#[derive(Debug)]
pub struct OneofValidatorsOutput {
  pub validators: HashMap<Ident, TokenStream2>,
}

pub fn extract_oneof_validators(
  item: &ItemEnum,
  oneof_desc: &OneofDescriptor,
) -> Result<OneofValidatorsOutput, Error> {
  let mut validators: HashMap<Ident, TokenStream2> = HashMap::new();
  let mut oneof_variants: HashMap<String, OneofField> = HashMap::new();

  let oneof_proto_name = &oneof_desc.name();

  for variant in &item.variants {
    let OneofVariantAttrs { name, enum_path } =
      extract_oneof_variant_attrs(&variant.attrs, &variant.ident)?;

    oneof_variants.insert(
      name,
      OneofField {
        ident: variant.ident.clone(),
        enum_path,
        span: variant.span(),
      },
    );
  }

  for field in oneof_desc.fields() {
    let OneofField {
      enum_path: enum_ident,
      ident: field_ident,
      span: field_span,
      ..
    } = oneof_variants
      .remove(field.name())
      .ok_or(Error::new(
        Span::call_site(),
        format!(
          "Could not process the data for field {} in oneof {}. Is the name set correctly?",
          field.name(),
          oneof_proto_name
        ),
      ))?;

    let mut field_validators = TokenStream2::new();

    let field_options = field.options();
    let field_rules_descriptor = field_options.get_extension(&FIELD_RULES_EXT_DESCRIPTOR);

    if let ProstValue::Message(field_rules_message) = field_rules_descriptor.as_ref() {
      let field_rules = FieldRules::decode(field_rules_message.encode_to_vec().as_slice())
        .map_err(|e| Error::new_spanned(item, format!("Could not decode field rules: {}", e)))?;

      let ignore = field_rules.ignore();
      let is_required = field_rules.required() && field.supports_presence();

      if matches!(ignore, Ignore::Always) {
        continue;
      }

      let field_name = field.name();

      let item_rust_ident = new_ident(field.name());
      let field_context_ident = new_ident("field_context");
      let index_ident = new_ident("idx");
      let item_ident = new_ident("item");
      let key_ident = new_ident("key");
      let map_value_ident = new_ident("val");
      let violations_ident = new_ident("violations");
      let parent_messages_ident = new_ident("parent_messages");
      let map_key_context_ident = new_ident("key_context");
      let map_value_context_ident = new_ident("value_context");
      let vec_item_context_ident = new_ident("item_context");

      let validation_data = ValidationData {
        full_name: field.full_name(),
        is_required,
        is_in_oneof: true,
        is_optional: true,
        is_boxed: field_is_boxed(&field, oneof_desc.parent_message()),
        field_span,
        proto_name: field_name,
        tag: field.number() as i32,
        ignore,
        map_keys_type: None,
        map_values_type: None,
        violations_ident: &violations_ident,
        field_context_ident: &field_context_ident,
        item_ident: &item_ident,
        parent_messages_ident: &parent_messages_ident,
        map_key_ident: &key_ident,
        map_value_ident: &map_value_ident,
        index_ident: &index_ident,
        item_rust_ident: &item_rust_ident,
        field_kind: FieldKind::Single(get_field_type(&field)),
        map_key_context_ident: &map_key_context_ident,
        map_value_context_ident: &map_value_context_ident,
        vec_item_context_ident: &vec_item_context_ident,
        value_ident: OnceCell::new(),
      };

      if !field_rules.cel.is_empty() {
        field_validators.extend(get_cel_rules_checked(
          &CelRuleTemplateTarget::Field {
            field_desc: &field,
            validation_data: &validation_data,
            field_span,
          },
          &field_rules.cel,
        )?);
      }

      if let Some(ref rules_type) = field_rules.r#type {
        let rules = get_field_rules(enum_ident, &field, &validation_data, rules_type)?;
        field_validators.extend(rules);
      }

      if !field_validators.is_empty() {
        field_validators = validation_data.get_aggregated_validator_tokens(field_validators);
      }

      if field_is_message(&field.kind()) {
        validation_data.get_message_field_validator_tokens(
          &mut field_validators,
          FieldKind::Single(FieldType::Message),
        );
      }

      validators.insert(field_ident, field_validators);
    }
  }

  Ok(OneofValidatorsOutput { validators })
}
