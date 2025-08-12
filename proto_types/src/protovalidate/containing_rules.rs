use std::{
  collections::HashSet,
  fmt::{Debug, Display, Write},
  hash::Hash,
};

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{Ident, LitByteStr};

use crate::{
  protovalidate::{
    AnyRules, BytesRules, DurationRules, EnumRules, Fixed32Rules, Fixed64Rules, Int32Rules,
    Int64Rules, SFixed32Rules, SFixed64Rules, SInt32Rules, SInt64Rules, StringRules, UInt32Rules,
    UInt64Rules,
  },
  Duration,
};

#[derive(Debug, Clone)]
pub enum ItemList {
  Slice {
    error_message: String,
    tokens: TokenStream,
  },
  HashSet {
    error_message: String,
    tokens: TokenStream,
    static_ident: Ident,
  },
}

#[derive(Debug, Clone)]
pub struct ContainingRules {
  pub in_list_rule: Option<ItemList>,
  pub not_in_list_rule: Option<ItemList>,
}

fn format_items_list<T>(items: &[T]) -> String
where
  T: Display,
{
  items
    .iter()
    .map(|i| i.to_string())
    .collect::<Vec<String>>()
    .join(", ")
}

fn format_items_list_wrapped_in_quotes<T>(items: &[T]) -> String
where
  T: Display,
{
  items
    .iter()
    .map(|i| format!("'{}'", i))
    .collect::<Vec<String>>()
    .join(", ")
}

