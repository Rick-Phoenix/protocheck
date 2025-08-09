use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::field_descriptor_proto::Type as ProtoType;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum FieldType {
  Double,
  Float,
  Int64,
  Uint64,
  Int32,
  Fixed64,
  Fixed32,
  Bool,
  String,
  Bytes,
  Uint32,
  Enum,
  Sfixed32,
  Sfixed64,
  Sint32,
  Sint64,
  Group,
  Message,
  Duration,
  Timestamp,
  Any,
}

impl FieldType {
  pub fn is_float(&self) -> bool {
    matches!(self, Self::Float) || matches!(self, Self::Double)
  }

  pub fn is_scalar(&self) -> bool {
    !matches!(
      self,
      Self::Message | Self::Duration | Self::Timestamp | Self::Any
    )
  }
}

impl From<FieldType> for ProtoType {
  fn from(value: FieldType) -> Self {
    match value {
      FieldType::Double => ProtoType::Double,
      FieldType::Float => ProtoType::Float,
      FieldType::Int32 => ProtoType::Int32,
      FieldType::Int64 => ProtoType::Int64,
      FieldType::Uint32 => ProtoType::Uint32,
      FieldType::Uint64 => ProtoType::Uint64,
      FieldType::Sint32 => ProtoType::Sint32,
      FieldType::Sint64 => ProtoType::Sint64,
      FieldType::Fixed32 => ProtoType::Fixed32,
      FieldType::Fixed64 => ProtoType::Fixed64,
      FieldType::Sfixed32 => ProtoType::Sfixed32,
      FieldType::Sfixed64 => ProtoType::Sfixed64,
      FieldType::Bool => ProtoType::Bool,
      FieldType::String => ProtoType::String,
      FieldType::Bytes => ProtoType::Bytes,
      FieldType::Message => ProtoType::Message,
      FieldType::Enum => ProtoType::Enum,
      FieldType::Group => ProtoType::Group,
      FieldType::Duration => ProtoType::Message,
      FieldType::Timestamp => ProtoType::Message,
      FieldType::Any => ProtoType::Message,
    }
  }
}

impl From<ProtoType> for FieldType {
  fn from(value: ProtoType) -> Self {
    match value {
      ProtoType::Double => FieldType::Double,
      ProtoType::Float => FieldType::Float,
      ProtoType::Int32 => FieldType::Int32,
      ProtoType::Int64 => FieldType::Int64,
      ProtoType::Uint32 => FieldType::Uint32,
      ProtoType::Uint64 => FieldType::Uint64,
      ProtoType::Sint32 => FieldType::Sint32,
      ProtoType::Sint64 => FieldType::Sint64,
      ProtoType::Fixed32 => FieldType::Fixed32,
      ProtoType::Fixed64 => FieldType::Fixed64,
      ProtoType::Sfixed32 => FieldType::Sfixed32,
      ProtoType::Sfixed64 => FieldType::Sfixed64,
      ProtoType::Bool => FieldType::Bool,
      ProtoType::String => FieldType::String,
      ProtoType::Bytes => FieldType::Bytes,
      ProtoType::Message => FieldType::Message,
      ProtoType::Enum => FieldType::Enum,
      ProtoType::Group => FieldType::Group,
    }
  }
}

impl From<FieldType> for i32 {
  fn from(val: FieldType) -> Self {
    match val {
      FieldType::Double => ProtoType::Double.into(),
      FieldType::Float => ProtoType::Float.into(),
      FieldType::Int32 => ProtoType::Int32.into(),
      FieldType::Int64 => ProtoType::Int64.into(),
      FieldType::Uint32 => ProtoType::Uint32.into(),
      FieldType::Uint64 => ProtoType::Uint64.into(),
      FieldType::Sint32 => ProtoType::Sint32.into(),
      FieldType::Sint64 => ProtoType::Sint64.into(),
      FieldType::Fixed32 => ProtoType::Fixed32.into(),
      FieldType::Fixed64 => ProtoType::Fixed64.into(),
      FieldType::Sfixed32 => ProtoType::Sfixed32.into(),
      FieldType::Sfixed64 => ProtoType::Sfixed64.into(),
      FieldType::Bool => ProtoType::Bool.into(),
      FieldType::String => ProtoType::String.into(),
      FieldType::Bytes => ProtoType::Bytes.into(),
      FieldType::Message => ProtoType::Message.into(),
      FieldType::Enum => ProtoType::Enum.into(),
      FieldType::Group => ProtoType::Group.into(),
      FieldType::Duration => ProtoType::Message.into(),
      FieldType::Timestamp => ProtoType::Message.into(),
      FieldType::Any => ProtoType::Message.into(),
    }
  }
}

impl ToTokens for FieldType {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let field_kind_path = quote! { protocheck::types::FieldType };

    let variant_tokens = match self {
      FieldType::Double => quote! { Double },
      FieldType::Float => quote! { Float },
      FieldType::Int64 => quote! { Int64 },
      FieldType::Uint64 => quote! { Uint64 },
      FieldType::Int32 => quote! { Int32 },
      FieldType::Fixed64 => quote! { Fixed64 },
      FieldType::Fixed32 => quote! { Fixed32 },
      FieldType::Bool => quote! { Bool },
      FieldType::String => quote! { String },
      FieldType::Group => quote! { Group },
      FieldType::Message => quote! { Message },
      FieldType::Duration => quote! { Duration },
      FieldType::Timestamp => quote! { Timestamp },
      FieldType::Any => quote! { Any },
      FieldType::Bytes => quote! { Bytes },
      FieldType::Uint32 => quote! { Uint32 },
      FieldType::Enum => quote! { Enum },
      FieldType::Sfixed32 => quote! { Sfixed32 },
      FieldType::Sfixed64 => quote! { Sfixed64 },
      FieldType::Sint32 => quote! { Sint32 },
      FieldType::Sint64 => quote! { Sint64 },
    };

    tokens.extend(quote! { #field_kind_path::#variant_tokens });
  }
}
