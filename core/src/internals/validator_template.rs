use quote::{quote, ToTokens};
use random_string::charsets::ALPHA_LOWER;

use crate::{field_data::FieldData, Ident2, ProtoType, Span2, TokenStream2};

#[derive(Debug)]
pub enum GeneratedCodeKind {
  FieldRule {
    validator_path: TokenStream2,
    target_value_tokens: TokenStream2,
  },
  MessageField,
  MapValidationLoop {
    map_level_rules: Vec<ValidatorCallTemplate>,
    key_rules: Vec<ValidatorCallTemplate>,
    value_rules: Vec<ValidatorCallTemplate>,
  },
  RepeatedValidationLoop {
    vec_level_rules: Vec<ValidatorCallTemplate>,
    items_rules: Vec<ValidatorCallTemplate>,
    unique_values: bool,
    float_values: bool,
  },
  OneofField {
    is_required: bool,
  },
  CelRule {
    expression: String,
    message: String,
    rule_id: String,
    is_for_message: bool,
  },
  EnumDefinedOnly {
    enum_type_ident: String,
    enum_name: String,
  },
}

#[derive(Debug)]
pub struct ValidatorCallTemplate {
  pub field_data: FieldData,
  pub kind: GeneratedCodeKind,
}

impl ToTokens for ValidatorCallTemplate {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let field_proto_name = &self.field_data.proto_name;
    let field_tag = self.field_data.tag;
    let field_proto_type = self.field_data.proto_type as i32;
    let field_is_optional = self.field_data.is_optional;
    let field_is_repeated_item = self.field_data.is_repeated_item;
    let field_is_map_key = self.field_data.is_map_key;
    let field_is_map_value = self.field_data.is_map_value;
    let field_is_in_map = field_is_map_key || field_is_map_value;
    let field_is_in_oneof = self.field_data.is_in_oneof;
    let key_type = self.field_data.key_type;
    let value_type = self.field_data.value_type;

    let field_rust_ident = Ident2::new(&self.field_data.rust_name, Span2::call_site());
    let parent_messages_ident = Ident2::new("parent_messages", Span2::call_site());
    let violations_ident = Ident2::new("violations", Span2::call_site());
    let index_ident = Ident2::new("idx", Span2::call_site());
    let item_ident = Ident2::new("item", Span2::call_site());
    let key_ident = Ident2::new("key", Span2::call_site());
    let val_ident = Ident2::new("val", Span2::call_site());
    let oneof_ident = Ident2::new("val", Span2::call_site());

