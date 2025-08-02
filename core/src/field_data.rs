use proc_macro2::TokenStream;
use prost_reflect::{FieldDescriptor, Kind};
use quote::{quote, ToTokens};

use crate::{
  protovalidate::{field_path_element::Subscript, FieldPathElement, Ignore},
  ProtoType,
};

#[derive(Clone, Debug)]
pub struct FieldContext<'a> {
  pub field_data: &'a FieldData,
  pub parent_elements: &'a [FieldPathElement],
  pub subscript: Option<Subscript>,
  pub key_type: Option<ProtoType>,
  pub value_type: Option<ProtoType>,
}

#[derive(Clone, Debug)]
pub enum FieldKind {
  Map,
  MapKey,
  MapValue,
  Repeated,
  RepeatedItem,
  Scalar,
  Duration,
  Timestamp,
  Message,
  FieldMask,
  Empty,
}

impl FieldKind {
  pub fn is_map(&self) -> bool {
    matches!(self, FieldKind::Map)
  }

  pub fn is_map_key(&self) -> bool {
    matches!(self, FieldKind::MapKey)
  }

  pub fn is_map_value(&self) -> bool {
    matches!(self, FieldKind::MapValue)
  }

  pub fn is_repeated(&self) -> bool {
    matches!(self, FieldKind::Repeated)
  }

  pub fn is_repeated_item(&self) -> bool {
    matches!(self, FieldKind::RepeatedItem)
  }

  pub fn is_scalar(&self) -> bool {
    matches!(self, FieldKind::Scalar)
  }

  pub fn is_timestamp(&self) -> bool {
    matches!(self, FieldKind::Timestamp)
  }

  pub fn is_duration(&self) -> bool {
    matches!(self, FieldKind::Duration)
  }

  pub fn is_message(&self) -> bool {
    matches!(self, FieldKind::Message)
  }

  pub fn has_validators(&self) -> bool {
    self.is_timestamp() || self.is_duration()
  }
}

impl ToTokens for FieldKind {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let field_kind_path = quote! { protocheck::field_data::FieldKind };

    let variant_tokens = match self {
      FieldKind::Map => quote! { Map },
      FieldKind::MapKey => quote! { MapKey },
      FieldKind::MapValue => quote! { MapValue },
      FieldKind::Repeated => quote! { Repeated },
      FieldKind::RepeatedItem => quote! { RepeatedItem },
      FieldKind::Scalar => quote! { Scalar },
      FieldKind::Duration => quote! { Duration },
      FieldKind::Timestamp => quote! { Timestamp },
      FieldKind::Message => quote! { Message },
      FieldKind::FieldMask => quote! { FieldMask },
      FieldKind::Empty => quote! { Empty },
    };

    tokens.extend(quote! { #field_kind_path::#variant_tokens });
  }
}

impl FieldKind {
  pub fn from_field_desc(field_desc: &FieldDescriptor) -> Self {
    if field_desc.is_map() {
      return Self::Map;
    } else if field_desc.is_list() {
      return Self::Repeated;
    }

    match field_desc.kind() {
      Kind::Message(message_desc) => match message_desc.full_name() {
        "google.protobuf.Duration" => Self::Duration,
        "google.protobuf.Timestamp" => Self::Timestamp,
        "google.protobuf.FieldMask" => Self::FieldMask,
        "google.protobuf.Empty" => Self::Empty,
        _ => Self::Message,
      },
      _ => Self::Scalar,
    }
  }

  pub fn from_inner_field_desc(field_desc: &FieldDescriptor) -> Self {
    match field_desc.kind() {
      Kind::Message(message_desc) => match message_desc.full_name() {
        "google.protobuf.Duration" => Self::Duration,
        "google.protobuf.Timestamp" => Self::Timestamp,
        "google.protobuf.FieldMask" => Self::FieldMask,
        "google.protobuf.Empty" => Self::Empty,
        _ => Self::Message,
      },
      _ => Self::Scalar,
    }
  }
}

#[derive(Clone, Debug)]
pub struct FieldData {
  pub rust_name: String,
  pub proto_name: String,
  pub tag: u32,
  pub kind: FieldKind,
  pub proto_type: ProtoType,
  pub ignore: Ignore,
}
