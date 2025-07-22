use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use quote::ToTokens;
use google::protobuf::field_descriptor_proto::Type as ProtoType;

use crate::buf::validate::field_path_element::Subscript;
use crate::buf::validate::FieldPathElement;

pub mod macros;
pub mod impls;

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
pub struct FieldData<'a> {
  pub name: String,
  pub tag: u32,
  pub is_repeated: bool,
  pub is_map: bool,
  pub is_required: bool,
  pub subscript: Option<Subscript>,
  pub parent_elements: &'a [FieldPathElement],
  pub for_key: bool,
  pub key_type: Option<ProtoType>,   
  pub value_type: Option<ProtoType>,
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

#[derive(Debug)]
pub enum GeneratedCodeKind {
  FieldRule,
  NestedMessageRecursion {
    is_optional: bool,
    is_repeated: bool,
  },
}

#[derive(Debug)]
pub struct ValidatorCallTemplate {
  pub validator_path: Option<TokenStream>,
  pub target_value_tokens: Option<TokenStream>,

  pub field_rust_ident_str: String,
  pub field_tag: u32,
  pub field_proto_name: String,
  pub field_proto_type: ProtoType,
  pub field_is_repeated: bool,
  pub field_is_map: bool,
  pub field_is_required: bool,
  pub for_key: bool,
  pub key_type: Option<ProtoType>,
  pub value_type: Option<ProtoType>,

  pub kind: GeneratedCodeKind,
}

impl ToTokens for ValidatorCallTemplate {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let field_rust_ident = Ident::new(&self.field_rust_ident_str, Span::call_site());
    let field_name_str = &self.field_proto_name;
    let field_tag = self.field_tag;
    let field_proto_type_val = self.field_proto_type as i32;
    let field_is_required = self.field_is_required;
    let violations_ident = Ident::new("violations", Span::call_site());
    let for_key = self.for_key;
    let key_type = self.key_type;
    let value_type = self.value_type;
    let parent_messages_ident = Ident::new("parent_messages", Span::call_site());

