use proc_macro2::TokenStream;
use prost_reflect::FieldDescriptor;
use proto_types::protovalidate_impls::{ContainingRules, LengthRules};
use quote::{format_ident, quote, ToTokens};
use regex::Regex;
use syn::Error;

use super::{protovalidate::StringRules, ValidatorKind, ValidatorTemplate};
use crate::{validation_data::ValidationData, validator_template::FieldValidator};

pub fn get_string_rules(
  static_defs: &mut Vec<TokenStream>,
  field_desc: &FieldDescriptor,
  validation_data: &ValidationData,
  rules: &StringRules,
) -> Result<Vec<ValidatorTemplate>, Error> {
  let mut templates: Vec<ValidatorTemplate> = Vec::new();

  let field_span = validation_data.field_span;
  let error_prefix = format!("Error for field {}:", validation_data.full_name);

  if let Some(const_val) = &rules.r#const {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::constants::constant },
          target_value_tokens: const_val.to_token_stream(),
        },
      },
    });
    return Ok(templates);
  }

  let ContainingRules {
    in_list,
    not_in_list,
  } = rules.containing_rules(field_span, &error_prefix)?;

  let LengthRules {
    len,
    min_len,
    max_len,
  } = rules.length_rules(field_span, &error_prefix)?;

  let LengthRules {
    len: len_bytes,
    min_len: min_bytes,
    max_len: max_bytes,
  } = rules.bytes_length_rules(field_span, &error_prefix)?;

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
          validator_path: quote! { protocheck::validators::strings::pattern },
          target_value_tokens: quote! { #static_regex_ident },
        },
      },
    });
  }

  if !in_list.is_empty() {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::containing::in_list },
          target_value_tokens: quote! { vec![ #(#in_list),* ] },
        },
      },
    });
  }

  if !not_in_list.is_empty() {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::containing::not_in_list },
          target_value_tokens: quote! { vec![ #(#not_in_list),* ] },
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
          validator_path: quote! { protocheck::validators::strings::len },
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
          validator_path: quote! { protocheck::validators::strings::min_len },
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
          validator_path: quote! { protocheck::validators::strings::max_len },
          target_value_tokens: max_len_value.into_token_stream(),
        },
      },
    });
  }

  if let Some(len_value) = len_bytes {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::strings::len },
          target_value_tokens: len_value.into_token_stream(),
        },
      },
    });
  }

  if let Some(min_len_value) = min_bytes {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::strings::min_len },
          target_value_tokens: min_len_value.into_token_stream(),
        },
      },
    });
  }

  if let Some(max_len_value) = max_bytes {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::strings::max_len },
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
          validator_path: quote! { protocheck::validators::strings::contains },
          target_value_tokens: contains_val.into_token_stream(),
        },
      },
    });
  }

  if let Some(ref not_contains_val) = rules.not_contains {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::strings::not_contains },
          target_value_tokens: not_contains_val.into_token_stream(),
        },
      },
    });
  }

  if let Some(ref prefix_val) = rules.prefix {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::strings::prefix },
          target_value_tokens: prefix_val.into_token_stream(),
        },
      },
    });
  }

  if let Some(ref suffix_val) = rules.suffix {
    templates.push(ValidatorTemplate {
      item_rust_name: validation_data.field_data.rust_name.clone(),
      kind: ValidatorKind::Field {
        validation_data: validation_data.clone(),
        field_validator: FieldValidator::Scalar {
          validator_path: quote! { protocheck::validators::strings::suffix },
          target_value_tokens: suffix_val.into_token_stream(),
        },
      },
    });
  }

  Ok(templates)
}
