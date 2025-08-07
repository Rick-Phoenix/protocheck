use std::{collections::HashSet, fmt::Debug, hash::Hash};

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Error, LitByteStr};

use crate::{
  protovalidate::{AnyRules, BytesRules, DurationRules, EnumRules, StringRules},
  Duration,
};

pub struct ContainingRules<T>
where
  T: PartialOrd + PartialEq + Debug + ToTokens,
{
  pub in_list: Vec<T>,
  pub not_in_list: Vec<T>,
}

impl DurationRules {
  pub fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<Duration>, Error> {
    let in_list = self.r#in.clone();
    let not_in_list = self.not_in.clone();

    validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl AnyRules {
  pub fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<&str>, Error> {
    let in_list: Vec<&str> = self.r#in.iter().map(|s| s.as_str()).collect();
    let not_in_list: Vec<&str> = self.not_in.iter().map(|s| s.as_str()).collect();

    validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl EnumRules {
  pub fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<i32>, Error> {
    let in_list = self.r#in.clone();
    let not_in_list = self.not_in.clone();

    validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl StringRules {
  pub fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<&str>, Error> {
    let in_list: Vec<&str> = self.r#in.iter().map(|s| s.as_str()).collect();
    let not_in_list: Vec<&str> = self.not_in.iter().map(|s| s.as_str()).collect();

    validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl BytesRules {
  pub fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(Option<TokenStream>, Option<TokenStream>), Error> {
    validate_in_not_in(&self.r#in, &self.not_in, field_span, error_prefix)?;

    let in_list_lit_byte_str = self
      .r#in
      .iter()
      .map(|b| LitByteStr::new(b, Span::call_site()));

    let not_in_list_lit_byte_str = self
      .not_in
      .iter()
      .map(|b| LitByteStr::new(b, Span::call_site()));

    let in_list_tokens = quote! { vec![ #(#in_list_lit_byte_str),* ] };
    let not_in_list_tokens = quote! { vec![ #(#not_in_list_lit_byte_str),* ] };

    Ok((Some(in_list_tokens), Some(not_in_list_tokens)))
  }
}

pub(crate) fn validate_in_not_in<T>(
  in_list: &[T],
  not_in_list: &[T],
  field_span: Span,
  error_prefix: &str,
) -> Result<(), Error>
where
  T: Eq + Hash + Debug,
{
  if in_list.is_empty() || not_in_list.is_empty() {
    return Ok(());
  }

  let (shorter_list, longer_list) = if in_list.len() > not_in_list.len() {
    (in_list, not_in_list)
  } else {
    (not_in_list, in_list)
  };

  let values_set: HashSet<&T> = shorter_list.iter().collect();
  let mut invalid_items: Vec<&T> = Vec::new();

  for item in longer_list {
    if values_set.contains(item) {
      invalid_items.push(item);
    }
  }

  if !invalid_items.is_empty() {
    return Err(Error::new(
      field_span,
      format!(
        "{} the following values are contained by 'in' and 'not_in': {:?}",
        error_prefix, invalid_items
      ),
    ));
  }

  Ok(())
}

pub(crate) trait FloatBits {
  type Bits: Eq + std::hash::Hash;

  fn to_bits_for_unique_check(&self) -> Self::Bits;
}

impl FloatBits for f32 {
  type Bits = u32;
  fn to_bits_for_unique_check(&self) -> u32 {
    self.to_bits()
  }
}

impl FloatBits for f64 {
  type Bits = u64;
  fn to_bits_for_unique_check(&self) -> u64 {
    self.to_bits()
  }
}

impl FloatBits for &f32 {
  type Bits = u32;
  fn to_bits_for_unique_check(&self) -> u32 {
    self.to_bits()
  }
}

impl FloatBits for &f64 {
  type Bits = u64;
  fn to_bits_for_unique_check(&self) -> u64 {
    self.to_bits()
  }
}

pub(crate) fn validate_in_not_in_floats<T, B>(
  in_list: &[T],
  not_in_list: &[T],
  field_span: Span,
  error_prefix: &str,
) -> Result<(), Error>
where
  B: Eq + Hash,
  T: FloatBits<Bits = B> + Debug,
{
  if in_list.is_empty() || not_in_list.is_empty() {
    return Ok(());
  }

  let (shorter_list, longer_list) = if in_list.len() > not_in_list.len() {
    (in_list, not_in_list)
  } else {
    (not_in_list, in_list)
  };

  let mut values_set: HashSet<B> = HashSet::new();

  for item in shorter_list {
    values_set.insert(item.to_bits_for_unique_check());
  }

  let mut invalid_items: Vec<&T> = Vec::new();

  for item in longer_list {
    let comparable = item.to_bits_for_unique_check();
    if values_set.contains(&comparable) {
      invalid_items.push(item);
    }
  }

  if !invalid_items.is_empty() {
    return Err(Error::new(
      field_span,
      format!(
        "{} the following values are contained by 'in' and 'not_in': {:?}",
        error_prefix, invalid_items
      ),
    ));
  }

  Ok(())
}
