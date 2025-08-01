use protocheck_core::field_data::FieldKind;
use quote::{quote, ToTokens};

use crate::{validation_data::ValidationData, Ident2, ProtoType, Span2, TokenStream2};

#[derive(Debug)]
pub enum FieldValidator {
  Scalar {
    validator_path: TokenStream2,
    target_value_tokens: TokenStream2,
  },
  MessageField,
  Map {
    map_level_rules: Vec<ValidatorTemplate>,
    key_rules: Vec<ValidatorTemplate>,
    value_rules: Vec<ValidatorTemplate>,
  },
  Repeated {
    vec_level_rules: Vec<ValidatorTemplate>,
    items_rules: Vec<ValidatorTemplate>,
    unique_values: bool,
    float_values: bool,
  },
  Cel {
    full_name: String,
    message_full_name: String,
    rule_id: String,
    error_message: String,
    static_program_ident: Ident2,
  },
  EnumDefinedOnly {
    enum_type_ident: String,
    enum_name: String,
  },
  Required,
}

#[derive(Debug)]
pub enum MessageValidator {
  Cel {
    full_name: String,
    rule_id: String,
    error_message: String,
    static_program_ident: Ident2,
  },
}

#[derive(Debug)]
pub enum ValidatorKind {
  Field {
    validation_data: ValidationData,
    field_validator: FieldValidator,
  },
  Message {
    message_validator: MessageValidator,
  },
  Oneof {
    is_required: bool,
  },
}

#[derive(Debug)]
pub struct ValidatorTemplate {
  pub item_rust_name: String,
  pub kind: ValidatorKind,
}

impl ToTokens for ValidatorTemplate {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let item_rust_name = &self.item_rust_name;
    let item_rust_ident = Ident2::new(item_rust_name, Span2::call_site());
    let parent_messages_ident = Ident2::new("parent_messages", Span2::call_site());
    let violations_ident = Ident2::new("violations", Span2::call_site());
    let index_ident = Ident2::new("idx", Span2::call_site());
    let item_ident = Ident2::new("item", Span2::call_site());
    let key_ident = Ident2::new("key", Span2::call_site());
    let val_ident = Ident2::new("val", Span2::call_site());
    let oneof_ident = Ident2::new("val", Span2::call_site());

