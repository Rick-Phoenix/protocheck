use quote::{quote, ToTokens};

use crate::{validation_data::ValidationData, Ident2, ProtoType, Span2, TokenStream2};

#[derive(Debug)]
pub enum FieldValidator {
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
}

#[derive(Debug)]
pub enum ValidatorKind {
  PureTokens(TokenStream2),
  Field {
    validation_data: Box<ValidationData>,
    field_validator: FieldValidator,
  },
  Oneof {
    is_required: bool,
    field_name: String,
    item_rust_ident: Ident2,
    violations_ident: Ident2,
    parent_messages_ident: Ident2,
  },
}

#[derive(Debug)]
pub struct ValidatorTemplate {
  pub kind: ValidatorKind,
}

impl ToTokens for ValidatorTemplate {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    match &self.kind {
      ValidatorKind::PureTokens(validator_tokens) => {
        tokens.extend(quote! {
          #validator_tokens
        });
      }
      ValidatorKind::Field {
        validation_data,
        field_validator,
      } => {
        let key_type_tokens = validation_data.key_type_tokens();
        let value_type_tokens = validation_data.value_type_tokens();
        let subscript_tokens = validation_data.subscript_tokens();

        let ValidationData {
          field_data_static_ident,
          key_ident,
          map_value_ident,
          index_ident,
          item_ident,
          parent_messages_ident,
          violations_ident,
          item_rust_ident,
          ..
        } = &**validation_data;

        match field_validator {
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
                    key_type: #key_type_tokens,
                    value_type: #value_type_tokens,
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

            let has_loop = *unique_values || !items_rules.is_empty();
            println!("Has Loop: {:?}", has_loop);

            let loop_tokens = has_loop.then_some(quote! {
              for (#index_ident, #item_ident) in self.#item_rust_ident.iter().enumerate() {
                #(#items_rules)*

                #unique_values_check_tokens
              }
            });

            tokens.extend(quote! {
              #(#vec_level_rules)*

              #values_hashset_tokens
              #loop_tokens
            });
          }
          FieldValidator::Map {
            map_level_rules,
            key_rules,
            value_rules,
          } => {
            let has_loop = !key_rules.is_empty() || !value_rules.is_empty();
            let loop_tokens = has_loop.then_some(quote! {
              for (#key_ident, #map_value_ident) in self.#item_rust_ident.iter() {
                #(#key_rules)*

                #(#value_rules)*
              }
            });

            tokens.extend(quote! {
              #(#map_level_rules)*

              #loop_tokens
            });
          }
        }
      }

      ValidatorKind::Oneof {
        is_required,
        field_name,
        item_rust_ident,
        violations_ident,
        parent_messages_ident,
      } => {
        let required_check = is_required.then_some(quote! {
          #violations_ident.push(protocheck::validators::oneofs::required(#field_name, #parent_messages_ident.as_slice()));
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

pub fn generate_key_subscript(key_proto_type: &ProtoType, key_ident: &Ident2) -> TokenStream2 {
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
