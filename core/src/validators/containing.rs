use core::slice;
use std::{ops::Deref, vec};

use ordered_float::OrderedFloat;
use proto_types::{Any, Duration, FieldMask};

use super::*;
use crate::protovalidate::violations_data::*;

pub trait ListRules: Sized {
  type LookupTarget: PartialEq + PartialOrd + Ord;
  const IN_VIOLATION: ViolationData;
  const NOT_IN_VIOLATION: ViolationData;

  fn is_in(&self, list: &SortedList<Self::LookupTarget>) -> bool;

  // Default impl works for any scalar value, FieldMask needs a separate impl
  // as shown below
  fn is_not_in(&self, list: &SortedList<Self::LookupTarget>) -> bool {
    !self.is_in(list)
  }
}

#[derive(Debug, Clone)]
pub struct SortedList<T: Ord> {
  items: Box<[T]>,
}

impl<T> SortedList<T>
where
  T: Ord,
{
  pub fn new<I: IntoIterator<Item = T>>(iter: I) -> Self {
    let mut items: Vec<T> = iter.into_iter().collect();

    items.sort();

    Self {
      items: items.into_boxed_slice(),
    }
  }

  #[must_use]
  pub fn as_slice(&self) -> &[T] {
    &self.items
  }

  pub fn contains(&self, item: &T) -> bool {
    self.items.binary_search(item).is_ok()
  }

  pub fn iter(&self) -> slice::Iter<'_, T> {
    self.into_iter()
  }

  #[allow(clippy::len_without_is_empty)]
  #[must_use]
  pub fn len(&self) -> usize {
    self.items.len()
  }
}

impl<T: Ord> Deref for SortedList<T> {
  type Target = [T];

  fn deref(&self) -> &Self::Target {
    &self.items
  }
}

impl<T: Ord> AsRef<[T]> for SortedList<T> {
  fn as_ref(&self) -> &[T] {
    &self.items
  }
}

impl<T: Ord> IntoIterator for SortedList<T> {
  type Item = T;
  type IntoIter = vec::IntoIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    self.items.into_vec().into_iter()
  }
}

impl<'a, T: Ord> IntoIterator for &'a SortedList<T> {
  type Item = &'a T;
  type IntoIter = slice::Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.items.iter()
  }
}

macro_rules! impl_lookup {
  ($typ:ty, $proto_type:ident) => {
    paste::paste! {
      impl ListRules for $typ {
        type LookupTarget = Self;
        const IN_VIOLATION: ViolationData = [< $proto_type _IN_VIOLATION >];
        const NOT_IN_VIOLATION: ViolationData = [< $proto_type _NOT_IN_VIOLATION >];

        fn is_in(&self, list: &SortedList<Self::LookupTarget>) -> bool {
          list.contains(self)
        }
      }
    }
  };

  // Wrappers
  ($wrapper:ty, $target:ty, $proto_type:ident) => {
    paste::paste! {
      impl ListRules for $wrapper {
        type LookupTarget = $target;
        const IN_VIOLATION: ViolationData = [< $proto_type _IN_VIOLATION >];
        const NOT_IN_VIOLATION: ViolationData = [< $proto_type _NOT_IN_VIOLATION >];

        fn is_in(&self, list: &SortedList<Self::LookupTarget>) -> bool {
          list.contains(self)
        }
      }
    }
  };
}

impl ListRules for f32 {
  type LookupTarget = OrderedFloat<Self>;
  const IN_VIOLATION: ViolationData = FLOAT_IN_VIOLATION;
  const NOT_IN_VIOLATION: ViolationData = FLOAT_NOT_IN_VIOLATION;

  fn is_in(&self, list: &SortedList<Self::LookupTarget>) -> bool {
    list.contains(&((*self).into()))
  }
}

impl ListRules for f64 {
  type LookupTarget = OrderedFloat<Self>;
  const IN_VIOLATION: ViolationData = DOUBLE_IN_VIOLATION;
  const NOT_IN_VIOLATION: ViolationData = DOUBLE_NOT_IN_VIOLATION;

  fn is_in(&self, list: &SortedList<Self::LookupTarget>) -> bool {
    list.contains(&((*self).into()))
  }
}

// Wrappers
impl_lookup!(EnumVariant, i32, ENUM);
impl_lookup!(Sint64, i64, SINT64);
impl_lookup!(Sint32, i32, SINT32);
impl_lookup!(Sfixed64, i64, SFIXED64);
impl_lookup!(Sfixed32, i32, SFIXED32);
impl_lookup!(Fixed64, u64, FIXED64);
impl_lookup!(Fixed32, u32, FIXED32);

impl_lookup!(i64, INT64);
impl_lookup!(i32, INT32);
impl_lookup!(u64, UINT64);
impl_lookup!(u32, UINT32);
impl_lookup!(Duration, DURATION);

impl ListRules for &::bytes::Bytes {
  type LookupTarget = &'static [u8];
  const IN_VIOLATION: ViolationData = BYTES_IN_VIOLATION;
  const NOT_IN_VIOLATION: ViolationData = BYTES_NOT_IN_VIOLATION;

  fn is_in(&self, list: &SortedList<Self::LookupTarget>) -> bool {
    list.contains(&self.as_ref())
  }
}

impl ListRules for &Any {
  type LookupTarget = &'static str;
  const IN_VIOLATION: ViolationData = ANY_IN_VIOLATION;
  const NOT_IN_VIOLATION: ViolationData = ANY_NOT_IN_VIOLATION;

  fn is_in(&self, list: &SortedList<Self::LookupTarget>) -> bool {
    list.contains(&self.type_url.as_str())
  }
}

impl ListRules for &str {
  type LookupTarget = &'static str;
  const IN_VIOLATION: ViolationData = STRING_IN_VIOLATION;
  const NOT_IN_VIOLATION: ViolationData = STRING_NOT_IN_VIOLATION;

  fn is_in(&self, list: &SortedList<Self::LookupTarget>) -> bool {
    list.contains(self)
  }
}

impl ListRules for FieldMask {
  type LookupTarget = &'static str;
  const IN_VIOLATION: ViolationData = FIELD_MASK_IN_VIOLATION;
  const NOT_IN_VIOLATION: ViolationData = FIELD_MASK_NOT_IN_VIOLATION;

  fn is_in(&self, list: &SortedList<Self::LookupTarget>) -> bool {
    self
      .paths
      .iter()
      .all(|p| list.contains(&p.as_str()))
  }

  fn is_not_in(&self, list: &SortedList<Self::LookupTarget>) -> bool {
    self
      .paths
      .iter()
      .all(|p| !list.contains(&p.as_str()))
  }
}

// We always receive refs or Copy types here anyway
#[allow(clippy::needless_pass_by_value)]
pub fn in_list<T>(
  field_context: &FieldContext,
  parent_elements: &[FieldPathElement],
  value: T,
  list: &SortedList<T::LookupTarget>,
  error_message: &str,
) -> Result<(), Violation>
where
  T: ListRules,
{
  let is_valid = value.is_in(list);

  if is_valid {
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

#[allow(clippy::needless_pass_by_value)]
pub fn not_in_list<T>(
  field_context: &FieldContext,
  parent_elements: &[FieldPathElement],
  value: T,
  list: &SortedList<T::LookupTarget>,
  error_message: &str,
) -> Result<(), Violation>
where
  T: ListRules,
{
  let is_valid = value.is_not_in(list);

  if is_valid {
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
