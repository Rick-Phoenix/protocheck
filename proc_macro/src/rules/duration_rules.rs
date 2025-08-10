use proc_macro2::{Ident, Span, TokenStream};
use proto_types::protovalidate::{
  ComparableGreaterThan, ComparableLessThan, ContainingRules, DurationRules,
};
use quote::quote;
use syn::Error;

use crate::{rules::core::hashset_to_tokens, validation_data::ValidationData};

pub fn get_duration_rules(
  validation_data: &ValidationData,
  rules: &DurationRules,
  static_defs: &mut Vec<TokenStream>,
) -> Result<TokenStream, Error> {
  let mut tokens = TokenStream::new();

  let field_span = validation_data.field_span;

  let error_prefix = format!("Error for field {}:", &validation_data.proto_name);

  let field_context_ident = validation_data.field_context_ident();
  let value_ident = validation_data.value_ident();

  if let Some(const_val) = rules.r#const {
    let error_message = format!("has to be equal to {:?}", const_val);

    let validator_tokens =
      validation_data.get_const_validator("duration", const_val, &error_message);
    tokens.extend(validator_tokens);

    return Ok(tokens);
  }

  let comparable_rules = rules.comparable_rules(field_span, &error_prefix)?;

  if let Some(lt_rule) = comparable_rules.less_than {
    match lt_rule {
      ComparableLessThan::Lt(lt_val) => {
        let error_message = format!("must be less than {}", lt_val);
        let validator_tokens =
          validation_data.get_comparable_validator("duration", "lt", lt_val, &error_message);

        tokens.extend(validator_tokens);
      }
      ComparableLessThan::Lte(lte_val) => {
        let error_message = format!("cannot be more than {}", lte_val);

        let validator_tokens =
          validation_data.get_comparable_validator("duration", "lte", lte_val, &error_message);

        tokens.extend(validator_tokens);
      }
    };
  }

  if let Some(gt_rule) = comparable_rules.greater_than {
    match gt_rule {
      ComparableGreaterThan::Gt(gt_val) => {
        let error_message = format!("must be more than {}", gt_val);

        let validator_tokens =
          validation_data.get_comparable_validator("duration", "gt", gt_val, &error_message);

        tokens.extend(validator_tokens);
      }
      ComparableGreaterThan::Gte(gte_val) => {
        let error_message = format!("cannot be less than {}", gte_val);

        let validator_tokens =
          validation_data.get_comparable_validator("duration", "gte", gte_val, &error_message);

        tokens.extend(validator_tokens);
      }
    };
  }

  let ContainingRules {
    in_list,
    not_in_list,
  } = rules.containing_rules(field_span, &error_prefix)?;

  if let Some((in_list, in_list_str)) = in_list {
    let in_list_ident = Ident::new(
      &format!("__{}_IN_LIST", validation_data.static_full_name()),
      Span::call_site(),
    );

    let type_tokens = quote! { ::protocheck::types::Duration };

    let error_message = format!("must be one of these values: [ {} ]", in_list_str);

    let hashset_tokens = hashset_to_tokens(in_list, &type_tokens);

    static_defs.push(quote! {
      static #in_list_ident: ::std::sync::LazyLock<std::collections::HashSet<::protocheck::types::Duration>> = ::std::sync::LazyLock::new(||{
        #hashset_tokens
      });
    });

    let validator_expression_tokens = quote! {
      protocheck::validators::containing::duration_in_list(&#field_context_ident, #value_ident, &#in_list_ident, #error_message)
    };

    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);
    tokens.extend(validator_tokens);
  }

  if let Some((not_in_list, not_in_list_str)) = not_in_list {
    let not_in_list_ident = Ident::new(
      &format!("__{}_NOT_IN_LIST", validation_data.static_full_name()),
      Span::call_site(),
    );

    let type_tokens = quote! { ::protocheck::types::Duration };
    let error_message = format!("cannot be one of these values: [ {} ]", not_in_list_str);
    let hashset_tokens = hashset_to_tokens(not_in_list, &type_tokens);

    static_defs.push(quote! {
      static #not_in_list_ident: ::std::sync::LazyLock<std::collections::HashSet<::protocheck::types::Duration>> = ::std::sync::LazyLock::new(||{
        #hashset_tokens
      });
    });

    let validator_expression_tokens = quote! {
      protocheck::validators::containing::duration_not_in_list(&#field_context_ident, #value_ident, &#not_in_list_ident, #error_message)
    };

    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);
    tokens.extend(validator_tokens);
  }

  Ok(tokens)
}