    match &self.kind {
      ValidatorKind::Field {
        validation_data,
        field_validator,
      } => {
        let ValidationData {
          is_required,
          is_optional,
          is_in_oneof,
          field_data_static_ident,
          field_data,
          ..
        } = validation_data;

        let field_proto_name = &field_data.proto_name;
        let field_tag = &field_data.tag;
        let field_proto_type = &field_data.proto_type;

        let key_type_tokens = field_data.key_type.map_or(quote! { None }, |key_type| {
          quote! { Some(#key_type as i32) }
        });
        let value_type_tokens = field_data.value_type.map_or(quote! { None }, |value_type| {
          quote! { Some(#value_type as i32) }
        });

        let subscript_tokens = match field_data.kind {
          FieldKind::RepeatedItem | FieldKind::Repeated => {
            quote! { Some(protocheck::types::protovalidate::field_path_element::Subscript::Index(#index_ident as u64)) }
          }
          FieldKind::Map | FieldKind::MapKey | FieldKind::MapValue => {
            if let Some(key_type_enum) = field_data.key_type {
              let key_subscript_tokens = generate_key_subscript(key_type_enum, &key_ident);
              quote! { Some(#key_subscript_tokens) }
            } else {
              quote! { compile_error!("Map key type is missing during macro expansion.") }
            }
          }
          _ => quote! { None },
        };

        let is_option = *is_optional && !*is_in_oneof;

        let value_ident = if *is_in_oneof {
          quote! { #oneof_ident }
        } else {
          match field_data.kind {
            FieldKind::RepeatedItem => quote! { #item_ident },
            FieldKind::MapKey => quote! { #key_ident },
            FieldKind::MapValue => quote! { #val_ident },
            _ => {
              if *is_optional {
                quote! { self.#item_rust_ident.as_ref() }
              } else {
                quote! { &self.#item_rust_ident }
              }
            }
          }
        };

        match field_validator {
          FieldValidator::Required => {
            tokens.extend(quote! {
              if #value_ident.is_none() {
                let field_context = protocheck::field_data::FieldContext {
                  field_data: &#field_data_static_ident,
                  parent_elements: #parent_messages_ident,
                  subscript: #subscript_tokens
                };

                #violations_ident.push(protocheck::validators::required::required(&field_context));
              }
            });
          }
          FieldValidator::Scalar {
            validator_path,
            target_value_tokens,
          } => {
            let unwrapped_val_ident = Ident2::new("val", Span2::call_site());
            let field_ident = if is_option {
              quote! { #unwrapped_val_ident }
            } else {
              quote! { #value_ident }
            };

            let field_context_tokens = quote! {
              let field_context = protocheck::field_data::FieldContext {
                field_data: &#field_data_static_ident,
                parent_elements: #parent_messages_ident.as_slice(),
                subscript: #subscript_tokens,
              };
            };

            let required_check = is_required.then_some(quote! {
              let required_violation = protocheck::validators::required(&field_context);
              #violations_ident.push(required_violation);
            });

            let validator_tokens = quote! {
              match #validator_path(&field_context, #field_ident, &#target_value_tokens) {
                Ok(_) => {},
                Err(v) => {
                  #violations_ident.push(v);
                },
              };
            };

            if is_option {
              tokens.extend(quote! {
                #field_context_tokens
                match #value_ident {
                  Some(#unwrapped_val_ident) => {
                    #validator_tokens
                  },
                  None => { #required_check }
                };
              });
            } else {
              tokens.extend(quote! {
                #field_context_tokens
                #validator_tokens
              });
            }
          }
          FieldValidator::EnumDefinedOnly {
            enum_type_ident,
            enum_name,
          } => {
            let enum_ident_tokens: TokenStream2 = enum_type_ident
          .parse()
          .unwrap_or(quote! { compile_error!(format!("Failed to parse enum ident {} into tokens for enum {}", enum_type_ident, enum_name)) });

            let enum_val_ident = if is_option {
              let unwrapped_val_ident = Ident2::new("enum_val", Span2::call_site());
              &quote! { #unwrapped_val_ident }
            } else {
              &value_ident
            };

            let validator_tokens = quote! {
              if !#enum_ident_tokens::try_from(*#enum_val_ident).is_ok() {
                let field_context = protocheck::field_data::FieldContext {
                  field_data: &#field_data_static_ident,
                  parent_elements: #parent_messages_ident.as_slice(),
                  subscript: #subscript_tokens,
                };
                #violations_ident.push(protocheck::validators::enums::defined_only(&field_context, #enum_name));
              }
            };

            if is_option {
              tokens.extend(quote! {
                if let Some(#enum_val_ident) = #value_ident {
                  #validator_tokens
                }
              });
            } else {
              tokens.extend(quote! {
                #validator_tokens
              });
            }
          }
          FieldValidator::Repeated {
            vec_level_rules,
            items_rules,
            unique_values,
            float_values,
          } => {
            let values_hashset_tokens = unique_values.then_some(quote! {
              let mut processed_values = std::collections::HashSet::new();
              let mut not_unique = false;
            });

            let unique_values_check_tokens = unique_values.then(|| {
              let hashset_ident = Ident2::new("processed_values", Span2::call_site());
              let not_unique = Ident2::new("not_unique", Span2::call_site());
              let func_name = if *float_values {
                quote! { unique_floats }
              } else {
                quote! { unique }
              };

              quote! {
                if !not_unique {
                  let field_context = protocheck::field_data::FieldContext {
                    field_data: &#field_data_static_ident,
                    parent_elements: #parent_messages_ident,
                    subscript: #subscript_tokens,
                  };
                  match protocheck::validators::repeated::#func_name(&field_context, &#item_ident, &mut #hashset_ident) {
                    Ok(_) => {},
                    Err(v) => {
                      #not_unique = true;
                      #violations_ident.push(v);
                    }
                  };
                }
              }
            });

            tokens.extend(quote! {
              #(#vec_level_rules)*

              #values_hashset_tokens
              for (#index_ident, #item_ident) in self.#item_rust_ident.iter().enumerate() {
                #(#items_rules)*

                #unique_values_check_tokens
              }
            });
          }
          FieldValidator::Map {
            map_level_rules,
            key_rules,
            value_rules,
          } => {
            tokens.extend(quote! {
              #(#map_level_rules)*

              for (#key_ident, #val_ident) in self.#item_rust_ident.iter() {
                #(#key_rules)*

                #(#value_rules)*
              }
            });
          }
          FieldValidator::MessageField => {
            let field_proto_name = &field_data.proto_name;
            let field_tag = &field_data.tag;
            let field_proto_type = &field_data.proto_type;

            let current_nested_field_element = quote! {
              protocheck::types::protovalidate::FieldPathElement {
                field_name: Some(#field_proto_name.to_string()),
                field_number: Some(#field_tag as i32),
                field_type: Some(#field_proto_type as i32),
                key_type: #key_type_tokens,
                value_type: #value_type_tokens,
                subscript: #subscript_tokens,
              }
            };

            let target_ident = if is_option {
              let unwrapped_message_ident = Ident2::new("msg_val", Span2::call_site());
              &quote! { #unwrapped_message_ident }
            } else {
              &value_ident
            };

            let validator_tokens = quote! {
              #parent_messages_ident.push(#current_nested_field_element);
              #target_ident.nested_validate(#parent_messages_ident, #violations_ident);
              #parent_messages_ident.pop();
            };

            let required_check = is_required.then_some(quote! {
              let field_context = protocheck::field_data::FieldContext {
                field_data: &#field_data_static_ident,
                parent_elements: #parent_messages_ident,
                subscript: #subscript_tokens
              };
              #violations_ident.push(protocheck::validators::required::required(&field_context));
            });

            if is_option {
              tokens.extend(quote! {
                match #value_ident {
                  Some(#target_ident) => {
                    #validator_tokens
                  },
                  None => { #required_check }
                };
              });
            } else {
              tokens.extend(quote! {
                #validator_tokens
              });
            }
          }
          FieldValidator::Cel {
            rule_id,
            error_message,
            static_program_ident,
            ..
          } => {
            let cel_field_enum_tokens = match field_data.kind {
              FieldKind::Duration => {
                quote! { protocheck::validators::cel::CelFieldKind::Duration(#value_ident) }
              }
              FieldKind::Timestamp => {
                quote! { protocheck::validators::cel::CelFieldKind::Timestamp(&#value_ident) }
              }
              _ => {
                if !is_option {
                  quote! { protocheck::validators::cel::CelFieldKind::Other(Some(&#value_ident)) }
                } else {
                  quote! { protocheck::validators::cel::CelFieldKind::Other(#value_ident) }
                }
              }
            };

            tokens.extend(quote! {
              let rule_data = protocheck::validators::cel::CelField {
                rule_id: #rule_id.to_string(),
                error_message: #error_message.to_string(),
                field_context: &protocheck::field_data::FieldContext {
                   field_data: &#field_data_static_ident,
                   parent_elements: #parent_messages_ident,
                   subscript: #subscript_tokens,
                },
              };

              match protocheck::validators::cel::validate_cel_field(&rule_data, &#static_program_ident, Some(&#cel_field_enum_tokens)) {
                Ok(_) => {},
                Err(v) => violations.push(v),
              };
            });
          }
        }
      }
      ValidatorKind::Message { message_validator } => match message_validator {
        MessageValidator::Cel {
          rule_id,
          error_message,
          static_program_ident,
          full_name,
        } => {
          tokens.extend(quote! {
            let rule_data = protocheck::validators::cel::CelRuleData {
              rule_id: #rule_id.to_string(),
              error_message: #error_message.to_string(),
              rule_target: protocheck::validators::cel::CelRuleTarget::Message {
                name: #item_rust_name.to_string(),
                parent_elements: #parent_messages_ident,
                full_name: #full_name.to_string()
              }
            };

            match protocheck::validators::cel::validate_cel(&crate::DESCRIPTOR_POOL, &rule_data, &#static_program_ident, Some(self)) {
              Ok(_) => {},
              Err(v) => violations.push(v),
            };
          });
        }
      },
      ValidatorKind::Oneof { is_required } => {
        let required_check = is_required.then_some(quote! {
          #violations_ident.push(protocheck::validators::oneofs::required(#item_rust_name, #parent_messages_ident.as_slice()));
        });
        tokens.extend(quote! {
          match &self.#item_rust_ident {
            Some(oneof) => { oneof.nested_validate(#parent_messages_ident, #violations_ident); },
            None => { #required_check }
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
