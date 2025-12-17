use core::slice;
use std::collections::hash_set;

use ordered_float::OrderedFloat;
use proto_types::{Any, Duration};

use super::*;
use crate::protovalidate::violations_data::{in_violations::*, not_in_violations::*};

pub trait ListRules: Sized {
  type LookupTarget;
  const IN_VIOLATION: &'static LazyLock<ViolationData>;
  const NOT_IN_VIOLATION: &'static LazyLock<ViolationData>;

  fn is_in(container: &ItemLookup<Self::LookupTarget>, item: Self) -> bool;
}

#[derive(Clone, Copy, Debug)]
pub enum ItemLookup<'a, T> {
  Slice(&'a [T]),
  Set(&'a HashSet<T>),
}

pub enum ItemLookupIter<'a, T> {
  Slice(slice::Iter<'a, T>),
  Set(hash_set::Iter<'a, T>),
}

impl<'a, T> ExactSizeIterator for ItemLookupIter<'a, T> {
  fn len(&self) -> usize {
    match self {
      ItemLookupIter::Slice(iter) => iter.len(),
      ItemLookupIter::Set(iter) => iter.len(),
    }
  }
}

impl<'a, T> Iterator for ItemLookupIter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    match self {
      ItemLookupIter::Slice(iter) => iter.next(),
      ItemLookupIter::Set(iter) => iter.next(),
    }
  }
}

impl<'a, T> IntoIterator for ItemLookup<'a, T> {
  type Item = &'a T;
  type IntoIter = ItemLookupIter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    match self {
      ItemLookup::Slice(s) => ItemLookupIter::Slice(s.iter()),
      ItemLookup::Set(s) => ItemLookupIter::Set(s.iter()),
    }
  }
}

impl<'a, T> From<&'a HashSet<T>> for ItemLookup<'a, T> {
  fn from(value: &'a HashSet<T>) -> Self {
    Self::Set(value)
  }
}

impl<'a, T> From<&'a [T]> for ItemLookup<'a, T> {
  fn from(value: &'a [T]) -> Self {
    Self::Slice(value)
  }
}

macro_rules! impl_hash_lookup {
  ($typ:ty, $proto_type:ident) => {
    paste::paste! {
      impl ListRules for $typ {
        type LookupTarget = $typ;
        const IN_VIOLATION: &'static LazyLock<ViolationData> = &[< $proto_type _IN_VIOLATION >];
        const NOT_IN_VIOLATION: &'static LazyLock<ViolationData> = &[< $proto_type _NOT_IN_VIOLATION >];

        fn is_in(container: &ItemLookup<$typ>, item: $typ) -> bool {
          match container {
            ItemLookup::Slice(slice) => slice.contains(&item),
            ItemLookup::Set(set) => set.contains(&item),
          }
        }
      }
    }
  };

  ($wrapper:ty, $target:ty, $proto_type:ident) => {
    paste::paste! {
      impl ListRules for $wrapper {
        type LookupTarget = $target;
        const IN_VIOLATION: &'static LazyLock<ViolationData> = &[< $proto_type _IN_VIOLATION >];
        const NOT_IN_VIOLATION: &'static LazyLock<ViolationData> = &[< $proto_type _NOT_IN_VIOLATION >];

        fn is_in(container: &ItemLookup<$target>, item: $wrapper) -> bool {
          match container {
            ItemLookup::Slice(slice) => slice.contains(&item),
            ItemLookup::Set(set) => set.contains(&item),
          }
        }
      }
    }
  };
}

impl ListRules for f32 {
  type LookupTarget = OrderedFloat<f32>;
  const IN_VIOLATION: &'static LazyLock<ViolationData> = &FLOAT_IN_VIOLATION;
  const NOT_IN_VIOLATION: &'static LazyLock<ViolationData> = &FLOAT_NOT_IN_VIOLATION;

  fn is_in(container: &ItemLookup<OrderedFloat<f32>>, item: Self) -> bool {
    match container {
      ItemLookup::Slice(items) => items.contains(&OrderedFloat(item)),
      ItemLookup::Set(set) => set.contains(&OrderedFloat(item)),
    }
  }
}

