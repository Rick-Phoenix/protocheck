use bytes::Bytes;
use prost_reflect::DescriptorPool;
use std::collections::HashMap;
use std::collections::HashSet;
use syn::braced;
use syn::bracketed;
use syn::DeriveInput;
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
use syn::Type;
use syn::TypePath;

use crate::validator::extract_validators;

mod validator;

#[macro_use]
extern crate lazy_static;

lazy_static! {
  static ref RUST_TO_PROTO_MAP: HashMap<String, String> = {
    let mut type_map = HashMap::new();

    type_map.insert("String".to_string(), "string".to_string());
    type_map.insert("bool".to_string(), "bool".to_string());
    type_map.insert("f32".to_string(), "float".to_string());
    type_map.insert("f64".to_string(), "double".to_string());

    type_map.insert("i8".to_string(), "int32".to_string());
    type_map.insert("u8".to_string(), "uint32".to_string());
    type_map.insert("i16".to_string(), "int32".to_string());
    type_map.insert("u16".to_string(), "uint32".to_string());
    type_map.insert("i32".to_string(), "int32".to_string());
    type_map.insert("u32".to_string(), "uint32".to_string());
    type_map.insert("i64".to_string(), "int64".to_string());
    type_map.insert("u64".to_string(), "uint64".to_string());
    type_map.insert("isize".to_string(), "int64".to_string());
    type_map.insert("usize".to_string(), "uint64".to_string());

    type_map
  };
}

#[proc_macro_attribute]
pub fn protobuf_validate(args: TokenStream, input: TokenStream) -> TokenStream {
  let _ = args;

  let input_clone = input.clone();
  let _ast = parse_macro_input!(input_clone as DeriveInput);

  let struct_name = _ast.ident.to_string();
  println!("{}", struct_name.to_string());

  let original_input_as_proc_macro2: proc_macro2::TokenStream = input.into();

  extract_validators(_ast);

  quote! {
      #original_input_as_proc_macro2

    impl macro_impl::WithValidator for User {
      fn validate(&self) -> bool {
        let program = cel_interpreter::Program::compile("this.name == 'Me'").unwrap();
        let mut context = cel_interpreter::Context::default();

        context.add_variable("this", self).unwrap();

        let value = program.execute(&context).unwrap();

        match value {
          cel_interpreter::Value::Bool(val) => val,
          _ => {
            panic!("Expected a boolean")
          }
        }
      }
    }
  }
  .into()
}

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
          let content;

          bracketed!(content in input);
          let parsed_nums = content.parse::<NumbersAttribute>()?;
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

#[derive(Default)]
struct ProtoTypeInfo {
  pub proto_type: String,
  pub is_repeated: bool,
  pub is_optional: bool,
}

fn get_proto_type_info(ty: &Type) -> ProtoTypeInfo {
  let mut info = ProtoTypeInfo::default();
  let mut base_type = String::default();
  let mut first_arg = String::default();
  let mut second_arg = String::default();
  let mut i = 0;

  // 1. Ensure the type is a TypePath (e.g., `std::collections::Vec`, `Option`, `i32`)
  if let Type::Path(TypePath { path, .. }) = ty {
    // 2. Get the last segment of the path (e.g., `Vec` from `std::collections::Vec`)
    if let Some(segment) = path.segments.last() {
      base_type = segment.ident.to_string();

      // 3. Check for angle bracketed arguments (generics like `<T>` or `<K, V>`)
      if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
        for arg in &args.args {
          if let syn::GenericArgument::Type(ty_arg) = arg {
            let segment = ty_arg.to_token_stream().to_string();
            match i {
              0 => first_arg = segment,
              1 => second_arg = segment,
              _ => {}
            }

            if i >= 1 {
              break;
            }
          }

          i = i + 1
        }
      }
    }
  }

  let inner_type = get_proto_type(&first_arg).unwrap_or(first_arg);

  let base_t: &str = &base_type;
  match base_t {
    "Option" => {
      info.is_optional = true;
      info.proto_type = inner_type;
    }
    "Vec" => {
      info.is_repeated = true;
      if inner_type == "u8" {
        info.proto_type = "bytes".to_string();
      } else {
        info.proto_type = inner_type;
      }
    }
    "HashMap" => {
      let values_type = get_proto_type(&second_arg).unwrap_or(second_arg);
      info.proto_type = format!("map<{}, {}>", inner_type, values_type);
    }
    other => match RUST_TO_PROTO_MAP.get(other) {
      Some(ty) => {
        info.proto_type = ty.to_string();
      }
      None => {
        info.proto_type = other.to_string();
      }
    },
  };

  info
}

