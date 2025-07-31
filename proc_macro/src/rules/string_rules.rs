use quote::{quote, ToTokens};
use syn::Error;

use super::{protovalidate::StringRules, ValidatorKind, ValidatorTemplate};
use crate::{
  rules::{comparable_rules::validate_len, containing_rules::validate_in_not_in},
  validation_data::ValidationData,
  validator_template::FieldValidator,
};

pub fn get_string_rules(
  validation_data: &ValidationData,
  string_rules: &StringRules,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut templates: Vec<ValidatorTemplate> = Vec::new();

  let field_span = validation_data.field_span;
  let error_prefix = format!("Error for field {}:", validation_data.full_name);

  if let Some(len_value) = string_rules.len {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::strings::len },
          target_value_tokens: len_value.into_token_stream(),
        },
      },
    });
  }

  if let Some(min_len_value) = string_rules.min_len {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::strings::min_len },
          target_value_tokens: min_len_value.into_token_stream(),
        },
      },
    });
  }

  if let Some(max_len_value) = string_rules.max_len {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::strings::max_len },
          target_value_tokens: max_len_value.into_token_stream(),
        },
      },
    });
  }

  validate_in_not_in(
    &string_rules.r#in,
    &string_rules.not_in,
    &error_prefix,
    field_span,
  )?;

  validate_len(
    string_rules.len,
    string_rules.min_len,
    string_rules.max_len,
    &error_prefix,
    false,
    field_span,
  )?;
  validate_len(
    string_rules.len_bytes,
    string_rules.min_bytes,
    string_rules.max_bytes,
    &error_prefix,
    true,
    field_span,
  )?;

  Ok(templates)
}
