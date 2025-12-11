use proto_types::{Duration, Timestamp};

use super::*;
use crate::protovalidate::violations_data::const_violations::*;

pub trait ConstRule {
  const CONST_VIOLATION: &'static LazyLock<ViolationData>;
}

macro_rules! impl_const {
  ($target:ty, $proto_ty:ident) => {
    paste::paste! {
      impl ConstRule for $target {
        const CONST_VIOLATION: &'static LazyLock<ViolationData> = &[< $proto_ty _CONST_VIOLATION >];
      }
    }
  };
}

impl_const!(&str, STRING);
impl_const!(EnumVariant, ENUM);
impl_const!(bool, BOOL);
impl_const!(&::bytes::Bytes, BYTES);
impl_const!(Duration, DURATION);
impl_const!(Timestamp, TIMESTAMP);
impl_const!(i64, INT64);
impl_const!(i32, INT32);
impl_const!(u64, UINT64);
impl_const!(u32, UINT32);
impl_const!(Sint64, SINT64);
impl_const!(Sint32, SINT32);
impl_const!(Sfixed64, SFIXED64);
impl_const!(Sfixed32, SFIXED32);
impl_const!(Fixed64, FIXED64);
impl_const!(Fixed32, FIXED32);
impl_const!(f64, DOUBLE);
impl_const!(f32, FLOAT);

pub fn constant<T, V>(
  field_context: &FieldContext,
  value: V,
  target: T,
  error_message: &str,
) -> Result<(), Violation>
where
  V: ConstRule + PartialEq<T>,
{
  let is_valid = value == target;

  if is_valid {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      V::CONST_VIOLATION,
      error_message,
    ))
  }
}
