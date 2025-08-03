use proto_types::protovalidate::BoolRules;
use quote::{quote, ToTokens};
use syn::Error;

use super::{ValidatorKind, ValidatorTemplate};
use crate::{validation_data::ValidationData, validator_template::FieldValidator};

pub fn get_bool_rules(
  validation_data: &ValidationData,
  rules: &BoolRules,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut templates: Vec<ValidatorTemplate> = Vec::new();

  if let Some(const_val) = rules.r#const {
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
  }

  Ok(templates)
}
