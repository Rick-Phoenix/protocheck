use crate::*;

pub fn get_field_rules(
  static_defs: &mut TokenStream2,
  field_rust_enum: Option<String>,
  field_desc: &FieldDescriptor,
  validation_data: &ValidationData,
  field_rules: &RulesType,
) -> Result<TokenStream2, Error> {
  let mut rules_tokens = TokenStream2::new();
  let mut error: Option<&str> = None;

  let field_name = &validation_data.full_name;
  let field_proto_kind = &field_desc.kind();
  let field_span = validation_data.field_span;

  let error_prefix = &format!("Error for field {}:", field_name);

  field_rules.matches_type(
    validation_data.field_kind.inner_type(),
    field_span,
    error_prefix,
  )?;

  match field_rules {
    RulesType::Float(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Double(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Int32(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Int64(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Uint32(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Uint64(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Sint32(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Sint64(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Fixed32(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Fixed64(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Sfixed32(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Sfixed64(rules) => {
      let rules = get_numeric_rules(validation_data, rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::String(string_rules) => {
      let rules = get_string_rules(static_defs, validation_data, string_rules)?;
      rules_tokens.extend(rules);
    }
    RulesType::Enum(enum_rules) => {
      if let ProstReflectKind::Enum(enum_descriptor) = &field_proto_kind {
        match field_rust_enum {
          Some(enum_ident) => {
            let rules = get_enum_rules(
              enum_ident,
              enum_descriptor,
              validation_data,
              enum_rules,
              static_defs,
            )?;
            rules_tokens.extend(rules);
          }
          None => error = Some("could not find enum field ident"),
        };
      } else {
        error = Some("could not find enum descriptor")
      }
    }
    RulesType::Duration(duration_rules) => {
      let rules = get_duration_rules(validation_data, duration_rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Timestamp(timestamp_rules) => {
      let rules = get_timestamp_rules(validation_data, timestamp_rules)?;
      rules_tokens.extend(rules);
    }
    RulesType::Any(any_rules) => {
      let rules = get_any_rules(validation_data, any_rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    RulesType::Bool(bool_rules) => {
      let rules = get_bool_rules(validation_data, bool_rules)?;
      rules_tokens.extend(rules);
    }
    RulesType::Bytes(bytes_rules) => {
      let rules = get_bytes_rules(validation_data, bytes_rules, static_defs)?;
      rules_tokens.extend(rules);
    }
    _ => {}
  };

  if let Some(err) = error {
    return Err(Error::new(field_span, format!("{} {}", error_prefix, err)));
  }

  Ok(rules_tokens)
}

pub fn get_field_kind(field_desc: &FieldDescriptor) -> FieldKind {
  let field_type = get_field_type(field_desc);

  if field_desc.is_list() {
    FieldKind::Repeated(field_type)
  } else if field_desc.is_map() {
    FieldKind::Map(field_type)
  } else {
    FieldKind::Single(field_type)
  }
}

pub fn get_field_type(field_desc: &FieldDescriptor) -> FieldType {
  match field_desc.kind() {
    ProstReflectKind::Message(message_desc) => match message_desc.full_name() {
      "google.protobuf.Duration" => FieldType::Duration,
      "google.protobuf.Timestamp" => FieldType::Timestamp,
      "google.protobuf.Any" => FieldType::Any,
      _ => FieldType::Message,
    },
    ProstReflectKind::Double => FieldType::Double,
    ProstReflectKind::Float => FieldType::Float,
    ProstReflectKind::Int32 => FieldType::Int32,
    ProstReflectKind::Int64 => FieldType::Int64,
    ProstReflectKind::Uint32 => FieldType::Uint32,
    ProstReflectKind::Uint64 => FieldType::Uint64,
    ProstReflectKind::Sint32 => FieldType::Sint32,
    ProstReflectKind::Sint64 => FieldType::Sint64,
    ProstReflectKind::Fixed32 => FieldType::Fixed32,
    ProstReflectKind::Fixed64 => FieldType::Fixed64,
    ProstReflectKind::Sfixed32 => FieldType::Sfixed32,
    ProstReflectKind::Sfixed64 => FieldType::Sfixed64,
    ProstReflectKind::Bool => FieldType::Bool,
    ProstReflectKind::String => FieldType::String,
    ProstReflectKind::Bytes => FieldType::Bytes,
    ProstReflectKind::Enum(_) => FieldType::Enum,
  }
}

pub fn convert_kind_to_proto_type(kind: ProstReflectKind) -> ProtoType {
  match kind {
    ProstReflectKind::Double => ProtoType::Double,
    ProstReflectKind::Float => ProtoType::Float,
    ProstReflectKind::Int32 => ProtoType::Int32,
    ProstReflectKind::Int64 => ProtoType::Int64,
    ProstReflectKind::Uint32 => ProtoType::Uint32,
    ProstReflectKind::Uint64 => ProtoType::Uint64,
    ProstReflectKind::Sint32 => ProtoType::Sint32,
    ProstReflectKind::Sint64 => ProtoType::Sint64,
    ProstReflectKind::Fixed32 => ProtoType::Fixed32,
    ProstReflectKind::Fixed64 => ProtoType::Fixed64,
    ProstReflectKind::Sfixed32 => ProtoType::Sfixed32,
    ProstReflectKind::Sfixed64 => ProtoType::Sfixed64,
    ProstReflectKind::Bool => ProtoType::Bool,
    ProstReflectKind::String => ProtoType::String,
    ProstReflectKind::Bytes => ProtoType::Bytes,
    ProstReflectKind::Message(_) => ProtoType::Message,
    ProstReflectKind::Enum(_) => ProtoType::Enum,
  }
}

pub(crate) fn invalid_lists_error<T>(
  field_span: Span,
  field_name: &str,
  invalid_items: &[T],
) -> Error
where
  T: Debug,
{
  Error::new(
    field_span,
    format!(
      "Error for field {}: the following values are contained by 'in' and 'not_in': {:?}",
      field_name, invalid_items
    ),
  )
}

pub fn get_plural_suffix(items: u64) -> &'static str {
  if items != 1 {
    "s"
  } else {
    ""
  }
}

pub fn get_field_error(field_name: &str, field_span: Span, error: &str) -> Error {
  Error::new(
    field_span,
    format!("Error for field {}: {}", field_name, error),
  )
}
