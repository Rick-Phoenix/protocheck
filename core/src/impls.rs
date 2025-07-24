use quote::{quote, ToTokens};

use crate::{
  field_data::FieldData,
  protovalidate::{field_path_element::Subscript, FieldPathElement, Ignore},
  ProtoType, TokenStream2,
};

pub fn option_to_tokens<T>(option: &Option<T>) -> TokenStream2
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

pub fn option_string_to_tokens(option: &Option<String>) -> TokenStream2 {
  match option {
    Some(val) => {
      quote! { Some(#val.to_string()) }
    }
    None => {
      quote! { None }
    }
  }
}

impl ToTokens for FieldData {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let rust_name = &self.rust_name;
    let proto_name = &self.proto_name;
    let tag = self.tag;
    let is_repeated = self.is_repeated;
    let is_repeated_item = self.is_repeated_item;
    let is_map = self.is_map;
    let is_map_key = self.is_map_key;
    let is_map_value = self.is_map_value;
    let is_required = self.is_required;
    let is_optional = self.is_optional;
    let proto_type = &self.proto_type;
    let ignore = &self.ignore;

    let enum_tokens = option_string_to_tokens(&self.enum_full_name);
    let key_type_tokens = option_to_tokens(&self.key_type);
    let value_type_tokens = option_to_tokens(&self.value_type);

    tokens.extend(quote! {
      proto_types::FieldData {
        rust_name: #rust_name.to_string(),
        proto_name: #proto_name.to_string(),
        proto_type: #proto_type,
        tag: #tag,
        is_repeated: #is_repeated,
        is_repeated_item: #is_repeated_item,
        is_map: #is_map,
        is_map_key: #is_map_key,
        is_map_value: #is_map_value,
        is_required: #is_required,
        is_optional: #is_optional,
        key_type: #key_type_tokens,
        value_type: #value_type_tokens,
        enum_full_name: #enum_tokens,
        ignore: #ignore,
      }
    });
  }
}
