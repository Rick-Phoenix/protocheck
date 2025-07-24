mod buf {
  pub mod validate {
    include!(concat!(env!("OUT_DIR"), "/buf.validate.rs"));
  }
}

mod google {
  pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
  }
}

pub use buf::validate as protovalidate;
pub use google::protobuf;
use proc_macro2::TokenStream as TokenStream2;
use protobuf::field_descriptor_proto::Type as ProtoType;
use protovalidate::{field_path_element::Subscript, FieldPathElement, Ignore};
use quote::{quote, ToTokens};

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

impl ToTokens for ProtoType {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let path = quote! { protocheck::types::protobuf::field_descriptor_proto::Type };

    match self {
      ProtoType::Double => tokens.extend(quote! { #path::Double }),
      ProtoType::Float => tokens.extend(quote! { #path::Float }),
      ProtoType::Int64 => tokens.extend(quote! { #path::Int64 }),
      ProtoType::Uint64 => tokens.extend(quote! { #path::Uint64 }),
      ProtoType::Int32 => tokens.extend(quote! { #path::Int32 }),
      ProtoType::Fixed64 => tokens.extend(quote! { #path::Fixed64 }),
      ProtoType::Fixed32 => tokens.extend(quote! { #path::Fixed32 }),
      ProtoType::Bool => tokens.extend(quote! { #path::Bool }),
      ProtoType::String => tokens.extend(quote! { #path::String }),
      ProtoType::Group => tokens.extend(quote! { #path::Group }),
      ProtoType::Message => tokens.extend(quote! { #path::Message }),
      ProtoType::Bytes => tokens.extend(quote! { #path::Bytes }),
      ProtoType::Uint32 => tokens.extend(quote! { #path::Uint32 }),
      ProtoType::Enum => tokens.extend(quote! { #path::Enum }),
      ProtoType::Sfixed32 => tokens.extend(quote! { #path::Sfixed32 }),
      ProtoType::Sfixed64 => tokens.extend(quote! { #path::Sfixed64 }),
      ProtoType::Sint32 => tokens.extend(quote! { #path::Sint32 }),
      ProtoType::Sint64 => tokens.extend(quote! { #path::Sint64 }),
    }
  }
}

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
