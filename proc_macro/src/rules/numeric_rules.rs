use proto_types::protovalidate_impls::{
  ComparableGreaterThan, ComparableLessThan, ContainingRules, NumericRules,
};
use quote::{quote, ToTokens};
use syn::Error;

use super::{ValidatorKind, ValidatorTemplate};
use crate::validation_data::ValidationData;

pub fn get_numeric_rules<T: NumericRules>(
  validation_data: &ValidationData,
  rules: &T,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut templates: Vec<ValidatorTemplate> = Vec::new();

  let field_span = validation_data.field_span;

  let error_prefix = format!(
    "Error for field {}:",
    &validation_data.field_data.proto_name
  );

  if let Some(const_val) = rules.constant() {
    templates.push(ValidatorTemplate {
      kind: ValidatorKind::PureTokens(
        validation_data.get_constant_validator(const_val.to_token_stream()),
      ),
    });

    return Ok(templates);
  }

  let comparable_rules = rules.comparable_rules(field_span, &error_prefix)?;
  let ContainingRules {
    in_list,
    not_in_list,
  } = rules.containing_rules(field_span, &error_prefix)?;

  let field_context_ident = &validation_data.field_context_ident;
  let value_ident = validation_data.value_ident();

  if !in_list.is_empty() {
    templates.push(ValidatorTemplate {
      kind: ValidatorKind::PureTokens(
        validation_data.get_in_list_validator(quote! { vec![ #(#in_list),* ] }),
      ),
    });
  }

  if !not_in_list.is_empty() {
    templates.push(ValidatorTemplate {
      kind: ValidatorKind::PureTokens(
        validation_data.get_not_in_list_validator(quote! { vec![ #(#not_in_list),* ] }),
      ),
    });
  }

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

  if let Some(func_tokens) = rules.finite() {
    let validator_expression_tokens = quote! {
      #func_tokens(&#field_context_ident, #value_ident)
    };
    let validator_tokens = validation_data.get_validator_tokens(validator_expression_tokens);

    templates.push(ValidatorTemplate {
      kind: ValidatorKind::PureTokens(validator_tokens),
    });
  }

  Ok(templates)
}
