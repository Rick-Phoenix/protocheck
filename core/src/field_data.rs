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
  pub field_type: ProtoType,
  pub value_type: Option<ProtoType>,
  pub field_kind: FieldKind,
}

/// The kind of field being validated. This extra context helps generating more precise violation reports.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum FieldKind {
  Map,
  MapKey,
  MapValue,
  Repeated,
  RepeatedItem,
  Single,
}

impl FieldKind {
  pub fn is_map_key(&self) -> bool {
    matches!(self, FieldKind::MapKey)
  }

  pub fn is_map_value(&self) -> bool {
    matches!(self, FieldKind::MapValue)
  }

  pub fn is_repeated_item(&self) -> bool {
    matches!(self, FieldKind::RepeatedItem)
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
      FieldKind::Map => quote! { Map },
      FieldKind::MapKey => quote! { MapKey },
      FieldKind::MapValue => quote! { MapValue },
      FieldKind::Repeated => quote! { Repeated },
      FieldKind::RepeatedItem => quote! { RepeatedItem },
      FieldKind::Single => quote! { Single },
    };

    tokens.extend(quote! { #field_kind_path::#variant_tokens });
  }
}
