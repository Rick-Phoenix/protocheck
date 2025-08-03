use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Type};

use crate::{attribute_extractors::extract_proto_name_attribute, Ident2, Span2, TokenStream2};

enum CelConversionKind {
  DirectConversion,
  TryIntoConversion,
}

enum OuterType {
  Option {
    inner: CelConversionKind,
    is_box: bool,
  },
  Vec(CelConversionKind),
  HashMap(CelConversionKind),
  Scalar,
}

impl OuterType {
  fn from_type(ty: &Type) -> Self {
    if is_option(ty) {
      let inner = get_inner_type(ty).expect("Failed to unwrap inner in is_option");
      if is_box(inner) {
        Self::Option {
          inner: CelConversionKind::from_type(
            get_inner_type(inner).expect("Failed to unwrap nested inner in is_box"),
          ),
          is_box: true,
        }
      } else {
        Self::Option {
          inner: CelConversionKind::from_type(inner),
          is_box: false,
        }
      }
    } else if is_vec(ty) {
      Self::Vec(CelConversionKind::from_type(
        get_inner_type(ty).expect("Failed to unwrap inner type in is_vec"),
      ))
    } else if is_hashmap(ty) {
      Self::HashMap(CelConversionKind::from_type(
        get_hashmap_value_type(ty).expect("Failed to unwrap inner type in is_hashmap"),
      ))
    } else {
      Self::Scalar
    }
  }
}

fn get_inner_type(ty: &Type) -> Option<&Type> {
  if let syn::Type::Path(type_path) = ty {
    if let Some(segment) = type_path.path.segments.last() {
      if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
        if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
          return Some(inner_type);
        }
      }
    }
  }
  None
}

fn get_hashmap_value_type(ty: &Type) -> Option<&Type> {
  if let syn::Type::Path(type_path) = ty {
    if let Some(segment) = type_path.path.segments.last() {
      if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
        if let Some(syn::GenericArgument::Type(value_type)) = args.args.get(1) {
          return Some(value_type);
        }
      }
    }
  }
  None
}

impl CelConversionKind {
  pub fn from_type(ty: &Type) -> Self {
    if supports_cel_into(ty) {
      Self::DirectConversion
    } else {
      Self::TryIntoConversion
    }
  }
}

