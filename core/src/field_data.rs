use crate::{ProtoType, protovalidate::field_path_element::Subscript};

/// The context for the field being validated.
#[derive(Clone, Debug)]
pub struct FieldContext<'a> {
  pub proto_name: &'a str,
  pub tag: i32,
  pub subscript: Option<Subscript>,
  pub key_type: Option<ProtoType>,
  pub field_type: ProtoType,
  pub value_type: Option<ProtoType>,
  pub field_kind: FieldKind,
}

impl FieldContext<'_> {
  #[must_use]
  pub fn as_path_element(&self) -> FieldPathElement {
    FieldPathElement {
      field_number: Some(self.tag),
      field_name: Some(self.proto_name.to_string()),
      field_type: Some(self.field_type as i32),
      key_type: self.key_type.map(|t| t as i32),
      value_type: self.value_type.map(|t| t as i32),
      subscript: self.subscript.clone(),
    }
  }
}

/// The kind of field being validated. This extra context helps generating more precise violation reports.
#[derive(Clone, Default, Debug, Copy, PartialEq, Eq)]
pub enum FieldKind {
  Map,
  MapKey,
  MapValue,
  Repeated,
  RepeatedItem,
  #[default]
  Single,
}

impl FieldKind {
  #[must_use]
  pub const fn is_map_key(&self) -> bool {
    matches!(self, Self::MapKey)
  }

  #[must_use]
  pub const fn is_map_value(&self) -> bool {
    matches!(self, Self::MapValue)
  }

  #[must_use]
  pub const fn is_repeated_item(&self) -> bool {
    matches!(self, Self::RepeatedItem)
  }
}

#[cfg(feature = "totokens")]
use proc_macro2::TokenStream;
use proto_types::protovalidate::FieldPathElement;
#[cfg(feature = "totokens")]
use quote::{ToTokens, quote};

#[cfg(feature = "totokens")]
impl ToTokens for FieldKind {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let field_kind_path = quote! { ::protocheck::field_data::FieldKind };

    let variant_tokens = match self {
      Self::Map => quote! { Map },
      Self::MapKey => quote! { MapKey },
      Self::MapValue => quote! { MapValue },
      Self::Repeated => quote! { Repeated },
      Self::RepeatedItem => quote! { RepeatedItem },
      Self::Single => quote! { Single },
    };

    tokens.extend(quote! { #field_kind_path::#variant_tokens });
  }
}
