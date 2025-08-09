use proc_macro2::TokenStream;
use proto_types::FieldType;
use quote::{quote, ToTokens};

use crate::{
  protovalidate::{field_path_element::Subscript, FieldPathElement},
  ProtoType,
};

#[derive(Clone, Debug)]
pub struct FieldContext<'a> {
  pub rust_name: &'a str,
  pub proto_name: &'a str,
  pub tag: u32,
  pub parent_elements: &'a [FieldPathElement],
  pub subscript: Option<Subscript>,
  pub key_type: Option<ProtoType>,
  pub value_type: Option<ProtoType>,
  pub field_kind: FieldKind,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum FieldKind {
  Map(FieldType),
  MapKey(FieldType),
  MapValue(FieldType),
  Repeated(FieldType),
  RepeatedItem(FieldType),
  Single(FieldType),
}

impl FieldKind {
  pub fn inner_type(&self) -> FieldType {
    match self {
      FieldKind::Map(field_type) => *field_type,
      FieldKind::MapKey(field_type) => *field_type,
      FieldKind::MapValue(field_type) => *field_type,
      FieldKind::Repeated(field_type) => *field_type,
      FieldKind::RepeatedItem(field_type) => *field_type,
      FieldKind::Single(field_type) => *field_type,
    }
  }

  pub fn is_copy(&self) -> bool {
    !matches!(
      self.inner_type(),
      FieldType::String | FieldType::Message | FieldType::Bytes | FieldType::Any
    )
  }

  pub fn is_map_key(&self) -> bool {
    matches!(self, FieldKind::MapKey(_))
  }

  pub fn is_map_value(&self) -> bool {
    matches!(self, FieldKind::MapValue(_))
  }

  pub fn is_repeated_item(&self) -> bool {
    matches!(self, FieldKind::RepeatedItem(_))
  }

  pub fn is_in_loop(&self) -> bool {
    self.is_map_key() || self.is_map_value() || self.is_repeated_item()
  }
}

impl ToTokens for FieldKind {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let field_kind_path = quote! { protocheck::field_data::FieldKind };

    let variant_tokens = match self {
      FieldKind::Map(v) => quote! { Map(#v) },
      FieldKind::MapKey(v) => quote! { MapKey(#v) },
      FieldKind::MapValue(v) => quote! { MapValue(#v) },
      FieldKind::Repeated(v) => quote! { Repeated(#v) },
      FieldKind::RepeatedItem(v) => quote! { RepeatedItem(#v) },
      FieldKind::Single(v) => quote! { Single(#v) },
    };

    tokens.extend(quote! { #field_kind_path::#variant_tokens });
  }
}
