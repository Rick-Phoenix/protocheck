use quote::{quote, ToTokens};

use crate::{field_data::FieldData, TokenStream2};

pub fn option_to_tokens<T>(option: &Option<T>) -> TokenStream2
where
  T: ToTokens,
{
  match option {
    Some(value) => {
      quote! { ::core::option::Option::Some(#value) }
    }
    None => {
      quote! { ::core::option::Option::None }
    }
  }
}

pub fn option_string_to_tokens(option: &Option<String>) -> TokenStream2 {
  match option {
    Some(val) => {
      quote! { Some(#val.to_string()) }
    }
    None => {
      quote! { None }
    }
  }
}

impl ToTokens for FieldData {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let rust_name = &self.rust_name;
    let proto_name = &self.proto_name;
    let tag = self.tag;
    let ignore = &self.ignore;

    tokens.extend(quote! {
      protocheck::field_data::FieldData {
        rust_name: #rust_name.to_string(),
        proto_name: #proto_name.to_string(),
        tag: #tag,
        ignore: #ignore,
      }
    });
  }
}