impl ListRules for f64 {
  type LookupTarget = OrderedFloat<f64>;
  const IN_VIOLATION: &'static LazyLock<ViolationData> = &DOUBLE_IN_VIOLATION;
  const NOT_IN_VIOLATION: &'static LazyLock<ViolationData> = &DOUBLE_NOT_IN_VIOLATION;

  fn is_in(container: &ItemLookup<OrderedFloat<f64>>, item: Self) -> bool {
    match container {
      ItemLookup::Slice(items) => items.contains(&OrderedFloat(item)),
      ItemLookup::Set(set) => set.contains(&OrderedFloat(item)),
    }
  }
}

// Wrappers
impl_hash_lookup!(EnumVariant, i32, ENUM);
impl_hash_lookup!(Sint64, i64, SINT64);
impl_hash_lookup!(Sint32, i32, SINT32);
impl_hash_lookup!(Sfixed64, i64, SFIXED64);
impl_hash_lookup!(Sfixed32, i32, SFIXED32);
impl_hash_lookup!(Fixed64, u64, FIXED64);
impl_hash_lookup!(Fixed32, u32, FIXED32);

impl_hash_lookup!(i64, INT64);
impl_hash_lookup!(i32, INT32);
impl_hash_lookup!(u64, UINT64);
impl_hash_lookup!(u32, UINT32);
impl_hash_lookup!(Duration, DURATION);

impl ListRules for &::bytes::Bytes {
  type LookupTarget = &'static [u8];
  const IN_VIOLATION: &'static LazyLock<ViolationData> = &BYTES_IN_VIOLATION;
  const NOT_IN_VIOLATION: &'static LazyLock<ViolationData> = &BYTES_NOT_IN_VIOLATION;

  fn is_in(container: &ItemLookup<&[u8]>, item: &::bytes::Bytes) -> bool {
    match container {
      ItemLookup::Slice(slice) => slice.contains(&item.as_ref()),
      ItemLookup::Set(set) => set.contains(&item.as_ref()),
    }
  }
}

impl ListRules for &Any {
  type LookupTarget = &'static str;
  const IN_VIOLATION: &'static LazyLock<ViolationData> = &ANY_IN_VIOLATION;
  const NOT_IN_VIOLATION: &'static LazyLock<ViolationData> = &ANY_NOT_IN_VIOLATION;

  fn is_in(container: &ItemLookup<&str>, item: &Any) -> bool {
    match container {
      ItemLookup::Slice(slice) => slice.contains(&item.type_url.as_ref()),
      ItemLookup::Set(set) => set.contains(&item.type_url.as_ref()),
    }
  }
}

impl ListRules for &str {
  type LookupTarget = &'static str;
  const IN_VIOLATION: &'static LazyLock<ViolationData> = &STRING_IN_VIOLATION;
  const NOT_IN_VIOLATION: &'static LazyLock<ViolationData> = &STRING_NOT_IN_VIOLATION;

  fn is_in(container: &ItemLookup<&str>, item: &str) -> bool {
    match container {
      ItemLookup::Slice(slice) => slice.contains(&item),
      ItemLookup::Set(set) => set.contains(&item),
    }
  }
}

pub fn in_list<T>(
  field_context: &FieldContext,
  parent_elements: &[FieldPathElement],
  value: T,
  list: &ItemLookup<T::LookupTarget>,
  error_message: &str,
) -> Result<(), Violation>
where
  T: ListRules,
{
  let has_item = T::is_in(list, value);

  if has_item {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      T::IN_VIOLATION,
      error_message,
      parent_elements,
    ))
  }
}

pub fn not_in_list<T>(
  field_context: &FieldContext,
  parent_elements: &[FieldPathElement],
  value: T,
  list: &ItemLookup<T::LookupTarget>,
  error_message: &str,
) -> Result<(), Violation>
where
  T: ListRules,
{
  let has_item = T::is_in(list, value);

  if !has_item {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      T::NOT_IN_VIOLATION,
      error_message,
      parent_elements,
    ))
  }
}
