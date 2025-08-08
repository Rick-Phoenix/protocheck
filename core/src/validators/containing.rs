use std::collections::HashSet;

use paste::paste;
use prost::bytes::Bytes;
use proto_types::{Any, Duration};

use crate::{
  field_data::FieldContext,
  protovalidate::Violation,
  validators::static_data::{base_violations::create_violation, in_rules::*, not_in_rules::*},
};

macro_rules! in_list_validator {
    (
    $name_ty:ident,
    $value_ty:ty,
    $target_ty:ty,
    $violation_path:ident
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
            &[< $violation_path _ IN_VIOLATION >],
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
            &[< $violation_path _ NOT_IN_VIOLATION >],
            concat!(stringify!($name_ty), ".not_in"),
            error_message
          ))
        }
      }
    }
  };

}

in_list_validator!(string, &str, &'static str, STRING);
in_list_validator!(bytes, &Bytes, Bytes, BYTES, value);
in_list_validator!(any, &Any, &'static str, ANY, type_url);
in_list_validator!(duration, Duration, Duration, DURATION);
in_list_validator!(enum, i32, i32, ENUM);
in_list_validator!(float, u32, u32, FLOAT);
in_list_validator!(double, u64, u64, DOUBLE);
in_list_validator!(int64, i64, i64, INT64);
in_list_validator!(int32, i32, i32, INT32);
in_list_validator!(sint64, i64, i64, SINT64);
in_list_validator!(sint32, i32, i32, SINT32);
in_list_validator!(sfixed64, i64, i64, SFIXED64);
in_list_validator!(sfixed32, i32, i32, SFIXED32);
in_list_validator!(fixed64, u64, u64, FIXED64);
in_list_validator!(fixed32, u32, u32, FIXED32);
in_list_validator!(uint64, u64, u64, UINT64);
in_list_validator!(uint32, u32, u32, UINT32);
