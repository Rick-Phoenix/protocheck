use paste::paste;
use proto_types::{Duration, Timestamp};

use crate::{
  field_data::FieldContext,
  protovalidate::Violation,
  validators::static_data::{
    base_violations::create_violation, gt_rules::*, gte_rules::*, lt_rules::*, lte_rules::*,
  },
};

macro_rules! comparable_validator {
  ($proto_type:ident, $rust_type:ty) => {
    paste! {
      pub fn [< $proto_type _lt >](field_context: &FieldContext, value: $rust_type, target: $rust_type, error_message: &'static str) -> Result<(), Violation> {
        let check = value < target;

        create_violation!($proto_type, check, field_context, lt, error_message)
      }

      pub fn [< $proto_type _lte >](field_context: &FieldContext, value: $rust_type, target: $rust_type, error_message: &'static str) -> Result<(), Violation> {
        let check = value <= target;

        create_violation!($proto_type, check, field_context, lte, error_message)
      }

      pub fn [< $proto_type _gt >](field_context: &FieldContext, value: $rust_type, target: $rust_type, error_message: &'static str) -> Result<(), Violation> {
        let check = value > target;

        create_violation!($proto_type, check, field_context, gt, error_message)
      }

      pub fn [< $proto_type _gte >](field_context: &FieldContext, value: $rust_type, target: $rust_type, error_message: &'static str) -> Result<(), Violation> {
        let check = value >= target;

        create_violation!($proto_type, check, field_context, gte, error_message)
      }
    }
  };
}

comparable_validator!(float, f32);
comparable_validator!(double, f64);

comparable_validator!(int64, i64);
comparable_validator!(int32, i32);
comparable_validator!(sint64, i64);
comparable_validator!(sint32, i32);
comparable_validator!(sfixed64, i64);
comparable_validator!(sfixed32, i32);

comparable_validator!(uint64, u64);
comparable_validator!(uint32, u32);
comparable_validator!(fixed64, u64);
comparable_validator!(fixed32, u32);

comparable_validator!(duration, Duration);
comparable_validator!(timestamp, Timestamp);
