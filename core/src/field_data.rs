use proc_macro2::TokenStream;
use proto_types::FieldType;
use quote::{quote, ToTokens};

use crate::{
  protovalidate::{field_path_element::Subscript, FieldPathElement, Ignore},
  ProtoType,
};

#[derive(Clone, Debug)]
pub struct FieldContext<'a> {
  pub field_data: &'a FieldData,
  pub parent_elements: &'a [FieldPathElement],
  pub subscript: &'a Option<Subscript>,
  pub key_type: &'a Option<ProtoType>,
  pub value_type: &'a Option<ProtoType>,
  pub field_kind: &'a FieldKind,
}

#[derive(Clone, Debug)]
pub enum FieldKind {
  MapKey(FieldType),
  MapValue(FieldType),
  RepeatedItem(FieldType),
  Single(FieldType),
}

impl FieldKind {
  pub fn inner_type(&self) -> &FieldType {
    match self {
      FieldKind::MapKey(field_type) => field_type,
      FieldKind::MapValue(field_type) => field_type,
      FieldKind::RepeatedItem(field_type) => field_type,
      FieldKind::Single(field_type) => field_type,
    }
  }
}

impl FieldKind {
  pub fn is_map_key(&self) -> bool {
    matches!(self, FieldKind::MapKey(_))
  }

  pub fn is_map_value(&self) -> bool {
    matches!(self, FieldKind::MapValue(_))
  }

  pub fn is_repeated_item(&self) -> bool {
    matches!(self, FieldKind::RepeatedItem(_))
  }

  pub fn is_scalar(&self) -> bool {
    matches!(self, FieldKind::Single(_))
  }
}

impl ToTokens for FieldKind {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let field_kind_path = quote! { protocheck::field_data::FieldKind };

    let variant_tokens = match self {
      FieldKind::MapKey(v) => quote! { MapKey(#v) },
      FieldKind::MapValue(v) => quote! { MapValue(#v) },
      FieldKind::RepeatedItem(v) => quote! { RepeatedItem(#v) },
      FieldKind::Single(v) => quote! { Single(#v) },
    };

    tokens.extend(quote! { #field_kind_path::#variant_tokens });
  }
}

#[derive(Clone, Debug)]
pub struct FieldData {
  pub rust_name: String,
  pub proto_name: String,
  pub tag: u32,
  pub ignore: Ignore,
}
