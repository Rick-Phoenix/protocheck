use proto_types::{Duration, Timestamp};

use super::*;
use crate::protovalidate::violations_data::{
  gt_violations::*, gte_violations::*, lt_violations::*, lte_violations::*,
};

pub trait Comparable: PartialOrd {
  const LT_VIOLATION: &'static LazyLock<ViolationData>;
  const LTE_VIOLATION: &'static LazyLock<ViolationData>;
  const GT_VIOLATION: &'static LazyLock<ViolationData>;
  const GTE_VIOLATION: &'static LazyLock<ViolationData>;
}

macro_rules! impl_comparable {
  ($target:ty, $proto_ty:ident) => {
    paste::paste! {
      impl Comparable for $target {
        const LT_VIOLATION: &'static LazyLock<ViolationData> = &[< $proto_ty _LT_VIOLATION >];
        const LTE_VIOLATION: &'static LazyLock<ViolationData> = &[< $proto_ty _LTE_VIOLATION >];
        const GT_VIOLATION: &'static LazyLock<ViolationData> = &[< $proto_ty _GT_VIOLATION >];
        const GTE_VIOLATION: &'static LazyLock<ViolationData> = &[< $proto_ty _GTE_VIOLATION >];
      }
    }
  };
}

impl_comparable!(f32, FLOAT);
impl_comparable!(f64, DOUBLE);
impl_comparable!(i32, INT32);
impl_comparable!(i64, INT64);
impl_comparable!(u32, UINT32);
impl_comparable!(u64, UINT64);
impl_comparable!(Sint32, SINT32);
impl_comparable!(Sint64, SINT64);
impl_comparable!(Sfixed32, SFIXED32);
impl_comparable!(Sfixed64, SFIXED64);
impl_comparable!(Fixed32, FIXED32);
impl_comparable!(Fixed64, FIXED64);

impl_comparable!(Duration, DURATION);
impl_comparable!(Timestamp, TIMESTAMP);

pub fn lt<T>(
  field_context: &FieldContext,
  value: T,
  target: T,
  error_message: &str,
) -> Result<(), Violation>
where
  T: Comparable,
{
  let is_valid = value < target;

  if is_valid {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      T::LT_VIOLATION,
      error_message,
    ))
  }
}

pub fn lte<T>(
  field_context: &FieldContext,
  value: T,
  target: T,
  error_message: &str,
) -> Result<(), Violation>
where
  T: Comparable,
{
  let is_valid = value <= target;

  if is_valid {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      T::LTE_VIOLATION,
      error_message,
    ))
  }
}

pub fn gt<T>(
  field_context: &FieldContext,
  value: T,
  target: T,
  error_message: &str,
) -> Result<(), Violation>
where
  T: Comparable,
{
  let is_valid = value > target;

  if is_valid {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      T::GT_VIOLATION,
      error_message,
    ))
  }
}

pub fn gte<T>(
  field_context: &FieldContext,
  value: T,
  target: T,
  error_message: &str,
) -> Result<(), Violation>
where
  T: Comparable,
{
  let is_valid = value >= target;

  if is_valid {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      T::GTE_VIOLATION,
      error_message,
    ))
  }
}
