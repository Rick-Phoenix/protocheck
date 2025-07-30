use quote::{quote, ToTokens};

use crate::{
  field_data::{CelRuleTemplateTarget, FieldData, FieldKind},
  Ident2, ProtoType, Span2, TokenStream2,
};

#[derive(Debug)]
pub enum ValidatorKind {
  FieldRule {
    validator_path: TokenStream2,
    target_value_tokens: TokenStream2,
    field_data: FieldData,
  },
  MessageField {
    field_data: FieldData,
  },
  MapValidationLoop {
    map_level_rules: Vec<ValidatorTemplate>,
    key_rules: Vec<ValidatorTemplate>,
    value_rules: Vec<ValidatorTemplate>,
    field_data: FieldData,
  },
  RepeatedValidationLoop {
    vec_level_rules: Vec<ValidatorTemplate>,
    items_rules: Vec<ValidatorTemplate>,
    unique_values: bool,
    float_values: bool,
    field_data: FieldData,
  },
  OneofField {
    is_required: bool,
  },
  CelRule {
    rule_id: String,
    error_message: String,
    rule_target: CelRuleTemplateTarget,
    static_program_ident: Ident2,
  },
  EnumDefinedOnly {
    enum_type_ident: String,
    enum_name: String,
    field_data: FieldData,
  },
}

#[derive(Debug)]
pub struct ValidatorTemplate {
  pub item_rust_name: String,
  pub kind: ValidatorKind,
}

