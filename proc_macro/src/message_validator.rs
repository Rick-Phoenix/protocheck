use crate::*;

pub fn extract_message_validators(
  item: &ItemStruct,
  message_desc: &MessageDescriptor,
) -> Result<TokenStream2, Error> {
  let ItemStruct { fields, .. } = item;

  let mut validators: TokenStream2 = TokenStream2::new();

  let mut rust_field_spans: HashMap<String, Span> = HashMap::new();
  // <Field name, Enum name>
  let mut enum_fields: HashMap<String, String> = HashMap::new();

  for field in fields {
    if let Some(ident) = &field.ident {
      for attr in field
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("prost"))
      {
        if let Some(enum_path) = attr.parse_args::<ProstAttrData>()?.enum_path {
          enum_fields.insert(ident.to_string(), enum_path);
        }
      }

      rust_field_spans.insert(ident.to_string(), field.span());
    }
  }

  let message_options = message_desc.options();
  let message_rules_descriptor = message_options.get_extension(&MESSAGE_RULES_EXT_DESCRIPTOR);

  let violations_ident = new_ident("violations");
  let parent_messages_ident = new_ident("parent_messages");

  // Message Rules
  if let ProstValue::Message(message_rules_msg) = message_rules_descriptor.as_ref() {
    let message_rules = MessageRules::decode(message_rules_msg.encode_to_vec().as_slice())
      .map_err(|e| error!(item, format!("Could not decode message rules: {e}")))?;

    if !message_rules.cel.is_empty() {
      validators.extend(get_cel_rules_checked(
        &CelRuleTemplateTarget::Message {
          message_desc,
          parent_messages_ident: parent_messages_ident.clone(),
          violations_ident: violations_ident.clone(),
          struct_span: item.span(),
        },
        &message_rules.cel,
      )?);
    }
  }

  // Oneof rules
  for oneof in message_desc.oneofs() {
    if oneof.is_synthetic() {
      continue;
    }

    if let ProstValue::Message(oneof_rules_msg) = oneof
      .options()
      .get_extension(&ONEOF_RULES_EXT_DESCRIPTOR)
      .as_ref()
    {
      let oneof_rules = OneofRules::decode(oneof_rules_msg.encode_to_vec().as_slice())
        .map_err(|e| error!(item, format!("Could not decode oneof rules: {}", e)))?;

      let oneof_proto_name = oneof.name();
      let oneof_rust_ident = proto_name_to_rust_ident(oneof_proto_name);

      let required_check = oneof_rules.required().then_some(quote! {
        #violations_ident.push(::protocheck::validators::oneofs::required(#oneof_proto_name, #parent_messages_ident.as_slice()));
      });

      validators.extend(quote! {
        match &self.#oneof_rust_ident {
          Some(oneof) => { oneof.validate(#parent_messages_ident, #violations_ident); },
          None => { #required_check }
        };
      });
    }
  }

  // Field Rules
  for field in message_desc.fields() {
    if let Some(containing_oneof) = field.containing_oneof().as_ref()
      && !containing_oneof.is_synthetic() {
        continue;
      }

    let field_proto_name = field.name();
    let field_rust_name = proto_name_to_rust_name(field_proto_name);

    let mut field_validators = TokenStream2::new();

    let item_rust_ident = proto_name_to_rust_ident(field_proto_name);
    let field_context_ident = new_ident("field_context");
    let index_ident = new_ident("idx");
    let item_ident = new_ident("item");
    let key_ident = new_ident("key");
    let map_value_ident = new_ident("val");
    let map_key_context_ident = new_ident("key_context");
    let map_value_context_ident = new_ident("value_context");
    let vec_item_context_ident = new_ident("item_context");

    let field_span = rust_field_spans
      .get(field_rust_name.as_ref())
      .cloned()
      .unwrap_or_else(Span::call_site);

    let field_rust_enum = enum_fields.get(field_rust_name.as_ref()).cloned();

    let is_repeated = field.is_list();
    let is_map = field.is_map();
    let is_optional = field.supports_presence();
    let field_tag = field.number();

    let field_options = field.options();
    let field_rules_descriptor = field_options.get_extension(&FIELD_RULES_EXT_DESCRIPTOR);

    if let ProstValue::Message(field_rules_msg) = field_rules_descriptor.as_ref() {
      let field_rules = FieldRules::decode(field_rules_msg.encode_to_vec().as_slice())
        .map_err(|e| Error::new_spanned(item, format!("Could not decode field rules: {}", e)))?;

      let ignore = field_rules.ignore();
      let is_required = field_rules.required() && field.supports_presence();

      if matches!(ignore, Ignore::Always) {
        continue;
      }

      let mut validation_data = ValidationData {
        proto_name: field_proto_name,
        tag: field_tag as i32,
        ignore,
        full_name: field.full_name(),
        is_required,
        is_in_oneof: false,
        is_optional,
        is_boxed: field_is_boxed(&field, message_desc),
        field_span,
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
        field_kind: get_field_kind(&field),
        map_key_context_ident: &map_key_context_ident,
        map_value_context_ident: &map_value_context_ident,
        vec_item_context_ident: &vec_item_context_ident,
        value_ident: OnceCell::new(),
      };

      let field_rules_type = field_rules.r#type.as_ref();
      if is_repeated {
        get_repeated_rules(
          &validation_data,
          &mut field_validators,
          field_rust_enum,
          &field,
          &field_rules,
        )?;
      } else if is_map {
        get_map_rules(
          &mut validation_data,
          &mut field_validators,
          field_rust_enum,
          &field,
          &field_rules,
        )?;
      } else {
        if let Some(rules_type) = field_rules_type {
          let rules = get_field_rules(field_rust_enum, &field, &validation_data, rules_type)?;

          field_validators.extend(rules);
        }

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

        if field_is_message(&field.kind()) {
          validation_data.get_message_field_validator_tokens(
            &mut field_validators,
            FieldKind::Single(FieldType::Message),
          );
        }

        if !field_validators.is_empty() {
          field_validators = validation_data.get_aggregated_validator_tokens(field_validators);
        } else if is_required {
          validation_data.get_required_only_validator(&mut field_validators);
        }
      }

      validators.extend(field_validators);
    }
  }

  Ok(validators)
}

pub fn field_is_boxed(field_desc: &FieldDescriptor, message_desc: &MessageDescriptor) -> bool {
  if let ProstReflectKind::Message(field_message_desc) = field_desc.kind() {
    return !field_desc.is_list() && field_message_desc.full_name() == message_desc.full_name();
  }
  false
}

pub fn field_is_message(field_kind: &ProstReflectKind) -> bool {
  if let ProstReflectKind::Message(field_message_desc) = field_kind
    && !field_message_desc
      .full_name()
      .starts_with("google.")
    {
      return true;
    }
  false
}
