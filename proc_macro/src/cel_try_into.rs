use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Error, Type};

use crate::{attribute_extractors::extract_proto_name_attribute, Ident2, Span2, TokenStream2};

enum OuterType {
  Option(InnerType),
  Vec(InnerType),
  HashMap(InnerType),
  Normal(InnerType),
}

enum InnerType {
  Box,
  SupportsInto,
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
      Self::SupportsInto => quote! { #val_tokens.to_owned().into() },
      Self::TryInto => quote! { #val_tokens.to_owned().try_into()? },
      Self::Bytes => quote! { #val_tokens.to_vec().into() },
      Self::F32 => quote! { (*#val_tokens as f64).into() },
      Self::U32 => quote! { (*#val_tokens as u64).into() },
      Self::I32 => quote! { (*#val_tokens as i64).into() },
    }
  }

  pub fn from_type(ty: &Type) -> Self {
    if is_box(ty) {
      Self::Box
    } else if supports_cel_into(ty) {
      Self::SupportsInto
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
      let inner = get_inner_type(ty)?;
      Ok(Self::Option(InnerType::from_type(inner)))
    } else if is_vec(ty) {
      let inner = get_inner_type(ty)?;
      Ok(Self::Vec(InnerType::from_type(inner)))
    } else if is_hashmap(ty) {
      let value_type = get_hashmap_value_type(ty)?;
      Ok(Self::HashMap(InnerType::from_type(value_type)))
    } else {
      Ok(Self::Normal(InnerType::from_type(ty)))
    }
  }
}

fn get_inner_type(ty: &Type) -> Result<&Type, ()> {
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

fn get_hashmap_value_type(ty: &Type) -> Result<&Type, ()> {
  let mut hashmap_value_type: Option<&Type> = None;
  if let syn::Type::Path(type_path) = ty
    && let Some(segment) = type_path.path.segments.last()
      && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
        && let Some(syn::GenericArgument::Type(value_type)) = args.args.get(1) {
          hashmap_value_type = Some(value_type);
        }

  if let Some(output_type) = hashmap_value_type {
    Ok(output_type)
  } else {
    Err(())
  }
}

pub fn derive_cel_value_oneof(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let enum_name = &ast.ident;

  let variants = if let syn::Data::Enum(data_enum) = &ast.data {
    &data_enum.variants
  } else {
    return syn::Error::new_spanned(ast, "OneofTryIntoCelValue can only be used on enums")
      .to_compile_error()
      .into();
  };

  let max_recursion_depth = quote! { 10 };

  let mut match_arms = Vec::<TokenStream2>::new();

  for variant in variants {
    let variant_ident = &variant.ident;
    let mut proto_name: String = String::new();

    for attr in variant.attrs.iter() {
      if attr.path().is_ident("protocheck") {
        match attr.parse_nested_meta(|meta| {
          proto_name =
            extract_proto_name_attribute(&enum_name.to_string(), attr, variant_ident, meta)?;
          Ok(())
        }) {
          Ok(_) => {}
          Err(e) => return e.to_compile_error().into(),
        };

        break;
      }
    }

    if let syn::Fields::Unnamed(fields) = &variant.fields
      && let Some(variant_type) = &fields.unnamed.get(0) {
        let type_ident = &variant_type.ty;
        let val_ident = format_ident!("v");

        let into_expression = InnerType::from_type(type_ident).conversion_tokens(&quote! { #val_ident });

        let arm = quote! {
          #enum_name::#variant_ident(#val_ident) => {
            if depth >= #max_recursion_depth {
              Ok((#proto_name.to_string(), ::cel::Value::Null))
            } else {
              Ok((#proto_name.to_string(), #into_expression))
            }
          }
        };
        match_arms.push(arm);
      }
  }

  let expanded = quote! {
    impl #enum_name {
      pub fn try_into_cel_value(&self) -> Result<(String, ::cel::Value), ::protocheck::types::cel::CelConversionError> {
        self.try_into_cel_value_recursive(0)
      }

      fn try_into_cel_value_recursive(&self, depth: usize) -> Result<(String, ::cel::Value), ::protocheck::types::cel::CelConversionError> {
         match self {
          #(#match_arms),*
        }
      }
    }
  };

  expanded.into()
}

pub(crate) fn derive_cel_value_struct(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);

  let struct_name = &ast.ident;

  let fields = if let syn::Data::Struct(syn::DataStruct {
    fields: syn::Fields::Named(fields),
    ..
  }) = &ast.data
  {
    &fields.named
  } else {
    panic!("This derive macro only works on structs with named fields");
  };

  let fields_map_ident = Ident2::new("fields", Span2::call_site());
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
      let outer_type = match OuterType::try_from(field_type) {
        Ok(ty) => ty,
        Err(_) => {
          return Error::new_spanned(
            field,
            format!(
              "Could not parse the outer type for field {} in struct {}",
              field_name, struct_name
            ),
          )
          .to_compile_error()
          .into()
        }
      };

      let val_ident = format_ident!("v");
      let val_tokens = quote! { #val_ident };

      match outer_type {
        OuterType::Option(inner) => {
          let conversion_tokens = inner.conversion_tokens(&val_tokens);
          tokens.extend(quote! {
            if let Some(#val_ident) = &value.#field_ident {
              #fields_map_ident.insert(#field_name.into(), #conversion_tokens);
            } else {
              #fields_map_ident.insert(#field_name.into(), ::cel::Value::Null);
            }
          });
        }
        OuterType::Vec(inner) => {
          let conversion_tokens = inner.conversion_tokens(&val_tokens);
          tokens.extend(quote! {
            let mut converted: Vec<::cel::Value> = Vec::new();
            for #val_ident in &value.#field_ident {
              converted.push(#conversion_tokens);
            }

            #fields_map_ident.insert(#field_name.into(), ::cel::Value::List(converted.into()));
          });
        }

        OuterType::HashMap(inner) => {
          let conversion_tokens = inner.conversion_tokens(&val_tokens);
          tokens.extend(quote! {
            let mut field_map: ::std::collections::HashMap<::cel::objects::Key, ::cel::Value> = ::std::collections::HashMap::new();

            for (k, #val_ident) in &value.#field_ident {
              field_map.insert(k.clone().into(), #conversion_tokens);
            }

            #fields_map_ident.insert(#field_name.into(), ::cel::Value::Map(field_map.into()));
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

  let expanded = quote! {
    impl #struct_name {
      fn try_into_cel_value_recursive(&self, depth: usize) -> Result<::cel::Value, ::protocheck::types::cel::CelConversionError> {
        if depth >= #max_recursion_depth {
          return Ok(::cel::Value::Null);
        }

        let mut #fields_map_ident: ::std::collections::HashMap<::cel::objects::Key, ::cel::Value> = std::collections::HashMap::new();
        let value = self;

        #tokens

        Ok(::cel::Value::Map(#fields_map_ident.into()))
      }
    }

    impl TryFrom<#struct_name> for ::cel::Value {
      type Error = ::protocheck::types::cel::CelConversionError;

      fn try_from(value: #struct_name) -> Result<Self, Self::Error> {
        value.try_into_cel_value_recursive(0)
      }
    }
  };

  expanded.into()
}

fn type_matches_path(ty: &Type, target_path: &str) -> bool {
  if let Ok(path) = syn::parse_str::<syn::Path>(target_path) {
    return ty.to_token_stream().to_string() == path.to_token_stream().to_string();
  }
  false
}

fn supports_cel_into(ty: &Type) -> bool {
  is_primitive(ty)
    || type_matches_path(ty, "::protocheck::types::FieldMask")
    || type_matches_path(ty, "::protocheck::types::Empty")
    || type_matches_path(ty, "::protocheck::types::Any")
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

fn is_primitive(ty: &Type) -> bool {
  if let syn::Type::Path(type_path) = ty
    && let Some(segment) = type_path.path.segments.last() {
      let type_name = segment.ident.to_string();

      return matches!(
        type_name.as_str(),
        "bool"
          | "i64"
          | "u64"
          | "f64"
          | "str"
          | "String"
      );
    }
  false
}
