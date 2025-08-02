use std::{collections::HashSet, fmt::Debug, hash::Hash};

use proc_macro2::Span;
use syn::Error;

pub fn validate_in_not_in<T>(
  in_list: &[T],
  not_in_list: &[T],
  error_prefix: &str,
  field_span: Span,
) -> Result<(), Error>
where
  T: Eq + Hash + Debug,
{
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

pub trait FloatBits {
  type Bits: Eq + std::hash::Hash;

  #[allow(dead_code)]
  fn to_bits_for_unique_check(&self) -> Self::Bits;
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

pub fn _validate_in_not_in_floats<T, B>(
  in_list: &[T],
  not_in_list: &[T],
  error_prefix: &str,
  field_span: Span,
) -> Result<(), Error>
where
  B: Eq + Hash,
  T: FloatBits<Bits = B> + Debug,
{
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
