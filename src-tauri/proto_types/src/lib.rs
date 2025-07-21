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
  pub violation_rule_id: Option<String>,

  pub field_rust_ident_str: String,
  pub field_tag: u32,
  pub field_proto_name: String,
  pub field_proto_type: google::protobuf::field_descriptor_proto::Type,
  pub field_is_repeated: bool,
  pub field_is_map: bool,
  pub field_is_required: bool,

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
    let parent_messages_ident = Ident::new("parent_messages", Span::call_site());

    match &self.kind {
      GeneratedCodeKind::FieldRule => {
        let validator = self.validator_path.as_ref().unwrap();
        let target = self.target_value_tokens.as_ref().unwrap();
        let violation_rule_id_str = self.violation_rule_id.as_ref().unwrap();

        let current_field_path_element_common = quote! {
            field_name: Some(#field_name_str.to_string()),
            field_number: Some(#field_tag as i32),
            field_type: Some(#field_proto_type_val),
            key_type: None,
            value_type: None,
        };

        if self.field_is_repeated {
          let item_ident = Ident::new("item", Span::call_site());
          let index_ident = Ident::new("idx", Span::call_site());

          tokens.extend(quote! {
                        for (#index_ident, #item_ident) in self.#field_rust_ident.iter().enumerate() {
                            let mut current_item_parent_elements = #parent_messages_ident.clone();
                            let current_field_path_element = proto_types::buf::validate::FieldPathElement {
                                #current_field_path_element_common
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
                            #current_field_path_element_common
                            subscript: None,
                        };
                        current_field_parent_elements.push(current_field_path_element);

                        let field_data_for_call = proto_types::FieldData {
                            name: #field_name_str.to_string(),
                            tag: #field_tag,
                            is_repeated: false,
                            is_map: false,
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
                            #item_ident.nested_validate(#parent_messages_ident, #violations_ident); // Recurse
                            #parent_messages_ident.pop(); // Pop after the call
                        }
                    });
        } else if *is_optional {
          // Option<NestedMessage>
          tokens.extend(quote! {
              if let Some(nested_msg_instance) = &self.#field_rust_ident {
                  #parent_messages_ident.push(#current_nested_field_element); // Push this field's element
                  nested_msg_instance.nested_validate(#parent_messages_ident, #violations_ident); // Recurse
                  #parent_messages_ident.pop(); // Pop after the call
              }
          });
        } else {
          // Direct NestedMessage
          tokens.extend(quote! {
              #parent_messages_ident.push(#current_nested_field_element); // Push this field's element
              self.#field_rust_ident.nested_validate(#parent_messages_ident, #violations_ident); // Recurse
              #parent_messages_ident.pop(); // Pop after the call
          });
        }
      }
    }
  }
}
