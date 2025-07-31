use proto_types::protovalidate::int64_rules::{GreaterThan, LessThan};
use quote::{quote, ToTokens};
use syn::Error;

use super::{protovalidate::Int64Rules, ValidatorKind, ValidatorTemplate};
use crate::{
  rules::{
    comparable_rules::{validate_gt_lt, Gt, Lt},
    containing_rules::validate_in_not_in,
  },
  validation_data::ValidationData,
  validator_template::FieldValidator,
};

pub fn get_int64_rules(
  validation_data: ValidationData,
  rules: &Int64Rules,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut templates: Vec<ValidatorTemplate> = Vec::new();

  let mut lt: Option<Lt<i64>> = None;
  let mut gt: Option<Gt<i64>> = None;

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

  if let Some(lt_rule) = rules.less_than {
    match lt_rule {
      LessThan::Lt(val) => {
        lt = Some(Lt { val, eq: false });
        templates.push(ValidatorTemplate {
          item_rust_name: validation_data.field_data.rust_name.clone(),
          kind: ValidatorKind::Field {
            validation_data: validation_data.clone(),
            field_validator: FieldValidator::Scalar {
              validator_path: quote! { protocheck::validators::numeric::lt },
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
              validator_path: quote! { protocheck::validators::numeric::lte },
              target_value_tokens: val.to_token_stream(),
            },
          },
        });
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
              validator_path: quote! { protocheck::validators::numeric::gt },
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
              validator_path: quote! { protocheck::validators::numeric::gte },
              target_value_tokens: val.to_token_stream(),
            },
          },
        });
      }
    };
  }

  validate_gt_lt(&gt, &lt, &error_prefix, field_span)?;
  validate_in_not_in(&rules.r#in, &rules.not_in, &error_prefix, field_span)?;

  Ok(templates)
}
