pub mod validate {
  use field_path_element::Subscript;
  use quote::{quote, ToTokens};

  use crate::TokenStream2;

  include!(concat!(env!("OUT_DIR"), "/buf.validate.rs"));

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
}