pub(crate) fn derive_cel_value_oneof(input: TokenStream) -> TokenStream {
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

    if let syn::Fields::Unnamed(fields) = &variant.fields {
      if fields.unnamed.len() == 1 {
        let variant_type = &fields.unnamed.get(0).unwrap().ty;

        let into_expression = if is_box(variant_type) {
          quote! { (*val).try_into_cel_value_recursive(depth + 1)? }
        } else {
          match CelConversionKind::from_type(variant_type) {
            CelConversionKind::DirectConversion => {
              quote! { val.to_owned().into() }
            }
            CelConversionKind::TryIntoConversion => {
              quote! { val.to_owned().try_into()? }
            }
          }
        };

        let arm = quote! {
          #enum_name::#variant_ident(val) => {
            if depth >= #max_recursion_depth {
              Ok((#proto_name.to_string(), cel_interpreter::Value::Null))
            } else {
              Ok((#proto_name.to_string(), #into_expression))
            }
          }
        };
        match_arms.push(arm);
      }
    }
  }

  let expanded = quote! {
    impl #enum_name {
      pub fn try_into_cel_value(&self) -> Result<(String, ::cel_interpreter::Value), protocheck::validators::cel::CelConversionError> {
        self.try_into_cel_value_recursive(0)
      }

      pub fn try_into_cel_value_recursive(&self, depth: usize) -> Result<(String, ::cel_interpreter::Value), protocheck::validators::cel::CelConversionError> {
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
      let outer_type = OuterType::from_type(field_type);

      match outer_type {
        OuterType::Option { inner, is_box } => {
          if is_box {
            tokens.extend(quote! {
              if let Some(v) = value.#field_ident.as_deref() {
                #fields_map_ident.insert(#field_name.into(), (*v).try_into_cel_value_recursive(depth + 1)?);
              } else {
                #fields_map_ident.insert(#field_name.into(), cel_interpreter::Value::Null);
              }
            });
          } else {
            match inner {
              CelConversionKind::DirectConversion => {
                tokens.extend(quote! {
                  if let Some(v) = &value.#field_ident {
                    #fields_map_ident.insert(#field_name.into(), v.into());
                  } else {
                    #fields_map_ident.insert(#field_name.into(), cel_interpreter::Value::Null);
                  }
                });
              }
              CelConversionKind::TryIntoConversion => {
                tokens.extend(quote! {
                  if let Some(v) = value.#field_ident {
                    #fields_map_ident.insert(#field_name.into(), v.try_into()?);
                  } else {
                    #fields_map_ident.insert(#field_name.into(), cel_interpreter::Value::Null);
                  }
                });
              }
            }
          }
        }
        OuterType::Vec(kind) => {
          let cel_val = match kind {
            CelConversionKind::DirectConversion => {
              quote! {
                converted.push(item.into());
              }
            }
            CelConversionKind::TryIntoConversion => {
              quote! {
                converted.push(item.to_owned().try_into_cel_value_recursive(depth + 1)?);
              }
            }
          };

          tokens.extend(quote! {
            let mut converted: Vec<cel_interpreter::Value> = Vec::new();
            for item in &value.#field_ident {
              #cel_val
            }

            #fields_map_ident.insert(#field_name.into(), cel_interpreter::Value::List(converted.into()));
          });
        }

        OuterType::HashMap(value_kind) => {
          let cel_val = match value_kind {
            CelConversionKind::DirectConversion => {
              quote! {
                v.into();
              }
            }
            CelConversionKind::TryIntoConversion => {
              quote! {
                v.to_owned().try_into()?;
              }
            }
          };

          tokens.extend(quote! {
            let mut field_map: std::collections::HashMap<cel_interpreter::objects::Key, cel_interpreter::Value> = std::collections::HashMap::new();

            for (k, v) in &value.#field_ident {
              let cel_val = #cel_val;
              field_map.insert(k.clone().into(), cel_val);
            }

            #fields_map_ident.insert(#field_name.into(), cel_interpreter::Value::Map(field_map.into()));
          });
        }
        OuterType::Scalar => {
          tokens.extend(quote! {
            #fields_map_ident.insert(#field_name.into(), value.#field_ident.into());
          });
        }
      };
    }
  }

  let expanded = quote! {
    impl #struct_name {
      pub fn try_into_cel_value_recursive(&self, depth: usize) -> Result<::cel_interpreter::Value, protocheck::validators::cel::CelConversionError> {
        if depth >= #max_recursion_depth {
          return Ok(::cel_interpreter::Value::Null);
        }

        let mut #fields_map_ident: std::collections::HashMap<cel_interpreter::objects::Key, cel_interpreter::Value> = std::collections::HashMap::new();
        let value = self;

        #tokens

        Ok(cel_interpreter::Value::Map(#fields_map_ident.into()))
      }
    }

    impl TryFrom<#struct_name> for ::cel_interpreter::Value {
      type Error = protocheck::validators::cel::CelConversionError;

      fn try_from(value: #struct_name) -> Result<Self, Self::Error> {
        value.clone().try_into_cel_value_recursive(0)
      }
    }
  };

  expanded.into()
}

fn type_matches_path(ty: &Type, target_path: &str) -> bool {
  let path: syn::Path =
    syn::parse_str(target_path).expect("Failed to unwrap type in type_matches_path");
  ty.to_token_stream().to_string() == path.to_token_stream().to_string()
}

fn is_option(ty: &Type) -> bool {
  if let syn::Type::Path(type_path) = ty {
    if let Some(segment) = type_path.path.segments.last() {
      return segment.ident == "Option";
    }
  }
  false
}

fn is_box(ty: &Type) -> bool {
  if let syn::Type::Path(type_path) = ty {
    if let Some(segment) = type_path.path.segments.last() {
      return segment.ident == "Box";
    }
  }
  false
}

fn is_vec(ty: &Type) -> bool {
  if let syn::Type::Path(type_path) = ty {
    if let Some(segment) = type_path.path.segments.last() {
      return segment.ident == "Vec";
    }
  }
  false
}

fn is_hashmap(ty: &Type) -> bool {
  if let syn::Type::Path(type_path) = ty {
    if let Some(segment) = type_path.path.segments.last() {
      return segment.ident == "HashMap";
    }
  }
  false
}

fn supports_cel_into(ty: &Type) -> bool {
  is_primitive(ty)
    || type_matches_path(ty, "protocheck::types::FieldMask")
    || type_matches_path(ty, "protocheck::types::Empty")
}

fn is_primitive(ty: &Type) -> bool {
  if let syn::Type::Path(type_path) = ty {
    if let Some(segment) = type_path.path.segments.last() {
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
          | "f32"
          | "f64"
          | "char"
          | "str"
          | "String"
      );
    }
  }
  false
}
