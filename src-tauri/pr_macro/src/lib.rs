use std::collections::HashSet;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::LitInt;
use syn::Token;
use syn::{parse_macro_input, Ident, Type};

struct NumbersAttribute {
  numbers: Punctuated<LitInt, Token![,]>,
}

impl Parse for NumbersAttribute {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    Ok(NumbersAttribute {
      numbers: Punctuated::parse_terminated(input)?,
    })
  }
}

struct Range {
  pub start: LitInt,
  _to_token: Ident,
  pub end: LitInt,
}

impl Parse for Range {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let start: LitInt = input.parse()?;
    let to_token: Ident = input.parse()?;
    if to_token != "to" {
      return Err(syn::Error::new_spanned(to_token, "expected `to` keyword"));
    }

    let end: LitInt = input.parse()?;

    Ok(Range {
      start,
      _to_token: to_token,
      end,
    })
  }
}

struct RangesAttribute {
  pub ranges: Punctuated<Range, Token![,]>,
}

impl Parse for RangesAttribute {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    Ok(RangesAttribute {
      ranges: Punctuated::parse_terminated(input)?,
    })
  }
}

#[proc_macro_derive(ProtoMessage, attributes(field_nr, reserved_nrs, reserved_ranges))]
pub fn proto_message_macro_derive(input: TokenStream) -> TokenStream {
  // 1. Parse the input `item` as a DeriveInput.
  // DeriveInput is a special syn struct designed for parsing `#[derive]` input.

  let input_ast = parse_macro_input!(input as syn::DeriveInput);

  // Extract the name (identifier) of the struct or enum
  let name = &input_ast.ident;

  let mut field_info = Vec::new();
  let mut field_nr = 1;

  let mut reserved_nums: HashSet<i32> = HashSet::new();

  if let syn::Data::Struct(data_struct) = &input_ast.data {
    for attr in input_ast.attrs {
      if attr.path().is_ident("reserved_nrs") {
        match attr.parse_args_with(NumbersAttribute::parse) {
          Ok(parsed_numbers) => {
            for int_lit in parsed_numbers.numbers {
              if let Ok(num) = int_lit.base10_parse::<i32>() {
                reserved_nums.insert(num);
              } else {
                return syn::Error::new_spanned(int_lit, "Expected an integer literal")
                  .to_compile_error()
                  .into();
              }
            }
          }
          Err(e) => {
            return e.to_compile_error().into();
          }
        }
      }
      if attr.path().is_ident("reserved_ranges") {
        match attr.parse_args_with(RangesAttribute::parse) {
          Ok(parsed_attr) => {
            for range in parsed_attr.ranges {
              let start_val = match range.start.base10_parse::<i32>() {
                Ok(val) => val,
                Err(e) => {
                  return syn::Error::new_spanned(
                    &range.start,
                    format!("Invalid start number: {}", e),
                  )
                  .to_compile_error()
                  .into()
                }
              };
              let end_val = match range.end.base10_parse::<i32>() {
                Ok(val) => val,
                Err(e) => {
                  return syn::Error::new_spanned(&range.end, format!("Invalid end number: {}", e))
                    .to_compile_error()
                    .into()
                }
              };

              for n in start_val..=end_val {
                reserved_nums.insert(n);
              }
            }
          }
          Err(e) => return e.to_compile_error().into(),
        }
      }
    }
    if let syn::Fields::Named(fields_named) = &data_struct.fields {
      for field in &fields_named.named {
        let mut field_num = field_nr;
        let mut increase_nr = true;
        let field_name_ident = field
          .ident
          .as_ref()
          .expect("Named fields must have an identifier");

        let field_type: &Type = &field.ty;

        let field_type_str = field_type.to_token_stream().to_string();

        for attr in &field.attrs {
          if attr.path().is_ident("field_nr") {
            match attr.parse_args::<LitInt>() {
              Ok(lit_int) => match lit_int.base10_parse::<i32>() {
                Ok(num) => {
                  if num != field_num {
                    field_num = num;
                    reserved_nums.insert(num);
                    increase_nr = false;
                  }

                  break;
                }
                Err(e) => {
                  return syn::Error::new_spanned(
                    attr,
                    format!(
                      "Invalid 'field_nr' value: expected an integer, got parsing error: {}",
                      e
                    ),
                  )
                  .to_compile_error()
                  .into();
                }
              },
              Err(e) => {
                return syn::Error::new_spanned(
                                    attr,
                                    format!("Invalid 'field_nr' attribute: expected `#[field_nr = NUMBER]`, got parsing error: {}", e)
                                ).to_compile_error().into();
              }
            };
          };
        }

        while reserved_nums.contains(&field_num) {
          field_num = field_num + 1;
          field_nr = field_nr + 1;
        }

        field_info.push(quote! {
          (#field_num, stringify!(#field_name_ident).to_string(), #field_type_str.to_string())
        });

        if increase_nr {
          field_nr = field_nr + 1;
        }
      }
    };
  };

  // We'll also need to handle generics so the impl works for generic structs.
  // syn::Generics provides fields for type parameters, lifetimes, and const parameters.
  let (impl_generics, ty_generics, where_clause) = input_ast.generics.split_for_impl();

  // 2. Generate the output `TokenStream`.
  // This will be an `impl` block for the `Hello` trait for the struct.
  let output = quote! {
      // Implement the `Hello` trait for the given struct `name`.
      // We use `impl_generics` and `ty_generics` to ensure the impl applies correctly
      // if the original struct is generic (e.g., `struct MyGeneric<T>`).
    impl #impl_generics macro_impl::ProtoMessage for #name #ty_generics #where_clause {
      fn get_fields(&self) -> macro_impl::MessageData {
        macro_impl::MessageData {
          fields: vec![#(#field_info),*],
        }
      }
    }
  };

  // 3. Convert and return the generated TokenStream.
  output.into()
}
