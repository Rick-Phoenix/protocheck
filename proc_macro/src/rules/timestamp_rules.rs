use proto_types::protovalidate::{
  timestamp_rules::{GreaterThan, LessThan},
  TimestampRules,
};
use quote::{quote, ToTokens};
use syn::Error;

use super::{ValidatorKind, ValidatorTemplate};
use crate::{validation_data::ValidationData, validator_template::FieldValidator};

pub fn get_timestamp_rules(
  validation_data: &ValidationData,
  rules: &TimestampRules,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut templates: Vec<ValidatorTemplate> = Vec::new();

  let field_span = validation_data.field_span;

  let error_prefix = format!(
    "Error for field {}:",
    &validation_data.field_data.proto_name
  );

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
    return Ok(templates);
  }

  if let Some(within_rule) = rules.within {
    if within_rule.is_negative() {
      return Err(Error::new(
        field_span,
        format!("{} timestamp.within cannot be negative", error_prefix),
      ));
    }

    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::timestamps::within },
          target_value_tokens: within_rule.to_token_stream(),
        },
      },
    });
  }

  let (greater_than, less_than) = rules.comparable_rules(field_span, &error_prefix)?;

  if let Some(lt_rule) = less_than {
    match lt_rule {
      LessThan::Lt(val) => {
        templates.push(ValidatorTemplate {
          item_rust_name: validation_data.field_data.rust_name.clone(),
          kind: ValidatorKind::Field {
            validation_data: validation_data.clone(),
            field_validator: FieldValidator::Scalar {
              validator_path: quote! { protocheck::validators::comparables::lt },
              target_value_tokens: val.to_token_stream(),
            },
          },
        });
      }
      LessThan::Lte(val) => {
        templates.push(ValidatorTemplate {
          item_rust_name: validation_data.field_data.rust_name.clone(),
          kind: ValidatorKind::Field {
            validation_data: validation_data.clone(),
            field_validator: FieldValidator::Scalar {
              validator_path: quote! { protocheck::validators::comparables::lte },
              target_value_tokens: val.to_token_stream(),
            },
          },
        });
      }
      LessThan::LtNow(val) => {
        if val {
          templates.push(ValidatorTemplate {
            item_rust_name: validation_data.field_data.rust_name.clone(),
            kind: ValidatorKind::Field {
              validation_data: validation_data.clone(),
              field_validator: FieldValidator::Scalar {
                validator_path: quote! { protocheck::validators::timestamps::lt_now },
                target_value_tokens: quote! { () },
              },
            },
          });
        }
      }
    };
  }

  if let Some(gt_rule) = greater_than {
    match gt_rule {
      GreaterThan::Gt(val) => {
        templates.push(ValidatorTemplate {
          item_rust_name: validation_data.field_data.rust_name.clone(),
          kind: ValidatorKind::Field {
            validation_data: validation_data.clone(),
            field_validator: FieldValidator::Scalar {
              validator_path: quote! { protocheck::validators::comparables::gt },
              target_value_tokens: val.to_token_stream(),
            },
          },
        });
      }
      GreaterThan::Gte(val) => {
        templates.push(ValidatorTemplate {
          item_rust_name: validation_data.field_data.rust_name.clone(),
          kind: ValidatorKind::Field {
            validation_data: validation_data.clone(),
            field_validator: FieldValidator::Scalar {
              validator_path: quote! { protocheck::validators::comparables::gte },
              target_value_tokens: val.to_token_stream(),
            },
          },
        });
      }
      GreaterThan::GtNow(val) => {
        if val {
          templates.push(ValidatorTemplate {
            item_rust_name: validation_data.field_data.rust_name.clone(),
            kind: ValidatorKind::Field {
              validation_data: validation_data.clone(),
              field_validator: FieldValidator::Scalar {
                validator_path: quote! { protocheck::validators::timestamps::gt_now },
                target_value_tokens: quote! { () },
              },
            },
          });
        }
      }
    };
  }

  Ok(templates)
}
