use std::collections::HashSet;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parenthesized;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::LitInt;
use syn::LitStr;
use syn::Token;
use syn::{parse_macro_input, Type};

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

struct StringsAttribute {
  strings: Punctuated<LitStr, Token![,]>,
}

impl Parse for StringsAttribute {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    Ok(StringsAttribute {
      strings: Punctuated::parse_terminated(input)?,
    })
  }
}

mod kw {
  syn::custom_keyword!(to);
}

struct Range {
  pub start: LitInt,
  _to_token: kw::to,
  pub end: LitInt,
}

impl Parse for Range {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let start: LitInt = input.parse()?;
    let to_token: kw::to = input.parse()?;

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

#[proc_macro_derive(
  ProtoMessage,
  attributes(field_nr, reserved_nrs, reserved_ranges, reserved_names, protoschema)
)]
pub fn proto_message_macro_derive(input: TokenStream) -> TokenStream {
  // DeriveInput is a special syn struct designed for parsing `#[derive]` input.

  let input_ast = parse_macro_input!(input as syn::DeriveInput);

  let name = &input_ast.ident;

  let mut field_info = Vec::new();
  let mut field_nr = 1;

  let mut reserved_nums: HashSet<i32> = HashSet::new();
  let mut reserved_names: Vec<String> = Vec::new();

  if let syn::Data::Struct(data_struct) = &input_ast.data {
    for attr in input_ast.attrs {
      if attr.path().is_ident("reserved_names") {
        match attr.parse_args_with(StringsAttribute::parse) {
          Ok(parsed_strings) => {
            for str in parsed_strings.strings {
              reserved_names.push(str.value());
            }
          }
          Err(e) => return e.to_compile_error().into(),
        }
      }
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
      if attr.path().is_ident("protoschema") {
        match attr.parse_nested_meta(|meta| {
          if meta.path.is_ident("reserved_ranges") {
            let content;

            parenthesized!(content in meta.input);
            let parsed_attr = content.parse::<RangesAttribute>()?;
            for range in parsed_attr.ranges {
              let start_val = range.start.base10_parse::<i32>()?;
              let end_val = range.end.base10_parse::<i32>()?;

              for n in start_val..=end_val {
                reserved_nums.insert(n);
              }
            }
          } else {
            return Err(meta.error(format!(
              "unrecognized `protoschema` argument `{}`",
              meta.path.to_token_stream()
            )));
          };

          Ok(())
        }) {
          Err(e) => return e.to_compile_error().into(),
          _ => {}
        };
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

  let (impl_generics, ty_generics, where_clause) = input_ast.generics.split_for_impl();

  let output = quote! {
    impl #impl_generics macro_impl::ProtoMessage for #name #ty_generics #where_clause {
      fn get_fields(&self) -> macro_impl::MessageData {
        macro_impl::MessageData {
          fields: vec![#(#field_info),*],
        }
      }
    }
  };

  output.into()
}
