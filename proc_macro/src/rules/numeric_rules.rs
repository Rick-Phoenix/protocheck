use std::{fmt::Debug, hash::Hash};

use proc_macro2::{Ident, Span, TokenStream};
use proto_types::{
  protovalidate_impls::{ComparableGreaterThan, ComparableLessThan, ContainingRules, NumericRules},
  FieldType,
};
use quote::{quote, ToTokens};
use syn::Error;

use crate::{rules::core::hashset_to_tokens, validation_data::ValidationData};

pub fn get_numeric_rules<HashableType, T: NumericRules<HashableType>>(
  validation_data: &ValidationData,
  rules: &T,
  static_defs: &mut Vec<TokenStream>,
) -> Result<TokenStream, Error>
where
  HashableType: Debug + Copy + ToTokens + Eq + PartialOrd + Hash,
{
  let mut tokens = TokenStream::new();

  let field_span = validation_data.field_span;

  let error_prefix = format!("Error for field {}:", &validation_data.proto_name);

  if let Some(const_val) = rules.constant() {
    let error_message = format!("has to be equal to {:?}", const_val);

    let validator_tokens =
      validation_data.get_constant_validator(&const_val.to_token_stream(), &error_message);

    tokens.extend(validator_tokens);

    return Ok(tokens);
  }

  let comparable_rules = rules.comparable_rules(field_span, &error_prefix)?;
  let ContainingRules {
    in_list,
    not_in_list,
  } = rules.containing_rules(field_span, &error_prefix)?;

  let field_context_ident = &validation_data.field_context_ident();
  let value_ident = validation_data.value_ident();

  if let Some((in_list, in_list_str)) = in_list {
    let in_list_ident = Ident::new(
      &format!("__{}_IN_LIST", validation_data.static_full_name()),
      Span::call_site(),
    );
    let type_tokens = rules.hashable_type_tokens();
    let error_message = format!("must be one of these values: [ {} ]", in_list_str);
    let hashset_tokens = hashset_to_tokens(in_list, &type_tokens);

    static_defs.push(quote! {
      static #in_list_ident: ::std::sync::LazyLock<std::collections::HashSet<#type_tokens>> = ::std::sync::LazyLock::new(||{
        #hashset_tokens
      });
    });

    let validator_expression_tokens = match validation_data.field_kind.inner_type() {
      FieldType::Float | FieldType::Double => quote! {
        protocheck::validators::containing::in_list(&#field_context_ident, #value_ident.to_bits(), &#in_list_ident, #error_message)
      },

      _ => quote! {
        protocheck::validators::containing::in_list(&#field_context_ident, #value_ident, &#in_list_ident, #error_message)
      },
    };

    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);
    tokens.extend(validator_tokens);
  }

  if let Some((not_in_list, not_in_list_str)) = not_in_list {
    let not_in_list_ident = Ident::new(
      &format!("__{}_NOT_IN_LIST", validation_data.static_full_name()),
      Span::call_site(),
    );
    let type_tokens = rules.hashable_type_tokens();
    let error_message = format!("cannot be one of these values: [ {} ]", not_in_list_str);
    let hashset_tokens = hashset_to_tokens(not_in_list, &type_tokens);

    static_defs.push(quote! {
      static #not_in_list_ident: ::std::sync::LazyLock<std::collections::HashSet<#type_tokens>> = ::std::sync::LazyLock::new(||{
        #hashset_tokens
      });
    });

    let validator_expression_tokens = match validation_data.field_kind.inner_type() {
      FieldType::Float | FieldType::Double => quote! {
        protocheck::validators::containing::not_in_list(&#field_context_ident, #value_ident.to_bits(), &#not_in_list_ident, #error_message)
      },

      _ => quote! {
        protocheck::validators::containing::not_in_list(&#field_context_ident, #value_ident, &#not_in_list_ident, #error_message)
      },
    };

    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);
    tokens.extend(validator_tokens);
  }

  if let Some(lt_rule) = comparable_rules.less_than {
    match lt_rule {
      ComparableLessThan::Lt(lt_val) => {
        let error_message = format!("must be smaller than {:?}", lt_val);
        let validator_tokens =
          validation_data.get_lt_validator(&lt_val.to_token_stream(), &error_message);
        tokens.extend(validator_tokens);
      }
      ComparableLessThan::Lte(lte_val) => {
        let error_message = format!("cannot be greater than {:?}", lte_val);
        let validator_tokens =
          validation_data.get_lte_validator(&lte_val.to_token_stream(), &error_message);
        tokens.extend(validator_tokens);
      }
    };
  }

  if let Some(gt_rule) = comparable_rules.greater_than {
    match gt_rule {
      ComparableGreaterThan::Gt(gt_val) => {
        let error_message = format!("must be more than {:?}", gt_val);
        let validator_tokens =
          validation_data.get_gt_validator(&gt_val.to_token_stream(), &error_message);

        tokens.extend(validator_tokens);
      }
      ComparableGreaterThan::Gte(gte_val) => {
        let error_message = format!("cannot be less than {:?}", gte_val);
        let validator_tokens =
          validation_data.get_gte_validator(&gte_val.to_token_stream(), &error_message);

        tokens.extend(validator_tokens);
      }
    };
  }

  if let Some(func_tokens) = rules.finite() {
    let validator_expression_tokens = quote! {
      #func_tokens(&#field_context_ident, #value_ident)
    };
    let validator_tokens = validation_data.get_validator_tokens(&validator_expression_tokens);

    tokens.extend(validator_tokens);
  }

  Ok(tokens)
}
