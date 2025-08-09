use std::collections::HashSet;

use paste::paste;
use prost::bytes::Bytes;
use proto_types::{Any, Duration};

use crate::{
  field_data::FieldContext,
  protovalidate::Violation,
  validators::static_data::{
    base_violations::create_violation, in_violations::*, not_in_violations::*,
  },
};

macro_rules! in_list_validator {
  (
    $name_ty:ident,
    $value_ty:ty,
    $target_ty:ty
    $(, $value_override:expr)?
  ) => {
    macro_rules! _get_value_for_contains {
      (value, $original_val:expr) => { $original_val };
      (type_url, $original_val:expr) => { $original_val.type_url.as_str() };
      (, $original_val:expr) => { &$original_val };
    }
    paste! {
      pub fn [<$name_ty _ in_list>](field_context: &FieldContext, value: $value_ty, target: &'static HashSet<$target_ty>, error_message: &'static str) -> Result<(), Violation> {
        let value_to_check = _get_value_for_contains!($($value_override)?, value);

        let check = target.contains(value_to_check);
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

      pub fn [<$name_ty _ not_in_list>](field_context: &FieldContext, value: $value_ty, target: &'static HashSet<$target_ty>, error_message: &'static str) -> Result<(), Violation> {
        let value_to_check = _get_value_for_contains!($($value_override)?, value);

        let check = !target.contains(value_to_check);
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

in_list_validator!(string, &str, &'static str);
in_list_validator!(bytes, &Bytes, Bytes, value);
in_list_validator!(any, &Any, &'static str, type_url);
in_list_validator!(duration, Duration, Duration);

in_list_validator!(enum, i32, i32);

in_list_validator!(float, u32, u32);
in_list_validator!(double, u64, u64);

in_list_validator!(int64, i64, i64);
in_list_validator!(int32, i32, i32);
in_list_validator!(sint64, i64, i64);
in_list_validator!(sint32, i32, i32);
in_list_validator!(sfixed64, i64, i64);
in_list_validator!(sfixed32, i32, i32);

in_list_validator!(fixed64, u64, u64);
in_list_validator!(fixed32, u32, u32);
in_list_validator!(uint64, u64, u64);
in_list_validator!(uint32, u32, u32);