    let subscript = if field_is_repeated_item || self.field_data.is_repeated {
      quote! { Some(protocheck::types::protovalidate::field_path_element::Subscript::Index(#index_ident as u64)) }
    } else if self.field_data.is_map || field_is_in_map {
      if let Some(key_type_enum) = key_type {
        let key_subscript_tokens = generate_key_subscript(key_type_enum, &key_ident);
        quote! { Some(#key_subscript_tokens) }
      } else {
        quote! {compile_error!("Map key type is missing during macro expansion.")}
      }
    } else {
      quote! { None }
    };

    let value_ident = if field_is_in_oneof {
      quote! { #oneof_ident }
    } else if field_is_repeated_item {
      quote! { #item_ident }
    } else if field_is_map_key {
      quote! { #key_ident }
    } else if field_is_map_value {
      quote! { #val_ident }
    } else {
      quote! { &self.#field_rust_ident }
    };

    let (key_type_tokens, value_type_tokens) = if key_type.is_some() && value_type.is_some() {
      (
        quote! { Some(#key_type as i32) },
        quote! { Some(#value_type as i32) },
      )
    } else {
      (quote! { None }, quote! { None })
    };

    let field_data = self.field_data.clone();

    match &self.kind {
      GeneratedCodeKind::EnumDefinedOnly {
        enum_type_ident,
        enum_name,
      } => {
        let enum_ident_tokens: TokenStream2 = enum_type_ident
          .parse()
          .expect("Failed to parse enum ident into tokens");

        tokens.extend(quote! {
          if !#enum_ident_tokens::try_from(*#value_ident).is_ok() {
            let field_context = protocheck::field_data::FieldContext {
              field_data: #field_data,
              parent_elements: #parent_messages_ident.as_slice(),
              subscript: #subscript,
            };
            #violations_ident.push(protocheck::validators::enums::defined_only(field_context, #enum_name));
          }
        });
      }
      GeneratedCodeKind::FieldRule {
        validator_path,
        target_value_tokens,
      } => {
        let field_ident = if field_is_in_oneof || !field_is_optional {
          quote! { Some(#value_ident) }
        } else {
          quote! { #value_ident }
        };

        tokens.extend(quote! {
          let field_context = protocheck::field_data::FieldContext {
            field_data: #field_data,
            parent_elements: #parent_messages_ident.as_slice(),
            subscript: #subscript,
          };

          match #validator_path(field_context, #field_ident, #target_value_tokens) {
            Ok(_) => {},
            Err(v) => {
              #violations_ident.push(v);
            },
          };
        });
      }
      GeneratedCodeKind::MessageField => {
        let current_nested_field_element = quote! {
          protocheck::types::protovalidate::FieldPathElement {
            field_name: Some(#field_proto_name.to_string()),
            field_number: Some(#field_tag as i32),
            field_type: Some(#field_proto_type),
            key_type: #key_type_tokens,
            value_type: #value_type_tokens,
            subscript: #subscript,
          }
        };

        if field_is_optional && !field_is_in_oneof {
          tokens.extend(quote! {
            if let Some(nested_msg_instance) = #value_ident {
              #parent_messages_ident.push(#current_nested_field_element);
              nested_msg_instance.nested_validate(#parent_messages_ident, #violations_ident);
              #parent_messages_ident.pop();
            }
          });
        } else {
          tokens.extend(quote! {
            #parent_messages_ident.push(#current_nested_field_element);
            #value_ident.nested_validate(#parent_messages_ident, #violations_ident);
            #parent_messages_ident.pop();
          });
        }
      }
      GeneratedCodeKind::RepeatedValidationLoop {
        vec_level_rules,
        items_rules,
        unique_values,
        float_values,
      } => {
        let (values_hashset, unique_values_check) = if *unique_values {
          let hashset_ident = Ident2::new("processed_values", Span2::call_site());
          let not_unique = Ident2::new("not_unique", Span2::call_site());
          let func_name = if *float_values {
            quote! { unique_floats }
          } else {
            quote! { unique }
          };
          (
            Some(quote! {
              let mut processed_values = std::collections::HashSet::new();
              let mut not_unique = false;
            }),
            Some(quote! {
              if !not_unique {
                let field_context = protocheck::field_data::FieldContext {
                  field_data: #field_data,
                  parent_elements: #parent_messages_ident,
                  subscript: #subscript,
                };
                match protocheck::validators::repeated::#func_name(field_context, #item_ident, &mut #hashset_ident) {
                  Ok(_) => {},
                  Err(v) => {
                    #not_unique = true;
                    #violations_ident.push(v);
                  }
                };
              }
            }),
          )
        } else {
          (None, None)
        };
        tokens.extend(quote! {
          #(#vec_level_rules)*

          #values_hashset
          for (#index_ident, #item_ident) in self.#field_rust_ident.iter().enumerate() {
            #(#items_rules)*

            #unique_values_check
          }
        });
      }
      GeneratedCodeKind::MapValidationLoop {
        map_level_rules,
        key_rules,
        value_rules,
      } => {
        tokens.extend(quote! {
          #(#map_level_rules)*

          for (#key_ident, #val_ident) in self.#field_rust_ident.iter() {
            let map_entry_field_path_element = protocheck::types::protovalidate::FieldPathElement {
              field_name: Some(#field_proto_name.to_string()),
              field_number: Some(#field_tag as i32),
              field_type: Some(#field_proto_type),
              key_type: #key_type_tokens,
              value_type: #value_type_tokens,
              subscript: #subscript,
            };

            #parent_messages_ident.push(map_entry_field_path_element);

            #(#key_rules)*

            #(#value_rules)*

            #parent_messages_ident.pop();
          }
        });
      }
      GeneratedCodeKind::OneofField { is_required } => {
        tokens.extend(quote! {
          match &self.#field_rust_ident {
            Some(oneof) => { oneof.nested_validate(#parent_messages_ident, #violations_ident); },
            None => {
              if #is_required {
                let field_context = protocheck::field_data::FieldContext {
                  field_data: #field_data,
                  parent_elements: #parent_messages_ident.as_slice(),
                  subscript: None,
                };

                let violation = protocheck::validators::oneofs::required(field_context);
                #violations_ident.push(violation);
              }
            }
          };
        });
      }
      GeneratedCodeKind::CelRule {
        expression,
        message,
        rule_id,
        is_for_message,
      } => {
        let (context_target, program_type) = if *is_for_message {
          (quote! { &self }, "MESSAGE")
        } else {
          (quote! { &self.#field_rust_ident }, "FIELD")
        };

        let random_string = random_string::generate(5, ALPHA_LOWER);
        let static_program_ident = Ident2::new(
          &format!(
            "__CEL_{}_PROGRAM_{}_{}",
            program_type, self.field_data.rust_name, random_string
          ),
          Span2::call_site(),
        );

        tokens.extend(quote! {
          #[allow(non_upper_case_globals)]
          static #static_program_ident: std::sync::LazyLock<cel_interpreter::Program> = std::sync::LazyLock::new(|| {
            cel_interpreter::Program::compile(#expression).expect("Cel program failed to compile")
          });

          let program = &#static_program_ident;
          let mut cel_context = cel_interpreter::Context::default();
          cel_context.add_variable("this", #context_target).expect("Failed to add 'this' to the cel program");

          let field_context = protocheck::field_data::FieldContext {
            field_data: #field_data,
            subscript: None,
            parent_elements: #parent_messages_ident.as_slice(),
          };

          match protocheck::validators::cel::validate_cel(field_context, program, cel_context, #message.to_string(), #rule_id.to_string(), #is_for_message) {
            Ok(_) => {},
            Err(v) => violations.push(v),
          };
        });
      }
    }
  }
}

fn generate_key_subscript(key_proto_type: ProtoType, key_ident: &Ident2) -> TokenStream2 {
  let subscript_path = quote! { protocheck::types::protovalidate::field_path_element::Subscript };

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

    _ => {
      quote! { compile_error!(format!("Unsupported Protobuf type {:?} for map key. Only integral, string, and bool types are allowed.",
          key_proto_type
      )) }
    }
  }
}