    match &self.kind {
      GeneratedCodeKind::FieldRule => {
        let validator = self.validator_path.as_ref().unwrap();
        let target = self.target_value_tokens.as_ref().unwrap();

        if self.field_is_map { 
          let value_ident = Ident::new("value", Span::call_site());
          let key_ident = Ident::new("key", Span::call_site());

          let key_subscript_gen = if let Some(key_type_enum) = key_type {
            generate_key_subscript(key_type_enum, &key_ident)
        } else {
          quote! { proto_types::buf::validate::field_path_element::Subscript::StringKey("unknown_key_type".to_string()) }
        };
          if for_key {
            tokens.extend(quote! {
              let current_item_parent_elements = #parent_messages_ident.as_slice();
              for (#key_ident, #value_ident) in self.#field_rust_ident.iter() {
                let item_field_data = proto_types::FieldData {
                  name: #field_name_str.to_string(),
                  tag: #field_tag,
                  is_repeated: false,
                  is_map: false,
                  is_required: #field_is_required,
                  for_key: true,
                  subscript: Some(#key_subscript_gen),
                  parent_elements: current_item_parent_elements,
                  key_type: Some(#key_type),
                  value_type: Some(#value_type),
                };
                match #validator(item_field_data, #key_ident, #target) {
                  Ok(_) => {},
                  Err(v) => {
                     #violations_ident.push(v);
                  },
                };
              }
            });
          } else {
              tokens.extend(quote! {
              let current_item_parent_elements = #parent_messages_ident.as_slice();
              for (#key_ident, #value_ident) in self.#field_rust_ident.iter() {
                let item_field_data = proto_types::FieldData {
                  name: #field_name_str.to_string(),
                  tag: #field_tag,
                  is_repeated: false,
                  is_map: false,
                  is_required: #field_is_required,
                  subscript: Some(#key_subscript_gen),
                  parent_elements: current_item_parent_elements,
                  for_key: false,
                  key_type: Some(#key_type),
                  value_type: Some(#value_type),
                };
                match #validator(item_field_data, #value_ident, #target) {
                  Ok(_) => {},
                  Err(v) => {
                     #violations_ident.push(v);
                  },
                };
              }
            });
          }
        }
        else if self.field_is_repeated {
          let item_ident = Ident::new("item", Span::call_site());
          let index_ident = Ident::new("idx", Span::call_site());

          tokens.extend(quote! {
            let current_item_parent_elements = #parent_messages_ident.as_slice();
            for (#index_ident, #item_ident) in self.#field_rust_ident.iter().enumerate() {
              let item_field_data = proto_types::FieldData {
                name: #field_name_str.to_string(),
                tag: #field_tag,
                is_repeated: false,
                is_map: false,
                is_required: #field_is_required,
                subscript: Some(proto_types::buf::validate::field_path_element::Subscript::Index(#index_ident as u64)),
                parent_elements: current_item_parent_elements,
                for_key: false,
                key_type: None,
                value_type: None,
              };
              match #validator(item_field_data, #item_ident, #target) {
                Ok(_) => {},
                Err(v) => {
                   #violations_ident.push(v);
                },
              };
            }
          });
        } else {
          tokens.extend(quote! {
            let current_field_parent_elements = #parent_messages_ident.as_slice();

            let field_data_for_call = proto_types::FieldData {
              name: #field_name_str.to_string(),
              tag: #field_tag,
              is_repeated: false,
              is_map: false,
              is_required: #field_is_required,
              subscript: None,
              parent_elements: current_field_parent_elements,
              for_key: false,
              key_type: None, 
              value_type: None,
            };

            match #validator(field_data_for_call, &self.#field_rust_ident, #target) {
              Ok(_) => {},
              Err(v) => {
                #violations_ident.push(v);
              },
            };
        });
        }
      }
      GeneratedCodeKind::NestedMessageRecursion {
        is_optional,
        is_repeated,
      } => {
        let current_nested_field_element = quote! {
          proto_types::buf::validate::FieldPathElement {
            field_name: Some(#field_name_str.to_string()),
            field_number: Some(#field_tag as i32),
            field_type: Some(#field_proto_type_val),
            key_type: None,
            value_type: None,
            subscript: None,
          }
        };

        if *is_repeated {
          let item_ident = Ident::new("item", Span::call_site());
          let index_ident = Ident::new("idx", Span::call_site());

          tokens.extend(quote! {
            for (#index_ident, #item_ident) in self.#field_rust_ident.iter().enumerate() {
              let mut nested_item_element = #current_nested_field_element;
              nested_item_element.subscript = Some(proto_types::buf::validate::field_path_element::Subscript::Index(#index_ident as u64));

              #parent_messages_ident.push(nested_item_element); 
              #item_ident.nested_validate(#parent_messages_ident, #violations_ident); 
              #parent_messages_ident.pop(); 
            }
          });
        } else if *is_optional {
          tokens.extend(quote! {
            if let Some(nested_msg_instance) = &self.#field_rust_ident {
              #parent_messages_ident.push(#current_nested_field_element); 
              nested_msg_instance.nested_validate(#parent_messages_ident, #violations_ident); 
              #parent_messages_ident.pop(); 
            }
          });
        } else {
          tokens.extend(quote! {
            #parent_messages_ident.push(#current_nested_field_element);
            self.#field_rust_ident.nested_validate(#parent_messages_ident, #violations_ident);
            #parent_messages_ident.pop();
          });
        }
      }
    }
  }
}

fn generate_key_subscript(key_proto_type: ProtoType, key_ident: &Ident) -> TokenStream {
  let subscript_path = quote! { proto_types::buf::validate::field_path_element::Subscript };

  match key_proto_type {
    ProtoType::String => quote! { #subscript_path::StringKey(#key_ident.clone().into()) },
    ProtoType::Uint64 => quote! { #subscript_path::UintKey(#key_ident.clone().into()) },
    ProtoType::Uint32 => quote! { #subscript_path::UintKey(#key_ident.clone().into()) },
    ProtoType::Int64 => quote! { #subscript_path::IntKey(#key_ident.clone().into()) },
    ProtoType::Int32 => quote! { #subscript_path::IntKey(#key_ident.clone().into()) }, 
    ProtoType::Fixed64 => quote! { #subscript_path::UintKey(#key_ident.clone().into()) },
    ProtoType::Fixed32 => quote! { #subscript_path::UintKey(#key_ident.clone().into()) },
    ProtoType::Sfixed64 => quote! { #subscript_path::IntKey(#key_ident.clone().into()) },
    ProtoType::Sfixed32 => quote! { #subscript_path::IntKey(#key_ident.clone().into()) },
    ProtoType::Sint64 => quote! { #subscript_path::IntKey(#key_ident.clone().into()) },
    ProtoType::Sint32 => quote! { #subscript_path::IntKey(#key_ident.clone().into()) },
    ProtoType::Bool => quote! { #subscript_path::BoolKey(#key_ident.clone().into()) },

    _ => panic!(
        "Unsupported Protobuf type {:?} for map key. Only integral, string, and bool types are allowed.",
        key_proto_type
    ),
  }
}

