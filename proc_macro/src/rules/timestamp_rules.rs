use proto_types::{
  protovalidate::{
    timestamp_rules::{GreaterThan, LessThan},
    TimestampRules,
  },
  Timestamp,
};
use quote::{quote, ToTokens};
use syn::Error;

use super::{ValidatorKind, ValidatorTemplate};
use crate::{
  rules::comparable_rules::{validate_gt_lt, Gt, Lt},
  validation_data::ValidationData,
  validator_template::FieldValidator,
};

pub fn get_timestamp_rules(
  validation_data: &ValidationData,
  rules: &TimestampRules,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut templates: Vec<ValidatorTemplate> = Vec::new();

  let mut lt: Option<Lt<Timestamp>> = None;
  let mut gt: Option<Gt<Timestamp>> = None;

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

  if let Some(lt_rule) = rules.less_than {
    match lt_rule {
      LessThan::Lt(val) => {
        lt = Some(Lt { val, eq: false });
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
        lt = Some(Lt { val, eq: true });
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
          if let Some(ref gt_rule) = gt {
            if gt_rule.val.is_future() {
              return Err(Error::new(
                field_span,
                format!(
                  "{} gt or gte cannot be in the future if lt_now is true",
                  error_prefix
                ),
              ));
            }

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
      }
    };
  }

  if let Some(gt_rule) = rules.greater_than {
    match gt_rule {
      GreaterThan::Gt(val) => {
        gt = Some(Gt { val, eq: false });
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
        gt = Some(Gt { val, eq: true });
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
          if let Some(ref lt_rule) = lt {
            if lt_rule.val.is_past() {
              return Err(Error::new(
                field_span,
                format!(
                  "{} lt or lte cannot be in the past if gt_now is true",
                  error_prefix
                ),
              ));
            }

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
      }
    };
  }

  validate_gt_lt(&gt, &lt, &error_prefix, field_span)?;

  Ok(templates)
}
