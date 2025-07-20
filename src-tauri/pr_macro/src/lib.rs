use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;

use crate::protogen::parse_proto_message;
use crate::validator::extract_validators;

mod protogen;
mod validator;

#[proc_macro_attribute]
pub fn protobuf_validate(args: TokenStream, input: TokenStream) -> TokenStream {
  let _ = args;

  let input_clone = input.clone();
  let _ast = parse_macro_input!(input_clone as DeriveInput);

  let struct_name = _ast.ident.to_string();
  // println!("{}", struct_name.to_string());

  let original_input_as_proc_macro2: proc_macro2::TokenStream = input.into();

  extract_validators(_ast);

  quote! {
      #original_input_as_proc_macro2

    impl macro_impl::validators::WithValidator for User {
      fn validate(&self) -> Result<(), macro_impl::validators::buf::validate::Violation> {
        let check = macro_impl::validators::strings::max_len(&self.name, 1);
        match check {
          Ok(_) => Ok(()),
          Err(v) => Err(v)
        }
        // let program = cel_interpreter::Program::compile("this.name == 'Me'").unwrap();
        // let mut context = cel_interpreter::Context::default();
        //
        // context.add_variable("this", self).unwrap();
        //
        // let value = program.execute(&context).unwrap();
        //
        // match value {
        //   cel_interpreter::Value::Bool(val) => val,
        //   _ => {
        //     panic!("Expected a boolean")
        //   }
        // }
      }
    }
  }
  .into()
}

#[proc_macro_derive(
  ProtoMessage,
  attributes(field_num, reserved_nums, reserved_ranges, reserved_names, protoschema)
)]
pub fn proto_message_macro_derive(input: TokenStream) -> TokenStream {
  parse_proto_message(input)
}
