#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![allow(clippy::collapsible_if)]

use std::{
  borrow::Cow,
  cell::OnceCell,
  collections::{HashMap, HashSet},
  fmt::{Debug, Display},
  hash::Hash,
  sync::{Arc, LazyLock},
};

use bytes::Bytes;
use convert_case::{Case, Casing};
use pool_loader::DESCRIPTOR_POOL;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use prost_reflect::{
  prost::Message, DescriptorPool, DynamicMessage, EnumDescriptor, ExtensionDescriptor,
  FieldDescriptor, Kind as ProstReflectKind, MessageDescriptor, OneofDescriptor, ReflectMessage,
  Value as ProstValue,
};
use proto_types::{
  field_descriptor_proto::Type as ProtoType,
  protovalidate::{field_rules::Type as RulesType, *},
  Duration, Empty, FieldMask, FieldType, Timestamp,
};
use protocheck_core::field_data::FieldKind;
use quote::{format_ident, quote, ToTokens};
use regex::Regex;
use syn::{
  parse::ParseStream, parse_macro_input, punctuated::Punctuated, spanned::Spanned, Attribute,
  DeriveInput, Error, Ident, ItemStruct, LitByteStr, LitStr, Meta, Path, Token, Type,
};

use crate::{
  attribute_extractors::*, cel_rule_template::*, message_validator::*, oneof_validator::*,
  pool_loader::*, rules::*, special_field_names::*, utils::*, validation_data::*,
};

#[macro_use]
mod macros;
mod attribute_extractors;
mod cel_rule_template;
#[cfg(feature = "cel")]
mod cel_try_into;
mod message_validator;
mod oneof_validator;
mod pool_loader;
mod rules;
mod special_field_names;
mod utils;
mod validation_data;

/// Adds conversion functions into [`cel::Value`] for oneofs.
#[cfg(feature = "cel")]
#[proc_macro_derive(OneofTryIntoCelValue)]
pub fn oneof_try_into_cel_value_derive(input: TokenStream) -> TokenStream {
  cel_try_into::derive_cel_value_oneof(input)
}

/// Adds conversion functions into [`cel::Value`] for messages.
#[cfg(feature = "cel")]
#[proc_macro_derive(TryIntoCelValue)]
pub fn try_into_cel_value_derive(input: TokenStream) -> TokenStream {
  cel_try_into::derive_cel_value_struct(input)
}

/// Adds the validation methods to the generated protobuf message structs.
#[proc_macro_attribute]
pub fn protobuf_validate(attrs: TokenStream, input: TokenStream) -> TokenStream {
  let proto_message_name_tokens = parse_macro_input!(attrs as LitStr);
  let proto_message_name = proto_message_name_tokens.value();

  let input = parse_macro_input!(input as ItemStruct);

  let message_desc = match DESCRIPTOR_POOL.get_message_by_name(&proto_message_name) {
    Some(message) => message,
    None => {
      return Error::new_spanned(
        proto_message_name_tokens,
        format!(
          "Message {} not found in the descriptor pool",
          proto_message_name
        ),
      )
      .to_compile_error()
      .into()
    }
  };

  let (validators, static_defs): (TokenStream2, TokenStream2) =
    match extract_message_validators(&input, &message_desc) {
      Ok((validators_data, static_defs)) => (validators_data, static_defs),
      Err(e) => return e.to_compile_error().into(),
    };

  let struct_ident = &input.ident;

  let output = quote! {
    #static_defs

    #input

    impl #struct_ident {
      pub fn validate(&self) -> Result<(), ::protocheck::types::protovalidate::Violations> {
        let mut violations: Vec<::protocheck::types::protovalidate::Violation> = Vec::new();
        let mut parent_messages: Vec<::protocheck::types::protovalidate::FieldPathElement> = Vec::new();

        self.nested_validate(&mut parent_messages, &mut violations);

        if violations.len() > 0 {
          return Err(::protocheck::types::protovalidate::Violations { violations });
        }
        Ok(())
      }

      pub fn nested_validate(
        &self,
        parent_messages: &mut Vec<::protocheck::types::protovalidate::FieldPathElement>,
        violations: &mut Vec<::protocheck::types::protovalidate::Violation>
      ) {

        #validators

      }
    }

    impl ::protocheck::ProtoValidator for #struct_ident {
      fn validate(&self) -> Result<(), ::protocheck::types::protovalidate::Violations> {
        self.validate()
      }
    }
  };

  output.into()
}

/// Adds validation methods to oneofs contained in messages with validators.
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
  let mut static_defs: TokenStream2 = TokenStream2::new();

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
    #static_defs

    #original_input_as_proc_macro2

    impl #oneof_rust_ident {
      pub fn validate(
        &self,
        parent_messages: &mut Vec<::protocheck::types::protovalidate::FieldPathElement>,
        violations: &mut Vec<::protocheck::types::protovalidate::Violation>,
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

/// Adds the protobuf field name of the corresponding rust enum variant for a oneof.
#[proc_macro_derive(Oneof, attributes(protocheck))]
pub fn derive_oneof(_input: TokenStream) -> TokenStream {
  TokenStream::new()
}
