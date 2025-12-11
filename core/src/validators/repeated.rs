use proto_types::{Duration, Timestamp};

use super::*;
use crate::protovalidate::violations_data::{
  REPEATED_MAX_ITEMS_VIOLATION, REPEATED_MIN_ITEMS_VIOLATION, REPEATED_UNIQUE_VIOLATION,
};

pub fn min_items<T>(
  field_context: &FieldContext,
  value: &[T],
  min_items: u64,
  error_message: &str,
) -> Result<(), Violation> {
  let check = value.len() >= min_items as usize;

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &REPEATED_MIN_ITEMS_VIOLATION,
      error_message,
    ))
  }
}

pub fn max_items<T>(
  field_context: &FieldContext,
  value: &[T],
  max_items: u64,
  error_message: &str,
) -> Result<(), Violation> {
  let check = value.len() <= max_items as usize;

  if check {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &REPEATED_MAX_ITEMS_VIOLATION,
      error_message,
    ))
  }
}

pub enum UniqueLookup<T> {
  Vec(Vec<T>),
  Set(HashSet<T>),
}

pub trait UniqueItem<Item = Self> {
  type Container;

  fn check_unique(container: &mut Self::Container, item: Self) -> bool;
}

macro_rules! impl_unique {
  ($target:ty) => {
    impl UniqueItem for $target {
      type Container = UniqueLookup<Self>;

      fn check_unique(container: &mut UniqueLookup<Self>, item: Self) -> bool {
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

#[cfg(not(feature = "ordered-float"))]
impl UniqueItem for f32 {
  type Container = UniqueLookup<Self>;

  fn check_unique(container: &mut UniqueLookup<Self>, item: Self) -> bool {
    match container {
      UniqueLookup::Vec(vec) => {
        if vec.contains(&item) {
          false
        } else {
          vec.push(item);
          true
        }
      }
      UniqueLookup::Set(_) => {
        panic!("Cannot use set lookups for floats without the ordered-float flag")
      }
    }
  }
}

#[cfg(not(feature = "ordered-float"))]
impl UniqueItem for f64 {
  type Container = UniqueLookup<Self>;

  fn check_unique(container: &mut UniqueLookup<Self>, item: Self) -> bool {
    match container {
      UniqueLookup::Vec(vec) => {
        if vec.contains(&item) {
          false
        } else {
          vec.push(item);
          true
        }
      }
      UniqueLookup::Set(_) => {
        panic!("Cannot use set lookups for floats without the ordered-float flag")
      }
    }
  }
}

#[cfg(feature = "ordered-float")]
impl UniqueItem<ordered_float::OrderedFloat<f32>> for f32 {
  type Container = UniqueLookup<ordered_float::OrderedFloat<f32>>;

  fn check_unique(
    container: &mut UniqueLookup<ordered_float::OrderedFloat<f32>>,
    item: Self,
  ) -> bool {
    let item = ordered_float::OrderedFloat(item);

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

#[cfg(feature = "ordered-float")]
impl UniqueItem<ordered_float::OrderedFloat<f64>> for f64 {
  type Container = UniqueLookup<ordered_float::OrderedFloat<f64>>;

  fn check_unique(
    container: &mut UniqueLookup<ordered_float::OrderedFloat<f64>>,
    item: Self,
  ) -> bool {
    let item = ordered_float::OrderedFloat(item);

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

impl UniqueItem for &::bytes::Bytes {
  type Container = UniqueLookup<Self>;

  fn check_unique(container: &mut UniqueLookup<Self>, item: Self) -> bool {
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

impl UniqueItem for &str {
  type Container = UniqueLookup<Self>;

  fn check_unique(container: &mut UniqueLookup<Self>, item: Self) -> bool {
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

pub fn unique<T, I>(
  field_context: &FieldContext,
  value: T,
  processed_values: &mut T::Container,
) -> Result<(), Violation>
where
  T: UniqueItem<I>,
{
  let is_valid = T::check_unique(processed_values, value);

  if is_valid {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      &REPEATED_UNIQUE_VIOLATION,
      "must contain unique values",
    ))
  }
}
