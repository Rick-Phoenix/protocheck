use std::collections::HashSet;
use syn::braced;
use syn::bracketed;
use syn::Ident;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::LitInt;
use syn::Token;

#[derive(Default, Debug)]
struct ConfigValues {
  pub message_name: String,
  pub reserved_nums: Vec<i32>,
  reserved_nums_set: HashSet<i32>,
  pub reserved_ranges: Vec<(i32, i32)>,
  pub reserved_names: Vec<String>,
}

#[derive(Default, Debug)]
struct MacroConfig {
  pub values: ConfigValues,
}

impl Parse for MacroConfig {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut values = ConfigValues::default();

    while !input.is_empty() {
      let key_ident: Ident = input.parse()?;

      input.parse::<Token![=]>()?;

      let key_string = key_ident.to_string();
      match key_string.as_str() {
        "message_name" => {
          let parsed_name: syn::Path = input.parse()?;
          values.message_name = parsed_name.to_token_stream().to_string();
        }
        "reserved_nums" => {
          let parsed_nums = input.parse::<NumbersAttribute>()?;
          for int_lit in parsed_nums.numbers {
            if let Ok(num) = int_lit.base10_parse::<i32>() {
              values.reserved_nums.push(num);
              values.reserved_nums_set.insert(num);
            } else {
              return Err(syn::Error::new_spanned(
                int_lit,
                "Expected an integer literal",
              ));
            }
          }
        }
        "reserved_ranges" => {
          let content;

          bracketed!(content in input);

          let parsed_attr = content.parse::<RangesAttribute>()?;
          for range in parsed_attr.ranges {
            let start_val = range.start.base10_parse::<i32>()?;
            let end_val = range.end.base10_parse::<i32>()?;

            for n in start_val..=end_val {
              values.reserved_nums_set.insert(n);
            }

            values.reserved_ranges.push((start_val, end_val));
          }
        }
        "reserved_names" => {
          let content;

          bracketed!(content in input);
          let parsed_strings = content.parse::<StringsAttribute>()?;
          for str in parsed_strings.strings {
            values.reserved_names.push(str.to_string());
          }
        }

        _ => {
          return Err(syn::Error::new_spanned(
            key_ident,
            format!("unrecognized configuration key {}", key_string),
          ));
        }
      }

      if !input.is_empty() {
        input.parse::<Token![,]>()?;
      }
    }

    if !input.is_empty() {
      input.parse::<Token![,]>()?;
    }

    if !input.is_empty() {
      return Err(input.error("expected trailing comma or end of input"));
    }

    Ok(MacroConfig { values })
  }
}

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
  strings: Punctuated<Ident, Token![,]>,
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
  attributes(field_num, reserved_nums, reserved_ranges, reserved_names, protoschema)
)]
pub fn proto_message_macro_derive(input: TokenStream) -> TokenStream {
  // DeriveInput is a special syn struct designed for parsing `#[derive]` input.

  let input_ast = parse_macro_input!(input as syn::DeriveInput);

  let struct_name = &input_ast.ident;

  let mut field_info = Vec::new();
  let mut field_index = 1;

  let mut config = MacroConfig::default();

  if let syn::Data::Struct(data_struct) = &input_ast.data {
    for attr in input_ast.attrs {
      if attr.path().is_ident("protoschema") {
        match attr.parse_nested_meta(|meta| {
          if meta.input.peek(syn::token::Paren) {
            println!("Got it")
          }
          if meta.path.is_ident("config") {
            if meta.input.peek(Token![=]) {
              let content;
              let value_tokens = meta.value()?;
              braced!(content in value_tokens);
              let parsed_config: MacroConfig = syn::parse2(content.parse()?)?;
              config = parsed_config;
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
        let mut field_num = field_index;
        let mut options: Option<proc_macro2::TokenStream> = None;
        let mut increase_nr = true;
        let mut skip_field = false;
        let field_name_ident = field
          .ident
          .as_ref()
          .expect("Named fields must have an identifier");

        let field_type = field.ty.to_token_stream().to_string();
        let mut proto_type = field_type.clone();

        for attr in &field.attrs {
          if attr.path().is_ident("field_num") {
            match attr.parse_args::<LitInt>() {
              Ok(lit_int) => match lit_int.base10_parse::<i32>() {
                Ok(num) => {
                  if num != field_num {
                    field_num = num;
                    config.values.reserved_nums_set.insert(num);
                    increase_nr = false;
                  }

                  break;
                }
                Err(e) => {
                  return syn::Error::new_spanned(
                    attr,
                    format!(
                      "Invalid 'field_num' value: expected an integer, got parsing error: {}",
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
                  format!("Invalid 'field_num' attribute: {}", e),
                )
                .to_compile_error()
                .into();
              }
            };
          } else if attr.path().is_ident("protoschema") {
            match attr.parse_nested_meta(|meta| {
              if meta.path.is_ident("ignore") {
                skip_field = true;
              } else if meta.path.is_ident("proto_type") {
                if meta.input.peek(Token![=]) {
                  let value_stream = meta.value()?;
                  let parsed_proto_type: syn::Path = value_stream.parse()?;
                  proto_type = parsed_proto_type.to_token_stream().to_string();
                }
              } else if meta.path.is_ident("options") {
                if meta.input.peek(Token![=]) {
                  let value = meta.value()?;
                  let content;
                  braced!(content in value);
                  options = Some(content.parse::<proc_macro2::TokenStream>()?);
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
          };
        }

        if skip_field {
          continue;
        }

        while config.values.reserved_nums_set.contains(&field_num) {
          field_num = field_num + 1;
          field_index = field_index + 1;
        }

        let parsed_options = match &options {
          Some(tokens) => {
            let tokens_string = tokens.to_string();
            quote! { Some(format!("[{}]", #tokens_string)) }
          }
          None => {
            quote! { None }
          }
        };

        field_info.push(quote! (
          (
            stringify!(#field_name_ident).to_string(),

            macro_impl::ProtoField {
              field_num: #field_num,
              name: stringify!(#field_name_ident).to_string(),
              rust_type: #field_type.to_string(),
              proto_type: #proto_type.to_string(),
              options: #parsed_options,
            },
          )
        ));

        if increase_nr {
          field_index = field_index + 1;
        }
      }
    };
  };

  let (impl_generics, ty_generics, where_clause) = input_ast.generics.split_for_impl();

  let ConfigValues {
    mut message_name,
    reserved_nums,
    reserved_ranges,
    reserved_names,
    ..
  } = config.values;

  if message_name == "" {
    message_name = struct_name.clone().to_string();
  }

  let ranges_tokens: Vec<TokenStream2> = reserved_ranges
    .into_iter()
    .map(|(a, b)| {
      quote! { (#a, #b) }
    })
    .collect();

  let output = quote! {
    impl #impl_generics macro_impl::ProtoMessage for #struct_name #ty_generics #where_clause {
      fn fields(&self) -> std::collections::HashMap<String, macro_impl::ProtoField> {
        vec![ #(#field_info),* ]
          .into_iter()
          .collect::<std::collections::HashMap<String, macro_impl::ProtoField>>()
      }

      fn data(&self) -> macro_impl::MessageData {
        macro_impl::MessageData {
          name: #message_name.to_string(),
          fields: self.fields(),
          reserved_nums: vec![ #(#reserved_nums),* ],
          reserved_ranges: vec![ #(#ranges_tokens),* ],
          reserved_names: vec![ #(#reserved_names.to_string()),* ],
        }
      }
    }
  };

  eprintln!("{}", output.to_string());

  output.into()
}
