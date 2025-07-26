#![allow(clippy::field_reassign_with_default)]

use pool_loader::DESCRIPTOR_POOL;
use proc_macro::TokenStream;
pub(crate) use proc_macro2::{Ident as Ident2, Span as Span2, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};

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

  // println!("{}", proto_message_name);

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

// #[proc_macro_derive(
//   ProtoMessage,
//   attributes(field_num, reserved_nums, reserved_ranges, reserved_names, protoschema)
// )]
// pub fn proto_message_macro_derive(input: TokenStream) -> TokenStream {
//   parse_proto_message(input)
// }
