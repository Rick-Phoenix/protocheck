use std::{collections::HashMap, sync::LazyLock};

use prost_reflect::{prost::Message, Kind, MessageDescriptor, Value as ProstValue};
use regex::Regex;
use syn::{DeriveInput, Error, LitStr, Token};

use super::{
  protovalidate::{FieldRules, Ignore},
  FieldData, GeneratedCodeKind, MessageRules, OneofRules, ProtoType, ValidatorCallTemplate,
};
use crate::{
  pool_loader::get_rule_extensions_descriptors,
  rules::{
    cel_rules::get_cel_rules,
    core::{convert_kind_to_proto_type, get_field_rules},
  },
  Span2,
};

static MAP_ENUM_REGEX: LazyLock<Regex> = LazyLock::new(|| {
  // This regex matches:
  // ^          - start of the string
  // [^,]+,     - matches the key type (e.g., "string"), followed by a comma
  // \s*        - optional whitespace
  // enumeration\( - matches "enumeration(" literally
  // (            - START of capturing group 1
  //   [^)]+      - matches one or more characters that are NOT a closing parenthesis
  // )            - END of capturing group 1
  // \)           - matches the closing parenthesis literally
  // $          - end of the string
  Regex::new(r"^[^,]+,\s*enumeration\(([^)]+)\)$").expect("Failed to compile MAP_ENUM_REGEX")
});

pub fn extract_validators(
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
    // println!("{:?}", oneof.name());
    if let ProstValue::Message(oneof_rules_msg) = oneof
      .options()
      .get_extension(&oneof_ext_descriptor)
      .as_ref()
    {
      let oneof_rules = OneofRules::decode(oneof_rules_msg.encode_to_vec().as_slice()).unwrap();
      if oneof_rules.required() {
        let name = oneof.name();
        let mut field_data = FieldData::default();
        field_data.rust_name = name.to_string();
        field_data.proto_name = name.to_string();

        validation_data.push(ValidatorCallTemplate {
          validator_path: None,
          target_value_tokens: None,
          field_data,
          kind: GeneratedCodeKind::OneofRule,
        });
      }
    }
  }

  // Field Rules
  for field_desc in message_desc.fields() {
    if field_desc.containing_oneof().is_some() {
      continue;
    }
    let field_name = field_desc.name();
    // println!("{}", field_name.to_string());
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
