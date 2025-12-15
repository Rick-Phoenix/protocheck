use proto_types::FieldType;

use crate::{
  protovalidate::{field_path_element::Subscript, FieldPathElement},
  ProtoType,
};

/// The context for the field being validated.
#[derive(Clone, Debug)]
pub struct FieldContext<'a> {
  pub proto_name: &'a str,
  pub tag: i32,
  pub parent_elements: &'a [FieldPathElement],
  pub subscript: Option<Subscript>,
  pub key_type: Option<ProtoType>,
  pub value_type: Option<ProtoType>,
  pub field_kind: FieldKind,
}

/// The kind of field being validated. This extra context helps generating more precise violation reports.
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
    !matches!(self, Self::Map(_) | Self::Repeated(_))
      && !matches!(
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
}

#[cfg(feature = "totokens")]
use proc_macro2::TokenStream;
#[cfg(feature = "totokens")]
use quote::{quote, ToTokens};

#[cfg(feature = "totokens")]
impl ToTokens for FieldKind {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let field_kind_path = quote! { ::protocheck::field_data::FieldKind };

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