pub(crate) fn get_list_kind<T, Hashable>(
  rule_name: &str,
  slice: &[T],
  set: HashSet<Hashable>,
  error_prefix: &str,
  wrap_items_in_quotes: bool,
  type_tokens: TokenStream,
  field_full_name: &str,
) -> Option<ItemList>
where
  T: Debug + ToTokens + Display,
  Hashable: Debug + ToTokens,
{
  if slice.is_empty() && set.is_empty() {
    None
  } else {
    let stringified_list = if wrap_items_in_quotes {
      format_items_list_wrapped_in_quotes(slice)
    } else {
      format_items_list(slice)
    };

    let error_message = format!("{}: [ {} ]", error_prefix, stringified_list);

    if slice.len() >= 16 {
      let static_ident = Ident::new(
        &format!("__{}_{}_LIST", field_full_name, rule_name.to_uppercase()),
        Span::call_site(),
      );

      Some(ItemList::HashSet {
        error_message,
        tokens: hashset_to_tokens(set, type_tokens, &static_ident),
        static_ident,
      })
    } else {
      Some(ItemList::Slice {
        error_message,
        tokens: quote! { [ #(#slice),* ] },
      })
    }
  }
}

macro_rules! standard_containing_rules {
  ($struct_target:ident, $type:ty, $wrap_with_quotes:expr $(, $error_prefix:ident)?) => {
    containing_rules!($struct_target, $type, $type, $wrap_with_quotes $(, $error_prefix)?);
  };
}

macro_rules! containing_rules {
  ($struct_target:ident, $type:ty, $type_tokens:ty, $wrap_with_quotes:expr $(, $error_prefix:ident)?) => {
    macro_rules! _create_error_message {
      (type_url, $constant_part:literal) => {  concat!("the type url ", $constant_part)  };
      (, $constant_part:literal) => { $constant_part };
    }
    impl $struct_target {
      pub fn containing_rules(&self, field_full_name: &str) -> Result<ContainingRules, Vec<$type>> {
        let in_list_slice = &self.r#in;
        let not_in_list_slice = &self.not_in;

        let (in_list_hashset, not_in_list_hashset) = get_validated_lists(&self.r#in, &self.not_in)?;

        let in_list_rule = get_list_kind("in", in_list_slice, in_list_hashset, _create_error_message!($($error_prefix)?, "must be one of these values"), $wrap_with_quotes, quote! { $type_tokens }, field_full_name );
        let not_in_list_rule = get_list_kind("not_in", not_in_list_slice, not_in_list_hashset, _create_error_message!($($error_prefix)?, "cannot be one of these values"), $wrap_with_quotes, quote! { $type_tokens }, field_full_name );

        Ok(ContainingRules {
          in_list_rule,
          not_in_list_rule,
        })
      }
    }
  };
}

containing_rules!(DurationRules, Duration, ::protocheck::types::Duration, true);
containing_rules!(AnyRules, String, &'static str, true, type_url);
standard_containing_rules!(EnumRules, i32, false);
standard_containing_rules!(StringRules, String, true);
standard_containing_rules!(Int64Rules, i64, false);
standard_containing_rules!(Int32Rules, i32, false);
standard_containing_rules!(SInt64Rules, i64, false);
standard_containing_rules!(SInt32Rules, i32, false);
standard_containing_rules!(SFixed64Rules, i64, false);
standard_containing_rules!(SFixed32Rules, i32, false);
standard_containing_rules!(UInt64Rules, u64, false);
standard_containing_rules!(UInt32Rules, u32, false);
standard_containing_rules!(Fixed64Rules, u64, false);
standard_containing_rules!(Fixed32Rules, u32, false);

impl BytesRules {
  pub fn containing_rules(
    &self,
    field_full_name: &str,
  ) -> Result<ContainingRules, Vec<LitByteStr>> {
    let in_list_slice = &self.r#in;
    let not_in_list_slice = &self.not_in;

    let in_list_hashset: HashSet<LitByteStr> = in_list_slice
      .iter()
      .map(|b| LitByteStr::new(b, Span::call_site()))
      .collect();

    let not_in_list_hashset: HashSet<LitByteStr> = not_in_list_slice
      .iter()
      .map(|b| LitByteStr::new(b, Span::call_site()))
      .collect();

    let invalid_items: Vec<LitByteStr> = in_list_hashset
      .intersection(&not_in_list_hashset)
      .cloned()
      .collect();

    if !invalid_items.is_empty() {
      return Err(invalid_items);
    }

    let in_list = (!in_list_hashset.is_empty()).then(|| {
      let stringified_list = self
        .r#in
        .iter()
        .map(|b| format_bytes(b))
        .collect::<Vec<String>>()
        .join(", ");

      let error_message = format!("must be one of these values: [ {} ]", stringified_list);

      if in_list_slice.len() >= 16 {
        let static_ident = format_ident!("__{}_IN_LIST", field_full_name);
        ItemList::HashSet {
          error_message,
          tokens: byte_lit_hashset_to_tokens(in_list_hashset, &static_ident),
          static_ident,
        }
      } else {
        let lit_byte_vec: Vec<&LitByteStr> = in_list_hashset.iter().collect();
        ItemList::Slice {
          error_message,
          tokens: quote! { [ #(#lit_byte_vec),* ] },
        }
      }
    });

    let not_in_list = (!not_in_list_hashset.is_empty()).then(|| {
      let stringified_list = self
        .not_in
        .iter()
        .map(|b| format_bytes(b))
        .collect::<Vec<String>>()
        .join(", ");

      let error_message = format!("cannot be one of these values: [ {} ]", stringified_list);

      if not_in_list_slice.len() >= 16 {
        let static_ident = format_ident!("__{}_NOT_IN_LIST", field_full_name);

        ItemList::HashSet {
          error_message,
          tokens: byte_lit_hashset_to_tokens(not_in_list_hashset, &static_ident),
          static_ident,
        }
      } else {
        let lit_byte_vec: Vec<&LitByteStr> = not_in_list_hashset.iter().collect();
        ItemList::Slice {
          error_message,
          tokens: quote! { [ #(#lit_byte_vec),* ] },
        }
      }
    });

    Ok(ContainingRules {
      in_list_rule: in_list,
      not_in_list_rule: not_in_list,
    })
  }
}

pub(crate) fn format_bytes(bytes: &[u8]) -> String {
  let mut s = String::with_capacity(bytes.len() * 2);
  s.push('\'');

  for &byte in bytes.iter() {
    match byte {
      b'\n' => s.push_str("\\n"),
      b'\r' => s.push_str("\\r"),
      b'\t' => s.push_str("\\t"),
      b'\\' => s.push_str("\\\\"),
      b'"' => s.push_str("\\\""),

      32..=126 => s.push(byte as char),

      _ => {
        write!(s, "\\x{:02x}", byte).unwrap();
      }
    }
  }

  s.push('\'');
  s
}

pub(crate) fn get_validated_lists<T>(
  in_list: &[T],
  not_in_list: &[T],
) -> Result<(HashSet<T>, HashSet<T>), Vec<T>>
where
  T: Clone + Hash + Eq + Debug + ToTokens + Display,
{
  let in_list_hashset: HashSet<T> = in_list.iter().cloned().collect();
  let not_in_list_hashset: HashSet<T> = not_in_list.iter().cloned().collect();

  let invalid_items: Vec<T> = in_list_hashset
    .intersection(&not_in_list_hashset)
    .cloned()
    .collect();

  if !invalid_items.is_empty() {
    return Err(invalid_items);
  }

  Ok((in_list_hashset, not_in_list_hashset))
}

fn wrap_hashset_tokens(
  set_ident: Ident,
  hashset_tokens: TokenStream,
  type_tokens: TokenStream,
  static_ident: &Ident,
) -> TokenStream {
  quote! {
    static #static_ident: ::std::sync::LazyLock<std::collections::HashSet<#type_tokens>> = ::std::sync::LazyLock::new(||{
      let mut #set_ident: ::std::collections::HashSet<#type_tokens> = ::std::collections::HashSet::new();
      #hashset_tokens
      #set_ident
    });
  }
}

pub(crate) fn hashset_to_tokens<T>(
  hashset: HashSet<T>,
  type_tokens: TokenStream,
  static_ident: &Ident,
) -> TokenStream
where
  T: ToTokens,
{
  let set_ident = Ident::new("set", Span::call_site());

  let mut hashset_tokens = TokenStream::new();

  for item in hashset {
    hashset_tokens.extend(quote! {
      #set_ident.insert(#item);
    });
  }

  wrap_hashset_tokens(set_ident, hashset_tokens, type_tokens, static_ident)
}

pub(crate) fn byte_lit_hashset_to_tokens(
  hashset: HashSet<LitByteStr>,
  static_ident: &Ident,
) -> TokenStream {
  let set_ident = Ident::new("set", Span::call_site());
  let mut hashset_tokens = TokenStream::new();

  for item in hashset {
    hashset_tokens.extend(quote! {
      #set_ident.insert(::bytes::Bytes::from_static(#item));
    });
  }

  wrap_hashset_tokens(
    set_ident,
    hashset_tokens,
    quote! { ::bytes::Bytes },
    static_ident,
  )
}
