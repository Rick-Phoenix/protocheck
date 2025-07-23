use std::collections::HashSet;

use proc_macro2::{Ident, Span};
use proto_types::{buf::validate::EnumRules, FieldData, GeneratedCodeKind, ValidatorCallTemplate};
use quote::quote;

pub fn get_enum_rules(
  field_data: FieldData,
  enum_rules: &EnumRules,
  enum_values: HashSet<i32>,
) -> Result<Vec<ValidatorCallTemplate>, Box<dyn std::error::Error>> {
  let mut templates: Vec<ValidatorCallTemplate> = Vec::new();
  let enum_full_name = field_data
    .enum_full_name
    .clone()
    .ok_or_else(|| "Enum field missing full enum name in FieldData")?;

  if enum_rules.defined_only() {
    let static_name_str = format!(
      "__VALID_{}_VALUES",
      enum_full_name.replace('.', "_").to_uppercase()
    );
    let enum_static_ident = Ident::new(&static_name_str, Span::call_site());

    templates.push(ValidatorCallTemplate {
      validator_path: Some(quote! { macro_impl::validators::enums::defined_only }),
      target_value_tokens: Some(
        quote! { &crate::__protobuf_validators_consts::#enum_static_ident },
      ),
      field_data,
      kind: GeneratedCodeKind::FieldRule,
    });
  }

  if enum_rules.r#in.len() > 0 {
    for n in enum_rules.r#in.iter() {
      let mut invalid_numbers: Vec<i32> = Vec::new();
      if !enum_values.contains(n) {
        invalid_numbers.push(*n);
      }
      if !invalid_numbers.is_empty() {
        return Err(Box::new(syn::Error::new(
          Span::call_site(),
          format!(
            "enum_rules.in contains values that are not in the {} enum: {:?}",
            enum_full_name, invalid_numbers
          ),
        )));
      }
    }
  }

  Ok(templates)
}
