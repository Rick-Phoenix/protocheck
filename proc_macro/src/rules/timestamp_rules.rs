use proc_macro2::TokenStream;
use proto_types::{
  protovalidate::{
    timestamp_rules::{GreaterThan, LessThan},
    TimestampRules,
  },
  Timestamp, TimestampError,
};
use quote::quote;
use syn::Error;

use crate::validation_data::ValidationData;

pub fn get_timestamp_rules(
  validation_data: &ValidationData,
  rules: &TimestampRules,
) -> Result<TokenStream, Error> {
  let mut tokens = TokenStream::new();

  let field_span = validation_data.field_span;
  let error_prefix = format!("Error for field {}:", &validation_data.proto_name);

  let format_timestamp = |t: Timestamp, msg: &str| -> Result<String, Error> {
    t.format(&format!("{} {}", msg, "%d %b %Y %R %Z"))
      .map_err(|e: TimestampError| {
        Error::new(
          field_span,
          format!(
            "{} failed to convert protobuf timestamp to chrono timestamp: {}",
            error_prefix, e
          ),
        )
      })
  };

  let field_context_ident = &validation_data.field_context_ident();
  let value_ident = validation_data.value_ident();

  if let Some(const_val) = rules.r#const {
    let error_message = format_timestamp(const_val, "must be equal to")?;

    let validator_tokens =
      validation_data.get_const_validator("timestamp", const_val, &error_message);

    tokens.extend(validator_tokens);

    return Ok(tokens);
  }

  if let Some(within_val) = rules.within {
    let error_message = format!("must be within {} from now", within_val,);

    let validator_expression_tokens = quote! {
      protocheck::validators::timestamps::within(&#field_context_ident, #value_ident, #within_val, #error_message)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  let (greater_than, less_than) = rules.comparable_rules(field_span, &error_prefix)?;

  if let Some(lt_rule) = less_than {
    match lt_rule {
      LessThan::Lt(lt_val) => {
        let error_message = format_timestamp(lt_val, "must be earlier than")?;

        let validator_tokens =
          validation_data.get_comparable_validator("timestamp", "lt", lt_val, &error_message);

        tokens.extend(validator_tokens);
      }
      LessThan::Lte(lte_val) => {
        let error_message = format_timestamp(lte_val, "cannot be later than")?;

        let validator_tokens =
          validation_data.get_comparable_validator("timestamp", "lte", lte_val, &error_message);

        tokens.extend(validator_tokens);
      }
      LessThan::LtNow(enabled) => {
        if enabled {
          let validator_expression_tokens = quote! {
            protocheck::validators::timestamps::lt_now(&#field_context_ident, #value_ident)
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
        let error_message = format_timestamp(gt_val, "must be later than")?;

        let validator_tokens =
          validation_data.get_comparable_validator("timestamp", "gt", gt_val, &error_message);

        tokens.extend(validator_tokens);
      }
      GreaterThan::Gte(gte_val) => {
        let error_message = format_timestamp(gte_val, "cannot be earlier than")?;

        let validator_tokens =
          validation_data.get_comparable_validator("timestamp", "gte", gte_val, &error_message);

        tokens.extend(validator_tokens);
      }
      GreaterThan::GtNow(enabled) => {
        if enabled {
          let validator_expression_tokens = quote! {
            protocheck::validators::timestamps::gt_now(&#field_context_ident, #value_ident)
          };
          let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

          tokens.extend(validator_tokens);
        }
      }
    };
  }

  Ok(tokens)
}
