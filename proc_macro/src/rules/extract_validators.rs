use std::{collections::HashMap, sync::LazyLock};

use proc_macro2::{Ident as Ident2, TokenStream};
use prost_reflect::{
  prost::Message, Kind, MessageDescriptor, OneofDescriptor, Value as ProstValue,
};
use protocheck_core::field_data::FieldKind;
use quote::quote;
use regex::Regex;
use syn::{DeriveInput, Error, Ident, LitStr, Token};

use super::{
  protovalidate::{FieldRules, Ignore},
  FieldData, MessageRules, OneofRules, ValidatorKind, ValidatorTemplate,
};
use crate::{
  cel_rule_template::CelRuleTemplateTarget,
  pool_loader::{
    FIELD_RULES_EXT_DESCRIPTOR, MESSAGE_RULES_EXT_DESCRIPTOR, ONEOF_RULES_EXT_DESCRIPTOR,
  },
  rules::{
    cel_rules::get_cel_rules,
    core::{convert_kind_to_proto_type, get_field_rules},
    map_rules::get_map_rules,
    repeated_rules::get_repeated_rules,
  },
  validation_data::ValidationData,
  validator_template::FieldValidator,
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
  input_tokens: &DeriveInput,
  oneof_desc: &OneofDescriptor,
) -> Result<(HashMap<Ident, Vec<ValidatorTemplate>>, Vec<TokenStream>), Error> {
  let mut validators: HashMap<Ident, Vec<ValidatorTemplate>> = HashMap::new();
  let mut oneof_variants: HashMap<Ident, OneofField> = HashMap::new();
  let mut static_defs: Vec<TokenStream> = Vec::new();

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
              let proto_field_name = meta
                .value()
                .map_err(|e| {
                  Error::new_spanned(attr, format!("Failed to parse the proto_name attribute's value tokens: {}", e))
                })?
                .parse::<LitStr>()
                .map_err(|e| {
                  Error::new_spanned(
                    attr,
                    format!("Failed to parse the value for the proto_name attribute into a string literal: {}", e),
                  )
                })?;
              let field_ident_entry = oneof_variants.get_mut(&variant.ident).unwrap();
              field_ident_entry.proto_name = proto_field_name.value();
            }

            Ok(())
          });
        } else if attr.path().is_ident("prost") {
          let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("enumeration") && meta.input.peek(Token![=]) {
              let enum_name = meta.value().map_err(|e| {
                Error::new_spanned(attr, format!("Failed to parse the enumeration attribute's value tokens: {}", e))
              })?.parse::<LitStr>().map_err(|e| {
                Error::new_spanned(attr, format!("Failed to parse the value for the enumeration attribute to a string literal: {}", e))
              })?;
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

    let mut field_validators: Vec<ValidatorTemplate> = Vec::new();

    let field_options = field.options();
    let field_rules_descriptor = field_options.get_extension(&FIELD_RULES_EXT_DESCRIPTOR);

    if let ProstValue::Message(field_rules_message) = field_rules_descriptor.as_ref() {
      let field_rules = FieldRules::decode(field_rules_message.encode_to_vec().as_slice())
        .map_err(|e| {
          Error::new_spanned(input_tokens, format!("Could not decode field rules: {}", e))
        })?;

      let ignore = field_rules.ignore();
      let is_required = field_rules.required() && field.supports_presence();

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
        kind: FieldKind::from(&field),
        key_type: None,
        value_type: None,
        ignore,
      };

      let field_data_static_ident = Ident2::new(
        &format!(
          "__VALIDATOR_FIELD_DATA_{}",
          field.full_name().replace(".", "_").to_uppercase()
        ),
        Span2::call_site(),
      );

      let field_data_static_tokens = quote! {
        static #field_data_static_ident: std::sync::LazyLock<protocheck::field_data::FieldData> = std::sync::LazyLock::new(|| {
          #field_data
        });
      };

      let validation_data = ValidationData {
        is_required,
        is_in_oneof: true,
        is_optional: true,
        field_data,
        field_span,
        field_data_static_ident,
      };

      if !field_rules.cel.is_empty() {
        field_validators.extend(get_cel_rules(
          &CelRuleTemplateTarget::Field(field.clone(), validation_data.clone()),
          &field_rules.cel,
          &mut static_defs,
        )?);
      }

      if let Some(ref rules_type) = field_rules.r#type {
        let rules = get_field_rules(enum_ident, &field, &validation_data, rules_type)?;
        field_validators.extend(rules);
      }

      if let Kind::Message(field_message_type) = &field_kind {
        if !field_message_type
          .full_name()
          .starts_with("google.protobuf")
        {
          let template = ValidatorTemplate {
            item_rust_name: field.name().to_string(),
            kind: ValidatorKind::Field {
              validation_data,
              field_validator: FieldValidator::MessageField,
            },
          };
          field_validators.push(template);
        }
      }

      if !field_validators.is_empty() {
        static_defs.push(field_data_static_tokens);
      }

      validators.insert(field_ident, field_validators);
    }
  }

  Ok((validators, static_defs))
}

