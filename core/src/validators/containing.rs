use std::collections::HashSet;

use paste::paste;
use proto_types::{Any, Duration};

use crate::{
  field_data::FieldContext,
  protovalidate::Violation,
  validators::static_data::{
    base_violations::create_violation, in_violations::*, not_in_violations::*,
  },
};

macro_rules! in_slice_list_validator {
  (
    $name_ty:ident,
    $value_ty:ty,
    $target_ty:ty
    $(, $value_override:expr)?
  ) => {
    in_list_validator!(slice, $name_ty, $value_ty, [$target_ty] $(, $value_override)?);
  };
}

macro_rules! in_hashset_list_validator {
  (
    $name_ty:ident,
    $value_ty:ty,
    $target_ty:ty
    $(, $value_override:expr)?
  ) => {
    in_list_validator!(hashset, $name_ty, $value_ty, HashSet<$target_ty> $(, $value_override)? );
  };
}

macro_rules! in_list_validator {
  (
    $list_ty:ident,
    $name_ty:ident,
    $value_ty:ty,
    $target_ty:ty
    $(, $value_override:expr)?
  ) => {
    macro_rules! _get_value_for_contains {
      ($override:expr, $value:ident, $target:ident) => { ($override)($value, $target) };
      (, $value:ident, $target:ident) => { $target.contains(&$value) };
    }
    paste! {
      pub fn [<$name_ty _in_ $list_ty _list>](field_context: &FieldContext, value: $value_ty, target: &'static $target_ty, error_message: &'static str) -> Result<(), Violation> {
        let check = _get_value_for_contains!($($value_override)?, value, target);

        if check {
          Ok(())
        } else {
          Err(create_violation(
            field_context,
            &[< $name_ty:upper _ IN_VIOLATION >],
            concat!(stringify!($name_ty), ".in"),
            error_message
          ))
        }
      }

      pub fn [<$name_ty _not_in_ $list_ty _list>](field_context: &FieldContext, value: $value_ty, target: &'static $target_ty, error_message: &'static str) -> Result<(), Violation> {
        let check = !_get_value_for_contains!($($value_override)?, value, target);

        if check {
          Ok(())
        } else {
          Err(create_violation(
            field_context,
            &[< $name_ty:upper _ NOT_IN_VIOLATION >],
            concat!(stringify!($name_ty), ".not_in"),
            error_message
          ))
        }
      }
    }
  };

}

in_hashset_list_validator!(string, &str, &'static str);
in_slice_list_validator!(string, &str, &'static str);

#[cfg(feature = "bytes")]
in_hashset_list_validator!(
  bytes,
  &bytes::Bytes,
  bytes::Bytes,
  |value: &bytes::Bytes, target: &'static HashSet<::bytes::Bytes>| target.contains(value)
);
#[cfg(feature = "bytes")]
in_slice_list_validator!(
  bytes,
  &bytes::Bytes,
  &'static [u8],
  |value: &bytes::Bytes, target: &'static [&'static [u8]]| target.iter().any(|by| by == value)
);

in_hashset_list_validator!(any, &Any, &'static str, |value: &Any,
                                                     target: &'static HashSet<
  &'static str,
>| {
  target.contains(value.type_url.as_str())
});
in_slice_list_validator!(any, &Any, &'static str, |value: &Any, target: &[&str]| {
  target.contains(&value.type_url.as_str())
});

in_hashset_list_validator!(duration, Duration, Duration);
in_slice_list_validator!(duration, Duration, Duration);

in_hashset_list_validator!(enum, i32, i32);
in_slice_list_validator!(enum, i32, i32);

in_hashset_list_validator!(
  float,
  f32,
  u32,
  |value: f32, target: &'static HashSet<u32>| target.contains(&value.to_bits())
);
in_slice_list_validator!(float, f32, f32);
in_hashset_list_validator!(
  double,
  f64,
  u64,
  |value: f64, target: &'static HashSet<u64>| target.contains(&value.to_bits())
);
in_slice_list_validator!(double, f64, f64);

in_hashset_list_validator!(int64, i64, i64);
in_slice_list_validator!(int64, i64, i64);
in_hashset_list_validator!(int32, i32, i32);
in_slice_list_validator!(int32, i32, i32);
in_hashset_list_validator!(sint64, i64, i64);
in_slice_list_validator!(sint64, i64, i64);
in_hashset_list_validator!(sint32, i32, i32);
in_slice_list_validator!(sint32, i32, i32);
in_hashset_list_validator!(sfixed64, i64, i64);
in_slice_list_validator!(sfixed64, i64, i64);
in_hashset_list_validator!(sfixed32, i32, i32);
in_slice_list_validator!(sfixed32, i32, i32);

in_hashset_list_validator!(fixed64, u64, u64);
in_slice_list_validator!(fixed64, u64, u64);
in_hashset_list_validator!(fixed32, u32, u32);
in_slice_list_validator!(fixed32, u32, u32);
in_hashset_list_validator!(uint64, u64, u64);
in_slice_list_validator!(uint64, u64, u64);
in_hashset_list_validator!(uint32, u32, u32);
in_slice_list_validator!(uint32, u32, u32);
