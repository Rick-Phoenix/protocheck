use syn_utils::{Float, Int, RustType, TypeInfo, Uint};

use crate::*;

pub fn get_conversion_tokens(type_info: &TypeInfo, val_tokens: &TokenStream2) -> TokenStream2 {
  match type_info.type_.as_ref() {
    RustType::Box(_) => quote! { (*#val_tokens).try_into_cel_value_recursive(depth + 1)? },
    RustType::Bytes => quote! { #val_tokens.to_vec().into() },
    RustType::Float(Float::F32) => quote! { (*#val_tokens as f64).into() },
    RustType::Uint(Uint::U32) => quote! { (*#val_tokens as u64).into() },
    RustType::Int(Int::I32) => quote! { (*#val_tokens as i64).into() },
    _ => {
      quote! { #val_tokens.clone().try_into().map_err(::protocheck::types::cel::CelConversionError::from)? }
    }
  }
}

pub fn derive_cel_value_oneof(item: ItemEnum) -> Result<TokenStream2, Error> {
  let enum_name = &item.ident;

  let variants = &item.variants;

  let max_recursion_depth = quote! { 10 };

  let mut match_arms = Vec::<TokenStream2>::new();

  for variant in variants {
    let variant_ident = &variant.ident;
    let proto_name = variant_ident.to_string().to_case(Case::Snake);

    if let syn::Fields::Unnamed(fields) = &variant.fields
      && let Some(variant_type) = &fields.unnamed.get(0) {
        let type_ident = &variant_type.ty;
        let val_ident = new_ident("v");

        let type_info = TypeInfo::from_type(type_ident)?;

        let into_expression = get_conversion_tokens(&type_info, &quote! { #val_ident });

        let arm = quote! {
          #enum_name::#variant_ident(#val_ident) => {
            if depth >= #max_recursion_depth {
              Ok((#proto_name.to_string(), ::protocheck::cel::Value::Null))
            } else {
              Ok((#proto_name.to_string(), #into_expression))
            }
          }
        };
        match_arms.push(arm);
      }
  }

  // We cannot rely on the try_into impl as is here, because we need to know
  // the name of the specific oneof variant being used, so we need this helper here.
  // In the future we might skip this and just use the name of the oneof, to mirror
  // the rust side of things more accurately
  Ok(quote! {
    impl #enum_name {
      #[doc(hidden)]
      pub fn try_into_cel_value(&self) -> Result<(String, ::protocheck::cel::Value), ::protocheck::types::cel::CelConversionError> {
        self.try_into_cel_value_recursive(0)
      }

      #[doc(hidden)]
      fn try_into_cel_value_recursive(&self, depth: usize) -> Result<(String, ::protocheck::cel::Value), ::protocheck::types::cel::CelConversionError> {
         match self {
          #(#match_arms),*
        }
      }
    }

    impl TryFrom<#enum_name> for ::protocheck::cel::Value {
      type Error = ::protocheck::types::cel::CelConversionError;

      fn try_from(value: #enum_name) -> Result<Self, Self::Error> {
        Ok(value.try_into_cel_value_recursive(0)?.1)
      }
    }
  })
}

pub(crate) fn derive_cel_value_struct(item: ItemStruct) -> Result<TokenStream2, Error> {
  let struct_name = &item.ident;

  let fields = if let syn::Fields::Named(fields) = &item.fields {
    &fields.named
  } else {
    bail!(
      item,
      "This derive macro only works on structs with named fields"
    );
  };

  let fields_map_ident = Ident::new("fields", Span::call_site());
  let mut tokens = TokenStream2::new();

  let max_recursion_depth = quote! { 10 };

  for field in fields {
    let field_ident = field.ident.as_ref().unwrap();
    let field_name = field_ident.to_string();
    let field_type = &field.ty;
    let mut is_oneof = false;

    for attr in &field.attrs {
      if attr.path().is_ident("prost") {
        let _ = attr.parse_nested_meta(|meta| {
          if meta.path.is_ident("oneof") {
            is_oneof = true;
          }
          Ok(())
        });
      }
    }

    if is_oneof {
      tokens.extend(quote! {
        if let Some(oneof) = &value.#field_ident {
          let (oneof_field_name, cel_val) = oneof.try_into_cel_value()?;
          #fields_map_ident.insert(oneof_field_name.into(), cel_val);
        }
      });
    } else {
      let outer_type = TypeInfo::from_type(field_type)?;

      let val_ident = new_ident("v");
      let val_tokens = quote! { #val_ident };

      match outer_type.type_.as_ref() {
        RustType::Option(inner) => {
          let conversion_tokens = get_conversion_tokens(inner, &val_tokens);

          tokens.extend(quote! {
            if let Some(#val_ident) = &value.#field_ident {
              #fields_map_ident.insert(#field_name.into(), #conversion_tokens);
            } else {
              #fields_map_ident.insert(#field_name.into(), ::protocheck::cel::Value::Null);
            }
          });
        }
        RustType::Vec(inner) => {
          let conversion_tokens = get_conversion_tokens(inner, &val_tokens);

          tokens.extend(quote! {
            let mut converted: Vec<::protocheck::cel::Value> = Vec::new();
            for #val_ident in &value.#field_ident {
              converted.push(#conversion_tokens);
            }

            #fields_map_ident.insert(#field_name.into(), ::protocheck::cel::Value::List(converted.into()));
          });
        }

        RustType::HashMap((k, v)) => {
          let keys_ident = new_ident("key");
          let keys_conversion_tokens = get_conversion_tokens(k, &quote! { #keys_ident });
          let values_conversion_tokens = get_conversion_tokens(v, &val_tokens);
          tokens.extend(quote! {
            let mut field_map: ::std::collections::HashMap<::protocheck::cel::objects::Key, ::protocheck::cel::Value> = ::std::collections::HashMap::new();

            for (#keys_ident, #val_ident) in &value.#field_ident {
              field_map.insert(#keys_conversion_tokens, #values_conversion_tokens);
            }

            #fields_map_ident.insert(#field_name.into(), ::protocheck::cel::Value::Map(field_map.into()));
          });
        }
        _ => {
          let val_tokens = quote! { (&value.#field_ident) };
          let conversion_tokens = get_conversion_tokens(&outer_type, &val_tokens);

          tokens.extend(quote! {
            #fields_map_ident.insert(#field_name.into(), #conversion_tokens);
          });
        }
      };
    }
  }

  Ok(quote! {
    impl #struct_name {
      #[doc(hidden)]
      fn try_into_cel_value_recursive(&self, depth: usize) -> Result<::protocheck::cel::Value, ::protocheck::types::cel::CelConversionError> {
        if depth >= #max_recursion_depth {
          return Ok(::protocheck::cel::Value::Null);
        }

        let mut #fields_map_ident: ::std::collections::HashMap<::protocheck::cel::objects::Key, ::protocheck::cel::Value> = std::collections::HashMap::new();
        let value = self;

        #tokens

        Ok(::protocheck::cel::Value::Map(#fields_map_ident.into()))
      }
    }

    impl TryFrom<#struct_name> for ::protocheck::cel::Value {
      type Error = ::protocheck::types::cel::CelConversionError;

      fn try_from(value: #struct_name) -> Result<Self, Self::Error> {
        value.try_into_cel_value_recursive(0)
      }
    }
  })
}
