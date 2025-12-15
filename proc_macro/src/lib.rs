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
  parse::ParseStream, parse_macro_input, parse_quote, punctuated::Punctuated, spanned::Spanned,
  Attribute, Error, Expr, Ident, Item, ItemEnum, ItemStruct, Lit, LitByteStr, LitStr, Meta, Path,
  Token,
};

use crate::{
  attributes::*, cel_rule_template::*, message_validator::*, oneof_validator::*, pool_loader::*,
  rules::*, special_field_names::*, utils::*, validation_data::*,
};

#[macro_use]
mod macros;
mod attributes;
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

/// Adds conversion functions into [`cel::Value`] for messages.
#[cfg(feature = "cel")]
#[proc_macro_derive(TryIntoCelValue)]
pub fn try_into_cel_value_derive(input: TokenStream) -> TokenStream {
  let item = parse_macro_input!(input as Item);

  let result = match item {
    Item::Struct(s) => cel_try_into::derive_cel_value_struct(s),
    Item::Enum(e) => cel_try_into::derive_cel_value_oneof(e),
    _ => {
      return error!(item, "This macro only works on enums (oneofs) and structs")
        .into_compile_error()
        .into()
    }
  };

  match result {
    Ok(tokens) => tokens.into(),
    Err(e) => e.into_compile_error().into(),
  }
}

/// Adds the validation methods to the generated protobuf message structs.
#[proc_macro_attribute]
pub fn protobuf_validate(attrs: TokenStream, input: TokenStream) -> TokenStream {
  let proto_message_name_tokens = parse_macro_input!(attrs as LitStr);
  let proto_message_name = proto_message_name_tokens.value();

  let mut item = parse_macro_input!(input as ItemStruct);

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

  let validators = match extract_message_validators(&item, &message_desc) {
    Ok(v) => v,
    Err(e) => return e.to_compile_error().into(),
  };

  let struct_ident = &item.ident;

  if cfg!(feature = "cel") {
    let cel_attr: Attribute = parse_quote!(#[derive(::protocheck::macros::TryIntoCelValue)]);

    item.attrs.push(cel_attr);
  }

  let output = quote! {
    #item

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
  let mut item = parse_macro_input!(input as ItemEnum);

  let proto_oneof_name_tokens = parse_macro_input!(attrs as LitStr);
  let oneof_full_name = proto_oneof_name_tokens.value();

  let (parent_message_name, oneof_name) = match oneof_full_name.rsplit_once('.') {
    Some((parent, oneof)) => (parent, oneof),
    None => {
      return Error::new_spanned(
        proto_oneof_name_tokens,
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
        item,
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

  for oneof in message_desc.oneofs() {
    if oneof.name() == oneof_name {
      match extract_oneof_validators(&item, &oneof) {
        Ok(OneofValidatorsOutput {
          validators: validators_data,
        }) => {
          validators = validators_data;
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

  let oneof_rust_ident = &item.ident;

  if cfg!(feature = "cel") {
    let cel_attr: Attribute = parse_quote!(#[derive(::protocheck::macros::TryIntoCelValue)]);

    item.attrs.push(cel_attr);
  }

  let output = quote! {
    #[derive(::protocheck::macros::Oneof)]
    #item

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

  output.into()
}

/// Adds the protobuf field name of the corresponding rust enum variant for a oneof.
#[proc_macro_derive(Oneof, attributes(protocheck))]
pub fn derive_oneof(_input: TokenStream) -> TokenStream {
  TokenStream::new()
}
