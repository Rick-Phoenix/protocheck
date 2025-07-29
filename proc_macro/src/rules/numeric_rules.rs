use proto_types::protovalidate::int64_rules::{GreaterThan, LessThan};
use quote::{quote, ToTokens};
use syn::Error;

use super::{protovalidate::Int64Rules, FieldData, ValidatorKind, ValidatorTemplate};
use crate::Span2;

#[derive(Debug)]
struct GtLt {
  pub val: i64,
  pub eq: bool,
}

pub fn get_int64_rules(
  field_span: Span2,
  field_data: &FieldData,
  rules: &Int64Rules,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut templates: Vec<ValidatorTemplate> = Vec::new();

  let mut lt: Option<GtLt> = None;
  let mut gt: Option<GtLt> = None;

  let error_prefix = format!("Error for field {}:", &field_data.proto_name);

  if let Some(const_val) = rules.r#const {
    templates.push(ValidatorTemplate {
      field_data: field_data.clone(),
      kind: ValidatorKind::FieldRule {
        validator_path: quote! { protocheck::validators::numeric::const },
        target_value_tokens: const_val.to_token_stream(),
      },
    });
    return Ok(templates);
  }

  if let Some(lt_rule) = rules.less_than {
    let field_data = field_data.clone();
    match lt_rule {
      LessThan::Lt(val) => {
        lt = Some(GtLt { val, eq: false });
        templates.push(ValidatorTemplate {
          field_data,
          kind: ValidatorKind::FieldRule {
            validator_path: quote! { protocheck::validators::numeric::lt },
            target_value_tokens: val.to_token_stream(),
          },
        });
      }
      LessThan::Lte(val) => {
        lt = Some(GtLt { val, eq: true });
        templates.push(ValidatorTemplate {
          field_data,
          kind: ValidatorKind::FieldRule {
            validator_path: quote! { protocheck::validators::numeric::lte },
            target_value_tokens: val.to_token_stream(),
          },
        });
      }
    };
  }

  if let Some(gt_rule) = rules.greater_than {
    let field_data = field_data.clone();
    match gt_rule {
      GreaterThan::Gt(val) => {
        gt = Some(GtLt { val, eq: false });
        templates.push(ValidatorTemplate {
          field_data,
          kind: ValidatorKind::FieldRule {
            validator_path: quote! { protocheck::validators::numeric::gt },
            target_value_tokens: val.to_token_stream(),
          },
        });
      }
      GreaterThan::Gte(val) => {
        gt = Some(GtLt { val, eq: true });
        templates.push(ValidatorTemplate {
          field_data,
          kind: ValidatorKind::FieldRule {
            validator_path: quote! { protocheck::validators::numeric::gte },
            target_value_tokens: val.to_token_stream(),
          },
        });
      }
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
