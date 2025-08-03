use proto_types::protovalidate::AnyRules;
use quote::quote;
use syn::Error;

use super::{ValidatorKind, ValidatorTemplate};
use crate::{
  rules::containing_rules::validate_in_not_in, validation_data::ValidationData,
  validator_template::FieldValidator,
};

pub fn get_any_rules(
  validation_data: &ValidationData,
  rules: &AnyRules,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut templates: Vec<ValidatorTemplate> = Vec::new();

  let field_span = validation_data.field_span;

  let error_prefix = format!(
    "Error for field {}:",
    &validation_data.field_data.proto_name
  );

  validate_in_not_in(&rules.r#in, &rules.not_in, &error_prefix, field_span)?;

  if !rules.r#in.is_empty() {
    let in_list = rules.r#in.clone();
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

  Ok(templates)
}
