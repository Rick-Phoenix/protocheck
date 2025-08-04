use proc_macro2::{Span, TokenStream};
use prost_reflect::FieldDescriptor;
use proto_types::{protovalidate::BytesRules, protovalidate_impls::LengthRules};
use quote::{format_ident, quote, ToTokens};
use regex::Regex;
use syn::{Error, LitByteStr};

use super::{ValidatorKind, ValidatorTemplate};
use crate::{validation_data::ValidationData, validator_template::FieldValidator};

pub fn get_bytes_rules(
  static_defs: &mut Vec<TokenStream>,
  field_desc: &FieldDescriptor,
  validation_data: &ValidationData,
  rules: &BytesRules,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut templates: Vec<ValidatorTemplate> = Vec::new();

  let field_span = validation_data.field_span;
  let error_prefix = format!("Error for field {}:", validation_data.full_name);

  if let Some(const_val) = &rules.r#const {
    let const_val_tokens = LitByteStr::new(const_val, Span::call_site());

    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::constants::constant },
          target_value_tokens: const_val_tokens.to_token_stream(),
        },
      },
    });
    return Ok(templates);
  }

  let (in_list, not_in_list) = rules.containing_rules(field_span, &error_prefix)?;

  let LengthRules {
    len,
    min_len,
    max_len,
  } = rules.length_rules(field_span, &error_prefix)?;

  if let Some(ref pattern) = rules.pattern {
    Regex::new(pattern).map_err(|e| {
      Error::new(
        field_span,
        format!("{} invalid regex pattern: {}", error_prefix, e),
      )
    })?;

    let static_regex_ident = format_ident!("__{}_REGEX", field_desc.full_name());
    static_defs.push(quote! {
      static #static_regex_ident: std::sync::LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(#pattern).unwrap()
      });
    });

    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::bytes::pattern },
          target_value_tokens: quote! { #static_regex_ident },
        },
      },
    });
  }

  if let Some(in_list_tokens) = in_list {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::containing::in_list },
          target_value_tokens: in_list_tokens,
        },
      },
    });
  }

  if let Some(not_in_list_tokens) = not_in_list {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::containing::not_in_list },
          target_value_tokens: not_in_list_tokens,
        },
      },
    });
  }

  if let Some(len_value) = len {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::bytes::len },
          target_value_tokens: len_value.into_token_stream(),
        },
      },
    });
  }

  if let Some(min_len_value) = min_len {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::bytes::min_len },
          target_value_tokens: min_len_value.into_token_stream(),
        },
      },
    });
  }

  if let Some(max_len_value) = max_len {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::bytes::max_len },
          target_value_tokens: max_len_value.into_token_stream(),
        },
      },
    });
  }

  if let Some(ref contains_val) = rules.contains {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::bytes::contains },
          target_value_tokens: LitByteStr::new(contains_val, Span::call_site()).to_token_stream(),
        },
      },
    });
  }

  if let Some(ref prefix) = rules.prefix {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::bytes::prefix },
          target_value_tokens: LitByteStr::new(prefix, Span::call_site()).to_token_stream(),
        },
      },
    });
  }

  if let Some(ref suffix) = rules.suffix {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::bytes::suffix },
          target_value_tokens: LitByteStr::new(suffix, Span::call_site()).to_token_stream(),
        },
      },
    });
  }

  Ok(templates)
}
