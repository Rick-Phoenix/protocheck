use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use quote::ToTokens;
use google::protobuf::field_descriptor_proto::Type as ProtoType;

use crate::buf::validate::field_path_element::Subscript;
use crate::buf::validate::FieldPathElement;
use crate::buf::validate::Ignore;

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

#[derive(Clone,Debug)]
pub struct FieldContext<'a> {
  pub field_data: FieldData,
  pub parent_elements: &'a [FieldPathElement],
  pub subscript: Option<Subscript>,
}

#[derive(Clone, Debug, Default)]
pub struct FieldData {
  pub rust_name: String,
  pub proto_name: String,
  pub tag: u32,
  pub is_repeated: bool,
  pub is_map: bool,
  pub is_required: bool,
  pub is_optional: bool,
  pub is_for_key: bool,
  pub key_type: Option<ProtoType>,   
  pub value_type: Option<ProtoType>,
  pub proto_type: ProtoType,
  pub enum_full_name: Option<String>,
  pub ignore: Ignore,
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

#[derive(Debug)]
pub enum GeneratedCodeKind {
  FieldRule,
  NestedMessageRecursion,
  MapValidationLoop {
    map_level_rules: Vec<ValidatorCallTemplate>,
    key_rules: Vec<ValidatorCallTemplate>,
    value_rules: Vec<ValidatorCallTemplate>,
    value_is_message: bool, 
  },
  OneofRule, 
  FieldCelRule {
    expression: String,
    message: String,
  }, 
  MessageCelRule {
    expression: String,
    message: String,
  }
}

#[derive(Debug)]
pub struct ValidatorCallTemplate {
  pub validator_path: Option<TokenStream>,
  pub target_value_tokens: Option<TokenStream>,

  pub field_data: FieldData,

  pub kind: GeneratedCodeKind,
}

impl ToTokens for ValidatorCallTemplate {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let field_proto_name = &self.field_data.proto_name;
    let field_tag = self.field_data.tag;
    let field_proto_type = self.field_data.proto_type as i32;
    let field_is_optional = self.field_data.is_optional;
    let is_for_key = self.field_data.is_for_key;
    let key_type = self.field_data.key_type;
    let value_type = self.field_data.value_type;

    let field_rust_ident = Ident::new(&self.field_data.rust_name, Span::call_site());
    let parent_messages_ident = Ident::new("parent_messages", Span::call_site());
    let violations_ident = Ident::new("violations", Span::call_site());
    let item_ident = Ident::new("item", Span::call_site());
    let index_ident = Ident::new("idx", Span::call_site());
    let key_ident = Ident::new("key", Span::call_site());
    let val_ident = Ident::new("val", Span::call_site());

    let field_data = self.field_data.clone();

