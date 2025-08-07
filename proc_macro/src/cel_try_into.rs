use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Error, Type};

use crate::{attribute_extractors::extract_proto_name_attribute, Ident2, Span2, TokenStream2};

enum OuterType {
  Option {
    conversion_tokens: Option<TokenStream2>,
    is_box: bool,
  },
  Vec {
    conversion_tokens: TokenStream2,
  },
  HashMap {
    conversion_tokens: TokenStream2,
  },
  Normal {
    is_f32: bool,
  },
}

fn get_conversion_tokens(ty: &Type) -> TokenStream2 {
  if supports_cel_into(ty) {
    quote! { v.to_owned().into() }
  } else if is_bytes(ty) {
    quote! { v.to_vec().into() }
  } else if is_f32(ty) {
    quote! { (*v as f64).into() }
  } else {
    quote! { v.to_owned().try_into()? }
  }
}

impl TryFrom<&Type> for OuterType {
  type Error = ();

  fn try_from(ty: &Type) -> Result<Self, Self::Error> {
    if is_option(ty) {
      let inner = get_inner_type(ty)?;
      if is_box(inner) {
        Ok(Self::Option {
          conversion_tokens: None,
          is_box: true,
        })
      } else {
        Ok(Self::Option {
          conversion_tokens: Some(get_conversion_tokens(inner)),
          is_box: false,
        })
      }
    } else if is_vec(ty) {
      let inner = get_inner_type(ty)?;
      Ok(Self::Vec {
        conversion_tokens: get_conversion_tokens(inner),
      })
    } else if is_hashmap(ty) {
      let value_type = get_hashmap_value_type(ty)?;
      Ok(Self::HashMap {
        conversion_tokens: get_conversion_tokens(value_type),
      })
    } else {
      Ok(Self::Normal { is_f32: is_f32(ty) })
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

        let into_expression = if is_box(type_ident) {
          quote! { (*v).try_into_cel_value_recursive(depth + 1)? }
        } else {
          let conversion_tokens = get_conversion_tokens(type_ident);
          quote! { #conversion_tokens }
        };

        let arm = quote! {
          #enum_name::#variant_ident(v) => {
            if depth >= #max_recursion_depth {
              Ok((#proto_name.to_string(), ::cel_interpreter::Value::Null))
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
      pub fn try_into_cel_value(&self) -> Result<(String, ::cel_interpreter::Value), ::protocheck::types::cel::CelConversionError> {
        self.try_into_cel_value_recursive(0)
      }

      pub fn try_into_cel_value_recursive(&self, depth: usize) -> Result<(String, ::cel_interpreter::Value), ::protocheck::types::cel::CelConversionError> {
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

      match outer_type {
        OuterType::Option {
          conversion_tokens,
          is_box,
        } => {
          if is_box {
            tokens.extend(quote! {
              if let Some(boxed_val) = value.#field_ident.as_deref() {
                #fields_map_ident.insert(#field_name.into(), (*boxed_val).try_into_cel_value_recursive(depth + 1)?);
              } else {
                #fields_map_ident.insert(#field_name.into(), ::cel_interpreter::Value::Null);
              }
            });
          } else {
            tokens.extend(quote! {
              if let Some(v) = &value.#field_ident {
                #fields_map_ident.insert(#field_name.into(), #conversion_tokens);
              } else {
                #fields_map_ident.insert(#field_name.into(), ::cel_interpreter::Value::Null);
              }
            });
          }
        }
        OuterType::Vec { conversion_tokens } => {
          tokens.extend(quote! {
            let mut converted: Vec<::cel_interpreter::Value> = Vec::new();
            for v in &value.#field_ident {
              converted.push(#conversion_tokens);
            }

            #fields_map_ident.insert(#field_name.into(), ::cel_interpreter::Value::List(converted.into()));
          });
        }

        OuterType::HashMap { conversion_tokens } => {
          tokens.extend(quote! {
            let mut field_map: std::collections::HashMap<::cel_interpreter::objects::Key, ::cel_interpreter::Value> = std::collections::HashMap::new();

            for (k, v) in &value.#field_ident {
              field_map.insert(k.clone().into(), #conversion_tokens);
            }

            #fields_map_ident.insert(#field_name.into(), ::cel_interpreter::Value::Map(field_map.into()));
          });
        }
        OuterType::Normal { is_f32 } => {
          if is_f32 {
            tokens.extend(quote! {
              #fields_map_ident.insert(#field_name.into(), (value.#field_ident as f64).into());
            });
          } else {
            tokens.extend(quote! {
              #fields_map_ident.insert(#field_name.into(), value.#field_ident.to_owned().into());
            });
          }
        }
      };
    }
  }

  let expanded = quote! {
    impl #struct_name {
      pub fn try_into_cel_value_recursive(&self, depth: usize) -> Result<::cel_interpreter::Value, ::protocheck::types::cel::CelConversionError> {
        if depth >= #max_recursion_depth {
          return Ok(::cel_interpreter::Value::Null);
        }

        let mut #fields_map_ident: std::collections::HashMap<::cel_interpreter::objects::Key, ::cel_interpreter::Value> = std::collections::HashMap::new();
        let value = self;

        #tokens

        Ok(::cel_interpreter::Value::Map(#fields_map_ident.into()))
      }
    }

    impl TryFrom<#struct_name> for ::cel_interpreter::Value {
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

fn is_primitive(ty: &Type) -> bool {
  if let syn::Type::Path(type_path) = ty
    && let Some(segment) = type_path.path.segments.last() {
      let type_name = segment.ident.to_string();

      return matches!(
        type_name.as_str(),
        "bool"
          | "i8"
          | "i16"
          | "i32"
          | "i64"
          | "i128"
          | "u8"
          | "u16"
          | "u32"
          | "u64"
          | "u128"
          | "f64"
          | "char"
          | "str"
          | "String"
      );
    }
  false
}
