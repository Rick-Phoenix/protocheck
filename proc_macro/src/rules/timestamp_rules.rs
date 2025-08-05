use proto_types::protovalidate::{
  timestamp_rules::{GreaterThan, LessThan},
  TimestampRules,
};
use quote::{quote, ToTokens};
use syn::Error;

use super::{ValidatorKind, ValidatorTemplate};
use crate::validation_data::ValidationData;

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

  let field_context_ident = &validation_data.field_context_ident;
  let value_ident = validation_data.value_ident();

  if let Some(const_val) = rules.r#const {
    templates.push(ValidatorTemplate {
      kind: ValidatorKind::PureTokens(
        validation_data.get_constant_validator(const_val.to_token_stream()),
      ),
    });

    return Ok(templates);
  }

  if let Some(within_val) = rules.within {
    let validator_expression_tokens = quote! {
      protocheck::validators::timestamps::within(&#field_context_ident, #value_ident, #within_val)
    };
    let validator_tokens = validation_data.get_validator_tokens(validator_expression_tokens);

    templates.push(ValidatorTemplate {
      kind: ValidatorKind::PureTokens(validator_tokens),
    });
  }

  let (greater_than, less_than) = rules.comparable_rules(field_span, &error_prefix)?;

  if let Some(lt_rule) = less_than {
    match lt_rule {
      LessThan::Lt(lt_val) => {
        templates.push(ValidatorTemplate {
          kind: ValidatorKind::PureTokens(
            validation_data.get_lt_validator(lt_val.to_token_stream()),
          ),
        });
      }
      LessThan::Lte(lte_val) => {
        templates.push(ValidatorTemplate {
          kind: ValidatorKind::PureTokens(
            validation_data.get_lte_validator(lte_val.to_token_stream()),
          ),
        });
      }
      LessThan::LtNow(enabled) => {
        if enabled {
          let validator_expression_tokens = quote! {
            protocheck::validators::timestamps::lt_now(&#field_context_ident, #value_ident)
          };
          let validator_tokens = validation_data.get_validator_tokens(validator_expression_tokens);

          templates.push(ValidatorTemplate {
            kind: ValidatorKind::PureTokens(validator_tokens),
          });
        }
      }
    };
  }

  if let Some(gt_rule) = greater_than {
    match gt_rule {
      GreaterThan::Gt(gt_val) => {
        templates.push(ValidatorTemplate {
          kind: ValidatorKind::PureTokens(
            validation_data.get_gt_validator(gt_val.to_token_stream()),
          ),
        });
      }
      GreaterThan::Gte(gte_val) => {
        templates.push(ValidatorTemplate {
          kind: ValidatorKind::PureTokens(
            validation_data.get_gte_validator(gte_val.to_token_stream()),
          ),
        });
      }
      GreaterThan::GtNow(enabled) => {
        if enabled {
          let validator_expression_tokens = quote! {
            protocheck::validators::timestamps::gt_now(&#field_context_ident, #value_ident)
          };
          let validator_tokens = validation_data.get_validator_tokens(validator_expression_tokens);

          templates.push(ValidatorTemplate {
            kind: ValidatorKind::PureTokens(validator_tokens),
          });
        }
      }
    };
  }

  Ok(templates)
}
