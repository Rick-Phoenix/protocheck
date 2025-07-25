#![allow(clippy::field_reassign_with_default)]

use std::collections::HashSet;

use pool_loader::DESCRIPTOR_POOL;
use proc_macro::TokenStream;
pub(crate) use proc_macro2::{Ident as Ident2, Span as Span2};
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, DeriveInput, LitStr, Token};

use crate::rules::extract_validators::extract_validators;

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

  println!("{}", proto_message_name);

  let input_clone = input.clone();
  let ast = parse_macro_input!(input_clone as DeriveInput);

  let struct_ident = ast.ident.clone();

  let original_input_as_proc_macro2: proc_macro2::TokenStream = input.into();

  let validator_call_templates = extract_validators(ast, message_desc.unwrap()).unwrap();

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

#[proc_macro]
pub fn generate_enum_valid_values(input: TokenStream) -> TokenStream {
  let parsed_args =
    parse_macro_input!(input with Punctuated::<LitStr, Token![,]>::parse_terminated);
  let package_filters: HashSet<String> =
    parsed_args.iter().map(|lit_str| lit_str.value()).collect();

  let mut generated_constants = proc_macro2::TokenStream::new();

  for enum_descriptor in DESCRIPTOR_POOL.all_enums() {
    let full_name = enum_descriptor.full_name();

    let should_include = if package_filters.is_empty() {
      true
    } else {
      package_filters
        .iter()
        .any(|filter| full_name.starts_with(filter))
    };

    if should_include {
      // println!("Full Name: {}", full_name);
      let static_name_str = format!(
        "__VALID_{}_VALUES",
        full_name.replace('.', "_").to_uppercase()
      );
      let static_ident = Ident2::new(&static_name_str, Span2::call_site());

      let mut insert_statements = proc_macro2::TokenStream::new();
      for value in enum_descriptor.values() {
        let num = value.number();
        insert_statements.extend(quote! {
          s.insert(#num);
        });
      }

      generated_constants.extend(quote! {
        #[allow(non_upper_case_globals)]
        pub static #static_ident: std::sync::LazyLock<std::collections::HashSet<i32>> = std::sync::LazyLock::new(|| {
          let mut s = std::collections::HashSet::new();
          #insert_statements
          s
        });
      });
    }
  }

  let output = quote! {
    #[allow(clippy::all)]
    #[allow(non_snake_case)]
    #[allow(unused)]
    pub mod __protobuf_validators_consts {
      use std::collections::HashSet;
      use std::sync::LazyLock;

      #generated_constants
    }
  };

  output.into()
}

// #[proc_macro_derive(
//   ProtoMessage,
//   attributes(field_num, reserved_nums, reserved_ranges, reserved_names, protoschema)
// )]
// pub fn proto_message_macro_derive(input: TokenStream) -> TokenStream {
//   parse_proto_message(input)
// }