pub fn extract_message_validators(
  input_tokens: &DeriveInput,
  message_desc: &MessageDescriptor,
) -> Result<(Vec<ValidatorTemplate>, Vec<TokenStream>), Error> {
  let mut validators: Vec<ValidatorTemplate> = Vec::new();
  let mut static_defs: Vec<TokenStream> = Vec::new();

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
                  let enum_name = meta
                    .value()
                    .map_err(|e| {
                      Error::new_spanned(
                        attr,
                        format!(
                          "Failed to parse the enumeration attribute's value tokens: {}",
                          e
                        ),
                      )
                    })?
                    .parse::<LitStr>()
                    .map_err(|e| {
                      Error::new_spanned(
                        attr,
                        format!(
                          "Failed to parse the enumeration attribute to a string literal: {}",
                          e
                        ),
                      )
                    })?;
                  rust_enum_paths.insert(ident.to_string(), enum_name.value());
                } else if meta.path.is_ident("map") {
                  let attr_content = meta
                    .value()
                    .map_err(|e| {
                      Error::new_spanned(
                        attr,
                        format!("Failed to parse the map attribute's value tokens: {}", e),
                      )
                    })?
                    .parse::<LitStr>()
                    .map_err(|e| {
                      Error::new_spanned(
                        attr,
                        format!("Failed to parse map attribute to a string literal: {}", e),
                      )
                    })?
                    .value();
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

  let message_options = message_desc.options();
  let message_rules_descriptor = message_options.get_extension(&MESSAGE_RULES_EXT_DESCRIPTOR);

  // Message Rules
  if let ProstValue::Message(message_rules_msg) = message_rules_descriptor.as_ref() {
    let message_rules = MessageRules::decode(message_rules_msg.encode_to_vec().as_slice())
      .map_err(|e| {
        Error::new_spanned(
          input_tokens,
          format!("Could not decode message rules: {}", e),
        )
      })?;

    if !message_rules.cel.is_empty() {
      validators.extend(get_cel_rules(
        &CelRuleTemplateTarget::Message(message_desc.clone()),
        &message_rules.cel,
        &mut static_defs,
      )?);
    }
  }

  // Oneof rules
  for oneof in message_desc.oneofs() {
    if oneof_is_synthetic(&oneof) {
      continue;
    }

    if let ProstValue::Message(oneof_rules_msg) = oneof
      .options()
      .get_extension(&ONEOF_RULES_EXT_DESCRIPTOR)
      .as_ref()
    {
      let oneof_rules =
        OneofRules::decode(oneof_rules_msg.encode_to_vec().as_slice()).map_err(|e| {
          Error::new_spanned(input_tokens, format!("Could not decode oneof rules: {}", e))
        })?;

      validators.push(ValidatorTemplate {
        item_rust_name: oneof.name().to_string(),
        kind: ValidatorKind::Oneof {
          is_required: oneof_rules.required(),
        },
      });
    }
  }

  // Field Rules
  for field_desc in message_desc.fields() {
    if let Some(containing_oneof) = field_desc.containing_oneof().as_ref() {
      if !oneof_is_synthetic(containing_oneof) {
        continue;
      }
    }

    let mut field_validators: Vec<ValidatorTemplate> = Vec::new();

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
    let field_rules_descriptor = field_options.get_extension(&FIELD_RULES_EXT_DESCRIPTOR);

    if let ProstValue::Message(field_rules_msg) = field_rules_descriptor.as_ref() {
      let field_rules =
        FieldRules::decode(field_rules_msg.encode_to_vec().as_slice()).map_err(|e| {
          Error::new_spanned(input_tokens, format!("Could not decode field rules: {}", e))
        })?;

      let ignore = field_rules.ignore();
      let is_required = field_rules.required() && field_desc.supports_presence();

      if matches!(ignore, Ignore::Always) {
        continue;
      }

      let field_data = FieldData {
        rust_name: field_name.to_string(),
        proto_name: field_name.to_string(),
        tag: field_tag,
        proto_type: convert_kind_to_proto_type(&field_kind),
        kind: FieldKind::from(&field_desc),
        key_type: None,
        value_type: None,
        ignore,
      };

      let field_data_static_ident = Ident2::new(
        &format!(
          "__VALIDATOR_FIELD_DATA_{}",
          field_desc.full_name().replace(".", "_").to_uppercase()
        ),
        Span2::call_site(),
      );

      let field_data_static_tokens = quote! {
        static #field_data_static_ident: std::sync::LazyLock<protocheck::field_data::FieldData> = std::sync::LazyLock::new(|| {
          #field_data
        });
      };

      let validation_data = ValidationData {
        is_required,
        is_in_oneof: false,
        is_optional,
        field_data,
        field_span,
        field_data_static_ident,
      };

      let field_rules_type = field_rules.r#type.as_ref();

      if is_repeated {
        let repeated_rules = get_repeated_rules(
          &validation_data,
          &mut static_defs,
          field_rust_enum,
          &field_desc,
          field_rules_type,
        )?;

        if let Some(rules) = repeated_rules {
          field_validators.push(rules);
        }
      } else if is_map {
        let map_rules = get_map_rules(
          &validation_data,
          &mut static_defs,
          field_rust_enum,
          &field_desc,
          field_rules_type,
        )?;

        if let Some(rules) = map_rules {
          field_validators.push(rules);
        }
      } else if let Kind::Message(field_message_desc) = field_desc.kind() {
        if !field_rules.cel.is_empty() {
          field_validators.extend(get_cel_rules(
            &CelRuleTemplateTarget::Field(field_desc.clone(), validation_data.clone()),
            &field_rules.cel,
            &mut static_defs,
          )?);
        }

        if !field_message_desc
          .full_name()
          .starts_with("google.protobuf")
        {
          let template = ValidatorTemplate {
            item_rust_name: field_desc.name().to_string(),
            kind: ValidatorKind::Field {
              validation_data,
              field_validator: FieldValidator::MessageField,
            },
          };
          field_validators.push(template);
        }
      } else if let Some(rules_type) = field_rules_type {
        let rules = get_field_rules(field_rust_enum, &field_desc, &validation_data, rules_type)?;

        field_validators.extend(rules);
      } else if is_required {
        field_validators.push(ValidatorTemplate {
          item_rust_name: validation_data.field_data.rust_name.to_string(),
          kind: ValidatorKind::Field {
            validation_data,
            field_validator: FieldValidator::Required,
          },
        });
      }

      if !field_validators.is_empty() {
        static_defs.push(field_data_static_tokens);
        validators.extend(field_validators);
      }
    }
  }

  Ok((validators, static_defs))
}

fn oneof_is_synthetic(oneof: &OneofDescriptor) -> bool {
  let is_synthetic_optional_field = oneof.name().starts_with('_')
    && oneof.fields().count() == 1
    && oneof.name() == format!("_{}", oneof.fields().next().unwrap().name());
  is_synthetic_optional_field
}
