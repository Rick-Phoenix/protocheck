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
