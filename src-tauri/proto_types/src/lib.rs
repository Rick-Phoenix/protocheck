use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

use crate::buf::validate::field_path_element::Subscript;
use crate::buf::validate::FieldPathElement;

pub mod macros;

pub mod buf {
  pub mod validate {
    include!(concat!(env!("OUT_DIR"), "/buf.validate.rs"));
  }
}

pub mod google {
  pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
  }
}

#[derive(Clone, Debug)]
pub struct FieldData {
  pub name: String,
  pub tag: u32,
  pub is_repeated: bool,
  pub is_map: bool,
  pub is_required: bool,
  pub subscript: Option<Subscript>,
  pub parent_elements: Vec<FieldPathElement>,
}

impl ToTokens for FieldData {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = &self.name;
    let tag = self.tag;
    let is_repeated = self.is_repeated;
    let is_map = self.is_map;
    let is_required = self.is_required;
    let subscript = &self.subscript;
    let parent_elements = &self.parent_elements;

    let subscript_expr = match subscript {
      Some(s) => quote! { ::core::option::Option::Some(#s) },
      None => quote! { ::core::option::Option::None },
    };

    tokens.extend(quote! {
        proto_types::FieldData {
            name: #name.to_string(),
            tag: #tag,
            is_repeated: #is_repeated,
            is_map: #is_map,
            is_required: #is_required,
            subscript: #subscript_expr,
            parent_elements: vec![#(#parent_elements),*],
        }
    });
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
      Some(name_str) => quote! { ::core::option::Option::Some(#name_str.clone()) },
      None => quote! { ::core::option::Option::None },
    };

    tokens.extend(quote! {
      proto_types::buf::validate::FieldPathElement {
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

impl ToTokens for Subscript {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    match self {
      Subscript::Index(value) => {
        tokens.extend(quote! {
            proto_types::buf::validate::Subscript::Index(#value)
        });
      }
      Subscript::BoolKey(value) => {
        tokens.extend(quote! {
            proto_types::buf::validate::Subscript::BoolKey(#value)
        });
      }
      Subscript::IntKey(value) => {
        tokens.extend(quote! {
            proto_types::buf::validate::Subscript::IntKey(#value)
        });
      }
      Subscript::UintKey(value) => {
        tokens.extend(quote! {
            proto_types::buf::validate::Subscript::UintKey(#value)
        });
      }
      Subscript::StringKey(value) => {
        tokens.extend(quote! {
            proto_types::buf::validate::Subscript::StringKey(#value)
        });
      }
    }
  }
}
