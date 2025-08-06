use proc_macro2::TokenStream;
use proto_types::{protovalidate::Ignore, FieldType};
use protocheck_core::field_data::FieldKind;
use quote::quote;

use crate::{Ident2, ProtoType, Span2};

#[derive(Debug, Clone)]
pub(crate) struct ValidationData<'a> {
  pub full_name: &'a str,
  pub rust_name: &'a str,
  pub proto_name: &'a str,
  pub ignore: Ignore,
  pub tag: u32,
  pub is_required: bool,
  pub is_optional: bool,
  pub is_in_oneof: bool,
  pub field_span: Span2,
  pub key_type: Option<ProtoType>,
  pub value_type: Option<ProtoType>,
  pub key_ident: &'a Ident2,
  pub map_value_ident: &'a Ident2,
  pub index_ident: &'a Ident2,
  pub item_ident: &'a Ident2,
  pub item_rust_ident: &'a Ident2,
  pub violations_ident: &'a Ident2,
  pub parent_messages_ident: &'a Ident2,
  pub field_context_ident: &'a Ident2,
  pub map_key_context_ident: &'a Ident2,
  pub map_value_context_ident: &'a Ident2,
  pub vec_item_context_ident: &'a Ident2,
  pub field_kind: FieldKind,
}

pub struct RepeatedValidator {
  pub vec_level_rules: TokenStream,
  pub items_rules: TokenStream,
  pub items_context_tokens: TokenStream,
  pub unique_values: bool,
  pub float_values: bool,
}

pub struct MapValidator {
  pub map_level_rules: TokenStream,
  pub key_rules: TokenStream,
  pub key_context_tokens: TokenStream,
  pub value_rules: TokenStream,
  pub value_context_tokens: TokenStream,
}

