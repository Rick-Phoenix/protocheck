use crate::*;

enum OuterType {
  Option(InnerType),
  Vec(InnerType),
  HashMap(InnerType, InnerType),
  Normal(InnerType),
}

enum InnerType {
  Box,
  TryInto,
  Bytes,
  F32,
  U32,
  I32,
}

impl InnerType {
  pub fn conversion_tokens(&self, val_tokens: &TokenStream2) -> TokenStream2 {
    match self {
      Self::Box => quote! { (*#val_tokens).try_into_cel_value_recursive(depth + 1)? },
      Self::TryInto => {
        quote! { #val_tokens.clone().try_into().map_err(::protocheck::types::cel::CelConversionError::from)? }
      }
      Self::Bytes => quote! { #val_tokens.to_vec().into() },
      Self::F32 => quote! { (*#val_tokens as f64).into() },
      Self::U32 => quote! { (*#val_tokens as u64).into() },
      Self::I32 => quote! { (*#val_tokens as i64).into() },
    }
  }

  pub fn from_type(ty: &Type) -> Self {
    if is_box(ty) {
      Self::Box
    } else if is_bytes(ty) {
      Self::Bytes
    } else if is_f32(ty) {
      Self::F32
    } else if is_u32(ty) {
      Self::U32
    } else if is_i32(ty) {
      Self::I32
    } else {
      Self::TryInto
    }
  }
}

impl TryFrom<&Type> for OuterType {
  type Error = ();

  fn try_from(ty: &Type) -> Result<Self, Self::Error> {
    if is_option(ty) {
      let inner = get_type_argument(ty)?;
      Ok(Self::Option(InnerType::from_type(inner)))
    } else if is_vec(ty) {
      let inner = get_type_argument(ty)?;
      Ok(Self::Vec(InnerType::from_type(inner)))
    } else if is_hashmap(ty) {
      let (keys_type, values_type) = get_hashmap_types(ty)?;
      Ok(Self::HashMap(
        InnerType::from_type(keys_type),
        InnerType::from_type(values_type),
      ))
    } else {
      Ok(Self::Normal(InnerType::from_type(ty)))
    }
  }
}

fn get_type_argument(ty: &Type) -> Result<&Type, ()> {
  let mut output: Option<&Type> = None;
  if let syn::Type::Path(type_path) = ty
    && let Some(segment) = type_path.path.segments.last()
      && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
        && let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
          output = Some(inner_type);
        }

  if let Some(output_type) = output {
    Ok(output_type)
  } else {
    Err(())
  }
}

fn get_hashmap_types(ty: &Type) -> Result<(&Type, &Type), ()> {
  let mut hashmap_values_type: Option<&Type> = None;
  let mut hashmap_keys_type: Option<&Type> = None;
  if let syn::Type::Path(type_path) = ty
    && let Some(segment) = type_path.path.segments.last()
      && let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
        if let Some(syn::GenericArgument::Type(keys_type)) = args.args.get(0) {
          hashmap_keys_type = Some(keys_type);
        }
        if let Some(syn::GenericArgument::Type(value_type)) = args.args.get(1) {
          hashmap_values_type = Some(value_type);
        }
  }

  if let Some(keys_type) = hashmap_keys_type && let Some(values_type) = hashmap_values_type {
    Ok((keys_type, values_type))
  } else {
    Err(())
  }
}

pub fn derive_cel_value_oneof(item: ItemEnum) -> Result<TokenStream2, Error> {
  let enum_name = &item.ident;

  let variants = &item.variants;

  let max_recursion_depth = quote! { 10 };

  let mut match_arms = Vec::<TokenStream2>::new();

  for variant in variants {
    let variant_ident = &variant.ident;
    let mut proto_name: String = String::new();

    if proto_name.is_empty() {
      proto_name = variant_ident.to_string().to_case(Case::Snake);
    }

    if let syn::Fields::Unnamed(fields) = &variant.fields
      && let Some(variant_type) = &fields.unnamed.get(0) {
        let type_ident = &variant_type.ty;
        let val_ident = new_ident("v");

        let into_expression = InnerType::from_type(type_ident).conversion_tokens(&quote! { #val_ident });

        let arm = quote! {
          #enum_name::#variant_ident(#val_ident) => {
            if depth >= #max_recursion_depth {
              Ok((#proto_name.to_string(), ::cel_derive::CelValue::Null))
            } else {
              Ok((#proto_name.to_string(), #into_expression))
            }
          }
        };
        match_arms.push(arm);
      }
  }

  Ok(quote! {
    impl #enum_name {
      pub fn try_into_cel_value(&self) -> Result<(String, ::cel_derive::CelValue), ::cel_derive::CelConversionError> {
        self.try_into_cel_value_recursive(0)
      }

      fn try_into_cel_value_recursive(&self, depth: usize) -> Result<(String, ::cel_derive::CelValue), ::cel_derive::CelConversionError> {
         match self {
          #(#match_arms),*
        }
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
      let outer_type = OuterType::try_from(field_type).map_err(|_| {
        error!(
          field,
          format!(
            "Could not parse the outer type for field {} in struct {}",
            field_name, struct_name
          )
        )
      })?;

      let val_ident = new_ident("v");
      let val_tokens = quote! { #val_ident };

      match outer_type {
        OuterType::Option(inner) => {
          let conversion_tokens = inner.conversion_tokens(&val_tokens);
          tokens.extend(quote! {
            if let Some(#val_ident) = &value.#field_ident {
              #fields_map_ident.insert(#field_name.into(), #conversion_tokens);
            } else {
              #fields_map_ident.insert(#field_name.into(), ::protocheck::cel::Value::Null);
            }
          });
        }
        OuterType::Vec(inner) => {
          let conversion_tokens = inner.conversion_tokens(&val_tokens);
          tokens.extend(quote! {
            let mut converted: Vec<::protocheck::cel::Value> = Vec::new();
            for #val_ident in &value.#field_ident {
              converted.push(#conversion_tokens);
            }

            #fields_map_ident.insert(#field_name.into(), ::protocheck::cel::Value::List(converted.into()));
          });
        }

        OuterType::HashMap(keys_type, values_type) => {
          let keys_ident = new_ident("key");
          let keys_conversion_tokens = keys_type.conversion_tokens(&quote! { #keys_ident });
          let values_conversion_tokens = values_type.conversion_tokens(&val_tokens);
          tokens.extend(quote! {
            let mut field_map: ::std::collections::HashMap<::protocheck::cel::objects::Key, ::protocheck::cel::Value> = ::std::collections::HashMap::new();

            for (#keys_ident, #val_ident) in &value.#field_ident {
              field_map.insert(#keys_conversion_tokens, #values_conversion_tokens);
            }

            #fields_map_ident.insert(#field_name.into(), ::protocheck::cel::Value::Map(field_map.into()));
          });
        }
        OuterType::Normal(ty) => {
          let val_tokens = quote! { (&value.#field_ident) };
          let conversion_tokens = ty.conversion_tokens(&val_tokens);

          tokens.extend(quote! {
            #fields_map_ident.insert(#field_name.into(), #conversion_tokens);
          });
        }
      };
    }
  }

  Ok(quote! {
    impl #struct_name {
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

fn type_matches_path(ty: &Type, target_path: &str) -> bool {
  if let Ok(path) = syn::parse_str::<syn::Path>(target_path) {
    return ty.to_token_stream().to_string() == path.to_token_stream().to_string();
  }
  false
}

fn is_bytes(ty: &Type) -> bool {
  type_matches_path(ty, "::prost::bytes::Bytes")
}

fn is_option(ty: &Type) -> bool {
  if let syn::Type::Path(type_path) = ty
    && let Some(segment) = type_path.path.segments.last() {
      return segment.ident == "Option";
    }
  false
}

fn is_box(ty: &Type) -> bool {
  if let syn::Type::Path(type_path) = ty
    && let Some(segment) = type_path.path.segments.last() {
      return segment.ident == "Box";
    }
  false
}

fn is_vec(ty: &Type) -> bool {
  if let syn::Type::Path(type_path) = ty
    && let Some(segment) = type_path.path.segments.last() {
      return segment.ident == "Vec";
    }
  false
}

fn is_hashmap(ty: &Type) -> bool {
  if let syn::Type::Path(type_path) = ty
    && let Some(segment) = type_path.path.segments.last() {
      return segment.ident == "HashMap";
    }
  false
}

fn is_f32(ty: &Type) -> bool {
  if let syn::Type::Path(type_path) = ty
    && let Some(segment) = type_path.path.segments.last() {
      return segment.ident == "f32";
    }
  false
}

fn is_u32(ty: &Type) -> bool {
  if let syn::Type::Path(type_path) = ty
    && let Some(segment) = type_path.path.segments.last() {
      return segment.ident == "u32";
    }
  false
}

fn is_i32(ty: &Type) -> bool {
  if let syn::Type::Path(type_path) = ty
    && let Some(segment) = type_path.path.segments.last() {
      return segment.ident == "i32";
    }
  false
}

fn new_ident(name: &str) -> Ident {
  Ident::new(name, Span::call_site())
}
