use std::{collections::HashSet, fmt::Debug, hash::Hash};

use proc_macro2::Span;
use quote::ToTokens;
use syn::{Error, LitByteStr};

use crate::{
  protovalidate::{AnyRules, BytesRules, DurationRules, EnumRules, StringRules},
  Duration,
};

pub struct ContainingRules<T>
where
  T: PartialOrd + Debug + ToTokens + Eq + Hash,
{
  pub in_list: HashSet<T>,
  pub not_in_list: HashSet<T>,
}

impl DurationRules {
  pub fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<Duration>, Error> {
    let in_list = self.r#in.clone();
    let not_in_list = self.not_in.clone();

    let (in_hashset, not_in_hashset) =
      validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list: in_hashset,
      not_in_list: not_in_hashset,
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

    let (in_hashset, not_in_hashset) =
      validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list: in_hashset,
      not_in_list: not_in_hashset,
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

    let (in_hashset, not_in_hashset) =
      validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list: in_hashset,
      not_in_list: not_in_hashset,
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

    let (in_hashset, not_in_hashset) =
      validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list: in_hashset,
      not_in_list: not_in_hashset,
    })
  }
}

impl BytesRules {
  pub fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(HashSet<LitByteStr>, HashSet<LitByteStr>), Error> {
    let (in_list, not_in_list) =
      validate_in_not_in(&self.r#in, &self.not_in, field_span, error_prefix)?;

    let in_list_lit_byte_str: HashSet<LitByteStr> = in_list
      .iter()
      .map(|b| LitByteStr::new(b, Span::call_site()))
      .collect();

    let not_in_list_lit_byte_str: HashSet<LitByteStr> = not_in_list
      .iter()
      .map(|b| LitByteStr::new(b, Span::call_site()))
      .collect();

    Ok((in_list_lit_byte_str, not_in_list_lit_byte_str))
  }
}

pub(crate) fn validate_in_not_in<T>(
  in_list: &[T],
  not_in_list: &[T],
  field_span: Span,
  error_prefix: &str,
) -> Result<(HashSet<T>, HashSet<T>), Error>
where
  T: Eq + Hash + Debug + Clone,
{
  let mut in_list_hashset: HashSet<T> = HashSet::new();
  let mut not_in_list_hashset: HashSet<T> = HashSet::new();

  if in_list.is_empty() || not_in_list.is_empty() {
    return Ok((in_list_hashset, not_in_list_hashset));
  }

  let (shorter_list, longer_list, shorter_list_hashset, longer_list_hashset) =
    if in_list.len() < not_in_list.len() {
      (
        in_list,
        not_in_list,
        &mut in_list_hashset,
        &mut not_in_list_hashset,
      )
    } else {
      (
        not_in_list,
        in_list,
        &mut not_in_list_hashset,
        &mut in_list_hashset,
      )
    };

  for item in shorter_list {
    shorter_list_hashset.insert(item.clone());
  }

  let mut invalid_items: Vec<T> = Vec::new();

  for item in longer_list {
    let comparable = item.clone();
    if shorter_list_hashset.contains(&comparable) {
      invalid_items.push(item.clone());
    }
    longer_list_hashset.insert(comparable);
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

  Ok((in_list_hashset, not_in_list_hashset))
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
) -> Result<(HashSet<B>, HashSet<B>), Error>
where
  B: Eq + Hash + Copy,
  T: FloatBits<Bits = B> + Debug + Copy,
{
  let mut in_list_hashset: HashSet<B> = HashSet::new();
  let mut not_in_list_hashset: HashSet<B> = HashSet::new();

  if in_list.is_empty() || not_in_list.is_empty() {
    return Ok((in_list_hashset, not_in_list_hashset));
  }

  let (shorter_list, longer_list, shorter_list_hashset, longer_list_hashset) =
    if in_list.len() < not_in_list.len() {
      (
        in_list,
        not_in_list,
        &mut in_list_hashset,
        &mut not_in_list_hashset,
      )
    } else {
      (
        not_in_list,
        in_list,
        &mut not_in_list_hashset,
        &mut in_list_hashset,
      )
    };

  for item in shorter_list {
    shorter_list_hashset.insert(item.to_bits_for_unique_check());
  }

  let mut invalid_items: Vec<T> = Vec::new();

  for item in longer_list {
    let comparable = item.to_bits_for_unique_check();
    if shorter_list_hashset.contains(&comparable) {
      invalid_items.push(*item);
    }
    longer_list_hashset.insert(comparable);
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

  Ok((in_list_hashset, not_in_list_hashset))
}
