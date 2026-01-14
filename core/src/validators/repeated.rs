use ordered_float::OrderedFloat;
use proto_types::{Duration, Timestamp};

use super::*;
use crate::protovalidate::violations_data::{
  REPEATED_MAX_ITEMS_VIOLATION, REPEATED_MIN_ITEMS_VIOLATION, REPEATED_UNIQUE_VIOLATION,
};

pub fn min_items<T>(
  field_context: &FieldContext,
  parent_elements: &[FieldPathElement],
  value: &[T],
  min_items: u64,
  error_message: &str,
) -> Result<(), Violation> {
  #[allow(clippy::cast_possible_truncation)]
  let is_valid = value.len() >= min_items as usize;

  if is_valid {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      REPEATED_MIN_ITEMS_VIOLATION,
      error_message,
      parent_elements,
    ))
  }
}

pub fn max_items<T>(
  field_context: &FieldContext,
  parent_elements: &[FieldPathElement],
  value: &[T],
  max_items: u64,
  error_message: &str,
) -> Result<(), Violation> {
  #[allow(clippy::cast_possible_truncation)]
  let is_valid = value.len() <= max_items as usize;

  if is_valid {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      REPEATED_MAX_ITEMS_VIOLATION,
      error_message,
      parent_elements,
    ))
  }
}

pub enum UniqueLookup<T> {
  Vec(Vec<T>),
  Set(HashSet<T>),
}

impl<T> UniqueLookup<T> {
  #[must_use]
  pub fn with_capacity(cap: usize) -> Self {
    if cap <= 24 {
      Self::Vec(Vec::with_capacity(cap))
    } else {
      Self::Set(HashSet::with_capacity(cap))
    }
  }
}

pub trait UniqueItem {
  type LookupTarget<'a>
  where
    Self: 'a;

  // This is for the types without Hash that can only initialize Vecs
  #[must_use]
  fn new_container<'a>(len: usize) -> UniqueLookup<Self::LookupTarget<'a>> {
    UniqueLookup::with_capacity(len)
  }

  fn check_unique<'a>(&'a self, container: &mut UniqueLookup<Self::LookupTarget<'a>>) -> bool;
}

macro_rules! impl_unique {
  ($target:ty) => {
    impl UniqueItem for $target {
      type LookupTarget<'a> = Self;

      fn check_unique(&self, container: &mut UniqueLookup<Self>) -> bool {
        match container {
          UniqueLookup::Vec(vec) => {
            if vec.contains(self) {
              false
            } else {
              vec.push(*self);
              true
            }
          }
          UniqueLookup::Set(set) => set.insert(*self),
        }
      }
    }
  };
}

impl_unique!(i64);
impl_unique!(i32);
impl_unique!(u64);
impl_unique!(u32);
impl_unique!(EnumVariant);
impl_unique!(Sint64);
impl_unique!(Sint32);
impl_unique!(Sfixed64);
impl_unique!(Sfixed32);
impl_unique!(Fixed64);
impl_unique!(Fixed32);
impl_unique!(Timestamp);
impl_unique!(Duration);

impl UniqueItem for str {
  type LookupTarget<'a>
    = &'a Self
  where
    Self: 'a;

  fn check_unique<'a>(&'a self, container: &mut UniqueLookup<Self::LookupTarget<'a>>) -> bool {
    match container {
      UniqueLookup::Vec(vec) => {
        if vec.contains(&self) {
          false
        } else {
          vec.push(self);
          true
        }
      }
      UniqueLookup::Set(set) => set.insert(self),
    }
  }
}

impl UniqueItem for String {
  type LookupTarget<'a>
    = &'a str
  where
    Self: 'a;

  fn check_unique<'a>(&'a self, container: &mut UniqueLookup<Self::LookupTarget<'a>>) -> bool {
    match container {
      UniqueLookup::Vec(vec) => {
        if vec.contains(&self.as_str()) {
          false
        } else {
          vec.push(self);
          true
        }
      }
      UniqueLookup::Set(set) => set.insert(self),
    }
  }
}

impl UniqueItem for f32 {
  type LookupTarget<'a> = OrderedFloat<Self>;

  fn check_unique(&self, container: &mut UniqueLookup<OrderedFloat<Self>>) -> bool {
    let item = OrderedFloat(*self);

    match container {
      UniqueLookup::Vec(vec) => {
        if vec.contains(&item) {
          false
        } else {
          vec.push(item);
          true
        }
      }
      UniqueLookup::Set(set) => set.insert(item),
    }
  }
}

impl UniqueItem for f64 {
  type LookupTarget<'a> = OrderedFloat<Self>;

  fn check_unique(&self, container: &mut UniqueLookup<OrderedFloat<Self>>) -> bool {
    let item = OrderedFloat(*self);

    match container {
      UniqueLookup::Vec(vec) => {
        if vec.contains(&item) {
          false
        } else {
          vec.push(item);
          true
        }
      }
      UniqueLookup::Set(set) => set.insert(item),
    }
  }
}

impl UniqueItem for ::bytes::Bytes {
  type LookupTarget<'a> = &'a Self;

  fn check_unique<'a>(&'a self, container: &mut UniqueLookup<&'a Self>) -> bool {
    match container {
      UniqueLookup::Vec(vec) => {
        if vec.contains(&self) {
          false
        } else {
          vec.push(self);
          true
        }
      }
      UniqueLookup::Set(set) => set.insert(self),
    }
  }
}

pub fn unique<'a, T>(
  field_context: &FieldContext,
  parent_elements: &[FieldPathElement],
  value: &'a T,
  processed_values: &mut UniqueLookup<T::LookupTarget<'a>>,
) -> Result<(), Violation>
where
  T: UniqueItem,
{
  let is_valid = value.check_unique(processed_values);

  if is_valid {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      REPEATED_UNIQUE_VIOLATION,
      "must contain unique values",
      parent_elements,
    ))
  }
}