fn get_subscript_tokens(
  field_data: &FieldData,
  index_ident: &Ident2,
  key_ident: &Ident2,
) -> TokenStream2 {
  if field_data.kind.is_repeated_item() || field_data.kind.is_repeated() {
    quote! { Some(protocheck::types::protovalidate::field_path_element::Subscript::Index(#index_ident as u64)) }
  } else if field_data.kind.is_map()
    || field_data.kind.is_map_key()
    || field_data.kind.is_map_value()
  {
    if let Some(key_type_enum) = field_data.key_type {
      let key_subscript_tokens = generate_key_subscript(key_type_enum, key_ident);
      quote! { Some(#key_subscript_tokens) }
    } else {
      quote! { compile_error!("Map key type is missing during macro expansion.") }
    }
  } else {
    quote! { None }
  }
}

fn get_key_value_type_tokens(field_data: &FieldData) -> (TokenStream2, TokenStream2) {
  if let Some(key_type) = field_data.key_type {
    if let Some(value_type) = field_data.value_type {
      (
        quote! { Some(#key_type as i32) },
        quote! { Some(#value_type as i32) },
      )
    } else {
      (quote! { None }, quote! { None })
    }
  } else {
    (quote! { None }, quote! { None })
  }
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

    let get_value_ident = |field_data: &FieldData| -> TokenStream2 {
      if field_data.is_in_oneof {
        quote! { #oneof_ident }
      } else {
        match field_data.kind {
          FieldKind::RepeatedItem => quote! { #item_ident },
          FieldKind::MapKey => quote! { #key_ident },
          FieldKind::MapValue => quote! { #val_ident },
          _ => {
            if field_data.is_optional {
              quote! { self.#item_rust_ident.as_ref() }
            } else {
              quote! { &self.#item_rust_ident }
            }
          }
        }
      }
    };

    match &self.kind {
      ValidatorKind::EnumDefinedOnly {
        enum_type_ident,
        enum_name,
        field_data,
      } => {
        let enum_ident_tokens: TokenStream2 = enum_type_ident
          .parse()
          .unwrap_or(quote! { compile_error!(format!("Failed to parse enum ident {} into tokens for enum {}", enum_type_ident, enum_name)) });
        let subscript = get_subscript_tokens(field_data, &index_ident, &key_ident);
        let value_ident = get_value_ident(field_data);

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
      ValidatorKind::FieldRule {
        validator_path,
        target_value_tokens,
        field_data,
      } => {
        let subscript = get_subscript_tokens(field_data, &index_ident, &key_ident);
        let value_ident = get_value_ident(field_data);

        let field_ident = if field_data.is_in_oneof || !field_data.is_optional {
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
      ValidatorKind::MessageField { field_data } => {
        let subscript = get_subscript_tokens(field_data, &index_ident, &key_ident);
        let field_proto_name = &field_data.proto_name;
        let field_tag = &field_data.tag;
        let field_proto_type = &field_data.proto_type;
        let (key_type_tokens, value_type_tokens) = get_key_value_type_tokens(field_data);
        let value_ident = get_value_ident(field_data);

        let current_nested_field_element = quote! {
          protocheck::types::protovalidate::FieldPathElement {
            field_name: Some(#field_proto_name.to_string()),
            field_number: Some(#field_tag as i32),
            field_type: Some(#field_proto_type as i32),
            key_type: #key_type_tokens,
            value_type: #value_type_tokens,
            subscript: #subscript,
          }
        };

        let is_option = field_data.is_optional && !field_data.is_in_oneof;

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

        if is_option {
          tokens.extend(quote! {
            if let Some(#target_ident) = #value_ident {
              #validator_tokens
            }
          });
        } else {
          tokens.extend(quote! {
            #validator_tokens
          });
        }
      }
      ValidatorKind::RepeatedValidationLoop {
        vec_level_rules,
        items_rules,
        unique_values,
        float_values,
        field_data,
      } => {
        let subscript = get_subscript_tokens(field_data, &index_ident, &key_ident);

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
          for (#index_ident, #item_ident) in self.#item_rust_ident.iter().enumerate() {
            #(#items_rules)*

            #unique_values_check
          }
        });
      }
      ValidatorKind::MapValidationLoop {
        map_level_rules,
        key_rules,
        value_rules,
        field_data,
      } => {
        let subscript = get_subscript_tokens(field_data, &index_ident, &key_ident);
        let field_proto_name = &field_data.proto_name;
        let field_tag = &field_data.tag;
        let field_proto_type = &field_data.proto_type;
        let (key_type_tokens, value_type_tokens) = get_key_value_type_tokens(field_data);

        tokens.extend(quote! {
          #(#map_level_rules)*

          for (#key_ident, #val_ident) in self.#item_rust_ident.iter() {
            let map_entry_field_path_element = protocheck::types::protovalidate::FieldPathElement {
              field_name: Some(#field_proto_name.to_string()),
              field_number: Some(#field_tag as i32),
              field_type: Some(#field_proto_type as i32),
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
      ValidatorKind::OneofField { is_required } => {
        tokens.extend(quote! {
          match &self.#item_rust_ident {
            Some(oneof) => { oneof.nested_validate(#parent_messages_ident, #violations_ident); },
            None => {
              if #is_required {
                let violation = protocheck::validators::oneofs::required(#item_rust_name, #parent_messages_ident.as_slice());
                #violations_ident.push(violation);
              }
            }
          };
        });
      }
      ValidatorKind::CelRule {
        rule_id,
        error_message,
        rule_target: rule_template,
        static_program_ident,
      } => {
        let is_option = rule_template.is_option();
        let (context_target, rule_target_tokens) = match rule_template {
          CelRuleTemplateTarget::Message(message_desc) => {
            let name = message_desc.name();
            (
              quote! { &self },
              quote! {
                protocheck::validators::cel::CelRuleTarget::Message {
                  name: #name.to_string(),
                  parent_elements: #parent_messages_ident,
                }
              },
            )
          }
          CelRuleTemplateTarget::Field(_, field_data) => {
            let subscript = get_subscript_tokens(field_data, &index_ident, &key_ident);
            (
              get_value_ident(field_data),
              quote! {
                protocheck::validators::cel::CelRuleTarget::Field {
                  field_context: protocheck::field_data::FieldContext {
                    field_data: #field_data,
                    parent_elements: #parent_messages_ident,
                    subscript: #subscript,
                  }
                }
              },
            )
          }
        };

        let target_tokens = if !is_option {
          &quote! { Some(#context_target) }
        } else {
          &context_target
        };

        tokens.extend(quote! {
          let rule_data = protocheck::validators::cel::CelRuleData {
            rule_id: #rule_id.to_string(),
            error_message: #error_message.to_string(),
            rule_target: #rule_target_tokens
          };

          match protocheck::validators::cel::validate_cel(&rule_data, &#static_program_ident, #target_tokens) {
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
