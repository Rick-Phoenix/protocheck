use proc_macro2::TokenStream;
use prost_reflect::{FieldDescriptor, Kind};
use protocheck_core::field_data::FieldKind;
use quote::{quote, ToTokens};
use syn::Error;

use super::{
  field_rules::Type as RulesType, protovalidate::Ignore, ProtoType, ValidatorKind,
  ValidatorTemplate,
};
use crate::{
  cel_rule_template::CelRuleTemplateTarget,
  rules::{cel_rules::get_cel_rules, core::get_field_rules},
  validation_data::ValidationData,
  validator_template::FieldValidator,
};

pub fn get_repeated_rules(
  validation_data: &ValidationData,
  static_defs: &mut Vec<TokenStream>,
  field_rust_enum: Option<String>,
  field_desc: &FieldDescriptor,
  field_rules: Option<&RulesType>,
) -> Result<Option<ValidatorTemplate>, Error> {
  let mut vec_level_rules: Vec<ValidatorTemplate> = Vec::new();
  let mut items_rules: Vec<ValidatorTemplate> = Vec::new();

  let field_span = validation_data.field_span;

  let mut item_is_message = false;
  if let Kind::Message(item_desc) = field_desc.kind() {
    if !item_desc.full_name().starts_with("google.protobuf") {
      item_is_message = true;
    }
  }

  let error_prefix = format!("Error for field {}:", validation_data.full_name);

  let mut unique_values = false;
  let float_values = matches!(validation_data.field_data.proto_type, ProtoType::Float)
    || matches!(validation_data.field_data.proto_type, ProtoType::Double);
  let mut ignore_items_validators = false;

  if let Some(RulesType::Repeated(repeated_rules)) = field_rules {
    if repeated_rules.unique() {
      if item_is_message {
        return Err(syn::Error::new(
          field_span,
          format!(
            "{} repeated.unique only works for scalar fields",
            error_prefix
          ),
        ));
      }

      unique_values = true;
    }

    let mut min_items: Option<u64> = None;
    let mut max_items: Option<u64> = None;

    if repeated_rules.min_items() > 0 {
      let rule_val = repeated_rules.min_items();
      min_items = Some(rule_val);
      vec_level_rules.push(ValidatorTemplate {
        item_rust_name: validation_data.field_data.rust_name.clone(),
        kind: ValidatorKind::Field {
          validation_data: validation_data.clone(),
          field_validator: FieldValidator::Scalar {
            validator_path: quote! { protocheck::validators::repeated::min_items },
            target_value_tokens: rule_val.to_token_stream(),
          },
        },
      });
    }

    if repeated_rules.max_items() > 0 {
      let rule_val = repeated_rules.max_items();
      max_items = Some(rule_val);
      vec_level_rules.push(ValidatorTemplate {
        item_rust_name: validation_data.field_data.rust_name.clone(),
        kind: ValidatorKind::Field {
          validation_data: validation_data.clone(),
          field_validator: FieldValidator::Scalar {
            validator_path: quote! { protocheck::validators::repeated::max_items },
            target_value_tokens: rule_val.to_token_stream(),
          },
        },
      });
    }

    if min_items.is_some() && max_items.is_some() && min_items.unwrap() > max_items.unwrap() {
      return Err(syn::Error::new(
        field_span,
        format!(
          "{} repeated.min_items cannot be larger than repeated.max_items",
          error_prefix
        ),
      ));
    }

    if let Some(items_rules_descriptor) = repeated_rules.items.as_ref() {
      let ignore = items_rules_descriptor.ignore();
      if let Some(ref rules_type) = items_rules_descriptor.r#type {
        if matches!(ignore, Ignore::Always) {
          ignore_items_validators = true
        } else {
          let mut items_validation_data = validation_data.clone();
          items_validation_data.field_data.kind = FieldKind::RepeatedItem;

          if !item_is_message {
            let rules_for_single_item = get_field_rules(
              field_rust_enum,
              field_desc,
              &items_validation_data,
              rules_type,
            )?;

            items_rules.extend(rules_for_single_item);
          }

          if !items_rules_descriptor.cel.is_empty() {
            let cel_rules = get_cel_rules(
              &CelRuleTemplateTarget::Field(field_desc.clone(), items_validation_data),
              &items_rules_descriptor.cel,
              static_defs,
            )?;
            items_rules.extend(cel_rules);
          }
        }
      }
    }
  }

  if item_is_message && !ignore_items_validators {
    let mut message_items_validation_data = validation_data.clone();
    message_items_validation_data.field_data.kind = FieldKind::RepeatedItem;

    let message_items_validator = ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: message_items_validation_data,
        field_validator: FieldValidator::MessageField,
      },
    };

    items_rules.push(message_items_validator);
  }

  if vec_level_rules.is_empty() && items_rules.is_empty() {
    Ok(None)
  } else {
    Ok(Some(ValidatorTemplate {
      item_rust_name: field_desc.name().to_string(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Repeated {
          vec_level_rules,
          items_rules,
          unique_values,
          float_values,
        },
      },
    }))
  }
}
