#![allow(clippy::len_without_is_empty)]
include!("./buf.validate.rs");

use crate::protovalidate::field_path_element::Subscript;

mod violations;

use std::fmt::{self, Display};

pub use violations::*;

impl Display for Subscript {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::BoolKey(val) => write!(f, "{val}"),
      Self::IntKey(val) => write!(f, "{val}"),
      Self::Index(val) | Self::UintKey(val) => write!(f, "{val}"),
      Self::StringKey(val) => write!(f, "{val}"),
    }
  }
}

#[cfg(feature = "totokens")]
mod totokens {
  use proc_macro2::TokenStream;
  use quote::{ToTokens, quote};

  use crate::protovalidate::{FieldPathElement, Ignore, field_path_element::Subscript};

  impl ToTokens for Ignore {
    fn to_tokens(&self, tokens: &mut TokenStream) {
      let path = quote! { ::protocheck::types::protovalidate::Ignore };

      match self {
        Self::Unspecified => tokens.extend(quote! { #path::Unspecified }),
        Self::IfZeroValue => tokens.extend(quote! { #path::IfZeroValue }),
        Self::Always => tokens.extend(quote! { #path::Always }),
      }
    }
  }

  impl ToTokens for Subscript {
    fn to_tokens(&self, tokens: &mut TokenStream) {
      match self {
        Self::Index(value) => {
          tokens.extend(quote! {
              ::protocheck::types::protovalidate::Subscript::Index(#value)
          });
        }
        Self::BoolKey(value) => {
          tokens.extend(quote! {
              ::protocheck::types::protovalidate::Subscript::BoolKey(#value)
          });
        }
        Self::IntKey(value) => {
          tokens.extend(quote! {
              ::protocheck::types::protovalidate::Subscript::IntKey(#value)
          });
        }
        Self::UintKey(value) => {
          tokens.extend(quote! {
              ::protocheck::types::protovalidate::Subscript::UintKey(#value)
          });
        }
        Self::StringKey(value) => {
          tokens.extend(quote! {
              ::protocheck::types::protovalidate::Subscript::StringKey(#value)
          });
        }
      }
    }
  }

  impl ToTokens for FieldPathElement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
      let field_number = &self.field_number;
      let field_name = &self.field_name;
      let field_type = &self.field_type;
      let key_type = &self.key_type;
      let value_type = &self.value_type;
      let subscript = &self.subscript;

      let field_name_expr = match field_name {
        Some(name_str) => quote! { Some(#name_str.clone()) },
        None => quote! { None },
      };

      tokens.extend(quote! {
        ::protocheck::types::protovalidate::FieldPathElement {
          field_number: #field_number,
          field_name: #field_name_expr,
          field_type: #field_type,
          key_type: #key_type,
          value_type: #value_type,
          subscript: #subscript,
        }
      });
    }
  }
}
