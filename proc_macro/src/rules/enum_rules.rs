use std::collections::HashSet;

use prost_reflect::EnumDescriptor;
use syn::{Error, Ident};

use super::{protovalidate::EnumRules, FieldData, GeneratedCodeKind, ValidatorCallTemplate};
use crate::Span2;

pub fn get_enum_rules(
  oneof_ident: Option<Ident>,
  field_type_ident: String,
  field_span: Span2,
  enum_desc: &EnumDescriptor,
  field_data: &FieldData,
  enum_rules: &EnumRules,
) -> Result<Vec<ValidatorCallTemplate>, Error> {
  let mut templates: Vec<ValidatorCallTemplate> = Vec::new();

  let mut enum_field_data = field_data.clone();
  let enum_name = enum_desc.full_name();
  enum_field_data.enum_full_name = Some(enum_name.to_string());

  let enum_values: HashSet<i32> = enum_desc.values().map(|e| e.number()).collect();

  if enum_rules.defined_only() {
    templates.push(ValidatorCallTemplate {
      validator_path: None,
      field_data: field_data.clone(),
      target_value_tokens: None,
      kind: GeneratedCodeKind::EnumDefinedOnly {
        enum_type_ident: field_type_ident.clone(),
        enum_name: enum_name.to_string(),
      },
      oneof_ident,
    });
  }

  if !enum_rules.r#in.is_empty() {
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
