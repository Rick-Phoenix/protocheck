use proto_types::{
  protovalidate::DurationRules,
  protovalidate_impls::{ComparableGreaterThan, ComparableLessThan, ContainingRules},
};
use quote::{quote, ToTokens};
use syn::Error;

use super::{ValidatorKind, ValidatorTemplate};
use crate::validation_data::ValidationData;

pub fn get_duration_rules(
  validation_data: &ValidationData,
  rules: &DurationRules,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut templates: Vec<ValidatorTemplate> = Vec::new();

  let field_span = validation_data.field_span;

  let error_prefix = format!(
    "Error for field {}:",
    &validation_data.field_data.proto_name
  );

  if let Some(const_val) = rules.r#const {
    templates.push(ValidatorTemplate {
      kind: ValidatorKind::PureTokens(
        validation_data.get_constant_validator(const_val.to_token_stream()),
      ),
    });

    return Ok(templates);
  }

  let comparable_rules = rules.comparable_rules(field_span, &error_prefix)?;

  if let Some(lt_rule) = comparable_rules.less_than {
    match lt_rule {
      ComparableLessThan::Lt(lt_val) => {
        templates.push(ValidatorTemplate {
          kind: ValidatorKind::PureTokens(
            validation_data.get_lt_validator(lt_val.to_token_stream()),
          ),
        });
      }
      ComparableLessThan::Lte(lte_val) => {
        templates.push(ValidatorTemplate {
          kind: ValidatorKind::PureTokens(
            validation_data.get_lte_validator(lte_val.to_token_stream()),
          ),
        });
      }
    };
  }

  if let Some(gt_rule) = comparable_rules.greater_than {
    match gt_rule {
      ComparableGreaterThan::Gt(gt_val) => {
        templates.push(ValidatorTemplate {
          kind: ValidatorKind::PureTokens(
            validation_data.get_gt_validator(gt_val.to_token_stream()),
          ),
        });
      }
      ComparableGreaterThan::Gte(gte_val) => {
        templates.push(ValidatorTemplate {
          kind: ValidatorKind::PureTokens(
            validation_data.get_gte_validator(gte_val.to_token_stream()),
          ),
        });
      }
    };
  }

  let ContainingRules {
    in_list,
    not_in_list,
  } = rules.containing_rules(field_span, &error_prefix)?;

  if !in_list.is_empty() {
    let in_list_tokens = quote! { vec![ #(#in_list),* ] };
    templates.push(ValidatorTemplate {
      kind: ValidatorKind::PureTokens(validation_data.get_in_list_validator(in_list_tokens)),
    });
  }

  if !not_in_list.is_empty() {
    let not_in_list_tokens = quote! { vec![ #(#not_in_list),* ] };
    templates.push(ValidatorTemplate {
      kind: ValidatorKind::PureTokens(
        validation_data.get_not_in_list_validator(not_in_list_tokens),
      ),
    });
  }

  Ok(templates)
}