impl ValidationData<'_> {
  pub fn field_context_tokens(&self) -> TokenStream {
    let Self {
      parent_messages_ident,
      proto_name,
      rust_name,
      tag,
      ignore,
      ..
    } = self;
    let key_type_tokens = self.key_type_tokens();
    let value_type_tokens = self.value_type_tokens();
    let subscript_tokens = self.subscript_tokens();
    let field_context_ident = self.field_context_ident();
    let field_kind = &self.field_kind;

    quote! {
      let #field_context_ident = protocheck::field_data::FieldContext {
        parent_elements: #parent_messages_ident.as_slice(),
        subscript: #subscript_tokens,
        key_type: #key_type_tokens,
        value_type: #value_type_tokens,
        field_kind: #field_kind,
        proto_name: #proto_name,
        rust_name: #rust_name,
        tag: #tag,
        ignore: #ignore
      };
    }
  }

  pub fn aggregate_map_rules(&self, rules_data: &MapValidator) -> TokenStream {
    let MapValidator {
      map_level_rules,
      key_rules,
      value_rules,
      key_context_tokens,
      value_context_tokens,
    } = rules_data;

    let key_ident = &self.key_ident;
    let map_value_ident = &self.map_value_ident;
    let item_rust_ident = &self.item_rust_ident;

    let has_loop = !key_rules.is_empty() || !value_rules.is_empty();
    let map_level_context_tokens =
      (!map_level_rules.is_empty()).then_some(self.field_context_tokens());

    let loop_tokens = has_loop.then_some(quote! {
      for (#key_ident, #map_value_ident) in self.#item_rust_ident.iter() {
        #key_context_tokens
        #value_context_tokens

        #key_rules

        #value_rules
      }
    });

    quote! {
      #map_level_context_tokens
      #map_level_rules

      #loop_tokens
    }
  }

  pub fn field_context_ident(&self) -> &Ident2 {
    match self.field_kind {
      FieldKind::RepeatedItem(_) => self.vec_item_context_ident,
      FieldKind::MapKey(_) => self.map_key_context_ident,
      FieldKind::MapValue(_) => self.map_value_context_ident,
      FieldKind::Single(_) => self.field_context_ident,
    }
  }

  pub fn aggregate_vec_rules(&self, rules_data: &RepeatedValidator) -> TokenStream {
    let index_ident = &self.index_ident;
    let item_ident = &self.item_ident;
    let item_rust_ident = &self.item_rust_ident;
    let violations_ident = &self.violations_ident;

    let RepeatedValidator {
      vec_level_rules,
      items_rules,
      unique_values,
      float_values,
      items_context_tokens,
    } = rules_data;

    let field_context_tokens = self.field_context_tokens();

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
          match protocheck::validators::repeated::#func_name(&field_context, #item_ident, &mut #hashset_ident) {
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
    let vec_level_field_context = (!vec_level_rules.is_empty()).then_some(&field_context_tokens);

    let loop_tokens = has_loop.then_some(quote! {
      for (#index_ident, #item_ident) in self.#item_rust_ident.iter().enumerate() {
        #items_context_tokens
        #items_rules

        #unique_values_check_tokens
      }
    });

    quote! {
      #vec_level_field_context
      #vec_level_rules

      #values_hashset_tokens
      #loop_tokens
    }
  }

  pub fn get_lt_validator(&self, lt_val: &TokenStream) -> TokenStream {
    let field_context_ident = self.field_context_ident();

    let value_ident = self.value_ident();
    let validator_expression_tokens = quote! {
      protocheck::validators::comparables::lt(&#field_context_ident, #value_ident, #lt_val)
    };

    self.get_validator_tokens(&validator_expression_tokens)
  }

  pub fn get_lte_validator(&self, lte_val: &TokenStream) -> TokenStream {
    let field_context_ident = self.field_context_ident();

    let value_ident = self.value_ident();
    let validator_expression_tokens = quote! {
      protocheck::validators::comparables::lte(&#field_context_ident, #value_ident, #lte_val)
    };

    self.get_validator_tokens(&validator_expression_tokens)
  }

  pub fn get_gt_validator(&self, gt_val: &TokenStream) -> TokenStream {
    let field_context_ident = self.field_context_ident();

    let value_ident = self.value_ident();
    let validator_expression_tokens = quote! {
      protocheck::validators::comparables::gt(&#field_context_ident, #value_ident, #gt_val)
    };

    self.get_validator_tokens(&validator_expression_tokens)
  }

  pub fn get_gte_validator(&self, gte_val: &TokenStream) -> TokenStream {
    let field_context_ident = self.field_context_ident();

    let value_ident = self.value_ident();
    let validator_expression_tokens = quote! {
      protocheck::validators::comparables::gte(&#field_context_ident, #value_ident, #gte_val)
    };

    self.get_validator_tokens(&validator_expression_tokens)
  }

  pub fn get_constant_validator(&self, const_val: &TokenStream) -> TokenStream {
    let field_context_ident = self.field_context_ident();

    let value_ident = self.value_ident();
    let validator_expression_tokens = quote! {
      protocheck::validators::constants::constant(&#field_context_ident, #value_ident, #const_val)
    };

    self.get_validator_tokens(&validator_expression_tokens)
  }

  pub fn get_in_list_validator(&self, in_list: &TokenStream) -> TokenStream {
    let field_context_ident = self.field_context_ident();

    let value_ident = self.value_ident();
    let validator_expression_tokens = quote! {
      protocheck::validators::containing::in_list(&#field_context_ident, #value_ident, &#in_list)
    };

    self.get_validator_tokens(&validator_expression_tokens)
  }

  pub fn get_not_in_list_validator(&self, not_in_list: &TokenStream) -> TokenStream {
    let field_context_ident = self.field_context_ident();

    let value_ident = self.value_ident();
    let validator_expression_tokens = quote! {
      protocheck::validators::containing::not_in_list(&#field_context_ident, #value_ident, &#not_in_list)
    };

    self.get_validator_tokens(&validator_expression_tokens)
  }

  pub fn get_message_field_validator_tokens(&self) -> TokenStream {
    let Self {
      parent_messages_ident,
      violations_ident,
      ..
    } = self;

    let field_proto_name = &self.proto_name;
    let field_tag = self.tag;
    let field_proto_type: ProtoType = self.field_kind.inner_type().into();
    let item_rust_ident = self.item_rust_ident;

    let nested_key_type = self.key_type_tokens_as_i32();
    let nested_value_type = self.value_type_tokens_as_i32();

    let subscript_tokens = self.subscript_tokens();

    let field_path_element_tokens = quote! {
      protocheck::types::protovalidate::FieldPathElement {
        field_name: Some(#field_proto_name.to_string()),
        field_number: Some(#field_tag as i32),
        field_type: Some(#field_proto_type as i32),
        key_type: #nested_key_type,
        value_type: #nested_value_type,
        subscript: #subscript_tokens,
      };
    };

    if self.is_option() {
      quote! {
        if let Some(msg) = &self.#item_rust_ident {
          let current_nested_field_element = #field_path_element_tokens;

          #parent_messages_ident.push(current_nested_field_element);
          msg.nested_validate(#parent_messages_ident, #violations_ident);
          #parent_messages_ident.pop();
        }
      }
    } else {
      quote! {
        let current_nested_field_element = #field_path_element_tokens;

        #parent_messages_ident.push(current_nested_field_element);
        &self.#item_rust_ident.nested_validate(#parent_messages_ident, #violations_ident);
        #parent_messages_ident.pop();
      }
    }
  }

  pub fn get_required_validation_tokens(&self) -> Option<TokenStream> {
    let field_context_tokens = self.field_context_tokens();
    let field_context_ident = self.field_context_ident();
    let violations_ident = &self.violations_ident;

    self.is_required.then_some(quote! {
      #field_context_tokens
      let required_violation = protocheck::validators::required::required(&#field_context_ident);
      #violations_ident.push(required_violation);
    })
  }

  pub fn get_required_only_validator(&self) -> TokenStream {
    let item_rust_ident = &self.item_rust_ident;
    let required_validation_tokens = self.get_required_validation_tokens();

    quote! {
      if self.#item_rust_ident.is_none() {
        #required_validation_tokens
      }
    }
  }

  pub fn get_aggregated_validator_tokens(&self, tokens: &TokenStream) -> TokenStream {
    let field_context_tokens = self.field_context_tokens();
    let required_check = self.get_required_validation_tokens();
    let item_rust_ident = self.item_rust_ident;

    if self.is_option() {
      quote! {
        match self.#item_rust_ident.as_ref() {
          Some(val) => {
            #field_context_tokens
            #tokens
          },
          None => { #required_check }
        };
      }
    } else {
      quote! {
        #field_context_tokens
        #tokens
      }
    }
  }

  pub fn get_validator_tokens(&self, validator_expression_tokens: &TokenStream) -> TokenStream {
    let violations_ident = &self.violations_ident;

    quote! {
      match #validator_expression_tokens {
        Ok(_) => {}
        Err(v) => #violations_ident.push(v)
      };
    }
  }

  pub fn is_option(&self) -> bool {
    self.is_optional && !self.is_in_oneof
  }

  pub fn key_type_tokens(&self) -> TokenStream {
    self.key_type.map_or(quote! { None }, |key_type| {
      quote! { Some(#key_type) }
    })
  }

  pub fn key_type_tokens_as_i32(&self) -> TokenStream {
    self
      .key_type
      .map_or(quote! { None }, |k| quote! { Some(#k as i32) })
  }

  pub fn value_type_tokens(&self) -> TokenStream {
    self.value_type.map_or(quote! { None }, |value_type| {
      quote! { Some(#value_type) }
    })
  }

  pub fn value_type_tokens_as_i32(&self) -> TokenStream {
    self
      .value_type
      .map_or(quote! { None }, |v| quote! { Some(#v as i32) })
  }

  pub fn subscript_tokens(&self) -> TokenStream {
    match self.field_kind {
      FieldKind::RepeatedItem(_) => {
        let index_ident = self.index_ident;
        quote! { Some(protocheck::types::protovalidate::field_path_element::Subscript::Index(#index_ident as u64)) }
      }
      FieldKind::MapKey(_) | FieldKind::MapValue(_) => {
        if let Some(key_type_enum) = self.key_type {
          let key_subscript_tokens = generate_key_subscript(&key_type_enum, self.key_ident);
          quote! { Some(#key_subscript_tokens) }
        } else {
          quote! { compile_error!("Map key type is missing during macro expansion.") }
        }
      }
      _ => quote! { None },
    }
  }

  pub fn value_ident(&self) -> TokenStream {
    let Self {
      item_rust_ident,
      key_ident,
      map_value_ident,
      item_ident,
      ..
    } = self;
    match self.field_kind {
      FieldKind::RepeatedItem(_) => quote! { #item_ident },
      FieldKind::MapKey(_) => quote! { #key_ident },
      FieldKind::MapValue(_) => quote! { #map_value_ident },
      _ => {
        let val_tokens = if self.is_option() {
          quote! { val }
        } else {
          quote! { self.#item_rust_ident }
        };

        if matches!(self.field_kind.inner_type(), FieldType::String) {
          quote! { &#val_tokens }
        } else {
          val_tokens
        }
      }
    }
  }
}

pub fn generate_key_subscript(key_proto_type: &ProtoType, key_ident: &Ident2) -> TokenStream {
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
