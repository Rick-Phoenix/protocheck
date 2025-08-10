use bytes::Bytes;
use paste::paste;
use proto_types::{Duration, Timestamp};

use crate::{
  field_data::FieldContext,
  protovalidate::Violation,
  validators::static_data::{base_violations::create_violation, const_violations::*},
};

macro_rules! const_validator {
  ($proto_type:ident, $value_type:ty) => {
    paste! {
      pub fn [< $proto_type _const>](field_context: &FieldContext, value: $value_type, target: $value_type, error_message: &'static str) -> Result<(), Violation> {
        let check = value == target;

        create_violation!($proto_type, check, field_context, const, error_message)
      }
    }
  };
}

pub fn bytes_const(
  field_context: &FieldContext,
  value: &Bytes,
  target: &'static [u8],
  error_message: &'static str,
) -> Result<(), Violation> {
  let check = value == target;

  create_violation!(bytes, check, field_context, const, error_message)
}

const_validator!(string, &str);
const_validator!(bool, bool);

const_validator!(duration, Duration);
const_validator!(timestamp, Timestamp);

const_validator!(float, f32);
const_validator!(double, f64);

const_validator!(int64, i64);
const_validator!(int32, i32);
const_validator!(sint64, i64);
const_validator!(sint32, i32);
const_validator!(sfixed64, i64);
const_validator!(sfixed32, i32);

const_validator!(uint64, u64);
const_validator!(uint32, u32);
const_validator!(fixed64, u64);
const_validator!(fixed32, u32);

const_validator!(enum, i32);
