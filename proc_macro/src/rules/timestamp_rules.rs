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
use crate::{validation_data::ValidationData, validator_template::FieldValidator};

#[derive(Debug)]
struct GtLt {
  pub val: Timestamp,
  pub eq: bool,
}

pub fn get_timestamp_rules(
  validation_data: &ValidationData,
  rules: &TimestampRules,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut templates: Vec<ValidatorTemplate> = Vec::new();

  let mut lt: Option<GtLt> = None;
  let mut gt: Option<GtLt> = None;

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
        lt = Some(GtLt { val, eq: false });
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
        lt = Some(GtLt { val, eq: true });
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
      LessThan::LtNow(_) => {}
    };
  }

  if let Some(gt_rule) = rules.greater_than {
    match gt_rule {
      GreaterThan::Gt(val) => {
        gt = Some(GtLt { val, eq: false });
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
        gt = Some(GtLt { val, eq: true });
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
      GreaterThan::GtNow(_) => {}
    };
  }

  if let Some(gt_val) = gt {
    if let Some(lt_val) = lt {
      if lt_val.eq && gt_val.eq && lt_val.val > gt_val.val {
        return Err(Error::new(
          field_span,
          format!("{} Lte cannot be larger than Gte", error_prefix),
        ));
      }
      if !lt_val.eq && !gt_val.eq && lt_val.val >= gt_val.val {
        return Err(Error::new(
          field_span,
          format!("{} Lt cannot be larger than or equal to Gt", error_prefix),
        ));
      }
      if lt_val.eq && !gt_val.eq && lt_val.val >= gt_val.val {
        return Err(Error::new(
          field_span,
          format!("{} Lte cannot be larger than or equal to Gt", error_prefix),
        ));
      }
      if !lt_val.eq && gt_val.eq && lt_val.val > gt_val.val {
        return Err(Error::new(
          field_span,
          format!("{} Lt cannot be larger than Gte", error_prefix),
        ));
      }
    }
  }

  Ok(templates)
}
