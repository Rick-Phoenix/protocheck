include!(concat!(env!("OUT_DIR"), "/buf.validate.rs"));

use quote::{quote, ToTokens};

use crate::protovalidate::field_path_element::Subscript;

mod comparable_rules;
mod containing_rules;
mod into_comparable;
mod numeric_rules;
mod rule_matching;
mod violations;

use std::fmt::{self, Display};

pub use comparable_rules::{
  ComparableGreaterThan, ComparableLessThan, ComparableRules, LengthRules,
};
pub use containing_rules::ContainingRules;
pub use numeric_rules::NumericRules;

use crate::TokenStream2;

impl ToTokens for Ignore {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let path = quote! { protocheck::types::protovalidate::Ignore };

    match self {
      Ignore::Unspecified => tokens.extend(quote! { #path::Unspecified }),
      Ignore::IfZeroValue => tokens.extend(quote! { #path::IfZeroValue }),
      Ignore::Always => tokens.extend(quote! { #path::Always }),
    }
  }
}

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

impl ToTokens for Subscript {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    match self {
      Subscript::Index(value) => {
        tokens.extend(quote! {
            protocheck::types::protovalidate::Subscript::Index(#value)
        });
      }
      Subscript::BoolKey(value) => {
        tokens.extend(quote! {
            protocheck::types::protovalidate::Subscript::BoolKey(#value)
        });
      }
      Subscript::IntKey(value) => {
        tokens.extend(quote! {
            protocheck::types::protovalidate::Subscript::IntKey(#value)
        });
      }
      Subscript::UintKey(value) => {
        tokens.extend(quote! {
            protocheck::types::protovalidate::Subscript::UintKey(#value)
        });
      }
      Subscript::StringKey(value) => {
        tokens.extend(quote! {
            protocheck::types::protovalidate::Subscript::StringKey(#value)
        });
      }
    }
  }
}

impl ToTokens for FieldPathElement {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
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
      protocheck::types::protovalidate::FieldPathElement {
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
