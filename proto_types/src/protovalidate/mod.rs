include!(concat!(env!("OUT_DIR"), "/buf.validate.rs"));

use crate::protovalidate::field_path_element::Subscript;

#[cfg(feature = "protocheck")]
mod length_rules;

#[cfg(feature = "protocheck")]
pub use length_rules::*;

#[cfg(feature = "protocheck")]
mod substring_rules;

#[cfg(feature = "protocheck")]
pub use substring_rules::*;

#[cfg(feature = "protocheck")]
mod comparable_rules;

#[cfg(feature = "protocheck")]
mod containing_rules;

#[cfg(feature = "protocheck")]
mod into_comparable;

#[cfg(feature = "protocheck")]
mod numeric_rules;

#[cfg(feature = "protocheck")]
mod rule_matching;

#[cfg(feature = "protocheck")]
mod const_rules;

#[cfg(feature = "protocheck")]
pub use const_rules::*;

mod violations;

use std::fmt::{self, Display};

#[cfg(feature = "protocheck")]
pub use comparable_rules::*;
#[cfg(feature = "protocheck")]
pub use containing_rules::{ContainingRules, ItemList};
#[cfg(feature = "protocheck")]
pub use numeric_rules::NumericRules;

impl Display for Subscript {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Subscript::Index(val) => write!(f, "{}", val),
      Subscript::BoolKey(val) => write!(f, "{}", val),
      Subscript::IntKey(val) => write!(f, "{}", val),
      Subscript::UintKey(val) => write!(f, "{}", val),
      Subscript::StringKey(val) => write!(f, "{}", val),
    }
  }
}

#[cfg(feature = "totokens")]
mod totokens {
  use proc_macro2::TokenStream;
  use quote::{quote, ToTokens};

  use crate::protovalidate::{field_path_element::Subscript, FieldPathElement, Ignore};

  impl ToTokens for Ignore {
    fn to_tokens(&self, tokens: &mut TokenStream) {
      let path = quote! { ::protocheck::types::protovalidate::Ignore };

      match self {
        Ignore::Unspecified => tokens.extend(quote! { #path::Unspecified }),
        Ignore::IfZeroValue => tokens.extend(quote! { #path::IfZeroValue }),
        Ignore::Always => tokens.extend(quote! { #path::Always }),
      }
    }
  }

  impl ToTokens for Subscript {
    fn to_tokens(&self, tokens: &mut TokenStream) {
      match self {
        Subscript::Index(value) => {
          tokens.extend(quote! {
              ::protocheck::types::protovalidate::Subscript::Index(#value)
          });
        }
        Subscript::BoolKey(value) => {
          tokens.extend(quote! {
              ::protocheck::types::protovalidate::Subscript::BoolKey(#value)
          });
        }
        Subscript::IntKey(value) => {
          tokens.extend(quote! {
              ::protocheck::types::protovalidate::Subscript::IntKey(#value)
          });
        }
        Subscript::UintKey(value) => {
          tokens.extend(quote! {
              ::protocheck::types::protovalidate::Subscript::UintKey(#value)
          });
        }
        Subscript::StringKey(value) => {
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
