use proc_macro::TokenStream;
pub(crate) use proc_macro2::{Ident as Ident2, Span as Span2, TokenStream as TokenStream2};

mod into_cel;

#[proc_macro_derive(TryIntoCelValue)]
pub fn try_into_cel_value_derive(input: TokenStream) -> TokenStream {
  into_cel::derive_cel_value_struct(input)
}
