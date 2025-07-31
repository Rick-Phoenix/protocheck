use proto_types::{
  protovalidate::{
    duration_rules::{GreaterThan, LessThan},
    DurationRules,
  },
  Duration,
};
use quote::{quote, ToTokens};
use syn::Error;

use super::{ValidatorKind, ValidatorTemplate};
use crate::{
  rules::{
    comparable_rules::{validate_gt_lt, Gt, Lt},
    containing_rules::validate_in_not_in,
  },
  validation_data::ValidationData,
  validator_template::FieldValidator,
};

pub fn get_duration_rules(
  validation_data: &ValidationData,
  rules: &DurationRules,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut templates: Vec<ValidatorTemplate> = Vec::new();

  let mut lt: Option<Lt<Duration>> = None;
  let mut gt: Option<Gt<Duration>> = None;

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
    };
  }

  validate_gt_lt(&gt, &lt, &error_prefix, field_span)?;
  validate_in_not_in(&rules.r#in, &rules.not_in, &error_prefix, field_span)?;

  if !rules.r#in.is_empty() {
    let in_list = rules.r#in.clone();
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::containing::in_list },
          target_value_tokens: quote! { &vec![ #(#in_list),* ] },
        },
      },
    });
  }

  Ok(templates)
}
