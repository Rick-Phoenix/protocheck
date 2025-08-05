use std::collections::HashMap;

use proc_macro2::{Ident as Ident2, TokenStream};
use prost_reflect::{
  prost::Message, FieldDescriptor, Kind, MessageDescriptor, OneofDescriptor, Value as ProstValue,
};
use protocheck_core::field_data::FieldKind;
use quote::{format_ident, quote};
use syn::{DeriveInput, Error, Ident};

use super::{
  protovalidate::{FieldRules, Ignore},
  FieldData, MessageRules, OneofRules, ValidatorKind, ValidatorTemplate,
};
use crate::{
  attribute_extractors::{extract_proto_name_attribute, ProstAttrData},
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
  Span2,
};

#[derive(Clone, Debug)]
struct OneofField {
  pub ident: Ident,
  pub proto_name: String,
  pub enum_ident: Option<String>,
  pub span: Span2,
}

#[derive(Debug)]
pub struct OneofValidatorsOutput {
  pub validators: HashMap<Ident, Vec<ValidatorTemplate>>,
  pub static_defs: Vec<TokenStream>,
}

pub fn extract_oneof_validators(
  input_tokens: &DeriveInput,
  oneof_desc: &OneofDescriptor,
) -> Result<OneofValidatorsOutput, Error> {
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
          attr.parse_nested_meta(|meta| {
            let proto_field_name =
              extract_proto_name_attribute(oneof_name, attr, &variant.ident, meta)?;
            let field_ident_entry = oneof_variants.get_mut(&variant.ident).unwrap();
            field_ident_entry.proto_name = proto_field_name;

            Ok(())
          })?;
        } else if attr.path().is_ident("prost") {
          match attr.parse_args::<ProstAttrData>() {
            Ok(parsed_data) => {
              if let Some(enum_name) = parsed_data.enum_path {
                let field_ident_entry = oneof_variants.get_mut(&variant.ident).unwrap();
                field_ident_entry.enum_ident = Some(enum_name);
              }
            }
            Err(e) => {
              return Err(Error::new_spanned(
                attr,
                format!(
                  "Could not extract the 'enumeration' attribute for variant {} in oneof {}: {}",
                  &variant.ident, oneof_name, e
                ),
              ))
            }
          };
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

      let item_rust_ident = Ident2::new(field.name(), Span2::call_site());
      let field_context_ident = format_ident!("field_context");
      let index_ident = format_ident!("idx");
      let item_ident = format_ident!("item");
      let key_ident = format_ident!("key");
      let map_value_ident = format_ident!("val");
      let violations_ident = format_ident!("violations");
      let parent_messages_ident = format_ident!("parent_messages");

      let validation_data = ValidationData {
        full_name: field.full_name().to_string(),
        is_required,
        is_in_oneof: true,
        is_optional: true,
        field_data,
        field_span,
        field_data_static_ident,
        key_type: None,
        value_type: None,
        violations_ident: violations_ident.clone(),
        field_context_ident,
        item_ident,
        parent_messages_ident: parent_messages_ident.clone(),
        key_ident,
        map_value_ident,
        index_ident,
        item_rust_ident: item_rust_ident.clone(),
        field_kind: FieldKind::from_field_desc(&field),
      };

      if !field_rules.cel.is_empty() {
        field_validators.extend(get_cel_rules(
          &CelRuleTemplateTarget::Field {
            field_desc: &field,
            is_boxed: field_is_boxed(&field, oneof_desc.parent_message()),
            validation_data: &validation_data,
          },
          &field_rules.cel,
          &mut static_defs,
        )?);
      }

      if let Some(ref rules_type) = field_rules.r#type {
        let rules = get_field_rules(
          &mut static_defs,
          enum_ident,
          &field,
          &validation_data,
          rules_type,
        )?;
        field_validators.extend(rules);
      }

      if let Kind::Message(field_message_type) = &field_kind {
        if !field_message_type
          .full_name()
          .starts_with("google.protobuf")
        {
          let validator_tokens = validation_data.get_message_field_validator_tokens();

          let template = ValidatorTemplate {
            kind: ValidatorKind::PureTokens(validator_tokens),
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

  Ok(OneofValidatorsOutput {
    validators,
    static_defs,
  })
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
            match attr.parse_args::<ProstAttrData>() {
              Ok(parsed_data) => {
                if let Some(enum_name) = parsed_data.enum_path {
                  rust_enum_paths.insert(ident.to_string(), enum_name.as_str().into());
                }
              }
              Err(e) => {
                return Err(Error::new_spanned(
                  attr,
                  format!(
                    "Could not extract the 'enumeration' attribute for field {} in struct {}: {}",
                    ident, input_tokens.ident, e
                  ),
                ))
              }
            };
          }
        }
        rust_field_spans.insert(ident.to_string(), ident.span());
      }
    }
  }

  let message_options = message_desc.options();
  let message_rules_descriptor = message_options.get_extension(&MESSAGE_RULES_EXT_DESCRIPTOR);

  let violations_ident = format_ident!("violations");
  let parent_messages_ident = format_ident!("parent_messages");

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
        &CelRuleTemplateTarget::Message {
          message_desc,
          parent_messages_ident: &parent_messages_ident,
          violations_ident: &violations_ident,
        },
        &message_rules.cel,
        &mut static_defs,
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
      let oneof_rules =
        OneofRules::decode(oneof_rules_msg.encode_to_vec().as_slice()).map_err(|e| {
          Error::new_spanned(input_tokens, format!("Could not decode oneof rules: {}", e))
        })?;

      let item_rust_ident = Ident2::new(oneof.name(), Span2::call_site());
      validators.push(ValidatorTemplate {
        kind: ValidatorKind::Oneof {
          is_required: oneof_rules.required(),
          field_name: oneof.name().to_string(),
          parent_messages_ident: parent_messages_ident.clone(),
          violations_ident: violations_ident.clone(),
          item_rust_ident,
        },
      });
    }
  }

  // Field Rules
  for field in message_desc.fields() {
    if let Some(containing_oneof) = field.containing_oneof().as_ref() {
      if !containing_oneof.is_synthetic() {
        continue;
      }
    }

    let mut field_validators: Vec<ValidatorTemplate> = Vec::new();

    let item_rust_ident = Ident2::new(field.name(), Span2::call_site());
    let field_context_ident = format_ident!("field_context");
    let index_ident = format_ident!("idx");
    let item_ident = format_ident!("item");
    let key_ident = format_ident!("key");
    let value_ident = format_ident!("val");

    let field_name = field.name();
    let field_span = rust_field_spans
      .get(field_name)
      .cloned()
      .unwrap_or_else(Span2::call_site);

    let field_rust_enum = rust_enum_paths.get(field_name).cloned();

    let field_kind = field.kind();
    let is_repeated = field.is_list();
    let is_map = field.is_map();
    let is_optional = field.supports_presence();
    let field_tag = field.number();

    let field_options = field.options();
    let field_rules_descriptor = field_options.get_extension(&FIELD_RULES_EXT_DESCRIPTOR);

    if let ProstValue::Message(field_rules_msg) = field_rules_descriptor.as_ref() {
      let field_rules =
        FieldRules::decode(field_rules_msg.encode_to_vec().as_slice()).map_err(|e| {
          Error::new_spanned(input_tokens, format!("Could not decode field rules: {}", e))
        })?;

      let ignore = field_rules.ignore();
      let is_required = field_rules.required() && field.supports_presence();

      if matches!(ignore, Ignore::Always) {
        continue;
      }

      let field_data = FieldData {
        rust_name: field_name.to_string(),
        proto_name: field_name.to_string(),
        tag: field_tag,
        proto_type: convert_kind_to_proto_type(&field_kind),
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
        full_name: field.full_name().to_string(),
        is_required,
        is_in_oneof: false,
        is_optional,
        field_data,
        field_span,
        field_data_static_ident,
        key_type: None,
        value_type: None,
        violations_ident: violations_ident.clone(),
        parent_messages_ident: parent_messages_ident.clone(),
        item_rust_ident: item_rust_ident.clone(),
        index_ident,
        map_value_ident: value_ident,
        key_ident,
        item_ident,
        field_context_ident,
        field_kind: FieldKind::from_field_desc(&field),
      };

      if !field_rules.cel.is_empty() {
        field_validators.extend(get_cel_rules(
          &CelRuleTemplateTarget::Field {
            field_desc: &field,
            is_boxed: field_is_boxed(&field, message_desc),
            validation_data: &validation_data,
          },
          &field_rules.cel,
          &mut static_defs,
        )?);
      }

      let field_rules_type = field_rules.r#type.as_ref();
      if is_repeated {
        let repeated_rules = get_repeated_rules(
          &validation_data,
          &mut static_defs,
          field_rust_enum,
          &field,
          field_rules_type,
        )?;

        if let Some(rules) = repeated_rules {
          field_validators.push(rules);
        }
      } else if is_map {
        let map_rules = get_map_rules(
          validation_data,
          &mut static_defs,
          field_rust_enum,
          &field,
          field_rules_type,
        )?;

        if let Some(rules) = map_rules {
          field_validators.push(rules);
        }
      } else if let Some(rules_type) = field_rules_type {
        let rules = get_field_rules(
          &mut static_defs,
          field_rust_enum,
          &field,
          &validation_data,
          rules_type,
        )?;

        field_validators.extend(rules);
      } else if let Kind::Message(field_message_desc) = field.kind() {
        if !field_message_desc
          .full_name()
          .starts_with("google.protobuf")
        {
          let validator_tokens = validation_data.get_message_field_validator_tokens();

          let template = ValidatorTemplate {
            kind: ValidatorKind::PureTokens(validator_tokens),
          };
          field_validators.push(template);
        }
      } else if is_required {
        field_validators.push(ValidatorTemplate {
          kind: ValidatorKind::PureTokens(validation_data.get_required_only_validator()),
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

pub fn field_is_boxed(field_desc: &FieldDescriptor, message_desc: &MessageDescriptor) -> bool {
  if let Kind::Message(field_message_desc) = field_desc.kind() {
    return field_message_desc.full_name() == message_desc.full_name();
  }
  false
}
