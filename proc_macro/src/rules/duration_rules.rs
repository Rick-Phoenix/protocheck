use proc_macro2::TokenStream;
use proto_types::{
  protovalidate::DurationRules,
  protovalidate_impls::{ComparableGreaterThan, ComparableLessThan, ContainingRules},
};
use quote::{quote, ToTokens};
use syn::Error;

use crate::validation_data::ValidationData;

pub fn get_duration_rules(
  validation_data: &ValidationData,
  rules: &DurationRules,
) -> Result<TokenStream, Error> {
  let mut tokens = TokenStream::new();

  let field_span = validation_data.field_span;

  let error_prefix = format!("Error for field {}:", &validation_data.proto_name);

  if let Some(const_val) = rules.r#const {
    let validator_tokens = validation_data.get_constant_validator(const_val.to_token_stream());
    tokens.extend(validator_tokens);

    return Ok(tokens);
  }

  let comparable_rules = rules.comparable_rules(field_span, &error_prefix)?;

  if let Some(lt_rule) = comparable_rules.less_than {
    match lt_rule {
      ComparableLessThan::Lt(lt_val) => {
        let validator_tokens = validation_data.get_lt_validator(lt_val.to_token_stream());

        tokens.extend(validator_tokens);
      }
      ComparableLessThan::Lte(lte_val) => {
        let validator_tokens = validation_data.get_lte_validator(lte_val.to_token_stream());
        tokens.extend(validator_tokens);
      }
    };
  }

  if let Some(gt_rule) = comparable_rules.greater_than {
    match gt_rule {
      ComparableGreaterThan::Gt(gt_val) => {
        let validator_tokens = validation_data.get_gt_validator(gt_val.to_token_stream());
        tokens.extend(validator_tokens);
      }
      ComparableGreaterThan::Gte(gte_val) => {
        let validator_tokens = validation_data.get_gte_validator(gte_val.to_token_stream());

        tokens.extend(validator_tokens);
      }
    };
  }

  let ContainingRules {
    in_list,
    not_in_list,
  } = rules.containing_rules(field_span, &error_prefix)?;

  if !in_list.is_empty() {
    let in_list_tokens = quote! { vec![ #(#in_list),* ] };
    let validator_tokens = validation_data.get_in_list_validator(in_list_tokens);

    tokens.extend(validator_tokens);
  }

  if !not_in_list.is_empty() {
    let not_in_list_tokens = quote! { vec![ #(#not_in_list),* ] };
    let validator_tokens = validation_data.get_not_in_list_validator(not_in_list_tokens);

    tokens.extend(validator_tokens);
  }

  Ok(tokens)
}
