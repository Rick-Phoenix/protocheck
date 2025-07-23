use proc_macro2::{Ident, Span};
use proto_types::{buf::validate::EnumRules, FieldData, GeneratedCodeKind, ValidatorCallTemplate};
use quote::quote;

pub fn get_enum_rules(
  field_data: FieldData,
  enum_rules: &EnumRules,
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

  Ok(templates)
}
