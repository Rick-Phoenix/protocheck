use std::collections::HashSet;

use prost_reflect::EnumDescriptor;
use syn::Error;

use super::{protovalidate::EnumRules, ValidatorKind, ValidatorTemplate};
use crate::{validation_data::ValidationData, validator_template::FieldValidator};

pub fn get_enum_rules(
  field_type_ident: String,
  enum_desc: &EnumDescriptor,
  validation_data: &ValidationData,
  enum_rules: &EnumRules,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut templates: Vec<ValidatorTemplate> = Vec::new();

  let enum_name = enum_desc.name();

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

  if !enum_rules.r#in.is_empty() {
    let enum_values: HashSet<i32> = enum_desc.values().map(|e| e.number()).collect();
    for n in enum_rules.r#in.iter() {
      let mut invalid_numbers: Vec<i32> = Vec::new();
      if !enum_values.contains(n) {
        invalid_numbers.push(*n);
      }
      if !invalid_numbers.is_empty() {
        return Err(syn::Error::new(
          validation_data.field_span.clone(),
          format!(
            "enum_rules.in contains values that are not in the {} enum: {:?}",
            enum_name, invalid_numbers
          ),
        ));
      }
    }
  }

  Ok(templates)
}
