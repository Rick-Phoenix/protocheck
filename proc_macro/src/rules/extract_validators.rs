use std::{collections::HashMap, sync::LazyLock};

use prost_reflect::{
  prost::Message, Kind, MessageDescriptor, OneofDescriptor, Value as ProstValue,
};
use regex::Regex;
use syn::{DeriveInput, Error, Ident, LitStr, Token};

use super::{
  protovalidate::{FieldRules, Ignore},
  FieldData, GeneratedCodeKind, MessageRules, OneofRules, ProtoType, ValidatorCallTemplate,
};
use crate::{
  pool_loader::{
    get_rule_extensions_descriptors, FIELD_RULES_EXT_DESCRIPTOR, ONEOF_RULES_EXT_DESCRIPTOR,
  },
  rules::{
    cel_rules::get_cel_rules,
    core::{convert_kind_to_proto_type, get_field_rules},
  },
  Span2,
};

static MAP_ENUM_REGEX: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"^[^,]+,\s*enumeration\(([^)]+)\)$").expect("Failed to compile MAP_ENUM_REGEX")
});

#[derive(Clone, Debug)]
struct OneofField {
  pub ident: Ident,
  pub proto_name: String,
  pub enum_ident: Option<String>,
  pub span: Span2,
}

pub fn extract_oneof_validators(
  input_tokens: DeriveInput,
  oneof_desc: OneofDescriptor,
) -> Result<HashMap<Ident, Vec<ValidatorCallTemplate>>, Error> {
  let mut validators: HashMap<Ident, Vec<ValidatorCallTemplate>> = HashMap::new();
  let mut oneof_variants: HashMap<Ident, OneofField> = HashMap::new();

  let oneof_name = &oneof_desc.name();

  if let syn::Data::Enum(data_enum) = &input_tokens.data {
    for variant in &data_enum.variants {
      oneof_variants.insert(
        variant.ident.clone(),
        OneofField {
          ident: variant.ident.clone(),
          proto_name: String::new(),
          enum_ident: None,
          span: variant.ident.span(),
        },
      );

      for attr in &variant.attrs {
        if attr.path().is_ident("protocheck") {
          let _ = attr.parse_nested_meta(|meta| {
            if meta.input.peek(Token![=]) && meta.path.is_ident("proto_name") {
              let proto_field_name = meta.value().unwrap().parse::<LitStr>().unwrap();
              let field_ident_entry = oneof_variants.get_mut(&variant.ident).unwrap();
              field_ident_entry.proto_name = proto_field_name.value();
            }

            Ok(())
          });
        } else if attr.path().is_ident("prost") {
          let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("enumeration") && meta.input.peek(Token![=]) {
              let enum_name = meta.value().unwrap().parse::<LitStr>().unwrap();
              let field_ident_entry = oneof_variants.get_mut(&variant.ident).unwrap();
              field_ident_entry.enum_ident = Some(enum_name.value());
            }

            Ok(())
          });
        }
      }
    }
  }

  let mut fields_data: HashMap<String, OneofField> = HashMap::new();

  for data in oneof_variants.into_values() {
    fields_data.insert(data.proto_name.clone(), data);
  }

  if let ProstValue::Message(oneof_rules_msg) = oneof_desc
    .options()
    .get_extension(&ONEOF_RULES_EXT_DESCRIPTOR)
    .as_ref()
  {
    let oneof_rules = OneofRules::decode(oneof_rules_msg.encode_to_vec().as_slice()).unwrap();
    if oneof_rules.required() {
      let mut field_data = FieldData::default();
      field_data.rust_name = oneof_name.to_string();
      field_data.proto_name = oneof_name.to_string();
    }
  }

  for field in oneof_desc.fields() {
    let OneofField {
      enum_ident,
      ident: field_ident,
      span: field_span,
      ..
    } = fields_data.get(field.name()).cloned().ok_or(Error::new(
      Span2::call_site(),
      format!(
        "Could not process the data for field {} in oneof {}. Is proto_name set correctly?",
        field.name(),
        oneof_name
      ),
    ))?;

    let mut field_validators: Vec<ValidatorCallTemplate> = Vec::new();

    let field_options = field.options();
    let field_rules_descriptor = field_options.get_extension(&FIELD_RULES_EXT_DESCRIPTOR);

    if let ProstValue::Message(field_rules_message) = field_rules_descriptor.as_ref() {
      let field_rules = FieldRules::decode(field_rules_message.encode_to_vec().as_slice()).unwrap();

      let ignore = field_rules.ignore();
      let is_required = field_rules.required();

      if matches!(ignore, Ignore::Always) {
        continue;
      }

      let field_name = field.name();
      let field_kind = field.kind();

      let field_data = FieldData {
        rust_name: field_name.to_string(),
        proto_name: field_name.to_string(),
        tag: field.number(),
        proto_type: convert_kind_to_proto_type(&field_kind),
        is_required,
        is_repeated: false,
        is_repeated_item: false,
        is_map: false,
        is_map_key: false,
        is_map_value: false,
        key_type: None,
        value_type: None,
        enum_full_name: None,
        ignore,
        is_optional: true,
      };

      if let Kind::Message(field_message_type) = &field_kind {
        if !field_message_type
          .full_name()
          .starts_with("google.protobuf")
        {
          let template = ValidatorCallTemplate {
            validator_path: None,
            target_value_tokens: None,
            field_data,
            kind: GeneratedCodeKind::NestedMessageRecursion,
          };
          continue;
        }
      }

      if !field_rules.cel.is_empty() {
        field_validators.extend(get_cel_rules(&field_data, &field_rules.cel, false)?);
      }

      if let Some(ref rules_type) = field_rules.r#type {
        let rules = get_field_rules(enum_ident, field_span, &field, &field_data, rules_type)?;
        field_validators.extend(rules);
      }

      validators.insert(field_ident, field_validators);
    }
  }

  Ok(validators)
}

