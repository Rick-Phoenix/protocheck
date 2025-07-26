#![allow(clippy::field_reassign_with_default)]

use pool_loader::DESCRIPTOR_POOL;
use proc_macro::TokenStream;
pub(crate) use proc_macro2::Span as Span2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, LitStr};

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

#[proc_macro_attribute]
pub fn protobuf_validate_oneof(attrs: TokenStream, input: TokenStream) -> TokenStream {
  let proto_oneof_name_tokens = parse_macro_input!(attrs as LitStr);

  let oneof_full_name = proto_oneof_name_tokens.value();

  if oneof_full_name.is_empty() {
    return quote! {}.into();
  }

  let (parent_message_name, oneof_name) = match oneof_full_name.rsplit_once('.') {
    Some((parent, oneof)) => (parent, oneof),
    None => return quote! {}.into(),
  };

  println!(
    "Parent message: {}, Oneof: {}",
    parent_message_name, oneof_name
  );

  let input_clone = input.clone();
  let ast = parse_macro_input!(input_clone as DeriveInput);

  let original_input_as_proc_macro2: proc_macro2::TokenStream = input.into();

  let mut oneof_variants: Vec<Ident> = Vec::new();

  if let syn::Data::Enum(data_enum) = &ast.data {
    for variant in &data_enum.variants {
      oneof_variants.push(variant.ident.clone());
    }
  }

  // let message_desc = DESCRIPTOR_POOL.get_message_by_name(&proto_message_name);
  //
  // if message_desc.is_none() {
  //   println!("Message {} not found", proto_message_name);
  //   return quote! {}.into();
  // }

  // println!("{}", proto_message_name);

  //
  // let validator_call_templates = extract_validators(ast, message_desc.unwrap()).unwrap();

  let output = quote! {
    #original_input_as_proc_macro2

    // impl protocheck::validators::WithValidator for #struct_ident {
    //   fn validate(&self) -> Result<(), protocheck::types::protovalidate::Violations> {
    //     let mut violations: Vec<protocheck::types::protovalidate::Violation> = Vec::new();
    //     let mut parent_messages: Vec<protocheck::types::protovalidate::FieldPathElement> = Vec::new();
    //
    //     self.nested_validate(&mut parent_messages, &mut violations);
    //
    //     if violations.len() > 0 {
    //       return Err(protocheck::types::protovalidate::Violations { violations });
    //     }
    //     Ok(())
    //   }
    //
    //   fn nested_validate(
    //     &self,
    //     parent_messages: &mut Vec<protocheck::types::protovalidate::FieldPathElement>,
    //     violations: &mut Vec<protocheck::types::protovalidate::Violation>,
    //   ) {
    //
    //     #(#validator_call_templates)*
    //
    //   }
    // }
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
