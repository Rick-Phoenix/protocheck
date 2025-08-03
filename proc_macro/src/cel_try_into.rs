use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Error, LitStr, Type};

use crate::{Ident2, Span2, TokenStream2};

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

  let mut match_arms = Vec::<TokenStream2>::new();

  for variant in variants {
    let variant_ident = &variant.ident;
    let mut proto_name: String = String::new();

    for attr in variant.attrs.iter() {
      if attr.path().is_ident("protocheck") {
        match attr.parse_nested_meta(|meta| {
          if meta.path.is_ident("proto_name") {
            if let Ok(proto_name_tokens) = meta.value() {
              proto_name = proto_name_tokens
                .parse::<LitStr>()
                .map_err(|e| {
                  Error::new_spanned(
                    attr,
                    format!(
                      "Could not extract proto_name attribute for variant {} in oneof enum {}: {}",
                      variant_ident, enum_name, e
                    ),
                  )
                })?
                .value();
            }
          }
          Ok(())
        }) {
          Ok(_) => {}
          Err(e) => return e.to_compile_error().into(),
        };
      }
    }

    if let syn::Fields::Unnamed(fields) = &variant.fields {
      if fields.unnamed.len() == 1 {
        let variant_type = &fields.unnamed.get(0).unwrap().ty;

        let target_type = if is_box(variant_type) {
          match get_inner_type_from_box(variant_type) {
            Some(t) => t,
            None => {
              return Error::new_spanned(
                variant_ident,
                format!(
                  "Could not parse the boxed type for field {} in struct {}",
                  variant_ident, enum_name
                ),
              )
              .to_compile_error()
              .into()
            }
          }
        } else {
          variant_type
        };

        let into_expression = if is_duration(target_type) {
          quote! {
            let chrono_duration: chrono::Duration = val.to_owned().try_into()?;
            Ok((#proto_name.to_string(), cel_interpreter::Value::Duration(chrono_duration.into())))
          }
        } else if is_timestamp(target_type) {
          quote! {
            let chrono_timestamp: ::chrono::DateTime<::chrono::FixedOffset> = val.to_owned().try_into()?;
            Ok((#proto_name.to_string(), cel_interpreter::Value::Timestamp(chrono_timestamp.into())))
          }
        } else if is_primitive(target_type) || supports_cel_into(target_type) {
          quote! { Ok((#proto_name.to_string(), val.to_owned().into())) }
        } else if is_box(variant_type) {
          quote! { Ok((#proto_name.to_string(), (*val).try_into_cel_value_recursive(depth + 1)?)) }
        } else {
          quote! { Ok((#proto_name.to_string(), val.to_owned().try_into()?)) }
        };

        let arm = quote! {
          #enum_name::#variant_ident(val) => {
            if depth >= Self::MAX_RECURSION_DEPTH {
              Ok((#proto_name.to_string(), cel_interpreter::Value::Null))
            } else {
              #into_expression
            }
          }
        };
        match_arms.push(arm);
      }
    }
  }

  let expanded = quote! {
    impl #enum_name {
      const MAX_RECURSION_DEPTH: usize = 10;

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
    } else if let syn::Type::Path(type_path) = field_type {
      if let Some(segment) = type_path.path.segments.last() {
        let type_ident = &segment.ident;

        match type_ident.to_string().as_str() {
          "Option" => {
            if let syn::PathArguments::AngleBracketed(type_args) = &segment.arguments {
              if let Some(syn::GenericArgument::Type(inner_type)) = type_args.args.first() {
                let target_type = if is_box(inner_type) {
                  match get_inner_type_from_box(inner_type) {
                    Some(t) => t,
                    None => {
                      return Error::new_spanned(
                        field_ident,
                        format!(
                          "Could not parse the boxed type for field {} in struct {}",
                          field_ident, struct_name
                        ),
                      )
                      .to_compile_error()
                      .into()
                    }
                  }
                } else {
                  inner_type
                };

                if is_duration(target_type) {
                  tokens.extend(quote! {
                    if let Some(v) = &value.#field_ident {
                      let cel_val = cel_interpreter::Value::Duration(v.to_owned().try_into()?);
                      #fields_map_ident.insert(#field_name.into(), cel_val);
                    } else {
                      #fields_map_ident.insert(#field_name.into(), cel_interpreter::Value::Null);
                    }
                  });
                } else if is_timestamp(target_type) {
                  tokens.extend(quote! {
                    if let Some(v) = &value.#field_ident {
                      let cel_val = cel_interpreter::Value::Timestamp(v.to_owned().try_into()?);
                      #fields_map_ident.insert(#field_name.into(), cel_val);
                    } else {
                      #fields_map_ident.insert(#field_name.into(), cel_interpreter::Value::Null);
                    }
                  });
                } else if is_primitive(target_type) || supports_cel_into(target_type) {
                  tokens.extend(quote! {
                    if let Some(v) = &value.#field_ident {
                      #fields_map_ident.insert(#field_name.into(), v.into());
                    } else {
                      #fields_map_ident.insert(#field_name.into(), cel_interpreter::Value::Null);
                    }
                  });
                } else if is_box(inner_type) {
                  tokens.extend(quote! {
                    if let Some(v) = value.#field_ident.as_deref() {
                      #fields_map_ident.insert(#field_name.into(), v.try_into_cel_value_recursive(depth + 1)?);
                    } else {
                      #fields_map_ident.insert(#field_name.into(), cel_interpreter::Value::Null);
                    }
                  });
                } else {
                  tokens.extend(quote! {
                    if let Some(v) = &value.#field_ident {
                      #fields_map_ident.insert(#field_name.into(), v.try_into_cel_value_recursive(depth + 1)?);
                    } else {
                      #fields_map_ident.insert(#field_name.into(), cel_interpreter::Value::Null);
                    }
                  });
                }
              }
            }
          }
          "Vec" => {
            if let syn::PathArguments::AngleBracketed(type_args) = &segment.arguments {
              if let Some(syn::GenericArgument::Type(inner_type)) = type_args.args.first() {
                let cel_val = if is_duration(inner_type) {
                  quote! {
                    let chrono_duration: chrono::Duration = item.to_owned().try_into()?;
                    converted.push(cel_interpreter::Value::Duration(chrono_duration.into()));
                  }
                } else if is_timestamp(inner_type) {
                  quote! {
                    let chrono_timestamp: ::chrono::DateTime<::chrono::FixedOffset> = item.to_owned().try_into()?;
                    converted.push(cel_interpreter::Value::Timestamp(chrono_timestamp.into()));
                  }
                } else if is_primitive(inner_type) || supports_cel_into(inner_type) {
                  quote! {
                    converted.push(item.into());
                  }
                } else {
                  quote! {
                    converted.push(item.to_owned().try_into_cel_value_recursive(depth + 1)?);
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
            }
          }
          "HashMap" => {
            if let syn::PathArguments::AngleBracketed(type_args) = &segment.arguments {
              let key_generic = type_args.args.first();
              let value_generic = type_args.args.get(1);

              if let (
                Some(syn::GenericArgument::Type(_)),
                Some(syn::GenericArgument::Type(value_type)),
              ) = (key_generic, value_generic)
              {
                let cel_val = if is_duration(value_type) {
                  quote! {
                    cel_interpreter::Value::Duration(v.to_owned().try_into()?);
                  }
                } else if is_timestamp(value_type) {
                  quote! {
                      cel_interpreter::Value::Timestamp(v.to_owned().try_into()?);
                  }
                } else if is_primitive(value_type) || supports_cel_into(value_type) {
                  quote! {
                    v.into();
                  }
                } else {
                  quote! {
                    v.to_owned().try_into()?;
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
            }
          }
          _ => {
            if is_primitive(field_type) {
              tokens.extend(quote! {
                #fields_map_ident.insert(#field_name.into(), value.#field_ident.into());
              });
            }
          }
        };
      }
    }
  }

  let expanded = quote! {
    impl #struct_name {
      const MAX_RECURSION_DEPTH: usize = 10;

      pub fn try_into_cel_value_recursive(&self, depth: usize) -> Result<::cel_interpreter::Value, protocheck::validators::cel::CelConversionError> {
        if depth >= Self::MAX_RECURSION_DEPTH {
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
  let path: syn::Path = syn::parse_str(target_path).unwrap();
  ty.to_token_stream().to_string() == path.to_token_stream().to_string()
}

fn supports_cel_into(ty: &Type) -> bool {
  type_matches_path(ty, "protocheck::types::FieldMask")
    || type_matches_path(ty, "protocheck::types::Empty")
}

fn is_duration(ty: &Type) -> bool {
  type_matches_path(ty, "protocheck::types::Duration")
}

fn is_timestamp(ty: &Type) -> bool {
  type_matches_path(ty, "protocheck::types::Timestamp")
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

fn is_box(ty: &Type) -> bool {
  if let Type::Path(type_path) = ty {
    if let Some(segment) = type_path.path.segments.last() {
      return segment.ident == "Box";
    }
  }
  false
}

fn get_inner_type_from_box(ty: &Type) -> Option<&Type> {
  if let syn::Type::Path(type_path) = ty {
    if let Some(segment) = type_path.path.segments.last() {
      if segment.ident == "Box" {
        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
          if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
            return Some(inner_type);
          }
        }
      }
    }
  }
  None
}
