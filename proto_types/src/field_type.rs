use crate::field_descriptor_proto::Type as ProtoType;

/// This is an enhanced enum for protobuf types, allowing you to identify well known types such as Any, Timestamp or Duration more precisely. It is non-exhaustive because other well known types can be added in the future, but it is safe to assume that any non matching variant can will be of the `Message` type.
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
#[non_exhaustive]
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
  #[must_use]
  pub const fn is_scalar(&self) -> bool {
    !matches!(
      self,
      Self::Message | Self::Duration | Self::Timestamp | Self::Any
    )
  }

  /// Returns the short, lowercase name for the field type.
  #[must_use]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::Double => "double",
      Self::Float => "float",
      Self::Int64 => "int64",
      Self::Uint64 => "uint64",
      Self::Int32 => "int32",
      Self::Fixed64 => "fixed64",
      Self::Fixed32 => "fixed32",
      Self::Bool => "bool",
      Self::String => "string",
      Self::Bytes => "bytes",
      Self::Uint32 => "uint32",
      Self::Enum => "enum",
      Self::Sfixed32 => "sfixed32",
      Self::Sfixed64 => "sfixed64",
      Self::Sint32 => "sint32",
      Self::Sint64 => "sint64",
      Self::Group => "group",
      Self::Message => "message",
      Self::Duration => "duration",
      Self::Timestamp => "timestamp",
      Self::Any => "any",
    }
  }

  /// Returns the full name for the field type, using the
  /// fully-qualified name for Google's well-known types.
  #[must_use]
  pub const fn full_name(&self) -> &'static str {
    match self {
      // Special cases for well-known types
      Self::Duration => "google.protobuf.Duration",
      Self::Timestamp => "google.protobuf.Timestamp",
      Self::Any => "google.protobuf.Any",

      // For all other types, it falls back to the short name.
      // The `_` arm catches all variants not matched above.
      _ => self.name(),
    }
  }
}

impl From<FieldType> for ProtoType {
  fn from(value: FieldType) -> Self {
    match value {
      FieldType::Double => Self::Double,
      FieldType::Float => Self::Float,
      FieldType::Int32 => Self::Int32,
      FieldType::Int64 => Self::Int64,
      FieldType::Uint32 => Self::Uint32,
      FieldType::Uint64 => Self::Uint64,
      FieldType::Sint32 => Self::Sint32,
      FieldType::Sint64 => Self::Sint64,
      FieldType::Fixed32 => Self::Fixed32,
      FieldType::Fixed64 => Self::Fixed64,
      FieldType::Sfixed32 => Self::Sfixed32,
      FieldType::Sfixed64 => Self::Sfixed64,
      FieldType::Bool => Self::Bool,
      FieldType::String => Self::String,
      FieldType::Bytes => Self::Bytes,
      FieldType::Message | FieldType::Duration | FieldType::Timestamp | FieldType::Any => {
        Self::Message
      }
      FieldType::Enum => Self::Enum,
      FieldType::Group => Self::Group,
    }
  }
}

impl From<ProtoType> for FieldType {
  fn from(value: ProtoType) -> Self {
    match value {
      ProtoType::Double => Self::Double,
      ProtoType::Float => Self::Float,
      ProtoType::Int32 => Self::Int32,
      ProtoType::Int64 => Self::Int64,
      ProtoType::Uint32 => Self::Uint32,
      ProtoType::Uint64 => Self::Uint64,
      ProtoType::Sint32 => Self::Sint32,
      ProtoType::Sint64 => Self::Sint64,
      ProtoType::Fixed32 => Self::Fixed32,
      ProtoType::Fixed64 => Self::Fixed64,
      ProtoType::Sfixed32 => Self::Sfixed32,
      ProtoType::Sfixed64 => Self::Sfixed64,
      ProtoType::Bool => Self::Bool,
      ProtoType::String => Self::String,
      ProtoType::Bytes => Self::Bytes,
      ProtoType::Message => Self::Message,
      ProtoType::Enum => Self::Enum,
      ProtoType::Group => Self::Group,
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
      FieldType::Message | FieldType::Duration | FieldType::Timestamp | FieldType::Any => {
        ProtoType::Message.into()
      }
      FieldType::Enum => ProtoType::Enum.into(),
      FieldType::Group => ProtoType::Group.into(),
    }
  }
}

#[cfg(feature = "totokens")]
mod totokens {
  use proc_macro2::TokenStream;
  use quote::{ToTokens, quote};

  use crate::FieldType;

  impl ToTokens for FieldType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
      let field_kind_path = quote! { ::protocheck::types::FieldType };

      let variant_tokens = match self {
        Self::Double => quote! { Double },
        Self::Float => quote! { Float },
        Self::Int64 => quote! { Int64 },
        Self::Uint64 => quote! { Uint64 },
        Self::Int32 => quote! { Int32 },
        Self::Fixed64 => quote! { Fixed64 },
        Self::Fixed32 => quote! { Fixed32 },
        Self::Bool => quote! { Bool },
        Self::String => quote! { String },
        Self::Group => quote! { Group },
        Self::Message => quote! { Message },
        Self::Duration => quote! { Duration },
        Self::Timestamp => quote! { Timestamp },
        Self::Any => quote! { Any },
        Self::Bytes => quote! { Bytes },
        Self::Uint32 => quote! { Uint32 },
        Self::Enum => quote! { Enum },
        Self::Sfixed32 => quote! { Sfixed32 },
        Self::Sfixed64 => quote! { Sfixed64 },
        Self::Sint32 => quote! { Sint32 },
        Self::Sint64 => quote! { Sint64 },
      };

      tokens.extend(quote! { #field_kind_path::#variant_tokens });
    }
  }
}
