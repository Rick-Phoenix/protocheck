use proto_types::protovalidate::BoolRules;
use quote::ToTokens;
use syn::Error;

use super::{ValidatorKind, ValidatorTemplate};
use crate::validation_data::ValidationData;

pub fn get_bool_rules(
  validation_data: &ValidationData,
  rules: &BoolRules,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut templates: Vec<ValidatorTemplate> = Vec::new();

  if let Some(const_val) = rules.r#const {
    templates.push(ValidatorTemplate {
      kind: ValidatorKind::PureTokens(
        validation_data.get_constant_validator(const_val.to_token_stream()),
      ),
    });

    return Ok(templates);
  }

  Ok(templates)
}
