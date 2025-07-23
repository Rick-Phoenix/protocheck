use std::collections::HashSet;

use proc_macro::TokenStream;
use crate::validator::pool_loader::DESCRIPTOR_POOL;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, punctuated::Punctuated, LitStr, Token};
use proc_macro2::{Ident,Span};

use crate::protogen::parse_proto_message;
use crate::validator::core::extract_validators;

mod protogen;
mod validator;

#[proc_macro]
pub fn generate_enum_valid_values(input: TokenStream) -> TokenStream {
  let parsed_args = parse_macro_input!(input with Punctuated::<LitStr, Token![,]>::parse_terminated);
  let package_filters: HashSet<String> = parsed_args.iter().map(|lit_str| lit_str.value()).collect();

  let mut generated_constants = proc_macro2::TokenStream::new();

  for enum_descriptor in DESCRIPTOR_POOL.all_enums() {
    let full_name = enum_descriptor.full_name();
    
    let should_include = if package_filters.is_empty() {
      true 
    } else {
      package_filters.iter().any(|filter| full_name.starts_with(filter))
    };

    if should_include {
      println!("Full Name: {}", full_name);
      let static_name_str = format!("__VALID_{}_VALUES", full_name.replace('.', "_").to_uppercase());
      let static_ident = Ident::new(&static_name_str, Span::call_site());

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

  generated_constants.into()
}


#[proc_macro_attribute]
pub fn protobuf_validate(args: TokenStream, input: TokenStream) -> TokenStream {
  let _ = args;

  let input_clone = input.clone();
  let _ast = parse_macro_input!(input_clone as DeriveInput);

  let struct_ident = _ast.ident.clone();

  // let struct_name = _ast.ident.to_string();
  // println!("{}", struct_name.to_string());

  let original_input_as_proc_macro2: proc_macro2::TokenStream = input.into();

  let validator_call_templates = extract_validators(_ast).unwrap();

  let output = quote! {
    #original_input_as_proc_macro2

    impl macro_impl::validators::WithValidator for #struct_ident {
      fn validate(&self) -> Result<(), proto_types::buf::validate::Violations> {
        let mut violations: Vec<proto_types::buf::validate::Violation> = Vec::new();
        let mut parent_messages: Vec<proto_types::buf::validate::FieldPathElement> = Vec::new();

        self.nested_validate(&mut parent_messages, &mut violations);

        if violations.len() > 0 {
          return Err(proto_types::buf::validate::Violations { violations });
        }
        Ok(())
      }

      fn nested_validate(
        &self,
        parent_messages: &mut Vec<proto_types::buf::validate::FieldPathElement>,
        violations: &mut Vec<proto_types::buf::validate::Violation>,
      ) {

        #(#validator_call_templates)*

      }
    }
  };

  // eprintln!("{}", output);

  output.into()
}

#[proc_macro_derive(
  ProtoMessage,
  attributes(field_num, reserved_nums, reserved_ranges, reserved_names, protoschema)
)]
pub fn proto_message_macro_derive(input: TokenStream) -> TokenStream {
  parse_proto_message(input)
}
