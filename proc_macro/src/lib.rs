#![allow(clippy::field_reassign_with_default)]

use std::collections::HashMap;

use pool_loader::DESCRIPTOR_POOL;
use proc_macro::TokenStream;
pub(crate) use proc_macro2::{Ident as Ident2, Span as Span2, TokenStream as TokenStream2};
pub(crate) use proto_types::field_descriptor_proto::Type as ProtoType;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Error, Ident, LitStr, Type};
use validator_template::ValidatorTemplate;

use crate::{
  extract_validators::{extract_oneof_validators, OneofValidatorsOutput},
  rules::extract_validators::{self, extract_message_validators},
};

mod cel_rule_template;
mod namespaces;
mod pool_loader;
mod protogen;
mod rules;
mod validation_data;
mod validator_template;

fn type_matches_path(ty: &Type, target_path: &str) -> bool {
  let path: syn::Path = syn::parse_str(target_path).unwrap();
  ty.to_token_stream().to_string() == path.to_token_stream().to_string()
}

fn is_primitive(ty: &syn::Type) -> bool {
  if let syn::Type::Path(type_path) = ty {
    if let Some(segment) = type_path.path.segments.last() {
      let type_name = segment.ident.to_string();

      return matches!(
        type_name.as_str(),
        "bool"
          | "i8"
          | "i16"
          | "i32"
          | "i64"
          | "i128"
          | "u8"
          | "u16"
          | "u32"
          | "u64"
          | "u128"
          | "f32"
          | "f64"
          | "char"
          | "str"
          | "String"
      );
    }
  }
  false
}

