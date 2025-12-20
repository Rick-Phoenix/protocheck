#[macro_use]
mod macros;
mod cel_try_into;

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use syn::{Error, Ident, Item, ItemEnum, ItemStruct, Type, parse_macro_input};

#[proc_macro_derive(TryIntoCelValue)]
pub fn try_into_cel_value_derive(input: TokenStream) -> TokenStream {
  let item = parse_macro_input!(input as Item);

  let result = match item {
    Item::Struct(s) => cel_try_into::derive_cel_value_struct(s),
    Item::Enum(e) => cel_try_into::derive_cel_value_oneof(e),
    _ => {
      return error!(
        item,
        "The TryIntoCelValue macro only works on enums and structs"
      )
      .into_compile_error()
      .into();
    }
  };

  match result {
    Ok(tokens) => tokens.into(),
    Err(e) => e.into_compile_error().into(),
  }
}
