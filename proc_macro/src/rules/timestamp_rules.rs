use proc_macro2::TokenStream;
use proto_types::protovalidate::{
  timestamp_rules::{GreaterThan, LessThan},
  TimestampRules,
};
use quote::{quote, ToTokens};
use syn::Error;

use crate::validation_data::ValidationData;

pub fn get_timestamp_rules(
  validation_data: &ValidationData,
  rules: &TimestampRules,
) -> Result<TokenStream, Error> {
  let mut tokens = TokenStream::new();

  let field_span = validation_data.field_span;

  let error_prefix = format!("Error for field {}:", &validation_data.proto_name);

  let field_context_ident = &validation_data.field_context_ident();
  let value_ident = validation_data.value_ident();

  if let Some(const_val) = rules.r#const {
    let validator_tokens = validation_data.get_constant_validator(&const_val.to_token_stream());

    tokens.extend(validator_tokens);

    return Ok(tokens);
  }

  if let Some(within_val) = rules.within {
    let validator_expression_tokens = quote! {
      protocheck::validators::timestamps::within(&#field_context_ident, #value_ident.clone(), #within_val)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  let (greater_than, less_than) = rules.comparable_rules(field_span, &error_prefix)?;

  if let Some(lt_rule) = less_than {
    match lt_rule {
      LessThan::Lt(lt_val) => {
        let validator_tokens = validation_data.get_lt_validator(&lt_val.to_token_stream());

        tokens.extend(validator_tokens);
      }
      LessThan::Lte(lte_val) => {
        let validator_tokens = validation_data.get_lte_validator(&lte_val.to_token_stream());

        tokens.extend(validator_tokens);
      }
      LessThan::LtNow(enabled) => {
        if enabled {
          let validator_expression_tokens = quote! {
            protocheck::validators::timestamps::lt_now(&#field_context_ident, #value_ident.clone())
          };
          let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

          tokens.extend(validator_tokens);
        }
      }
    };
  }

  if let Some(gt_rule) = greater_than {
    match gt_rule {
      GreaterThan::Gt(gt_val) => {
        let validator_tokens = validation_data.get_gt_validator(&gt_val.to_token_stream());

        tokens.extend(validator_tokens);
      }
      GreaterThan::Gte(gte_val) => {
        let validator_tokens = validation_data.get_gte_validator(&gte_val.to_token_stream());

        tokens.extend(validator_tokens);
      }
      GreaterThan::GtNow(enabled) => {
        if enabled {
          let validator_expression_tokens = quote! {
            protocheck::validators::timestamps::gt_now(&#field_context_ident, #value_ident.clone())
          };
          let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

          tokens.extend(validator_tokens);
        }
      }
    };
  }

  Ok(tokens)
}
