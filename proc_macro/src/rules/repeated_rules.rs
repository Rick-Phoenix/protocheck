use prost_reflect::{FieldDescriptor, Kind};
use quote::{quote, ToTokens};
use syn::Error;

use super::{
  field_rules::Type as RulesType, protovalidate::Ignore, FieldData, ProtoType,
  ValidatorCallTemplate, ValidatorKind,
};
use crate::{
  rules::{
    cel_rules::{get_cel_rules, CelRuleKind},
    core::get_field_rules,
  },
  Span2,
};

pub fn get_repeated_rules(
  field_rust_enum: Option<String>,
  field_desc: &FieldDescriptor,
  field_span: Span2,
  field_data: &FieldData,
  field_rules: Option<&RulesType>,
) -> Result<Option<ValidatorCallTemplate>, Error> {
  let mut vec_level_rules: Vec<ValidatorCallTemplate> = Vec::new();
  let mut items_rules: Vec<ValidatorCallTemplate> = Vec::new();

  let mut item_is_message = false;
  if let Kind::Message(item_desc) = field_desc.kind() {
    if !item_desc.full_name().starts_with("google.protobuf") {
      item_is_message = true;
    }
  }

  let mut unique_values = false;
  let float_values = matches!(field_data.proto_type, ProtoType::Float)
    || matches!(field_data.proto_type, ProtoType::Double);

  if let Some(RulesType::Repeated(repeated_rules)) = field_rules {
    if repeated_rules.unique() {
      if item_is_message {
        return Err(syn::Error::new(
          field_span,
          "repeated.unique only works for scalar fields",
        ));
      }

      unique_values = true;
    }

    let mut min_items: Option<u64> = None;
    let mut max_items: Option<u64> = None;

    if repeated_rules.min_items() > 0 {
      let rule_val = repeated_rules.min_items();
      min_items = Some(rule_val);
      vec_level_rules.push(ValidatorCallTemplate {
        field_data: field_data.clone(),
        kind: ValidatorKind::FieldRule {
          validator_path: quote! { protocheck::validators::repeated::min_items },
          target_value_tokens: rule_val.to_token_stream(),
        },
      });
    }

    if repeated_rules.max_items() > 0 {
      let rule_val = repeated_rules.max_items();
      max_items = Some(rule_val);
      vec_level_rules.push(ValidatorCallTemplate {
        field_data: field_data.clone(),
        kind: ValidatorKind::FieldRule {
          validator_path: quote! { protocheck::validators::repeated::max_items },
          target_value_tokens: rule_val.to_token_stream(),
        },
      });
    }

    if min_items.is_some() && max_items.is_some() && min_items.unwrap() > max_items.unwrap() {
      return Err(syn::Error::new(
        field_span,
        "repeated.min_items cannot be larger than repeated.max_items",
      ));
    }

    if let Some(items_rules_descriptor) = repeated_rules.items.as_ref() {
      let ignore = items_rules_descriptor.ignore();
      if let Some(ref rules_type) = items_rules_descriptor.r#type {
        if !matches!(ignore, Ignore::Always) {
          let mut items_field_data = field_data.clone();
          items_field_data.ignore = ignore;
          items_field_data.is_repeated = false;
          items_field_data.is_repeated_item = true;
          items_field_data.is_required = items_rules_descriptor.required();

          if !items_rules_descriptor.cel.is_empty() {
            let cel_rules = get_cel_rules(
              &CelRuleKind::Field(field_desc),
              &items_field_data,
              &items_rules_descriptor.cel,
            )?;
            items_rules.extend(cel_rules);
          }

          if !item_is_message {
            let rules_for_single_item = get_field_rules(
              field_rust_enum,
              field_span,
              field_desc,
              &items_field_data,
              rules_type,
            )?;

            items_rules.extend(rules_for_single_item);
          }
        }
      }
    }
  }

  if item_is_message {
    let mut items_field_data = field_data.clone();
    items_field_data.is_repeated = false;
    items_field_data.is_repeated_item = true;

    let items_message_rules = ValidatorCallTemplate {
      field_data: items_field_data,
      kind: ValidatorKind::MessageField,
    };

    items_rules.push(items_message_rules);
  }

  if vec_level_rules.is_empty() && items_rules.is_empty() {
    Ok(None)
  } else {
    Ok(Some(ValidatorCallTemplate {
      field_data: field_data.clone(),
      kind: ValidatorKind::RepeatedValidationLoop {
        vec_level_rules,
        items_rules,
        unique_values,
        float_values,
      },
    }))
  }
}
