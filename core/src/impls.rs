use quote::{quote, ToTokens};

use crate::TokenStream2;

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
