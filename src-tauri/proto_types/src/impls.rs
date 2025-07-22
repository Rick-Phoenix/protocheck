use crate::google::protobuf::field_descriptor_proto::Type as ProtoType;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

impl ToTokens for ProtoType {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let path = quote! { proto_types::google::protobuf::field_descriptor_proto::Type }; // The full path to the enum

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

use crate::buf::validate::Ignore;

impl ToTokens for Ignore {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let path = quote! { proto_types::buf::validate::Ignore };

    match self {
      Ignore::Unspecified => tokens.extend(quote! { #path::Unspecified }),
      Ignore::IfZeroValue => tokens.extend(quote! { #path::IfZeroValue }),
      Ignore::Always => tokens.extend(quote! { #path::Always }),
    }
  }
}

pub fn option_to_tokens<T>(option: &Option<T>) -> TokenStream
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

use crate::buf::validate::field_path_element::Subscript;

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

use crate::FieldData;

impl ToTokens for FieldData {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let rust_name = &self.rust_name;
    let proto_name = &self.proto_name;
    let tag = self.tag;
    let is_repeated = self.is_repeated;
    let is_map = self.is_map;
    let is_required = self.is_required;
    let is_optional = self.is_optional;
    let is_for_key = self.is_for_key;
    let proto_type = &self.proto_type;

    let key_type_tokens = option_to_tokens(&self.key_type);
    let value_type_tokens = option_to_tokens(&self.value_type);
    let ignore_tokens = option_to_tokens(&self.ignore);

    tokens.extend(quote! {
      proto_types::FieldData {
        rust_name: #rust_name.to_string(),
        proto_name: #proto_name.to_string(),
        proto_type: #proto_type,
        tag: #tag,
        is_repeated: #is_repeated,
        is_map: #is_map,
        is_required: #is_required,
        is_optional: #is_optional,
        is_for_key: #is_for_key,
        key_type: #key_type_tokens,
        value_type: #value_type_tokens,
        ignore: #ignore_tokens,
      }
    });
  }
}
