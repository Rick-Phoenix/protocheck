use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use quote::ToTokens;

use crate::buf::validate::field_path_element::Subscript;
use crate::buf::validate::FieldPathElement;

pub mod macros;

pub mod buf {
  pub mod validate {
    include!(concat!(env!("OUT_DIR"), "/buf.validate.rs"));
  }
}

pub mod google {
  pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
  }
}

#[derive(Clone, Debug)]
pub struct FieldData {
  pub name: String,
  pub tag: u32,
  pub is_repeated: bool,
  pub is_map: bool,
  pub is_required: bool,
  pub subscript: Option<Subscript>,
  pub parent_elements: Vec<FieldPathElement>,
}

impl ToTokens for FieldData {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = &self.name;
    let tag = self.tag;
    let is_repeated = self.is_repeated;
    let is_map = self.is_map;
    let is_required = self.is_required;
    let subscript = &self.subscript;
    let parent_elements = &self.parent_elements;

    let subscript_expr = match subscript {
      Some(s) => quote! { ::core::option::Option::Some(#s) },
      None => quote! { ::core::option::Option::None },
    };

    tokens.extend(quote! {
        proto_types::FieldData {
            name: #name.to_string(),
            tag: #tag,
            is_repeated: #is_repeated,
            is_map: #is_map,
            is_required: #is_required,
            subscript: #subscript_expr,
            parent_elements: vec![#(#parent_elements),*],
        }
    });
  }
}

impl ToTokens for FieldPathElement {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let field_number = &self.field_number;
    let field_name = &self.field_name;
    let field_type = &self.field_type;
    let key_type = &self.key_type;
    let value_type = &self.value_type;
    let subscript = &self.subscript;

    let field_name_expr = match field_name {
      Some(name_str) => quote! { ::core::option::Option::Some(#name_str.clone()) },
      None => quote! { ::core::option::Option::None },
    };

    tokens.extend(quote! {
      proto_types::buf::validate::FieldPathElement {
            field_number: #field_number,
            field_name: #field_name_expr,
            field_type: #field_type,
            key_type: #key_type,
            value_type: #value_type,
            subscript: #subscript,
        }
    });
  }
}

impl ToTokens for Subscript {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    match self {
      Subscript::Index(value) => {
        tokens.extend(quote! {
            proto_types::buf::validate::Subscript::Index(#value)
        });
      }
      Subscript::BoolKey(value) => {
        tokens.extend(quote! {
            proto_types::buf::validate::Subscript::BoolKey(#value)
        });
      }
      Subscript::IntKey(value) => {
        tokens.extend(quote! {
            proto_types::buf::validate::Subscript::IntKey(#value)
        });
      }
      Subscript::UintKey(value) => {
        tokens.extend(quote! {
            proto_types::buf::validate::Subscript::UintKey(#value)
        });
      }
      Subscript::StringKey(value) => {
        tokens.extend(quote! {
            proto_types::buf::validate::Subscript::StringKey(#value)
        });
      }
    }
  }
}

pub struct ValidatorCallTemplate {
  pub validator_path: TokenStream,
  pub target_value_tokens: TokenStream,
  pub base_field_data: FieldData,
  pub violation_rule_id: String,
}

impl ToTokens for ValidatorCallTemplate {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let validator = &self.validator_path;
    let target = &self.target_value_tokens;
    let violation_rule_id_str = &self.violation_rule_id;

    let field_name_str = &self.base_field_data.name;
    let field_tag = self.base_field_data.tag;
    let field_rust_ident = Ident::new(field_name_str, Span::call_site());
    let field_is_required = self.base_field_data.is_required;
    let field_is_map = self.base_field_data.is_map;

    let is_repeated_field = self.base_field_data.is_repeated;

    // Create an Ident for `parent_messages` that will exist in the generated code
    let parent_messages_ident = Ident::new("parent_messages", Span::call_site());
    let violations_ident = Ident::new("violations", Span::call_site());


    if is_repeated_field {
      let item_ident = Ident::new("item", Span::call_site());
      let index_ident = Ident::new("idx", Span::call_site());

      tokens.extend(quote! {
                for (#index_ident, #item_ident) in self.#field_rust_ident.iter().enumerate() {
                    let mut current_item_parent_elements = #parent_messages_ident.clone();

                    let current_field_path_element = proto_types::buf::validate::FieldPathElement {
                        field_name: Some(#field_name_str.to_string()),
                        field_number: Some(#field_tag as i32),
                        field_type: Some(proto_types::google::protobuf::field_descriptor_proto::Type::String as i32),
                        key_type: None, 
                        value_type: None, 
                        subscript: Some(proto_types::buf::validate::field_path_element::Subscript::Index(#index_ident as u64)),
                    };

                    current_item_parent_elements.push(current_field_path_element);

                    let item_field_data = proto_types::FieldData {
                        name: #field_name_str.to_string(),
                        tag: #field_tag,
                        is_repeated: false,
                        is_map: false,
                        is_required: #field_is_required,
                        subscript: Some(proto_types::buf::validate::field_path_element::Subscript::Index(#index_ident as u64)),
                        parent_elements: current_item_parent_elements,
                    };

                    match #validator(item_field_data, #item_ident, #target) {
                        Ok(_) => {},
                        Err(mut v) => {
                           v.rule_id = Some(format!("{}.{}", #violation_rule_id_str, v.rule_id.unwrap_or_default()));
                           #violations_ident.push(v);
                        },
                    };
                }
            });
    } else {
      tokens.extend(quote! {
                let mut current_field_parent_elements = #parent_messages_ident.clone();

                let current_field_path_element = proto_types::buf::validate::FieldPathElement {
                    field_name: Some(#field_name_str.to_string()),
                    field_number: Some(#field_tag as i32),
                    field_type: Some(proto_types::google::protobuf::field_descriptor_proto::Type::String as i32), 
                    key_type: None, 
                    value_type: None, 
                    subscript: None,
                };

                current_field_parent_elements.push(current_field_path_element);

                let field_data_for_call = proto_types::FieldData {
                    name: #field_name_str.to_string(),
                    tag: #field_tag,
                    is_repeated: false,
                    is_map: #field_is_map,
                    is_required: #field_is_required,
                    subscript: None,
                    parent_elements: current_field_parent_elements,
                };

                match #validator(field_data_for_call, &self.#field_rust_ident, #target) {
                    Ok(_) => {},
                    Err(mut v) => {
                        v.rule_id = Some(format!("{}.{}", #violation_rule_id_str, v.rule_id.unwrap_or_default()));
                        #violations_ident.push(v);
                    },
                };
            });
    }
  }
}