fn get_proto_type(rust_type: &str) -> Option<String> {
  match RUST_TO_PROTO_MAP.get(rust_type) {
    Some(ty) => Some(ty.clone()),
    None => None,
  }
}

#[proc_macro_derive(
  ProtoMessage,
  attributes(field_num, reserved_nums, reserved_ranges, reserved_names, protoschema)
)]
pub fn proto_message_macro_derive(input: TokenStream) -> TokenStream {
  let input_ast = parse_macro_input!(input as syn::DeriveInput);

  let struct_name = &input_ast.ident;

  let mut fields_info = Vec::new();
  let mut fields_index = 1;

  let mut config = MacroConfig::default();

  if let syn::Data::Struct(data_struct) = &input_ast.data {
    for attr in input_ast.attrs {
      if attr.path().is_ident("protoschema") {
        match attr.parse_nested_meta(|meta| {
          if meta.input.peek(Token![=]) {
            let content;
            let value_tokens = meta.value()?;
            if meta.path.is_ident("config") {
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
        let mut field_num = fields_index;
        let mut options: Option<proc_macro2::TokenStream> = None;
        let mut is_index = true;
        let mut skip_field = false;
        let field_name = field
          .ident
          .as_ref()
          .expect("Named fields must have an identifier")
          .to_string();

        let field_type = &field.ty;

        let field_type_full = field_type.to_token_stream().to_string();

        let ProtoTypeInfo {
          mut proto_type,
          is_repeated,
          is_optional,
        } = get_proto_type_info(field_type);

        for attr in &field.attrs {
          if attr.path().is_ident("protoschema") {
            match attr.parse_nested_meta(|meta| {
              if meta.path.is_ident("ignore") {
                skip_field = true;
              } else if meta.input.peek(Token![=]) {
                let value_tokens = meta.value()?;
                if meta.path.is_ident("field_num") {
                  let parsed_int = value_tokens.parse::<LitInt>()?;
                  let num = parsed_int.base10_parse::<i32>()?;
                  if num != field_num {
                    field_num = num;
                    is_index = false;
                  }
                } else if meta.path.is_ident("proto_type") {
                  let parsed_proto_type: syn::Path = value_tokens.parse()?;
                  proto_type = parsed_proto_type.to_token_stream().to_string();
                } else if meta.path.is_ident("options") {
                  let content;
                  braced!(content in value_tokens);
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

        if is_index {
          while config.values.reserved_nums_set.contains(&field_num) {
            field_num = field_num + 1;
            fields_index = fields_index + 1;
          }
        }

        config.values.reserved_nums_set.insert(field_num);

        let parsed_options = match &options {
          Some(tokens) => {
            let tokens_string = tokens.to_string();
            quote! { Some(format!("[{}]", #tokens_string)) }
          }
          None => {
            quote! { None }
          }
        };

        fields_info.push(quote! (
          (
            #field_name.to_string(),

            macro_impl::ProtoField {
              field_num: #field_num,
              name: #field_name.to_string(),
              rust_type: #field_type_full.to_string(),
              proto_type: #proto_type.to_string(),
              options: #parsed_options,
              repeated: #is_repeated,
              optional: #is_optional,
            },
          )
        ));

        if is_index {
          fields_index = fields_index + 1;
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
        vec![ #(#fields_info),* ]
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

      fn get_name(&self) -> &str {
        #message_name
      }
    }
  };

  // eprintln!("{}", output.to_string());

  output.into()
}