    match &self.kind {
      GeneratedCodeKind::FieldCelRule { expression, message } => {
        let static_field_program_ident = Ident::new(&format!("__CEL_FIELD_PROGRAM_{}_{}", self.field_data.rust_name, expression.chars().filter(|c| c.is_alphanumeric()).take(10).collect::<String>()), Span::call_site());
        tokens.extend(quote! {
          #[allow(non_upper_case_globals)]
          static #static_field_program_ident: std::sync::LazyLock<cel_interpreter::Program> = std::sync::LazyLock::new(|| {
            cel_interpreter::Program::compile(#expression).expect("Cel program failed to compile")
          });

          let program = &#static_field_program_ident;
          let mut context = cel_interpreter::Context::default();
          context.add_variable("this", &self.#field_rust_ident).expect("Failed to add 'this' to the cel program");

          match macro_impl::validators::cel::validate_cel(program, context, #message.to_string()) {
            Ok(_) => {},
            Err(v) => violations.push(v),
          };
        });
      },
      GeneratedCodeKind::FieldRule => {
        let validator = self.validator_path.as_ref().unwrap();
        let target = self.target_value_tokens.as_ref().unwrap();

        if self.field_data.is_repeated {
          tokens.extend(quote! {
            let current_item_parent_elements = #parent_messages_ident.as_slice();
            for (#index_ident, #item_ident) in self.#field_rust_ident.iter().enumerate() {
              let field_data = #field_data;
              let field_context = proto_types::FieldContext {
                field_data,
                parent_elements: current_item_parent_elements,
                subscript: Some(proto_types::buf::validate::field_path_element::Subscript::Index(#index_ident as u64)),
              };
              match #validator(field_context, Some(#item_ident), #target) {
                Ok(_) => {},
                Err(v) => {
                   #violations_ident.push(v);
                },
              };
            }
          });
        } else if self.field_data.is_map {
          let key_subscript_gen_tokens = if let Some(key_type_enum) = key_type {
            generate_key_subscript(key_type_enum, &key_ident)
          } else {
             quote! {compile_error!("Map key type is missing during macro expansion.")} 
          };

          let data_ident = if is_for_key {
            &key_ident
          } else {
            &val_ident
          };

          tokens.extend(quote! {
            let current_field_parent_elements = #parent_messages_ident.as_slice();

            let field_data = #field_data;

            let field_context = proto_types::FieldContext {
              field_data,
              parent_elements: current_field_parent_elements,
              subscript: Some(#key_subscript_gen_tokens),
            };

            match #validator(field_context, Some(#data_ident), #target) {
              Ok(_) => {},
              Err(v) => {
                #violations_ident.push(v);
              },
            };
          });
        } else {
          let field_ident = if field_is_optional {
            quote! { &self.#field_rust_ident }
          } else {
            quote! { Some(&self.#field_rust_ident) }
          };
          tokens.extend(quote! {
            let current_field_parent_elements = #parent_messages_ident.as_slice();

            let field_data = #field_data;

            let field_context = proto_types::FieldContext {
              field_data,
              parent_elements: current_field_parent_elements,
              subscript: None,
            };

            match #validator(field_context, #field_ident, #target) {
              Ok(_) => {},
              Err(v) => {
                #violations_ident.push(v);
              },
            };
          });
        }
      }
      GeneratedCodeKind::NestedMessageRecursion  => {
        let current_nested_field_element = quote! {
          proto_types::buf::validate::FieldPathElement {
            field_name: Some(#field_proto_name.to_string()),
            field_number: Some(#field_tag as i32),
            field_type: Some(#field_proto_type),
            key_type: None,
            value_type: None,
            subscript: None,
          }
        };

        if self.field_data.is_repeated {
          tokens.extend(quote! {
            for (#index_ident, #item_ident) in self.#field_rust_ident.iter().enumerate() {
              let mut nested_item_element = #current_nested_field_element;
              nested_item_element.subscript = Some(proto_types::buf::validate::field_path_element::Subscript::Index(#index_ident as u64));

              #parent_messages_ident.push(nested_item_element); 
              #item_ident.nested_validate(#parent_messages_ident, #violations_ident); 
              #parent_messages_ident.pop(); 
            }
          });
        } else if self.field_data.is_optional {
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
      },
      GeneratedCodeKind::MapValidationLoop {map_level_rules, key_rules, value_rules, value_is_message } => {
        let key_subscript_gen_tokens = if let Some(key_type_enum) = key_type {
            generate_key_subscript(key_type_enum, &key_ident)
        } else {
           quote! {compile_error!("Map key type is missing during macro expansion.")} 
        };


        tokens.extend(quote! {
          #(#map_level_rules)* 

          for (#key_ident, #val_ident) in self.#field_rust_ident.iter() {
            let map_entry_field_path_element = proto_types::buf::validate::FieldPathElement {
              field_name: Some(#field_proto_name.to_string()),
              field_number: Some(#field_tag as i32),
              field_type: Some(#field_proto_type),
              key_type: Some(#key_type as i32),
              value_type: Some(#value_type as i32),
              subscript: Some(#key_subscript_gen_tokens),
            };
            #parent_messages_ident.push(map_entry_field_path_element);

            #(#key_rules)*

            if #value_is_message {
              #val_ident.nested_validate(#parent_messages_ident, #violations_ident);
            } else {
              #(#value_rules)*
            }

            #parent_messages_ident.pop();
          }
        });
      },
      GeneratedCodeKind::OneofRule => {
        tokens.extend(quote! {
          if !&self.#field_rust_ident.is_some() {
            let current_field_parent_elements = #parent_messages_ident.as_slice();

            let field_data = #field_data;

            let field_context = proto_types::FieldContext {
              field_data,
              parent_elements: current_field_parent_elements,
              subscript: None,
            };

            let violation = macro_impl::validators::oneofs::required(field_context);
            #violations_ident.push(violation);
          }
        });
      },
      GeneratedCodeKind::MessageCelRule { expression, message } => {
        let static_program_ident = Ident::new(&format!("__CEL_MESSAGE_PROGRAM_{}_{}", self.field_data.rust_name, expression.chars().filter(|c| c.is_alphanumeric()).take(10).collect::<String>()), Span::call_site());

        tokens.extend(quote! {
          #[allow(non_upper_case_globals)]
          static #static_program_ident: std::sync::LazyLock<cel_interpreter::Program> = std::sync::LazyLock::new(|| {
            cel_interpreter::Program::compile(#expression).expect("Cel program failed to compile")
          });

          let current_field_parent_elements = #parent_messages_ident.as_slice();

          let program = &#static_program_ident;
          let mut cel_context = cel_interpreter::Context::default();
          cel_context.add_variable("this", &self).expect("Failed to add 'this' to the cel program");

          let field_data = #field_data;

          let field_context = proto_types::FieldContext {
            field_data,
            subscript: None, 
            parent_elements: current_field_parent_elements,
          };

          match macro_impl::validators::cel::validate_cel(field_context, program, cel_context, #message.to_string()) {
            Ok(_) => {},
            Err(v) => violations.push(v),
          };
        });
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

    _ => quote! { compile_error!(format!("Unsupported Protobuf type {:?} for map key. Only integral, string, and bool types are allowed.",
        key_proto_type
    )) },
  }
}

