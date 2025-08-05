use proc_macro2::TokenStream;
use protocheck_core::field_data::FieldKind;
use quote::quote;

use crate::{
  rules::FieldData, validator_template::generate_key_subscript, Ident2, ProtoType, Span2,
};

#[derive(Debug, Clone)]
pub(crate) struct ValidationData {
  pub full_name: String,
  pub is_required: bool,
  pub is_optional: bool,
  pub is_in_oneof: bool,
  pub field_data_static_ident: Ident2,
  pub field_span: Span2,
  pub field_data: FieldData,
  pub key_type: Option<ProtoType>,
  pub value_type: Option<ProtoType>,
  pub key_ident: Ident2,
  pub map_value_ident: Ident2,
  pub index_ident: Ident2,
  pub item_ident: Ident2,
  pub item_rust_ident: Ident2,
  pub violations_ident: Ident2,
  pub parent_messages_ident: Ident2,
  pub field_context_ident: Ident2,
  pub field_kind: FieldKind,
}

impl ValidationData {
  pub fn get_required_only_validator(&self) -> TokenStream {
    let item_rust_ident = &self.item_rust_ident;
    let required_validation_tokens = self.get_required_validation_tokens();

    quote! {
      if self.#item_rust_ident.is_none() {
        #required_validation_tokens
      }
    }
  }

  pub fn get_lt_validator(&self, lt_val: TokenStream) -> TokenStream {
    let field_context_ident = &self.field_context_ident;

    let value_ident = self.value_ident();
    let validator_expression_tokens = quote! {
      protocheck::validators::comparables::lt(&#field_context_ident, #value_ident, &#lt_val)
    };

    self.get_validator_tokens(validator_expression_tokens)
  }

  pub fn get_lte_validator(&self, lte_val: TokenStream) -> TokenStream {
    let field_context_ident = &self.field_context_ident;

    let value_ident = self.value_ident();
    let validator_expression_tokens = quote! {
      protocheck::validators::comparables::lte(&#field_context_ident, #value_ident, &#lte_val)
    };

    self.get_validator_tokens(validator_expression_tokens)
  }

  pub fn get_gt_validator(&self, gt_val: TokenStream) -> TokenStream {
    let field_context_ident = &self.field_context_ident;

    let value_ident = self.value_ident();
    let validator_expression_tokens = quote! {
      protocheck::validators::comparables::gt(&#field_context_ident, #value_ident, &#gt_val)
    };

    self.get_validator_tokens(validator_expression_tokens)
  }

  pub fn get_gte_validator(&self, gte_val: TokenStream) -> TokenStream {
    let field_context_ident = &self.field_context_ident;

    let value_ident = self.value_ident();
    let validator_expression_tokens = quote! {
      protocheck::validators::comparables::gte(&#field_context_ident, #value_ident, &#gte_val)
    };

    self.get_validator_tokens(validator_expression_tokens)
  }

  pub fn get_constant_validator(&self, const_val: TokenStream) -> TokenStream {
    let field_context_ident = &self.field_context_ident;

    let value_ident = self.value_ident();
    let validator_expression_tokens = quote! {
      protocheck::validators::constants::constant(&#field_context_ident, #value_ident, &#const_val)
    };

    self.get_validator_tokens(validator_expression_tokens)
  }

  pub fn get_in_list_validator(&self, in_list: TokenStream) -> TokenStream {
    let field_context_ident = &self.field_context_ident;

    let value_ident = self.value_ident();
    let validator_expression_tokens = quote! {
      protocheck::validators::containing::in_list(&#field_context_ident, #value_ident, &#in_list)
    };

    self.get_validator_tokens(validator_expression_tokens)
  }

  pub fn get_not_in_list_validator(&self, in_list: TokenStream) -> TokenStream {
    let field_context_ident = &self.field_context_ident;

    let value_ident = self.value_ident();
    let validator_expression_tokens = quote! {
      protocheck::validators::containing::in_list(&#field_context_ident, #value_ident, &#in_list)
    };

    self.get_validator_tokens(validator_expression_tokens)
  }

  pub fn get_message_field_validator_tokens(&self) -> TokenStream {
    let Self {
      parent_messages_ident,
      violations_ident,
      ..
    } = self;

    let field_proto_name = &self.field_data.proto_name;
    let field_tag = self.field_data.tag;
    let field_proto_type = self.field_data.proto_type;

    let nested_key_type = self.key_type_tokens_as_i32();
    let nested_value_type = self.value_type_tokens_as_i32();

    let subscript_tokens = self.subscript_tokens();

    let value_ident = self.value_ident();

    if self.is_option() {
      quote! {
        if let Some(msg) = #value_ident {
          let current_nested_field_element = protocheck::types::protovalidate::FieldPathElement {
            field_name: Some(#field_proto_name.to_string()),
            field_number: Some(#field_tag as i32),
            field_type: Some(#field_proto_type as i32),
            key_type: #nested_key_type,
            value_type: #nested_value_type,
            subscript: #subscript_tokens,
          };

          #parent_messages_ident.push(current_nested_field_element);
          msg.nested_validate(#parent_messages_ident, #violations_ident);
          #parent_messages_ident.pop();
        }
      }
    } else {
      quote! {
        let current_nested_field_element = protocheck::types::protovalidate::FieldPathElement {
          field_name: Some(#field_proto_name.to_string()),
          field_number: Some(#field_tag as i32),
          field_type: Some(#field_proto_type as i32),
          key_type: #nested_key_type,
          value_type: #nested_value_type,
          subscript: #subscript_tokens,
        };

        #parent_messages_ident.push(current_nested_field_element);
        #value_ident.nested_validate(#parent_messages_ident, #violations_ident);
        #parent_messages_ident.pop();
      }
    }
  }

  pub fn get_required_validation_tokens(&self) -> Option<TokenStream> {
    let field_context_tokens = self.field_context_tokens();
    let field_context_ident = &self.field_context_ident;
    let violations_ident = &self.violations_ident;

    self.is_required.then_some(quote! {
      #field_context_tokens
      let required_violation = protocheck::validators::required(&#field_context_ident);
      #violations_ident.push(required_violation);
    })
  }

  pub fn get_validator_tokens(&self, validator_expression_tokens: TokenStream) -> TokenStream {
    let field_context_tokens = self.field_context_tokens();
    let unwrapped_val_ident = Ident2::new("val", Span2::call_site());
    let value_ident = self.value_ident();
    let required_check = self.get_required_validation_tokens();
    let violations_ident = &self.violations_ident;

    if self.is_option() {
      quote! {
        match #value_ident {
          Some(#unwrapped_val_ident) => {
            #field_context_tokens
            match #validator_expression_tokens {
              Ok(_) => {}
              Err(v) => #violations_ident.push(v)
            };
          },
          None => { #required_check }
        };
      }
    } else {
      quote! {
        #field_context_tokens
        match #validator_expression_tokens {
          Ok(_) => {}
          Err(v) => #violations_ident.push(v)
        };
      }
    }
  }

  pub fn is_option(&self) -> bool {
    self.is_optional && !self.is_in_oneof
  }

  pub fn field_context_tokens(&self) -> TokenStream {
    let Self {
      field_data_static_ident,
      parent_messages_ident,
      field_context_ident,
      ..
    } = self;
    let key_type_tokens = self.key_type_tokens();
    let value_type_tokens = self.value_type_tokens();
    let subscript_tokens = self.subscript_tokens();
    let field_kind = &self.field_kind;

    quote! {
      let #field_context_ident = protocheck::field_data::FieldContext {
        field_data: &#field_data_static_ident,
        parent_elements: #parent_messages_ident.as_slice(),
        subscript: #subscript_tokens,
        key_type: #key_type_tokens,
        value_type: #value_type_tokens,
        field_kind: #field_kind,
      };
    }
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
      FieldKind::RepeatedItem => {
        let index_ident = &self.index_ident;
        quote! { Some(protocheck::types::protovalidate::field_path_element::Subscript::Index(#index_ident as u64)) }
      }
      FieldKind::MapKey | FieldKind::MapValue => {
        if let Some(key_type_enum) = self.key_type {
          let key_subscript_tokens = generate_key_subscript(&key_type_enum, &self.key_ident);
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
      FieldKind::RepeatedItem => quote! { #item_ident },
      FieldKind::MapKey => quote! { #key_ident },
      FieldKind::MapValue => quote! { #map_value_ident },
      _ => {
        if self.is_in_oneof {
          quote! { val }
        } else if self.is_optional {
          quote! { self.#item_rust_ident.as_ref() }
        } else {
          quote! { &self.#item_rust_ident }
        }
      }
    }
  }
}