#[proc_macro_derive(TryIntoCelValue)]
pub fn try_into_cel_value_derive(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);

  let struct_name = &ast.ident;

  let fields = if let syn::Data::Struct(syn::DataStruct {
    fields: syn::Fields::Named(fields),
    ..
  }) = &ast.data
  {
    &fields.named
  } else {
    panic!("This derive macro only works on structs with named fields");
  };

  let fields_map_ident = Ident2::new("fields", Span2::call_site());
  let mut tokens = TokenStream2::new();

  for field in fields {
    let field_ident = field.ident.as_ref().unwrap();
    let field_name = field_ident.to_string();
    let field_type = &field.ty;

    if let syn::Type::Path(type_path) = field_type {
      if let Some(segment) = type_path.path.segments.last() {
        let type_ident = &segment.ident;

        match type_ident.to_string().as_str() {
          "Option" => {
            if let syn::PathArguments::AngleBracketed(type_args) = &segment.arguments {
              if let Some(syn::GenericArgument::Type(inner_type)) = type_args.args.first() {
                if type_matches_path(inner_type, "protocheck::types::Duration") {
                  tokens.extend(quote! {
                    if let Some(v) = &value.#field_ident {
                      let cel_val = cel_interpreter::Value::Duration(v.to_owned().try_into().map_err(|e: protocheck::types::DurationError| e.to_string())?);
                      #fields_map_ident.insert(#field_name.into(), cel_val);
                    } else {
                      #fields_map_ident.insert(#field_name.into(), cel_interpreter::Value::Null);
                    }
                  });
                } else if type_matches_path(inner_type, "protocheck::types::Timestamp") {
                  tokens.extend(quote! {
                    if let Some(v) = &value.#field_ident {
                      let cel_val = cel_interpreter::Value::Timestamp(v.to_owned().try_into().map_err(|e: protocheck::types::TimestampError| e.to_string())?);
                      #fields_map_ident.insert(#field_name.into(), cel_val);
                    } else {
                      #fields_map_ident.insert(#field_name.into(), cel_interpreter::Value::Null);
                    }
                  });
                } else if is_primitive(inner_type)
                  || type_matches_path(inner_type, "protocheck::types::FieldMask")
                {
                  tokens.extend(quote! {
                    if let Some(v) = &value.#field_ident {
                      #fields_map_ident.insert(#field_name.into(), v.into());
                    } else {
                      #fields_map_ident.insert(#field_name.into(), cel_interpreter::Value::Null);
                    }
                  });
                } else {
                  tokens.extend(quote! {
                    if let Some(v) = &value.#field_ident {
                      #fields_map_ident.insert(#field_name.into(), v.to_owned().try_into()?);
                    } else {
                      #fields_map_ident.insert(#field_name.into(), cel_interpreter::Value::Null);
                    }
                  });
                }
              }
            }
          }
          "Vec" => {
            if let syn::PathArguments::AngleBracketed(type_args) = &segment.arguments {
              if let Some(syn::GenericArgument::Type(inner_type)) = type_args.args.first() {
                if type_matches_path(inner_type, "protocheck::types::Timestamp") {
                  tokens.extend(quote! {
                    let mut converted: Vec<cel_interpreter::Value> = Vec::new();
                    for item in value.#field_ident {
                      let chrono_timestamp: DateTime<FixedOffset> = item.to_owned().try_into().map_err(|e: protocheck::types::TimestampError| e.to_string())?;
                      converted.push(cel_interpreter::Value::Timestamp(chrono_timestamp.into()));
                    }

                    #fields_map_ident.insert(#field_name.into(), cel_interpreter::Value::List(converted.into()));
                  });
                } else if type_matches_path(inner_type, "protocheck::types::Duration") {
                  tokens.extend(quote! {
                    let mut converted: Vec<cel_interpreter::Value> = Vec::new();
                    for item in value.#field_ident {
                      let chrono_duration: chrono::Duration = item.to_owned().try_into().map_err(|e: protocheck::types::DurationError| e.to_string())?;
                      converted.push(cel_interpreter::Value::Duration(chrono_duration.into()));
                    }

                    #fields_map_ident.insert(#field_name.into(), cel_interpreter::Value::List(converted.into()));
                  });
                } else if is_primitive(inner_type) {
                  tokens.extend(quote! {
                    #fields_map_ident.insert(#field_name.into(), cel_interpreter::Value::List(value.#field_ident.clone()));
                  });
                }
              }
            }
          }
          "HashMap" => {
            if let syn::PathArguments::AngleBracketed(type_args) = &segment.arguments {
              let key_generic = type_args.args.first();
              let value_generic = type_args.args.get(1);

              if let (
                Some(syn::GenericArgument::Type(_)),
                Some(syn::GenericArgument::Type(value_type)),
              ) = (key_generic, value_generic)
              {
                let map_iter_tokens = if type_matches_path(
                  value_type,
                  "protocheck::types::Duration",
                ) {
                  quote! {
                    let cel_val = cel_interpreter::Value::Duration(v.to_owned().try_into().map_err(|e: protocheck::types::DurationError| e.to_string())?);
                    field_map.insert(k.into(), cel_val);
                  }
                } else if type_matches_path(value_type, "protocheck::types::Timestamp") {
                  quote! {
                      let cel_val = cel_interpreter::Value::Timestamp(v.to_owned().try_into().map_err(|e: protocheck::types::TimestampError| e.to_string())?);
                      field_map.insert(k.into(), cel_val);
                  }
                } else if is_primitive(value_type)
                  || type_matches_path(value_type, "protocheck::types::FieldMask")
                {
                  quote! {
                      field_map.insert(k.into(), v.into());
                  }
                } else {
                  quote! {
                      field_map.insert(k.into(), v.to_owned().try_into()?);
                  }
                };

                tokens.extend(quote! {
                  let mut field_map: std::collections::HashMap<cel_interpreter::objects::Key, cel_interpreter::Value> = std::collections::HashMap::new();

                  for (k, v) in value.#field_ident {
                    #map_iter_tokens

                  }

                  #fields_map_ident.insert(#field_name.into(), cel_interpreter::Value::Map(field_map.into()));
                });
              }
            }
          }
          _ => {
            if is_primitive(field_type) {
              tokens.extend(quote! {
                #fields_map_ident.insert(#field_name.into(), value.#field_ident.into());
              });
            }
          }
        };
      }
    }
  }

  let expanded = quote! {
    impl TryFrom<#struct_name> for ::cel_interpreter::Value {
      type Error = String;

      fn try_from(value: #struct_name) -> Result<Self, Self::Error> {
        let mut #fields_map_ident: std::collections::HashMap<cel_interpreter::objects::Key, cel_interpreter::Value> = std::collections::HashMap::new();
        #tokens
        Ok(cel_interpreter::Value::Map(#fields_map_ident.into()))
      }
    }
  };

  expanded.into()
  // TokenStream::new()
}

