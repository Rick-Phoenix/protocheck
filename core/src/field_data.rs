use crate::{protovalidate::field_path_element::Subscript, ProtoType};

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

impl<'a> FieldContext<'a> {
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
use proto_types::protovalidate::FieldPathElement;
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
