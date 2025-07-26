#![allow(clippy::field_reassign_with_default)]

use std::collections::HashMap;

use pool_loader::DESCRIPTOR_POOL;
use proc_macro::TokenStream;
pub(crate) use proc_macro2::{Span as Span2, TokenStream as TokenStream2};
use protocheck_core::internals::validator_template::ValidatorCallTemplate;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error, Ident, LitStr};

use crate::{
  extract_validators::extract_oneof_validators,
  rules::extract_validators::{self, extract_message_validators},
};

mod namespaces;
mod pool_loader;
mod protogen;
mod rules;

#[proc_macro_attribute]
pub fn protobuf_validate(attrs: TokenStream, input: TokenStream) -> TokenStream {
  let proto_message_name_tokens = parse_macro_input!(attrs as LitStr);

  let proto_message_name = proto_message_name_tokens.value();

  if proto_message_name.is_empty() {
    return quote! {}.into();
  }

  let message_desc = DESCRIPTOR_POOL.get_message_by_name(&proto_message_name);

  if message_desc.is_none() {
    println!("Message {} not found", proto_message_name);
    return quote! {}.into();
  }

  let input_clone = input.clone();
  let ast = parse_macro_input!(input_clone as DeriveInput);

  let struct_ident = ast.ident.clone();

  let original_input_as_proc_macro2: proc_macro2::TokenStream = input.into();

  let validator_call_templates = extract_message_validators(ast, message_desc.unwrap()).unwrap();

  let output = quote! {
    #original_input_as_proc_macro2

    impl protocheck::validators::WithValidator for #struct_ident {
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

        #(#validator_call_templates)*

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
  let oneof_enum_name = &ast.ident.clone();
  let original_input_as_proc_macro2: proc_macro2::TokenStream = input.into();

  let proto_oneof_name_tokens = parse_macro_input!(attrs as LitStr);
  let oneof_full_name = proto_oneof_name_tokens.value();

  if oneof_full_name.is_empty() {
    return Error::new_spanned(
      &ast,
      format!("Could not find protobuf path for oneof {}", ast.ident),
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

  let message_desc = DESCRIPTOR_POOL.get_message_by_name(parent_message_name);

  if message_desc.is_none() {
    return Error::new_spanned(
      ast,
      format!(
        "Parent message {} not found for oneof {}",
        parent_message_name, oneof_name
      ),
    )
    .to_compile_error()
    .into();
  }

  let mut validators: HashMap<Ident, Vec<ValidatorCallTemplate>> = HashMap::new();

  for oneof in message_desc.unwrap().oneofs() {
    if oneof.name() == oneof_name {
      match extract_oneof_validators(ast, oneof) {
        Ok(validators_data) => validators = validators_data,
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

  let output = quote! {
    #original_input_as_proc_macro2

    impl protocheck::validators::WithValidator for #oneof_enum_name {
      fn validate(&self) -> Result<(), protocheck::types::protovalidate::Violations> {
        self.nested_validate(&mut parent_messages, &mut violations);
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

// #[proc_macro_derive(
//   ProtoMessage,
//   attributes(field_num, reserved_nums, reserved_ranges, reserved_names, protoschema)
// )]
// pub fn proto_message_macro_derive(input: TokenStream) -> TokenStream {
//   parse_proto_message(input)
// }