#[proc_macro_attribute]
pub fn protobuf_validate(attrs: TokenStream, input: TokenStream) -> TokenStream {
  let proto_message_name_tokens = parse_macro_input!(attrs as LitStr);
  let proto_message_name = proto_message_name_tokens.value();

  let input_clone = input.clone();
  let ast = parse_macro_input!(input_clone as DeriveInput);

  if proto_message_name.is_empty() {
    return Error::new_spanned(
      &ast.ident,
      format!("Found empty message name for {}", &ast.ident),
    )
    .to_compile_error()
    .into();
  }

  let message_desc = match DESCRIPTOR_POOL.get_message_by_name(&proto_message_name) {
    Some(message) => message,
    None => {
      return Error::new_spanned(
        proto_message_name_tokens,
        format!("Message {} not found", proto_message_name),
      )
      .to_compile_error()
      .into()
    }
  };

  let (validators, static_defs): (Vec<ValidatorTemplate>, Vec<TokenStream2>) =
    match extract_message_validators(&ast, &message_desc) {
      Ok((validators_data, static_defs)) => (validators_data, static_defs),
      Err(e) => return e.to_compile_error().into(),
    };

  let original_input_as_proc_macro2: proc_macro2::TokenStream = input.into();
  let struct_ident = &ast.ident;

  let output = quote! {
    #(#static_defs)*

    #original_input_as_proc_macro2

    impl protocheck::validators::ProtoValidator for #struct_ident {
      fn validate(&self) -> Result<(), protocheck::types::protovalidate::Violations> {
        let mut violations: Vec<protocheck::types::protovalidate::Violation> = Vec::new();
        let mut parent_messages: Vec<protocheck::types::protovalidate::FieldPathElement> = Vec::new();

        self.nested_validate(&mut parent_messages, &mut violations);

        if violations.len() > 0 {
          return Err(protocheck::types::protovalidate::Violations { violations });
        }
        Ok(())
      }

      fn nested_validate(
        &self,
        parent_messages: &mut Vec<protocheck::types::protovalidate::FieldPathElement>,
        violations: &mut Vec<protocheck::types::protovalidate::Violation>,
      ) {

        #(#validators)*

      }
    }
  };

  // eprintln!("{}", output);

  output.into()
}

#[proc_macro_attribute]
pub fn protobuf_validate_oneof(attrs: TokenStream, input: TokenStream) -> TokenStream {
  let input_clone = input.clone();
  let ast = parse_macro_input!(input_clone as DeriveInput);

  let proto_oneof_name_tokens = parse_macro_input!(attrs as LitStr);
  let oneof_full_name = proto_oneof_name_tokens.value();

  if oneof_full_name.is_empty() {
    return Error::new_spanned(
      &ast,
      format!("Found empty oneof name attribute for {}", &ast.ident),
    )
    .to_compile_error()
    .into();
  }

  let (parent_message_name, oneof_name) = match oneof_full_name.rsplit_once('.') {
    Some((parent, oneof)) => (parent, oneof),
    None => {
      return Error::new_spanned(
        ast,
        format!(
          "Could not extract parent message and oneof name for {}",
          oneof_full_name
        ),
      )
      .to_compile_error()
      .into()
    }
  };

  let message_desc = match DESCRIPTOR_POOL.get_message_by_name(parent_message_name) {
    Some(message) => message,
    None => {
      return Error::new_spanned(
        ast,
        format!(
          "Parent message {} not found for oneof {}",
          parent_message_name, oneof_name
        ),
      )
      .to_compile_error()
      .into()
    }
  };

  let mut validators: HashMap<Ident, Vec<ValidatorTemplate>> = HashMap::new();
  let mut static_defs: Vec<TokenStream2> = Vec::new();

  for oneof in message_desc.oneofs() {
    if oneof.name() == oneof_name {
      match extract_oneof_validators(&ast, &oneof) {
        Ok(OneofValidatorsOutput {
          validators: validators_data,
          static_defs: static_definitions,
        }) => {
          validators = validators_data;
          static_defs = static_definitions;
        }
        Err(e) => return e.to_compile_error().into(),
      };
      break;
    }
  }

  let mut validators_tokens = TokenStream2::new();

  for (ident, validator) in validators {
    validators_tokens.extend(quote! {
      Self::#ident(val) => {
        #(#validator)*
      },
    });
  }

  let original_input_as_proc_macro2: proc_macro2::TokenStream = input.into();
  let oneof_rust_ident = &ast.ident;

  let output = quote! {
    #(#static_defs)*

    #original_input_as_proc_macro2

    impl protocheck::validators::ProtoValidator for #oneof_rust_ident {
      fn validate(&self) -> Result<(), protocheck::types::protovalidate::Violations> {
        Ok(())
      }

      fn nested_validate(
        &self,
        parent_messages: &mut Vec<protocheck::types::protovalidate::FieldPathElement>,
        violations: &mut Vec<protocheck::types::protovalidate::Violation>,
      ) {
        match self {
          #validators_tokens
        };
      }
    }
  };

  // eprintln!("{}", output);

  output.into()
}

#[proc_macro_derive(Oneof, attributes(protocheck))]
pub fn derive_oneof(_input: TokenStream) -> TokenStream {
  TokenStream::new()
}
