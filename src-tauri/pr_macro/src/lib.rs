use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;

use crate::protogen::parse_proto_message;
use crate::validator::core::extract_validators;

mod protogen;
mod validator;

#[proc_macro_attribute]
pub fn protobuf_validate(args: TokenStream, input: TokenStream) -> TokenStream {
  let _ = args;

  let input_clone = input.clone();
  let _ast = parse_macro_input!(input_clone as DeriveInput);

  let struct_ident = _ast.ident.clone();

  // let struct_name = _ast.ident.to_string();
  // println!("{}", struct_name.to_string());

  let original_input_as_proc_macro2: proc_macro2::TokenStream = input.into();

  let validator_tokens = extract_validators(_ast).unwrap();

  let output = quote! {
    #original_input_as_proc_macro2

    impl macro_impl::validators::WithValidator for #struct_ident {
      fn validate(&self) -> Result<(), proto_types::buf::validate::Violations> {
        let mut violations: Vec<proto_types::buf::validate::Violation> = Vec::new();
        #(#validator_tokens)*
        if violations.len() > 0 {
          return Err(proto_types::buf::validate::Violations { violations });
        }
        Ok(())
      }

      fn nested_validate(&self, parent_messages: &mut Vec<proto_types::buf::validate::FieldPathElement>, violations: &mut Vec<proto_types::buf::validate::Violation>) -> Result<(), proto_types::buf::validate::Violations> {
        if violations.len() > 0 {
          return Err(proto_types::buf::validate::Violations { violations });
        }
        Ok(())
      }
    }
  };

  eprintln!("{}", output);

  output.into()
}

#[proc_macro_derive(
  ProtoMessage,
  attributes(field_num, reserved_nums, reserved_ranges, reserved_names, protoschema)
)]
pub fn proto_message_macro_derive(input: TokenStream) -> TokenStream {
  parse_proto_message(input)
}
