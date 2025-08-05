#![allow(clippy::field_reassign_with_default)]

use std::collections::HashMap;

use pool_loader::DESCRIPTOR_POOL;
use proc_macro::TokenStream;
pub(crate) use proc_macro2::{Ident as Ident2, Span as Span2, TokenStream as TokenStream2};
pub(crate) use proto_types::field_descriptor_proto::Type as ProtoType;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error, Ident, LitStr};

use crate::{
  extract_validators::{extract_oneof_validators, OneofValidatorsOutput},
  rules::extract_validators::{self, extract_message_validators},
};

mod attribute_extractors;
mod cel_rule_template;
mod cel_try_into;
mod namespaces;
mod pool_loader;
mod protogen;
mod rules;
mod validation_data;

#[proc_macro_derive(OneofTryIntoCelValue)]
pub fn oneof_try_into_cel_value_derive(input: TokenStream) -> TokenStream {
  cel_try_into::derive_cel_value_oneof(input)
}

#[proc_macro_derive(TryIntoCelValue)]
pub fn try_into_cel_value_derive(input: TokenStream) -> TokenStream {
  cel_try_into::derive_cel_value_struct(input)
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

  let (validators, static_defs): (TokenStream2, Vec<TokenStream2>) =
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
        violations: &mut Vec<protocheck::types::protovalidate::Violation>
      ) {

        #validators

      }
    }
  };

  eprintln!("{}", output);

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

  let mut validators: HashMap<Ident, TokenStream2> = HashMap::new();
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
        #validator
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
