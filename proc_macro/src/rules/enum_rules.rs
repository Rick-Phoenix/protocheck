use std::collections::HashSet;

use prost_reflect::EnumDescriptor;
use quote::{quote, ToTokens};
use syn::Error;

use super::{protovalidate::EnumRules, ValidatorKind, ValidatorTemplate};
use crate::{
  rules::containing_rules::validate_in_not_in, validation_data::ValidationData,
  validator_template::FieldValidator,
};

pub fn get_enum_rules(
  field_type_ident: String,
  enum_desc: &EnumDescriptor,
  validation_data: &ValidationData,
  enum_rules: &EnumRules,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut templates: Vec<ValidatorTemplate> = Vec::new();

  let enum_name = enum_desc.name();
  let error_prefix = format!("Error for field {}:", validation_data.full_name);

  let field_span = validation_data.field_span;

  if let Some(const_val) = enum_rules.r#const {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::constants::constant },
          target_value_tokens: const_val.to_token_stream(),
        },
      },
    });

    return Ok(templates);
  }

  if enum_rules.defined_only() {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::EnumDefinedOnly {
          enum_type_ident: field_type_ident.clone(),
          enum_name: enum_name.to_string(),
        },
      },
    });
  }

  validate_in_not_in(
    &enum_rules.r#in,
    &enum_rules.not_in,
    &error_prefix,
    field_span,
  )?;

  if !enum_rules.r#in.is_empty() {
    let enum_values: HashSet<i32> = enum_desc.values().map(|e| e.number()).collect();
    for n in enum_rules.r#in.iter() {
      let mut invalid_numbers: Vec<i32> = Vec::new();
      if !enum_values.contains(n) {
        invalid_numbers.push(*n);
      }
      if !invalid_numbers.is_empty() {
        return Err(syn::Error::new(
          field_span,
          format!(
            "{} enum_rules.in contains values that are not in the {} enum: {:?}",
            error_prefix, enum_name, invalid_numbers
          ),
        ));
      }
    }

    let in_list = enum_rules.r#in.clone();

    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::containing::in_list },
          target_value_tokens: quote! { vec![ #(#in_list),* ] },
        },
      },
    });
  }

  if !enum_rules.not_in.is_empty() {
    let not_in_list = enum_rules.not_in.clone();

    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::containing::not_in_list },
          target_value_tokens: quote! { vec![ #(#not_in_list),* ] },
        },
      },
    });
  }

  Ok(templates)
}
