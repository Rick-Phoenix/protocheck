use std::collections::HashSet;

use prost_reflect::EnumDescriptor;
use syn::Error;

use super::{protovalidate::EnumRules, FieldData, ValidatorKind, ValidatorTemplate};
use crate::Span2;

pub fn get_enum_rules(
  field_type_ident: String,
  field_span: Span2,
  enum_desc: &EnumDescriptor,
  field_data: &FieldData,
  enum_rules: &EnumRules,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut templates: Vec<ValidatorTemplate> = Vec::new();

  let mut enum_field_data = field_data.clone();
  let enum_name = enum_desc.full_name();
  enum_field_data.enum_full_name = Some(enum_name.to_string());

  if enum_rules.defined_only() {
    templates.push(ValidatorTemplate {
      field_data: field_data.clone(),
      kind: ValidatorKind::EnumDefinedOnly {
        enum_type_ident: field_type_ident.clone(),
        enum_name: enum_name.to_string(),
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
          field_span,
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