pub fn extract_message_validators(
  input_tokens: DeriveInput,
  message_desc: MessageDescriptor,
) -> Result<Vec<ValidatorCallTemplate>, Error> {
  let mut validation_data: Vec<ValidatorCallTemplate> = Vec::new();

  let mut rust_field_spans: HashMap<String, Span2> = HashMap::new();
  let mut rust_enum_paths: HashMap<String, String> = HashMap::new();

  if let syn::Data::Struct(syn::DataStruct { fields, .. }) = &input_tokens.data {
    for field in fields {
      if let Some(ident) = &field.ident {
        for attr in &field.attrs {
          if attr.path().is_ident("prost") {
            let _ = attr.parse_nested_meta(|meta| {
              if meta.input.peek(Token![=]) {
                if meta.path.is_ident("enumeration") {
                  let enum_name = meta.value().unwrap().parse::<LitStr>().unwrap();
                  rust_enum_paths.insert(ident.to_string(), enum_name.value());
                } else if meta.path.is_ident("map") {
                  let attr_content = meta.value().unwrap().parse::<LitStr>().unwrap().value();
                  if let Some(captures) = MAP_ENUM_REGEX.captures(&attr_content) {
                    if let Some(enum_name) = captures.get(1) {
                      rust_enum_paths.insert(ident.to_string(), enum_name.as_str().into());
                    }
                  }
                }
              }

              Ok(())
            });
          }
        }
        rust_field_spans.insert(ident.to_string(), ident.span());
      }
    }
  }

  let (field_ext_descriptor, message_ext_descriptor, oneof_ext_descriptor) =
    get_rule_extensions_descriptors(input_tokens)?;

  let message_options = message_desc.options();
  let message_rules_descriptor = message_options.get_extension(&message_ext_descriptor);

  // Message Rules
  if let ProstValue::Message(message_rules_msg) = message_rules_descriptor.as_ref() {
    let message_rules = MessageRules::decode(message_rules_msg.encode_to_vec().as_slice()).unwrap();

    if !message_rules.cel.is_empty() {
      let mut field_data = FieldData::default();
      field_data.rust_name = message_desc.name().to_string();
      field_data.proto_name = message_desc.name().to_string();
      field_data.tag = 0;
      field_data.proto_type = ProtoType::Message;
      validation_data.extend(get_cel_rules(&field_data, &message_rules.cel, true)?);
    }
  }

  // Oneof rules
  for oneof in message_desc.oneofs() {
    if let ProstValue::Message(oneof_rules_msg) = oneof
      .options()
      .get_extension(&oneof_ext_descriptor)
      .as_ref()
    {
      let oneof_rules = OneofRules::decode(oneof_rules_msg.encode_to_vec().as_slice()).unwrap();
      let name = oneof.name();
      let mut field_data = FieldData::default();
      field_data.rust_name = name.to_string();
      field_data.proto_name = name.to_string();

      validation_data.push(ValidatorCallTemplate {
        validator_path: None,
        target_value_tokens: None,
        field_data,
        kind: GeneratedCodeKind::OneofField {
          is_required: oneof_rules.required(),
        },
      });
    }
  }

  // Field Rules
  for field_desc in message_desc.fields() {
    if field_desc.containing_oneof().is_some() {
      continue;
    }
    let field_name = field_desc.name();
    let field_span = rust_field_spans
      .get(field_name)
      .cloned()
      .unwrap_or_else(Span2::call_site);

    let field_rust_enum = rust_enum_paths.get(field_name).cloned();

    let field_kind = field_desc.kind();
    let is_repeated = field_desc.is_list();
    let is_map = field_desc.is_map();
    let is_optional = field_desc.supports_presence();
    let field_tag = field_desc.number();

    let field_options = field_desc.options();
    let field_rules_descriptor = field_options.get_extension(&field_ext_descriptor);

    if let ProstValue::Message(field_rules_msg) = field_rules_descriptor.as_ref() {
      let field_rules = FieldRules::decode(field_rules_msg.encode_to_vec().as_slice()).unwrap();

      let ignore = field_rules.ignore();
      let is_required = field_rules.required();

      if matches!(ignore, Ignore::Always) {
        continue;
      }

      let mut field_data = FieldData {
        rust_name: field_name.to_string(),
        proto_name: field_name.to_string(),
        tag: field_tag,
        proto_type: convert_kind_to_proto_type(&field_kind),
        is_required,
        is_repeated,
        is_repeated_item: false,
        is_map,
        is_map_key: false,
        is_map_value: false,
        key_type: None,
        value_type: None,
        enum_full_name: None,
        ignore,
        is_optional,
      };

      if let Kind::Message(field_message_type) = field_desc.kind() {
        if !is_map
          && !field_message_type
            .full_name()
            .starts_with("google.protobuf")
        {
          field_data.is_repeated = false;
          field_data.is_repeated_item = is_repeated;

          let template = ValidatorCallTemplate {
            validator_path: None,
            target_value_tokens: None,
            field_data,
            kind: GeneratedCodeKind::NestedMessageRecursion,
          };
          validation_data.push(template);
          continue;
        }
      }

      if !field_rules.cel.is_empty() {
        validation_data.extend(get_cel_rules(&field_data, &field_rules.cel, false).unwrap());
      }

      if let Some(ref rules_type) = field_rules.r#type {
        let rules = get_field_rules(
          field_rust_enum,
          field_span,
          &field_desc,
          &field_data,
          rules_type,
        )?;

        validation_data.extend(rules);
      }
    }
  }

  Ok(validation_data)
}
